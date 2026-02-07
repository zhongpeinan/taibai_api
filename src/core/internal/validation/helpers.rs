//! Helper validation wrappers for internal core API types.

use crate::common::validation::{BadValue, ErrorList, Path, invalid, is_dns1123_label};

pub fn validate_container_name(name: &str, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    for msg in is_dns1123_label(name) {
        all_errs.push(invalid(path, BadValue::String(name.to_string()), &msg));
    }
    all_errs
}

/// Validates a ConfigMap or Secret data key.
///
/// Keys must follow these rules:
/// - Must be non-empty and no more than 253 characters
/// - Consist of alphanumeric characters, '-', '_' or '.'
/// - Start and end with an alphanumeric character
///
/// Ported from k8s.io/kubernetes/pkg/apis/core/validation/validation.go
pub(crate) fn is_config_map_key(key: &str) -> Vec<String> {
    let mut errors = Vec::new();

    if key.is_empty() {
        errors.push("must be non-empty".to_string());
        return errors;
    }

    if key.len() > 253 {
        errors.push(format!(
            "must be no more than 253 characters (got {})",
            key.len()
        ));
    }

    if let Some(first) = key.chars().next() {
        if !first.is_alphanumeric() {
            errors.push("must start with an alphanumeric character".to_string());
        }
    }

    if let Some(last) = key.chars().last() {
        if !last.is_alphanumeric() {
            errors.push("must end with an alphanumeric character".to_string());
        }
    }

    for ch in key.chars() {
        if !ch.is_alphanumeric() && ch != '-' && ch != '_' && ch != '.' {
            errors.push(format!(
                "must consist of alphanumeric characters, '-', '_' or '.' (invalid character: '{}')",
                ch
            ));
            break;
        }
    }

    errors
}
