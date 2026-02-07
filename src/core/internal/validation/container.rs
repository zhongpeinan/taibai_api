//! Container validation for Kubernetes core internal API
//!
//! This module implements validation for containers and orchestrates validation
//! of container components (probes, env, ports, resources, volume mounts).

use crate::common::ToInternal;
use crate::common::validation::{
    BadValue, ErrorList, Path, forbidden, invalid, not_supported, required,
};
use crate::core::internal::InternalContainer as Container;
use crate::core::internal::validation::container_ports::{
    accumulate_unique_host_ports, validate_container_ports,
};
use crate::core::internal::validation::env::{validate_env, validate_env_from};
use crate::core::internal::validation::helpers::validate_container_name;
use crate::core::internal::validation::probe::{
    validate_lifecycle, validate_liveness_probe, validate_readiness_probe, validate_startup_probe,
};
use crate::core::internal::validation::resources::validate_container_resource_requirements;
use crate::core::internal::validation::volume::{validate_volume_devices, validate_volume_mounts};
use crate::core::internal::{ContainerPort, EnvFromSource, EnvVar, VolumeSource};
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

// ============================================================================
// Constants
// ============================================================================

/// Supported image pull policies
static SUPPORTED_PULL_POLICIES: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| HashSet::from(["Always", "IfNotPresent", "Never"]));

/// Supported termination message policies
static SUPPORTED_TERMINATION_MESSAGE_POLICIES: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| HashSet::from(["File", "FallbackToLogsOnError"]));

// ============================================================================
// Container Validation
// ============================================================================

/// Validates a single container (for regular containers in pod spec).
///
/// This performs full container validation including:
/// - Common container validation (name, image, ports, env, resources, etc.)
/// - Pod-specific validation (image whitespace check)
/// - Lifecycle hooks validation
/// - Probe validation (liveness, readiness, startup)
pub fn validate_container(
    container: &Container,
    volumes: &HashMap<String, VolumeSource>,
    pod_claim_names: &HashSet<String>,
    grace_period: &Option<i64>,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Common validation for all container types
    all_errs.extend(validate_container_common(
        container,
        volumes,
        pod_claim_names,
        path,
    ));

    // Pod-specific validation
    all_errs.extend(validate_container_only_for_pod(container, path));

    let internal_lifecycle = container.lifecycle.clone().map(ToInternal::to_internal);
    all_errs.extend(validate_lifecycle(
        internal_lifecycle.as_ref(),
        grace_period,
        &path.child("lifecycle"),
    ));

    // Probe validation
    let internal_liveness = container
        .liveness_probe
        .clone()
        .map(ToInternal::to_internal);
    let internal_readiness = container
        .readiness_probe
        .clone()
        .map(ToInternal::to_internal);
    let internal_startup = container.startup_probe.clone().map(ToInternal::to_internal);

    all_errs.extend(validate_liveness_probe(
        internal_liveness.as_ref(),
        grace_period,
        &path.child("livenessProbe"),
    ));
    all_errs.extend(validate_readiness_probe(
        internal_readiness.as_ref(),
        grace_period,
        &path.child("readinessProbe"),
    ));
    all_errs.extend(validate_startup_probe(
        internal_startup.as_ref(),
        grace_period,
        &path.child("startupProbe"),
    ));

    all_errs
}

/// Validates container fields common to all container types.
///
/// This is used for regular containers, init containers, and ephemeral containers.
/// It validates:
/// - Name (DNS label, required)
/// - Image (required)
/// - Ports
/// - Environment variables
/// - Volume mounts and devices
/// - Image pull policy
/// - Resources
/// - Security context
/// - Termination message path and policy
pub fn validate_container_common(
    container: &Container,
    volumes: &HashMap<String, VolumeSource>,
    pod_claim_names: &HashSet<String>,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate container name (required, DNS label)
    let name_path = path.child("name");
    if container.name.is_empty() {
        all_errs.push(required(&name_path, "container name is required"));
    } else {
        all_errs.extend(validate_container_name(&container.name, &name_path));
    }

    // Validate image (required)
    if let Some(ref image) = container.image {
        if image.is_empty() {
            all_errs.push(required(&path.child("image"), "image is required"));
        }
    } else {
        all_errs.push(required(&path.child("image"), "image is required"));
    }

    // Validate termination message policy
    if let Some(ref policy) = container.termination_message_policy {
        all_errs.extend(validate_termination_message_policy(
            policy,
            &path.child("terminationMessagePolicy"),
        ));
    } else {
        all_errs.push(required(
            &path.child("terminationMessagePolicy"),
            "terminationMessagePolicy is required",
        ));
    }

    // Validate container ports
    if !container.ports.is_empty() {
        let internal_ports: Vec<ContainerPort> = container
            .ports
            .iter()
            .cloned()
            .map(ToInternal::to_internal)
            .collect();
        all_errs.extend(validate_container_ports(
            &internal_ports,
            &path.child("ports"),
        ));
    }

    // Validate environment variables
    if !container.env.is_empty() {
        let internal_env: Vec<EnvVar> = container
            .env
            .iter()
            .cloned()
            .map(ToInternal::to_internal)
            .collect();
        all_errs.extend(validate_env(&internal_env, &path.child("env")));
    }

    // Validate environment from sources
    if !container.env_from.is_empty() {
        let internal_env_from: Vec<EnvFromSource> = container
            .env_from
            .iter()
            .cloned()
            .map(ToInternal::to_internal)
            .collect();
        all_errs.extend(validate_env_from(
            &internal_env_from,
            &path.child("envFrom"),
        ));
    }

    let vol_devices: HashMap<String, String> = container
        .volume_devices
        .iter()
        .map(|dev| (dev.name.clone(), dev.device_path.clone()))
        .collect();
    let vol_mounts: HashMap<String, String> = container
        .volume_mounts
        .iter()
        .map(|mnt| (mnt.name.clone(), mnt.mount_path.clone()))
        .collect();
    let internal_mounts: Vec<crate::core::internal::VolumeMount> = container
        .volume_mounts
        .iter()
        .cloned()
        .map(|mnt| mnt.to_internal())
        .collect();
    let internal_devices: Vec<crate::core::internal::VolumeDevice> = container
        .volume_devices
        .iter()
        .cloned()
        .map(|dev| dev.to_internal())
        .collect();

    all_errs.extend(validate_volume_mounts(
        &internal_mounts,
        &vol_devices,
        volumes,
        container,
        &path.child("volumeMounts"),
    ));

    all_errs.extend(validate_volume_devices(
        &internal_devices,
        &vol_mounts,
        volumes,
        &path.child("volumeDevices"),
    ));

    // Validate image pull policy
    if let Some(ref policy) = container.image_pull_policy {
        all_errs.extend(validate_pull_policy(policy, &path.child("imagePullPolicy")));
    }

    // Validate resource requirements
    if let Some(ref resources) = container.resources {
        let internal_resources = resources.clone().to_internal();
        all_errs.extend(validate_container_resource_requirements(
            &internal_resources,
            pod_claim_names,
            &path.child("resources"),
        ));
    }

    // Validate security context
    if let Some(ref sc) = container.security_context {
        all_errs.extend(
            crate::core::internal::validation::security::validate_security_context(
                sc,
                &path.child("securityContext"),
            ),
        );
    }

    all_errs
}

/// Validates container fields that are only checked for pods (not templates).
///
/// Currently validates:
/// - Image field must not have leading or trailing whitespace
fn validate_container_only_for_pod(container: &Container, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if let Some(ref image) = container.image {
        let trimmed = image.trim();
        if image != trimmed {
            all_errs.push(invalid(
                &path.child("image"),
                BadValue::String(image.clone()),
                "must not have leading or trailing whitespace",
            ));
        }
    }

    all_errs
}

/// Validates a list of regular containers.
///
/// Validates:
/// - At least one container must be present
/// - Container names must be unique
/// - Each container's fields are valid
/// - Host port conflicts across all containers
pub fn validate_containers(
    containers: &[Container],
    volumes: &HashMap<String, VolumeSource>,
    pod_claim_names: &HashSet<String>,
    grace_period: &Option<i64>,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // At least one container is required
    if containers.is_empty() {
        all_errs.push(required(path, "must specify at least one container"));
        return all_errs;
    }

    let mut all_names = HashSet::new();

    for (i, container) in containers.iter().enumerate() {
        let idx_path = path.index(i);

        // Validate the container
        all_errs.extend(validate_container(
            container,
            volumes,
            pod_claim_names,
            grace_period,
            &idx_path,
        ));

        // Check for duplicate container names
        if !container.name.is_empty() {
            if all_names.contains(&container.name) {
                all_errs.push(crate::common::validation::duplicate(
                    &idx_path.child("name"),
                    BadValue::String(container.name.clone()),
                ));
            } else {
                all_names.insert(container.name.clone());
            }
        }
    }

    // Check for host port conflicts across all containers
    let port_sets: Vec<Vec<ContainerPort>> = containers
        .iter()
        .map(|c| {
            c.ports
                .iter()
                .cloned()
                .map(ToInternal::to_internal)
                .collect()
        })
        .collect();
    let port_slices: Vec<&[ContainerPort]> =
        port_sets.iter().map(|ports| ports.as_slice()).collect();
    all_errs.extend(accumulate_unique_host_ports(&port_slices, path));

    all_errs
}

/// Validates a list of init containers.
///
/// Init containers have similar validation to regular containers, but:
/// - Lifecycle hooks, probes are only allowed with restartPolicy=Always
/// - Names must be unique within init containers AND not conflict with regular containers
/// - Host port conflicts are checked per init container (they run one-by-one)
pub fn validate_init_containers(
    init_containers: &[Container],
    regular_containers: &[Container],
    volumes: &HashMap<String, VolumeSource>,
    pod_claim_names: &HashSet<String>,
    _grace_period: &Option<i64>,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Collect names from regular containers
    let mut all_names: HashSet<String> =
        regular_containers.iter().map(|c| c.name.clone()).collect();

    for (i, container) in init_containers.iter().enumerate() {
        let idx_path = path.index(i);

        // Validate common container fields
        all_errs.extend(validate_container_common(
            container,
            volumes,
            pod_claim_names,
            &idx_path,
        ));

        // Check for duplicate names (must be unique across regular + init containers)
        if !container.name.is_empty() {
            if all_names.contains(&container.name) {
                all_errs.push(crate::common::validation::duplicate(
                    &idx_path.child("name"),
                    BadValue::String(container.name.clone()),
                ));
            } else {
                all_names.insert(container.name.clone());
            }
        }

        // Init containers run one-by-one, so check host port conflicts individually
        let port_sets: Vec<Vec<ContainerPort>> = vec![
            container
                .ports
                .iter()
                .cloned()
                .map(ToInternal::to_internal)
                .collect(),
        ];
        let port_slices: Vec<&[ContainerPort]> =
            port_sets.iter().map(|ports| ports.as_slice()).collect();
        all_errs.extend(accumulate_unique_host_ports(&port_slices, &path));

        // For now, disallow lifecycle and probes in init containers
        // TODO: Add support for restartPolicy=Always init containers in later phase
        if container.lifecycle.is_some() {
            all_errs.push(forbidden(
                &idx_path.child("lifecycle"),
                "may not be set for init containers without restartPolicy=Always",
            ));
        }
        if container.liveness_probe.is_some() {
            all_errs.push(forbidden(
                &idx_path.child("livenessProbe"),
                "may not be set for init containers without restartPolicy=Always",
            ));
        }
        if container.readiness_probe.is_some() {
            all_errs.push(forbidden(
                &idx_path.child("readinessProbe"),
                "may not be set for init containers without restartPolicy=Always",
            ));
        }
        if container.startup_probe.is_some() {
            all_errs.push(forbidden(
                &idx_path.child("startupProbe"),
                "may not be set for init containers without restartPolicy=Always",
            ));
        }
    }

    all_errs
}

// ============================================================================
// Helper Validators
// ============================================================================

/// Validates image pull policy.
fn validate_pull_policy(policy: &str, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if policy.is_empty() {
        all_errs.push(required(path, "imagePullPolicy is required"));
    } else if !SUPPORTED_PULL_POLICIES.contains(policy) {
        let valid: Vec<&str> = SUPPORTED_PULL_POLICIES.iter().copied().collect();
        all_errs.push(not_supported(
            path,
            BadValue::String(policy.to_string()),
            &valid,
        ));
    }

    all_errs
}

/// Validates termination message policy.
fn validate_termination_message_policy(policy: &str, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if policy.is_empty() {
        all_errs.push(required(path, "terminationMessagePolicy is required"));
    } else if !SUPPORTED_TERMINATION_MESSAGE_POLICIES.contains(policy) {
        let valid: Vec<&str> = SUPPORTED_TERMINATION_MESSAGE_POLICIES
            .iter()
            .copied()
            .collect();
        all_errs.push(not_supported(
            path,
            BadValue::String(policy.to_string()),
            &valid,
        ));
    }

    all_errs
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_container_missing_name() {
        let container = Container {
            name: String::new(),
            image: Some("nginx:latest".to_string()),
            ..Default::default()
        };

        let errs = validate_container(
            &container,
            &HashMap::new(),
            &HashSet::new(),
            &None,
            &Path::nil(),
        );
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.detail.contains("container name is required"))
        );
    }

    #[test]
    fn test_validate_container_missing_image() {
        let container = Container {
            name: "nginx".to_string(),
            image: None,
            ..Default::default()
        };

        let errs = validate_container(
            &container,
            &HashMap::new(),
            &HashSet::new(),
            &None,
            &Path::nil(),
        );
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.detail.contains("image is required"))
        );
    }

    #[test]
    fn test_validate_container_empty_image() {
        let container = Container {
            name: "nginx".to_string(),
            image: Some(String::new()),
            ..Default::default()
        };

        let errs = validate_container(
            &container,
            &HashMap::new(),
            &HashSet::new(),
            &None,
            &Path::nil(),
        );
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.detail.contains("image is required"))
        );
    }

    #[test]
    fn test_validate_container_image_with_whitespace() {
        let container = Container {
            name: "nginx".to_string(),
            image: Some(" nginx:latest ".to_string()),
            termination_message_policy: Some("File".to_string()),
            ..Default::default()
        };

        let errs = validate_container_only_for_pod(&container, &Path::nil());
        assert!(!errs.is_empty());
        assert!(errs.errors.iter().any(|e| {
            e.detail
                .contains("must not have leading or trailing whitespace")
        }));
    }

    #[test]
    fn test_validate_pull_policy_invalid() {
        let errs = validate_pull_policy("InvalidPolicy", &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.error_type == crate::common::validation::ErrorType::NotSupported)
        );
    }

    #[test]
    fn test_validate_pull_policy_valid() {
        assert!(validate_pull_policy("Always", &Path::nil()).is_empty());
        assert!(validate_pull_policy("IfNotPresent", &Path::nil()).is_empty());
        assert!(validate_pull_policy("Never", &Path::nil()).is_empty());
    }

    #[test]
    fn test_validate_termination_message_policy_invalid() {
        let errs = validate_termination_message_policy("InvalidPolicy", &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.error_type == crate::common::validation::ErrorType::NotSupported)
        );
    }

    #[test]
    fn test_validate_termination_message_policy_valid() {
        assert!(validate_termination_message_policy("File", &Path::nil()).is_empty());
        assert!(
            validate_termination_message_policy("FallbackToLogsOnError", &Path::nil()).is_empty()
        );
    }

    #[test]
    fn test_validate_containers_empty_list() {
        let errs = validate_containers(&[], &HashMap::new(), &HashSet::new(), &None, &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.detail.contains("must specify at least one container"))
        );
    }

    #[test]
    fn test_validate_containers_duplicate_names() {
        let containers = vec![
            Container {
                name: "nginx".to_string(),
                image: Some("nginx:latest".to_string()),
                termination_message_policy: Some("File".to_string()),
                ..Default::default()
            },
            Container {
                name: "nginx".to_string(),
                image: Some("nginx:alpine".to_string()),
                termination_message_policy: Some("File".to_string()),
                ..Default::default()
            },
        ];

        let errs = validate_containers(
            &containers,
            &HashMap::new(),
            &HashSet::new(),
            &None,
            &Path::nil(),
        );
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.error_type == crate::common::validation::ErrorType::Duplicate)
        );
    }

    #[test]
    fn test_validate_container_valid() {
        let container = Container {
            name: "nginx".to_string(),
            image: Some("nginx:latest".to_string()),
            termination_message_policy: Some("File".to_string()),
            image_pull_policy: Some("Always".to_string()),
            ..Default::default()
        };

        let errs = validate_container(
            &container,
            &HashMap::new(),
            &HashSet::new(),
            &None,
            &Path::nil(),
        );
        assert!(errs.is_empty(), "Valid container should not produce errors");
    }

    #[test]
    fn test_validate_init_containers_lifecycle_forbidden() {
        let init_container = Container {
            name: "init".to_string(),
            image: Some("busybox:latest".to_string()),
            termination_message_policy: Some("File".to_string()),
            lifecycle: Some(Default::default()),
            ..Default::default()
        };

        let errs = validate_init_containers(
            &[init_container],
            &[],
            &HashMap::new(),
            &HashSet::new(),
            &None,
            &Path::nil(),
        );
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.detail.contains("may not be set for init containers"))
        );
    }

    #[test]
    fn test_validate_init_containers_duplicate_with_regular() {
        let regular_container = Container {
            name: "nginx".to_string(),
            image: Some("nginx:latest".to_string()),
            termination_message_policy: Some("File".to_string()),
            ..Default::default()
        };

        let init_container = Container {
            name: "nginx".to_string(),
            image: Some("busybox:latest".to_string()),
            termination_message_policy: Some("File".to_string()),
            ..Default::default()
        };

        let errs = validate_init_containers(
            &[init_container],
            &[regular_container],
            &HashMap::new(),
            &HashSet::new(),
            &None,
            &Path::nil(),
        );
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.error_type == crate::common::validation::ErrorType::Duplicate)
        );
    }
}
