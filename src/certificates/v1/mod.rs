//! Certificates v1 API types
//!
//! This module contains the v1 version of the Kubernetes Certificates API types.
//!
//! Source: api-master/certificates/v1/types.go

pub mod conversion;
pub mod defaults;
pub mod validation;

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::common::{HasTypeMeta, ObjectMeta, ResourceSchema, TypeMeta, VersionedObject};
use crate::impl_unimplemented_prost_message;

// ============================================================================
// Certificate Signing Request
// ============================================================================

/// CertificateSigningRequest objects provide a mechanism to obtain x509 certificates
/// by submitting a certificate signing request, and having it asynchronously approved and issued.
///
/// Kubelets use this API to obtain:
///  1. client certificates to authenticate to kube-apiserver (with the "kubernetes.io/kube-apiserver-client-kubelet" signerName).
///  2. serving certificates for TLS endpoints kube-apiserver can connect to securely (with the "kubernetes.io/kubelet-serving" signerName).
///
/// This API can be used to request client certificates to authenticate to kube-apiserver
/// (with the "kubernetes.io/kube-apiserver-client" signerName),
/// or to obtain certificates from custom non-Kubernetes signers.
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
    pub request: crate::core::internal::helper::ByteString,

    /// signerName indicates the requested signer, and is a qualified name.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub signer_name: String,

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

/// ExtraValue masks the value so protobuf can generate.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(transparent)]
pub struct ExtraValue {
    pub inner: Vec<String>,
}

impl From<Vec<String>> for ExtraValue {
    fn from(v: Vec<String>) -> Self {
        Self { inner: v }
    }
}

impl From<ExtraValue> for Vec<String> {
    fn from(e: ExtraValue) -> Self {
        e.inner
    }
}

impl AsRef<[String]> for ExtraValue {
    fn as_ref(&self) -> &[String] {
        &self.inner
    }
}

/// CertificateSigningRequestStatus contains conditions used to indicate
/// approved/denied/failed status of the request, and the issued certificate.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CertificateSigningRequestStatus {
    /// conditions applied to the request. Known conditions are "Approved", "Denied", and "Failed".
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<CertificateSigningRequestCondition>,

    /// certificate is populated with an issued certificate by the signer after an Approved condition is present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<crate::core::internal::helper::ByteString>,
}

/// CertificateSigningRequestCondition describes a condition of a CertificateSigningRequest object.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CertificateSigningRequestCondition {
    /// type of the condition. Known conditions are "Approved", "Denied", and "Failed".
    #[serde(rename = "type", default = "default_request_condition_type")]
    pub type_: RequestConditionType,

    /// status of the condition, one of True, False, Unknown.
    #[serde(default)]
    pub status: crate::core::internal::ConditionStatus,

    /// reason indicates a brief reason for the request state.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,

    /// message contains a human readable message with details about the request state.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,

    /// lastUpdateTime is the time of the last update to this condition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<crate::common::Timestamp>,

    /// lastTransitionTime is the time the condition last transitioned from one status to another.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<crate::common::Timestamp>,
}

fn default_request_condition_type() -> RequestConditionType {
    RequestConditionType::Approved
}

/// RequestConditionType is the type of a CertificateSigningRequestCondition.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "PascalCase")]
pub enum RequestConditionType {
    /// Approved indicates the request was approved and should be issued by the signer.
    #[default]
    Approved,
    /// Denied indicates the request was denied and should not be issued by the signer.
    Denied,
    /// Failed indicates the signer failed to issue the certificate.
    Failed,
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
// Key Usage
// ============================================================================

/// KeyUsage specifies valid usage contexts for keys.
///
/// See:
/// - https://tools.ietf.org/html/rfc5280#section-4.2.1.3
/// - https://tools.ietf.org/html/rfc5280#section-4.2.1.12
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum KeyUsage {
    /// "signing"
    Signing,
    /// "digital signature"
    DigitalSignature,
    /// "content commitment"
    ContentCommitment,
    /// "key encipherment"
    KeyEncipherment,
    /// "key agreement"
    KeyAgreement,
    /// "data encipherment"
    DataEncipherment,
    /// "cert sign"
    CertSign,
    /// "crl sign"
    CrlSign,
    /// "encipher only"
    EncipherOnly,
    /// "decipher only"
    DecipherOnly,
    /// "any"
    Any,
    /// "server auth"
    ServerAuth,
    /// "client auth"
    ClientAuth,
    /// "code signing"
    CodeSigning,
    /// "email protection"
    EmailProtection,
    /// "s/mime"
    Smime,
    /// "ipsec end system"
    IpsecEndSystem,
    /// "ipsec tunnel"
    IpsecTunnel,
    /// "ipsec user"
    IpsecUser,
    /// "timestamping"
    Timestamping,
    /// "ocsp signing"
    OcspSigning,
    /// "microsoft sgc"
    MicrosoftSgc,
    /// "netscape sgc"
    NetscapeSgc,
}

// ============================================================================
// Signer Name Constants
// ============================================================================

/// Built in signerName values that are honored by kube-controller-manager.
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
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {}

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
        "v1"
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
        "v1"
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
        "v1"
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
        "v1"
    }
    fn kind_static() -> &'static str {
        "CertificateSigningRequestList"
    }
    fn resource_static() -> &'static str {
        "certificatesigningrequests"
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

// ----------------------------------------------------------------------------
// Protobuf Placeholder
// ----------------------------------------------------------------------------

impl_unimplemented_prost_message!(CertificateSigningRequest);
impl_unimplemented_prost_message!(CertificateSigningRequestList);

#[cfg(test)]
mod trait_tests;

#[cfg(test)]
mod serde_roundtrip_tests;

#[cfg(test)]
mod conversion_roundtrip_tests;
