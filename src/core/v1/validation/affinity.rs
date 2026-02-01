//! Affinity validation for Kubernetes core/v1 API.
//!
//! Delegates to internal validation for consistency.

use crate::common::ToInternal;
use crate::common::validation::{ErrorList, Path};
use crate::core::internal::validation::affinity as internal_affinity_validation;
use crate::core::v1::affinity::Affinity;

pub fn validate_affinity(affinity: &Affinity, path: &Path) -> ErrorList {
    let internal_affinity = affinity.clone().to_internal();
    internal_affinity_validation::validate_affinity(&internal_affinity, path)
}
