//! Environment variable validation for Kubernetes core/v1 API.

use crate::common::ToInternal;
use crate::common::validation::{ErrorList, Path};
use crate::core::internal;
use crate::core::internal::validation::env as internal_env_validation;
use crate::core::v1::env::{EnvFromSource, EnvVar};

/// Validates a list of environment variables.
pub fn validate_env(vars: &[EnvVar], path: &Path) -> ErrorList {
    let internal_vars: Vec<internal::EnvVar> =
        vars.iter().cloned().map(ToInternal::to_internal).collect();
    internal_env_validation::validate_env(&internal_vars, path)
}

/// Validates a list of EnvFromSource entries.
pub fn validate_env_from(vars: &[EnvFromSource], path: &Path) -> ErrorList {
    let internal_vars: Vec<internal::EnvFromSource> =
        vars.iter().cloned().map(ToInternal::to_internal).collect();
    internal_env_validation::validate_env_from(&internal_vars, path)
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::v1::env::{ConfigMapEnvSource, SecretEnvSource};
    use crate::core::v1::validation::selector::{is_valid_config_map_key, is_valid_env_var_name};

    #[test]
    fn test_is_valid_env_var_name() {
        assert!(is_valid_env_var_name("FOO"));
        assert!(is_valid_env_var_name("_FOO"));
        assert!(is_valid_env_var_name("FOO_BAR"));
        assert!(is_valid_env_var_name("FOO123"));

        assert!(!is_valid_env_var_name(""));
        assert!(!is_valid_env_var_name("123FOO"));
        assert!(!is_valid_env_var_name("FOO-BAR"));
        assert!(!is_valid_env_var_name("FOO.BAR"));
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
        assert!(!is_valid_config_map_key("foo/bar"));
        assert!(!is_valid_config_map_key("foo\\bar"));
        assert!(!is_valid_config_map_key("foo..bar"));
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
