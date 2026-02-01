//! PodSpec validation for Kubernetes core/v1 API.

use crate::common::ToInternal;
use crate::common::validation::{ErrorList, Path};
use crate::core::internal::validation::pod_spec as internal_pod_spec_validation;
use crate::core::v1::pod::PodSpec;

/// Validates a PodSpec.
pub fn validate_pod_spec(spec: &PodSpec, path: &Path) -> ErrorList {
    let internal_spec = spec.clone().to_internal();
    internal_pod_spec_validation::validate_pod_spec(&internal_spec, path)
}
