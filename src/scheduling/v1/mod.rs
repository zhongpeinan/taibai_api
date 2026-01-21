//! Scheduling v1 API types
//!
//! This module contains the v1 version of the Kubernetes Scheduling API types.
//!
//! The scheduling API provides support for configuring pod priority and preemption.
//!
//! Source: api-master/scheduling/v1/types.go

use crate::common::{
    ApplyDefaults, HasTypeMeta, ListMeta, ObjectMeta, ResourceSchema, TypeMeta,
    UnimplementedConversion, VersionedObject,
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
    use serde_json;

    // PriorityClass tests
    #[test]
    fn test_priority_class_default() {
        let pc = PriorityClass::default();
        assert!(pc.metadata.is_none());
        assert!(!pc.global_default);
        assert!(pc.description.is_empty());
    }

    #[test]
    fn test_priority_class_with_value() {
        let pc = PriorityClass {
            value: Some(1000000),
            global_default: false,
            description: "High priority workload".to_string(),
            ..Default::default()
        };
        assert_eq!(pc.value, Some(1000000));
        assert_eq!(pc.description, "High priority workload");
    }

    #[test]
    fn test_priority_class_global_default() {
        let pc = PriorityClass {
            value: Some(1000),
            global_default: true,
            description: "Default priority class".to_string(),
            ..Default::default()
        };
        assert!(pc.global_default);
    }

    #[test]
    fn test_priority_class_serialize() {
        let pc = PriorityClass {
            value: Some(500000),
            global_default: false,
            description: "Medium priority".to_string(),
            ..Default::default()
        };
        let json = serde_json::to_string(&pc).unwrap();
        assert!(json.contains(r#""value":500000"#));
        assert!(json.contains(r#""globalDefault":false"#));
        assert!(json.contains(r#""description":"Medium priority""#));
    }

    #[test]
    fn test_priority_class_deserialize() {
        let json = r#"{
            "value": 1000000,
            "globalDefault": false,
            "description": "High priority workload"
        }"#;
        let pc: PriorityClass = serde_json::from_str(json).unwrap();
        assert_eq!(pc.value, Some(1000000));
        assert!(!pc.global_default);
        assert_eq!(pc.description, "High priority workload");
    }

    #[test]
    fn test_priority_class_round_trip() {
        let original = PriorityClass {
            value: Some(1000000),
            global_default: true,
            description: "System critical".to_string(),
            preemption_policy: Some(PreemptionPolicy::PreemptLowerPriority),
            ..Default::default()
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: PriorityClass = serde_json::from_str(&json).unwrap();
        assert_eq!(original.value, deserialized.value);
        assert_eq!(original.global_default, deserialized.global_default);
    }

    #[test]
    fn test_priority_class_with_preemption_policy() {
        let pc = PriorityClass {
            value: Some(1000000),
            preemption_policy: Some(PreemptionPolicy::Never),
            ..Default::default()
        };
        assert_eq!(pc.preemption_policy, Some(PreemptionPolicy::Never));
    }

    // PriorityClassList tests
    #[test]
    fn test_priority_class_list_default() {
        let list = PriorityClassList::default();
        assert!(list.items.is_empty());
    }

    #[test]
    fn test_priority_class_list_with_items() {
        let list = PriorityClassList {
            items: vec![PriorityClass {
                value: Some(1000),
                ..Default::default()
            }],
            ..Default::default()
        };
        assert_eq!(list.items.len(), 1);
    }

    #[test]
    fn test_priority_class_list_serialize() {
        let list = PriorityClassList {
            items: vec![PriorityClass {
                value: Some(1000),
                ..Default::default()
            }],
            ..Default::default()
        };
        let json = serde_json::to_string(&list).unwrap();
        assert!(json.contains(r#""items"#));
    }

    // Constants tests
    #[test]
    fn test_constants() {
        assert_eq!(DEFAULT_PRIORITY_WHEN_NO_DEFAULT_CLASS_EXISTS, 0);
        assert_eq!(HIGHEST_USER_DEFINABLE_PRIORITY, 1_000_000_000);
        assert_eq!(SYSTEM_CRITICAL_PRIORITY, 2_000_000_000);
        assert_eq!(SYSTEM_PRIORITY_CLASS_PREFIX, "system-");
        assert_eq!(SYSTEM_CLUSTER_CRITICAL, "system-cluster-critical");
        assert_eq!(SYSTEM_NODE_CRITICAL, "system-node-critical");
    }

    // Edge case tests
    #[test]
    fn test_priority_class_zero_value() {
        let pc = PriorityClass {
            value: Some(0),
            ..Default::default()
        };
        assert_eq!(pc.value, Some(0));
    }

    #[test]
    fn test_priority_class_max_user_priority() {
        let pc = PriorityClass {
            value: Some(HIGHEST_USER_DEFINABLE_PRIORITY),
            ..Default::default()
        };
        assert_eq!(pc.value, Some(HIGHEST_USER_DEFINABLE_PRIORITY));
    }

    #[test]
    fn test_priority_class_system_critical() {
        let pc = PriorityClass {
            value: Some(SYSTEM_CRITICAL_PRIORITY),
            global_default: false,
            ..Default::default()
        };
        assert_eq!(pc.value, Some(SYSTEM_CRITICAL_PRIORITY));
    }

    #[test]
    fn test_empty_priority_class_deserialize() {
        let json = r#"{}"#;
        let pc: PriorityClass = serde_json::from_str(json).unwrap();
        assert_eq!(pc.value, None);
        assert!(!pc.global_default);
    }

    // Integration test
    #[test]
    fn test_full_priority_class_workflow() {
        let pc = PriorityClass {
            type_meta: TypeMeta::default(),
            value: Some(1000000),
            global_default: false,
            description: "High priority production workload".to_string(),
            preemption_policy: Some(PreemptionPolicy::PreemptLowerPriority),
            ..Default::default()
        };

        let json = serde_json::to_string_pretty(&pc).unwrap();
        let deserialized: PriorityClass = serde_json::from_str(&json).unwrap();

        assert_eq!(pc.value, deserialized.value);
        assert_eq!(pc.description, deserialized.description);
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

impl ApplyDefaults for PriorityClass {
    fn apply_defaults(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "scheduling.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "PriorityClass".to_string();
        }
    }
}

impl ApplyDefaults for PriorityClassList {
    fn apply_defaults(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "scheduling.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "PriorityClassList".to_string();
        }
    }
}

// ----------------------------------------------------------------------------
// Version Conversion Placeholder
// ----------------------------------------------------------------------------

impl UnimplementedConversion for PriorityClass {}
impl UnimplementedConversion for PriorityClassList {}

// ----------------------------------------------------------------------------
// Protobuf Placeholder
// ----------------------------------------------------------------------------

impl_unimplemented_prost_message!(PriorityClass);
impl_unimplemented_prost_message!(PriorityClassList);
