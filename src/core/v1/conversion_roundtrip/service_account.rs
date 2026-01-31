use crate::common::test_utils::assert_conversion_roundtrip;
use crate::common::{ApplyDefault, ListMeta, ObjectMeta, TypeMeta};
use crate::core::internal;
use crate::core::v1::reference::{LocalObjectReference, ObjectReference};
use crate::core::v1::{ServiceAccount, ServiceAccountList};

fn service_account_basic() -> ServiceAccount {
    ServiceAccount {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("builder".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        secrets: vec![ObjectReference {
            name: Some("builder-token".to_string()),
            ..Default::default()
        }],
        image_pull_secrets: vec![LocalObjectReference {
            name: Some("registry".to_string()),
        }],
        automount_service_account_token: Some(true),
    }
}

fn service_account_list_basic() -> ServiceAccountList {
    let mut item = service_account_basic();
    item.apply_default();
    ServiceAccountList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("7".to_string()),
            ..Default::default()
        }),
        items: vec![item],
    }
}

#[test]
fn conversion_roundtrip_service_account() {
    assert_conversion_roundtrip::<ServiceAccount, internal::config::ServiceAccount>(
        service_account_basic(),
    );
}

#[test]
fn conversion_roundtrip_service_account_list() {
    assert_conversion_roundtrip::<ServiceAccountList, internal::config::ServiceAccountList>(
        service_account_list_basic(),
    );
}
