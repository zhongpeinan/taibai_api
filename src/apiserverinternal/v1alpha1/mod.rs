//! APIServerInternal v1alpha1 API types
//!
//! This module contains the APIServerInternal v1alpha1 API types.
//!
//! Corresponds to [Kubernetes APIServerInternal v1alpha1](https://github.com/kubernetes/apiserver/blob/master/pkg/apis/apiserverinternal/v1alpha1/types.go)

use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

use crate::common::{
    ApplyDefault, HasTypeMeta, ListMeta, ObjectMeta, ResourceSchema, TypeMeta,
    UnimplementedConversion, VersionedObject,
};
use crate::impl_unimplemented_prost_message;

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
///
/// Corresponds to [Kubernetes StorageVersionConditionType](https://github.com/kubernetes/apiserver/blob/master/pkg/apis/apiserverinternal/v1alpha1/types.go#L87)
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
///
/// Corresponds to [Kubernetes ConditionStatus](https://github.com/kubernetes/apiserver/blob/master/pkg/apis/apiserverinternal/v1alpha1/types.go#L94)
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
///
/// Corresponds to [Kubernetes StorageVersionCondition](https://github.com/kubernetes/apiserver/blob/master/pkg/apis/apiserverinternal/v1alpha1/types.go#L103)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StorageVersionCondition {
    /// Type of the condition.
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<StorageVersionConditionType>,

    /// Status of the condition, one of True, False, Unknown.
    #[serde(default)]
    pub status: ConditionStatus,

    /// If set, this represents the .metadata.generation that the condition was set based upon.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,

    /// Last time the condition transitioned from one status to another.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<String>,

    /// The reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,

    /// A human readable message indicating details about the transition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
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

// ============================================================================
// Trait Implementations for StorageVersion and StorageVersionList
// ============================================================================

// ----------------------------------------------------------------------------
// ResourceSchema Implementation
// ----------------------------------------------------------------------------

impl ResourceSchema for StorageVersion {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "apiserverinternal.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1alpha1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "StorageVersion"
    }
    fn resource(_: &Self::Meta) -> &str {
        "storageversions"
    }

    fn group_static() -> &'static str {
        "apiserverinternal.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1alpha1"
    }
    fn kind_static() -> &'static str {
        "StorageVersion"
    }
    fn resource_static() -> &'static str {
        "storageversions"
    }
}

impl ResourceSchema for StorageVersionList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "apiserverinternal.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1alpha1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "StorageVersionList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "storageversions"
    }

    fn group_static() -> &'static str {
        "apiserverinternal.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1alpha1"
    }
    fn kind_static() -> &'static str {
        "StorageVersionList"
    }
    fn resource_static() -> &'static str {
        "storageversions"
    }
}

// ----------------------------------------------------------------------------
// HasTypeMeta Implementation
// ----------------------------------------------------------------------------

impl HasTypeMeta for StorageVersion {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for StorageVersionList {
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

impl VersionedObject for StorageVersion {
    fn metadata(&self) -> &ObjectMeta {
        self.metadata
            .as_ref()
            .unwrap_or_else(|| static_default_object_meta())
    }

    fn metadata_mut(&mut self) -> &mut ObjectMeta {
        self.metadata.get_or_insert_with(ObjectMeta::default)
    }
}

// Helper function for static default ObjectMeta
fn static_default_object_meta() -> &'static ObjectMeta {
    static DEFAULT: OnceLock<ObjectMeta> = OnceLock::new();
    DEFAULT.get_or_init(ObjectMeta::default)
}

// Note: StorageVersionList does not implement VersionedObject because its metadata is ListMeta

// ----------------------------------------------------------------------------
// ApplyDefaults Implementation
// ----------------------------------------------------------------------------

impl ApplyDefault for StorageVersion {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "apiserverinternal.k8s.io/v1alpha1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "StorageVersion".to_string();
        }
    }
}

impl ApplyDefault for StorageVersionList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "apiserverinternal.k8s.io/v1alpha1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "StorageVersionList".to_string();
        }
    }
}

// ----------------------------------------------------------------------------
// Version Conversion Placeholder (using UnimplementedConversion)
// ----------------------------------------------------------------------------

impl UnimplementedConversion for StorageVersion {}

// ----------------------------------------------------------------------------
// Protobuf Placeholder (using macro)
// ----------------------------------------------------------------------------

impl_unimplemented_prost_message!(StorageVersion);
impl_unimplemented_prost_message!(StorageVersionList);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_version_default() {
        let sv = StorageVersion::default();
        assert!(sv.metadata.is_none());
    }

    #[test]
    fn test_storage_version_spec_default() {
        let _spec = StorageVersionSpec::default();
        // Empty struct
    }

    #[test]
    fn test_storage_version_status_default() {
        let status = StorageVersionStatus::default();
        assert!(status.storage_versions.is_empty());
        assert!(status.common_encoding_version.is_none());
        assert!(status.conditions.is_empty());
    }

    #[test]
    fn test_server_storage_version_default() {
        let ssv = ServerStorageVersion::default();
        assert_eq!(ssv.api_server_id, "");
        assert_eq!(ssv.encoding_version, "");
        assert!(ssv.decodable_versions.is_empty());
        assert!(ssv.served_versions.is_empty());
    }

    #[test]
    fn test_server_storage_version_with_values() {
        let ssv = ServerStorageVersion {
            api_server_id: "server-1".to_string(),
            encoding_version: "v1".to_string(),
            decodable_versions: vec!["v1".to_string(), "v2beta1".to_string()],
            served_versions: vec!["v1".to_string()],
        };
        assert_eq!(ssv.api_server_id, "server-1");
        assert_eq!(ssv.encoding_version, "v1");
        assert_eq!(ssv.decodable_versions.len(), 2);
        assert_eq!(ssv.served_versions.len(), 1);
    }

    #[test]
    fn test_storage_version_condition_type_constants() {
        assert_eq!(
            StorageVersionConditionType::ALL_ENCODING_VERSIONS_EQUAL,
            "AllEncodingVersionsEqual"
        );
    }

    #[test]
    fn test_storage_version_condition_type_from_string() {
        let svct: StorageVersionConditionType = "AllEncodingVersionsEqual".to_string().into();
        assert_eq!(svct.0, "AllEncodingVersionsEqual");
    }

    #[test]
    fn test_storage_version_condition_type_from_str() {
        let svct: StorageVersionConditionType = "AllEncodingVersionsEqual".into();
        assert_eq!(svct.0, "AllEncodingVersionsEqual");
    }

    #[test]
    fn test_storage_version_condition_type_as_ref() {
        let svct: StorageVersionConditionType = "AllEncodingVersionsEqual".into();
        assert_eq!(svct.as_ref(), "AllEncodingVersionsEqual");
    }

    #[test]
    fn test_condition_status_constants() {
        assert_eq!(ConditionStatus::TRUE, "True");
        assert_eq!(ConditionStatus::FALSE, "False");
        assert_eq!(ConditionStatus::UNKNOWN, "Unknown");
    }

    #[test]
    fn test_condition_status_from_string() {
        let cs: ConditionStatus = "True".to_string().into();
        assert_eq!(cs.0, "True");
    }

    #[test]
    fn test_condition_status_from_str() {
        let cs: ConditionStatus = "True".into();
        assert_eq!(cs.0, "True");
    }

    #[test]
    fn test_condition_status_as_ref() {
        let cs: ConditionStatus = "True".into();
        assert_eq!(cs.as_ref(), "True");
    }

    #[test]
    fn test_storage_version_condition_default() {
        let condition = StorageVersionCondition::default();
        assert!(condition.type_.is_none());
        assert_eq!(condition.status.0, "");
        assert!(condition.observed_generation.is_none());
        assert!(condition.last_transition_time.is_none());
        assert_eq!(condition.reason, "");
        assert_eq!(condition.message, "");
    }

    #[test]
    fn test_storage_version_condition_with_values() {
        let condition = StorageVersionCondition {
            type_: Some(StorageVersionConditionType::ALL_ENCODING_VERSIONS_EQUAL.into()),
            status: ConditionStatus::TRUE.into(),
            observed_generation: Some(5),
            last_transition_time: Some("2024-01-15T10:00:00Z".to_string()),
            reason: "AllEncodingVersionsEqual".to_string(),
            message: "All servers agree on the encoding version".to_string(),
        };
        assert!(condition.type_.is_some());
        assert_eq!(condition.status.0, "True");
        assert_eq!(condition.observed_generation, Some(5));
        assert_eq!(condition.reason, "AllEncodingVersionsEqual");
    }

    #[test]
    fn test_storage_version_serialize() {
        let sv = StorageVersion {
            type_meta: TypeMeta::default(),
            metadata: None,
            spec: StorageVersionSpec::default(),
            status: StorageVersionStatus::default(),
        };
        let json = serde_json::to_string(&sv).unwrap();
        assert!(json.contains("\"spec\":{}"));
        assert!(json.contains("\"status\":{}"));
    }

    #[test]
    fn test_storage_version_deserialize() {
        let json = r#"{"spec":{},"status":{}}"#;
        let sv: StorageVersion = serde_json::from_str(json).unwrap();
        assert!(sv.metadata.is_none());
    }

    #[test]
    fn test_storage_version_round_trip() {
        let original = StorageVersion {
            type_meta: TypeMeta::default(),
            metadata: None,
            spec: StorageVersionSpec::default(),
            status: StorageVersionStatus {
                common_encoding_version: Some("v1".to_string()),
                ..Default::default()
            },
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: StorageVersion = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_storage_version_list_default() {
        let list = StorageVersionList::default();
        assert!(list.metadata.is_none());
        assert!(list.items.is_empty());
    }

    #[test]
    fn test_storage_version_list_with_items() {
        let sv = StorageVersion::default();
        let list = StorageVersionList {
            type_meta: TypeMeta::default(),
            metadata: None,
            items: vec![sv],
        };
        assert_eq!(list.items.len(), 1);
    }

    #[test]
    fn test_storage_version_list_serialize() {
        let list = StorageVersionList {
            type_meta: TypeMeta::default(),
            metadata: None,
            items: vec![],
        };
        let json = serde_json::to_string(&list).unwrap();
        assert!(json.contains("\"items\":[]"));
    }

    #[test]
    fn test_storage_version_list_deserialize() {
        let json = r#"{"items":[]}"#;
        let list: StorageVersionList = serde_json::from_str(json).unwrap();
        assert!(list.items.is_empty());
    }

    #[test]
    fn test_storage_version_status_with_conditions() {
        let status = StorageVersionStatus {
            storage_versions: vec![ServerStorageVersion {
                api_server_id: "server-1".to_string(),
                encoding_version: "v1".to_string(),
                ..Default::default()
            }],
            common_encoding_version: Some("v1".to_string()),
            conditions: vec![StorageVersionCondition {
                type_: Some(StorageVersionConditionType::ALL_ENCODING_VERSIONS_EQUAL.into()),
                status: ConditionStatus::TRUE.into(),
                ..Default::default()
            }],
        };
        assert_eq!(status.storage_versions.len(), 1);
        assert_eq!(status.conditions.len(), 1);
        assert_eq!(status.common_encoding_version, Some("v1".to_string()));
    }

    #[test]
    fn test_storage_version_with_metadata() {
        let metadata = ObjectMeta {
            name: Some("pods".to_string()),
            ..Default::default()
        };
        let sv = StorageVersion {
            metadata: Some(metadata),
            ..Default::default()
        };
        assert_eq!(sv.metadata.as_ref().unwrap().name, Some("pods".to_string()));
    }

    #[test]
    fn test_server_storage_version_serialize() {
        let ssv = ServerStorageVersion {
            api_server_id: "server-1".to_string(),
            encoding_version: "v1".to_string(),
            decodable_versions: vec!["v1".to_string()],
            served_versions: vec!["v1".to_string()],
        };
        let json = serde_json::to_string(&ssv).unwrap();
        // api_server_id becomes apiServerId with camelCase
        assert!(json.contains("\"apiServerId\":\"server-1\""));
        assert!(json.contains("\"encodingVersion\":\"v1\""));
    }

    #[test]
    fn test_server_storage_version_deserialize() {
        let json = r#"{"apiServerId":"server-1","encodingVersion":"v1"}"#;
        let ssv: ServerStorageVersion = serde_json::from_str(json).unwrap();
        assert_eq!(ssv.api_server_id, "server-1");
        assert_eq!(ssv.encoding_version, "v1");
    }

    #[test]
    fn test_server_storage_version_round_trip() {
        let original = ServerStorageVersion {
            api_server_id: "server-1".to_string(),
            encoding_version: "v1".to_string(),
            decodable_versions: vec!["v1".to_string(), "v2beta1".to_string()],
            served_versions: vec!["v1".to_string()],
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: ServerStorageVersion = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }
}
