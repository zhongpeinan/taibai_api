//! Pod validation
//!
//! Validates Pod and container specifications.

use crate::common::validation::{ErrorList, Path};
use crate::core::v1::{Pod, PodSpec};

/// Validates a Pod
pub fn validate_pod(_pod: &Pod) -> ErrorList {
    ErrorList::new()
    // TODO: Implement in Phase 3
}

/// Validates a PodSpec
pub fn validate_pod_spec(_spec: &PodSpec, _path: &Path) -> ErrorList {
    ErrorList::new()
    // TODO: Implement in Phase 3
}

/// Validates Pod update
pub fn validate_pod_update(_new: &Pod, _old: &Pod) -> ErrorList {
    ErrorList::new()
    // TODO: Implement in Phase 3
}
