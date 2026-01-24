use serde::{Deserialize, Serialize};

use crate::common::{ListMeta, ObjectMeta, TypeMeta};

use super::StorageVersionCondition;

/// Storage version of a specific resource.
///
/// Corresponds to [Kubernetes StorageVersion](https://github.com/kubernetes/apiserver/blob/master/pkg/apis/apiserverinternal/v1alpha1/types.go#L27)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StorageVersion {
    /// TypeMeta for this resource
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard object metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// Spec is an empty spec. It is here to comply with Kubernetes API style.
    #[serde(default)]
    pub spec: StorageVersionSpec,

    /// API server instances report the version they can decode and the version they
    /// encode objects to when persisting objects in the backend.
    #[serde(default)]
    pub status: StorageVersionStatus,
}

/// StorageVersionSpec is an empty spec.
///
/// Corresponds to [Kubernetes StorageVersionSpec](https://github.com/kubernetes/apiserver/blob/master/pkg/apis/apiserverinternal/v1alpha1/types.go#L41)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StorageVersionSpec {}

/// API server instances report the versions they can decode and the version they
/// encode objects to when persisting objects in the backend.
///
/// Corresponds to [Kubernetes StorageVersionStatus](https://github.com/kubernetes/apiserver/blob/master/pkg/apis/apiserverinternal/v1alpha1/types.go#L46)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StorageVersionStatus {
    /// The reported versions per API server instance.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub storage_versions: Vec<ServerStorageVersion>,

    /// If all API server instances agree on the same encoding storage version,
    /// then this field is set to that version. Otherwise this field is left empty.
    /// API servers should finish updating its storageVersionStatus entry before
    /// serving write operations, so that this field will be in sync with the reality.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub common_encoding_version: Option<String>,

    /// The latest available observations of the storageVersion's state.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<StorageVersionCondition>,
}

/// An API server instance reports the version it can decode and the version it
/// encodes objects to when persisting objects in the backend.
///
/// Corresponds to [Kubernetes ServerStorageVersion](https://github.com/kubernetes/apiserver/blob/master/pkg/apis/apiserverinternal/v1alpha1/types.go#L68)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServerStorageVersion {
    /// The ID of the reporting API server.
    #[serde(rename = "apiServerID")]
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub api_server_id: String,

    /// The API server encodes the object to this version when persisting it in
    /// the backend (e.g., etcd).
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub encoding_version: String,

    /// The API server can decode objects encoded in these versions.
    /// The encodingVersion must be included in the decodableVersions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub decodable_versions: Vec<String>,

    /// The API server can serve these versions.
    /// DecodableVersions must include all ServedVersions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub served_versions: Vec<String>,
}

/// A list of StorageVersions.
///
/// Corresponds to [Kubernetes StorageVersionList](https://github.com/kubernetes/apiserver/blob/master/pkg/apis/apiserverinternal/v1alpha1/types.go#L126)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StorageVersionList {
    /// TypeMeta for this resource
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// Items holds a list of StorageVersion
    #[serde(default)]
    pub items: Vec<StorageVersion>,
}
