//! PodSpec validation for Kubernetes core internal API types.

use crate::common::FromInternal;
use crate::common::validation::{ErrorList, Path};
use crate::core::internal::PodSpec;

/// Validates a PodSpec using internal types.
pub fn validate_pod_spec(spec: &PodSpec, path: &Path) -> ErrorList {
    let v1_spec = crate::core::v1::pod::PodSpec::from_internal(spec.clone());
    crate::core::v1::validation::pod_spec::validate_pod_spec(&v1_spec, path)
}
