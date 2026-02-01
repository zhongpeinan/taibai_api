//! Validation wrappers for Kubernetes Scheduling API v1 types.
//!
//! These wrappers convert v1 types to internal types before validation.

use crate::common::ToInternal;
use crate::common::validation::ErrorList;
use crate::scheduling::internal;

use super::{PriorityClass, PriorityClassList};

// ============================================================================
// PriorityClass Validation
// ============================================================================

/// Validates a v1 PriorityClass by converting to internal and delegating validation.
pub fn validate_priority_class(obj: &PriorityClass) -> ErrorList {
    let internal_obj = obj.clone().to_internal();
    internal::validation::validate_priority_class(&internal_obj)
}

/// Validates a v1 PriorityClassList by converting to internal and delegating validation.
pub fn validate_priority_class_list(obj: &PriorityClassList) -> ErrorList {
    let internal_obj = obj.clone().to_internal();
    internal::validation::validate_priority_class_list(&internal_obj)
}

/// Validates a v1 PriorityClass update by converting to internal and delegating validation.
pub fn validate_priority_class_update(obj: &PriorityClass, old: &PriorityClass) -> ErrorList {
    let internal_obj = obj.clone().to_internal();
    let internal_old = old.clone().to_internal();
    internal::validation::validate_priority_class_update(&internal_obj, &internal_old)
}
