//! Validation for Kubernetes Authentication API types
//!
//! Wrapper around versioned/internal validation.

pub use crate::authentication::v1::validation::{validate_token_request, validate_token_review};
