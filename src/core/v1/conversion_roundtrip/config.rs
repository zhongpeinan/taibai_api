use crate::common::test_utils::assert_conversion_roundtrip;
use crate::common::{ApplyDefault, ListMeta, ObjectMeta, TypeMeta};
use crate::core::internal::{self, ByteString};
use crate::core::v1::{ConfigMap, ConfigMapList, Secret, SecretList, secret_type};
use std::collections::BTreeMap;

fn config_map_basic() -> ConfigMap {
    ConfigMap {
        type_meta: TypeMeta::default(),
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
    let mut item = config_map_basic();
    item.apply_default();
    ConfigMapList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("4".to_string()),
            ..Default::default()
        }),
        items: vec![item],
    }
}

fn secret_basic() -> Secret {
    Secret {
        type_meta: TypeMeta::default(),
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
    let mut item = secret_basic();
    item.apply_default();
    SecretList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("5".to_string()),
            ..Default::default()
        }),
        items: vec![item],
    }
}

#[test]
fn conversion_roundtrip_config_map() {
    assert_conversion_roundtrip::<ConfigMap, internal::config::ConfigMap>(config_map_basic());
}

#[test]
fn conversion_roundtrip_config_map_list() {
    assert_conversion_roundtrip::<ConfigMapList, internal::config::ConfigMapList>(
        config_map_list_basic(),
    );
}

#[test]
fn conversion_roundtrip_secret() {
    assert_conversion_roundtrip::<Secret, internal::config::Secret>(secret_basic());
}

#[test]
fn conversion_roundtrip_secret_list() {
    assert_conversion_roundtrip::<SecretList, internal::config::SecretList>(secret_list_basic());
}
