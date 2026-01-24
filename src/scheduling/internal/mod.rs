//! Scheduling internal API types
//!
//! This module contains internal types for Kubernetes scheduling resources.
//!
//! Source: k8s.io/kubernetes/pkg/apis/scheduling/types.go

use crate::common::{InternalObject, ListMeta, ObjectMeta, TypeMeta};
use crate::core::internal::PreemptionPolicy;
use crate::impl_has_object_meta;
use serde::{Deserialize, Serialize};

/// PriorityClass defines mapping from a priority class name to a priority integer value.
///
/// Mirrors the internal scheduling.PriorityClass definition.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PriorityClass {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object metadata.
    #[serde(default, skip_serializing_if = "ObjectMeta::is_empty")]
    pub metadata: ObjectMeta,

    /// value represents the integer value of this priority class.
    #[serde(default)]
    pub value: i32,

    /// globalDefault specifies whether this PriorityClass should be considered as the default.
    #[serde(default)]
    pub global_default: bool,

    /// description is an arbitrary string that provides usage guidelines.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,

    /// preemptionPolicy is the policy for preempting lower priority pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preemption_policy: Option<PreemptionPolicy>,
}

impl_has_object_meta!(PriorityClass);
impl InternalObject for PriorityClass {}

/// PriorityClassList is a collection of priority classes.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PriorityClassList {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "ListMeta::is_empty")]
    pub metadata: ListMeta,
    /// PriorityClass items.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<PriorityClass>,
}

#[cfg(test)]
mod tests {}
