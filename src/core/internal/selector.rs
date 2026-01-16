//! Label Selector types from the Kubernetes Core API
//!
//! This module contains types for selecting objects by labels and fields.
//! These are shared types used across different Kubernetes API versions.

use crate::common::util::Quantity;
use serde::{Deserialize, Serialize};

/// LabelSelector is a label query over a set of resources.
///
/// Corresponds to [Kubernetes LabelSelector](https://github.com/kubernetes/apimachinery/blob/master/pkg/apis/meta/v1/types.go#L1189)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct LabelSelector {
    /// matchLabels is a map of {key,value} pairs.
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub match_labels: std::collections::HashMap<String, String>,

    /// matchExpressions is a list of label selector requirements.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub match_expressions: Vec<LabelSelectorRequirement>,
}

/// LabelSelectorRequirement is a selector that contains values, a key, and an operator.
///
/// Corresponds to [Kubernetes LabelSelectorRequirement](https://github.com/kubernetes/apimachinery/blob/master/pkg/apis/meta/v1/types.go#L1202)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct LabelSelectorRequirement {
    /// key is the label key that the selector applies to.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub key: String,

    /// operator represents a key's relationship to a set of values.
    pub operator: String,

    /// values is an array of string values.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
}

/// LabelSelectorOperator constants
pub mod label_selector_operator {
    pub const IN: &str = "In";
    pub const NOT_IN: &str = "NotIn";
    pub const EXISTS: &str = "Exists";
    pub const DOES_NOT_EXIST: &str = "DoesNotExist";
}

/// NodeSelector represents a node selector.
///
/// Corresponds to [Kubernetes NodeSelector](https://github.com/kubernetes/apimachinery/blob/master/pkg/apis/core/types.go#L3310)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NodeSelector {
    /// nodeSelectorTerms is a list of node selector terms. The terms are ORed.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub node_selector_terms: Vec<NodeSelectorTerm>,
}

/// NodeSelectorTerm represents expressions and fields required to select nodes.
///
/// Corresponds to [Kubernetes NodeSelectorTerm](https://github.com/kubernetes/apimachinery/blob/master/pkg/apis/core/types.go#L3319)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NodeSelectorTerm {
    /// matchExpressions is a list of node selector requirements by node's labels.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub match_expressions: Vec<NodeSelectorRequirement>,
    /// matchFields is a list of node selector requirements by node's fields.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub match_fields: Vec<NodeSelectorRequirement>,
}

/// NodeSelectorRequirement is a selector that contains values, a key, and an operator.
///
/// Corresponds to [Kubernetes NodeSelectorRequirement](https://github.com/kubernetes/apimachinery/blob/master/pkg/apis/core/types.go#L3328)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NodeSelectorRequirement {
    /// key is the label key that the selector applies to.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub key: String,
    /// operator represents a key's relationship to a set of values.
    pub operator: String,
    /// values is an array of string values.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_label_selector_default() {
        let selector = LabelSelector::default();
        assert!(selector.match_labels.is_empty());
        assert!(selector.match_expressions.is_empty());
    }

    #[test]
    fn test_label_selector_with_match_labels() {
        let mut match_labels = std::collections::HashMap::new();
        match_labels.insert("app".to_string(), "nginx".to_string());
        match_labels.insert("env".to_string(), "prod".to_string());

        let selector = LabelSelector {
            match_labels: match_labels.clone(),
            match_expressions: Vec::new(),
        };

        assert_eq!(selector.match_labels.len(), 2);
        assert_eq!(selector.match_labels.get("app"), Some(&"nginx".to_string()));
        assert_eq!(selector.match_labels.get("env"), Some(&"prod".to_string()));
    }

    #[test]
    fn test_label_selector_with_match_expressions() {
        let expressions = vec![
            LabelSelectorRequirement {
                key: "tier".to_string(),
                operator: label_selector_operator::IN.to_string(),
                values: vec!["frontend".to_string(), "backend".to_string()],
            },
            LabelSelectorRequirement {
                key: "environment".to_string(),
                operator: label_selector_operator::NOT_IN.to_string(),
                values: vec!["test".to_string()],
            },
        ];

        let selector = LabelSelector {
            match_labels: std::collections::HashMap::new(),
            match_expressions: expressions.clone(),
        };

        assert_eq!(selector.match_expressions.len(), 2);
        assert_eq!(selector.match_expressions[0].key, "tier");
        assert_eq!(selector.match_expressions[1].key, "environment");
    }

    #[test]
    fn test_label_selector_serialize() {
        let mut match_labels = std::collections::HashMap::new();
        match_labels.insert("app".to_string(), "myapp".to_string());

        let selector = LabelSelector {
            match_labels,
            match_expressions: vec![LabelSelectorRequirement {
                key: "version".to_string(),
                operator: label_selector_operator::EXISTS.to_string(),
                values: vec![],
            }],
        };

        let json = serde_json::to_string(&selector).unwrap();
        assert!(json.contains("\"matchLabels\""));
        assert!(json.contains("\"app\":\"myapp\""));
        assert!(json.contains("\"matchExpressions\""));
        assert!(json.contains("\"version\""));
        assert!(json.contains("\"Exists\""));
    }

    #[test]
    fn test_label_selector_deserialize() {
        let json = r#"{"matchLabels":{"app":"nginx","env":"prod"}}"#;
        let selector: LabelSelector = serde_json::from_str(json).unwrap();

        assert_eq!(selector.match_labels.len(), 2);
        assert_eq!(selector.match_labels.get("app"), Some(&"nginx".to_string()));
        assert_eq!(selector.match_labels.get("env"), Some(&"prod".to_string()));
    }

    #[test]
    fn test_label_selector_round_trip() {
        let mut match_labels = std::collections::HashMap::new();
        match_labels.insert("component".to_string(), "database".to_string());

        let original = LabelSelector {
            match_labels,
            match_expressions: vec![LabelSelectorRequirement {
                key: "storage".to_string(),
                operator: label_selector_operator::DOES_NOT_EXIST.to_string(),
                values: vec![],
            }],
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: LabelSelector = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_label_selector_requirement_default() {
        let requirement = LabelSelectorRequirement::default();
        assert!(requirement.key.is_empty());
        assert!(requirement.operator.is_empty());
        assert!(requirement.values.is_empty());
    }

    #[test]
    fn test_label_selector_requirement_with_fields() {
        let requirement = LabelSelectorRequirement {
            key: "environment".to_string(),
            operator: label_selector_operator::IN.to_string(),
            values: vec!["dev".to_string(), "staging".to_string(), "prod".to_string()],
        };

        assert_eq!(requirement.key, "environment");
        assert_eq!(requirement.operator, "In");
        assert_eq!(requirement.values.len(), 3);
        assert_eq!(requirement.values[0], "dev");
        assert_eq!(requirement.values[2], "prod");
    }

    #[test]
    fn test_label_selector_requirement_serialize() {
        let requirement = LabelSelectorRequirement {
            key: "tier".to_string(),
            operator: label_selector_operator::NOT_IN.to_string(),
            values: vec!["cache".to_string()],
        };

        let json = serde_json::to_string(&requirement).unwrap();
        assert!(json.contains("\"key\":\"tier\""));
        assert!(json.contains("\"operator\":\"NotIn\""));
        assert!(json.contains("\"values\":[\"cache\"]"));
    }

    #[test]
    fn test_label_selector_requirement_empty_values_serialization() {
        let requirement = LabelSelectorRequirement {
            key: "app".to_string(),
            operator: label_selector_operator::EXISTS.to_string(),
            values: vec![],
        };

        let json = serde_json::to_string(&requirement).unwrap();
        assert!(json.contains("\"key\":\"app\""));
        assert!(json.contains("\"operator\":\"Exists\""));
        // empty values should be omitted
        assert!(!json.contains("\"values\""));
    }

    #[test]
    fn test_label_selector_requirement_deserialize() {
        let json = r#"{"key":"version","operator":"In","values":["v1","v2"]}"#;
        let requirement: LabelSelectorRequirement = serde_json::from_str(json).unwrap();

        assert_eq!(requirement.key, "version");
        assert_eq!(requirement.operator, "In");
        assert_eq!(requirement.values.len(), 2);
        assert_eq!(requirement.values[0], "v1");
        assert_eq!(requirement.values[1], "v2");
    }

    #[test]
    fn test_label_selector_requirement_round_trip() {
        let original = LabelSelectorRequirement {
            key: "region".to_string(),
            operator: label_selector_operator::NOT_IN.to_string(),
            values: vec!["us-east-1".to_string(), "us-west-2".to_string()],
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: LabelSelectorRequirement = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_label_selector_operator_constants() {
        assert_eq!(label_selector_operator::IN, "In");
        assert_eq!(label_selector_operator::NOT_IN, "NotIn");
        assert_eq!(label_selector_operator::EXISTS, "Exists");
        assert_eq!(label_selector_operator::DOES_NOT_EXIST, "DoesNotExist");
    }

    #[test]
    fn test_label_selector_empty_serialization() {
        let selector = LabelSelector::default();
        let json = serde_json::to_string(&selector).unwrap();

        // Both fields empty should result in empty JSON object
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_label_selector_complex() {
        let mut match_labels = std::collections::HashMap::new();
        match_labels.insert("app".to_string(), "web".to_string());

        let original = LabelSelector {
            match_labels,
            match_expressions: vec![
                LabelSelectorRequirement {
                    key: "env".to_string(),
                    operator: label_selector_operator::IN.to_string(),
                    values: vec!["prod".to_string(), "staging".to_string()],
                },
                LabelSelectorRequirement {
                    key: "debug".to_string(),
                    operator: label_selector_operator::DOES_NOT_EXIST.to_string(),
                    values: vec![],
                },
            ],
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: LabelSelector = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
        assert!(json.contains("\"matchLabels\""));
        assert!(json.contains("\"app\":\"web\""));
        assert!(json.contains("\"matchExpressions\""));
    }
}

// ============================================================================
// Field Selector Types
// ============================================================================

/// ObjectFieldSelector selects an APIVersioned field of an object.
///
/// Corresponds to [Kubernetes ObjectFieldSelector](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2301)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ObjectFieldSelector {
    /// Version of the schema the FieldPath is written in terms of.
    /// If no value is specified, it will be defaulted to the APIVersion of the enclosing object.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub api_version: String,
    /// Path of the field to select in the specified API version.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub field_path: String,
}

/// ObjectFieldSelector API version constants
pub mod object_field_selector_api_version {
    pub const V1: &str = "v1";
}

/// ObjectFieldSelector field path constants
pub mod object_field_path {
    pub const METADATA_NAME: &str = "metadata.name";
    pub const METADATA_NAMESPACE: &str = "metadata.namespace";
    pub const METADATA_UID: &str = "metadata.uid";
    pub const SPEC_NODE_NAME: &str = "spec.nodeName";
    pub const SPEC_SERVICE_ACCOUNT_NAME: &str = "spec.serviceAccountName";
    pub const STATUS_HOST_IP: &str = "status.hostIP";
    pub const STATUS_POD_IP: &str = "status.podIP";
    pub const STATUS_POD_IPS: &str = "status.podIPs";
}

/// ResourceFieldSelector represents container resources (cpu, memory) and their output format.
///
/// Corresponds to [Kubernetes ResourceFieldSelector](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2311)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceFieldSelector {
    /// Container name: required for volumes, optional for env vars.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub container_name: String,
    /// Resource to select.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub resource: String,
    /// Specifies the output format of the exposed resources, defaults to "1".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub divisor: Option<Quantity>,
}

/// ResourceFieldSelector resource constants
pub mod resource_field_selector_resource {
    pub const LIMITS_CPU: &str = "limits.cpu";
    pub const LIMITS_MEMORY: &str = "limits.memory";
    pub const LIMITS_EPHEMERAL_STORAGE: &str = "limits.ephemeral-storage";
    pub const REQUESTS_CPU: &str = "requests.cpu";
    pub const REQUESTS_MEMORY: &str = "requests.memory";
    pub const REQUESTS_EPHEMERAL_STORAGE: &str = "requests.ephemeral-storage";
}

// ============================================================================
// ConfigMap and Secret Key Selectors
// ============================================================================

/// LocalObjectReference contains enough information to let you locate the
/// referenced object inside the same namespace.
///
/// Corresponds to [Kubernetes LocalObjectReference](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4986)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct LocalObjectReference {
    /// Name of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// ConfigMapKeySelector selects a key from a ConfigMap.
///
/// Corresponds to [Kubernetes ConfigMapKeySelector](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2323)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ConfigMapKeySelector {
    /// The ConfigMap to select from.
    #[serde(default, skip_serializing_if = "should_skip_name")]
    pub name: Option<String>,
    /// The key to select.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub key: String,
    /// Specify whether the ConfigMap or its key must be defined.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// SecretKeySelector selects a key of a Secret.
///
/// Corresponds to [Kubernetes SecretKeySelector](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2334)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SecretKeySelector {
    /// The name of the secret in the pod's namespace to select from.
    #[serde(default, skip_serializing_if = "should_skip_name")]
    pub name: Option<String>,
    /// The key of the secret to select from. Must be a valid secret key.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub key: String,
    /// Specify whether the Secret or its key must be defined.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// FileKeySelector selects a key of the env file.
///
/// Corresponds to [Kubernetes FileKeySelector](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2275)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct FileKeySelector {
    /// The name of the volume mount containing the env file.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub volume_name: String,
    /// The path within the volume from which to select the file.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub path: String,
    /// The key within the env file.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub key: String,
    /// Specify whether the file or its key must be defined.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// Helper function for checking if name should be skipped.
fn should_skip_name(name: &Option<String>) -> bool {
    name.as_ref().map_or(true, |s| s.is_empty())
}
