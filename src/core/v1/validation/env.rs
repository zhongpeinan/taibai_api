//! Environment variable validation for Kubernetes core/v1 API
//!
//! This module implements validation for environment variables and their sources.

use crate::common::validation::{BadValue, ErrorList, Path, forbidden, invalid, required};
use crate::core::v1::env::{ConfigMapEnvSource, EnvFromSource, EnvVar, SecretEnvSource};
use crate::core::v1::selector::{
    ConfigMapKeySelector, FileKeySelector, ObjectFieldSelector, ResourceFieldSelector,
    SecretKeySelector,
};
use std::collections::HashSet;
use std::sync::LazyLock;

// ============================================================================
// Constants
// ============================================================================

/// Valid field paths for env vars using downward API
static VALID_ENV_DOWNWARD_API_FIELD_PATHS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    HashSet::from([
        "metadata.name",
        "metadata.namespace",
        "metadata.uid",
        "spec.nodeName",
        "spec.serviceAccountName",
        "status.hostIP",
        "status.hostIPs",
        "status.podIP",
        "status.podIPs",
    ])
});

/// Valid resource field paths for container resources
static VALID_CONTAINER_RESOURCE_FIELD_PATHS: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| {
        HashSet::from([
            "limits.cpu",
            "limits.memory",
            "limits.ephemeral-storage",
            "requests.cpu",
            "requests.memory",
            "requests.ephemeral-storage",
        ])
    });

/// Prefixes for hugepages resource fields
const HUGEPAGES_REQUESTS_PREFIX: &str = "requests.hugepages-";
const HUGEPAGES_LIMITS_PREFIX: &str = "limits.hugepages-";

// ============================================================================
// Environment Variable Validation
// ============================================================================

/// Validates a list of environment variables.
///
/// Validates:
/// - Env var names are required and valid
/// - ValueFrom sources are properly configured
pub fn validate_env(vars: &[EnvVar], path: &Path) -> ErrorList {
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

/// Validates an ObjectFieldSelector.
///
/// Validates:
/// - apiVersion is required
/// - fieldPath is required and supported
fn validate_object_field_selector(selector: &ObjectFieldSelector, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // apiVersion is required
    if selector.api_version.is_empty() {
        all_errs.push(required(
            &path.child("apiVersion"),
            "apiVersion is required",
        ));
        return all_errs;
    }

    // fieldPath is required
    if selector.field_path.is_empty() {
        all_errs.push(required(&path.child("fieldPath"), "fieldPath is required"));
        return all_errs;
    }

    // Validate fieldPath is supported
    // Note: Full validation requires downward API conversion logic
    // For now, do basic validation against known paths
    let field_path = &selector.field_path;

    // Check if it's a direct match or a subscripted path (e.g., metadata.labels['foo'])
    let base_path = if let Some(bracket_pos) = field_path.find('[') {
        &field_path[..bracket_pos]
    } else {
        field_path.as_str()
    };

    // Check supported paths
    if !VALID_ENV_DOWNWARD_API_FIELD_PATHS.contains(base_path) {
        // Allow metadata.labels and metadata.annotations with subscripts
        if !base_path.starts_with("metadata.labels")
            && !base_path.starts_with("metadata.annotations")
        {
            let valid: Vec<&str> = VALID_ENV_DOWNWARD_API_FIELD_PATHS.iter().copied().collect();
            all_errs.push(crate::common::validation::not_supported(
                &path.child("fieldPath"),
                BadValue::String(field_path.clone()),
                &valid,
            ));
        }
    }

    all_errs
}

/// Validates a ResourceFieldSelector.
///
/// Validates:
/// - resource is required
/// - resource is supported (cpu, memory, ephemeral-storage, hugepages)
/// - containerName is required for volume sources
fn validate_container_resource_field_selector(
    selector: &ResourceFieldSelector,
    path: &Path,
    volume: bool,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // containerName is required for volume sources
    if volume && selector.container_name.is_empty() {
        all_errs.push(required(
            &path.child("containerName"),
            "containerName is required for volume sources",
        ));
    }

    // resource is required
    if selector.resource.is_empty() {
        all_errs.push(required(&path.child("resource"), "resource is required"));
    } else {
        // Check if resource is supported
        let mut is_supported =
            VALID_CONTAINER_RESOURCE_FIELD_PATHS.contains(selector.resource.as_str());

        // Also check hugepages prefixes
        if !is_supported {
            is_supported = selector.resource.starts_with(HUGEPAGES_REQUESTS_PREFIX)
                || selector.resource.starts_with(HUGEPAGES_LIMITS_PREFIX);
        }

        if !is_supported {
            let valid: Vec<&str> = VALID_CONTAINER_RESOURCE_FIELD_PATHS
                .iter()
                .copied()
                .collect();
            all_errs.push(crate::common::validation::not_supported(
                &path.child("resource"),
                BadValue::String(selector.resource.clone()),
                &valid,
            ));
        }
    }

    // TODO: Validate divisor for resource quantities (Phase 6)

    all_errs
}

/// Validates a ConfigMapKeySelector.
///
/// Validates:
/// - name is required and valid
/// - key is required and valid
fn validate_config_map_key_selector(selector: &ConfigMapKeySelector, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Name is required
    if let Some(ref name) = selector.name {
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

    // Key is required
    if selector.key.is_empty() {
        all_errs.push(required(&path.child("key"), "key is required"));
    } else if !is_valid_config_map_key(&selector.key) {
        all_errs.push(invalid(
            &path.child("key"),
            BadValue::String(selector.key.clone()),
            "invalid config map key",
        ));
    }

    all_errs
}

/// Validates a SecretKeySelector.
///
/// Validates:
/// - name is required and valid
/// - key is required and valid
fn validate_secret_key_selector(selector: &SecretKeySelector, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Name is required
    if let Some(ref name) = selector.name {
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

    // Key is required
    if selector.key.is_empty() {
        all_errs.push(required(&path.child("key"), "key is required"));
    } else if !is_valid_config_map_key(&selector.key) {
        all_errs.push(invalid(
            &path.child("key"),
            BadValue::String(selector.key.clone()),
            "invalid secret key",
        ));
    }

    all_errs
}

/// Validates a FileKeySelector.
///
/// Validates:
/// - key is required and valid
/// - volumeName is required and valid DNS label
/// - path is required and does not contain backsteps
fn validate_file_key_selector(selector: &FileKeySelector, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Key is required and must be a valid env var name
    if selector.key.is_empty() {
        all_errs.push(required(&path.child("key"), "key is required"));
    } else if !is_valid_env_var_name(&selector.key) {
        all_errs.push(invalid(
            &path.child("key"),
            BadValue::String(selector.key.clone()),
            "key must be a valid environment variable name",
        ));
    }

    // VolumeName is required and must be a DNS label
    if selector.volume_name.is_empty() {
        all_errs.push(required(
            &path.child("volumeName"),
            "volumeName is required",
        ));
    } else if !crate::common::validation::is_dns1123_label(&selector.volume_name).is_empty() {
        all_errs.push(invalid(
            &path.child("volumeName"),
            BadValue::String(selector.volume_name.clone()),
            "must be a valid DNS label",
        ));
    }

    // Path is required and must not contain backsteps
    if selector.path.is_empty() {
        all_errs.push(required(&path.child("path"), "path is required"));
    } else if selector.path.contains("..") {
        all_errs.push(invalid(
            &path.child("path"),
            BadValue::String(selector.path.clone()),
            "must not contain '..'",
        ));
    }

    all_errs
}

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
// Helper Functions
// ============================================================================

/// Validates an environment variable name.
///
/// A valid env var name:
/// - Starts with a letter or underscore
/// - Contains only alphanumeric characters and underscores
/// - Does not contain '='
fn is_valid_env_var_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    // Must start with letter or underscore
    let first_char = name.chars().next().unwrap();
    if !first_char.is_alphabetic() && first_char != '_' {
        return false;
    }

    // Rest must be alphanumeric or underscore
    for c in name.chars() {
        if !c.is_alphanumeric() && c != '_' {
            return false;
        }
    }

    true
}

/// Validates a ConfigMap/Secret key.
///
/// Keys must:
/// - Not be empty
/// - Not contain '/' or '\' or '..'
/// - Not be '.' or '..'
fn is_valid_config_map_key(key: &str) -> bool {
    if key.is_empty() || key == "." || key == ".." {
        return false;
    }

    if key.contains('/') || key.contains('\\') || key.contains("..") {
        return false;
    }

    true
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

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
