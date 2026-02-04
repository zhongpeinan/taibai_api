//! Core/v1 metadata and defaults tests

use crate::common::{ApplyDefault, HasTypeMetaReadOnly, ResourceSchema, VersionedObject};
use crate::core::v1::{
    Binding, ConfigMap, LimitRange, Namespace, PersistentVolume, Pod, Service,
};

fn assert_metadata_access<T: VersionedObject + Default>() {
    let resource = T::default();
    let meta = resource.metadata();
    assert!(meta.name.is_none());

    let mut resource = T::default();
    resource.metadata_mut().name = Some("test".to_string());
    assert_eq!(resource.metadata().name.as_deref(), Some("test"));
}

fn assert_apply_default_sets_type_meta<T: ApplyDefault + ResourceSchema + Default + HasTypeMetaReadOnly>() {
    let mut resource = T::default();
    resource.apply_default();
    let group = <T as ResourceSchema>::group_static();
    let version = <T as ResourceSchema>::version_static();
    let expected_api_version = if group.is_empty() {
        version.to_string()
    } else {
        format!("{}/{}", group, version)
    };
    assert_eq!(resource.type_meta().api_version, expected_api_version);
    assert_eq!(resource.type_meta().kind, <T as ResourceSchema>::kind_static());
}

#[test]
fn versioned_object_metadata_access() {
    assert_metadata_access::<Namespace>();
    assert_metadata_access::<ConfigMap>();
    assert_metadata_access::<Binding>();
    assert_metadata_access::<LimitRange>();
    assert_metadata_access::<Service>();
    assert_metadata_access::<PersistentVolume>();
    assert_metadata_access::<Pod>();
}

#[test]
fn apply_default_sets_type_meta() {
    assert_apply_default_sets_type_meta::<Namespace>();
    assert_apply_default_sets_type_meta::<ConfigMap>();
    assert_apply_default_sets_type_meta::<Binding>();
    assert_apply_default_sets_type_meta::<LimitRange>();
    assert_apply_default_sets_type_meta::<Service>();
    assert_apply_default_sets_type_meta::<PersistentVolume>();
    assert_apply_default_sets_type_meta::<Pod>();
}
