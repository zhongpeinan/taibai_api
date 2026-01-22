//! ComponentStatus types from the Kubernetes Core API
//!
//! This module contains types for component status resources.
//!
//! Source: k8s.io/api/core/v1/types.go

use crate::common::{ListMeta, ObjectMeta, TypeMeta};
use crate::impl_versioned_object;
use serde::{Deserialize, Serialize};

// Re-export for public use
pub use crate::core::internal::{ComponentCondition, ComponentConditionType};

// ============================================================================
// ComponentStatus
// ============================================================================

/// ComponentStatus (and ComponentStatusList) holds the cluster validation info.
///
/// Deprecated: This API is deprecated in v1.19+
///
/// Corresponds to [Kubernetes ComponentStatus](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L8018)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ComponentStatus {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// List of component conditions observed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<ComponentCondition>,
}
impl_versioned_object!(ComponentStatus);

impl Default for ComponentStatus {
    fn default() -> Self {
        Self {
            type_meta: TypeMeta::default(),
            metadata: None,
            conditions: Vec::new(),
        }
    }
}

/// ComponentStatusList is a list of ComponentStatus objects.
///
/// Corresponds to [Kubernetes ComponentStatusList](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L8039)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComponentStatusList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(default)]
    pub metadata: Option<ListMeta>,
    /// List of ComponentStatus objects.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ComponentStatus>,
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

impl crate::common::traits::ResourceSchema for ComponentStatusList {
    type Meta = ();

    fn group(_meta: &Self::Meta) -> &str {
        ""
    }

    fn version(_meta: &Self::Meta) -> &str {
        "v1"
    }

    fn kind(_meta: &Self::Meta) -> &str {
        "ComponentStatusList"
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
        "ComponentStatusList"
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

impl crate::common::traits::ApplyDefault for ComponentStatus {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "ComponentStatus".to_string();
        }
    }
}

impl crate::common::traits::ApplyDefault for ComponentStatusList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "ComponentStatusList".to_string();
        }
    }
}

impl crate::common::traits::UnimplementedConversion for ComponentStatus {}
impl crate::common::traits::UnimplementedConversion for ComponentStatusList {}

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
        assert!(cs.metadata.is_none());
        assert!(cs.conditions.is_empty());
    }

    #[test]
    fn test_component_status_with_conditions() {
        let cs = ComponentStatus {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("scheduler".to_string()),
                ..Default::default()
            }),
            conditions: vec![ComponentCondition {
                r#type: ComponentConditionType::Healthy,
                status: "True".to_string(),
                ..Default::default()
            }],
        };
        assert_eq!(cs.conditions.len(), 1);
        assert!(cs.metadata.is_some());
    }

    #[test]
    fn test_component_status_serialize() {
        let cs = ComponentStatus {
            type_meta: TypeMeta {
                kind: "ComponentStatus".to_string(),
                api_version: "v1".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("controller-manager".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let json = serde_json::to_string(&cs).unwrap();
        assert!(json.contains(r#""kind":"ComponentStatus""#));
        assert!(json.contains(r#""name":"controller-manager""#));
    }

    #[test]
    fn test_component_status_apply_default() {
        let mut cs = ComponentStatus {
            type_meta: TypeMeta::default(),
            ..Default::default()
        };
        cs.apply_default();
        assert_eq!(cs.type_meta.api_version, "v1");
        assert_eq!(cs.type_meta.kind, "ComponentStatus");
    }

    #[test]
    fn test_component_status_round_trip() {
        let original = ComponentStatus {
            type_meta: TypeMeta {
                kind: "ComponentStatus".to_string(),
                api_version: "v1".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("etcd-0".to_string()),
                ..Default::default()
            }),
            conditions: vec![ComponentCondition {
                r#type: ComponentConditionType::Healthy,
                status: "True".to_string(),
                message: "Healthy".to_string(),
                error: "".to_string(),
            }],
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: ComponentStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(
            original.metadata.unwrap().name,
            deserialized.metadata.unwrap().name
        );
    }

    // ComponentStatusList tests
    #[test]
    fn test_component_status_list_default() {
        let list = ComponentStatusList::default();
        assert!(list.metadata.is_none());
        assert!(list.items.is_empty());
    }

    #[test]
    fn test_component_status_list_with_items() {
        let list = ComponentStatusList {
            type_meta: TypeMeta {
                kind: "ComponentStatusList".to_string(),
                api_version: "v1".to_string(),
            },
            metadata: None,
            items: vec![ComponentStatus {
                type_meta: TypeMeta {
                    kind: "ComponentStatus".to_string(),
                    api_version: "v1".to_string(),
                },
                ..Default::default()
            }],
        };
        assert_eq!(list.items.len(), 1);
    }

    #[test]
    fn test_component_status_list_apply_default() {
        let mut list = ComponentStatusList {
            type_meta: TypeMeta::default(),
            ..Default::default()
        };
        list.apply_default();
        assert_eq!(list.type_meta.api_version, "v1");
        assert_eq!(list.type_meta.kind, "ComponentStatusList");
    }

    #[test]
    fn test_component_condition_type_constants() {
        assert_eq!(
            crate::core::internal::component_condition_type::HEALTHY,
            "Healthy"
        );
    }

    // Integration tests
    #[test]
    fn test_component_status_with_multiple_conditions() {
        let cs = ComponentStatus {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("scheduler".to_string()),
                ..Default::default()
            }),
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
    fn test_component_status_list_serialize() {
        let list = ComponentStatusList {
            type_meta: TypeMeta {
                kind: "ComponentStatusList".to_string(),
                api_version: "v1".to_string(),
            },
            metadata: Some(ListMeta {
                resource_version: Some("12345".to_string()),
                ..Default::default()
            }),
            items: vec![],
        };
        let json = serde_json::to_string(&list).unwrap();
        assert!(json.contains(r#""kind":"ComponentStatusList""#));
        assert!(json.contains(r#""resourceVersion":"12345""#));
    }
}
