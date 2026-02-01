//! Kubernetes Selector types
//!
//! This module contains selector-related types from the Kubernetes core/v1 API.
//! These types are used for selecting fields from pods, resources, ConfigMaps, and Secrets.

use crate::common::ApplyDefault;
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

impl ApplyDefault for ObjectFieldSelector {
    fn apply_default(&mut self) {
        if self.api_version.is_empty() {
            self.api_version = object_field_selector_api_version::V1.to_string();
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_field_selector_default_api_version() {
        let mut selector = ObjectFieldSelector {
            api_version: String::new(),
            field_path: "metadata.name".to_string(),
        };

        selector.apply_default();

        assert_eq!(selector.api_version, object_field_selector_api_version::V1);
    }
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
