//! Core/v1 metadata and defaults tests

use crate::common::{
    ApplyDefault, HasListMeta, HasTypeMetaReadOnly, ResourceSchema, VersionedObject,
};
use crate::core::v1::{
    Binding, ComponentStatusList, ConfigMap, ConfigMapList, EndpointsList, EventList, LimitRange,
    LimitRangeList, Namespace, NamespaceList, NodeList, PersistentVolume,
    PersistentVolumeClaimList, PersistentVolumeList, Pod, PodList, PodTemplateList,
    ReplicationControllerList, ResourceQuotaList, SecretList, Service, ServiceAccountList,
    ServiceList,
};

fn assert_metadata_access<T: VersionedObject + Default>() {
    let resource = T::default();
    let meta = resource.metadata();
    assert!(meta.name.is_none());

    let mut resource = T::default();
    resource.metadata_mut().name = Some("test".to_string());
    assert_eq!(resource.metadata().name.as_deref(), Some("test"));
}

fn assert_apply_default_sets_type_meta<
    T: ApplyDefault + ResourceSchema + Default + HasTypeMetaReadOnly,
>() {
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
    assert_eq!(
        resource.type_meta().kind,
        <T as ResourceSchema>::kind_static()
    );
}

fn assert_list_metadata_access<T: HasListMeta + Default>() {
    let list = T::default();
    let meta = list.list_meta();
    assert!(meta.resource_version.is_none());

    let mut list = T::default();
    list.list_meta_mut().resource_version = Some("rv".to_string());
    assert_eq!(list.list_meta().resource_version.as_deref(), Some("rv"));
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

#[test]
fn list_meta_accessors() {
    assert_list_metadata_access::<ComponentStatusList>();
    assert_list_metadata_access::<ConfigMapList>();
    assert_list_metadata_access::<EndpointsList>();
    assert_list_metadata_access::<EventList>();
    assert_list_metadata_access::<LimitRangeList>();
    assert_list_metadata_access::<NamespaceList>();
    assert_list_metadata_access::<NodeList>();
    assert_list_metadata_access::<PersistentVolumeClaimList>();
    assert_list_metadata_access::<PersistentVolumeList>();
    assert_list_metadata_access::<PodList>();
    assert_list_metadata_access::<PodTemplateList>();
    assert_list_metadata_access::<ReplicationControllerList>();
    assert_list_metadata_access::<ResourceQuotaList>();
    assert_list_metadata_access::<SecretList>();
    assert_list_metadata_access::<ServiceAccountList>();
    assert_list_metadata_access::<ServiceList>();
}
