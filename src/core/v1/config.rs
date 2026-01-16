//! ConfigMap and Secret types from the Kubernetes Core v1 API
//!
//! This module contains types for configuration and secret management.

use crate::common::{ListMeta, ObjectMeta};
use crate::core::internal::ByteString;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// ConfigMap holds configuration data for pods to consume.
///
/// Corresponds to [Kubernetes ConfigMap](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L8039)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ConfigMap {
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// Immutable, if set to true, ensures that data stored in the ConfigMap cannot be updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub immutable: Option<bool>,

    /// Data contains the configuration data.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub data: BTreeMap<String, String>,

    /// BinaryData contains the binary data.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub binary_data: BTreeMap<String, ByteString>,
}

/// ConfigMapList is a list of ConfigMaps.
///
/// Corresponds to [Kubernetes ConfigMapList](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L8076)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ConfigMapList {
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// Items is the list of ConfigMaps.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ConfigMap>,
}

/// Secret holds secret data of a certain type.
///
/// Corresponds to [Kubernetes Secret](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7896)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Secret {
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// Immutable, if set to true, ensures that data stored in the Secret cannot be updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub immutable: Option<bool>,

    /// Data contains the secret data. Each key must be a valid DNS_SUBDOMAIN.
    /// The values are base64 encoded strings.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub data: BTreeMap<String, ByteString>,

    /// stringData allows specifying non-binary secret data in string form.
    /// It is provided as a write-only input field for convenience.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub string_data: BTreeMap<String, String>,

    /// Used to facilitate programmatic handling of secret data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// SecretList is a list of Secret.
///
/// Corresponds to [Kubernetes SecretList](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L8022)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SecretList {
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// Items is a list of secret objects.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Secret>,
}

/// Secret type constants
///
/// More info: https://kubernetes.io/docs/concepts/configuration/secret/#secret-types
pub mod secret_type {
    /// Opaque (default) secret type
    pub const OPAQUE: &str = "Opaque";

    /// Service account token secret type
    pub const SERVICE_ACCOUNT_TOKEN: &str = "kubernetes.io/service-account-token";

    /// Docker config secret type
    pub const DOCKER_CONFIG_JSON: &str = "kubernetes.io/dockercfg";

    /// Docker config JSON secret type
    pub const DOCKER_CONFIG: &str = "kubernetes.io/dockerconfigjson";

    /// Basic auth secret type
    pub const BASIC_AUTH: &str = "kubernetes.io/basic-auth";

    /// SSH auth secret type
    pub const SSH_AUTH: &str = "kubernetes.io/ssh-auth";

    /// TLS secret type
    pub const TLS: &str = "kubernetes.io/tls";

    /// Bootstrap token secret type
    pub const BOOTSTRAP_TOKEN: &str = "bootstrap.kubernetes.io/token";
}

#[cfg(test)]
mod tests {
    use super::*;

    // ConfigMap tests
    #[test]
    fn test_config_map_default() {
        let cm = ConfigMap {
            metadata: None,
            immutable: None,
            data: BTreeMap::new(),
            binary_data: BTreeMap::new(),
        };
        assert!(cm.metadata.is_none());
        assert!(cm.immutable.is_none());
        assert!(cm.data.is_empty());
        assert!(cm.binary_data.is_empty());
    }

    #[test]
    fn test_config_map_with_data() {
        let mut data = BTreeMap::new();
        data.insert("key1".to_string(), "value1".to_string());
        data.insert("key2".to_string(), "value2".to_string());

        let cm = ConfigMap {
            metadata: Some(ObjectMeta {
                name: Some("my-config".to_string()),
                ..Default::default()
            }),
            immutable: Some(false),
            data,
            binary_data: BTreeMap::new(),
        };

        assert_eq!(
            cm.metadata.as_ref().unwrap().name,
            Some("my-config".to_string())
        );
        assert_eq!(cm.immutable, Some(false));
        assert_eq!(cm.data.len(), 2);
        assert_eq!(cm.data.get("key1"), Some(&"value1".to_string()));
    }

    #[test]
    fn test_config_map_with_binary_data() {
        let mut binary_data = BTreeMap::new();
        binary_data.insert(
            "cert.pem".to_string(),
            ByteString(b"certificate data".to_vec()),
        );

        let cm = ConfigMap {
            metadata: Some(ObjectMeta {
                name: Some("binary-config".to_string()),
                ..Default::default()
            }),
            immutable: None,
            data: BTreeMap::new(),
            binary_data,
        };

        assert_eq!(
            cm.metadata.as_ref().unwrap().name,
            Some("binary-config".to_string())
        );
        assert_eq!(cm.binary_data.len(), 1);
        assert_eq!(
            cm.binary_data.get("cert.pem"),
            Some(&ByteString(b"certificate data".to_vec()))
        );
    }

    #[test]
    fn test_config_map_serialize() {
        let mut data = BTreeMap::new();
        data.insert("key1".to_string(), "value1".to_string());

        let cm = ConfigMap {
            metadata: Some(ObjectMeta {
                name: Some("my-config".to_string()),
                ..Default::default()
            }),
            immutable: Some(false),
            data,
            binary_data: BTreeMap::new(),
        };

        let json = serde_json::to_string(&cm).unwrap();
        assert!(json.contains(r#""name":"my-config""#));
        assert!(json.contains(r#""key1":"value1""#));
        assert!(json.contains(r#""immutable":false"#));
    }

    #[test]
    fn test_config_map_deserialize() {
        let json = r#"{
            "metadata": {"name": "my-config"},
            "data": {"key1": "value1", "key2": "value2"},
            "immutable": true
        }"#;

        let cm: ConfigMap = serde_json::from_str(json).unwrap();
        assert_eq!(
            cm.metadata.as_ref().unwrap().name,
            Some("my-config".to_string())
        );
        assert_eq!(cm.data.len(), 2);
        assert_eq!(cm.immutable, Some(true));
    }

    #[test]
    fn test_config_map_with_binary_data_serialize() {
        let mut binary_data = BTreeMap::new();
        binary_data.insert("file.bin".to_string(), ByteString(vec![0x01, 0x02, 0x03]));

        let cm = ConfigMap {
            metadata: Some(ObjectMeta {
                name: Some("binary-config".to_string()),
                ..Default::default()
            }),
            immutable: None,
            data: BTreeMap::new(),
            binary_data,
        };

        let json = serde_json::to_string(&cm).unwrap();
        assert!(json.contains(r#""binaryData":{"#));
        assert!(json.contains(r#""file.bin":"AQID""#));
    }

    #[test]
    fn test_config_map_list_empty() {
        let list = ConfigMapList {
            metadata: None,
            items: vec![],
        };
        assert!(list.metadata.is_none());
        assert!(list.items.is_empty());
    }

    #[test]
    fn test_config_map_list_with_items() {
        let cm1 = ConfigMap {
            metadata: Some(ObjectMeta {
                name: Some("config1".to_string()),
                ..Default::default()
            }),
            immutable: None,
            data: BTreeMap::new(),
            binary_data: BTreeMap::new(),
        };

        let cm2 = ConfigMap {
            metadata: Some(ObjectMeta {
                name: Some("config2".to_string()),
                ..Default::default()
            }),
            immutable: None,
            data: BTreeMap::new(),
            binary_data: BTreeMap::new(),
        };

        let list = ConfigMapList {
            metadata: Some(ListMeta {
                resource_version: Some("12345".to_string()),
                ..Default::default()
            }),
            items: vec![cm1, cm2],
        };

        assert_eq!(list.items.len(), 2);
        assert_eq!(
            list.metadata.as_ref().unwrap().resource_version,
            Some("12345".to_string())
        );
    }

    #[test]
    fn test_config_map_list_serialize() {
        let list = ConfigMapList {
            metadata: Some(ListMeta {
                resource_version: Some("12345".to_string()),
                ..Default::default()
            }),
            items: vec![ConfigMap {
                metadata: Some(ObjectMeta {
                    name: Some("my-config".to_string()),
                    ..Default::default()
                }),
                immutable: None,
                data: BTreeMap::new(),
                binary_data: BTreeMap::new(),
            }],
        };

        let json = serde_json::to_string(&list).unwrap();
        assert!(json.contains(r#""resourceVersion":"12345""#));
        assert!(json.contains(r#""name":"my-config""#));
    }

    #[test]
    fn test_config_map_round_trip() {
        let mut data = BTreeMap::new();
        data.insert("key1".to_string(), "value1".to_string());

        let mut binary_data = BTreeMap::new();
        binary_data.insert("cert.pem".to_string(), ByteString(b"cert".to_vec()));

        let original = ConfigMap {
            metadata: Some(ObjectMeta {
                name: Some("my-config".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            immutable: Some(false),
            data,
            binary_data,
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: ConfigMap = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    // Secret tests
    #[test]
    fn test_secret_default() {
        let secret = Secret {
            metadata: None,
            immutable: None,
            data: BTreeMap::new(),
            string_data: BTreeMap::new(),
            type_: None,
        };
        assert!(secret.metadata.is_none());
        assert!(secret.immutable.is_none());
        assert!(secret.data.is_empty());
        assert!(secret.string_data.is_empty());
        assert!(secret.type_.is_none());
    }

    #[test]
    fn test_secret_with_data() {
        let mut data = BTreeMap::new();
        data.insert("password".to_string(), ByteString(b"secret123".to_vec()));
        data.insert("username".to_string(), ByteString(b"admin".to_vec()));

        let secret = Secret {
            metadata: Some(ObjectMeta {
                name: Some("my-secret".to_string()),
                ..Default::default()
            }),
            immutable: Some(false),
            data,
            string_data: BTreeMap::new(),
            type_: Some(secret_type::OPAQUE.to_string()),
        };

        assert_eq!(
            secret.metadata.as_ref().unwrap().name,
            Some("my-secret".to_string())
        );
        assert_eq!(secret.immutable, Some(false));
        assert_eq!(secret.data.len(), 2);
        assert_eq!(
            secret.data.get("password"),
            Some(&ByteString(b"secret123".to_vec()))
        );
        assert_eq!(secret.type_, Some(secret_type::OPAQUE.to_string()));
    }

    #[test]
    fn test_secret_with_string_data() {
        let mut string_data = BTreeMap::new();
        string_data.insert("username".to_string(), "admin".to_string());
        string_data.insert("password".to_string(), "secret123".to_string());

        let secret = Secret {
            metadata: Some(ObjectMeta {
                name: Some("my-secret".to_string()),
                ..Default::default()
            }),
            immutable: None,
            data: BTreeMap::new(),
            string_data,
            type_: Some(secret_type::OPAQUE.to_string()),
        };

        assert_eq!(
            secret.metadata.as_ref().unwrap().name,
            Some("my-secret".to_string())
        );
        assert_eq!(secret.string_data.len(), 2);
        assert_eq!(
            secret.string_data.get("username"),
            Some(&"admin".to_string())
        );
    }

    #[test]
    fn test_secret_serialize() {
        let mut data = BTreeMap::new();
        data.insert("password".to_string(), ByteString(b"secret123".to_vec()));

        let secret = Secret {
            metadata: Some(ObjectMeta {
                name: Some("my-secret".to_string()),
                ..Default::default()
            }),
            immutable: Some(false),
            data,
            string_data: BTreeMap::new(),
            type_: Some(secret_type::OPAQUE.to_string()),
        };

        let json = serde_json::to_string(&secret).unwrap();
        assert!(json.contains(r#""name":"my-secret""#));
        assert!(json.contains(r#""type":"Opaque""#));
        assert!(json.contains(r#""immutable":false"#));
        assert!(json.contains(r#""data":{"password":"c2VjcmV0MTIz"}"#));
    }

    #[test]
    fn test_secret_deserialize() {
        // Note: In Kubernetes API, secret data is base64 encoded.
        // "c2VjcmV0MTIz" decodes to "secret123"
        let json = r#"{
            "metadata": {"name": "my-secret"},
            "data": {"password": "c2VjcmV0MTIz"},
            "type": "Opaque",
            "immutable": true
        }"#;

        let secret: Secret = serde_json::from_str(json).unwrap();
        assert_eq!(
            secret.metadata.as_ref().unwrap().name,
            Some("my-secret".to_string())
        );
        assert_eq!(secret.type_, Some(secret_type::OPAQUE.to_string()));
        assert_eq!(secret.immutable, Some(true));
        // Verify the decoded password value
        assert_eq!(
            secret.data.get("password").unwrap().0,
            b"secret123".to_vec()
        );
    }

    #[test]
    fn test_secret_with_type_docker_config() {
        let secret = Secret {
            metadata: Some(ObjectMeta {
                name: Some("docker-secret".to_string()),
                ..Default::default()
            }),
            immutable: None,
            data: BTreeMap::new(),
            string_data: BTreeMap::new(),
            type_: Some(secret_type::DOCKER_CONFIG.to_string()),
        };

        assert_eq!(secret.type_, Some(secret_type::DOCKER_CONFIG.to_string()));
    }

    #[test]
    fn test_secret_with_type_tls() {
        let secret = Secret {
            metadata: Some(ObjectMeta {
                name: Some("tls-secret".to_string()),
                ..Default::default()
            }),
            immutable: None,
            data: BTreeMap::new(),
            string_data: BTreeMap::new(),
            type_: Some(secret_type::TLS.to_string()),
        };

        assert_eq!(secret.type_, Some(secret_type::TLS.to_string()));
    }

    #[test]
    fn test_secret_with_type_service_account_token() {
        let secret = Secret {
            metadata: Some(ObjectMeta {
                name: Some("token-secret".to_string()),
                ..Default::default()
            }),
            immutable: None,
            data: BTreeMap::new(),
            string_data: BTreeMap::new(),
            type_: Some(secret_type::SERVICE_ACCOUNT_TOKEN.to_string()),
        };

        assert_eq!(
            secret.type_,
            Some(secret_type::SERVICE_ACCOUNT_TOKEN.to_string())
        );
    }

    #[test]
    fn test_secret_list_empty() {
        let list = SecretList {
            metadata: None,
            items: vec![],
        };
        assert!(list.metadata.is_none());
        assert!(list.items.is_empty());
    }

    #[test]
    fn test_secret_list_with_items() {
        let secret1 = Secret {
            metadata: Some(ObjectMeta {
                name: Some("secret1".to_string()),
                ..Default::default()
            }),
            immutable: None,
            data: BTreeMap::new(),
            string_data: BTreeMap::new(),
            type_: None,
        };

        let secret2 = Secret {
            metadata: Some(ObjectMeta {
                name: Some("secret2".to_string()),
                ..Default::default()
            }),
            immutable: None,
            data: BTreeMap::new(),
            string_data: BTreeMap::new(),
            type_: None,
        };

        let list = SecretList {
            metadata: Some(ListMeta {
                resource_version: Some("67890".to_string()),
                ..Default::default()
            }),
            items: vec![secret1, secret2],
        };

        assert_eq!(list.items.len(), 2);
        assert_eq!(
            list.metadata.as_ref().unwrap().resource_version,
            Some("67890".to_string())
        );
    }

    #[test]
    fn test_secret_list_serialize() {
        let list = SecretList {
            metadata: Some(ListMeta {
                resource_version: Some("67890".to_string()),
                ..Default::default()
            }),
            items: vec![Secret {
                metadata: Some(ObjectMeta {
                    name: Some("my-secret".to_string()),
                    ..Default::default()
                }),
                immutable: None,
                data: BTreeMap::new(),
                string_data: BTreeMap::new(),
                type_: Some(secret_type::OPAQUE.to_string()),
            }],
        };

        let json = serde_json::to_string(&list).unwrap();
        assert!(json.contains(r#""resourceVersion":"67890""#));
        assert!(json.contains(r#""name":"my-secret""#));
        assert!(json.contains(r#""type":"Opaque""#));
    }

    #[test]
    fn test_secret_round_trip() {
        let mut data = BTreeMap::new();
        data.insert("password".to_string(), ByteString(b"secret123".to_vec()));

        let original = Secret {
            metadata: Some(ObjectMeta {
                name: Some("my-secret".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            immutable: Some(false),
            data,
            string_data: BTreeMap::new(),
            type_: Some(secret_type::OPAQUE.to_string()),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Secret = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_secret_type_constants() {
        assert_eq!(secret_type::OPAQUE, "Opaque");
        assert_eq!(
            secret_type::SERVICE_ACCOUNT_TOKEN,
            "kubernetes.io/service-account-token"
        );
        assert_eq!(secret_type::DOCKER_CONFIG_JSON, "kubernetes.io/dockercfg");
        assert_eq!(secret_type::DOCKER_CONFIG, "kubernetes.io/dockerconfigjson");
        assert_eq!(secret_type::BASIC_AUTH, "kubernetes.io/basic-auth");
        assert_eq!(secret_type::SSH_AUTH, "kubernetes.io/ssh-auth");
        assert_eq!(secret_type::TLS, "kubernetes.io/tls");
        assert_eq!(
            secret_type::BOOTSTRAP_TOKEN,
            "bootstrap.kubernetes.io/token"
        );
    }

    #[test]
    fn test_secret_with_all_secret_types() {
        let types = vec![
            secret_type::OPAQUE,
            secret_type::SERVICE_ACCOUNT_TOKEN,
            secret_type::DOCKER_CONFIG_JSON,
            secret_type::DOCKER_CONFIG,
            secret_type::BASIC_AUTH,
            secret_type::SSH_AUTH,
            secret_type::TLS,
            secret_type::BOOTSTRAP_TOKEN,
        ];

        for secret_type_str in types {
            let secret = Secret {
                metadata: Some(ObjectMeta {
                    name: Some(format!("{}-secret", secret_type_str.replace('.', "-"))),
                    ..Default::default()
                }),
                immutable: None,
                data: BTreeMap::new(),
                string_data: BTreeMap::new(),
                type_: Some(secret_type_str.to_string()),
            };

            assert_eq!(secret.type_, Some(secret_type_str.to_string()));
        }
    }

    #[test]
    fn test_config_map_immutable_true() {
        let cm = ConfigMap {
            metadata: Some(ObjectMeta {
                name: Some("immutable-config".to_string()),
                ..Default::default()
            }),
            immutable: Some(true),
            data: BTreeMap::new(),
            binary_data: BTreeMap::new(),
        };

        assert_eq!(cm.immutable, Some(true));
    }

    #[test]
    fn test_secret_immutable_true() {
        let secret = Secret {
            metadata: Some(ObjectMeta {
                name: Some("immutable-secret".to_string()),
                ..Default::default()
            }),
            immutable: Some(true),
            data: BTreeMap::new(),
            string_data: BTreeMap::new(),
            type_: Some(secret_type::OPAQUE.to_string()),
        };

        assert_eq!(secret.immutable, Some(true));
    }

    #[test]
    fn test_config_map_with_data_and_binary_data() {
        let mut data = BTreeMap::new();
        data.insert("config.yaml".to_string(), "key: value".to_string());

        let mut binary_data = BTreeMap::new();
        binary_data.insert(
            "image.png".to_string(),
            ByteString(vec![0x89, 0x50, 0x4E, 0x47]),
        );

        let cm = ConfigMap {
            metadata: Some(ObjectMeta {
                name: Some("mixed-config".to_string()),
                ..Default::default()
            }),
            immutable: None,
            data,
            binary_data,
        };

        assert_eq!(cm.data.len(), 1);
        assert_eq!(cm.binary_data.len(), 1);
        assert_eq!(cm.data.get("config.yaml"), Some(&"key: value".to_string()));
        assert_eq!(
            cm.binary_data.get("image.png"),
            Some(&ByteString(vec![0x89, 0x50, 0x4E, 0x47]))
        );
    }

    #[test]
    fn test_secret_with_data_and_string_data() {
        let mut data = BTreeMap::new();
        data.insert(
            "binary-secret".to_string(),
            ByteString(b"binary-value".to_vec()),
        );

        let mut string_data = BTreeMap::new();
        string_data.insert("string-secret".to_string(), "string-value".to_string());

        let secret = Secret {
            metadata: Some(ObjectMeta {
                name: Some("mixed-secret".to_string()),
                ..Default::default()
            }),
            immutable: None,
            data,
            string_data,
            type_: Some(secret_type::OPAQUE.to_string()),
        };

        assert_eq!(secret.data.len(), 1);
        assert_eq!(secret.string_data.len(), 1);
        assert_eq!(
            secret.data.get("binary-secret"),
            Some(&ByteString(b"binary-value".to_vec()))
        );
        assert_eq!(
            secret.string_data.get("string-secret"),
            Some(&"string-value".to_string())
        );
    }
}
