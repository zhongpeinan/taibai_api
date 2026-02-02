//! Container validation for Kubernetes core/v1 API
//!
//! This module implements validation for containers and orchestrates validation
//! of container components (probes, env, ports, resources, volume mounts).

use crate::common::ToInternal;
use crate::common::validation::{ErrorList, Path};
use crate::core::internal::validation::container as internal_container_validation;
use crate::core::internal::validation::container_ports::accumulate_unique_host_ports;
use crate::core::internal::validation::helpers::validate_container_name as internal_validate_container_name;
use crate::core::v1::pod::Container;
use crate::core::v1::volume::VolumeSource;
use std::collections::{HashMap, HashSet};

/// Validates a single container (for regular containers in pod spec).
pub fn validate_container(
    container: &Container,
    volumes: &HashMap<String, VolumeSource>,
    pod_claim_names: &HashSet<String>,
    grace_period: &Option<i64>,
    path: &Path,
) -> ErrorList {
    let internal_volumes: HashMap<String, crate::core::internal::VolumeSource> = volumes
        .iter()
        .map(|(name, source)| (name.clone(), source.clone().to_internal()))
        .collect();
    internal_container_validation::validate_container(
        container,
        &internal_volumes,
        pod_claim_names,
        grace_period,
        path,
    )
}

/// Validates a list of regular containers.
pub fn validate_containers(
    containers: &[Container],
    volumes: &HashMap<String, VolumeSource>,
    pod_claim_names: &HashSet<String>,
    grace_period: &Option<i64>,
    path: &Path,
) -> ErrorList {
    let internal_volumes: HashMap<String, crate::core::internal::VolumeSource> = volumes
        .iter()
        .map(|(name, source)| (name.clone(), source.clone().to_internal()))
        .collect();
    internal_container_validation::validate_containers(
        containers,
        &internal_volumes,
        pod_claim_names,
        grace_period,
        path,
    )
}

/// Validates a list of init containers.
pub fn validate_init_containers(
    init_containers: &[Container],
    regular_containers: &[Container],
    volumes: &HashMap<String, VolumeSource>,
    pod_claim_names: &HashSet<String>,
    grace_period: &Option<i64>,
    path: &Path,
) -> ErrorList {
    let internal_volumes: HashMap<String, crate::core::internal::VolumeSource> = volumes
        .iter()
        .map(|(name, source)| (name.clone(), source.clone().to_internal()))
        .collect();
    internal_container_validation::validate_init_containers(
        init_containers,
        regular_containers,
        &internal_volumes,
        pod_claim_names,
        grace_period,
        path,
    )
}

/// Validates container name.
pub fn validate_container_name(name: &str, path: &Path) -> ErrorList {
    internal_validate_container_name(name, path)
}

/// Validates ports for a list of containers.
pub fn validate_ports_for_containers(containers: &[Container], path: &Path) -> ErrorList {
    let port_sets: Vec<Vec<crate::core::internal::ContainerPort>> = containers
        .iter()
        .map(|c| {
            c.ports
                .iter()
                .cloned()
                .map(ToInternal::to_internal)
                .collect()
        })
        .collect();
    let port_slices: Vec<&[crate::core::internal::ContainerPort]> =
        port_sets.iter().map(|ports| ports.as_slice()).collect();
    accumulate_unique_host_ports(&port_slices, path)
}
