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
mod tests {
    use super::*;

    #[test]
    fn test_internal_types_exist() {
        // Test that we can create instances of re-exported types
        let _req = AdmissionRequest::default();
        let _resp = AdmissionResponse::default();
        let _review = AdmissionReview::default();
    }

    #[test]
    fn test_constants_reexported() {
        assert_eq!(operation::CREATE, "CREATE");
        assert_eq!(patch_type::JSON_PATCH, "JSONPatch");
    }
}
