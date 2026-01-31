use crate::common::test_utils::assert_conversion_roundtrip;
use crate::common::{ApplyDefault, ListMeta, ObjectMeta, TypeMeta};
use crate::core::internal;
use crate::core::v1::{Namespace, NamespaceList, NamespaceSpec, NamespaceStatus};

fn namespace_basic() -> Namespace {
    Namespace {
        type_meta: TypeMeta::default(),
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
    let mut item = namespace_basic();
    item.apply_default();
    NamespaceList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("6".to_string()),
            ..Default::default()
        }),
        items: vec![item],
    }
}

#[test]
fn conversion_roundtrip_namespace() {
    assert_conversion_roundtrip::<Namespace, internal::namespace::Namespace>(namespace_basic());
}

#[test]
fn conversion_roundtrip_namespace_list() {
    assert_conversion_roundtrip::<NamespaceList, internal::namespace::NamespaceList>(
        namespace_list_basic(),
    );
}
