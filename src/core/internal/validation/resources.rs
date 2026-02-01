//! Resource validation wrappers for internal core API types.

use crate::common::FromInternal;
use crate::common::validation::{ErrorList, Path};
use crate::core::internal::ResourceRequirements;
use crate::core::v1::resource::ResourceRequirements as V1ResourceRequirements;
use crate::core::v1::validation::resources as v1_resources_validation;
use std::collections::HashSet;

pub fn validate_pod_resource_requirements(
    resources: &ResourceRequirements,
    pod_claim_names: &HashSet<String>,
    path: &Path,
) -> ErrorList {
    let v1_resources = V1ResourceRequirements::from_internal(resources.clone());
    v1_resources_validation::validate_pod_resource_requirements_v1(
        &v1_resources,
        pod_claim_names,
        path,
    )
}

pub fn validate_container_resource_requirements(
    resources: &ResourceRequirements,
    pod_claim_names: &HashSet<String>,
    path: &Path,
) -> ErrorList {
    let v1_resources = V1ResourceRequirements::from_internal(resources.clone());
    v1_resources_validation::validate_container_resource_requirements_v1(
        &v1_resources,
        pod_claim_names,
        path,
    )
}
