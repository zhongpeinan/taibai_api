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
mod tests {
    use super::*;

    // ========================================================================
    // Compile-time Trait Checks
    // ========================================================================

    /// 编译时检查：确保内部版本资源实现了 prost::Message
    ///
    /// Note: internal types are re-exported from v1alpha1, so they already
    /// implement all necessary traits.
    #[test]
    fn prost_message() {
        fn check<T: prost::Message>() {}

        check::<ImageReview>();
        check::<ImageReviewList>();
    }
}
