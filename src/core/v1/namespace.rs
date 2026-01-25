//! Namespace types from the Kubernetes Core v1 API
//!
//! This module contains the Namespace type and its associated spec and status types.

use crate::common::{
    ApplyDefault, HasTypeMeta, ListMeta, ObjectMeta, ResourceSchema, Timestamp, TypeMeta,
    VersionedObject,
};
use crate::impl_unimplemented_prost_message;
use serde::{Deserialize, Serialize};

/// Namespace provides a scope for names.
///
/// Names of resources need to be unique within a namespace, but not across namespaces.
///
/// Corresponds to [Kubernetes Namespace](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7121)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Namespace {
    /// TypeMeta describes the type of this object
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// Spec defines the behavior of the Namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<NamespaceSpec>,

    /// Status describes the current status of a Namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<NamespaceStatus>,
}

/// NamespaceList is a list of Namespaces.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct NamespaceList {
    /// TypeMeta describes the type of this object
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// Items is the list of Namespace objects in the list.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Namespace>,
}

/// NamespaceSpec describes the attributes on a Namespace.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NamespaceSpec {
    /// Finalizers is an opaque list of values that must be empty to permanently remove object from storage.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub finalizers: Vec<String>,
}

/// NamespaceStatus is information about the current status of a Namespace.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NamespaceStatus {
    /// Phase is the current lifecycle phase of the namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,

    /// Represents the latest available observations of a namespace's current state.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<NamespaceCondition>,
}

/// NamespaceCondition is an condition of a namespace.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct NamespaceCondition {
    /// Type of namespace controller condition.
    #[serde(rename = "type")]
    pub type_: String,

    /// Status of the condition, one of True, False, Unknown.
    pub status: String,

    /// Last time the condition transitioned from one status to another.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<Timestamp>,

    /// Unique, one-word, CamelCase reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    /// Human-readable message indicating details about last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Namespace phase constants
pub mod namespace_phase {
    /// NamespaceActive means the namespace is available for use in the system
    pub const ACTIVE: &str = "Active";
    /// NamespaceTerminating means the namespace is undergoing graceful termination
    pub const TERMINATING: &str = "Terminating";
}

/// Namespace condition type constants
pub mod namespace_condition_type {
    /// NamespaceDeletionDiscoveryFailure contains information about namespace deleter errors during resource discovery.
    pub const DELETION_DISCOVERY_FAILURE: &str = "NamespaceDeletionDiscoveryFailure";
    /// NamespaceDeletionContentFailure contains information about namespace deleter errors during deletion of resources.
    pub const DELETION_CONTENT_FAILURE: &str = "NamespaceDeletionContentFailure";
    /// NamespaceDeletionGVParsingFailure contains information about namespace deleter errors parsing GV for legacy types.
    pub const DELETION_GV_PARSING_FAILURE: &str = "NamespaceDeletionGVParsingFailure";
}

/// Condition status constants
pub mod condition_status {
    /// ConditionTrue means a resource is in the condition.
    pub const TRUE: &str = "True";
    /// ConditionFalse means a resource is not in the condition.
    pub const FALSE: &str = "False";
    /// ConditionUnknown means kubernetes can't decide if a resource is in the condition or not.
    pub const UNKNOWN: &str = "Unknown";
}

/// Finalizer name constants
pub mod finalizer_name {
    /// FinalizerKubernetes is the kubernetes finalizer value
    pub const KUBERNETES: &str = "kubernetes";
}

#[cfg(test)]
mod tests {}

// ============================================================================
// Trait Implementations for Namespace and NamespaceList
// ============================================================================

impl ResourceSchema for Namespace {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        ""
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "Namespace"
    }
    fn resource(_: &Self::Meta) -> &str {
        "namespaces"
    }

    fn group_static() -> &'static str {
        ""
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "Namespace"
    }
    fn resource_static() -> &'static str {
        "namespaces"
    }
}

impl ResourceSchema for NamespaceList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        ""
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "NamespaceList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "namespaces"
    }

    fn group_static() -> &'static str {
        ""
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "NamespaceList"
    }
    fn resource_static() -> &'static str {
        "namespaces"
    }
}

impl HasTypeMeta for Namespace {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for NamespaceList {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl VersionedObject for Namespace {
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

impl ApplyDefault for Namespace {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "Namespace".to_string();
        }
    }
}

impl ApplyDefault for NamespaceList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "NamespaceList".to_string();
        }
    }
}

impl_unimplemented_prost_message!(Namespace);
impl_unimplemented_prost_message!(NamespaceList);
