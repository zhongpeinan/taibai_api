//! TestAPIGroup internal API types
//!
//! Mirrors k8s.io/apimachinery/pkg/apis/testapigroup.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::common::{InternalObject, ListMeta, ObjectMeta, Timestamp, TypeMeta};
use crate::impl_has_object_meta;

// ============================================================================
// Enums / Constants
// ============================================================================

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ConditionStatus(pub String);

impl ConditionStatus {
    pub const TRUE: &'static str = "True";
    pub const FALSE: &'static str = "False";
    pub const UNKNOWN: &'static str = "Unknown";

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl AsRef<str> for ConditionStatus {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<String> for ConditionStatus {
    fn from(value: String) -> Self {
        ConditionStatus(value)
    }
}

impl From<&str> for ConditionStatus {
    fn from(value: &str) -> Self {
        ConditionStatus(value.to_string())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default, Hash)]
#[serde(rename_all = "camelCase")]
pub struct CarpConditionType(pub String);

impl CarpConditionType {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl AsRef<str> for CarpConditionType {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<String> for CarpConditionType {
    fn from(value: String) -> Self {
        CarpConditionType(value)
    }
}

impl From<&str> for CarpConditionType {
    fn from(value: &str) -> Self {
        CarpConditionType(value.to_string())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default, Hash)]
#[serde(rename_all = "camelCase")]
pub struct CarpPhase(pub String);

impl CarpPhase {
    pub const PENDING: &'static str = "Pending";
    pub const RUNNING: &'static str = "Running";
    pub const SUCCEEDED: &'static str = "Succeeded";
    pub const FAILED: &'static str = "Failed";
    pub const UNKNOWN: &'static str = "Unknown";

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl AsRef<str> for CarpPhase {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<String> for CarpPhase {
    fn from(value: String) -> Self {
        CarpPhase(value)
    }
}

impl From<&str> for CarpPhase {
    fn from(value: &str) -> Self {
        CarpPhase(value.to_string())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default, Hash)]
#[serde(rename_all = "camelCase")]
pub struct RestartPolicy(pub String);

impl RestartPolicy {
    pub const ALWAYS: &'static str = "Always";
    pub const ON_FAILURE: &'static str = "OnFailure";
    pub const NEVER: &'static str = "Never";

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl AsRef<str> for RestartPolicy {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<String> for RestartPolicy {
    fn from(value: String) -> Self {
        RestartPolicy(value)
    }
}

impl From<&str> for RestartPolicy {
    fn from(value: &str) -> Self {
        RestartPolicy(value.to_string())
    }
}

// ============================================================================
// Carp Types
// ============================================================================

/// Carp is a collection of containers, used as either input (create, update) or as output (list, get).
///
/// Source: k8s.io/apimachinery/pkg/apis/testapigroup/types.go
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Carp {
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    pub metadata: ObjectMeta,

    #[serde(default)]
    pub spec: CarpSpec,

    #[serde(default)]
    pub status: CarpStatus,
}
impl_has_object_meta!(Carp);
impl InternalObject for Carp {}

/// CarpStatus represents information about the status of a carp.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CarpStatus {
    #[serde(default, skip_serializing_if = "CarpPhase::is_empty")]
    pub phase: CarpPhase,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<CarpCondition>,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,

    #[serde(default, skip_serializing_if = "String::is_empty", rename = "hostIP")]
    pub host_ip: String,

    #[serde(default, skip_serializing_if = "String::is_empty", rename = "carpIP")]
    pub carp_ip: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<Timestamp>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub infos: Vec<CarpInfo>,
}

/// CarpCondition describes current state of the carp.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CarpCondition {
    #[serde(rename = "type", default)]
    pub type_: CarpConditionType,

    #[serde(default)]
    pub status: ConditionStatus,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_probe_time: Option<Timestamp>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<Timestamp>,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
}

/// CarpInfo is a map-style list item keyed by A, B, and optional C.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CarpInfo {
    pub a: i64,

    pub b: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub c: Option<String>,

    pub data: String,
}

/// CarpSpec is a description of a carp.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CarpSpec {
    #[serde(default, skip_serializing_if = "RestartPolicy::is_empty")]
    pub restart_policy: RestartPolicy,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub termination_grace_period_seconds: Option<i64>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_deadline_seconds: Option<i64>,

    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub node_selector: BTreeMap<String, String>,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub service_account_name: String,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub node_name: String,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub hostname: String,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub subdomain: String,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub scheduler_name: String,
}

/// CarpList is a list of Carps.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CarpList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    #[serde(default)]
    pub metadata: ListMeta,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Carp>,
}

// ============================================================================
// Trait Implementations
// ============================================================================

impl crate::common::traits::ResourceSchema for Carp {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "testapigroup.apimachinery.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "__internal"
    }
    fn kind(_: &Self::Meta) -> &str {
        "Carp"
    }
    fn resource(_: &Self::Meta) -> &str {
        "carps"
    }

    fn group_static() -> &'static str {
        "testapigroup.apimachinery.k8s.io"
    }
    fn version_static() -> &'static str {
        "__internal"
    }
    fn kind_static() -> &'static str {
        "Carp"
    }
    fn resource_static() -> &'static str {
        "carps"
    }
}

impl crate::common::traits::ResourceSchema for CarpList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "testapigroup.apimachinery.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "__internal"
    }
    fn kind(_: &Self::Meta) -> &str {
        "CarpList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "carps"
    }

    fn group_static() -> &'static str {
        "testapigroup.apimachinery.k8s.io"
    }
    fn version_static() -> &'static str {
        "__internal"
    }
    fn kind_static() -> &'static str {
        "CarpList"
    }
    fn resource_static() -> &'static str {
        "carps"
    }
}

impl crate::common::traits::HasTypeMeta for Carp {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl crate::common::traits::HasTypeMeta for CarpList {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}
