//! Kubernetes Selector types
//!
//! This module contains selector-related types from the Kubernetes core/v1 API.
//! These types are used for selecting fields from pods, resources, ConfigMaps, and Secrets.

use crate::common::util::Quantity;
use serde::{Deserialize, Serialize};

/// ObjectFieldSelector selects a field from a pod object.
///
/// Supported fields: metadata.name, metadata.namespace, metadata.labels['<KEY>'],
/// metadata.annotations['<KEY>'], spec.nodeName, spec.serviceAccountName,
/// status.hostIP, status.podIP, status.podIPs.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ObjectFieldSelector {
    /// Version of the schema the FieldPath is written in terms of, defaults to "v1".
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub api_version: String,
    /// Path of the field to select in the specified API version.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub field_path: String,
}

/// Constants for ObjectFieldSelector API versions
pub mod object_field_selector_api_version {
    pub const V1: &str = "v1";
}

/// Constants for common ObjectFieldSelector field paths
pub mod object_field_path {
    pub const METADATA_NAME: &str = "metadata.name";
    pub const METADATA_NAMESPACE: &str = "metadata.namespace";
    pub const METADATA_UID: &str = "metadata.uid";
    pub const METADATA_LABELS: &str = "metadata.labels";
    pub const METADATA_ANNOTATIONS: &str = "metadata.annotations";
    pub const SPEC_NODE_NAME: &str = "spec.nodeName";
    pub const SPEC_SERVICE_ACCOUNT_NAME: &str = "spec.serviceAccountName";
    pub const STATUS_HOST_IP: &str = "status.hostIP";
    pub const STATUS_POD_IP: &str = "status.podIP";
    pub const STATUS_POD_IPS: &str = "status.podIPs";
}

/// ResourceFieldSelector represents container resources (cpu, memory) and their output format.
///
/// Supported resources: limits.cpu, limits.memory, limits.ephemeral-storage,
/// requests.cpu, requests.memory, requests.ephemeral-storage.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceFieldSelector {
    /// Container name: required for volumes, optional for env vars.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub container_name: String,
    /// Required: resource to select.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub resource: String,
    /// Specifies the output format of the exposed resources, defaults to "1".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub divisor: Option<Quantity>,
}

/// Constants for ResourceFieldSelector resources
pub mod resource_field_selector_resource {
    pub const LIMITS_CPU: &str = "limits.cpu";
    pub const LIMITS_MEMORY: &str = "limits.memory";
    pub const LIMITS_EPHEMERAL_STORAGE: &str = "limits.ephemeral-storage";
    pub const REQUESTS_CPU: &str = "requests.cpu";
    pub const REQUESTS_MEMORY: &str = "requests.memory";
    pub const REQUESTS_EPHEMERAL_STORAGE: &str = "requests.ephemeral-storage";
}

/// ConfigMapKeySelector selects a key from a ConfigMap.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct ConfigMapKeySelector {
    /// The ConfigMap to select from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The key to select.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub key: String,
    /// Specify whether the ConfigMap or its key must be defined.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}


/// SecretKeySelector selects a key of a Secret.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct SecretKeySelector {
    /// The name of the secret in the pod's namespace to select from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The key of the secret to select from.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub key: String,
    /// Specify whether the Secret or its key must be defined.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}


/// FileKeySelector selects a key from an env file.
///
/// Requires the EnvFiles feature gate to be enabled.
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_field_selector() {
        let selector = ObjectFieldSelector {
            api_version: object_field_selector_api_version::V1.to_string(),
            field_path: object_field_path::METADATA_NAME.to_string(),
        };

        let json = serde_json::to_string(&selector).unwrap();
        let deserialized: ObjectFieldSelector = serde_json::from_str(&json).unwrap();

        assert_eq!(selector.api_version, deserialized.api_version);
        assert_eq!(selector.field_path, deserialized.field_path);
    }

    #[test]
    fn test_object_field_selector_default() {
        let selector = ObjectFieldSelector::default();

        assert!(selector.api_version.is_empty());
        assert!(selector.field_path.is_empty());
    }

    #[test]
    fn test_resource_field_selector() {
        let selector = ResourceFieldSelector {
            container_name: "my-container".to_string(),
            resource: resource_field_selector_resource::LIMITS_CPU.to_string(),
            divisor: Some(Quantity::from("1")),
        };

        let json = serde_json::to_string(&selector).unwrap();
        let deserialized: ResourceFieldSelector = serde_json::from_str(&json).unwrap();

        assert_eq!(selector.container_name, deserialized.container_name);
        assert_eq!(selector.resource, deserialized.resource);
        assert!(selector.divisor.is_some());
    }

    #[test]
    fn test_resource_field_selector_without_divisor() {
        let selector = ResourceFieldSelector {
            container_name: String::new(),
            resource: resource_field_selector_resource::REQUESTS_MEMORY.to_string(),
            divisor: None,
        };

        let json = serde_json::to_string(&selector).unwrap();
        let deserialized: ResourceFieldSelector = serde_json::from_str(&json).unwrap();

        assert_eq!(selector.resource, deserialized.resource);
        assert!(selector.divisor.is_none());
    }

    #[test]
    fn test_config_map_key_selector() {
        let selector = ConfigMapKeySelector {
            name: Some("my-config".to_string()),
            key: "config-key".to_string(),
            optional: Some(true),
        };

        let json = serde_json::to_string(&selector).unwrap();
        let deserialized: ConfigMapKeySelector = serde_json::from_str(&json).unwrap();

        assert_eq!(selector.name, deserialized.name);
        assert_eq!(selector.key, deserialized.key);
        assert_eq!(selector.optional, deserialized.optional);
    }

    #[test]
    fn test_config_map_key_selector_default() {
        let selector = ConfigMapKeySelector::default();

        assert!(selector.name.is_none());
        assert!(selector.key.is_empty());
        assert!(selector.optional.is_none());
    }

    #[test]
    fn test_secret_key_selector() {
        let selector = SecretKeySelector {
            name: Some("my-secret".to_string()),
            key: "secret-key".to_string(),
            optional: Some(false),
        };

        let json = serde_json::to_string(&selector).unwrap();
        let deserialized: SecretKeySelector = serde_json::from_str(&json).unwrap();

        assert_eq!(selector.name, deserialized.name);
        assert_eq!(selector.key, deserialized.key);
        assert_eq!(selector.optional, deserialized.optional);
    }

    #[test]
    fn test_secret_key_selector_default() {
        let selector = SecretKeySelector::default();

        assert!(selector.name.is_none());
        assert!(selector.key.is_empty());
        assert!(selector.optional.is_none());
    }

    #[test]
    fn test_file_key_selector() {
        let selector = FileKeySelector {
            volume_name: "env-vol".to_string(),
            path: "/etc/config/env.json".to_string(),
            key: "DATABASE_URL".to_string(),
        };

        let json = serde_json::to_string(&selector).unwrap();
        let deserialized: FileKeySelector = serde_json::from_str(&json).unwrap();

        assert_eq!(selector.volume_name, deserialized.volume_name);
        assert_eq!(selector.path, deserialized.path);
        assert_eq!(selector.key, deserialized.key);
    }

    #[test]
    fn test_file_key_selector_default() {
        let selector = FileKeySelector::default();

        assert!(selector.volume_name.is_empty());
        assert!(selector.path.is_empty());
        assert!(selector.key.is_empty());
    }

    #[test]
    fn test_object_field_selector_constants() {
        assert_eq!(object_field_selector_api_version::V1, "v1");
        assert_eq!(object_field_path::METADATA_NAME, "metadata.name");
        assert_eq!(object_field_path::METADATA_NAMESPACE, "metadata.namespace");
        assert_eq!(object_field_path::SPEC_NODE_NAME, "spec.nodeName");
        assert_eq!(object_field_path::STATUS_POD_IP, "status.podIP");
    }

    #[test]
    fn test_resource_field_selector_constants() {
        assert_eq!(resource_field_selector_resource::LIMITS_CPU, "limits.cpu");
        assert_eq!(
            resource_field_selector_resource::LIMITS_MEMORY,
            "limits.memory"
        );
        assert_eq!(
            resource_field_selector_resource::REQUESTS_CPU,
            "requests.cpu"
        );
    }

    #[test]
    fn test_object_field_selector_round_trip() {
        let selector = ObjectFieldSelector {
            api_version: object_field_selector_api_version::V1.to_string(),
            field_path: object_field_path::STATUS_HOST_IP.to_string(),
        };

        let json = serde_json::to_string(&selector).unwrap();
        let deserialized: ObjectFieldSelector = serde_json::from_str(&json).unwrap();

        assert_eq!(selector, deserialized);
    }

    #[test]
    fn test_resource_field_selector_round_trip() {
        let selector = ResourceFieldSelector {
            container_name: "app".to_string(),
            resource: resource_field_selector_resource::LIMITS_EPHEMERAL_STORAGE.to_string(),
            divisor: Some("10Mi".into()),
        };

        let json = serde_json::to_string(&selector).unwrap();
        let deserialized: ResourceFieldSelector = serde_json::from_str(&json).unwrap();

        assert_eq!(selector, deserialized);
    }

    #[test]
    fn test_config_map_key_selector_round_trip() {
        let selector = ConfigMapKeySelector {
            name: Some("app-config".to_string()),
            key: "api-key".to_string(),
            optional: None,
        };

        let json = serde_json::to_string(&selector).unwrap();
        let deserialized: ConfigMapKeySelector = serde_json::from_str(&json).unwrap();

        assert_eq!(selector, deserialized);
    }

    #[test]
    fn test_secret_key_selector_round_trip() {
        let selector = SecretKeySelector {
            name: Some("db-credentials".to_string()),
            key: "password".to_string(),
            optional: Some(true),
        };

        let json = serde_json::to_string(&selector).unwrap();
        let deserialized: SecretKeySelector = serde_json::from_str(&json).unwrap();

        assert_eq!(selector, deserialized);
    }

    #[test]
    fn test_config_map_key_selector_empty_name_serialization() {
        let selector = ConfigMapKeySelector {
            name: None,
            key: "key".to_string(),
            optional: None,
        };

        let json = serde_json::to_string(&selector).unwrap();
        assert!(!json.contains("\"name\""));
    }

    #[test]
    fn test_secret_key_selector_empty_name_serialization() {
        let selector = SecretKeySelector {
            name: None,
            key: "key".to_string(),
            optional: None,
        };

        let json = serde_json::to_string(&selector).unwrap();
        assert!(!json.contains("\"name\""));
    }
}
