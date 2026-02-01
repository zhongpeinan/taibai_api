//! ConfigMap, Secret, and ServiceAccount conversion implementations
//!
//! Converts between core v1 and internal ConfigMap, Secret, and ServiceAccount types.
//! Based on k8s.io/kubernetes/pkg/apis/core/v1/conversion.go

use super::helpers::*;
use crate::common::{ApplyDefault, FromInternal, ToInternal};
use crate::core::internal;
use crate::core::v1::config;

// ============================================================================
// ConfigMap
// ============================================================================

impl ToInternal<internal::config::ConfigMap> for config::ConfigMap {
    fn to_internal(self) -> internal::config::ConfigMap {
        internal::config::ConfigMap {
            type_meta: crate::common::TypeMeta::default(),
            metadata: option_object_meta_to_meta(self.metadata),
            data: self.data,
            binary_data: self.binary_data,
            immutable: self.immutable,
        }
    }
}

impl FromInternal<internal::config::ConfigMap> for config::ConfigMap {
    fn from_internal(value: internal::config::ConfigMap) -> Self {
        let mut result = Self {
            type_meta: crate::common::TypeMeta::default(),
            metadata: meta_to_option_object_meta(value.metadata),
            data: value.data,
            binary_data: value.binary_data,
            immutable: value.immutable,
        };

        result
    }
}

// ============================================================================
// ConfigMapList
// ============================================================================

impl ToInternal<internal::config::ConfigMapList> for config::ConfigMapList {
    fn to_internal(self) -> internal::config::ConfigMapList {
        internal::config::ConfigMapList {
            type_meta: crate::common::TypeMeta::default(),
            metadata: option_list_meta_to_meta(self.metadata),
            items: self.items.into_iter().map(|i| i.to_internal()).collect(),
        }
    }
}

impl FromInternal<internal::config::ConfigMapList> for config::ConfigMapList {
    fn from_internal(value: internal::config::ConfigMapList) -> Self {
        let mut result = Self {
            type_meta: crate::common::TypeMeta::default(),
            metadata: meta_to_option_list_meta(value.metadata),
            items: value
                .items
                .into_iter()
                .map(config::ConfigMap::from_internal)
                .collect(),
        };

        result
    }
}

// ============================================================================
// Secret
// ============================================================================

impl ToInternal<internal::config::Secret> for config::Secret {
    fn to_internal(self) -> internal::config::Secret {
        let mut data = self.data;

        // string_data overwrites data
        // This matches upstream Kubernetes behavior where StringData is write-only
        // and gets merged into Data during conversion
        if !self.string_data.is_empty() {
            for (key, value) in self.string_data {
                data.insert(key, value.into_bytes().into());
            }
        }

        internal::config::Secret {
            type_meta: crate::common::TypeMeta::default(),
            metadata: option_object_meta_to_meta(self.metadata),
            data,
            r#type: option_string_to_secret_type(self.type_),
            immutable: self.immutable,
            string_data: Default::default(), // StringData is never stored in internal
        }
    }
}

impl FromInternal<internal::config::Secret> for config::Secret {
    fn from_internal(value: internal::config::Secret) -> Self {
        let mut result = Self {
            type_meta: crate::common::TypeMeta::default(),
            metadata: meta_to_option_object_meta(value.metadata),
            data: value.data,
            type_: secret_type_to_option_string(value.r#type),
            immutable: value.immutable,
            string_data: Default::default(), // StringData is write-only
        };

        result
    }
}

// ============================================================================
// SecretList
// ============================================================================

impl ToInternal<internal::config::SecretList> for config::SecretList {
    fn to_internal(self) -> internal::config::SecretList {
        internal::config::SecretList {
            type_meta: crate::common::TypeMeta::default(),
            metadata: option_list_meta_to_meta(self.metadata),
            items: self.items.into_iter().map(|i| i.to_internal()).collect(),
        }
    }
}

impl FromInternal<internal::config::SecretList> for config::SecretList {
    fn from_internal(value: internal::config::SecretList) -> Self {
        let mut result = Self {
            type_meta: crate::common::TypeMeta::default(),
            metadata: meta_to_option_list_meta(value.metadata),
            items: value
                .items
                .into_iter()
                .map(config::Secret::from_internal)
                .collect(),
        };

        result
    }
}

// ============================================================================
// ServiceAccount
// ============================================================================

impl ToInternal<internal::config::ServiceAccount> for config::ServiceAccount {
    fn to_internal(self) -> internal::config::ServiceAccount {
        internal::config::ServiceAccount {
            type_meta: crate::common::TypeMeta::default(),
            metadata: option_object_meta_to_meta(self.metadata),
            secrets: self
                .secrets
                .into_iter()
                .map(|s| internal::config::ObjectReference {
                    kind: s.kind.unwrap_or_default(),
                    namespace: s.namespace.unwrap_or_default(),
                    name: s.name.unwrap_or_default(),
                    uid: s.uid.unwrap_or_default(),
                    api_version: s.api_version.unwrap_or_default(),
                    resource_version: s.resource_version.unwrap_or_default(),
                    field_path: s.field_path.unwrap_or_default(),
                })
                .collect(),
            image_pull_secrets: self
                .image_pull_secrets
                .into_iter()
                .map(|i| internal::LocalObjectReference { name: i.name })
                .collect(),
            automount_service_account_token: self.automount_service_account_token,
        }
    }
}

impl FromInternal<internal::config::ServiceAccount> for config::ServiceAccount {
    fn from_internal(value: internal::config::ServiceAccount) -> Self {
        let mut result = Self {
            type_meta: crate::common::TypeMeta::default(),
            metadata: meta_to_option_object_meta(value.metadata),
            secrets: value
                .secrets
                .into_iter()
                .map(|s| crate::core::v1::reference::ObjectReference {
                    kind: if s.kind.is_empty() {
                        None
                    } else {
                        Some(s.kind)
                    },
                    namespace: if s.namespace.is_empty() {
                        None
                    } else {
                        Some(s.namespace)
                    },
                    name: if s.name.is_empty() {
                        None
                    } else {
                        Some(s.name)
                    },
                    uid: if s.uid.is_empty() { None } else { Some(s.uid) },
                    api_version: if s.api_version.is_empty() {
                        None
                    } else {
                        Some(s.api_version)
                    },
                    resource_version: if s.resource_version.is_empty() {
                        None
                    } else {
                        Some(s.resource_version)
                    },
                    field_path: if s.field_path.is_empty() {
                        None
                    } else {
                        Some(s.field_path)
                    },
                })
                .collect(),
            image_pull_secrets: value
                .image_pull_secrets
                .into_iter()
                .map(|i| crate::core::v1::reference::LocalObjectReference { name: i.name })
                .collect(),
            automount_service_account_token: value.automount_service_account_token,
        };

        result
    }
}

// ============================================================================
// ServiceAccountList
// ============================================================================

impl ToInternal<internal::config::ServiceAccountList> for config::ServiceAccountList {
    fn to_internal(self) -> internal::config::ServiceAccountList {
        internal::config::ServiceAccountList {
            type_meta: crate::common::TypeMeta::default(),
            metadata: option_list_meta_to_meta(self.metadata),
            items: self.items.into_iter().map(|i| i.to_internal()).collect(),
        }
    }
}

impl FromInternal<internal::config::ServiceAccountList> for config::ServiceAccountList {
    fn from_internal(value: internal::config::ServiceAccountList) -> Self {
        let mut result = Self {
            type_meta: crate::common::TypeMeta::default(),
            metadata: meta_to_option_list_meta(value.metadata),
            items: value
                .items
                .into_iter()
                .map(config::ServiceAccount::from_internal)
                .collect(),
        };

        result
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::internal;
    use std::collections::BTreeMap;

    #[test]
    fn test_configmap_roundtrip() {
        let mut data = BTreeMap::new();
        data.insert("key1".to_string(), "value1".to_string());
        data.insert("key2".to_string(), "value2".to_string());

        let v1_configmap = config::ConfigMap {
            type_meta: crate::common::TypeMeta {
                api_version: "v1".to_string(),
                kind: "ConfigMap".to_string(),
            },
            metadata: Some(crate::common::ObjectMeta {
                name: Some("my-config".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            data: data.clone(),
            binary_data: BTreeMap::new(),
            immutable: Some(true),
        };

        let internal_configmap = v1_configmap.clone().to_internal();
        assert_eq!(
            internal_configmap.metadata.name,
            Some("my-config".to_string())
        );
        assert_eq!(internal_configmap.data, data);
        assert_eq!(internal_configmap.immutable, Some(true));

        let mut roundtrip = config::ConfigMap::from_internal(internal_configmap);
        roundtrip.apply_default();
        assert_eq!(
            roundtrip.metadata.as_ref().unwrap().name,
            Some("my-config".to_string())
        );
        assert_eq!(roundtrip.data, data);
        assert_eq!(roundtrip.type_meta.api_version, "v1");
        assert_eq!(roundtrip.type_meta.kind, "ConfigMap");
    }

    #[test]
    fn test_secret_string_data_overwrites_data() {
        let mut data = BTreeMap::new();
        data.insert("username".to_string(), b"old-user".to_vec().into());

        let mut string_data = BTreeMap::new();
        string_data.insert("username".to_string(), "new-user".to_string());
        string_data.insert("password".to_string(), "secret123".to_string());

        let v1_secret = config::Secret {
            type_meta: crate::common::TypeMeta::default(),
            metadata: Some(crate::common::ObjectMeta {
                name: Some("my-secret".to_string()),
                ..Default::default()
            }),
            data: data.clone(),
            string_data: string_data.clone(),
            type_: Some("Opaque".to_string()),
            immutable: Some(false),
        };

        let internal_secret = v1_secret.to_internal();

        // StringData should overwrite Data
        assert_eq!(
            String::from_utf8(internal_secret.data.get("username").unwrap().0.clone()).unwrap(),
            "new-user"
        );
        assert_eq!(
            String::from_utf8(internal_secret.data.get("password").unwrap().0.clone()).unwrap(),
            "secret123"
        );
        assert!(matches!(
            internal_secret.r#type,
            internal::SecretType::Opaque
        ));
    }

    #[test]
    fn test_secret_type_enum_conversion() {
        let secret_types = vec![
            ("Opaque", internal::SecretType::Opaque),
            (
                "kubernetes.io/service-account-token",
                internal::SecretType::ServiceAccountToken,
            ),
            ("kubernetes.io/tls", internal::SecretType::Tls),
            ("kubernetes.io/basic-auth", internal::SecretType::BasicAuth),
        ];

        for (v1_type, expected_internal) in secret_types {
            let v1_secret = config::Secret {
                type_meta: crate::common::TypeMeta::default(),
                metadata: None,
                data: BTreeMap::new(),
                string_data: BTreeMap::new(),
                type_: Some(v1_type.to_string()),
                immutable: None,
            };

            let internal_secret = v1_secret.clone().to_internal();
            assert_eq!(internal_secret.r#type, expected_internal);

            let mut roundtrip = config::Secret::from_internal(internal_secret);
            roundtrip.apply_default();
            assert_eq!(roundtrip.type_.as_deref(), Some(v1_type));
        }
    }

    #[test]
    fn test_serviceaccount_roundtrip() {
        let v1_sa = config::ServiceAccount {
            type_meta: crate::common::TypeMeta {
                api_version: "v1".to_string(),
                kind: "ServiceAccount".to_string(),
            },
            metadata: Some(crate::common::ObjectMeta {
                name: Some("my-sa".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            secrets: vec![crate::core::v1::reference::ObjectReference {
                kind: Some("Secret".to_string()),
                name: Some("my-secret".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }],
            image_pull_secrets: vec![],
            automount_service_account_token: Some(true),
        };

        let internal_sa = v1_sa.clone().to_internal();
        assert_eq!(internal_sa.metadata.name, Some("my-sa".to_string()));
        assert_eq!(internal_sa.secrets.len(), 1);
        assert_eq!(internal_sa.secrets[0].name, "my-secret");
        assert_eq!(internal_sa.automount_service_account_token, Some(true));

        let mut roundtrip = config::ServiceAccount::from_internal(internal_sa);
        roundtrip.apply_default();
        assert_eq!(
            roundtrip.metadata.as_ref().unwrap().name,
            Some("my-sa".to_string())
        );
        assert_eq!(roundtrip.secrets.len(), 1);
        assert_eq!(roundtrip.secrets[0].name, Some("my-secret".to_string()));
        assert_eq!(roundtrip.type_meta.api_version, "v1");
        assert_eq!(roundtrip.type_meta.kind, "ServiceAccount");
    }

    #[test]
    fn test_configmap_list_roundtrip() {
        let v1_list = config::ConfigMapList {
            type_meta: crate::common::TypeMeta {
                api_version: "v1".to_string(),
                kind: "ConfigMapList".to_string(),
            },
            metadata: Some(crate::common::ListMeta::default()),
            items: vec![
                config::ConfigMap {
                    type_meta: crate::common::TypeMeta::default(),
                    metadata: Some(crate::common::ObjectMeta {
                        name: Some("config1".to_string()),
                        ..Default::default()
                    }),
                    data: BTreeMap::new(),
                    binary_data: BTreeMap::new(),
                    immutable: None,
                },
                config::ConfigMap {
                    type_meta: crate::common::TypeMeta::default(),
                    metadata: Some(crate::common::ObjectMeta {
                        name: Some("config2".to_string()),
                        ..Default::default()
                    }),
                    data: BTreeMap::new(),
                    binary_data: BTreeMap::new(),
                    immutable: None,
                },
            ],
        };

        let internal_list = v1_list.clone().to_internal();
        assert_eq!(internal_list.items.len(), 2);

        let mut roundtrip = config::ConfigMapList::from_internal(internal_list);
        roundtrip.apply_default();
        assert_eq!(roundtrip.items.len(), 2);
        assert_eq!(roundtrip.type_meta.api_version, "v1");
        assert_eq!(roundtrip.type_meta.kind, "ConfigMapList");
    }
}
