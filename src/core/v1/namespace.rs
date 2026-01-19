//! Namespace types from the Kubernetes Core v1 API
//!
//! This module contains the Namespace type and its associated spec and status types.

use crate::common::{ListMeta, ObjectMeta, Timestamp};
use serde::{Deserialize, Serialize};

/// Namespace provides a scope for names.
///
/// Names of resources need to be unique within a namespace, but not across namespaces.
///
/// Corresponds to [Kubernetes Namespace](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7121)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Namespace {
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// Spec defines the behavior of the Namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<NamespaceSpec>,

    /// Status describes the current status of a Namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<NamespaceStatus>,
}

/// NamespaceList is a list of Namespaces.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct NamespaceList {
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// Items is the list of Namespace objects in the list.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Namespace>,
}

/// NamespaceSpec describes the attributes on a Namespace.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NamespaceSpec {
    /// Finalizers is an opaque list of values that must be empty to permanently remove object from storage.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub finalizers: Vec<String>,
}

/// NamespaceStatus is information about the current status of a Namespace.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NamespaceStatus {
    /// Phase is the current lifecycle phase of the namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,

    /// Represents the latest available observations of a namespace's current state.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<NamespaceCondition>,
}

/// NamespaceCondition is an condition of a namespace.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct NamespaceCondition {
    /// Type of namespace controller condition.
    #[serde(rename = "type")]
    pub type_: String,

    /// Status of the condition, one of True, False, Unknown.
    pub status: String,

    /// Last time the condition transitioned from one status to another.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<Timestamp>,

    /// Unique, one-word, CamelCase reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    /// Human-readable message indicating details about last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Namespace phase constants
pub mod namespace_phase {
    /// NamespaceActive means the namespace is available for use in the system
    pub const ACTIVE: &str = "Active";
    /// NamespaceTerminating means the namespace is undergoing graceful termination
    pub const TERMINATING: &str = "Terminating";
}

/// Namespace condition type constants
pub mod namespace_condition_type {
    /// NamespaceDeletionDiscoveryFailure contains information about namespace deleter errors during resource discovery.
    pub const DELETION_DISCOVERY_FAILURE: &str = "NamespaceDeletionDiscoveryFailure";
    /// NamespaceDeletionContentFailure contains information about namespace deleter errors during deletion of resources.
    pub const DELETION_CONTENT_FAILURE: &str = "NamespaceDeletionContentFailure";
    /// NamespaceDeletionGVParsingFailure contains information about namespace deleter errors parsing GV for legacy types.
    pub const DELETION_GV_PARSING_FAILURE: &str = "NamespaceDeletionGVParsingFailure";
}

/// Condition status constants
pub mod condition_status {
    /// ConditionTrue means a resource is in the condition.
    pub const TRUE: &str = "True";
    /// ConditionFalse means a resource is not in the condition.
    pub const FALSE: &str = "False";
    /// ConditionUnknown means kubernetes can't decide if a resource is in the condition or not.
    pub const UNKNOWN: &str = "Unknown";
}

/// Finalizer name constants
pub mod finalizer_name {
    /// FinalizerKubernetes is the kubernetes finalizer value
    pub const KUBERNETES: &str = "kubernetes";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_namespace_default() {
        let ns = Namespace {
            metadata: None,
            spec: None,
            status: None,
        };
        assert!(ns.metadata.is_none());
        assert!(ns.spec.is_none());
        assert!(ns.status.is_none());
    }

    #[test]
    fn test_namespace_with_metadata() {
        let metadata = ObjectMeta {
            name: Some("my-namespace".to_string()),
            ..Default::default()
        };
        let ns = Namespace {
            metadata: Some(metadata),
            spec: None,
            status: None,
        };
        assert_eq!(
            ns.metadata.as_ref().unwrap().name,
            Some("my-namespace".to_string())
        );
    }

    #[test]
    fn test_namespace_spec() {
        let spec = NamespaceSpec {
            finalizers: vec![finalizer_name::KUBERNETES.to_string()],
        };
        assert_eq!(spec.finalizers.len(), 1);
        assert_eq!(spec.finalizers[0], finalizer_name::KUBERNETES);
    }

    #[test]
    fn test_namespace_status() {
        let status = NamespaceStatus {
            phase: Some(namespace_phase::ACTIVE.to_string()),
            conditions: vec![],
        };
        assert_eq!(status.phase, Some(namespace_phase::ACTIVE.to_string()));
    }

    #[test]
    fn test_namespace_condition() {
        let condition = NamespaceCondition {
            type_: namespace_condition_type::DELETION_DISCOVERY_FAILURE.to_string(),
            status: condition_status::TRUE.to_string(),
            last_transition_time: Some(Timestamp::from_str("2024-01-15T10:00:00Z").unwrap()),
            reason: Some("DiscoveryFailed".to_string()),
            message: Some("Failed to discover resources".to_string()),
        };
        assert_eq!(
            condition.type_,
            namespace_condition_type::DELETION_DISCOVERY_FAILURE
        );
        assert_eq!(condition.status, condition_status::TRUE);
    }

    #[test]
    fn test_namespace_list() {
        let ns_list = NamespaceList {
            metadata: Some(ListMeta {
                resource_version: Some("12345".to_string()),
                ..Default::default()
            }),
            items: vec![Namespace {
                metadata: Some(ObjectMeta {
                    name: Some("default".to_string()),
                    ..Default::default()
                }),
                spec: None,
                status: None,
            }],
        };
        assert_eq!(ns_list.items.len(), 1);
        assert_eq!(
            ns_list.metadata.as_ref().unwrap().resource_version,
            Some("12345".to_string())
        );
    }

    #[test]
    fn test_namespace_serialize() {
        let ns = Namespace {
            metadata: Some(ObjectMeta {
                name: Some("my-namespace".to_string()),
                ..Default::default()
            }),
            spec: Some(NamespaceSpec {
                finalizers: vec![finalizer_name::KUBERNETES.to_string()],
            }),
            status: Some(NamespaceStatus {
                phase: Some(namespace_phase::ACTIVE.to_string()),
                conditions: vec![],
            }),
        };

        let json = serde_json::to_string(&ns).unwrap();
        assert!(json.contains("\"name\":\"my-namespace\""));
        assert!(json.contains("\"phase\":\"Active\""));
        assert!(json.contains("\"finalizers\""));
    }

    #[test]
    fn test_namespace_deserialize() {
        let json = r#"{
            "metadata": {"name": "my-namespace"},
            "spec": {"finalizers": ["kubernetes"]},
            "status": {"phase": "Active"}
        }"#;
        let ns: Namespace = serde_json::from_str(json).unwrap();
        assert_eq!(
            ns.metadata.as_ref().unwrap().name,
            Some("my-namespace".to_string())
        );
        assert_eq!(ns.spec.as_ref().unwrap().finalizers.len(), 1);
        assert_eq!(
            ns.status.as_ref().unwrap().phase,
            Some(namespace_phase::ACTIVE.to_string())
        );
    }

    #[test]
    fn test_namespace_round_trip() {
        let original = Namespace {
            metadata: Some(ObjectMeta {
                name: Some("my-namespace".to_string()),
                ..Default::default()
            }),
            spec: Some(NamespaceSpec { finalizers: vec![] }),
            status: Some(NamespaceStatus {
                phase: Some(namespace_phase::ACTIVE.to_string()),
                conditions: vec![NamespaceCondition {
                    type_: "TestCondition".to_string(),
                    status: condition_status::TRUE.to_string(),
                    last_transition_time: None,
                    reason: None,
                    message: None,
                }],
            }),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Namespace = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_namespace_phase_constants() {
        assert_eq!(namespace_phase::ACTIVE, "Active");
        assert_eq!(namespace_phase::TERMINATING, "Terminating");
    }

    #[test]
    fn test_condition_status_constants() {
        assert_eq!(condition_status::TRUE, "True");
        assert_eq!(condition_status::FALSE, "False");
        assert_eq!(condition_status::UNKNOWN, "Unknown");
    }

    #[test]
    fn test_finalizer_name_constants() {
        assert_eq!(finalizer_name::KUBERNETES, "kubernetes");
    }

    #[test]
    fn test_namespace_condition_serialize() {
        let condition = NamespaceCondition {
            type_: namespace_condition_type::DELETION_CONTENT_FAILURE.to_string(),
            status: condition_status::FALSE.to_string(),
            last_transition_time: None,
            reason: None,
            message: None,
        };
        let json = serde_json::to_string(&condition).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(
            parsed["type"],
            namespace_condition_type::DELETION_CONTENT_FAILURE
        );
        assert_eq!(parsed["status"], condition_status::FALSE);
    }
}
