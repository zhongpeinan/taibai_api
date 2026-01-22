//! Namespace-related types from the Kubernetes Core API
//!
//! This module contains types for Kubernetes namespaces.

use crate::common::time::Timestamp;
use crate::impl_has_object_meta;
use crate::common::{ListMeta, ObjectMeta, TypeMeta};
use crate::core::internal::{NamespaceConditionType, NamespacePhase};
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
mod tests {
    use super::*;

    // ============================================================================
    // Namespace Tests
    // ============================================================================

    #[test]
    fn test_namespace_default() {
        let ns = Namespace::default();
        assert!(ns.metadata.is_none());
        assert!(ns.spec.is_none());
        assert!(ns.status.phase.is_none());
    }

    #[test]
    fn test_namespace_with_metadata() {
        let ns = Namespace {
            type_meta: TypeMeta {
                kind: "Namespace".to_string(),
                api_version: "v1".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("test-namespace".to_string()),
                ..Default::default()
            }),
            spec: None,
            status: NamespaceStatus::default(),
        };

        assert!(ns.metadata.is_some());
        assert_eq!(ns.metadata.unwrap().name.unwrap(), "test-namespace");
    }

    #[test]
    fn test_namespace_with_spec() {
        let ns = Namespace {
            type_meta: TypeMeta::default(),
            metadata: None,
            spec: Some(NamespaceSpec {
                finalizers: vec!["kubernetes".to_string()],
            }),
            status: NamespaceStatus::default(),
        };

        assert!(ns.spec.is_some());
        assert_eq!(ns.spec.unwrap().finalizers.len(), 1);
    }

    #[test]
    fn test_namespace_serialize() {
        let ns = Namespace {
            type_meta: TypeMeta {
                kind: "Namespace".to_string(),
                api_version: "v1".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("my-namespace".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };

        let json = serde_json::to_string(&ns).unwrap();
        assert!(json.contains(r#""kind":"Namespace""#));
        assert!(json.contains(r#""name":"my-namespace""#));
    }

    #[test]
    fn test_namespace_deserialize() {
        let json = r#"{
            "kind": "Namespace",
            "apiVersion": "v1",
            "metadata": {"name": "test-ns"},
            "spec": {},
            "status": {}
        }"#;

        let ns: Namespace = serde_json::from_str(json).unwrap();
        assert_eq!(ns.type_meta.kind, "Namespace");
        assert_eq!(ns.metadata.unwrap().name.unwrap(), "test-ns");
    }

    #[test]
    fn test_namespace_round_trip() {
        let original = Namespace {
            type_meta: TypeMeta {
                kind: "Namespace".to_string(),
                api_version: "v1".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("round-trip-ns".to_string()),
                ..Default::default()
            }),
            spec: Some(NamespaceSpec {
                finalizers: vec!["kubernetes".to_string()],
            }),
            ..Default::default()
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Namespace = serde_json::from_str(&json).unwrap();

        assert_eq!(
            original.metadata.unwrap().name,
            deserialized.metadata.unwrap().name
        );
    }

    #[test]
    fn test_namespace_with_status() {
        let ns = Namespace {
            type_meta: TypeMeta::default(),
            metadata: None,
            spec: None,
            status: NamespaceStatus {
                phase: Some(NamespacePhase::Active),
                ..Default::default()
            },
        };

        assert_eq!(ns.status.phase, Some(NamespacePhase::Active));
    }

    // ============================================================================
    // NamespaceList Tests
    // ============================================================================

    #[test]
    fn test_namespace_list_default() {
        let list = NamespaceList::default();
        assert!(list.items.is_empty());
    }

    #[test]
    fn test_namespace_list_with_items() {
        let list = NamespaceList {
            items: vec![Namespace {
                type_meta: TypeMeta {
                    kind: "Namespace".to_string(),
                    api_version: "v1".to_string(),
                },
                ..Default::default()
            }],
            ..Default::default()
        };

        assert_eq!(list.items.len(), 1);
    }

    #[test]
    fn test_namespace_list_serialize() {
        let list = NamespaceList {
            type_meta: TypeMeta {
                kind: "NamespaceList".to_string(),
                api_version: "v1".to_string(),
            },
            ..Default::default()
        };

        let json = serde_json::to_string(&list).unwrap();
        assert!(json.contains(r#""kind":"NamespaceList""#));
    }

    #[test]
    fn test_namespace_list_with_multiple_namespaces() {
        let list = NamespaceList {
            items: vec![
                Namespace {
                    type_meta: TypeMeta {
                        kind: "Namespace".to_string(),
                        api_version: "v1".to_string(),
                    },
                    metadata: Some(ObjectMeta {
                        name: Some("default".to_string()),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                Namespace {
                    type_meta: TypeMeta {
                        kind: "Namespace".to_string(),
                        api_version: "v1".to_string(),
                    },
                    metadata: Some(ObjectMeta {
                        name: Some("kube-system".to_string()),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };

        assert_eq!(list.items.len(), 2);
    }

    // ============================================================================
    // NamespaceSpec Tests
    // ============================================================================

    #[test]
    fn test_namespace_spec_default() {
        let spec = NamespaceSpec::default();
        assert!(spec.finalizers.is_empty());
    }

    #[test]
    fn test_namespace_spec_with_finalizers() {
        let spec = NamespaceSpec {
            finalizers: vec![
                "kubernetes".to_string(),
                "example.com/finalizer".to_string(),
            ],
        };

        assert_eq!(spec.finalizers.len(), 2);
        assert_eq!(spec.finalizers[0], "kubernetes");
    }

    #[test]
    fn test_namespace_spec_serialize() {
        let spec = NamespaceSpec {
            finalizers: vec!["kubernetes".to_string()],
        };

        let json = serde_json::to_string(&spec).unwrap();
        assert!(json.contains("\"finalizers\":[\"kubernetes\"]"));
    }

    #[test]
    fn test_namespace_spec_empty_finalizers_omitted() {
        let spec = NamespaceSpec { finalizers: vec![] };

        let json = serde_json::to_string(&spec).unwrap();
        // empty finalizers should be omitted
        assert!(!json.contains("\"finalizers\""));
    }

    #[test]
    fn test_namespace_spec_round_trip() {
        let original = NamespaceSpec {
            finalizers: vec!["kubernetes".to_string(), "custom.io/protection".to_string()],
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: NamespaceSpec = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_namespace_status_default() {
        let status = NamespaceStatus::default();
        assert!(status.phase.is_none());
        assert!(status.conditions.is_empty());
    }

    #[test]
    fn test_namespace_status_with_phase() {
        let status = NamespaceStatus {
            phase: Some(NamespacePhase::Active),
            conditions: vec![],
        };

        assert_eq!(status.phase, Some(NamespacePhase::Active));
    }

    #[test]
    fn test_namespace_status_serialize() {
        let status = NamespaceStatus {
            phase: Some(NamespacePhase::Terminating),
            conditions: vec![],
        };

        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains("\"phase\":\"Terminating\""));
        // empty conditions should be omitted
        assert!(!json.contains("\"conditions\""));
    }

    #[test]
    fn test_namespace_status_with_conditions() {
        let condition = NamespaceCondition {
            type_: NamespaceConditionType::NamespaceDeletionDiscoveryFailure,
            status: "True".to_string(),
            last_transition_time: Some(Timestamp::from_str("2024-01-15T10:00:00Z").unwrap()),
            reason: Some("DiscoveryFailed".to_string()),
            message: Some("Failed to discover resources".to_string()),
        };

        let status = NamespaceStatus {
            phase: Some(NamespacePhase::Terminating),
            conditions: vec![condition],
        };

        assert_eq!(status.conditions.len(), 1);
        assert_eq!(
            status.conditions[0].type_,
            NamespaceConditionType::NamespaceDeletionDiscoveryFailure
        );
    }

    #[test]
    fn test_namespace_status_round_trip() {
        let condition = NamespaceCondition {
            type_: NamespaceConditionType::NamespaceDeletionContentFailure,
            status: "False".to_string(),
            last_transition_time: None,
            reason: None,
            message: None,
        };

        let original = NamespaceStatus {
            phase: Some(NamespacePhase::Active),
            conditions: vec![condition],
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: NamespaceStatus = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_namespace_condition_default() {
        let condition = NamespaceCondition::default();
        assert!(condition.status.is_empty());
        assert!(condition.last_transition_time.is_none());
        assert!(condition.reason.is_none());
        assert!(condition.message.is_none());
    }

    #[test]
    fn test_namespace_condition_with_all_fields() {
        let condition = NamespaceCondition {
            type_: NamespaceConditionType::NamespaceDeletionGroupVersionParsingFailure,
            status: "True".to_string(),
            last_transition_time: Some(Timestamp::from_str("2024-01-15T10:00:00Z").unwrap()),
            reason: Some("ParsingError".to_string()),
            message: Some("Failed to parse GroupVersion".to_string()),
        };

        assert_eq!(
            condition.type_,
            NamespaceConditionType::NamespaceDeletionGroupVersionParsingFailure
        );
        assert_eq!(condition.status, "True");
        assert_eq!(
            condition
                .last_transition_time
                .as_ref()
                .unwrap()
                .to_rfc3339(),
            "2024-01-15T10:00:00Z"
        );
    }

    #[test]
    fn test_namespace_condition_serialize() {
        let condition = NamespaceCondition {
            type_: NamespaceConditionType::NamespaceDeletionDiscoveryFailure,
            status: "False".to_string(),
            last_transition_time: None,
            reason: None,
            message: None,
        };

        let json = serde_json::to_string(&condition).unwrap();
        assert!(json.contains("\"type\":\"NamespaceDeletionDiscoveryFailure\""));
        assert!(json.contains("\"status\":\"False\""));
        // None fields should be omitted
        assert!(!json.contains("\"lastTransitionTime\""));
        assert!(!json.contains("\"reason\""));
        assert!(!json.contains("\"message\""));
    }

    #[test]
    fn test_namespace_condition_round_trip() {
        let original = NamespaceCondition {
            type_: NamespaceConditionType::NamespaceDeletionContentFailure,
            status: "Unknown".to_string(),
            last_transition_time: Some(Timestamp::from_str("2024-01-15T10:00:00Z").unwrap()),
            reason: Some("ContentError".to_string()),
            message: Some("Failed to delete content".to_string()),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: NamespaceCondition = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_namespace_phase_serialize() {
        let json = serde_json::to_string(&NamespacePhase::Active).unwrap();
        assert_eq!(json, r#""Active""#);

        let json = serde_json::to_string(&NamespacePhase::Terminating).unwrap();
        assert_eq!(json, r#""Terminating""#);
    }

    #[test]
    fn test_namespace_phase_deserialize() {
        let phase: NamespacePhase = serde_json::from_str(r#""Active""#).unwrap();
        assert_eq!(phase, NamespacePhase::Active);

        let phase: NamespacePhase = serde_json::from_str(r#""Terminating""#).unwrap();
        assert_eq!(phase, NamespacePhase::Terminating);
    }

    #[test]
    fn test_namespace_condition_type_serialize() {
        let json =
            serde_json::to_string(&NamespaceConditionType::NamespaceDeletionDiscoveryFailure)
                .unwrap();
        assert_eq!(json, r#""NamespaceDeletionDiscoveryFailure""#);

        let json = serde_json::to_string(&NamespaceConditionType::NamespaceDeletionContentFailure)
            .unwrap();
        assert_eq!(json, r#""NamespaceDeletionContentFailure""#);
    }

    #[test]
    fn test_namespace_condition_type_deserialize() {
        let cond_type: NamespaceConditionType =
            serde_json::from_str(r#""NamespaceDeletionDiscoveryFailure""#).unwrap();
        assert_eq!(
            cond_type,
            NamespaceConditionType::NamespaceDeletionDiscoveryFailure
        );
    }
}
