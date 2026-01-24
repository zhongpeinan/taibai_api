//! Kubernetes ImagePolicy API types
//!
//! This module provides Rust representations of Kubernetes ImagePolicy API types.
//!
//! The ImagePolicy API is used for image review webhooks that can validate
//! whether images in a pod are allowed to run. These webhooks are part of
//! Kubernetes admission control and allow custom validation of container images.
//!
//! ## Use Cases
//!
//! Image review webhooks can be used to:
//! - Enforce image registry policies (e.g., only allow images from specific registries)
//! - Validate image signatures and attestations
//! - Check for known vulnerabilities in images
//! - Enforce image tag policies (e.g., require specific tags or forbid `:latest`)
//!
//! ## Webhook Flow
//!
//! 1. A pod creation request is submitted to Kubernetes
//! 2. The image review webhook is called with an `ImageReview` resource
//! 3. The webhook evaluates the images and returns `ImageReviewStatus`
//! 4. If `allowed: false`, the pod creation is rejected
//!
//! The module is organized into:
//! - `v1alpha1`: The v1alpha1 version of the ImagePolicy API
//! - `internal`: Internal types used within Kubernetes

pub mod internal;
pub mod v1alpha1;
pub mod validation;

// Re-export commonly used v1alpha1 types
pub use v1alpha1::{
    ImageReview, ImageReviewContainerSpec, ImageReviewList, ImageReviewSpec, ImageReviewStatus,
};
