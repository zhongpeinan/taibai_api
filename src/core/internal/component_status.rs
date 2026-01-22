//! ComponentStatus types from the Kubernetes Core API (Internal)
//!
//! This module contains internal types for component status resources.
//!
//! Source: k8s.io/kubernetes/pkg/apis/core/types.go

use crate::common::{ObjectMeta, TypeMeta};
use crate::impl_has_object_meta;
use serde::{Deserialize, Serialize};

// Re-export ComponentCondition and ComponentConditionType from parent module
pub use super::{ComponentCondition, ComponentConditionType};

// ============================================================================
// ComponentStatus
// ============================================================================

/// ComponentStatus (and ComponentStatusList) holds the cluster validation info.
///
/// Deprecated: This API is deprecated in v1.19+
///
/// Corresponds to [Kubernetes ComponentStatus](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/core/types.go)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ComponentStatus {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    pub metadata: ObjectMeta,
    /// List of component conditions observed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<ComponentCondition>,
}
impl_has_object_meta!(ComponentStatus);

impl Default for ComponentStatus {
    fn default() -> Self {
        Self {
            type_meta: TypeMeta::default(),
            metadata: ObjectMeta::default(),
            conditions: Vec::new(),
        }
    }
}

// ============================================================================
// Trait Implementations
// ============================================================================

impl crate::common::traits::ResourceSchema for ComponentStatus {
    type Meta = ();

    fn group(_meta: &Self::Meta) -> &str {
        ""
    }

    fn version(_meta: &Self::Meta) -> &str {
        "v1"
    }

    fn kind(_meta: &Self::Meta) -> &str {
        "ComponentStatus"
    }

    fn resource(_meta: &Self::Meta) -> &str {
        "componentstatuses"
    }

    fn group_static() -> &'static str
    where
        Self::Meta: Default,
    {
        ""
    }

    fn version_static() -> &'static str
    where
        Self::Meta: Default,
    {
        "v1"
    }

    fn kind_static() -> &'static str
    where
        Self::Meta: Default,
    {
        "ComponentStatus"
    }

    fn resource_static() -> &'static str
    where
        Self::Meta: Default,
    {
        "componentstatuses"
    }
}

impl crate::common::traits::HasTypeMeta for ComponentStatus {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }

    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ComponentCondition tests
    #[test]
    fn test_component_condition_default() {
        let condition = ComponentCondition::default();
        assert_eq!(condition.r#type, ComponentConditionType::Healthy);
        assert!(condition.status.is_empty());
        assert!(condition.message.is_empty());
        assert!(condition.error.is_empty());
    }

    #[test]
    fn test_component_condition_serialize() {
        let condition = ComponentCondition {
            r#type: ComponentConditionType::Healthy,
            status: "True".to_string(),
            message: "All components are healthy".to_string(),
            error: "".to_string(),
        };
        let json = serde_json::to_string(&condition).unwrap();
        assert!(json.contains(r#""type":"Healthy""#));
        assert!(json.contains(r#""status":"True""#));
        assert!(json.contains("All components are healthy"));
    }

    #[test]
    fn test_component_condition_round_trip() {
        let original = ComponentCondition {
            r#type: ComponentConditionType::Healthy,
            status: "False".to_string(),
            message: "Component is unhealthy".to_string(),
            error: "Connection refused".to_string(),
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: ComponentCondition = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    // ComponentStatus tests
    #[test]
    fn test_component_status_default() {
        let cs = ComponentStatus::default();
        assert!(cs.metadata.name.is_none());
        assert!(cs.conditions.is_empty());
    }

    #[test]
    fn test_component_status_with_conditions() {
        let cs = ComponentStatus {
            type_meta: TypeMeta::default(),
            metadata: ObjectMeta {
                name: Some("scheduler".to_string()),
                ..Default::default()
            },
            conditions: vec![ComponentCondition {
                r#type: ComponentConditionType::Healthy,
                status: "True".to_string(),
                ..Default::default()
            }],
        };
        assert_eq!(cs.conditions.len(), 1);
        assert!(cs.metadata.name.is_some());
    }

    #[test]
    fn test_component_status_serialize() {
        let cs = ComponentStatus {
            type_meta: TypeMeta {
                kind: "ComponentStatus".to_string(),
                api_version: "v1".to_string(),
            },
            metadata: ObjectMeta {
                name: Some("controller-manager".to_string()),
                ..Default::default()
            },
            ..Default::default()
        };
        let json = serde_json::to_string(&cs).unwrap();
        assert!(json.contains(r#""kind":"ComponentStatus""#));
        assert!(json.contains(r#""name":"controller-manager""#));
    }

    #[test]
    fn test_component_status_round_trip() {
        let original = ComponentStatus {
            type_meta: TypeMeta {
                kind: "ComponentStatus".to_string(),
                api_version: "v1".to_string(),
            },
            metadata: ObjectMeta {
                name: Some("etcd-0".to_string()),
                ..Default::default()
            },
            conditions: vec![ComponentCondition {
                r#type: ComponentConditionType::Healthy,
                status: "True".to_string(),
                message: "Healthy".to_string(),
                error: "".to_string(),
            }],
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: ComponentStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(original.metadata.name, deserialized.metadata.name);
    }

    #[test]
    fn test_component_condition_type_constants() {
        assert_eq!(component_condition_type::HEALTHY, "Healthy");
    }

    // Integration tests
    #[test]
    fn test_component_status_with_multiple_conditions() {
        let cs = ComponentStatus {
            type_meta: TypeMeta::default(),
            metadata: ObjectMeta {
                name: Some("scheduler".to_string()),
                ..Default::default()
            },
            conditions: vec![
                ComponentCondition {
                    r#type: ComponentConditionType::Healthy,
                    status: "True".to_string(),
                    message: "OK".to_string(),
                    error: "".to_string(),
                },
                ComponentCondition {
                    r#type: ComponentConditionType::Healthy,
                    status: "False".to_string(),
                    message: "Not OK".to_string(),
                    error: "Error".to_string(),
                },
            ],
        };
        assert_eq!(cs.conditions.len(), 2);
    }

    #[test]
    fn test_has_object_meta_impl() {
        let mut cs = ComponentStatus::default();
        cs.meta_mut().name = Some("test".to_string());
        assert_eq!(cs.meta().name, Some("test".to_string()));
    }
}
