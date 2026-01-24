//! Kubernetes Admission API types
//!
//! This module contains the admission API types.

pub mod internal;
pub mod v1;

pub use v1::{
    AdmissionRequest, AdmissionResponse, AdmissionReview, Operation, PatchType, operation,
    patch_type,
};

// Re-export internal types for convenience
pub use internal::{
    AdmissionRequest as InternalAdmissionRequest, AdmissionResponse as InternalAdmissionResponse,
    AdmissionReview as InternalAdmissionReview,
};
