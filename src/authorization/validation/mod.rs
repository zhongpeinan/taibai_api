//! Validation for Kubernetes Authorization API types
//!
//! Wrapper around v1 validation.

pub use crate::authorization::v1::validation::{
    validate_local_subject_access_review, validate_self_subject_access_review,
    validate_subject_access_review,
};
