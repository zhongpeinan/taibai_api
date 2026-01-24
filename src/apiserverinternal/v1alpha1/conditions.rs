use serde::{Deserialize, Serialize};

use crate::common::Timestamp;

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
