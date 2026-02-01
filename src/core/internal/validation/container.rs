//! Container validation wrappers for internal core API types.

use crate::common::validation::{ErrorList, Path};
use crate::core::internal::InternalContainer;
use crate::core::v1::validation::container as v1_container_validation;
use crate::core::v1::volume::VolumeSource;
use std::collections::{HashMap, HashSet};

pub fn validate_containers(
    containers: &[InternalContainer],
    volumes: &HashMap<String, VolumeSource>,
    pod_claim_names: &HashSet<String>,
    grace_period: &Option<i64>,
    path: &Path,
) -> ErrorList {
    v1_container_validation::validate_containers(
        containers,
        volumes,
        pod_claim_names,
        grace_period,
        path,
    )
}

pub fn validate_init_containers(
    init_containers: &[InternalContainer],
    containers: &[InternalContainer],
    volumes: &HashMap<String, VolumeSource>,
    pod_claim_names: &HashSet<String>,
    grace_period: &Option<i64>,
    path: &Path,
) -> ErrorList {
    v1_container_validation::validate_init_containers(
        init_containers,
        containers,
        volumes,
        pod_claim_names,
        grace_period,
        path,
    )
}
