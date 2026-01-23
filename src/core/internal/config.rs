//! ConfigMap and Secret types from the Kubernetes Core API
//!
//! This module contains types for configuration storage resources,
//! including ConfigMap, Secret, and ServiceAccount.
//!
//! Source: k8s-pkg/apis/core/types.go

use crate::common::{ListMeta, ObjectMeta, TypeMeta};
use crate::core::internal::helper::ByteString;
use crate::core::internal::{LocalObjectReference, SecretType};
use crate::impl_has_object_meta;
use serde::{Deserialize, Serialize};

// ============================================================================
// ConfigMap
// ============================================================================

/// ConfigMap holds configuration data for pods to consume.
///
/// Corresponds to [Kubernetes ConfigMap](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5324)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ConfigMap {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ObjectMeta,
    /// Data contains the configuration data.
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub data: std::collections::BTreeMap<String, String>,
    /// BinaryData contains the binary data.
    /// Keys are similar to Data but values are base64-encoded.
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub binary_data: std::collections::BTreeMap<String, ByteString>,
    /// Immutable, if set to true, guarantees that the stored data cannot be updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub immutable: Option<bool>,
}
impl_has_object_meta!(ConfigMap);

/// ConfigMapList is a list of ConfigMaps.
///
/// Corresponds to [Kubernetes ConfigMapList](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5337)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ConfigMapList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    /// Items is the list of ConfigMaps.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ConfigMap>,
}

// ============================================================================
// Secret
// ============================================================================

/// Secret holds secret data of a certain type.
///
/// Corresponds to [Kubernetes Secret](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5494)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Secret {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ObjectMeta,
    /// Data contains the secret data.
    /// Each key must consist of alphanumeric characters, '-', '_' or '.'.
    /// Values are base64-encoded strings.
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub data: std::collections::BTreeMap<String, ByteString>,
    /// Used to facilitate programmatic handling of secret data.
    #[serde(default)]
    pub r#type: SecretType,
    /// Immutable, if set to true, guarantees that the stored data cannot be updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub immutable: Option<bool>,
    /// StringData allows specifying non-binary secret data in string form.
    #[serde(
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty",
        skip_deserializing
    )]
    pub string_data: std::collections::BTreeMap<String, String>,
}
impl_has_object_meta!(Secret);

/// SecretList is a list of Secret.
///
/// Corresponds to [Kubernetes SecretList](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5512)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SecretList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    /// Items is the list of Secret objects.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Secret>,
}

// ============================================================================
// ServiceAccount
// ============================================================================

/// ServiceAccount binds together: * a name, a principal that can be authenticated
/// and authorized * secrets that will be used by pods running as this service account.
///
/// Corresponds to [Kubernetes ServiceAccount](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5615)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceAccount {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ObjectMeta,
    /// Secrets is the list of secrets allowed to be used by pods running as this ServiceAccount.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub secrets: Vec<ObjectReference>,
    /// ImagePullSecrets is a list of references to secrets in the same namespace to use for pulling any images.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub image_pull_secrets: Vec<LocalObjectReference>,
    /// AutomountServiceAccountToken indicates whether pods running as this service account should have an API token automatically mounted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub automount_service_account_token: Option<bool>,
}
impl_has_object_meta!(ServiceAccount);

/// ServiceAccountList is a list of ServiceAccount objects.
///
/// Corresponds to [Kubernetes ServiceAccountList](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5630)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceAccountList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    /// Items is the list of ServiceAccounts.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ServiceAccount>,
}

// ============================================================================
// ObjectReference for ServiceAccount
// ============================================================================

/// ObjectReference contains enough information to let you inspect or modify the referred object.
/// This is used in ServiceAccount.secrets field.
///
/// Corresponds to [Kubernetes ObjectReference](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5557)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ObjectReference {
    /// Kind of the referent.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub kind: String,
    /// Namespace of the referent.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub namespace: String,
    /// Name of the referent.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// UID of the referent.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub uid: String,
    /// API version of the referent.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub api_version: String,
    /// Specific resourceVersion to which this reference is made.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub resource_version: String,
    /// Specific field within this resource.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub field_path: String,
}

#[cfg(test)]
mod tests {
}
