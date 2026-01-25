//! Node internal API types
//!
//! This module contains internal types for Kubernetes node resources.
//!
//! Source: k8s.io/kubernetes/pkg/apis/node/types.go

use crate::common::{InternalObject, ListMeta, ObjectMeta, TypeMeta};
use crate::core::internal::{ResourceList, Toleration};
use crate::{impl_has_object_meta, impl_unimplemented_prost_message};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// RuntimeClass defines a class of container runtime supported in the cluster.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeClass {
    /// TypeMeta describes the type of this object.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "ObjectMeta::is_empty")]
    pub metadata: ObjectMeta,

    /// Handler specifies the underlying runtime and configuration.
    pub handler: String,

    /// Overhead represents the resource overhead associated with running a pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overhead: Option<Overhead>,

    /// Scheduling holds the scheduling constraints for this RuntimeClass.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheduling: Option<Scheduling>,
}

impl_has_object_meta!(RuntimeClass);
impl InternalObject for RuntimeClass {}

/// RuntimeClassList is a list of RuntimeClass objects.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeClassList {
    /// TypeMeta describes the type of this object.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "ListMeta::is_empty")]
    pub metadata: ListMeta,
    /// Items is a list of RuntimeClass objects.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<RuntimeClass>,
}

/// Overhead structure represents the resource overhead associated with running a pod.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Overhead {
    /// PodFixed represents the fixed resource overhead associated with running a pod.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub pod_fixed: ResourceList,
}

/// Scheduling specifies the scheduling constraints for nodes supporting a RuntimeClass.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Scheduling {
    /// NodeSelector lists labels that must be present on nodes that support this RuntimeClass.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub node_selector: BTreeMap<String, String>,
    /// Tolerations are appended to pods running with this RuntimeClass.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tolerations: Vec<Toleration>,
}

// ===========================================================================
// Protobuf Placeholder Implementations
// ===========================================================================

impl_unimplemented_prost_message!(RuntimeClass);
impl_unimplemented_prost_message!(RuntimeClassList);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::HasObjectMeta;

    // ========================================================================
    // Compile-time Trait Checks
    // ========================================================================

    /// 编译时检查：确保内部版本资源实现了必需的 traits
    #[test]
    fn internal_resources_implement_required_traits() {
        fn check<T: HasObjectMeta>() {}

        check::<RuntimeClass>();
    }

    /// 编译时检查：确保内部版本资源实现了 prost::Message
    #[test]
    fn prost_message() {
        fn check<T: prost::Message>() {}

        check::<RuntimeClass>();
        check::<RuntimeClassList>();
    }
}
