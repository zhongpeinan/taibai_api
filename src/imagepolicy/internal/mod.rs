//! ImagePolicy internal API types
//!
//! This module contains the internal types for the Kubernetes ImagePolicy API.
//!
//! The imagepolicy API is used for image review webhooks that can validate
//! whether images in a pod are allowed to run.
//!
//! Source: k8s-pkg/apis/imagepolicy/types.go

// Re-export all v1alpha1 types (internal and external types are essentially the same)
pub use crate::imagepolicy::v1alpha1::{
    IMAGE_POLICY_ANNOTATION_PREFIX, ImageReview, ImageReviewContainerSpec, ImageReviewList,
    ImageReviewSpec, ImageReviewStatus,
};

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {}
