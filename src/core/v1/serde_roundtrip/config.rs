use crate::common::test_utils::assert_serde_roundtrip;
use crate::common::{ListMeta, ObjectMeta, TypeMeta};
use crate::core::internal::ByteString;
use crate::core::v1::{ConfigMap, ConfigMapList, Secret, SecretList, secret_type};
use std::collections::BTreeMap;

fn config_map_basic() -> ConfigMap {
    ConfigMap {
        type_meta: TypeMeta {
            api_version: "v1".to_string(),
            kind: "ConfigMap".to_string(),
        },
        metadata: Some(ObjectMeta {
            name: Some("app-config".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        data: BTreeMap::from([("config".to_string(), "port=8080".to_string())]),
        binary_data: BTreeMap::from([("payload".to_string(), ByteString::from(b"data".to_vec()))]),
        immutable: Some(true),
    }
}

fn config_map_list_basic() -> ConfigMapList {
    ConfigMapList {
        type_meta: TypeMeta {
            api_version: "v1".to_string(),
            kind: "ConfigMapList".to_string(),
        },
        metadata: Some(ListMeta {
            resource_version: Some("4".to_string()),
            ..Default::default()
        }),
        items: vec![config_map_basic()],
    }
}

fn secret_basic() -> Secret {
    Secret {
        type_meta: TypeMeta {
            api_version: "v1".to_string(),
            kind: "Secret".to_string(),
        },
        metadata: Some(ObjectMeta {
            name: Some("app-secret".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        immutable: Some(false),
        data: BTreeMap::from([("token".to_string(), ByteString::from(b"secret".to_vec()))]),
        string_data: BTreeMap::new(),
        type_: Some(secret_type::OPAQUE.to_string()),
    }
}

fn secret_list_basic() -> SecretList {
    SecretList {
        type_meta: TypeMeta {
            api_version: "v1".to_string(),
            kind: "SecretList".to_string(),
        },
        metadata: Some(ListMeta {
            resource_version: Some("5".to_string()),
            ..Default::default()
        }),
        items: vec![secret_basic()],
    }
}

#[test]
fn serde_roundtrip_config_map() {
    assert_serde_roundtrip(&config_map_basic());
}

#[test]
fn serde_roundtrip_config_map_list() {
    assert_serde_roundtrip(&config_map_list_basic());
}

#[test]
fn serde_roundtrip_secret() {
    assert_serde_roundtrip(&secret_basic());
}

#[test]
fn serde_roundtrip_secret_list() {
    assert_serde_roundtrip(&secret_list_basic());
}
