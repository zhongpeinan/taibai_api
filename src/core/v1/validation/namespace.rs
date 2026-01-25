//! Namespace validation

use crate::common::validation::ErrorList;
use crate::core::v1::Namespace;

/// Validates a Namespace
pub fn validate_namespace(_namespace: &Namespace) -> ErrorList {
    ErrorList::new()
    // TODO: Implement in Phase 6
}

/// Validates Namespace update
pub fn validate_namespace_update(_new: &Namespace, _old: &Namespace) -> ErrorList {
    ErrorList::new()
    // TODO: Implement in Phase 6
}
