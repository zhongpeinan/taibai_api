//! Shared helpers for flowcontrol validation.

use std::collections::HashSet;

use crate::common::validation::{
    BadValue, Error, ErrorList, Path, invalid, is_dns1123_label, is_dns1123_subdomain,
    not_supported, required,
};

pub(crate) const FLOW_SCHEMA_MAX_MATCHING_PRECEDENCE: i32 = 10_000;
pub(crate) const PRIORITY_LEVEL_CONFIGURATION_QUEUING_MAX_QUEUES: i32 = 10_000_000;

pub(crate) const FLOW_SCHEMA_DEFAULT_MATCHING_PRECEDENCE: i32 = 1000;
pub(crate) const PRIORITY_LEVEL_CONFIGURATION_DEFAULT_HAND_SIZE: i32 = 8;
pub(crate) const PRIORITY_LEVEL_CONFIGURATION_DEFAULT_QUEUES: i32 = 64;
pub(crate) const PRIORITY_LEVEL_CONFIGURATION_DEFAULT_QUEUE_LENGTH_LIMIT: i32 = 50;
pub(crate) const PRIORITY_LEVEL_CONFIGURATION_DEFAULT_NOMINAL_CONCURRENCY_SHARES: i32 = 30;

pub(crate) const MAX_HASH_BITS: i32 = 60;

pub(crate) const SUPPORTED_VERBS: [&str; 9] = [
    "get",
    "list",
    "create",
    "update",
    "delete",
    "deletecollection",
    "patch",
    "watch",
    "proxy",
];

pub(crate) const SUPPORTED_SUBJECT_KINDS: [&str; 3] = ["ServiceAccount", "Group", "User"];

pub(crate) const SUPPORTED_PRIORITY_LEVEL_ENABLEMENT: [&str; 2] = ["Exempt", "Limited"];

pub(crate) const SUPPORTED_LIMIT_RESPONSE_TYPE: [&str; 2] = ["Queue", "Reject"];

pub(crate) const NS_ERR_INTRO: &str = "each member of this list must be '*' or a DNS-1123 label; ";

pub(crate) fn validate_namespace_name(name: &str, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    for msg in is_dns1123_label(name) {
        all_errs.push(invalid(path, BadValue::String(name.to_string()), &msg));
    }
    all_errs
}

pub(crate) fn validate_service_account_name(name: &str, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    for msg in is_dns1123_subdomain(name) {
        all_errs.push(invalid(path, BadValue::String(name.to_string()), &msg));
    }
    all_errs
}

pub(crate) fn validate_supported_list(
    values: &[String],
    supported: &[&str],
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let supported_set: HashSet<&str> = supported.iter().copied().collect();
    let mut bad = Vec::new();
    for value in values {
        if !supported_set.contains(value.as_str()) {
            bad.push(value.clone());
        }
    }
    if !bad.is_empty() {
        all_errs.push(not_supported(
            path,
            BadValue::String(format!("{:?}", values)),
            supported,
        ));
    }
    all_errs
}

pub(crate) fn validate_non_resource_url_path(path_value: &str, path: &Path) -> Option<Error> {
    if path_value.is_empty() {
        return Some(invalid(
            path,
            BadValue::String(path_value.to_string()),
            "must not be empty",
        ));
    }
    if path_value == "/" {
        return None;
    }
    if !path_value.starts_with('/') {
        return Some(invalid(
            path,
            BadValue::String(path_value.to_string()),
            "must start with slash",
        ));
    }
    if path_value.contains(' ') {
        return Some(invalid(
            path,
            BadValue::String(path_value.to_string()),
            "must not contain white-space",
        ));
    }
    if path_value.contains("//") {
        return Some(invalid(
            path,
            BadValue::String(path_value.to_string()),
            "must not contain double slash",
        ));
    }
    let wildcard_count = path_value.matches('*').count();
    if wildcard_count > 1 || (wildcard_count == 1 && !path_value.ends_with("/*")) {
        return Some(invalid(
            path,
            BadValue::String(path_value.to_string()),
            "wildcard can only do suffix matching",
        ));
    }
    None
}

pub(crate) fn has_wildcard(values: &[String]) -> bool {
    member_in_list("*", values)
}

pub(crate) fn member_in_list(seek: &str, values: &[String]) -> bool {
    values.iter().any(|value| value == seek)
}

pub(crate) fn required_entropy_bits(deck_size: i32, hand_size: i32) -> i32 {
    if deck_size <= 0 || hand_size <= 0 {
        return 0;
    }
    let bits = (deck_size as f64).log2() * (hand_size as f64);
    bits.ceil() as i32
}

pub(crate) fn required_field<T>(value: &Option<T>, path: &Path, message: &str) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if value.is_none() {
        all_errs.push(required(path, message));
    }
    all_errs
}
