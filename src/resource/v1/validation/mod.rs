//! Validation wrappers for Kubernetes Resource API v1 types.
//!
//! These wrappers convert v1 types to internal types before validation.

use crate::common::ToInternal;
use crate::common::validation::ErrorList;
use crate::resource::internal;

use super::{DeviceClass, ResourceClaim, ResourceClaimTemplate, ResourceSlice};

// ============================================================================
// DeviceClass Validation
// ============================================================================

/// Validates a v1 DeviceClass by converting to internal and delegating validation.
pub fn validate_device_class(obj: &DeviceClass) -> ErrorList {
    let internal_obj = obj.clone().to_internal();
    internal::validation::validate_device_class(&internal_obj)
}

/// Validates a v1 DeviceClass update by converting to internal and delegating validation.
pub fn validate_device_class_update(obj: &DeviceClass, old: &DeviceClass) -> ErrorList {
    let internal_obj = obj.clone().to_internal();
    let internal_old = old.clone().to_internal();
    internal::validation::validate_device_class_update(&internal_obj, &internal_old)
}

// ============================================================================
// ResourceClaim Validation
// ============================================================================

/// Validates a v1 ResourceClaim by converting to internal and delegating validation.
pub fn validate_resource_claim(obj: &ResourceClaim) -> ErrorList {
    let internal_obj = obj.clone().to_internal();
    internal::validation::validate_resource_claim(&internal_obj)
}

/// Validates a v1 ResourceClaim update by converting to internal and delegating validation.
pub fn validate_resource_claim_update(obj: &ResourceClaim, old: &ResourceClaim) -> ErrorList {
    let internal_obj = obj.clone().to_internal();
    let internal_old = old.clone().to_internal();
    internal::validation::validate_resource_claim_update(&internal_obj, &internal_old)
}

// ============================================================================
// ResourceClaimTemplate Validation
// ============================================================================

/// Validates a v1 ResourceClaimTemplate by converting to internal and delegating validation.
pub fn validate_resource_claim_template(obj: &ResourceClaimTemplate) -> ErrorList {
    let internal_obj = obj.clone().to_internal();
    internal::validation::validate_resource_claim_template(&internal_obj)
}

/// Validates a v1 ResourceClaimTemplate update by converting to internal and delegating validation.
pub fn validate_resource_claim_template_update(
    obj: &ResourceClaimTemplate,
    old: &ResourceClaimTemplate,
) -> ErrorList {
    let internal_obj = obj.clone().to_internal();
    let internal_old = old.clone().to_internal();
    internal::validation::validate_resource_claim_template_update(&internal_obj, &internal_old)
}

// ============================================================================
// ResourceSlice Validation
// ============================================================================

/// Validates a v1 ResourceSlice by converting to internal and delegating validation.
pub fn validate_resource_slice(obj: &ResourceSlice) -> ErrorList {
    let internal_obj = obj.clone().to_internal();
    internal::validation::validate_resource_slice(&internal_obj)
}

/// Validates a v1 ResourceSlice update by converting to internal and delegating validation.
pub fn validate_resource_slice_update(obj: &ResourceSlice, old: &ResourceSlice) -> ErrorList {
    let internal_obj = obj.clone().to_internal();
    let internal_old = old.clone().to_internal();
    internal::validation::validate_resource_slice_update(&internal_obj, &internal_old)
}
