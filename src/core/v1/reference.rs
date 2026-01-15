//! Object reference types from the Kubernetes Core v1 API
//!
//! This module contains types for referencing Kubernetes objects.

use serde::{Deserialize, Serialize};

/// ObjectReference contains enough information to let you inspect or modify the referred object.
///
/// Corresponds to [Kubernetes ObjectReference](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7408)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ObjectReference {
    /// Kind of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// Namespace of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    /// Name of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// UID of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,

    /// API version of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,

    /// Specific resourceVersion to which this reference is made, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_version: Option<String>,

    /// If referring to a piece of an object instead of an entire object, this string
    /// should contain a valid JSON/Go field access statement.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_path: Option<String>,
}

/// LocalObjectReference is a reference to another object within the same namespace.
///
/// Corresponds to [Kubernetes LocalObjectReference](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7459)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct LocalObjectReference {
    /// Name of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// TypedLocalObjectReference is a reference to another object within the same namespace
/// with a specific kind and optionally an API group.
///
/// Corresponds to [Kubernetes TypedLocalObjectReference](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7489)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TypedLocalObjectReference {
    /// APIGroup is the group for the resource being referenced.
    /// If APIGroup is not specified, the specified Kind must be in the core API group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_group: Option<String>,

    /// Kind is the type of resource being referenced.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// Name is the name of resource being referenced.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_reference_default() {
        let ref_obj = ObjectReference::default();
        assert!(ref_obj.kind.is_none());
        assert!(ref_obj.name.is_none());
    }

    #[test]
    fn test_object_reference_with_fields() {
        let ref_obj = ObjectReference {
            kind: Some("Pod".to_string()),
            namespace: Some("default".to_string()),
            name: Some("my-pod".to_string()),
            uid: Some("abc-123".to_string()),
            ..Default::default()
        };
        assert_eq!(ref_obj.kind, Some("Pod".to_string()));
        assert_eq!(ref_obj.namespace, Some("default".to_string()));
        assert_eq!(ref_obj.name, Some("my-pod".to_string()));
    }

    #[test]
    fn test_object_reference_serialize() {
        let ref_obj = ObjectReference {
            kind: Some("Pod".to_string()),
            name: Some("my-pod".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_string(&ref_obj).unwrap();
        assert!(json.contains("\"kind\":\"Pod\""));
        assert!(json.contains("\"name\":\"my-pod\""));
    }

    #[test]
    fn test_object_reference_deserialize() {
        let json = r#"{"kind":"Pod","name":"my-pod","namespace":"default"}"#;
        let ref_obj: ObjectReference = serde_json::from_str(json).unwrap();
        assert_eq!(ref_obj.kind, Some("Pod".to_string()));
        assert_eq!(ref_obj.name, Some("my-pod".to_string()));
        assert_eq!(ref_obj.namespace, Some("default".to_string()));
    }

    #[test]
    fn test_object_reference_round_trip() {
        let original = ObjectReference {
            kind: Some("Pod".to_string()),
            namespace: Some("default".to_string()),
            name: Some("my-pod".to_string()),
            uid: Some("abc-123".to_string()),
            api_version: Some("v1".to_string()),
            resource_version: Some("12345".to_string()),
            field_path: Some("spec.containers[0]".to_string()),
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: ObjectReference = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_local_object_reference_default() {
        let local_ref = LocalObjectReference::default();
        assert!(local_ref.name.is_none());
    }

    #[test]
    fn test_local_object_reference_with_name() {
        let local_ref = LocalObjectReference {
            name: Some("my-config".to_string()),
        };
        assert_eq!(local_ref.name, Some("my-config".to_string()));
    }

    #[test]
    fn test_local_object_reference_serialize() {
        let local_ref = LocalObjectReference {
            name: Some("my-config".to_string()),
        };
        let json = serde_json::to_string(&local_ref).unwrap();
        assert!(json.contains("\"name\":\"my-config\""));
    }

    #[test]
    fn test_local_object_reference_round_trip() {
        let original = LocalObjectReference {
            name: Some("my-config".to_string()),
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: LocalObjectReference = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_typed_local_object_reference_default() {
        let typed_ref = TypedLocalObjectReference::default();
        assert!(typed_ref.kind.is_none());
        assert!(typed_ref.name.is_none());
    }

    #[test]
    fn test_typed_local_object_reference_with_fields() {
        let typed_ref = TypedLocalObjectReference {
            api_group: Some("apps".to_string()),
            kind: Some("Deployment".to_string()),
            name: Some("my-deployment".to_string()),
        };
        assert_eq!(typed_ref.api_group, Some("apps".to_string()));
        assert_eq!(typed_ref.kind, Some("Deployment".to_string()));
    }

    #[test]
    fn test_typed_local_object_reference_serialize() {
        let typed_ref = TypedLocalObjectReference {
            api_group: Some("apps".to_string()),
            kind: Some("Deployment".to_string()),
            name: Some("my-deployment".to_string()),
        };
        let json = serde_json::to_string(&typed_ref).unwrap();
        assert!(json.contains("\"apiGroup\":\"apps\""));
        assert!(json.contains("\"kind\":\"Deployment\""));
    }

    #[test]
    fn test_typed_local_object_reference_round_trip() {
        let original = TypedLocalObjectReference {
            api_group: Some("".to_string()),
            kind: Some("Secret".to_string()),
            name: Some("my-secret".to_string()),
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: TypedLocalObjectReference = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_object_reference_empty_fields_omitted() {
        let ref_obj = ObjectReference {
            kind: Some("Pod".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_string(&ref_obj).unwrap();
        // Only kind should be present, other fields should be omitted
        assert!(json.contains("\"kind\":\"Pod\""));
        assert!(!json.contains("\"namespace\""));
        assert!(!json.contains("\"name\""));
    }
}
