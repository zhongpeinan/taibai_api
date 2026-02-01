use crate::common::test_utils::assert_serde_roundtrip;
use crate::common::{ListMeta, ObjectMeta, TypeMeta};
use crate::core::v1::{Namespace, NamespaceList, NamespaceSpec, NamespaceStatus};

fn namespace_basic() -> Namespace {
    Namespace {
        type_meta: TypeMeta {
            api_version: "v1".to_string(),
            kind: "Namespace".to_string(),
        },
        metadata: Some(ObjectMeta {
            name: Some("dev".to_string()),
            ..Default::default()
        }),
        spec: Some(NamespaceSpec {
            finalizers: vec!["kubernetes".to_string()],
        }),
        status: Some(NamespaceStatus {
            phase: Some("Active".to_string()),
            ..Default::default()
        }),
    }
}

fn namespace_list_basic() -> NamespaceList {
    NamespaceList {
        type_meta: TypeMeta {
            api_version: "v1".to_string(),
            kind: "NamespaceList".to_string(),
        },
        metadata: Some(ListMeta {
            resource_version: Some("6".to_string()),
            ..Default::default()
        }),
        items: vec![namespace_basic()],
    }
}

#[test]
fn serde_roundtrip_namespace() {
    assert_serde_roundtrip(&namespace_basic());
}

#[test]
fn serde_roundtrip_namespace_list() {
    assert_serde_roundtrip(&namespace_list_basic());
}
