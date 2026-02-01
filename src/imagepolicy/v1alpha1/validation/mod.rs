//! Validation for Kubernetes ImagePolicy v1alpha1 API types

mod image_review;

pub use image_review::{validate_image_review, validate_image_review_list};
