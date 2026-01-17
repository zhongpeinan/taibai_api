//! Kubernetes common metadata types
//!
//! This module contains the fundamental metadata types used across Kubernetes API objects.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::time::Timestamp;

/// TypeMeta describes an individual object in an API response or request
/// with Kind and Version fields.
///
/// Corresponds to [Kubernetes TypeMeta](https://github.com/kubernetes/apimachinery/blob/master/pkg/apis/meta/v1/types.go#L42)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
}

/// ListMeta describes metadata that synthetic resources must have, including lists and status objects.
///
/// Corresponds to [Kubernetes ListMeta](https://github.com/kubernetes/apimachinery/blob/master/pkg/apis/meta/v1/types.go#L2375)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListMeta {
    /// continue may be set if the user set a limit on the number of items returned, and indicates
    /// that the server has more data available.
    #[serde(rename = "continue", default, skip_serializing_if = "Option::is_none")]
    pub continue_: Option<String>,

    /// remainingItemCount is the number of subsequent items in the list which are not included
    /// in this list response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remaining_item_count: Option<i64>,

    /// resourceVersion sets a resource version constraint on what kind of objects are included in the response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_version: Option<String>,

    /// SelfLink is a URL representing this list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
}

/// ObjectMeta is metadata that all persisted resources must have, which includes all objects
/// users must create.
///
/// Corresponds to [Kubernetes ObjectMeta](https://github.com/kubernetes/apimachinery/blob/master/pkg/apis/meta/v1/types.go#L110)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub managed_fields: Vec<ManagedFieldsEntry>,

    /// CreationTimestamp is a timestamp representing the server time when this object was created.
    /// It is represented in RFC3339 form and is UTC. For example: "2024-01-15T10:00:00Z"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<Timestamp>,

    /// DeletionTimestamp is RFC3339 string representing the time when this object will be deleted.
    /// This field is set by the server when a graceful deletion is initiated. For example: "2024-01-15T10:00:00Z"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deletion_timestamp: Option<Timestamp>,
}

/// ManagedFieldsEntry is a workflow-id, a FieldSet and the group version of the resource
/// that the fieldset applies to.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ManagedFieldsEntry {
    /// Manager is an identifier of the workflow managing these fields.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manager: Option<String>,

    /// Operation is the type of operation which lead to this ManagedFieldsEntry being merged.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,

    /// APIVersion defines the version of this resource that this field set applies to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,

    /// Time is the timestamp of when the ManagedFieldsEntry was added.
    /// It is represented in RFC3339 form and is UTC. For example: "2024-01-15T10:00:00Z"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Timestamp>,

    /// FieldsType is the discriminator for the different fields format and version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields_type: Option<String>,

    /// FieldsV1 holds the first JSON version of the fields.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields_v1: Option<serde_json::Value>,
}

/// Condition defines an observation of a resource's state.
///
/// Corresponds to [Kubernetes Condition](https://github.com/kubernetes/apimachinery/blob/master/pkg/apis/meta/v1/types.go#L1339)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Condition {
    /// Type of condition in CamelCase or in foo.example.com/CamelCase.
    #[serde(rename = "type")]
    pub type_: String,

    /// Status of the condition, one of True, False, Unknown.
    pub status: String,

    /// ObservedGeneration represents the .metadata.generation that the condition was set based upon.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,

    /// LastTransitionTime is the last time the condition transitioned from one status to another.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<Timestamp>,

    /// Reason contains a programmatic identifier indicating the reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    /// Message is a human readable message indicating details about the transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// LabelSelector is a label query over a set of resources.
///
/// Corresponds to [Kubernetes LabelSelector](https://github.com/kubernetes/apimachinery/blob/master/pkg/apis/meta/v1/types.go#L1210)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct LabelSelector {
    /// matchLabels is a map of {key,value} pairs.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub match_labels: HashMap<String, String>,

    /// matchExpressions is a list of label selector requirements.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub match_expressions: Vec<LabelSelectorRequirement>,
}

/// LabelSelectorRequirement is a selector that contains values, a key, and an operator.
///
/// Corresponds to [Kubernetes LabelSelectorRequirement](https://github.com/kubernetes/apimachinery/blob/master/pkg/apis/meta/v1/types.go#L1246)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LabelSelectorRequirement {
    /// key is the label key that the selector applies to.
    pub key: String,

    /// operator represents a key's relationship to a set of values.
    pub operator: String,

    /// values is an array of string values.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
}

/// Label selector operator constants
pub mod label_selector_operator {
    /// In means the label must match one of the values
    pub const IN: &str = "In";
    /// NotIn means the label must not match any of the values
    pub const NOT_IN: &str = "NotIn";
    /// Exists means the label must exist (values must be empty)
    pub const EXISTS: &str = "Exists";
    /// DoesNotExist means the label must not exist
    pub const DOES_NOT_EXIST: &str = "DoesNotExist";
}

/// FieldSelectorRequirement is a selector that contains values, a key, and an operator.
///
/// Corresponds to [Kubernetes FieldSelectorRequirement](https://github.com/kubernetes/apimachinery/blob/master/pkg/apis/meta/v1/types.go#L1283)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FieldSelectorRequirement {
    /// key is the field key that the selector applies to.
    pub key: String,

    /// operator represents a key's relationship to a set of values.
    pub operator: String,

    /// values is an array of string values.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
}

/// Field selector operator constants
pub mod field_selector_operator {
    /// In means the field must match one of the values
    pub const IN: &str = "In";
    /// NotIn means the field must not match any of the values
    pub const NOT_IN: &str = "NotIn";
    /// Exists means the field must exist (values must be empty)
    pub const EXISTS: &str = "Exists";
    /// DoesNotExist means the field must not exist
    pub const DOES_NOT_EXIST: &str = "DoesNotExist";
}

/// GroupVersionKind unambiguously identifies a kind.
///
/// Corresponds to [Kubernetes GroupVersionKind](https://github.com/kubernetes/apimachinery/blob/master/pkg/apis/meta/v1/types.go#L76)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GroupVersionKind {
    /// Group is the API group.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub group: String,
    /// Version is the API version.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub version: String,
    /// Kind is the resource kind.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub kind: String,
}

/// GroupVersionResource unambiguously identifies a resource.
///
/// Corresponds to [Kubernetes GroupVersionResource](https://github.com/kubernetes/apimachinery/blob/master/pkg/apis/meta/v1/types.go#L86)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GroupVersionResource {
    /// Group is the API group.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub group: String,
    /// Version is the API version.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub version: String,
    /// Resource is the resource name.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub resource: String,
}

/// GroupResource identifies a resource by group and resource name.
///
/// Corresponds to [Kubernetes GroupResource](https://github.com/kubernetes/apimachinery/blob/master/pkg/apis/meta/v1/types.go#L1198)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GroupResource {
    /// Group is the API group.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub group: String,
    /// Resource is the resource name.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub resource: String,
}

/// StatusCause is a brief explanation of the reason for a condition.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StatusCause {
    /// A machine-readable description of the cause of the error.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    /// A human-readable description of the cause of the error.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
    /// The field of the resource that has caused this error.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub field: String,
}

/// StatusDetails is a set of additional properties that MAY be set by the
/// server to provide additional information about a response.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StatusDetails {
    /// The name attribute of the resource associated with the status StatusReason.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// The group attribute of the resource associated with the status StatusReason.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub group: String,
    /// The kind attribute of the resource associated with the status StatusReason.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub kind: String,
    /// The UID attribute of the resource associated with the status StatusReason.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub uid: String,
    /// The Causes array includes more details associated with the StatusReason failure.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub causes: Vec<StatusCause>,
    /// If specified, the time in seconds before the operation should be retried.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retry_after_seconds: Option<i32>,
}

/// Status is a return value for calls that don't return other objects.
///
/// Corresponds to [Kubernetes Status](https://github.com/kubernetes/apimachinery/blob/master/pkg/apis/meta/v1/types.go#L2356)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,
    /// Status of the operation (one of "Success" or "Failure").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// A human-readable description of the status of this operation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// A machine-readable description of why this operation is in the "Failure" status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Extended data associated with the reason.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<StatusDetails>,
    /// Suggested HTTP return code for this status, 0 if not set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
}

/// Status constants
pub mod status {
    /// StatusSuccess indicates that the operation succeeded
    pub const SUCCESS: &str = "Success";
    /// StatusFailure indicates that the operation failed
    pub const FAILURE: &str = "Failure";
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
            time: Some(Timestamp::from_str("2024-01-15T10:00:00Z")),
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

    #[test]
    fn test_condition() {
        let condition = Condition {
            type_: "Ready".to_string(),
            status: "True".to_string(),
            observed_generation: Some(1),
            last_transition_time: Some(Timestamp::from_str("2024-01-15T10:00:00Z")),
            reason: Some("MinimumReplicasAvailable".to_string()),
            message: Some("Deployment has minimum availability.".to_string()),
        };
        assert_eq!(condition.type_, "Ready");
        assert_eq!(condition.status, "True");
        assert_eq!(
            condition.reason,
            Some("MinimumReplicasAvailable".to_string())
        );
    }

    #[test]
    fn test_condition_serialize() {
        let condition = Condition {
            type_: "Ready".to_string(),
            status: "True".to_string(),
            observed_generation: None,
            last_transition_time: None,
            reason: None,
            message: None,
        };
        let json = serde_json::to_string(&condition).unwrap();
        assert!(json.contains("\"type\":\"Ready\""));
        assert!(json.contains("\"status\":\"True\""));
    }

    #[test]
    fn test_condition_deserialize() {
        let json = r#"{"type":"Ready","status":"True"}"#;
        let condition: Condition = serde_json::from_str(json).unwrap();
        assert_eq!(condition.type_, "Ready");
        assert_eq!(condition.status, "True");
    }

    #[test]
    fn test_condition_round_trip() {
        let original = Condition {
            type_: "Ready".to_string(),
            status: "True".to_string(),
            observed_generation: Some(5),
            last_transition_time: Some(Timestamp::from_str("2024-01-15T10:00:00Z")),
            reason: Some("MinimumReplicasAvailable".to_string()),
            message: Some("Deployment ready.".to_string()),
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Condition = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_label_selector_default() {
        let ls = LabelSelector::default();
        assert!(ls.match_labels.is_empty());
        assert!(ls.match_expressions.is_empty());
    }

    #[test]
    fn test_label_selector_with_match_labels() {
        let mut match_labels = HashMap::new();
        match_labels.insert("app".to_string(), "nginx".to_string());
        match_labels.insert("env".to_string(), "prod".to_string());

        let ls = LabelSelector {
            match_labels,
            match_expressions: vec![],
        };
        assert_eq!(ls.match_labels.len(), 2);
        assert_eq!(ls.match_labels.get("app"), Some(&"nginx".to_string()));
    }

    #[test]
    fn test_label_selector_with_match_expressions() {
        let ls = LabelSelector {
            match_labels: HashMap::new(),
            match_expressions: vec![LabelSelectorRequirement {
                key: "environment".to_string(),
                operator: label_selector_operator::IN.to_string(),
                values: vec!["prod".to_string(), "staging".to_string()],
            }],
        };
        assert_eq!(ls.match_expressions.len(), 1);
        assert_eq!(ls.match_expressions[0].key, "environment");
    }

    #[test]
    fn test_label_selector_serialize() {
        let mut match_labels = HashMap::new();
        match_labels.insert("app".to_string(), "nginx".to_string());

        let ls = LabelSelector {
            match_labels,
            match_expressions: vec![],
        };
        let json = serde_json::to_string(&ls).unwrap();
        assert!(json.contains("\"matchLabels\""));
        assert!(json.contains("\"app\":\"nginx\""));
    }

    #[test]
    fn test_label_selector_deserialize() {
        let json = r#"{"matchLabels":{"app":"nginx"}}"#;
        let ls: LabelSelector = serde_json::from_str(json).unwrap();
        assert_eq!(ls.match_labels.get("app"), Some(&"nginx".to_string()));
    }

    #[test]
    fn test_label_selector_round_trip() {
        let mut match_labels = HashMap::new();
        match_labels.insert("app".to_string(), "nginx".to_string());

        let original = LabelSelector {
            match_labels,
            match_expressions: vec![LabelSelectorRequirement {
                key: "env".to_string(),
                operator: label_selector_operator::EXISTS.to_string(),
                values: vec![],
            }],
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: LabelSelector = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_label_selector_requirement() {
        let req = LabelSelectorRequirement {
            key: "tier".to_string(),
            operator: label_selector_operator::NOT_IN.to_string(),
            values: vec!["frontend".to_string(), "backend".to_string()],
        };
        assert_eq!(req.key, "tier");
        assert_eq!(req.operator, label_selector_operator::NOT_IN);
        assert_eq!(req.values.len(), 2);
    }

    #[test]
    fn test_label_selector_operator_constants() {
        assert_eq!(label_selector_operator::IN, "In");
        assert_eq!(label_selector_operator::NOT_IN, "NotIn");
        assert_eq!(label_selector_operator::EXISTS, "Exists");
        assert_eq!(label_selector_operator::DOES_NOT_EXIST, "DoesNotExist");
    }
}
