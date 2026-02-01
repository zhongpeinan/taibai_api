//! Pod validation
//!
//! Validates Pod metadata and PodSpec.

use crate::common::ToInternal;
use crate::common::validation::ErrorList;
use crate::core::internal::validation::pod as internal_pod_validation;
use crate::core::v1::Pod;

/// Validates a Pod.
pub fn validate_pod(pod: &Pod) -> ErrorList {
    let internal_pod = pod.clone().to_internal();
    internal_pod_validation::validate_pod(&internal_pod)
}

/// Validates a PodSpec.
pub fn validate_pod_spec(
    spec: &crate::core::v1::PodSpec,
    path: &crate::common::validation::Path,
) -> ErrorList {
    let internal_spec = spec.clone().to_internal();
    internal_pod_validation::validate_pod_spec(&internal_spec, path)
}

/// Validates Pod update.
pub fn validate_pod_update(new: &Pod, old: &Pod) -> ErrorList {
    let internal_new = new.clone().to_internal();
    let internal_old = old.clone().to_internal();
    internal_pod_validation::validate_pod_update(&internal_new, &internal_old)
}
