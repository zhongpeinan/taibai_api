//! Validation for Kubernetes Autoscaling v1 API types
//!
//! Wrapper around internal validation (v1 -> internal -> validate)

use crate::autoscaling::internal::validation as internal_validation;
use crate::common::ToInternal;
use crate::common::validation::ErrorList;

use super::{HorizontalPodAutoscaler, Scale};

pub use crate::autoscaling::validation::{
    CrossVersionObjectReferenceValidationOptions, HorizontalPodAutoscalerSpecValidationOptions,
};

// =============================================================================
// Scale Validation
// =============================================================================

pub fn validate_scale(scale: &Scale) -> ErrorList {
    internal_validation::validate_scale(&scale.clone().to_internal())
}

// =============================================================================
// HorizontalPodAutoscaler Validation
// =============================================================================

pub fn validate_horizontal_pod_autoscaler(
    autoscaler: &HorizontalPodAutoscaler,
    opts: &HorizontalPodAutoscalerSpecValidationOptions,
) -> ErrorList {
    internal_validation::validate_horizontal_pod_autoscaler(&autoscaler.clone().to_internal(), opts)
}

pub fn validate_horizontal_pod_autoscaler_update(
    new_autoscaler: &HorizontalPodAutoscaler,
    old_autoscaler: &HorizontalPodAutoscaler,
    opts: &HorizontalPodAutoscalerSpecValidationOptions,
) -> ErrorList {
    internal_validation::validate_horizontal_pod_autoscaler_update(
        &new_autoscaler.clone().to_internal(),
        &old_autoscaler.clone().to_internal(),
        opts,
    )
}

pub fn validate_horizontal_pod_autoscaler_status_update(
    new_autoscaler: &HorizontalPodAutoscaler,
    old_autoscaler: &HorizontalPodAutoscaler,
) -> ErrorList {
    internal_validation::validate_horizontal_pod_autoscaler_status_update(
        &new_autoscaler.clone().to_internal(),
        &old_autoscaler.clone().to_internal(),
    )
}
