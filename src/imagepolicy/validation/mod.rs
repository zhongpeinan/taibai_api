//! Validation for Kubernetes ImagePolicy API types
//!
//! Wrapper around versioned/internal validation.

pub use crate::imagepolicy::v1alpha1::validation::{
    validate_image_review, validate_image_review_list,
};
