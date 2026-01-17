//! Kubernetes Certificates API types
//!
//! This module provides Rust representations of Kubernetes Certificates API types,
//! supporting both JSON (via serde) serialization.
//!
//! The module is organized into:
//! - `v1`: The v1 version of the Certificates API
//! - `internal`: Internal types used within Kubernetes

pub mod internal;
pub mod v1;

// Re-export commonly used v1 types
pub use v1::{
    CertificateSigningRequest, CertificateSigningRequestCondition, CertificateSigningRequestList,
    CertificateSigningRequestSpec, CertificateSigningRequestStatus, ExtraValue, KeyUsage,
    RequestConditionType,
};

// Re-export internal-only types
pub use internal::{
    ClusterTrustBundle, ClusterTrustBundleList, ClusterTrustBundleSpec, PodCertificateRequest,
    PodCertificateRequestList, PodCertificateRequestSpec, PodCertificateRequestStatus,
};
