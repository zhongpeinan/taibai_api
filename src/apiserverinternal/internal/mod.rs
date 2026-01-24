//! APIServerInternal internal API types
//!
//! Mirrors k8s.io/kubernetes/pkg/apis/apiserverinternal.

use serde::{Deserialize, Serialize};

use crate::common::{ListMeta, ObjectMeta, Timestamp, TypeMeta};
use crate::impl_has_object_meta;

/// StorageVersion of a specific resource.
///
/// Corresponds to [Kubernetes StorageVersion](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/apiserverinternal/types.go#L27)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StorageVersion {
    /// TypeMeta for this resource
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard object metadata.
    pub metadata: ObjectMeta,

    /// Spec is an empty spec. It is here to comply with Kubernetes API style.
    #[serde(default)]
    pub spec: StorageVersionSpec,

    /// API server instances report the version they can decode and the version they
    /// encode objects to when persisting objects in the backend.
    #[serde(default)]
    pub status: StorageVersionStatus,
}
impl_has_object_meta!(StorageVersion);

/// StorageVersionSpec is an empty spec.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StorageVersionSpec {}

/// API server instances report the versions they can decode and the version they
/// encode objects to when persisting objects in the backend.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StorageVersionStatus {
    /// The reported versions per API server instance.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub storage_versions: Vec<ServerStorageVersion>,

    /// If all API server instances agree on the same encoding storage version,
    /// then this field is set to that version. Otherwise this field is left empty.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub common_encoding_version: Option<String>,

    /// The latest available observations of the storageVersion's state.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<StorageVersionCondition>,
}

/// An API server instance reports the version it can decode and the version it
/// encodes objects to when persisting objects in the backend.
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

/// StorageVersionConditionType indicates the storage version condition type.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default, Hash)]
#[serde(rename_all = "camelCase")]
pub struct StorageVersionConditionType(pub String);

impl AsRef<str> for StorageVersionConditionType {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<String> for StorageVersionConditionType {
    fn from(s: String) -> Self {
        StorageVersionConditionType(s)
    }
}

impl From<&str> for StorageVersionConditionType {
    fn from(s: &str) -> Self {
        StorageVersionConditionType(s.to_string())
    }
}

impl StorageVersionConditionType {
    /// Indicates that encoding storage versions reported by all servers are equal.
    pub const ALL_ENCODING_VERSIONS_EQUAL: &'static str = "AllEncodingVersionsEqual";
}

/// ConditionStatus indicates status of condition from "True", "False", or "Unknown".
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ConditionStatus(pub String);

impl AsRef<str> for ConditionStatus {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<String> for ConditionStatus {
    fn from(s: String) -> Self {
        ConditionStatus(s)
    }
}

impl From<&str> for ConditionStatus {
    fn from(s: &str) -> Self {
        ConditionStatus(s.to_string())
    }
}

impl ConditionStatus {
    /// ConditionTrue indicates condition as "True"
    pub const TRUE: &'static str = "True";
    /// ConditionFalse indicates condition as "False"
    pub const FALSE: &'static str = "False";
    /// ConditionUnknown indicates condition as "Unknown"
    pub const UNKNOWN: &'static str = "Unknown";
}

/// Describes the state of the storageVersion at a certain point.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StorageVersionCondition {
    /// Type of the condition.
    #[serde(rename = "type")]
    #[serde(default)]
    pub type_: StorageVersionConditionType,

    /// Status of the condition, one of True, False, Unknown.
    #[serde(default)]
    pub status: ConditionStatus,

    /// If set, this represents the .metadata.generation that the condition was set based upon.
    #[serde(default, skip_serializing_if = "crate::common::util::is_zero_i64")]
    pub observed_generation: i64,

    /// Last time the condition transitioned from one status to another.
    #[serde(default = "Timestamp::zero", skip_serializing_if = "Timestamp::is_zero")]
    pub last_transition_time: Timestamp,

    /// The reason for the condition's last transition.
    #[serde(default)]
    pub reason: String,

    /// A human readable message indicating details about the transition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
}

/// A list of StorageVersions.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StorageVersionList {
    /// TypeMeta for this resource
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard list metadata.
    #[serde(default)]
    pub metadata: ListMeta,

    /// Items holds a list of StorageVersion
    #[serde(default)]
    pub items: Vec<StorageVersion>,
}
