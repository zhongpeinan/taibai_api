//! Kubernetes Certificates API types
//!
//! This module provides Rust representations of Kubernetes Certificates API types,
//! supporting both JSON (via serde) serialization.
//!
//! The module is organized into:
//! - `v1`: The v1 version of the Certificates API
//! - `v1alpha1`: The v1alpha1 version of the Certificates API
//! - `internal`: Internal types used within Kubernetes

pub mod internal;
pub mod v1;
pub mod v1alpha1;
pub mod v1beta1;
pub mod validation;

// Re-export commonly used v1 types
pub use v1::{
    CertificateSigningRequest, CertificateSigningRequestCondition, CertificateSigningRequestList,
    CertificateSigningRequestSpec, CertificateSigningRequestStatus, ExtraValue, KeyUsage,
    RequestConditionType,
};
pub use v1alpha1::{
    ClusterTrustBundle as ClusterTrustBundleV1Alpha1,
    ClusterTrustBundleList as ClusterTrustBundleListV1Alpha1,
    ClusterTrustBundleSpec as ClusterTrustBundleSpecV1Alpha1,
    PodCertificateRequest as PodCertificateRequestV1Alpha1,
    PodCertificateRequestList as PodCertificateRequestListV1Alpha1,
    PodCertificateRequestSpec as PodCertificateRequestSpecV1Alpha1,
    PodCertificateRequestStatus as PodCertificateRequestStatusV1Alpha1,
};
pub use v1beta1::{
    CertificateSigningRequest as CertificateSigningRequestV1Beta1,
    CertificateSigningRequestCondition as CertificateSigningRequestConditionV1Beta1,
    CertificateSigningRequestList as CertificateSigningRequestListV1Beta1,
    CertificateSigningRequestSpec as CertificateSigningRequestSpecV1Beta1,
    CertificateSigningRequestStatus as CertificateSigningRequestStatusV1Beta1,
    ClusterTrustBundle as ClusterTrustBundleV1Beta1,
    ClusterTrustBundleList as ClusterTrustBundleListV1Beta1,
    ClusterTrustBundleSpec as ClusterTrustBundleSpecV1Beta1, ExtraValue as ExtraValueV1Beta1,
    KeyUsage as KeyUsageV1Beta1, RequestConditionType as RequestConditionTypeV1Beta1,
};

// Re-export internal-only types
pub use internal::{
    ClusterTrustBundle, ClusterTrustBundleList, ClusterTrustBundleSpec, PodCertificateRequest,
    PodCertificateRequestList, PodCertificateRequestSpec, PodCertificateRequestStatus,
};
