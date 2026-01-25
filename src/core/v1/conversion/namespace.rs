//! Namespace conversion implementations
//!
//! Converts between core v1 and internal Namespace types.
//! Based on k8s.io/kubernetes/pkg/apis/core/v1/conversion.go

use super::helpers::*;
use crate::common::{ApplyDefault, FromInternal, ToInternal};
use crate::core::internal;
use crate::core::v1::namespace;

// ============================================================================
// Namespace
// ============================================================================

impl ToInternal<internal::Namespace> for namespace::Namespace {
    fn to_internal(self) -> internal::Namespace {
        internal::Namespace {
            type_meta: crate::common::TypeMeta::default(),
            metadata: option_object_meta_to_meta(self.metadata),
            spec: self.spec.map(|s| s.to_internal()),
            status: self.status.map(|s| s.to_internal()).unwrap_or_default(),
        }
    }
}

impl FromInternal<internal::Namespace> for namespace::Namespace {
    fn from_internal(value: internal::Namespace) -> Self {
        let mut result = Self {
            type_meta: crate::common::TypeMeta::default(),
            metadata: meta_to_option_object_meta(value.metadata),
            spec: value.spec.map(namespace::NamespaceSpec::from_internal),
            status: Some(namespace::NamespaceStatus::from_internal(value.status)),
        };
        result.apply_default();
        result
    }
}

// ============================================================================
// NamespaceList
// ============================================================================

impl ToInternal<internal::NamespaceList> for namespace::NamespaceList {
    fn to_internal(self) -> internal::NamespaceList {
        internal::NamespaceList {
            type_meta: crate::common::TypeMeta::default(),
            metadata: option_list_meta_to_meta(self.metadata),
            items: self.items.into_iter().map(|i| i.to_internal()).collect(),
        }
    }
}

impl FromInternal<internal::NamespaceList> for namespace::NamespaceList {
    fn from_internal(value: internal::NamespaceList) -> Self {
        let mut result = Self {
            type_meta: crate::common::TypeMeta::default(),
            metadata: meta_to_option_list_meta(value.metadata),
            items: value
                .items
                .into_iter()
                .map(namespace::Namespace::from_internal)
                .collect(),
        };
        result.apply_default();
        result
    }
}

// ============================================================================
// NamespaceSpec
// ============================================================================

impl ToInternal<internal::NamespaceSpec> for namespace::NamespaceSpec {
    fn to_internal(self) -> internal::NamespaceSpec {
        internal::NamespaceSpec {
            finalizers: self.finalizers,
        }
    }
}

impl FromInternal<internal::NamespaceSpec> for namespace::NamespaceSpec {
    fn from_internal(value: internal::NamespaceSpec) -> Self {
        Self {
            finalizers: value.finalizers,
        }
    }
}

// ============================================================================
// NamespaceStatus
// ============================================================================

impl ToInternal<internal::NamespaceStatus> for namespace::NamespaceStatus {
    fn to_internal(self) -> internal::NamespaceStatus {
        internal::NamespaceStatus {
            phase: option_string_to_namespace_phase(self.phase),
            conditions: self
                .conditions
                .into_iter()
                .map(|c| c.to_internal())
                .collect(),
        }
    }
}

impl FromInternal<internal::NamespaceStatus> for namespace::NamespaceStatus {
    fn from_internal(value: internal::NamespaceStatus) -> Self {
        Self {
            phase: namespace_phase_to_option_string(value.phase),
            conditions: value
                .conditions
                .into_iter()
                .map(namespace::NamespaceCondition::from_internal)
                .collect(),
        }
    }
}

// ============================================================================
// NamespaceCondition
// ============================================================================

impl ToInternal<internal::NamespaceCondition> for namespace::NamespaceCondition {
    fn to_internal(self) -> internal::NamespaceCondition {
        internal::NamespaceCondition {
            type_: string_to_namespace_condition_type(self.type_),
            status: self.status,
            last_transition_time: self.last_transition_time,
            reason: self.reason,
            message: self.message,
        }
    }
}

impl FromInternal<internal::NamespaceCondition> for namespace::NamespaceCondition {
    fn from_internal(value: internal::NamespaceCondition) -> Self {
        Self {
            type_: namespace_condition_type_to_string(value.type_),
            status: value.status,
            last_transition_time: value.last_transition_time,
            reason: value.reason,
            message: value.message,
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_namespace_roundtrip() {
        let v1_namespace = namespace::Namespace {
            type_meta: crate::common::TypeMeta {
                api_version: "v1".to_string(),
                kind: "Namespace".to_string(),
            },
            metadata: Some(crate::common::ObjectMeta {
                name: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(namespace::NamespaceSpec {
                finalizers: vec!["kubernetes".to_string()],
            }),
            status: Some(namespace::NamespaceStatus {
                phase: Some("Active".to_string()),
                conditions: vec![],
            }),
        };

        let internal_namespace = v1_namespace.clone().to_internal();
        assert_eq!(
            internal_namespace.metadata.name,
            Some("default".to_string())
        );
        assert!(matches!(
            internal_namespace.status.phase,
            Some(internal::NamespacePhase::Active)
        ));

        let roundtrip = namespace::Namespace::from_internal(internal_namespace);
        assert_eq!(
            roundtrip.metadata.as_ref().unwrap().name,
            Some("default".to_string())
        );
        assert_eq!(
            roundtrip.status.as_ref().unwrap().phase,
            Some("Active".to_string())
        );
        assert_eq!(roundtrip.type_meta.api_version, "v1");
        assert_eq!(roundtrip.type_meta.kind, "Namespace");
    }

    #[test]
    fn test_namespace_phase_enum_conversion() {
        // Test Active phase
        let v1_status = namespace::NamespaceStatus {
            phase: Some("Active".to_string()),
            conditions: vec![],
        };

        let internal_status = v1_status.to_internal();
        assert!(matches!(
            internal_status.phase,
            Some(internal::NamespacePhase::Active)
        ));

        let roundtrip = namespace::NamespaceStatus::from_internal(internal_status);
        assert_eq!(roundtrip.phase, Some("Active".to_string()));

        // Test Terminating phase
        let v1_status = namespace::NamespaceStatus {
            phase: Some("Terminating".to_string()),
            conditions: vec![],
        };

        let internal_status = v1_status.to_internal();
        assert!(matches!(
            internal_status.phase,
            Some(internal::NamespacePhase::Terminating)
        ));

        // Test None phase
        let v1_status = namespace::NamespaceStatus {
            phase: None,
            conditions: vec![],
        };

        let internal_status = v1_status.to_internal();
        assert!(internal_status.phase.is_none());
    }

    #[test]
    fn test_namespace_condition_type_conversion() {
        let v1_condition = namespace::NamespaceCondition {
            type_: "NamespaceDeletionContentFailure".to_string(),
            status: "True".to_string(),
            last_transition_time: None,
            reason: Some("ResourcesDeleting".to_string()),
            message: None,
        };

        let internal_condition = v1_condition.clone().to_internal();
        assert!(matches!(
            internal_condition.type_,
            internal::NamespaceConditionType::NamespaceDeletionContentFailure
        ));
        assert_eq!(internal_condition.status, "True");

        let roundtrip = namespace::NamespaceCondition::from_internal(internal_condition);
        assert_eq!(roundtrip.type_, "NamespaceDeletionContentFailure");
        assert_eq!(roundtrip.status, "True");
        assert_eq!(roundtrip.reason, Some("ResourcesDeleting".to_string()));
    }

    #[test]
    fn test_namespace_list_roundtrip() {
        let v1_list = namespace::NamespaceList {
            type_meta: crate::common::TypeMeta {
                api_version: "v1".to_string(),
                kind: "NamespaceList".to_string(),
            },
            metadata: Some(crate::common::ListMeta::default()),
            items: vec![
                namespace::Namespace {
                    type_meta: crate::common::TypeMeta::default(),
                    metadata: Some(crate::common::ObjectMeta {
                        name: Some("default".to_string()),
                        ..Default::default()
                    }),
                    spec: None,
                    status: None,
                },
                namespace::Namespace {
                    type_meta: crate::common::TypeMeta::default(),
                    metadata: Some(crate::common::ObjectMeta {
                        name: Some("kube-system".to_string()),
                        ..Default::default()
                    }),
                    spec: None,
                    status: None,
                },
            ],
        };

        let internal_list = v1_list.clone().to_internal();
        assert_eq!(internal_list.items.len(), 2);

        let roundtrip = namespace::NamespaceList::from_internal(internal_list);
        assert_eq!(roundtrip.items.len(), 2);
        assert_eq!(roundtrip.type_meta.api_version, "v1");
        assert_eq!(roundtrip.type_meta.kind, "NamespaceList");
    }
}
