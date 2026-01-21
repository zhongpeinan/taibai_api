//! Binding types from the Kubernetes Core v1 API
//!
//! This module contains types for binding objects to other objects.

use crate::common::ObjectMeta;
use crate::core::v1::reference::ObjectReference;
use serde::{Deserialize, Serialize};

/// Binding binds one object to another.
///
/// For example, a pod is bound to a node by a Binding.
///
/// Corresponds to [Kubernetes Binding](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7159)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Binding {
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// The target object that you want to bind to the standard object.
    pub target: ObjectReference,
}

/// Preconditions must be fulfilled before an operation (update, delete, etc.) is carried out.
///
/// Corresponds to [Kubernetes Preconditions](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7172)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Preconditions {
    /// Specifies the target UID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binding_with_target() {
        let binding = Binding {
            metadata: None,
            target: ObjectReference {
                kind: "Node".to_string(),
                name: Some("node-1".to_string()),
                ..Default::default()
            },
        };
        assert_eq!(binding.target.kind, Some("Node".to_string()));
        assert_eq!(binding.target.name, Some("node-1".to_string()));
    }

    #[test]
    fn test_binding_serialize() {
        let binding = Binding {
            metadata: Some(ObjectMeta {
                name: Some("my-pod".to_string()),
                ..Default::default()
            }),
            target: ObjectReference {
                kind: "Node".to_string(),
                name: Some("node-1".to_string()),
                ..Default::default()
            },
        };
        let json = serde_json::to_string(&binding).unwrap();
        assert!(json.contains("\"name\":\"my-pod\""));
        assert!(json.contains("\"target\""));
        assert!(json.contains("\"kind\":\"Node\""));
    }

    #[test]
    fn test_binding_deserialize() {
        let json = r#"{
            "metadata": {"name": "my-pod"},
            "target": {"kind": "Node", "name": "node-1"}
        }"#;
        let binding: Binding = serde_json::from_str(json).unwrap();
        assert_eq!(
            binding.metadata.as_ref().unwrap().name,
            Some("my-pod".to_string())
        );
        assert_eq!(binding.target.kind, Some("Node".to_string()));
        assert_eq!(binding.target.name, Some("node-1".to_string()));
    }

    #[test]
    fn test_binding_round_trip() {
        let original = Binding {
            metadata: Some(ObjectMeta {
                name: Some("my-pod".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            target: ObjectReference {
                kind: "Node".to_string(),
                name: Some("node-1".to_string()),
                uid: Some("node-uid-123".to_string()),
                ..Default::default()
            },
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Binding = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_preconditions_default() {
        let precond = Preconditions::default();
        assert!(precond.uid.is_none());
    }

    #[test]
    fn test_preconditions_with_uid() {
        let precond = Preconditions {
            uid: Some("abc-123".to_string()),
        };
        assert_eq!(precond.uid, Some("abc-123".to_string()));
    }

    #[test]
    fn test_preconditions_serialize() {
        let precond = Preconditions {
            uid: Some("abc-123".to_string()),
        };
        let json = serde_json::to_string(&precond).unwrap();
        assert!(json.contains("\"uid\":\"abc-123\""));
    }

    #[test]
    fn test_preconditions_deserialize() {
        let json = r#"{"uid":"abc-123"}"#;
        let precond: Preconditions = serde_json::from_str(json).unwrap();
        assert_eq!(precond.uid, Some("abc-123".to_string()));
    }

    #[test]
    fn test_preconditions_round_trip() {
        let original = Preconditions {
            uid: Some("test-uid-456".to_string()),
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Preconditions = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_preconditions_empty_uid_omitted() {
        let precond = Preconditions { uid: None };
        let json = serde_json::to_string(&precond).unwrap();
        // Empty object when uid is None
        assert_eq!(json, "{}");
    }
}
