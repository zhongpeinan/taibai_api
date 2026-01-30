//! Certificates v1beta1 API types
//!
//! This module contains the v1beta1 version of the Kubernetes Certificates API types.
//!
//! Source: api-master/certificates/v1beta1/types.go

pub mod conversion;
pub mod defaults;

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::common::{HasTypeMeta, ObjectMeta, ResourceSchema, TypeMeta, VersionedObject};
use crate::core::internal::ByteString;
use crate::impl_unimplemented_prost_message;

pub use crate::certificates::v1::{
    CertificateSigningRequestCondition, CertificateSigningRequestStatus, ExtraValue, KeyUsage,
    RequestConditionType,
};

// ============================================================================
// Certificate Signing Request
// ============================================================================

/// CertificateSigningRequest objects provide a mechanism to obtain x509 certificates
/// by submitting a certificate signing request, and having it asynchronously approved and issued.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CertificateSigningRequest {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::common::ObjectMeta>,

    /// spec contains the certificate request, and is immutable after creation.
    /// Only the request, signerName, expirationSeconds, and usages fields can be set on creation.
    /// Other fields are derived by Kubernetes and cannot be modified by users.
    #[serde(default)]
    pub spec: CertificateSigningRequestSpec,

    /// status contains information about whether the request is approved or denied,
    /// and the certificate issued by the signer, or the failure condition indicating signer failure.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<CertificateSigningRequestStatus>,
}

/// CertificateSigningRequestSpec contains the certificate request.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CertificateSigningRequestSpec {
    /// request contains an x509 certificate signing request encoded in a "CERTIFICATE REQUEST" PEM block.
    /// When serialized as JSON or YAML, the data is additionally base64-encoded.
    #[serde(default)]
    pub request: ByteString,

    /// signerName indicates the requested signer, and is a qualified name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signer_name: Option<String>,

    /// expirationSeconds is the requested duration of validity of the issued certificate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_seconds: Option<i32>,

    /// usages specifies a set of key usages requested in the issued certificate.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub usages: Vec<KeyUsage>,

    /// username contains the name of the user that created the CertificateSigningRequest.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub username: String,

    /// uid contains the uid of the user that created the CertificateSigningRequest.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub uid: String,

    /// groups contains group membership of the user that created the CertificateSigningRequest.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<String>,

    /// extra contains extra attributes of the user that created the CertificateSigningRequest.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extra: BTreeMap<String, ExtraValue>,
}

/// CertificateSigningRequestList is a collection of CertificateSigningRequest objects.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CertificateSigningRequestList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::common::ListMeta>,

    /// items is a collection of CertificateSigningRequest objects.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<CertificateSigningRequest>,
}

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
    pub metadata: Option<crate::common::ObjectMeta>,

    /// Spec contains the signer (if any) and trust anchors.
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
    #[serde(default, skip_serializing_if = "String::is_empty")]
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
    pub metadata: Option<crate::common::ListMeta>,

    /// Items is a collection of ClusterTrustBundle objects.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ClusterTrustBundle>,
}

// ============================================================================
// Signer Name Constants
// ============================================================================

impl CertificateSigningRequestSpec {
    /// "kubernetes.io/kube-apiserver-client" signer issues client certificates
    /// that can be used to authenticate to kube-apiserver.
    /// Never auto-approved by kube-controller-manager.
    pub const KUBE_APISERVER_CLIENT_SIGNER_NAME: &'static str =
        "kubernetes.io/kube-apiserver-client";

    /// "kubernetes.io/kube-apiserver-client-kubelet" issues client certificates
    /// that kubelets use to authenticate to kube-apiserver.
    /// Can be auto-approved by kube-controller-manager.
    pub const KUBE_APISERVER_CLIENT_KUBELET_SIGNER_NAME: &'static str =
        "kubernetes.io/kube-apiserver-client-kubelet";

    /// "kubernetes.io/kubelet-serving" issues serving certificates that kubelets
    /// use to serve TLS endpoints, which kube-apiserver can connect to securely.
    /// Never auto-approved by kube-controller-manager.
    pub const KUBELET_SERVING_SIGNER_NAME: &'static str = "kubernetes.io/kubelet-serving";

    /// "kubernetes.io/legacy-unknown" has no guarantees for trust at all.
    pub const LEGACY_UNKNOWN_SIGNER_NAME: &'static str = "kubernetes.io/legacy-unknown";
}

// ============================================================================
// Trait Implementations for Certificates Resources
// ============================================================================

// ----------------------------------------------------------------------------
// ResourceSchema Implementation
// ----------------------------------------------------------------------------

impl ResourceSchema for CertificateSigningRequest {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "certificates.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1beta1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "CertificateSigningRequest"
    }
    fn resource(_: &Self::Meta) -> &str {
        "certificatesigningrequests"
    }

    fn group_static() -> &'static str {
        "certificates.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1beta1"
    }
    fn kind_static() -> &'static str {
        "CertificateSigningRequest"
    }
    fn resource_static() -> &'static str {
        "certificatesigningrequests"
    }
}

impl ResourceSchema for CertificateSigningRequestList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "certificates.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1beta1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "CertificateSigningRequestList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "certificatesigningrequests"
    }

    fn group_static() -> &'static str {
        "certificates.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1beta1"
    }
    fn kind_static() -> &'static str {
        "CertificateSigningRequestList"
    }
    fn resource_static() -> &'static str {
        "certificatesigningrequests"
    }
}

impl ResourceSchema for ClusterTrustBundle {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "certificates.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1beta1"
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
        "v1beta1"
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
        "v1beta1"
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
        "v1beta1"
    }
    fn kind_static() -> &'static str {
        "ClusterTrustBundleList"
    }
    fn resource_static() -> &'static str {
        "clustertrustbundles"
    }
}

// ----------------------------------------------------------------------------
// HasTypeMeta Implementation
// ----------------------------------------------------------------------------

impl HasTypeMeta for CertificateSigningRequest {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for CertificateSigningRequestList {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

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

// ----------------------------------------------------------------------------
// VersionedObject Implementation
// ----------------------------------------------------------------------------

impl VersionedObject for CertificateSigningRequest {
    fn metadata(&self) -> &ObjectMeta {
        use std::sync::OnceLock;
        self.metadata.as_ref().unwrap_or_else(|| {
            static DEFAULT: OnceLock<ObjectMeta> = OnceLock::new();
            DEFAULT.get_or_init(ObjectMeta::default)
        })
    }

    fn metadata_mut(&mut self) -> &mut ObjectMeta {
        self.metadata.get_or_insert_with(ObjectMeta::default)
    }
}

impl VersionedObject for ClusterTrustBundle {
    fn metadata(&self) -> &ObjectMeta {
        use std::sync::OnceLock;
        self.metadata.as_ref().unwrap_or_else(|| {
            static DEFAULT: OnceLock<ObjectMeta> = OnceLock::new();
            DEFAULT.get_or_init(ObjectMeta::default)
        })
    }

    fn metadata_mut(&mut self) -> &mut ObjectMeta {
        self.metadata.get_or_insert_with(ObjectMeta::default)
    }
}

// ----------------------------------------------------------------------------
// Protobuf Placeholder
// ----------------------------------------------------------------------------

impl_unimplemented_prost_message!(CertificateSigningRequest);
impl_unimplemented_prost_message!(CertificateSigningRequestList);
impl_unimplemented_prost_message!(ClusterTrustBundle);
impl_unimplemented_prost_message!(ClusterTrustBundleList);

#[cfg(test)]
mod trait_tests;
