//! ConfigMap and Secret types from the Kubernetes Core API
//!
//! This module contains types for configuration storage resources,
//! including ConfigMap, Secret, and ServiceAccount.
//!
//! Source: k8s-pkg/apis/core/types.go

use crate::common::{ListMeta, ObjectMeta, TypeMeta};
use crate::core::internal::helper::ByteString;
use crate::core::internal::{LocalObjectReference, SecretType};
use serde::{Deserialize, Serialize};

// ============================================================================
// ConfigMap
// ============================================================================

/// ConfigMap holds configuration data for pods to consume.
///
/// Corresponds to [Kubernetes ConfigMap](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5324)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ConfigMap {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ObjectMeta,
    /// Data contains the configuration data.
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub data: std::collections::BTreeMap<String, String>,
    /// BinaryData contains the binary data.
    /// Keys are similar to Data but values are base64-encoded.
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub binary_data: std::collections::BTreeMap<String, ByteString>,
    /// Immutable, if set to true, guarantees that the stored data cannot be updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub immutable: Option<bool>,
}

/// ConfigMapList is a list of ConfigMaps.
///
/// Corresponds to [Kubernetes ConfigMapList](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5337)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ConfigMapList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    /// Items is the list of ConfigMaps.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ConfigMap>,
}

// ============================================================================
// Secret
// ============================================================================

/// Secret holds secret data of a certain type.
///
/// Corresponds to [Kubernetes Secret](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5494)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Secret {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ObjectMeta,
    /// Data contains the secret data.
    /// Each key must consist of alphanumeric characters, '-', '_' or '.'.
    /// Values are base64-encoded strings.
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub data: std::collections::BTreeMap<String, ByteString>,
    /// Used to facilitate programmatic handling of secret data.
    #[serde(default)]
    pub r#type: SecretType,
    /// Immutable, if set to true, guarantees that the stored data cannot be updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub immutable: Option<bool>,
    /// StringData allows specifying non-binary secret data in string form.
    #[serde(
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty",
        skip_deserializing
    )]
    pub string_data: std::collections::BTreeMap<String, String>,
}

/// SecretList is a list of Secret.
///
/// Corresponds to [Kubernetes SecretList](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5512)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SecretList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    /// Items is the list of Secret objects.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Secret>,
}

// ============================================================================
// ServiceAccount
// ============================================================================

/// ServiceAccount binds together: * a name, a principal that can be authenticated
/// and authorized * secrets that will be used by pods running as this service account.
///
/// Corresponds to [Kubernetes ServiceAccount](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5615)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceAccount {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ObjectMeta,
    /// Secrets is the list of secrets allowed to be used by pods running as this ServiceAccount.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub secrets: Vec<ObjectReference>,
    /// ImagePullSecrets is a list of references to secrets in the same namespace to use for pulling any images.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub image_pull_secrets: Vec<LocalObjectReference>,
    /// AutomountServiceAccountToken indicates whether pods running as this service account should have an API token automatically mounted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub automount_service_account_token: Option<bool>,
}

/// ServiceAccountList is a list of ServiceAccount objects.
///
/// Corresponds to [Kubernetes ServiceAccountList](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5630)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceAccountList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    /// Items is the list of ServiceAccounts.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ServiceAccount>,
}

// ============================================================================
// ObjectReference for ServiceAccount
// ============================================================================

/// ObjectReference contains enough information to let you inspect or modify the referred object.
/// This is used in ServiceAccount.secrets field.
///
/// Corresponds to [Kubernetes ObjectReference](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5557)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ObjectReference {
    /// Kind of the referent.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub kind: String,
    /// Namespace of the referent.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub namespace: String,
    /// Name of the referent.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// UID of the referent.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub uid: String,
    /// API group of the referent.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub api_group: String,
    /// API version of the referent.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub api_version: String,
    /// Specific resourceVersion to which this reference is made.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub resource_version: String,
    /// Specific field within this resource.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub field_path: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    // ConfigMap tests
    #[test]
    fn test_config_map_default() {
        let cm = ConfigMap {
            type_meta: TypeMeta::default(),
            metadata: None,
            data: std::collections::BTreeMap::new(),
            binary_data: std::collections::BTreeMap::new(),
            immutable: None,
        };
        assert!(cm.data.is_empty());
        assert!(cm.binary_data.is_empty());
        assert!(cm.immutable.is_none());
    }

    #[test]
    fn test_config_map_with_data() {
        let mut data = std::collections::BTreeMap::new();
        data.insert("key1".to_string(), "value1".to_string());
        data.insert("key2".to_string(), "value2".to_string());

        let cm = ConfigMap {
            type_meta: TypeMeta::default(),
            metadata: None,
            data,
            binary_data: std::collections::BTreeMap::new(),
            immutable: Some(true),
        };
        assert_eq!(cm.data.len(), 2);
        assert_eq!(cm.immutable, Some(true));
    }

    #[test]
    fn test_config_map_serialize() {
        let mut data = std::collections::BTreeMap::new();
        data.insert("config.yaml".to_string(), "key: value".to_string());

        let cm = ConfigMap {
            type_meta: TypeMeta {
                kind: "ConfigMap".to_string(),
                api_version: "v1".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("my-config".to_string()),
                ..Default::default()
            }),
            data,
            ..Default::default()
        };
        let json = serde_json::to_string(&cm).unwrap();
        assert!(json.contains(r#""kind":"ConfigMap""#));
        assert!(json.contains(r#""name":"my-config""#));
        assert!(json.contains(r#""data""#));
    }

    #[test]
    fn test_config_map_list_default() {
        let list = ConfigMapList::default();
        assert!(list.items.is_empty());
    }

    #[test]
    fn test_config_map_list_with_items() {
        let list = ConfigMapList {
            items: vec![ConfigMap {
                type_meta: TypeMeta {
                    kind: "ConfigMap".to_string(),
                    api_version: "v1".to_string(),
                },
                ..Default::default()
            }],
            ..Default::default()
        };
        assert_eq!(list.items.len(), 1);
    }

    // Secret tests
    #[test]
    fn test_secret_default() {
        let secret = Secret {
            type_meta: TypeMeta::default(),
            metadata: None,
            data: std::collections::BTreeMap::new(),
            r#type: SecretType::Opaque,
            immutable: None,
            string_data: std::collections::BTreeMap::new(),
        };
        assert!(secret.data.is_empty());
        assert_eq!(secret.r#type, SecretType::Opaque);
    }

    #[test]
    fn test_secret_serialize() {
        let secret = Secret {
            type_meta: TypeMeta {
                kind: "Secret".to_string(),
                api_version: "v1".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("my-secret".to_string()),
                ..Default::default()
            }),
            r#type: SecretType::Opaque,
            ..Default::default()
        };
        let json = serde_json::to_string(&secret).unwrap();
        assert!(json.contains(r#""kind":"Secret""#));
        assert!(json.contains(r#""name":"my-secret""#));
        assert!(json.contains(r#""type":"Opaque""#));
    }

    #[test]
    fn test_secret_list_default() {
        let list = SecretList::default();
        assert!(list.items.is_empty());
    }

    #[test]
    fn test_secret_list_with_items() {
        let list = SecretList {
            items: vec![Secret {
                type_meta: TypeMeta {
                    kind: "Secret".to_string(),
                    api_version: "v1".to_string(),
                },
                ..Default::default()
            }],
            ..Default::default()
        };
        assert_eq!(list.items.len(), 1);
    }

    #[test]
    fn test_secret_with_docker_config_type() {
        let secret = Secret {
            type_meta: TypeMeta::default(),
            metadata: None,
            data: std::collections::BTreeMap::new(),
            r#type: SecretType::DockerConfigJson,
            ..Default::default()
        };
        assert_eq!(secret.r#type, SecretType::DockerConfigJson);
    }

    // ServiceAccount tests
    #[test]
    fn test_service_account_default() {
        let sa = ServiceAccount {
            type_meta: TypeMeta::default(),
            metadata: None,
            secrets: vec![],
            image_pull_secrets: vec![],
            automount_service_account_token: None,
        };
        assert!(sa.secrets.is_empty());
        assert!(sa.image_pull_secrets.is_empty());
    }

    #[test]
    fn test_service_account_with_secrets() {
        let sa = ServiceAccount {
            type_meta: TypeMeta::default(),
            metadata: None,
            secrets: vec![ObjectReference {
                name: "my-secret".to_string(),
                ..Default::default()
            }],
            image_pull_secrets: vec![LocalObjectReference {
                name: Some("my-image-pull-secret".to_string()),
            }],
            automount_service_account_token: Some(true),
        };
        assert_eq!(sa.secrets.len(), 1);
        assert_eq!(sa.image_pull_secrets.len(), 1);
        assert_eq!(sa.automount_service_account_token, Some(true));
    }

    #[test]
    fn test_service_account_serialize() {
        let sa = ServiceAccount {
            type_meta: TypeMeta {
                kind: "ServiceAccount".to_string(),
                api_version: "v1".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("my-service-account".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let json = serde_json::to_string(&sa).unwrap();
        assert!(json.contains(r#""kind":"ServiceAccount""#));
        assert!(json.contains(r#""name":"my-service-account""#));
    }

    #[test]
    fn test_service_account_list_default() {
        let list = ServiceAccountList::default();
        assert!(list.items.is_empty());
    }

    #[test]
    fn test_service_account_list_with_items() {
        let list = ServiceAccountList {
            items: vec![ServiceAccount {
                type_meta: TypeMeta {
                    kind: "ServiceAccount".to_string(),
                    api_version: "v1".to_string(),
                },
                ..Default::default()
            }],
            ..Default::default()
        };
        assert_eq!(list.items.len(), 1);
    }

    // ObjectReference tests
    #[test]
    fn test_object_reference_default() {
        let reference = ObjectReference::default();
        assert!(reference.kind.is_empty());
        assert!(reference.name.is_empty());
    }

    #[test]
    fn test_object_reference_with_fields() {
        let reference = ObjectReference {
            kind: "Secret".to_string(),
            namespace: "default".to_string(),
            name: "my-secret".to_string(),
            uid: "abc123".to_string(),
            ..Default::default()
        };
        assert_eq!(reference.kind, "Secret");
        assert_eq!(reference.name, "my-secret");
    }

    #[test]
    fn test_object_reference_serialize() {
        let reference = ObjectReference {
            kind: "Secret".to_string(),
            name: "my-secret".to_string(),
            ..Default::default()
        };
        let json = serde_json::to_string(&reference).unwrap();
        assert!(json.contains(r#""kind":"Secret""#));
        assert!(json.contains(r#""name":"my-secret""#));
    }

    // Integration tests
    #[test]
    fn test_config_map_with_binary_data() {
        let mut binary_data = std::collections::BTreeMap::new();
        binary_data.insert("binary.bin".to_string(), ByteString(vec![0x00, 0x01, 0x02]));

        let cm = ConfigMap {
            type_meta: TypeMeta::default(),
            metadata: None,
            data: std::collections::BTreeMap::new(),
            binary_data,
            ..Default::default()
        };
        assert_eq!(cm.binary_data.len(), 1);
        // Verify base64 serialization
        let json = serde_json::to_string(&cm).unwrap();
        assert!(json.contains(r#""binaryData":{"binary.bin":"AAEC"}"#));
        assert_eq!(cm.binary_data["binary.bin"].0, vec![0x00, 0x01, 0x02]);
    }

    #[test]
    fn test_secret_with_data() {
        let mut data = std::collections::BTreeMap::new();
        data.insert("password".to_string(), ByteString(b"secret".to_vec()));

        let secret = Secret {
            type_meta: TypeMeta::default(),
            metadata: None,
            data,
            r#type: SecretType::Opaque,
            immutable: Some(false),
            string_data: std::collections::BTreeMap::new(),
        };
        assert_eq!(secret.data.len(), 1);
        // Verify base64 serialization
        let json = serde_json::to_string(&secret).unwrap();
        assert!(json.contains(r#""data":{"password":"c2VjcmV0"}"#));
        assert_eq!(secret.data["password"].0, b"secret".to_vec());
    }

    #[test]
    fn test_secret_with_multiple_keys() {
        let mut data = std::collections::BTreeMap::new();
        data.insert("username".to_string(), ByteString(b"admin".to_vec()));
        data.insert("password".to_string(), ByteString(b"secret".to_vec()));

        let secret = Secret {
            type_meta: TypeMeta::default(),
            metadata: None,
            data,
            ..Default::default()
        };
        assert_eq!(secret.data.len(), 2);
        // Verify both keys serialize to base64
        let json = serde_json::to_string(&secret).unwrap();
        assert!(json.contains(r#""data":{"#));
        assert!(json.contains(r#"username":"YWRtaW4=""#));
        assert!(json.contains(r#"password":"c2VjcmV0""#));
    }

    #[test]
    fn test_config_map_binary_data_round_trip() {
        let original = ConfigMap {
            type_meta: TypeMeta {
                kind: "ConfigMap".to_string(),
                api_version: "v1".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("test-config".to_string()),
                ..Default::default()
            }),
            data: std::collections::BTreeMap::new(),
            binary_data: {
                let mut map = std::collections::BTreeMap::new();
                map.insert("key".to_string(), ByteString(vec![1, 2, 3]));
                map
            },
            ..Default::default()
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: ConfigMap = serde_json::from_str(&json).unwrap();
        assert_eq!(
            original.binary_data["key"].0,
            deserialized.binary_data["key"].0
        );
    }

    #[test]
    fn test_secret_data_round_trip() {
        let original = Secret {
            type_meta: TypeMeta {
                kind: "Secret".to_string(),
                api_version: "v1".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("test-secret".to_string()),
                ..Default::default()
            }),
            data: {
                let mut map = std::collections::BTreeMap::new();
                map.insert("key".to_string(), ByteString(vec![0xFF, 0xFE, 0xFD]));
                map
            },
            ..Default::default()
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Secret = serde_json::from_str(&json).unwrap();
        assert_eq!(original.data["key"].0, deserialized.data["key"].0);
    }

    #[test]
    fn test_config_map_round_trip() {
        let original = ConfigMap {
            type_meta: TypeMeta {
                kind: "ConfigMap".to_string(),
                api_version: "v1".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("test-config".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: ConfigMap = serde_json::from_str(&json).unwrap();
        assert_eq!(
            original.metadata.unwrap().name,
            deserialized.metadata.unwrap().name
        );
    }

    #[test]
    fn test_secret_round_trip() {
        let original = Secret {
            type_meta: TypeMeta {
                kind: "Secret".to_string(),
                api_version: "v1".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("test-secret".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Secret = serde_json::from_str(&json).unwrap();
        assert_eq!(
            original.metadata.unwrap().name,
            deserialized.metadata.unwrap().name
        );
    }

    #[test]
    fn test_service_account_round_trip() {
        let original = ServiceAccount {
            type_meta: TypeMeta {
                kind: "ServiceAccount".to_string(),
                api_version: "v1".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("test-sa".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: ServiceAccount = serde_json::from_str(&json).unwrap();
        assert_eq!(
            original.metadata.unwrap().name,
            deserialized.metadata.unwrap().name
        );
    }
}
