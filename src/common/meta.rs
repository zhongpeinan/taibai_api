//! Kubernetes common metadata types
//!
//! This module contains the fundamental metadata types used across Kubernetes API objects.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// TypeMeta describes an individual object in an API response or request
/// with Kind and Version fields.
///
/// Corresponds to [Kubernetes TypeMeta](https://github.com/kubernetes/apimachinery/blob/master/pkg/apis/meta/v1/types.go#L42)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct TypeMeta {
    /// Kind is a string value representing the REST resource this object represents.
    ///
    /// Servers may infer this from the endpoint the client submits requests to.
    /// Cannot be updated.
    /// In CamelCase.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// APIVersion defines the versioned schema of this representation of an object.
    /// Servers should convert recognized schemas to the latest internal value,
    /// and may reject unrecognized values.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
    #[serde(
        rename = "apiVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub api_version: Option<String>,
}

/// ObjectMeta is metadata that all persisted resources must have, which includes all objects
/// users must create.
///
/// Note: This is a simplified version without time fields (creationTimestamp, deletionTimestamp, etc.)
/// since chrono is temporarily disabled due to Windows compilation issues.
///
/// Corresponds to [Kubernetes ObjectMeta](https://github.com/kubernetes/apimachinery/blob/master/pkg/apis/meta/v1/types.go#L110)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct ObjectMeta {
    /// Name must be unique within a namespace.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Namespace defines the space within which each name must be unique.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    /// UID is the unique in time and space value for this object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,

    /// An opaque value that represents the internal version of this object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_version: Option<String>,

    /// A sequence number representing a specific generation of the desired state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generation: Option<i64>,

    /// SelfLink is a URL representing this object.
    #[serde(rename = "selfLink", default, skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,

    /// Map of string keys and values that can be used to organize and categorize objects.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub labels: HashMap<String, String>,

    /// Annotations is an unstructured key value map stored with a resource.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub annotations: HashMap<String, String>,

    /// The name of the cluster which the object belongs to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,

    /// ManagedFields maps workflow-id and version to the set of fields that are managed by that workflow.
    #[serde(
        rename = "managedFields",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub managed_fields: Vec<ManagedFieldsEntry>,
}

/// ManagedFieldsEntry is a workflow-id, a FieldSet and the group version of the resource
/// that the fieldset applies to.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ManagedFieldsEntry {
    /// Manager is an identifier of the workflow managing these fields.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manager: Option<String>,

    /// Operation is the type of operation which lead to this ManagedFieldsEntry being merged.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,

    /// APIVersion defines the version of this resource that this field set applies to.
    #[serde(
        rename = "apiVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub api_version: Option<String>,

    /// Time is the timestamp of when the ManagedFieldsEntry was added.
    /// Note: Excluded for now since chrono is temporarily disabled.
    // #[serde(default, skip_serializing_if = "Option::is_none")]
    // pub time: Option<chrono::DateTime<chrono::Utc>>,

    /// FieldsType is the discriminator for the different fields format and version.
    #[serde(
        rename = "fieldsType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fields_type: Option<String>,

    /// FieldsV1 holds the first JSON version of the fields.
    #[serde(rename = "fieldsV1", default, skip_serializing_if = "Option::is_none")]
    pub fields_v1: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_meta_default() {
        let tm = TypeMeta::default();
        assert!(tm.kind.is_none());
        assert!(tm.api_version.is_none());
    }

    #[test]
    fn test_type_meta_with_values() {
        let tm = TypeMeta {
            kind: Some("Pod".to_string()),
            api_version: Some("v1".to_string()),
        };
        assert_eq!(tm.kind, Some("Pod".to_string()));
        assert_eq!(tm.api_version, Some("v1".to_string()));
    }

    #[test]
    fn test_serialize_deserialize() {
        let tm = TypeMeta {
            kind: Some("Pod".to_string()),
            api_version: Some("v1".to_string()),
        };

        let json = serde_json::to_string(&tm).unwrap();
        assert!(json.contains("\"kind\":\"Pod\""));
        assert!(json.contains("\"apiVersion\":\"v1\""));

        let deserialized: TypeMeta = serde_json::from_str(&json).unwrap();
        assert_eq!(tm, deserialized);
    }

    #[test]
    fn test_serialize_none_fields_omitted() {
        let tm = TypeMeta::default();
        let json = serde_json::to_string(&tm).unwrap();
        // Empty object when all fields are None (matches Go omitempty behavior)
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_json_format_matches_go() {
        // Verify JSON output matches Go's format exactly
        let tm = TypeMeta {
            kind: Some("Pod".to_string()),
            api_version: Some("v1".to_string()),
        };
        let json = serde_json::to_string(&tm).unwrap();
        // Go: {"kind":"Pod","apiVersion":"v1"}
        assert_eq!(json, r#"{"kind":"Pod","apiVersion":"v1"}"#);
    }

    #[test]
    fn test_deserialize_with_partial_fields() {
        let json = r#"{"kind":"Pod"}"#;
        let tm: TypeMeta = serde_json::from_str(json).unwrap();
        assert_eq!(tm.kind, Some("Pod".to_string()));
        assert!(tm.api_version.is_none());
    }

    // ObjectMeta tests
    #[test]
    fn test_object_meta_default() {
        let om = ObjectMeta::default();
        assert!(om.name.is_none());
        assert!(om.namespace.is_none());
        assert!(om.labels.is_empty());
        assert!(om.annotations.is_empty());
    }

    #[test]
    fn test_object_meta_with_name() {
        let om = ObjectMeta {
            name: Some("my-pod".to_string()),
            ..Default::default()
        };
        assert_eq!(om.name, Some("my-pod".to_string()));
    }

    #[test]
    fn test_object_meta_with_labels() {
        let mut labels = HashMap::new();
        labels.insert("app".to_string(), "nginx".to_string());
        labels.insert("env".to_string(), "prod".to_string());

        let om = ObjectMeta {
            name: Some("my-pod".to_string()),
            labels,
            ..Default::default()
        };
        assert_eq!(om.labels.len(), 2);
        assert_eq!(om.labels.get("app"), Some(&"nginx".to_string()));
    }

    #[test]
    fn test_object_meta_serialize() {
        let mut labels = HashMap::new();
        labels.insert("app".to_string(), "nginx".to_string());

        let om = ObjectMeta {
            name: Some("my-pod".to_string()),
            namespace: Some("default".to_string()),
            labels,
            ..Default::default()
        };
        let json = serde_json::to_string(&om).unwrap();
        assert!(json.contains("\"name\":\"my-pod\""));
        assert!(json.contains("\"namespace\":\"default\""));
        assert!(json.contains("\"app\":\"nginx\""));
    }

    #[test]
    fn test_object_meta_deserialize() {
        let json = r#"{"name":"my-pod","namespace":"default","labels":{"app":"nginx"}}"#;
        let om: ObjectMeta = serde_json::from_str(json).unwrap();
        assert_eq!(om.name, Some("my-pod".to_string()));
        assert_eq!(om.namespace, Some("default".to_string()));
        assert_eq!(om.labels.get("app"), Some(&"nginx".to_string()));
    }

    #[test]
    fn test_object_meta_round_trip() {
        let mut labels = HashMap::new();
        labels.insert("app".to_string(), "nginx".to_string());

        let original = ObjectMeta {
            name: Some("my-pod".to_string()),
            namespace: Some("default".to_string()),
            labels,
            ..Default::default()
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: ObjectMeta = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_object_meta_empty_labels_omitted() {
        let om = ObjectMeta {
            name: Some("my-pod".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_string(&om).unwrap();
        // Empty labels should be omitted
        assert!(!json.contains("labels"));
    }

    #[test]
    fn test_managed_fields_entry() {
        let entry = ManagedFieldsEntry {
            manager: Some("kubectl".to_string()),
            operation: Some("Apply".to_string()),
            api_version: Some("v1".to_string()),
            fields_type: Some("FieldsV1".to_string()),
            fields_v1: Some(serde_json::json!({})),
        };
        assert_eq!(entry.manager, Some("kubectl".to_string()));
    }

    #[test]
    fn test_object_meta_with_uid() {
        let om = ObjectMeta {
            name: Some("my-pod".to_string()),
            uid: Some("abc-123-def".to_string()),
            resource_version: Some("12345".to_string()),
            ..Default::default()
        };
        assert_eq!(om.uid, Some("abc-123-def".to_string()));
        assert_eq!(om.resource_version, Some("12345".to_string()));
    }

    #[test]
    fn test_object_meta_with_generation() {
        let om = ObjectMeta {
            name: Some("my-deployment".to_string()),
            generation: Some(5),
            ..Default::default()
        };
        assert_eq!(om.generation, Some(5));
    }
}
