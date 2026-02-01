//! Validation wrappers for Kubernetes Node API v1 types.
//!
//! These wrappers convert v1 types to internal types before validation.

use crate::common::ToInternal;
use crate::common::validation::ErrorList;
use crate::node::internal;

use super::{RuntimeClass, RuntimeClassList};

// ============================================================================
// RuntimeClass Validation
// ============================================================================

/// Validates a v1 RuntimeClass by converting to internal and delegating validation.
pub fn validate_runtime_class(obj: &RuntimeClass) -> ErrorList {
    let internal_obj = obj.clone().to_internal();
    internal::validation::validate_runtime_class(&internal_obj)
}

/// Validates a v1 RuntimeClassList by converting to internal and delegating validation.
pub fn validate_runtime_class_list(obj: &RuntimeClassList) -> ErrorList {
    let internal_obj = obj.clone().to_internal();
    internal::validation::validate_runtime_class_list(&internal_obj)
}

/// Validates a v1 RuntimeClass update by converting to internal and delegating validation.
pub fn validate_runtime_class_update(obj: &RuntimeClass, old: &RuntimeClass) -> ErrorList {
    let internal_obj = obj.clone().to_internal();
    let internal_old = old.clone().to_internal();
    internal::validation::validate_runtime_class_update(&internal_obj, &internal_old)
}
