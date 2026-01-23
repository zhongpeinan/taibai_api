//! Namespace-related types from the Kubernetes Core API
//!
//! This module contains types for Kubernetes namespaces.

use crate::common::time::Timestamp;
use crate::common::{ListMeta, ObjectMeta, TypeMeta};
use crate::core::internal::{NamespaceConditionType, NamespacePhase};
use crate::impl_has_object_meta;
use serde::{Deserialize, Serialize};

// ============================================================================
// Namespace
// ============================================================================

/// Namespace provides a scope for Names.
///
/// Corresponds to [Kubernetes Namespace](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5788)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Namespace {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ObjectMeta,
    /// Spec defines the behavior of the Namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<NamespaceSpec>,
    /// Status describes the current status of a Namespace.
    #[serde(default)]
    pub status: NamespaceStatus,
}
impl_has_object_meta!(Namespace);

/// NamespaceList is a list of Namespaces.
///
/// Corresponds to [Kubernetes NamespaceList](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5803)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NamespaceList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    /// Items is the list of Namespaces.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Namespace>,
}

// ============================================================================
// NamespaceSpec
// ============================================================================

/// NamespaceSpec describes the attributes of a namespace.
///
/// Corresponds to [Kubernetes NamespaceSpec](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5811)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NamespaceSpec {
    /// Finalizers is an opaque list of values that must be empty to permanently remove object from storage.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub finalizers: Vec<String>,
}

/// NamespaceStatus is information about the current status of a namespace.
///
/// Corresponds to [Kubernetes NamespaceStatus](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5826)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NamespaceStatus {
    /// Phase is the current lifecycle phase of the namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<NamespacePhase>,
    /// Conditions represents the latest available observations of a namespace's current state.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<NamespaceCondition>,
}

/// NamespaceCondition contais details about the state of a namespace.
///
/// Corresponds to [Kubernetes NamespaceCondition](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5856)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NamespaceCondition {
    /// Type of namespace condition.
    #[serde(rename = "type")]
    pub type_: NamespaceConditionType,
    /// Status of the condition, one of True, False, Unknown.
    pub status: String,
    /// LastTransitionTime is the last time the condition transitioned.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<Timestamp>,
    /// Reason is the reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Message is a human-readable message indicating details about the transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[cfg(test)]
mod tests {}
