//! DNS policy and configuration validation for Kubernetes core internal API types.

use crate::common::validation::{BadValue, ErrorList, Path, invalid, required};
use crate::core::internal::{DNSPolicy, PodDNSConfig, PodDNSConfigOption};

/// Maximum number of DNS nameservers (derived from Linux libc restrictions)
pub const MAX_DNS_NAMESERVERS: usize = 3;

/// Maximum number of DNS search paths (derived from Linux libc restrictions)
pub const MAX_DNS_SEARCH_PATHS: usize = 32;

/// Maximum total characters in DNS search list including spaces
pub const MAX_DNS_SEARCH_LIST_CHARS: usize = 2048;

/// Validates DNS policy.
pub fn validate_dns_policy(_policy: &DNSPolicy, _path: &Path) -> ErrorList {
    ErrorList::new()
}

/// Validates pod DNS configuration.
pub fn validate_pod_dns_config(
    dns_config: Option<&PodDNSConfig>,
    dns_policy: &DNSPolicy,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if matches!(dns_policy, DNSPolicy::None) {
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

    if let Some(config) = dns_config {
        all_errs.extend(validate_dns_config(config, path));
    }

    all_errs
}

fn validate_dns_config(config: &PodDNSConfig, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    all_errs.extend(validate_nameservers(
        &config.nameservers,
        &path.child("nameservers"),
    ));

    all_errs.extend(validate_search_paths(
        &config.searches,
        &path.child("searches"),
    ));

    all_errs.extend(validate_dns_options(
        &config.options,
        &path.child("options"),
    ));

    all_errs
}

fn validate_nameservers(nameservers: &[String], path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

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

fn validate_search_paths(searches: &[String], path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

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

    for (i, search) in searches.iter().enumerate() {
        if search == "." {
            continue;
        }

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

fn validate_dns_options(options: &[PodDNSConfigOption], path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (i, option) in options.iter().enumerate() {
        if option.name.is_empty() {
            all_errs.push(required(&path.index(i), "option name must not be empty"));
        }
    }

    all_errs
}

fn is_valid_ip(ip: &str) -> bool {
    ip.parse::<std::net::IpAddr>().is_ok()
}
