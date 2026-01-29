//! DNS policy and configuration validation for Kubernetes core/v1 API
//!
//! This module implements validation for pod DNS settings including:
//! - DNS policy validation
//! - DNS configuration validation (nameservers, searches, options)

use crate::common::validation::{BadValue, ErrorList, Path, invalid, not_supported, required};
use crate::core::v1::pod::{PodDNSConfig, PodDNSConfigOption};
use std::collections::HashSet;
use std::sync::LazyLock;

// ============================================================================
// Constants
// ============================================================================

/// Maximum number of DNS nameservers (derived from Linux libc restrictions)
pub const MAX_DNS_NAMESERVERS: usize = 3;

/// Maximum number of DNS search paths (derived from Linux libc restrictions)
pub const MAX_DNS_SEARCH_PATHS: usize = 32;

/// Maximum total characters in DNS search list including spaces
pub const MAX_DNS_SEARCH_LIST_CHARS: usize = 2048;

/// Supported DNS policies
static SUPPORTED_DNS_POLICIES: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| HashSet::from(["ClusterFirst", "ClusterFirstWithHostNet", "Default", "None"]));

// ============================================================================
// DNS Policy Validation
// ============================================================================

/// Validates DNS policy.
///
/// Valid policies:
/// - ClusterFirst: Use cluster DNS
/// - ClusterFirstWithHostNet: Use cluster DNS even with hostNetwork
/// - Default: Use node's DNS resolution
/// - None: Pod uses its own DNS configuration
pub fn validate_dns_policy(policy: &str, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if policy.is_empty() {
        all_errs.push(required(path, "dnsPolicy is required"));
    } else if !SUPPORTED_DNS_POLICIES.contains(policy) {
        let valid: Vec<&str> = SUPPORTED_DNS_POLICIES.iter().copied().collect();
        all_errs.push(not_supported(
            path,
            BadValue::String(policy.to_string()),
            &valid,
        ));
    }

    all_errs
}

// ============================================================================
// DNS Config Validation
// ============================================================================

/// Validates pod DNS configuration.
///
/// Validates:
/// - If dnsPolicy is "None", dnsConfig must be provided with at least one nameserver
/// - Nameservers: max 3, each must be a valid IP address
/// - Searches: max 32 paths, max 2048 total characters, each must be a DNS subdomain
/// - Options: each option must have a non-empty name
pub fn validate_pod_dns_config(
    dns_config: Option<&PodDNSConfig>,
    dns_policy: Option<&str>,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate DNSNone case: must provide at least one DNS nameserver
    if let Some(policy) = dns_policy {
        if policy == "None" {
            if dns_config.is_none() {
                all_errs.push(required(
                    path,
                    "must provide `dnsConfig` when `dnsPolicy` is None",
                ));
                return all_errs;
            }

            if let Some(config) = dns_config {
                if config.nameservers.is_empty() {
                    all_errs.push(required(
                        &path.child("nameservers"),
                        "must provide at least one DNS nameserver when `dnsPolicy` is None",
                    ));
                    return all_errs;
                }
            }
        }
    }

    // Validate DNS config if provided
    if let Some(config) = dns_config {
        all_errs.extend(validate_dns_config(config, path));
    }

    all_errs
}

/// Validates a DNS configuration.
fn validate_dns_config(config: &PodDNSConfig, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate nameservers
    all_errs.extend(validate_nameservers(
        &config.nameservers,
        &path.child("nameservers"),
    ));

    // Validate search paths
    all_errs.extend(validate_search_paths(
        &config.searches,
        &path.child("searches"),
    ));

    // Validate options
    all_errs.extend(validate_dns_options(
        &config.options,
        &path.child("options"),
    ));

    all_errs
}

/// Validates DNS nameservers.
fn validate_nameservers(nameservers: &[String], path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Check count
    if nameservers.len() > MAX_DNS_NAMESERVERS {
        all_errs.push(invalid(
            path,
            BadValue::String(format!("{} nameservers", nameservers.len())),
            &format!(
                "must not have more than {} nameservers",
                MAX_DNS_NAMESERVERS
            ),
        ));
    }

    // Validate each nameserver is a valid IP
    for (i, ns) in nameservers.iter().enumerate() {
        if !is_valid_ip(ns) {
            all_errs.push(invalid(
                &path.index(i),
                BadValue::String(ns.clone()),
                "must be a valid IP address",
            ));
        }
    }

    all_errs
}

/// Validates DNS search paths.
fn validate_search_paths(searches: &[String], path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Check count
    if searches.len() > MAX_DNS_SEARCH_PATHS {
        all_errs.push(invalid(
            path,
            BadValue::String(format!("{} search paths", searches.len())),
            &format!(
                "must not have more than {} search paths",
                MAX_DNS_SEARCH_PATHS
            ),
        ));
    }

    // Check total character count (including spaces between paths)
    let total_chars = searches.join(" ").len();
    if total_chars > MAX_DNS_SEARCH_LIST_CHARS {
        all_errs.push(invalid(
            path,
            BadValue::String(format!("{} characters", total_chars)),
            &format!(
                "must not have more than {} characters (including spaces) in the search list",
                MAX_DNS_SEARCH_LIST_CHARS
            ),
        ));
    }

    // Validate each search path is a DNS subdomain
    for (i, search) in searches.iter().enumerate() {
        // Special case: "." is allowed
        if search == "." {
            continue;
        }

        // Trim trailing "." before validation
        let search_trimmed = search.trim_end_matches('.');

        let dns_errs = crate::common::validation::is_dns1123_subdomain(search_trimmed);
        if !dns_errs.is_empty() {
            all_errs.push(invalid(
                &path.index(i),
                BadValue::String(search.clone()),
                "must be a valid DNS subdomain",
            ));
        }
    }

    all_errs
}

/// Validates DNS options.
fn validate_dns_options(options: &[PodDNSConfigOption], path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (i, option) in options.iter().enumerate() {
        if option.name.is_empty() {
            all_errs.push(required(&path.index(i), "option name must not be empty"));
        }
    }

    all_errs
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Validates if a string is a valid IP address (IPv4 or IPv6).
fn is_valid_ip(ip: &str) -> bool {
    // Try to parse as IPv4 or IPv6
    ip.parse::<std::net::IpAddr>().is_ok()
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_dns_policy_valid() {
        assert!(validate_dns_policy("ClusterFirst", &Path::nil()).is_empty());
        assert!(validate_dns_policy("ClusterFirstWithHostNet", &Path::nil()).is_empty());
        assert!(validate_dns_policy("Default", &Path::nil()).is_empty());
        assert!(validate_dns_policy("None", &Path::nil()).is_empty());
    }

    #[test]
    fn test_validate_dns_policy_invalid() {
        let errs = validate_dns_policy("InvalidPolicy", &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.error_type == crate::common::validation::ErrorType::NotSupported)
        );
    }

    #[test]
    fn test_validate_dns_policy_empty() {
        let errs = validate_dns_policy("", &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.detail.contains("dnsPolicy is required"))
        );
    }

    #[test]
    fn test_validate_pod_dns_config_none_policy_no_config() {
        let errs = validate_pod_dns_config(None, Some("None"), &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.detail.contains("must provide `dnsConfig`"))
        );
    }

    #[test]
    fn test_validate_pod_dns_config_none_policy_no_nameservers() {
        let config = PodDNSConfig {
            nameservers: vec![],
            searches: vec![],
            options: vec![],
        };
        let errs = validate_pod_dns_config(Some(&config), Some("None"), &Path::nil());
        assert!(!errs.is_empty());
        assert!(errs.errors.iter().any(|e| {
            e.detail
                .contains("must provide at least one DNS nameserver")
        }));
    }

    #[test]
    fn test_validate_nameservers_too_many() {
        let nameservers = vec![
            "8.8.8.8".to_string(),
            "8.8.4.4".to_string(),
            "1.1.1.1".to_string(),
            "1.0.0.1".to_string(), // 4 nameservers - exceeds limit of 3
        ];
        let errs = validate_nameservers(&nameservers, &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.detail.contains("must not have more than 3 nameservers"))
        );
    }

    #[test]
    fn test_validate_nameservers_invalid_ip() {
        let nameservers = vec!["not-an-ip".to_string()];
        let errs = validate_nameservers(&nameservers, &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.detail.contains("must be a valid IP address"))
        );
    }

    #[test]
    fn test_validate_nameservers_valid() {
        let nameservers = vec![
            "8.8.8.8".to_string(),
            "2001:4860:4860::8888".to_string(), // IPv6
        ];
        let errs = validate_nameservers(&nameservers, &Path::nil());
        assert!(
            errs.is_empty(),
            "Valid nameservers should not produce errors"
        );
    }

    #[test]
    fn test_validate_search_paths_too_many() {
        let searches: Vec<String> = (0..33)
            .map(|i| format!("search{}.example.com", i))
            .collect();
        let errs = validate_search_paths(&searches, &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| { e.detail.contains("must not have more than 32 search paths") })
        );
    }

    #[test]
    fn test_validate_search_paths_too_long() {
        // Create a search list that exceeds 2048 characters
        let long_search = "a".repeat(500);
        let searches = vec![
            long_search.clone(),
            long_search.clone(),
            long_search.clone(),
            long_search.clone(),
            long_search,
        ];
        let errs = validate_search_paths(&searches, &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| { e.detail.contains("must not have more than 2048 characters") })
        );
    }

    #[test]
    fn test_validate_search_paths_invalid_subdomain() {
        let searches = vec!["not a valid subdomain!".to_string()];
        let errs = validate_search_paths(&searches, &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.detail.contains("must be a valid DNS subdomain"))
        );
    }

    #[test]
    fn test_validate_search_paths_trailing_dot() {
        let searches = vec!["example.com.".to_string()];
        let errs = validate_search_paths(&searches, &Path::nil());
        assert!(
            errs.is_empty(),
            "Trailing dot should be trimmed and validated"
        );
    }

    #[test]
    fn test_validate_search_paths_dot_special_case() {
        let searches = vec![".".to_string()];
        let errs = validate_search_paths(&searches, &Path::nil());
        assert!(errs.is_empty(), "Single dot should be allowed");
    }

    #[test]
    fn test_validate_search_paths_valid() {
        let searches = vec!["example.com".to_string(), "cluster.local".to_string()];
        let errs = validate_search_paths(&searches, &Path::nil());
        assert!(
            errs.is_empty(),
            "Valid search paths should not produce errors"
        );
    }

    #[test]
    fn test_validate_dns_options_empty_name() {
        let options = vec![PodDNSConfigOption {
            name: String::new(),
            value: Some("value".to_string()),
        }];
        let errs = validate_dns_options(&options, &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.detail.contains("option name must not be empty"))
        );
    }

    #[test]
    fn test_validate_dns_options_valid() {
        let options = vec![
            PodDNSConfigOption {
                name: "ndots".to_string(),
                value: Some("5".to_string()),
            },
            PodDNSConfigOption {
                name: "timeout".to_string(),
                value: Some("30".to_string()),
            },
        ];
        let errs = validate_dns_options(&options, &Path::nil());
        assert!(
            errs.is_empty(),
            "Valid DNS options should not produce errors"
        );
    }

    #[test]
    fn test_is_valid_ip() {
        assert!(is_valid_ip("8.8.8.8"));
        assert!(is_valid_ip("192.168.1.1"));
        assert!(is_valid_ip("2001:4860:4860::8888"));
        assert!(is_valid_ip("::1"));

        assert!(!is_valid_ip("not-an-ip"));
        assert!(!is_valid_ip("256.256.256.256"));
        assert!(!is_valid_ip(""));
    }

    #[test]
    fn test_validate_pod_dns_config_valid() {
        let config = PodDNSConfig {
            nameservers: vec!["8.8.8.8".to_string()],
            searches: vec!["example.com".to_string()],
            options: vec![PodDNSConfigOption {
                name: "ndots".to_string(),
                value: Some("5".to_string()),
            }],
        };
        let errs = validate_pod_dns_config(Some(&config), Some("None"), &Path::nil());
        assert!(
            errs.is_empty(),
            "Valid DNS config should not produce errors"
        );
    }
}
