//! Scheduling v1 API types
//!
//! This module contains the v1 version of the Kubernetes Scheduling API types.
//!
//! The scheduling API provides support for configuring pod priority and preemption.
//!
//! Source: api-master/scheduling/v1/types.go

pub mod conversion;
pub mod validation;

use crate::common::{
    ApplyDefault, HasTypeMeta, ListMeta, ObjectMeta, ResourceSchema, TypeMeta, VersionedObject,
};
use crate::core::internal::PreemptionPolicy;
use crate::impl_unimplemented_prost_message;
use serde::{Deserialize, Serialize};

// ============================================================================
// PriorityClass Types
// ============================================================================

/// PriorityClass defines mapping from a priority class name to the priority
/// integer value.
///
/// PriorityClass is used to influence the order in which pods are scheduled
/// and preempted. Pods with higher priority values are scheduled before pods
/// with lower priority values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PriorityClass {
    /// TypeMeta describes the type of this object.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// value represents the integer value of this priority class.
    /// This is the actual priority that pods receive when they have the
    /// name of this class in their pod spec.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<i32>,

    /// globalDefault specifies whether this PriorityClass should be considered as
    /// the default priority for pods that do not have any priority class.
    /// Only one PriorityClass can be marked as `globalDefault`.
    #[serde(default)]
    pub global_default: bool,

    /// description is an arbitrary string that usually provides guidelines on
    /// when this priority class should be used.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,

    /// preemptionPolicy is the Policy for preempting pods with lower priority.
    /// One of Never, PreemptLowerPriority.
    /// Defaults to PreemptLowerPriority if unset.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preemption_policy: Option<PreemptionPolicy>,
}

/// PriorityClassList is a collection of priority classes.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PriorityClassList {
    /// TypeMeta describes the type of this object.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// items is the list of PriorityClasses.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<PriorityClass>,
}

// ============================================================================
// Constants
// ============================================================================

/// DefaultPriorityWhenNoDefaultClassExists is used to set priority of pods
/// that do not specify any priority class and there is no priority class
/// marked as default.
pub const DEFAULT_PRIORITY_WHEN_NO_DEFAULT_CLASS_EXISTS: i32 = 0;

/// HighestUserDefinablePriority is the highest priority for user defined
/// priority classes. Priority values larger than 1 billion are reserved
/// for Kubernetes system use.
pub const HIGHEST_USER_DEFINABLE_PRIORITY: i32 = 1_000_000_000;

/// SystemCriticalPriority is the beginning of the range of priority values
/// for critical system components.
pub const SYSTEM_CRITICAL_PRIORITY: i32 = 2 * HIGHEST_USER_DEFINABLE_PRIORITY;

/// SystemPriorityClassPrefix is the prefix reserved for system priority class names.
/// Other priority classes are not allowed to start with this prefix.
pub const SYSTEM_PRIORITY_CLASS_PREFIX: &str = "system-";

/// SystemClusterCritical is the system priority class name that represents
/// cluster-critical.
pub const SYSTEM_CLUSTER_CRITICAL: &str = "system-cluster-critical";

/// SystemNodeCritical is the system priority class name that represents
/// node-critical.
pub const SYSTEM_NODE_CRITICAL: &str = "system-node-critical";

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // Compile-time Trait Checks
    // ========================================================================

    /// 编译时检查：确保顶级资源实现了必需的 traits
    #[test]
    fn top_level_resources_implement_required_traits() {
        fn check<T: VersionedObject + ApplyDefault>() {}

        check::<PriorityClass>();
    }

    /// Note: Conversion traits are tested in conversion.rs module
    /// (uses local internal types, not scheduling::internal types)

    /// 编译时检查：确保���源实现了 prost::Message
    #[test]
    fn prost_message() {
        fn check<T: prost::Message>() {}

        check::<PriorityClass>();
        check::<PriorityClassList>();
    }

    // ========================================================================
    // Runtime Behavior Tests
    // ========================================================================

    #[test]
    fn test_apply_default_sets_preemption_policy() {
        let mut obj = PriorityClass::default();
        obj.apply_default();
        assert_eq!(
            obj.preemption_policy,
            Some(PreemptionPolicy::PreemptLowerPriority)
        );
    }

    #[test]
    fn test_apply_default_list_applies_item_defaults() {
        let mut list = PriorityClassList {
            items: vec![PriorityClass::default()],
            ..Default::default()
        };

        list.apply_default();

        assert_eq!(
            list.items[0].preemption_policy,
            Some(PreemptionPolicy::PreemptLowerPriority)
        );
    }
}

// ============================================================================
// Trait Implementations for Scheduling Resources
// ============================================================================

// ----------------------------------------------------------------------------
// ResourceSchema Implementation
// ----------------------------------------------------------------------------

impl ResourceSchema for PriorityClass {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "scheduling.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "PriorityClass"
    }
    fn resource(_: &Self::Meta) -> &str {
        "priorityclasses"
    }

    fn group_static() -> &'static str {
        "scheduling.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "PriorityClass"
    }
    fn resource_static() -> &'static str {
        "priorityclasses"
    }
}

impl ResourceSchema for PriorityClassList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "scheduling.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "PriorityClassList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "priorityclasses"
    }

    fn group_static() -> &'static str {
        "scheduling.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "PriorityClassList"
    }
    fn resource_static() -> &'static str {
        "priorityclasses"
    }
}

// ----------------------------------------------------------------------------
// HasTypeMeta Implementation
// ----------------------------------------------------------------------------

impl HasTypeMeta for PriorityClass {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for PriorityClassList {
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

impl VersionedObject for PriorityClass {
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

impl ApplyDefault for PriorityClass {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "scheduling.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "PriorityClass".to_string();
        }
        if self.preemption_policy.is_none() {
            self.preemption_policy = Some(PreemptionPolicy::PreemptLowerPriority);
        }
    }
}

impl ApplyDefault for PriorityClassList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "scheduling.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "PriorityClassList".to_string();
        }
        for item in &mut self.items {
            item.apply_default();
        }
    }
}

// ----------------------------------------------------------------------------
// Protobuf Placeholder
// ----------------------------------------------------------------------------

impl_unimplemented_prost_message!(PriorityClass);
impl_unimplemented_prost_message!(PriorityClassList);

#[cfg(test)]
mod trait_tests;

#[cfg(test)]
mod serde_roundtrip_tests;

#[cfg(test)]
mod conversion_roundtrip_tests;
