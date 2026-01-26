//! Certificates internal API types
//!
//! This module contains the internal types for the Kubernetes Certificates API.
//! Internal types have non-optional metadata fields, matching upstream internal API shapes.
//!
//! Source: k8s-pkg/apis/certificates/types.go

use crate::common::meta::Condition;
use crate::common::{ListMeta, ObjectMeta, TypeMeta};
use crate::impl_has_object_meta;
use serde::{Deserialize, Serialize};

// Re-export v1 types that are identical
pub use crate::certificates::v1::{
    CertificateSigningRequestCondition, CertificateSigningRequestSpec,
    CertificateSigningRequestStatus, ExtraValue, KeyUsage, RequestConditionType,
};

// ============================================================================
// CertificateSigningRequest (Internal)
// ============================================================================

/// Internal representation of CertificateSigningRequest.
///
/// Differs from v1 in that metadata and status are non-optional.
///
/// Source: k8s/pkg/apis/certificates/types.go
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CertificateSigningRequest {
    /// Type metadata (api_version, kind)
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard object metadata (non-optional in internal API)
    pub metadata: ObjectMeta,

    /// The certificate request itself and any additional information
    #[serde(default)]
    pub spec: CertificateSigningRequestSpec,

    /// Derived information about the request
    #[serde(default)]
    pub status: CertificateSigningRequestStatus,
}

impl_has_object_meta!(CertificateSigningRequest);

/// Internal representation of CertificateSigningRequestList.
///
/// Differs from v1 in that metadata is non-optional.
///
/// Source: k8s/pkg/apis/certificates/types.go
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CertificateSigningRequestList {
    /// Type metadata (api_version, kind)
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard list metadata (non-optional in internal API)
    pub metadata: ListMeta,

    /// List of certificate signing requests
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<CertificateSigningRequest>,
}

// ============================================================================
// Cluster Trust Bundle
// ============================================================================

/// ClusterTrustBundle is a cluster-scoped container for X.509 trust anchors (root certificates).
///
/// ClusterTrustBundle objects are considered to be readable by any authenticated user in the cluster.
///
/// It can be optionally associated with a particular assigner, in which case it contains one valid
/// set of trust anchors for that signer. Signers may have multiple associated ClusterTrustBundles;
/// each is an independent set of trust anchors for that signer.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClusterTrustBundle {
    /// Standard object metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::common::ObjectMeta>,

    /// Spec contains the signer (if any) and trust anchors.
    #[serde(default)]
    pub spec: ClusterTrustBundleSpec,
}

/// ClusterTrustBundleSpec contains the signer and trust anchors.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClusterTrustBundleSpec {
    /// SignerName indicates the associated signer, if any.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub signer_name: String,

    /// TrustBundle contains the individual X.509 trust anchors for this bundle,
    /// as PEM bundle of PEM-wrapped, DER-formatted X.509 certificates.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub trust_bundle: String,
}

/// ClusterTrustBundleList is a collection of ClusterTrustBundle objects.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClusterTrustBundleList {
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::common::ListMeta>,

    /// Items is a collection of ClusterTrustBundle objects.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ClusterTrustBundle>,
}

// MaxTrustBundleSize is the maximum size of a single trust bundle field.
pub const MAX_TRUST_BUNDLE_SIZE: usize = 1024 * 1024;

// ============================================================================
// Pod Certificate Request
// ============================================================================

/// PodCertificateRequest encodes a pod requesting a certificate from a given signer.
///
/// Kubelets use this API to implement podCertificate projected volumes.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodCertificateRequest {
    /// Standard object metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::common::ObjectMeta>,

    /// Spec contains the details about the certificate being requested.
    #[serde(default)]
    pub spec: PodCertificateRequestSpec,

    /// Status contains the issued certificate, and a standard set of conditions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PodCertificateRequestStatus>,
}

/// PodCertificateRequestSpec describes the certificate request. All fields are immutable after creation.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodCertificateRequestSpec {
    /// SignerName indicates the requested signer.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub signer_name: String,

    /// PodName is the name of the pod into which the certificate will be mounted.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub pod_name: String,

    /// PodUID is the UID of the pod into which the certificate will be mounted.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub pod_uid: String,

    /// ServiceAccountName is the name of the service account the pod is running as.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub service_account_name: String,

    /// ServiceAccountUID is the UID of the service account the pod is running as.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub service_account_uid: String,

    /// NodeName is the name of the node the pod is assigned to.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub node_name: String,

    /// NodeUID is the UID of the node the pod is assigned to.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub node_uid: String,

    /// MaxExpirationSeconds is the maximum lifetime permitted for the certificate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_expiration_seconds: Option<i32>,

    /// PKIXPublicKey is the PKIX-serialized public key the signer will issue the certificate to.
    #[serde(default)]
    pub pkix_public_key: crate::core::internal::helper::ByteString,

    /// ProofOfPossession proves that the requesting kubelet holds the private key.
    #[serde(default)]
    pub proof_of_possession: crate::core::internal::helper::ByteString,
}

/// PodCertificateRequestStatus contains the issued certificate and conditions.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodCertificateRequestStatus {
    /// Conditions applied to the request. Known conditions are "Denied", "Failed", and "Issued".
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<Condition>,

    /// CertificateChain is populated with an issued certificate by the signer.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub certificate_chain: String,

    /// NotBefore is the time at which the certificate becomes valid.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not_before: Option<crate::common::Timestamp>,

    /// BeginRefreshAt is the time at which the kubelet should begin trying to refresh the certificate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub begin_refresh_at: Option<crate::common::Timestamp>,

    /// NotAfter is the time at which the certificate expires.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not_after: Option<crate::common::Timestamp>,
}

/// PodCertificateRequestList is a collection of PodCertificateRequest objects.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodCertificateRequestList {
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::common::ListMeta>,

    /// Items is a collection of PodCertificateRequest objects.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<PodCertificateRequest>,
}

// Well-known condition types for PodCertificateRequests
pub const POD_CERTIFICATE_REQUEST_CONDITION_TYPE_DENIED: &str = "Denied";
pub const POD_CERTIFICATE_REQUEST_CONDITION_TYPE_FAILED: &str = "Failed";
pub const POD_CERTIFICATE_REQUEST_CONDITION_TYPE_ISSUED: &str = "Issued";

// Well-known condition reasons for PodCertificateRequests
pub const POD_CERTIFICATE_REQUEST_CONDITION_UNSUPPORTED_KEY_TYPE: &str = "UnsupportedKeyType";

// Constants for PodCertificateRequest field limits
pub const MAX_PKIX_PUBLIC_KEY_SIZE: usize = 10 * 1024;
pub const MAX_PROOF_OF_POSSESSION_SIZE: usize = 10 * 1024;
pub const MAX_CERTIFICATE_CHAIN_SIZE: usize = 100 * 1024;
pub const MIN_MAX_EXPIRATION_SECONDS: i32 = 60 * 60;
pub const MAX_MAX_EXPIRATION_SECONDS: i32 = 91 * 24 * 60 * 60;
pub const KUBERNETES_MAX_MAX_EXPIRATION_SECONDS: i32 = 24 * 60 * 60;

// ============================================================================
// Signer Name Constants (Internal)
// ============================================================================

/// Built in signerName values that are honored by kube-controller-manager.
pub const KUBE_APISERVER_CLIENT_SIGNER_NAME: &str = "kubernetes.io/kube-apiserver-client";
pub const KUBE_APISERVER_CLIENT_KUBELET_SIGNER_NAME: &str =
    "kubernetes.io/kube-apiserver-client-kubelet";
pub const KUBELET_SERVING_SIGNER_NAME: &str = "kubernetes.io/kubelet-serving";
pub const LEGACY_UNKNOWN_SIGNER_NAME: &str = "kubernetes.io/legacy-unknown";

// ============================================================================
// Condition Types
// ============================================================================

/// RequestConditionType is the type of a CertificateSigningRequestCondition.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "PascalCase")]
pub enum RequestConditionTypeInternal {
    /// Approved indicates the request was approved and should be issued by the signer.
    Approved,
    /// Denied indicates the request was denied and should not be issued by the signer.
    Denied,
    /// Failed indicates the signer failed to issue the certificate.
    Failed,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {}
