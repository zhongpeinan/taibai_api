use crate::common::test_utils::assert_serde_roundtrip;
use crate::common::{ListMeta, ObjectMeta, TypeMeta};
use crate::core::v1::reference::{LocalObjectReference, ObjectReference};
use crate::core::v1::{ServiceAccount, ServiceAccountList};

fn service_account_basic() -> ServiceAccount {
    ServiceAccount {
        type_meta: TypeMeta {
            api_version: "v1".to_string(),
            kind: "ServiceAccount".to_string(),
        },
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
    ServiceAccountList {
        type_meta: TypeMeta {
            api_version: "v1".to_string(),
            kind: "ServiceAccountList".to_string(),
        },
        metadata: Some(ListMeta {
            resource_version: Some("7".to_string()),
            ..Default::default()
        }),
        items: vec![service_account_basic()],
    }
}

#[test]
fn serde_roundtrip_service_account() {
    assert_serde_roundtrip(&service_account_basic());
}

#[test]
fn serde_roundtrip_service_account_list() {
    assert_serde_roundtrip(&service_account_list_basic());
}
