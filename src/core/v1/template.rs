//! Kubernetes Pod Template types
//!
//! This module contains pod template-related types from the Kubernetes core/v1 API.
//! Pod templates are used to define pod specifications that can be reused by controllers
//! such as Deployments, StatefulSets, DaemonSets, and Jobs.

use crate::common::meta::{ListMeta, ObjectMeta};
use crate::common::TypeMeta;
use serde::{Deserialize, Serialize};

/// PodTemplate describes a template for creating copies of a predefined pod.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodTemplate {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// Template defines the pods that will be created from this pod template.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<PodTemplateSpec>,
}

/// PodTemplateSpec describes the data a pod should have when created from a template.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodTemplateSpec {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// Specification of the desired behavior of the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<serde_json::Value>,
}

/// PodTemplateList is a list of PodTemplates.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodTemplateList {
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,
    /// List of pod templates.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<PodTemplate>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pod_template_default() {
        let template = PodTemplate::default();
        assert!(template.metadata.is_none());
        assert!(template.template.is_none());
    }

    #[test]
    fn test_pod_template_spec_default() {
        let spec = PodTemplateSpec::default();
        assert!(spec.metadata.is_none());
        assert!(spec.spec.is_none());
    }

    #[test]
    fn test_pod_template_list_default() {
        let list = PodTemplateList::default();
        assert!(list.metadata.is_none());
        assert!(list.items.is_empty());
    }

    #[test]
    fn test_pod_template_with_metadata() {
        let mut metadata = ObjectMeta::default();
        metadata.name = Some("my-template".to_string());

        let template = PodTemplate {
            metadata: Some(metadata),
            template: None,
        };

        assert!(template.metadata.is_some());
        assert_eq!(
            template.metadata.as_ref().and_then(|m| m.name.as_deref()),
            Some("my-template")
        );
    }

    #[test]
    fn test_pod_template_with_template_spec() {
        let template_spec = PodTemplateSpec::default();

        let template = PodTemplate {
            metadata: None,
            template: Some(template_spec),
        };

        assert!(template.template.is_some());
    }

    #[test]
    fn test_pod_template_list_with_items() {
        let template1 = PodTemplate {
            metadata: None,
            template: None,
        };

        let template2 = PodTemplate {
            metadata: None,
            template: None,
        };

        let list = PodTemplateList {
            metadata: None,
            items: vec![template1, template2],
        };

        assert_eq!(list.items.len(), 2);
    }

    #[test]
    fn test_pod_template_serialization() {
        let template = PodTemplate {
            metadata: Some(ObjectMeta {
                name: Some("test-template".to_string()),
                ..Default::default()
            }),
            template: Some(PodTemplateSpec::default()),
        };

        let json = serde_json::to_string(&template).unwrap();
        let deserialized: PodTemplate = serde_json::from_str(&json).unwrap();

        assert!(deserialized.metadata.is_some());
        assert!(deserialized.template.is_some());
    }

    #[test]
    fn test_pod_template_list_serialization() {
        let list = PodTemplateList {
            metadata: Some(ListMeta::default()),
            items: vec![PodTemplate::default()],
        };

        let json = serde_json::to_string(&list).unwrap();
        let deserialized: PodTemplateList = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.items.len(), 1);
        assert!(deserialized.metadata.is_some());
    }

    #[test]
    fn test_pod_template_spec_serialization() {
        let spec = PodTemplateSpec {
            metadata: Some(ObjectMeta::default()),
            spec: None,
        };

        let json = serde_json::to_string(&spec).unwrap();
        let deserialized: PodTemplateSpec = serde_json::from_str(&json).unwrap();

        assert!(deserialized.metadata.is_some());
    }

    #[test]
    fn test_pod_template_round_trip() {
        let template = PodTemplate {
            metadata: Some(ObjectMeta {
                name: Some("round-trip".to_string()),
                ..Default::default()
            }),
            template: Some(PodTemplateSpec::default()),
        };

        let json = serde_json::to_string(&template).unwrap();
        let deserialized: PodTemplate = serde_json::from_str(&json).unwrap();

        assert_eq!(template, deserialized);
    }

    #[test]
    fn test_pod_template_list_empty_items_serialization() {
        let list = PodTemplateList {
            metadata: None,
            items: vec![],
        };

        let json = serde_json::to_string(&list).unwrap();
        // Empty items should not be in JSON due to skip_serializing_if
        assert!(!json.contains("items"));

        let deserialized: PodTemplateList = serde_json::from_str(&json).unwrap();
        assert!(deserialized.items.is_empty());
    }

    #[test]
    fn test_pod_template_spec_with_placeholder_spec() {
        // Test that we can serialize a PodTemplateSpec with a placeholder spec value
        let spec_value = serde_json::json!({
            "containers": [{
                "name": "nginx",
                "image": "nginx:latest"
            }]
        });

        let spec = PodTemplateSpec {
            metadata: None,
            spec: Some(spec_value),
        };

        let json = serde_json::to_string(&spec).unwrap();
        let deserialized: PodTemplateSpec = serde_json::from_str(&json).unwrap();

        assert!(deserialized.spec.is_some());
    }

    #[test]
    fn test_pod_template_skip_empty_fields() {
        let template = PodTemplate {
            metadata: None,
            template: None,
        };

        let json = serde_json::to_string(&template).unwrap();
        // Empty fields should not be in JSON
        assert!(!json.contains("metadata"));
        assert!(!json.contains("template"));

        let deserialized: PodTemplate = serde_json::from_str(&json).unwrap();
        assert!(deserialized.metadata.is_none());
        assert!(deserialized.template.is_none());
    }
}
