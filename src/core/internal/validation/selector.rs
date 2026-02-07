//! Selector validation for Kubernetes core internal API.
//!
//! Ported from k8s.io/kubernetes/pkg/apis/core/validation/validation.go

use crate::common::Quantity;
use crate::common::validation::{BadValue, ErrorList, Path, invalid, not_supported, required};
use crate::core::internal::selector::{
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

/// Valid CPU divisor values
static VALID_CPU_DIVISORS: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| HashSet::from(["1m", "1"]));

/// Valid memory/ephemeral-storage/hugepages divisor values
static VALID_MEMORY_DIVISORS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    HashSet::from([
        "1", "1k", "1M", "1G", "1T", "1P", "1E", "1Ki", "1Mi", "1Gi", "1Ti", "1Pi", "1Ei",
    ])
});

// ============================================================================
// Selector Validation
// ============================================================================

/// Validates an ObjectFieldSelector.
///
/// Validates:
/// - apiVersion is required
/// - fieldPath is required and supported
pub fn validate_object_field_selector(selector: &ObjectFieldSelector, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if selector.api_version.is_empty() {
        all_errs.push(required(
            &path.child("apiVersion"),
            "apiVersion is required",
        ));
        return all_errs;
    }

    if selector.field_path.is_empty() {
        all_errs.push(required(&path.child("fieldPath"), "fieldPath is required"));
        return all_errs;
    }

    let field_path = &selector.field_path;

    let base_path = if let Some(bracket_pos) = field_path.find('[') {
        &field_path[..bracket_pos]
    } else {
        field_path.as_str()
    };

    if !VALID_ENV_DOWNWARD_API_FIELD_PATHS.contains(base_path) {
        if !base_path.starts_with("metadata.labels")
            && !base_path.starts_with("metadata.annotations")
        {
            let valid: Vec<&str> = VALID_ENV_DOWNWARD_API_FIELD_PATHS.iter().copied().collect();
            all_errs.push(not_supported(
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
pub fn validate_container_resource_field_selector(
    selector: &ResourceFieldSelector,
    path: &Path,
    volume: bool,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if volume && selector.container_name.is_empty() {
        all_errs.push(required(
            &path.child("containerName"),
            "containerName is required for volume sources",
        ));
    }

    if selector.resource.is_empty() {
        all_errs.push(required(&path.child("resource"), "resource is required"));
    } else {
        let mut is_supported =
            VALID_CONTAINER_RESOURCE_FIELD_PATHS.contains(selector.resource.as_str());

        if !is_supported {
            is_supported = selector.resource.starts_with(HUGEPAGES_REQUESTS_PREFIX)
                || selector.resource.starts_with(HUGEPAGES_LIMITS_PREFIX);
        }

        if !is_supported {
            let valid: Vec<&str> = VALID_CONTAINER_RESOURCE_FIELD_PATHS
                .iter()
                .copied()
                .collect();
            all_errs.push(not_supported(
                &path.child("resource"),
                BadValue::String(selector.resource.clone()),
                &valid,
            ));
        }
    }

    // Validate divisor for resource quantities
    if let Some(ref divisor) = selector.divisor {
        all_errs.extend(validate_container_resource_divisor(
            &selector.resource,
            divisor,
            path,
        ));
    }

    all_errs
}

/// Validates a resource field divisor.
///
/// Corresponds to [upstream validateContainerResourceDivisor](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/core/validation/validation.go)
fn validate_container_resource_divisor(
    resource_name: &str,
    divisor: &Quantity,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    let divisor_str = divisor.to_string();

    // Empty/zero divisor is allowed (means default of 1)
    if divisor_str == "0" || divisor_str.is_empty() {
        return all_errs;
    }

    match resource_name {
        "limits.cpu" | "requests.cpu" => {
            if !VALID_CPU_DIVISORS.contains(divisor_str.as_str()) {
                all_errs.push(invalid(
                    &path.child("divisor"),
                    BadValue::String(resource_name.to_string()),
                    "only divisor's values 1m and 1 are supported with the cpu resource",
                ));
            }
        }
        "limits.memory" | "requests.memory" => {
            if !VALID_MEMORY_DIVISORS.contains(divisor_str.as_str()) {
                all_errs.push(invalid(
                    &path.child("divisor"),
                    BadValue::String(resource_name.to_string()),
                    "only divisor's values 1, 1k, 1M, 1G, 1T, 1P, 1E, 1Ki, 1Mi, 1Gi, 1Ti, 1Pi, 1Ei are supported with the memory resource",
                ));
            }
        }
        "limits.ephemeral-storage" | "requests.ephemeral-storage" => {
            if !VALID_MEMORY_DIVISORS.contains(divisor_str.as_str()) {
                all_errs.push(invalid(
                    &path.child("divisor"),
                    BadValue::String(resource_name.to_string()),
                    "only divisor's values 1, 1k, 1M, 1G, 1T, 1P, 1E, 1Ki, 1Mi, 1Gi, 1Ti, 1Pi, 1Ei are supported with the local ephemeral storage resource",
                ));
            }
        }
        r if r.starts_with(HUGEPAGES_REQUESTS_PREFIX) || r.starts_with(HUGEPAGES_LIMITS_PREFIX) => {
            if !VALID_MEMORY_DIVISORS.contains(divisor_str.as_str()) {
                all_errs.push(invalid(
                    &path.child("divisor"),
                    BadValue::String(resource_name.to_string()),
                    "only divisor's values 1, 1k, 1M, 1G, 1T, 1P, 1E, 1Ki, 1Mi, 1Gi, 1Ti, 1Pi, 1Ei are supported with the hugepages resource",
                ));
            }
        }
        _ => {} // Unknown resource, skip divisor validation
    }

    all_errs
}

/// Validates a ConfigMapKeySelector.
///
/// Validates:
/// - name is required and valid
/// - key is required and valid
pub fn validate_config_map_key_selector(selector: &ConfigMapKeySelector, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

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
pub fn validate_secret_key_selector(selector: &SecretKeySelector, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

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
pub fn validate_file_key_selector(selector: &FileKeySelector, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if selector.key.is_empty() {
        all_errs.push(required(&path.child("key"), "key is required"));
    } else if !is_valid_env_var_name(&selector.key) {
        all_errs.push(invalid(
            &path.child("key"),
            BadValue::String(selector.key.clone()),
            "key must be a valid environment variable name",
        ));
    }

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

// ============================================================================
// Helper Functions
// ============================================================================

/// Validates an environment variable name.
///
/// A valid env var name:
/// - Starts with a letter or underscore
/// - Contains only alphanumeric characters and underscores
/// - Does not contain '='
pub(crate) fn is_valid_env_var_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    let first_char = name.chars().next().unwrap();
    if !first_char.is_alphabetic() && first_char != '_' {
        return false;
    }

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
/// - Not contain '/' or '\\' or '..'
/// - Not be '.' or '..'
pub(crate) fn is_valid_config_map_key(key: &str) -> bool {
    if key.is_empty() || key == "." || key == ".." {
        return false;
    }

    if key.contains('/') || key.contains('\\') || key.contains("..") {
        return false;
    }

    true
}
