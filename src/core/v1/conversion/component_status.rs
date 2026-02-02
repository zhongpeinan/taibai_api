//! ComponentStatus conversion implementations
//!
//! Converts between core v1 and internal ComponentStatus types.
//! Based on k8s.io/kubernetes/pkg/apis/core/v1/conversion.go

use super::helpers::*;
use crate::common::{ApplyDefault, FromInternal, ToInternal};
use crate::core::internal;
use crate::core::v1::component_status;

// ============================================================================
// ComponentStatus
// ============================================================================

impl ToInternal<internal::ComponentStatus> for component_status::ComponentStatus {
    fn to_internal(self) -> internal::ComponentStatus {
        internal::ComponentStatus {
            type_meta: crate::common::TypeMeta::default(),
            metadata: option_object_meta_to_meta(self.metadata),
            conditions: self.conditions, // ComponentCondition is shared, no conversion needed
        }
    }
}

impl FromInternal<internal::ComponentStatus> for component_status::ComponentStatus {
    fn from_internal(value: internal::ComponentStatus) -> Self {
        let mut result = Self {
            type_meta: crate::common::TypeMeta::default(),
            metadata: meta_to_option_object_meta(value.metadata),
            conditions: value.conditions,
        };

        result
    }
}

// ============================================================================
// ComponentStatusList
// ============================================================================

impl ToInternal<internal::ComponentStatusList> for component_status::ComponentStatusList {
    fn to_internal(self) -> internal::ComponentStatusList {
        internal::ComponentStatusList {
            type_meta: crate::common::TypeMeta::default(),
            metadata: option_list_meta_to_meta(self.metadata),
            items: self.items.into_iter().map(|i| i.to_internal()).collect(),
        }
    }
}

impl FromInternal<internal::ComponentStatusList> for component_status::ComponentStatusList {
    fn from_internal(value: internal::ComponentStatusList) -> Self {
        let mut result = Self {
            type_meta: crate::common::TypeMeta::default(),
            metadata: meta_to_option_list_meta(value.metadata),
            items: value
                .items
                .into_iter()
                .map(component_status::ComponentStatus::from_internal)
                .collect(),
        };

        result
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::internal;

    #[test]
    fn test_component_status_roundtrip() {
        let v1_component_status = component_status::ComponentStatus {
            type_meta: crate::common::TypeMeta {
                api_version: "v1".to_string(),
                kind: "ComponentStatus".to_string(),
            },
            metadata: Some(crate::common::ObjectMeta {
                name: Some("etcd-0".to_string()),
                ..Default::default()
            }),
            conditions: vec![internal::ComponentCondition {
                r#type: internal::ComponentConditionType::Healthy,
                status: "True".to_string(),
                message: "etcd is healthy".to_string(),
                error: String::new(),
            }],
        };

        let internal_component_status = v1_component_status.clone().to_internal();
        assert_eq!(
            internal_component_status.metadata.name,
            Some("etcd-0".to_string())
        );
        assert_eq!(internal_component_status.conditions.len(), 1);
        assert_eq!(internal_component_status.conditions[0].status, "True");

        let mut roundtrip =
            component_status::ComponentStatus::from_internal(internal_component_status);
        roundtrip.apply_default();
        assert_eq!(
            roundtrip.metadata.as_ref().unwrap().name,
            Some("etcd-0".to_string())
        );
        assert_eq!(roundtrip.conditions.len(), 1);
        assert_eq!(roundtrip.type_meta.api_version, "v1");
        assert_eq!(roundtrip.type_meta.kind, "ComponentStatus");
    }

    #[test]
    fn test_component_status_empty_conditions() {
        let v1_component_status = component_status::ComponentStatus {
            type_meta: crate::common::TypeMeta::default(),
            metadata: Some(crate::common::ObjectMeta {
                name: Some("scheduler".to_string()),
                ..Default::default()
            }),
            conditions: vec![],
        };

        let internal_component_status = v1_component_status.clone().to_internal();
        assert!(internal_component_status.conditions.is_empty());

        let mut roundtrip =
            component_status::ComponentStatus::from_internal(internal_component_status);
        roundtrip.apply_default();
        assert!(roundtrip.conditions.is_empty());
    }

    #[test]
    fn test_component_status_list_roundtrip() {
        let v1_list = component_status::ComponentStatusList {
            type_meta: crate::common::TypeMeta {
                api_version: "v1".to_string(),
                kind: "ComponentStatusList".to_string(),
            },
            metadata: Some(crate::common::ListMeta::default()),
            items: vec![
                component_status::ComponentStatus {
                    type_meta: crate::common::TypeMeta::default(),
                    metadata: Some(crate::common::ObjectMeta {
                        name: Some("etcd-0".to_string()),
                        ..Default::default()
                    }),
                    conditions: vec![],
                },
                component_status::ComponentStatus {
                    type_meta: crate::common::TypeMeta::default(),
                    metadata: Some(crate::common::ObjectMeta {
                        name: Some("scheduler".to_string()),
                        ..Default::default()
                    }),
                    conditions: vec![],
                },
            ],
        };

        let internal_list = v1_list.clone().to_internal();
        assert_eq!(internal_list.items.len(), 2);

        let mut roundtrip = component_status::ComponentStatusList::from_internal(internal_list);
        roundtrip.apply_default();
        assert_eq!(roundtrip.items.len(), 2);
        assert_eq!(roundtrip.type_meta.api_version, "v1");
        assert_eq!(roundtrip.type_meta.kind, "ComponentStatusList");
    }
}
