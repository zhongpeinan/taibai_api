//! Certificates v1alpha1 API types
//!
//! This module contains the v1alpha1 version of the Kubernetes Certificates API types.
//!
//! Source: api-master/certificates/v1alpha1/types.go

pub mod conversion;
pub mod defaults;

use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

use crate::common::meta::Condition;
use crate::common::{HasTypeMeta, ListMeta, ObjectMeta, ResourceSchema, TypeMeta, VersionedObject};
use crate::core::internal::helper::ByteString;
use crate::impl_unimplemented_prost_message;

// ============================================================================
// Cluster Trust Bundle
// ============================================================================

/// ClusterTrustBundle is a cluster-scoped container for X.509 trust anchors (root certificates).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClusterTrustBundle {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard object metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// spec contains the signer (if any) and trust anchors.
    #[serde(default)]
    pub spec: ClusterTrustBundleSpec,
}

/// ClusterTrustBundleSpec contains the signer and trust anchors.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClusterTrustBundleSpec {
    /// signerName indicates the associated signer, if any.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub signer_name: String,

    /// trustBundle contains the individual X.509 trust anchors for this bundle.
    pub trust_bundle: String,
}

/// ClusterTrustBundleList is a collection of ClusterTrustBundle objects.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClusterTrustBundleList {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// items is a collection of ClusterTrustBundle objects.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ClusterTrustBundle>,
}

// ============================================================================
// Pod Certificate Request
// ============================================================================

/// PodCertificateRequest encodes a pod requesting a certificate from a given signer.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodCertificateRequest {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard object metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// spec contains the details about the certificate being requested.
    #[serde(default)]
    pub spec: PodCertificateRequestSpec,

    /// status contains the issued certificate, and a standard set of conditions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PodCertificateRequestStatus>,
}

/// PodCertificateRequestSpec describes the certificate request.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodCertificateRequestSpec {
    /// signerName indicates the requested signer.
    pub signer_name: String,

    /// podName is the name of the pod into which the certificate will be mounted.
    pub pod_name: String,

    /// podUID is the UID of the pod into which the certificate will be mounted.
    #[serde(rename = "podUID")]
    pub pod_uid: String,

    /// serviceAccountName is the name of the service account the pod is running as.
    pub service_account_name: String,

    /// serviceAccountUID is the UID of the service account the pod is running as.
    #[serde(rename = "serviceAccountUID")]
    pub service_account_uid: String,

    /// nodeName is the name of the node the pod is assigned to.
    pub node_name: String,

    /// nodeUID is the UID of the node the pod is assigned to.
    #[serde(rename = "nodeUID")]
    pub node_uid: String,

    /// maxExpirationSeconds is the maximum lifetime permitted for the certificate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_expiration_seconds: Option<i32>,

    /// pkixPublicKey is the PKIX-serialized public key.
    #[serde(default)]
    pub pkix_public_key: ByteString,

    /// proofOfPossession proves that the requesting kubelet holds the private key.
    #[serde(default)]
    pub proof_of_possession: ByteString,
}

/// PodCertificateRequestStatus describes the status of the request.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodCertificateRequestStatus {
    /// conditions applied to the request.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<Condition>,

    /// certificateChain is populated with an issued certificate by the signer.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub certificate_chain: String,

    /// notBefore is the time at which the certificate becomes valid.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not_before: Option<crate::common::Timestamp>,

    /// beginRefreshAt is the time at which the kubelet should begin trying to refresh the certificate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub begin_refresh_at: Option<crate::common::Timestamp>,

    /// notAfter is the time at which the certificate expires.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not_after: Option<crate::common::Timestamp>,
}

/// PodCertificateRequestList is a collection of PodCertificateRequest objects.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodCertificateRequestList {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// items is a collection of PodCertificateRequest objects.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<PodCertificateRequest>,
}

// ============================================================================
// Pod Certificate Request Constants
// ============================================================================

/// Well-known condition types for PodCertificateRequests.
pub const POD_CERTIFICATE_REQUEST_CONDITION_TYPE_DENIED: &str = "Denied";
/// Well-known condition types for PodCertificateRequests.
pub const POD_CERTIFICATE_REQUEST_CONDITION_TYPE_FAILED: &str = "Failed";
/// Well-known condition types for PodCertificateRequests.
pub const POD_CERTIFICATE_REQUEST_CONDITION_TYPE_ISSUED: &str = "Issued";

/// Well-known condition reason for PodCertificateRequests.
pub const POD_CERTIFICATE_REQUEST_CONDITION_UNSUPPORTED_KEY_TYPE: &str = "UnsupportedKeyType";

// ============================================================================
// Trait Implementations for Certificates Resources
// ============================================================================

// ----------------------------------------------------------------------------
// ResourceSchema Implementation
// ----------------------------------------------------------------------------

impl ResourceSchema for ClusterTrustBundle {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "certificates.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1alpha1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "ClusterTrustBundle"
    }
    fn resource(_: &Self::Meta) -> &str {
        "clustertrustbundles"
    }

    fn group_static() -> &'static str {
        "certificates.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1alpha1"
    }
    fn kind_static() -> &'static str {
        "ClusterTrustBundle"
    }
    fn resource_static() -> &'static str {
        "clustertrustbundles"
    }
}

impl ResourceSchema for ClusterTrustBundleList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "certificates.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1alpha1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "ClusterTrustBundleList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "clustertrustbundles"
    }

    fn group_static() -> &'static str {
        "certificates.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1alpha1"
    }
    fn kind_static() -> &'static str {
        "ClusterTrustBundleList"
    }
    fn resource_static() -> &'static str {
        "clustertrustbundles"
    }
}

impl ResourceSchema for PodCertificateRequest {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "certificates.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1alpha1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "PodCertificateRequest"
    }
    fn resource(_: &Self::Meta) -> &str {
        "podcertificaterequests"
    }

    fn group_static() -> &'static str {
        "certificates.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1alpha1"
    }
    fn kind_static() -> &'static str {
        "PodCertificateRequest"
    }
    fn resource_static() -> &'static str {
        "podcertificaterequests"
    }
}

impl ResourceSchema for PodCertificateRequestList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "certificates.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1alpha1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "PodCertificateRequestList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "podcertificaterequests"
    }

    fn group_static() -> &'static str {
        "certificates.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1alpha1"
    }
    fn kind_static() -> &'static str {
        "PodCertificateRequestList"
    }
    fn resource_static() -> &'static str {
        "podcertificaterequests"
    }
}

// ----------------------------------------------------------------------------
// HasTypeMeta Implementation
// ----------------------------------------------------------------------------

impl HasTypeMeta for ClusterTrustBundle {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for ClusterTrustBundleList {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for PodCertificateRequest {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for PodCertificateRequestList {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

// ----------------------------------------------------------------------------
// VersionedObject Implementation
// ----------------------------------------------------------------------------

impl VersionedObject for ClusterTrustBundle {
    fn metadata(&self) -> &ObjectMeta {
        self.metadata
            .as_ref()
            .unwrap_or_else(|| static_default_object_meta())
    }

    fn metadata_mut(&mut self) -> &mut ObjectMeta {
        self.metadata.get_or_insert_with(ObjectMeta::default)
    }
}

impl VersionedObject for PodCertificateRequest {
    fn metadata(&self) -> &ObjectMeta {
        self.metadata
            .as_ref()
            .unwrap_or_else(|| static_default_object_meta())
    }

    fn metadata_mut(&mut self) -> &mut ObjectMeta {
        self.metadata.get_or_insert_with(ObjectMeta::default)
    }
}

fn static_default_object_meta() -> &'static ObjectMeta {
    static DEFAULT: OnceLock<ObjectMeta> = OnceLock::new();
    DEFAULT.get_or_init(ObjectMeta::default)
}

// ----------------------------------------------------------------------------
// Protobuf Placeholder
// ----------------------------------------------------------------------------

impl_unimplemented_prost_message!(ClusterTrustBundle);
impl_unimplemented_prost_message!(ClusterTrustBundleList);
impl_unimplemented_prost_message!(PodCertificateRequest);
impl_unimplemented_prost_message!(PodCertificateRequestList);

#[cfg(test)]
mod trait_tests;
