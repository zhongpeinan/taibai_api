//! Helper functions for core v1 API validation
//!
//! Provides reusable validation utilities used across multiple validators.

use crate::common::IntOrString;
use crate::common::validation::{
    BadValue, ErrorList, Path, forbidden, invalid, is_dns1035_label, is_dns1123_label,
    is_dns1123_subdomain, not_supported, required,
};
use std::collections::HashSet;
use std::net::IpAddr;

use super::constants::*;

// ============================================================================
// Port Validation
// ============================================================================

/// Validates that a port number is in the valid range (1-65535)
pub fn validate_port_number(port: i32, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if !(MIN_PORT..=MAX_PORT).contains(&port) {
        all_errs.push(invalid(
            path,
            BadValue::Int(port as i64),
            &format!("must be in the range {}-{}", MIN_PORT, MAX_PORT),
        ));
    }
    all_errs
}

/// Validates a port name (must be lowercase alphanumeric or '-', 1-15 chars)
pub fn validate_port_name(name: &str, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if name.is_empty() {
        return all_errs; // Empty is allowed for single-port services
    }

    let is_valid = name.len() <= 15
        && name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        && name.chars().any(|c| c.is_ascii_lowercase())
        && !name.starts_with('-')
        && !name.ends_with('-')
        && !name.contains("--");

    if !is_valid {
        all_errs.push(invalid(
            path,
            BadValue::String(name.to_string()),
            "must be a valid port name (lowercase alphanumeric or '-', 1-15 chars, must contain at least one letter, no consecutive hyphens)",
        ));
    }
    all_errs
}

/// Validates a port that can be either a number or a name (IntOrString)
pub fn validate_port_num_or_name(port: &IntOrString, path: &Path) -> ErrorList {
    match port {
        IntOrString::Int(num) => validate_port_number(*num, path),
        IntOrString::String(name) => validate_port_name(name, path),
    }
}

/// Validates that a node port is in the valid range (30000-32767)
pub fn validate_node_port(port: i32, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if !(MIN_NODE_PORT..=MAX_NODE_PORT).contains(&port) {
        all_errs.push(invalid(
            path,
            BadValue::Int(port as i64),
            &format!("must be in the range {}-{}", MIN_NODE_PORT, MAX_NODE_PORT),
        ));
    }
    all_errs
}

/// Validates port or int-string (for TargetPort)
pub fn validate_int_or_string_port(port: &IntOrString, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    match port {
        IntOrString::Int(p) => {
            all_errs.extend(validate_port_number(*p, path));
        }
        IntOrString::String(name) => {
            all_errs.extend(validate_port_name(name, path));
        }
    }
    all_errs
}

// ============================================================================
// Protocol Validation
// ============================================================================

/// Validates protocol (TCP, UDP, SCTP)
pub fn validate_protocol(protocol: &str, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if !SUPPORTED_PORT_PROTOCOLS.contains(protocol) {
        all_errs.push(not_supported(
            path,
            BadValue::String(protocol.to_string()),
            &SUPPORTED_PORT_PROTOCOLS.iter().copied().collect::<Vec<_>>(),
        ));
    }
    all_errs
}

// ============================================================================
// DNS Validation
// ============================================================================

/// Validates DNS1123 subdomain
pub fn validate_dns1123_subdomain(value: &str, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    for msg in is_dns1123_subdomain(value) {
        all_errs.push(invalid(path, BadValue::String(value.to_string()), &msg));
    }
    all_errs
}

/// Validates DNS1123 label
pub fn validate_dns1123_label(value: &str, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    for msg in is_dns1123_label(value) {
        all_errs.push(invalid(path, BadValue::String(value.to_string()), &msg));
    }
    all_errs
}

/// Validates DNS1035 label (for service names)
pub fn validate_dns1035_label(value: &str, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    for msg in is_dns1035_label(value) {
        all_errs.push(invalid(path, BadValue::String(value.to_string()), &msg));
    }
    all_errs
}

// ============================================================================
// Name Validation
// ============================================================================

/// Validates container name (DNS1123 label)
pub fn validate_container_name(name: &str, path: &Path) -> ErrorList {
    validate_dns1123_label(name, path)
}

/// Validates volume name (DNS1123 label)
pub fn validate_volume_name(name: &str, path: &Path) -> ErrorList {
    validate_dns1123_label(name, path)
}

// ============================================================================
// Environment Variable Validation
// ============================================================================

/// Validates environment variable name (C identifier)
pub fn validate_env_var_name(name: &str, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if name.is_empty() {
        all_errs.push(required(path, "name is required"));
        return all_errs;
    }

    // Must be a valid C identifier: [A-Za-z_][A-Za-z0-9_]*
    let is_valid = name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
        && !name.chars().next().unwrap().is_ascii_digit();

    if !is_valid {
        all_errs.push(invalid(
            path,
            BadValue::String(name.to_string()),
            "must be a C identifier (alphanumeric or '_', cannot start with digit)",
        ));
    }

    all_errs
}

// ============================================================================
// Numeric Field Validation
// ============================================================================

/// Validates that a value is non-negative (>= 0)
pub fn validate_nonnegative_field(value: i64, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if value < 0 {
        all_errs.push(invalid(path, BadValue::Int(value), IS_NEGATIVE_ERROR_MSG));
    }
    all_errs
}

/// Validates that a value is positive (> 0)
pub fn validate_positive_field(value: i64, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if value <= 0 {
        all_errs.push(invalid(
            path,
            BadValue::Int(value),
            IS_NOT_POSITIVE_ERROR_MSG,
        ));
    }
    all_errs
}

// ============================================================================
// Immutability Validation
// ============================================================================

/// Validates that a field has not changed (for updates)
pub fn validate_immutable_field<T: PartialEq>(new: &T, old: &T, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if new != old {
        all_errs.push(forbidden(path, FIELD_IMMUTABLE_ERROR_MSG));
    }
    all_errs
}

/// Validates that an optional field has not changed (for updates)
pub fn validate_immutable_field_option<T: PartialEq>(
    new: &Option<T>,
    old: &Option<T>,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    match (new, old) {
        (Some(n), Some(o)) if n != o => {
            all_errs.push(forbidden(path, FIELD_IMMUTABLE_ERROR_MSG));
        }
        (Some(_), None) | (None, Some(_)) => {
            all_errs.push(forbidden(path, FIELD_IMMUTABLE_ERROR_MSG));
        }
        _ => {}
    }
    all_errs
}

// ============================================================================
// Path Validation
// ============================================================================

/// Validates that a path is absolute
pub fn validate_absolute_path(path_str: &str, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if !path_str.starts_with('/') {
        all_errs.push(invalid(
            path,
            BadValue::String(path_str.to_string()),
            "must be an absolute path",
        ));
    }
    all_errs
}

/// Validates that a path doesn't contain backsteps (../)
pub fn validate_path_no_backsteps(path_str: &str, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if path_str.contains("..") {
        all_errs.push(invalid(
            path,
            BadValue::String(path_str.to_string()),
            "must not contain '..'",
        ));
    }
    all_errs
}

/// Validates local descending path (no .. or absolute path)
pub fn validate_local_descending_path(path_str: &str, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if path_str.starts_with('/') {
        all_errs.push(invalid(
            path,
            BadValue::String(path_str.to_string()),
            "must be a relative path",
        ));
    }

    if path_str.contains("..") {
        all_errs.push(invalid(
            path,
            BadValue::String(path_str.to_string()),
            "must not contain '..'",
        ));
    }

    all_errs
}

// ============================================================================
// IP Address Validation
// ============================================================================

/// Validates IP address format
pub fn validate_ip_address(ip: &str, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if ip.parse::<IpAddr>().is_err() {
        all_errs.push(invalid(
            path,
            BadValue::String(ip.to_string()),
            "must be a valid IP address",
        ));
    }
    all_errs
}

/// Check if a string is a valid IP address
pub fn is_ip_address(ip: &str) -> bool {
    ip.parse::<IpAddr>().is_ok()
}

// ============================================================================
// File Mode Validation
// ============================================================================

/// Validates file mode (0-0777 octal)
pub fn validate_file_mode(mode: i32, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if !(0..=MAX_FILE_MODE).contains(&mode) {
        all_errs.push(invalid(
            path,
            BadValue::Int(mode as i64),
            FILE_MODE_ERROR_MSG,
        ));
    }
    all_errs
}

// ============================================================================
// Duplicate Detection
// ============================================================================

/// Checks for duplicate names in a list and returns errors for each duplicate
pub fn validate_no_duplicate_names<T, F>(items: &[T], get_name: F, path: &Path) -> ErrorList
where
    F: Fn(&T) -> &str,
{
    let mut all_errs = ErrorList::new();
    let mut seen = HashSet::new();

    for (i, item) in items.iter().enumerate() {
        let name = get_name(item);
        if !seen.insert(name) {
            all_errs.push(crate::common::validation::duplicate(
                &path.index(i).child("name"),
                BadValue::String(name.to_string()),
            ));
        }
    }

    all_errs
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_port_number() {
        let path = Path::nil();

        // Valid ports
        assert!(validate_port_number(1, &path).is_empty());
        assert!(validate_port_number(80, &path).is_empty());
        assert!(validate_port_number(8080, &path).is_empty());
        assert!(validate_port_number(65535, &path).is_empty());

        // Invalid ports
        assert!(!validate_port_number(0, &path).is_empty());
        assert!(!validate_port_number(-1, &path).is_empty());
        assert!(!validate_port_number(65536, &path).is_empty());
    }

    #[test]
    fn test_validate_port_name() {
        let path = Path::nil();

        // Valid names
        assert!(validate_port_name("http", &path).is_empty());
        assert!(validate_port_name("http-80", &path).is_empty());
        assert!(validate_port_name("", &path).is_empty()); // Empty is allowed

        // Invalid names
        assert!(!validate_port_name("HTTP", &path).is_empty()); // Uppercase
        assert!(!validate_port_name("-http", &path).is_empty()); // Starts with dash
        assert!(!validate_port_name("http-", &path).is_empty()); // Ends with dash
        assert!(!validate_port_name("http--80", &path).is_empty()); // Consecutive dashes
        assert!(!validate_port_name("1234", &path).is_empty()); // No letter
        assert!(!validate_port_name("verylongportname", &path).is_empty()); // Too long (>15)
    }

    #[test]
    fn test_validate_protocol() {
        let path = Path::nil();

        // Valid protocols
        assert!(validate_protocol("TCP", &path).is_empty());
        assert!(validate_protocol("UDP", &path).is_empty());
        assert!(validate_protocol("SCTP", &path).is_empty());

        // Invalid protocol
        assert!(!validate_protocol("HTTP", &path).is_empty());
        assert!(!validate_protocol("tcp", &path).is_empty()); // Lowercase
    }

    #[test]
    fn test_validate_env_var_name() {
        let path = Path::nil();

        // Valid names
        assert!(validate_env_var_name("MY_VAR", &path).is_empty());
        assert!(validate_env_var_name("_VAR", &path).is_empty());
        assert!(validate_env_var_name("VAR123", &path).is_empty());

        // Invalid names
        assert!(!validate_env_var_name("", &path).is_empty()); // Empty
        assert!(!validate_env_var_name("123VAR", &path).is_empty()); // Starts with digit
        assert!(!validate_env_var_name("MY-VAR", &path).is_empty()); // Contains dash
        assert!(!validate_env_var_name("MY.VAR", &path).is_empty()); // Contains dot
    }

    #[test]
    fn test_validate_nonnegative_field() {
        let path = Path::nil();

        assert!(validate_nonnegative_field(0, &path).is_empty());
        assert!(validate_nonnegative_field(100, &path).is_empty());
        assert!(!validate_nonnegative_field(-1, &path).is_empty());
    }

    #[test]
    fn test_validate_positive_field() {
        let path = Path::nil();

        assert!(validate_positive_field(1, &path).is_empty());
        assert!(validate_positive_field(100, &path).is_empty());
        assert!(!validate_positive_field(0, &path).is_empty());
        assert!(!validate_positive_field(-1, &path).is_empty());
    }

    #[test]
    fn test_validate_immutable_field() {
        let path = Path::nil();

        assert!(validate_immutable_field(&"foo", &"foo", &path).is_empty());
        assert!(!validate_immutable_field(&"foo", &"bar", &path).is_empty());
    }

    #[test]
    fn test_validate_absolute_path() {
        let path = Path::nil();

        assert!(validate_absolute_path("/foo/bar", &path).is_empty());
        assert!(!validate_absolute_path("foo/bar", &path).is_empty());
    }

    #[test]
    fn test_validate_path_no_backsteps() {
        let path = Path::nil();

        assert!(validate_path_no_backsteps("/foo/bar", &path).is_empty());
        assert!(!validate_path_no_backsteps("/foo/../bar", &path).is_empty());
        assert!(!validate_path_no_backsteps("../foo", &path).is_empty());
    }

    #[test]
    fn test_validate_ip_address() {
        let path = Path::nil();

        assert!(validate_ip_address("192.168.1.1", &path).is_empty());
        assert!(validate_ip_address("2001:db8::1", &path).is_empty());
        assert!(!validate_ip_address("invalid", &path).is_empty());
    }

    #[test]
    fn test_validate_file_mode() {
        let path = Path::nil();

        assert!(validate_file_mode(0o644, &path).is_empty());
        assert!(validate_file_mode(0o777, &path).is_empty());
        assert!(validate_file_mode(0, &path).is_empty());
        assert!(!validate_file_mode(0o1000, &path).is_empty());
        assert!(!validate_file_mode(-1, &path).is_empty());
    }
}
