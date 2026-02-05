//! ConfigMap and Secret types from the Kubernetes Core v1 API
//!
//! This module contains types for configuration and secret management.

use crate::common::{
    ApplyDefault, HasTypeMeta, ListMeta, ObjectMeta, ResourceSchema, TypeMeta, VersionedObject,
};
use crate::core::internal::ByteString;
use crate::impl_unimplemented_prost_message;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// ConfigMap holds configuration data for pods to consume.
///
/// Corresponds to [Kubernetes ConfigMap](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L8039)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ConfigMap {
    /// TypeMeta describes the type of this object
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// Immutable, if set to true, ensures that data stored in the ConfigMap cannot be updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub immutable: Option<bool>,

    /// Data contains the configuration data.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub data: BTreeMap<String, String>,

    /// BinaryData contains the binary data.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub binary_data: BTreeMap<String, ByteString>,
}

/// ConfigMapList is a list of ConfigMaps.
///
/// Corresponds to [Kubernetes ConfigMapList](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L8076)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ConfigMapList {
    /// TypeMeta describes the type of this object
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// Items is the list of ConfigMaps.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ConfigMap>,
}

/// Secret holds secret data of a certain type.
///
/// Corresponds to [Kubernetes Secret](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7896)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Secret {
    /// TypeMeta describes the type of this object
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// Immutable, if set to true, ensures that data stored in the Secret cannot be updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub immutable: Option<bool>,

    /// Data contains the secret data. Each key must be a valid DNS_SUBDOMAIN.
    /// The values are base64 encoded strings.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub data: BTreeMap<String, ByteString>,

    /// stringData allows specifying non-binary secret data in string form.
    /// It is provided as a write-only input field for convenience.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub string_data: BTreeMap<String, String>,

    /// Used to facilitate programmatic handling of secret data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// SecretList is a list of Secret.
///
/// Corresponds to [Kubernetes SecretList](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L8022)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SecretList {
    /// TypeMeta describes the type of this object
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// Items is a list of secret objects.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Secret>,
}

/// Secret type constants
///
/// More info: https://kubernetes.io/docs/concepts/configuration/secret/#secret-types
pub mod secret_type {
    /// Opaque (default) secret type
    pub const OPAQUE: &str = "Opaque";

    /// Service account token secret type
    pub const SERVICE_ACCOUNT_TOKEN: &str = "kubernetes.io/service-account-token";

    /// Docker config secret type
    pub const DOCKER_CONFIG_JSON: &str = "kubernetes.io/dockercfg";

    /// Docker config JSON secret type
    pub const DOCKER_CONFIG: &str = "kubernetes.io/dockerconfigjson";

    /// Basic auth secret type
    pub const BASIC_AUTH: &str = "kubernetes.io/basic-auth";

    /// SSH auth secret type
    pub const SSH_AUTH: &str = "kubernetes.io/ssh-auth";

    /// TLS secret type
    pub const TLS: &str = "kubernetes.io/tls";

    /// Bootstrap token secret type
    pub const BOOTSTRAP_TOKEN: &str = "bootstrap.kubernetes.io/token";
}

// ============================================================================
// ServiceAccount
// ============================================================================

/// ServiceAccount binds together: * a name, understood by users and/or systems,
/// and an identity mapped to that name.
///
/// Corresponds to [Kubernetes ServiceAccount](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L8260)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceAccount {
    /// TypeMeta describes the type of this object
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// Secrets is a list of the secrets in the same namespace that pods can use.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub secrets: Vec<super::reference::ObjectReference>,

    /// ImagePullSecrets is a list of references to secrets in the same namespace.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub image_pull_secrets: Vec<super::reference::LocalObjectReference>,

    /// AutomountServiceAccountToken indicates whether pods running as this service account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub automount_service_account_token: Option<bool>,
}

/// ServiceAccountList is a list of ServiceAccount objects.
///
/// Corresponds to [Kubernetes ServiceAccountList](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L8273)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceAccountList {
    /// TypeMeta describes the type of this object
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// Items is the list of ServiceAccounts.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ServiceAccount>,
}

// ============================================================================
// Trait Implementations for ConfigMap, ConfigMapList, Secret, and SecretList
// ============================================================================

impl ResourceSchema for ConfigMap {
    type Meta = ();
    fn group(_: &Self::Meta) -> &str {
        ""
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "ConfigMap"
    }
    fn resource(_: &Self::Meta) -> &str {
        "configmaps"
    }
    fn group_static() -> &'static str {
        ""
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "ConfigMap"
    }
    fn resource_static() -> &'static str {
        "configmaps"
    }
}

impl ResourceSchema for ConfigMapList {
    type Meta = ();
    fn group(_: &Self::Meta) -> &str {
        ""
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "ConfigMapList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "configmaps"
    }
    fn group_static() -> &'static str {
        ""
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "ConfigMapList"
    }
    fn resource_static() -> &'static str {
        "configmaps"
    }
}

impl ResourceSchema for Secret {
    type Meta = ();
    fn group(_: &Self::Meta) -> &str {
        ""
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "Secret"
    }
    fn resource(_: &Self::Meta) -> &str {
        "secrets"
    }
    fn group_static() -> &'static str {
        ""
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "Secret"
    }
    fn resource_static() -> &'static str {
        "secrets"
    }
}

impl ResourceSchema for SecretList {
    type Meta = ();
    fn group(_: &Self::Meta) -> &str {
        ""
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "SecretList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "secrets"
    }
    fn group_static() -> &'static str {
        ""
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "SecretList"
    }
    fn resource_static() -> &'static str {
        "secrets"
    }
}

impl ResourceSchema for ServiceAccount {
    type Meta = ();
    fn group(_: &Self::Meta) -> &str {
        ""
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "ServiceAccount"
    }
    fn resource(_: &Self::Meta) -> &str {
        "serviceaccounts"
    }
    fn group_static() -> &'static str {
        ""
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "ServiceAccount"
    }
    fn resource_static() -> &'static str {
        "serviceaccounts"
    }
}

impl ResourceSchema for ServiceAccountList {
    type Meta = ();
    fn group(_: &Self::Meta) -> &str {
        ""
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "ServiceAccountList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "serviceaccounts"
    }
    fn group_static() -> &'static str {
        ""
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "ServiceAccountList"
    }
    fn resource_static() -> &'static str {
        "serviceaccounts"
    }
}

impl HasTypeMeta for ConfigMap {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for ConfigMapList {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for Secret {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for SecretList {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for ServiceAccount {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for ServiceAccountList {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

crate::impl_has_list_meta!(ConfigMapList);
crate::impl_has_list_meta!(SecretList);
crate::impl_has_list_meta!(ServiceAccountList);

impl VersionedObject for ConfigMap {
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

impl VersionedObject for Secret {
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

impl VersionedObject for ServiceAccount {
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

impl ApplyDefault for ConfigMap {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "ConfigMap".to_string();
        }
    }
}

impl ApplyDefault for ConfigMapList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "ConfigMapList".to_string();
        }
        for item in &mut self.items {
            item.apply_default();
        }
    }
}

impl ApplyDefault for Secret {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "Secret".to_string();
        }
        if self
            .type_
            .as_ref()
            .map(|value| value.is_empty())
            .unwrap_or(true)
        {
            self.type_ = Some(secret_type::OPAQUE.to_string());
        }
    }
}

impl ApplyDefault for SecretList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "SecretList".to_string();
        }
        for item in &mut self.items {
            item.apply_default();
        }
    }
}

impl ApplyDefault for ServiceAccount {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "ServiceAccount".to_string();
        }
    }
}

impl ApplyDefault for ServiceAccountList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "ServiceAccountList".to_string();
        }
        for item in &mut self.items {
            item.apply_default();
        }
    }
}

impl_unimplemented_prost_message!(ConfigMap);
impl_unimplemented_prost_message!(ConfigMapList);
impl_unimplemented_prost_message!(Secret);
impl_unimplemented_prost_message!(SecretList);
impl_unimplemented_prost_message!(ServiceAccount);
impl_unimplemented_prost_message!(ServiceAccountList);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secret_defaults_type_to_opaque() {
        let mut secret = Secret {
            type_meta: TypeMeta::default(),
            metadata: None,
            immutable: None,
            data: BTreeMap::new(),
            string_data: BTreeMap::new(),
            type_: None,
        };

        secret.apply_default();

        assert_eq!(secret.type_, Some(secret_type::OPAQUE.to_string()));
    }

    #[test]
    fn test_secret_defaults_empty_type_to_opaque() {
        let mut secret = Secret {
            type_meta: TypeMeta::default(),
            metadata: None,
            immutable: None,
            data: BTreeMap::new(),
            string_data: BTreeMap::new(),
            type_: Some(String::new()),
        };

        secret.apply_default();

        assert_eq!(secret.type_, Some(secret_type::OPAQUE.to_string()));
    }
}
