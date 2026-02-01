//! Validation for Kubernetes Certificates API types
//!
//! Wrapper around versioned/internal validation.

pub use crate::certificates::internal::validation::{
    validate_cluster_trust_bundle, validate_pod_certificate_request,
};
pub use crate::certificates::v1::validation::validate_certificate_signing_request;
