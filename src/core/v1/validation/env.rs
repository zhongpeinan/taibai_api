//! Environment variable validation for Kubernetes core/v1 API
//!
//! This module implements validation for environment variables and their sources.

use crate::common::validation::{BadValue, ErrorList, Path, invalid, required};
use crate::core::internal::validation::env as internal_env_validation;
use crate::core::v1::env::{ConfigMapEnvSource, EnvFromSource, EnvVar, SecretEnvSource};
use crate::core::v1::validation::selector::{
    is_valid_env_var_name, validate_config_map_key_selector,
    validate_container_resource_field_selector, validate_file_key_selector,
    validate_object_field_selector, validate_secret_key_selector,
};

// ============================================================================
// Environment Variable Validation
// ============================================================================

/// Validates a list of environment variables.
///
/// Validates:
/// - Env var names are required and valid
/// - ValueFrom sources are properly configured
pub fn validate_env(vars: &[EnvVar], path: &Path) -> ErrorList {
    internal_env_validation::validate_env(vars, path)
}

pub(crate) fn validate_env_v1(vars: &[EnvVar], path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (i, ev) in vars.iter().enumerate() {
        let idx_path = path.index(i);

        // Validate env var name
        if ev.name.is_empty() {
            all_errs.push(required(&idx_path.child("name"), "name is required"));
        } else if !is_valid_env_var_name(&ev.name) {
            all_errs.push(invalid(
                &idx_path.child("name"),
                BadValue::String(ev.name.clone()),
                "invalid environment variable name",
            ));
        }

        // Validate valueFrom
        all_errs.extend(validate_env_var_value_from(
            ev,
            &idx_path.child("valueFrom"),
        ));
    }

    all_errs
}

/// Validates an EnvVarSource.
///
/// Rules:
/// - Exactly one source type must be specified
/// - Cannot specify valueFrom when value is set
fn validate_env_var_value_from(ev: &EnvVar, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    let Some(ref value_from) = ev.value_from else {
        return all_errs;
    };

    let mut num_sources = 0;

    // FieldRef
    if let Some(ref field_ref) = value_from.field_ref {
        num_sources += 1;
        all_errs.extend(validate_object_field_selector(
            field_ref,
            &path.child("fieldRef"),
        ));
    }

    // ResourceFieldRef
    if let Some(ref resource_field_ref) = value_from.resource_field_ref {
        num_sources += 1;
        all_errs.extend(validate_container_resource_field_selector(
            resource_field_ref,
            &path.child("resourceFieldRef"),
            false, // Not a volume source
        ));
    }

    // ConfigMapKeyRef
    if let Some(ref config_map_key_ref) = value_from.config_map_key_ref {
        num_sources += 1;
        all_errs.extend(validate_config_map_key_selector(
            config_map_key_ref,
            &path.child("configMapKeyRef"),
        ));
    }

    // SecretKeyRef
    if let Some(ref secret_key_ref) = value_from.secret_key_ref {
        num_sources += 1;
        all_errs.extend(validate_secret_key_selector(
            secret_key_ref,
            &path.child("secretKeyRef"),
        ));
    }

    // FileKeyRef
    if let Some(ref file_key_ref) = value_from.file_key_ref {
        num_sources += 1;
        all_errs.extend(validate_file_key_selector(
            file_key_ref,
            &path.child("fileKeyRef"),
        ));
    }

    // Validate source count
    if num_sources == 0 {
        all_errs.push(invalid(
            path,
            BadValue::String(String::new()),
            "must specify one of: fieldRef, resourceFieldRef, configMapKeyRef, secretKeyRef, or fileKeyRef",
        ));
    } else if !ev.value.is_empty() && num_sources > 0 {
        all_errs.push(invalid(
            path,
            BadValue::String(String::new()),
            "may not be specified when value is not empty",
        ));
    } else if num_sources > 1 {
        all_errs.push(invalid(
            path,
            BadValue::String(String::new()),
            "may not have more than one field specified at a time",
        ));
    }

    all_errs
}

// ============================================================================
// EnvFrom Validation
// ============================================================================

/// Validates a list of EnvFromSource entries.
///
/// Validates:
/// - Prefix is a valid env var name
/// - Exactly one source type (configMapRef or secretRef)
pub fn validate_env_from(vars: &[EnvFromSource], path: &Path) -> ErrorList {
    internal_env_validation::validate_env_from(vars, path)
}

pub(crate) fn validate_env_from_v1(vars: &[EnvFromSource], path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (i, ev) in vars.iter().enumerate() {
        let idx_path = path.index(i);

        // Validate prefix
        if !ev.prefix.is_empty() && !is_valid_env_var_name(&ev.prefix) {
            all_errs.push(invalid(
                &idx_path.child("prefix"),
                BadValue::String(ev.prefix.clone()),
                "invalid environment variable prefix",
            ));
        }

        // Validate source (mutual exclusion)
        let mut num_sources = 0;

        if let Some(ref config_map_ref) = ev.config_map_ref {
            num_sources += 1;
            all_errs.extend(validate_config_map_env_source(
                config_map_ref,
                &idx_path.child("configMapRef"),
            ));
        }

        if let Some(ref secret_ref) = ev.secret_ref {
            num_sources += 1;
            all_errs.extend(validate_secret_env_source(
                secret_ref,
                &idx_path.child("secretRef"),
            ));
        }

        if num_sources == 0 {
            all_errs.push(invalid(
                &idx_path,
                BadValue::String(String::new()),
                "must specify one of: configMapRef or secretRef",
            ));
        } else if num_sources > 1 {
            all_errs.push(invalid(
                &idx_path,
                BadValue::String(String::new()),
                "may not have more than one field specified at a time",
            ));
        }
    }

    all_errs
}

// ============================================================================
// Source Validation Functions
// ============================================================================

/// Validates a ConfigMapEnvSource.
fn validate_config_map_env_source(source: &ConfigMapEnvSource, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Extract name from LocalObjectReference
    if let Some(ref name) = source.local_object_reference.name {
        if name.is_empty() {
            all_errs.push(required(&path.child("name"), "name is required"));
        } else if !crate::common::validation::is_dns1123_subdomain(name).is_empty() {
            all_errs.push(invalid(
                &path.child("name"),
                BadValue::String(name.clone()),
                "must be a valid DNS subdomain",
            ));
        }
    } else {
        all_errs.push(required(&path.child("name"), "name is required"));
    }

    all_errs
}

/// Validates a SecretEnvSource.
fn validate_secret_env_source(source: &SecretEnvSource, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Extract name from LocalObjectReference
    if let Some(ref name) = source.local_object_reference.name {
        if name.is_empty() {
            all_errs.push(required(&path.child("name"), "name is required"));
        } else if !crate::common::validation::is_dns1123_subdomain(name).is_empty() {
            all_errs.push(invalid(
                &path.child("name"),
                BadValue::String(name.clone()),
                "must be a valid DNS subdomain",
            ));
        }
    } else {
        all_errs.push(required(&path.child("name"), "name is required"));
    }

    all_errs
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::v1::validation::selector::{is_valid_config_map_key, is_valid_env_var_name};

    #[test]
    fn test_is_valid_env_var_name() {
        assert!(is_valid_env_var_name("FOO"));
        assert!(is_valid_env_var_name("_FOO"));
        assert!(is_valid_env_var_name("FOO_BAR"));
        assert!(is_valid_env_var_name("FOO123"));

        assert!(!is_valid_env_var_name(""));
        assert!(!is_valid_env_var_name("123FOO")); // Starts with digit
        assert!(!is_valid_env_var_name("FOO-BAR")); // Contains hyphen
        assert!(!is_valid_env_var_name("FOO.BAR")); // Contains dot
    }

    #[test]
    fn test_is_valid_config_map_key() {
        assert!(is_valid_config_map_key("foo"));
        assert!(is_valid_config_map_key("foo-bar"));
        assert!(is_valid_config_map_key("foo_bar"));
        assert!(is_valid_config_map_key("foo.bar"));

        assert!(!is_valid_config_map_key(""));
        assert!(!is_valid_config_map_key("."));
        assert!(!is_valid_config_map_key(".."));
        assert!(!is_valid_config_map_key("foo/bar")); // Contains /
        assert!(!is_valid_config_map_key("foo\\bar")); // Contains \
        assert!(!is_valid_config_map_key("foo..bar")); // Contains ..
    }

    #[test]
    fn test_validate_env_empty_name() {
        let vars = vec![EnvVar {
            name: String::new(),
            value: "value".to_string(),
            value_from: None,
        }];

        let errs = validate_env(&vars, &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.detail.contains("name is required"))
        );
    }

    #[test]
    fn test_validate_env_invalid_name() {
        let vars = vec![EnvVar {
            name: "123-invalid".to_string(),
            value: "value".to_string(),
            value_from: None,
        }];

        let errs = validate_env(&vars, &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.detail.contains("invalid environment variable name"))
        );
    }

    #[test]
    fn test_validate_env_from_no_source() {
        let vars = vec![EnvFromSource {
            prefix: String::new(),
            config_map_ref: None,
            secret_ref: None,
        }];

        let errs = validate_env_from(&vars, &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.detail.contains("must specify one of"))
        );
    }

    #[test]
    fn test_validate_env_from_multiple_sources() {
        let vars = vec![EnvFromSource {
            prefix: String::new(),
            config_map_ref: Some(ConfigMapEnvSource::new("config".to_string())),
            secret_ref: Some(SecretEnvSource::new("secret".to_string())),
        }];

        let errs = validate_env_from(&vars, &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.detail.contains("may not have more than one"))
        );
    }
}
