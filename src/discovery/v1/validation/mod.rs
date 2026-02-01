//! Validation wrappers for Kubernetes Discovery API v1 types.
//!
//! These wrappers convert v1 types to internal types before validation.

use crate::common::validation::ErrorList;
use crate::common::ToInternal;
use crate::discovery::internal;

use super::{EndpointSlice, EndpointSliceList};

// ============================================================================
// EndpointSlice Validation
// ============================================================================

/// Validates a v1 EndpointSlice by converting to internal and delegating validation.
pub fn validate_endpoint_slice(obj: &EndpointSlice) -> ErrorList {
    let internal_obj = obj.clone().to_internal();
    internal::validation::validate_endpoint_slice(&internal_obj)
}

/// Validates a v1 EndpointSliceList by converting to internal and delegating validation.
pub fn validate_endpoint_slice_list(obj: &EndpointSliceList) -> ErrorList {
    let internal_obj = obj.clone().to_internal();
    internal::validation::validate_endpoint_slice_list(&internal_obj)
}

/// Validates a v1 EndpointSlice update by converting to internal and delegating validation.
pub fn validate_endpoint_slice_update(obj: &EndpointSlice, old: &EndpointSlice) -> ErrorList {
    let internal_obj = obj.clone().to_internal();
    let internal_old = old.clone().to_internal();
    internal::validation::validate_endpoint_slice_update(&internal_obj, &internal_old)
}
