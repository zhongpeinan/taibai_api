//! RuntimeClass types from the Kubernetes Node v1 API
//!
//! This module contains types for defining container runtime classes.

use crate::common::{
    ApplyDefault, HasTypeMeta, ListMeta, ObjectMeta, ResourceSchema, TypeMeta, VersionedObject,
};

use crate::core::v1::{ResourceList, Toleration};
use crate::impl_unimplemented_prost_message;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// RuntimeClass defines a class of container runtime supported in the cluster.
///
/// The RuntimeClass is used to determine which container runtime is used to run
/// all containers in a pod. RuntimeClasses are manually defined by a
/// user or cluster provisioner, and referenced in the PodSpec. The Kubelet is
/// responsible for resolving the RuntimeClassName reference before running the
/// pod. For more details, see
/// https://kubernetes.io/docs/concepts/containers/runtime-class/
///
/// Corresponds to [Kubernetes RuntimeClass](https://github.com/kubernetes/api/blob/master/node/v1/types.go#L29)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeClass {
    /// TypeMeta describes the type of this object.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// Handler specifies the underlying runtime and configuration that the CRI
    /// implementation will use to handle pods of this class.
    ///
    /// The possible values are specific to the node & CRI configuration. It is
    /// assumed that all handlers are available on every node, and handlers of
    /// the same name are equivalent on every node.
    ///
    /// For example, a handler called "runc" might specify that the runc OCI
    /// runtime (using native Linux containers) will be used to run the containers
    /// in a pod.
    ///
    /// The Handler must be lowercase, conform to the DNS Label (RFC 1123) requirements,
    /// and is immutable.
    pub handler: String,

    /// Overhead represents the resource overhead associated with running a pod for a
    /// given RuntimeClass.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overhead: Option<Overhead>,

    /// Scheduling holds the scheduling constraints to ensure that pods running
    /// with this RuntimeClass are scheduled to nodes that support it.
    ///
    /// If scheduling is nil, this RuntimeClass is assumed to be supported by all
    /// nodes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheduling: Option<Scheduling>,
}

/// RuntimeClassList is a list of RuntimeClass objects.
///
/// Corresponds to [Kubernetes RuntimeClassList](https://github.com/kubernetes/api/blob/master/node/v1/types.go#L103)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeClassList {
    /// TypeMeta describes the type of this object.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// Items is a list of schema objects.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<RuntimeClass>,
}

/// Overhead structure represents the resource overhead associated with running a pod.
///
/// Corresponds to [Kubernetes Overhead](https://github.com/kubernetes/api/blob/master/node/v1/types.go#L73)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Overhead {
    /// PodFixed represents the fixed resource overhead associated with running a pod.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub pod_fixed: ResourceList,
}

/// Scheduling specifies the scheduling constraints for nodes supporting a
/// RuntimeClass.
///
/// Corresponds to [Kubernetes Scheduling](https://github.com/kubernetes/api/blob/master/node/v1/types.go#L80)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Scheduling {
    /// NodeSelector lists labels that must be present on nodes that support this
    /// RuntimeClass. Pods using this RuntimeClass can only be scheduled to a
    /// node matched by this selector. The RuntimeClass nodeSelector is merged
    /// with a pod's existing nodeSelector. Any conflicts will cause the pod to
    /// be rejected in admission.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub node_selector: BTreeMap<String, String>,

    /// Tolerations are appended (excluding duplicates) to pods running with this
    /// RuntimeClass during admission, effectively unioning the set of nodes
    /// tolerated by the pod and the RuntimeClass.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tolerations: Vec<Toleration>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{FromInternal, ToInternal};
    use crate::node::internal;

    // ========================================================================
    // Compile-time Trait Checks
    // ========================================================================

    /// 编译时检查：确保顶级资源实现了必需的 traits
    #[test]
    fn top_level_resources_implement_required_traits() {
        fn check<T: VersionedObject + ApplyDefault>() {}

        check::<RuntimeClass>();
    }

    /// 编译时检查：确保资源实现了版本转换 traits
    #[test]
    fn conversion_traits() {
        fn check<T, I>()
        where
            T: ToInternal<I> + FromInternal<I>,
        {
        }

        check::<RuntimeClass, internal::RuntimeClass>();
    }

    /// 编译时检查：确保资源实现了 prost::Message
    #[test]
    fn prost_message() {
        fn check<T: prost::Message>() {}

        check::<RuntimeClass>();
        check::<RuntimeClassList>();
    }

    // ========================================================================
    // Runtime Behavior Tests
    // ========================================================================

    #[test]
    fn test_apply_default_runtime_class() {
        let mut obj = RuntimeClass {
            type_meta: TypeMeta::default(),
            metadata: None,
            handler: "runc".to_string(),
            overhead: None,
            scheduling: None,
        };

        obj.apply_default();

        assert_eq!(obj.type_meta.api_version, "node.k8s.io/v1");
        assert_eq!(obj.type_meta.kind, "RuntimeClass");
    }

    #[test]
    fn test_apply_default_preserves_existing_values() {
        let mut obj = RuntimeClass {
            type_meta: TypeMeta {
                api_version: "custom.version/v1".to_string(),
                kind: "CustomKind".to_string(),
            },
            metadata: None,
            handler: "runc".to_string(),
            overhead: None,
            scheduling: None,
        };

        obj.apply_default();

        // Existing values should be preserved
        assert_eq!(obj.type_meta.api_version, "custom.version/v1");
        assert_eq!(obj.type_meta.kind, "CustomKind");
    }

    #[test]
    fn test_apply_default_partial_values() {
        let mut obj = RuntimeClass {
            type_meta: TypeMeta {
                api_version: "existing.version".to_string(),
                kind: "".to_string(),
            },
            metadata: None,
            handler: "runc".to_string(),
            overhead: None,
            scheduling: None,
        };

        obj.apply_default();

        // apiVersion should be preserved, kind should be defaulted
        assert_eq!(obj.type_meta.api_version, "existing.version");
        assert_eq!(obj.type_meta.kind, "RuntimeClass");
    }

    #[test]
    fn test_apply_default_runtime_class_list() {
        let mut obj = RuntimeClassList {
            type_meta: TypeMeta::default(),
            metadata: None,
            items: vec![],
        };

        obj.apply_default();

        assert_eq!(obj.type_meta.api_version, "node.k8s.io/v1");
        assert_eq!(obj.type_meta.kind, "RuntimeClassList");
    }

    #[test]
    fn test_apply_default_runtime_class_list_preserves_existing() {
        let mut obj = RuntimeClassList {
            type_meta: TypeMeta {
                api_version: "custom.version/v1".to_string(),
                kind: "CustomKind".to_string(),
            },
            metadata: None,
            items: vec![],
        };

        obj.apply_default();

        assert_eq!(obj.type_meta.api_version, "custom.version/v1");
        assert_eq!(obj.type_meta.kind, "CustomKind");
    }
}

// ============================================================================
// Trait Implementations for Node Resources
// ============================================================================

// ----------------------------------------------------------------------------
// ResourceSchema Implementation
// ----------------------------------------------------------------------------

impl ResourceSchema for RuntimeClass {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "node.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "RuntimeClass"
    }
    fn resource(_: &Self::Meta) -> &str {
        "runtimeclasses"
    }

    fn group_static() -> &'static str {
        "node.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "RuntimeClass"
    }
    fn resource_static() -> &'static str {
        "runtimeclasses"
    }
}

impl ResourceSchema for RuntimeClassList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "node.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "RuntimeClassList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "runtimeclasses"
    }

    fn group_static() -> &'static str {
        "node.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "RuntimeClassList"
    }
    fn resource_static() -> &'static str {
        "runtimeclasses"
    }
}

// ----------------------------------------------------------------------------
// HasTypeMeta Implementation
// ----------------------------------------------------------------------------

impl HasTypeMeta for RuntimeClass {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for RuntimeClassList {
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

impl VersionedObject for RuntimeClass {
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

// ----------------------------------------------------------------------------
// ApplyDefaults Implementation
// ----------------------------------------------------------------------------

impl ApplyDefault for RuntimeClass {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "node.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "RuntimeClass".to_string();
        }
    }
}

impl ApplyDefault for RuntimeClassList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "node.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "RuntimeClassList".to_string();
        }
        for item in &mut self.items {
            item.apply_default();
        }
    }
}

// ----------------------------------------------------------------------------
// Protobuf Placeholder
// ----------------------------------------------------------------------------

impl_unimplemented_prost_message!(RuntimeClass);
impl_unimplemented_prost_message!(RuntimeClassList);

// Note: ToInternal and FromInternal are implemented in conversion.rs
