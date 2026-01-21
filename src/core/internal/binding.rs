//! Binding types from the Kubernetes Core API
//!
//! This module contains types for resource bindings and secret references.

use crate::common::ObjectMeta;
use crate::core::v1::reference::ObjectReference;
use serde::{Deserialize, Serialize};

/// Binding binds an object to a target.
///
/// Corresponds to [Kubernetes Binding](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5901)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Binding {
    /// Standard object metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// The target object that the object should be bound to.
    pub target: ObjectReference,
}

/// SecretReference represents a secret reference.
///
/// Corresponds to [Kubernetes SecretReference](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L1429)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SecretReference {
    /// Name is unique within a namespace to reference a secret resource.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// Namespace defines the space within which the secret name must be unique.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub namespace: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binding_default() {
        let binding = Binding::default();
        assert!(binding.metadata.is_none());
    }

    #[test]
    fn test_binding_with_target() {
        let target = ObjectReference {
            kind: Some("Node".to_string()),
            name: Some("node-1".to_string()),
            ..Default::default()
        };

        let binding = Binding {
            metadata: None,
            target: target.clone(),
        };

        assert_eq!(binding.target.kind, Some("Node".to_string()));
        assert_eq!(binding.target.name, Some("node-1".to_string()));
    }

    #[test]
    fn test_binding_serialize() {
        let binding = Binding {
            metadata: None,
            target: ObjectReference {
                kind: Some("Node".to_string()),
                name: Some("node-2".to_string()),
                ..Default::default()
            },
        };

        let json = serde_json::to_string(&binding).unwrap();
        assert!(json.contains("\"kind\":\"Node\""));
        assert!(json.contains("\"name\":\"node-2\""));
        // None metadata should be omitted
        assert!(!json.contains("\"metadata\""));
    }

    #[test]
    fn test_binding_with_metadata() {
        let metadata = ObjectMeta {
            name: Some("pod-binding".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        };

        let binding = Binding {
            metadata: Some(metadata),
            target: ObjectReference {
                kind: Some("Node".to_string()),
                name: Some("node-1".to_string()),
                ..Default::default()
            },
        };

        let json = serde_json::to_string(&binding).unwrap();
        assert!(json.contains("\"metadata\""));
        assert!(json.contains("\"name\":\"pod-binding\""));
    }

    #[test]
    fn test_binding_deserialize() {
        let json = r#"{"target":{"kind":"Node","name":"node-3"}}"#;
        let binding: Binding = serde_json::from_str(json).unwrap();

        assert_eq!(binding.target.kind, Some("Node".to_string()));
        assert_eq!(binding.target.name, Some("node-3".to_string()));
    }

    #[test]
    fn test_binding_round_trip() {
        let metadata = ObjectMeta {
            name: Some("test-binding".to_string()),
            namespace: Some("test".to_string()),
            ..Default::default()
        };

        let original = Binding {
            metadata: Some(metadata),
            target: ObjectReference {
                kind: Some("Pod".to_string()),
                name: Some("pod-1".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            },
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Binding = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_secret_reference_default() {
        let secret_ref = SecretReference::default();
        assert!(secret_ref.name.is_empty());
        assert!(secret_ref.namespace.is_empty());
    }

    #[test]
    fn test_secret_reference_with_fields() {
        let secret_ref = SecretReference {
            name: "my-secret".to_string(),
            namespace: "default".to_string(),
        };

        assert_eq!(secret_ref.name, "my-secret");
        assert_eq!(secret_ref.namespace, "default");
    }

    #[test]
    fn test_secret_reference_serialize() {
        let secret_ref = SecretReference {
            name: "app-secret".to_string(),
            namespace: "production".to_string(),
        };

        let json = serde_json::to_string(&secret_ref).unwrap();
        assert!(json.contains("\"name\":\"app-secret\""));
        assert!(json.contains("\"namespace\":\"production\""));
    }

    #[test]
    fn test_secret_reference_empty_fields_omitted() {
        let secret_ref = SecretReference {
            name: String::new(),
            namespace: String::new(),
        };

        let json = serde_json::to_string(&secret_ref).unwrap();
        // empty fields should be omitted
        assert!(!json.contains("\"name\""));
        assert!(!json.contains("\"namespace\""));
    }

    #[test]
    fn test_secret_reference_deserialize() {
        let json = r#"{"name":"db-secret","namespace":"kube-system"}"#;
        let secret_ref: SecretReference = serde_json::from_str(json).unwrap();

        assert_eq!(secret_ref.name, "db-secret");
        assert_eq!(secret_ref.namespace, "kube-system");
    }

    #[test]
    fn test_secret_reference_round_trip() {
        let original = SecretReference {
            name: "tls-cert".to_string(),
            namespace: "ingress-nginx".to_string(),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: SecretReference = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_secret_reference_partial() {
        let secret_ref = SecretReference {
            name: "config-secret".to_string(),
            namespace: String::new(),
        };

        let json = serde_json::to_string(&secret_ref).unwrap();
        assert!(json.contains("\"name\":\"config-secret\""));
        // empty namespace should be omitted
        assert!(!json.contains("\"namespace\""));
    }
}
