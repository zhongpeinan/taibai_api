//! Kubernetes Admission internal API types
//!
//! This module contains the admission internal API types.
//!
//! Note: In this project, the internal and v1 types are identical in structure.
//! The v1 types are re-exported here for convenience.

pub use crate::admission::v1::{
    AdmissionRequest, AdmissionResponse, AdmissionReview, Operation, PatchType, operation,
    patch_type,
};

#[cfg(test)]
mod tests {}
