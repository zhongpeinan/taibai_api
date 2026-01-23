//! Environment Variable types from the Kubernetes Core API
//!
//! This module contains environment variable related types from the Kubernetes core/internal API.
//! These types are used for configuring environment variables in Pod containers.
//!
//! Source: k8s-pkg/apis/core/types.go

use crate::core::internal::selector::{
    ConfigMapKeySelector, FileKeySelector, LocalObjectReference, ObjectFieldSelector,
    ResourceFieldSelector, SecretKeySelector,
};
use serde::{Deserialize, Serialize};

/// EnvVar represents an environment variable present in a Container.
///
/// Corresponds to [Kubernetes EnvVar](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2229)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct EnvVar {
    /// Required: Name of the environment variable.
    /// May consist of any printable ASCII characters except '='.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// Optional: Defaults to ""; variable references $(VAR_NAME) are expanded
    /// using the previously defined environment variables in the container and
    /// any service environment variables.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub value: String,
    /// Optional: Specifies a source the value of this var should come from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_from: Option<EnvVarSource>,
}

/// EnvVarSource represents a source for the value of an EnvVar.
///
/// Only one of its fields may be set.
///
/// Corresponds to [Kubernetes EnvVarSource](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2251)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct EnvVarSource {
    /// Selects a field of the pod: supports metadata.name, metadata.namespace,
    /// metadata.labels['<KEY>'], metadata.annotations['<KEY>'], spec.nodeName,
    /// spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_ref: Option<ObjectFieldSelector>,
    /// Selects a resource of the container: only resources limits and requests
    /// (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu,
    /// requests.memory and requests.ephemeral-storage) are currently supported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_field_ref: Option<ResourceFieldSelector>,
    /// Selects a key of a ConfigMap.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config_map_key_ref: Option<ConfigMapKeySelector>,
    /// Selects a key of a secret in the pod's namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_key_ref: Option<SecretKeySelector>,
    /// FileKeyRef selects a key of the env file.
    /// Requires the EnvFiles feature gate to be enabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_key_ref: Option<FileKeySelector>,
}

/// EnvFromSource represents the source of a set of ConfigMaps or Secrets.
///
/// Corresponds to [Kubernetes EnvFromSource](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2345)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct EnvFromSource {
    /// Optional text to prepend to the name of each environment variable.
    /// Must be a C_IDENTIFIER.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub prefix: String,
    /// The ConfigMap to select from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config_map_ref: Option<ConfigMapEnvSource>,
    /// The Secret to select from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<SecretEnvSource>,
}

/// ConfigMapEnvSource selects a ConfigMap to populate environment variables with.
///
/// The contents of the target ConfigMap's Data field will represent the
/// key-value pairs as environment variables.
///
/// Corresponds to [Kubernetes ConfigMapEnvSource](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2358)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct ConfigMapEnvSource {
    /// The ConfigMap to select from.
    #[serde(default, skip_serializing_if = "should_skip_reference")]
    pub local_object_reference: LocalObjectReference,
    /// Specify whether the ConfigMap must be defined.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// SecretEnvSource selects a Secret to populate environment variables with.
///
/// The contents of the target Secret's Data field will represent the
/// key-value pairs as environment variables.
///
/// Corresponds to [Kubernetes SecretEnvSource](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2371)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct SecretEnvSource {
    /// The Secret to select from.
    #[serde(default, skip_serializing_if = "should_skip_reference")]
    pub local_object_reference: LocalObjectReference,
    /// Specify whether the Secret must be defined.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// Helper function for checking if LocalObjectReference should be skipped.
fn should_skip_reference(ref_: &LocalObjectReference) -> bool {
    ref_.name.as_ref().is_none_or(|s| s.is_empty())
}

#[cfg(test)]
mod tests {}
