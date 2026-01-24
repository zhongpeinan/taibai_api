use super::{StorageVersion, StorageVersionList};
use crate::common::{ApplyDefault, TypeMeta};

#[test]
fn storage_version_apply_default_sets_type_meta() {
    let mut value = StorageVersion::default();
    value.apply_default();
    assert_eq!(
        value.type_meta.api_version,
        "apiserverinternal.k8s.io/v1alpha1"
    );
    assert_eq!(value.type_meta.kind, "StorageVersion");
}

#[test]
fn storage_version_apply_default_preserves_existing_type_meta() {
    let mut value = StorageVersion {
        type_meta: TypeMeta {
            api_version: "custom/v1".to_string(),
            kind: "CustomKind".to_string(),
        },
        ..StorageVersion::default()
    };
    value.apply_default();
    assert_eq!(value.type_meta.api_version, "custom/v1");
    assert_eq!(value.type_meta.kind, "CustomKind");
}

#[test]
fn storage_version_list_apply_default_sets_type_meta() {
    let mut value = StorageVersionList::default();
    value.apply_default();
    assert_eq!(
        value.type_meta.api_version,
        "apiserverinternal.k8s.io/v1alpha1"
    );
    assert_eq!(value.type_meta.kind, "StorageVersionList");
}
