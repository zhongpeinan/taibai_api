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

impl Default for EnvVar {
    fn default() -> Self {
        Self {
            name: String::new(),
            value: String::new(),
            value_from: None,
        }
    }
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
pub struct ConfigMapEnvSource {
    /// The ConfigMap to select from.
    #[serde(default, skip_serializing_if = "should_skip_reference")]
    pub local_object_reference: LocalObjectReference,
    /// Specify whether the ConfigMap must be defined.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

impl Default for ConfigMapEnvSource {
    fn default() -> Self {
        Self {
            local_object_reference: LocalObjectReference::default(),
            optional: None,
        }
    }
}

/// SecretEnvSource selects a Secret to populate environment variables with.
///
/// The contents of the target Secret's Data field will represent the
/// key-value pairs as environment variables.
///
/// Corresponds to [Kubernetes SecretEnvSource](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2371)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SecretEnvSource {
    /// The Secret to select from.
    #[serde(default, skip_serializing_if = "should_skip_reference")]
    pub local_object_reference: LocalObjectReference,
    /// Specify whether the Secret must be defined.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

impl Default for SecretEnvSource {
    fn default() -> Self {
        Self {
            local_object_reference: LocalObjectReference::default(),
            optional: None,
        }
    }
}

/// Helper function for checking if LocalObjectReference should be skipped.
fn should_skip_reference(ref_: &LocalObjectReference) -> bool {
    ref_.name.as_ref().map_or(true, |s| s.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::util::Quantity;
    use crate::core::internal::selector::{
        object_field_path, object_field_selector_api_version, resource_field_selector_resource,
    };

    #[test]
    fn test_env_var_default() {
        let env = EnvVar::default();
        assert!(env.name.is_empty());
        assert!(env.value.is_empty());
        assert!(env.value_from.is_none());
    }

    #[test]
    fn test_env_var_with_value() {
        let env = EnvVar {
            name: "DATABASE_URL".to_string(),
            value: "postgres://localhost".to_string(),
            value_from: None,
        };

        let json = serde_json::to_string(&env).unwrap();
        let deserialized: EnvVar = serde_json::from_str(&json).unwrap();

        assert_eq!(env.name, deserialized.name);
        assert_eq!(env.value, deserialized.value);
        assert!(deserialized.value_from.is_none());
    }

    #[test]
    fn test_env_var_with_value_from() {
        let env = EnvVar {
            name: "POD_NAME".to_string(),
            value: String::new(),
            value_from: Some(EnvVarSource {
                field_ref: Some(ObjectFieldSelector {
                    api_version: object_field_selector_api_version::V1.to_string(),
                    field_path: object_field_path::METADATA_NAME.to_string(),
                }),
                ..Default::default()
            }),
        };

        let json = serde_json::to_string(&env).unwrap();
        let deserialized: EnvVar = serde_json::from_str(&json).unwrap();

        assert_eq!(env.name, deserialized.name);
        assert!(deserialized.value_from.is_some());
        assert!(deserialized.value_from.unwrap().field_ref.is_some());
    }

    #[test]
    fn test_env_var_source_default() {
        let source = EnvVarSource::default();
        assert!(source.field_ref.is_none());
        assert!(source.resource_field_ref.is_none());
        assert!(source.config_map_key_ref.is_none());
        assert!(source.secret_key_ref.is_none());
        assert!(source.file_key_ref.is_none());
    }

    #[test]
    fn test_env_var_source_with_field_ref() {
        let source = EnvVarSource {
            field_ref: Some(ObjectFieldSelector {
                api_version: "v1".to_string(),
                field_path: "metadata.namespace".to_string(),
            }),
            ..Default::default()
        };

        let json = serde_json::to_string(&source).unwrap();
        let deserialized: EnvVarSource = serde_json::from_str(&json).unwrap();

        assert!(deserialized.field_ref.is_some());
        assert!(deserialized.resource_field_ref.is_none());
    }

    #[test]
    fn test_env_var_source_with_resource_field_ref() {
        let source = EnvVarSource {
            resource_field_ref: Some(ResourceFieldSelector {
                container_name: "my-container".to_string(),
                resource: resource_field_selector_resource::LIMITS_CPU.to_string(),
                divisor: Some(Quantity::from("1")),
            }),
            ..Default::default()
        };

        let json = serde_json::to_string(&source).unwrap();
        let deserialized: EnvVarSource = serde_json::from_str(&json).unwrap();

        assert!(deserialized.resource_field_ref.is_some());
    }

    #[test]
    fn test_env_var_source_with_config_map_key_ref() {
        let source = EnvVarSource {
            config_map_key_ref: Some(ConfigMapKeySelector {
                name: Some("my-config".to_string()),
                key: "config-key".to_string(),
                optional: Some(true),
            }),
            ..Default::default()
        };

        let json = serde_json::to_string(&source).unwrap();
        let deserialized: EnvVarSource = serde_json::from_str(&json).unwrap();

        assert!(deserialized.config_map_key_ref.is_some());
    }

    #[test]
    fn test_env_var_source_with_secret_key_ref() {
        let source = EnvVarSource {
            secret_key_ref: Some(SecretKeySelector {
                name: Some("my-secret".to_string()),
                key: "secret-key".to_string(),
                optional: Some(false),
            }),
            ..Default::default()
        };

        let json = serde_json::to_string(&source).unwrap();
        let deserialized: EnvVarSource = serde_json::from_str(&json).unwrap();

        assert!(deserialized.secret_key_ref.is_some());
    }

    #[test]
    fn test_env_var_source_with_file_key_ref() {
        let source = EnvVarSource {
            file_key_ref: Some(FileKeySelector {
                volume_name: "env-vol".to_string(),
                path: "/etc/config/env.json".to_string(),
                key: "API_KEY".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };

        let json = serde_json::to_string(&source).unwrap();
        let deserialized: EnvVarSource = serde_json::from_str(&json).unwrap();

        assert!(deserialized.file_key_ref.is_some());
    }

    #[test]
    fn test_env_from_source_default() {
        let env_from = EnvFromSource::default();
        assert!(env_from.prefix.is_empty());
        assert!(env_from.config_map_ref.is_none());
        assert!(env_from.secret_ref.is_none());
    }

    #[test]
    fn test_env_from_source_with_config_map() {
        let env_from = EnvFromSource {
            prefix: "CONFIG_".to_string(),
            config_map_ref: Some(ConfigMapEnvSource {
                local_object_reference: LocalObjectReference {
                    name: Some("app-config".to_string()),
                },
                optional: Some(true),
            }),
            secret_ref: None,
        };

        let json = serde_json::to_string(&env_from).unwrap();
        let deserialized: EnvFromSource = serde_json::from_str(&json).unwrap();

        assert_eq!(env_from.prefix, deserialized.prefix);
        assert!(deserialized.config_map_ref.is_some());
        assert!(deserialized.secret_ref.is_none());
    }

    #[test]
    fn test_env_from_source_with_secret() {
        let env_from = EnvFromSource {
            prefix: String::new(),
            config_map_ref: None,
            secret_ref: Some(SecretEnvSource {
                local_object_reference: LocalObjectReference {
                    name: Some("db-secret".to_string()),
                },
                optional: None,
            }),
        };

        let json = serde_json::to_string(&env_from).unwrap();
        let deserialized: EnvFromSource = serde_json::from_str(&json).unwrap();

        assert!(deserialized.config_map_ref.is_none());
        assert!(deserialized.secret_ref.is_some());
    }

    #[test]
    fn test_config_map_env_source_default() {
        let source = ConfigMapEnvSource::default();
        assert!(source.local_object_reference.name.is_none());
        assert!(source.optional.is_none());
    }

    #[test]
    fn test_config_map_env_source_with_fields() {
        let source = ConfigMapEnvSource {
            local_object_reference: LocalObjectReference {
                name: Some("my-config".to_string()),
            },
            optional: Some(true),
        };

        let json = serde_json::to_string(&source).unwrap();
        let deserialized: ConfigMapEnvSource = serde_json::from_str(&json).unwrap();

        assert_eq!(
            source.local_object_reference.name,
            deserialized.local_object_reference.name
        );
        assert_eq!(source.optional, deserialized.optional);
    }

    #[test]
    fn test_secret_env_source_default() {
        let source = SecretEnvSource::default();
        assert!(source.local_object_reference.name.is_none());
        assert!(source.optional.is_none());
    }

    #[test]
    fn test_secret_env_source_with_fields() {
        let source = SecretEnvSource {
            local_object_reference: LocalObjectReference {
                name: Some("my-secret".to_string()),
            },
            optional: Some(false),
        };

        let json = serde_json::to_string(&source).unwrap();
        let deserialized: SecretEnvSource = serde_json::from_str(&json).unwrap();

        assert_eq!(
            source.local_object_reference.name,
            deserialized.local_object_reference.name
        );
        assert_eq!(source.optional, deserialized.optional);
    }

    #[test]
    fn test_env_var_serialize() {
        let env = EnvVar {
            name: "TEST_VAR".to_string(),
            value: "test-value".to_string(),
            value_from: None,
        };

        let json = serde_json::to_string(&env).unwrap();
        assert!(json.contains("\"name\":\"TEST_VAR\""));
        assert!(json.contains("\"value\":\"test-value\""));
    }

    #[test]
    fn test_env_var_deserialize() {
        let json = r#"{"name":"API_KEY","value":"secret123"}"#;
        let env: EnvVar = serde_json::from_str(json).unwrap();

        assert_eq!(env.name, "API_KEY");
        assert_eq!(env.value, "secret123");
    }

    #[test]
    fn test_env_var_round_trip() {
        let original = EnvVar {
            name: "DATABASE_URL".to_string(),
            value: String::new(),
            value_from: Some(EnvVarSource {
                secret_key_ref: Some(SecretKeySelector {
                    name: Some("api-secret".to_string()),
                    key: "key".to_string(),
                    optional: Some(true),
                }),
                ..Default::default()
            }),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: EnvVar = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_env_var_source_round_trip() {
        let original = EnvVarSource {
            field_ref: Some(ObjectFieldSelector {
                api_version: "v1".to_string(),
                field_path: "metadata.name".to_string(),
            }),
            ..Default::default()
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: EnvVarSource = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_env_from_source_round_trip() {
        let original = EnvFromSource {
            prefix: "APP_".to_string(),
            config_map_ref: Some(ConfigMapEnvSource {
                local_object_reference: LocalObjectReference {
                    name: Some("app-config".to_string()),
                },
                optional: None,
            }),
            secret_ref: None,
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: EnvFromSource = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_config_map_env_source_round_trip() {
        let original = ConfigMapEnvSource {
            local_object_reference: LocalObjectReference {
                name: Some("test-config".to_string()),
            },
            optional: Some(true),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: ConfigMapEnvSource = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_secret_env_source_round_trip() {
        let original = SecretEnvSource {
            local_object_reference: LocalObjectReference {
                name: Some("test-secret".to_string()),
            },
            optional: Some(false),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: SecretEnvSource = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_env_var_serialization_without_value_from() {
        let env = EnvVar {
            name: "SIMPLE_VAR".to_string(),
            value: "simple-value".to_string(),
            value_from: None,
        };

        let json = serde_json::to_string(&env).unwrap();
        assert!(!json.contains("valueFrom"));
    }

    #[test]
    fn test_env_var_serialization_without_value() {
        let env = EnvVar {
            name: "REF_VAR".to_string(),
            value: String::new(),
            value_from: Some(EnvVarSource {
                field_ref: Some(ObjectFieldSelector {
                    api_version: "v1".to_string(),
                    field_path: "metadata.name".to_string(),
                }),
                ..Default::default()
            }),
        };

        let json = serde_json::to_string(&env).unwrap();
        assert!(!json.contains("\"value\""));
    }

    #[test]
    fn test_env_from_source_serialization_without_prefix() {
        let env_from = EnvFromSource {
            prefix: String::new(),
            config_map_ref: Some(ConfigMapEnvSource {
                local_object_reference: LocalObjectReference {
                    name: Some("config".to_string()),
                },
                optional: None,
            }),
            secret_ref: None,
        };

        let json = serde_json::to_string(&env_from).unwrap();
        assert!(!json.contains("\"prefix\""));
    }

    #[test]
    fn test_config_map_env_source_empty_name_serialization() {
        let source = ConfigMapEnvSource {
            local_object_reference: LocalObjectReference {
                name: Some(String::new()),
            },
            optional: Some(true),
        };

        let json = serde_json::to_string(&source).unwrap();
        assert!(!json.contains("localObjectReference"));
    }

    #[test]
    fn test_secret_env_source_empty_name_serialization() {
        let source = SecretEnvSource {
            local_object_reference: LocalObjectReference {
                name: Some(String::new()),
            },
            optional: None,
        };

        let json = serde_json::to_string(&source).unwrap();
        assert!(!json.contains("localObjectReference"));
    }
}
