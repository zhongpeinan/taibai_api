use super::*;
use crate::apiserverinternal::internal;
use crate::common::{
    ApplyDefault, FromInternal, HasObjectMeta, ResourceSchema, ToInternal, VersionedObject,
};

#[test]
fn top_level_resources_implement_required_traits() {
    fn check_versioned<T: VersionedObject + ApplyDefault>() {}
    fn check_default<T: Default>() {}
    fn check_schema<T: ResourceSchema>() {}

    check_versioned::<StorageVersion>();

    check_default::<StorageVersion>();
    check_default::<StorageVersionList>();

    check_schema::<StorageVersion>();
    check_schema::<StorageVersionList>();
}

#[test]
fn top_level_resources_have_conversion_traits() {
    fn check_conversion<T, I>()
    where
        T: ToInternal<I> + FromInternal<I>,
    {
    }

    check_conversion::<StorageVersion, internal::StorageVersion>();
    check_conversion::<StorageVersionList, internal::StorageVersionList>();
}

#[test]
fn top_level_resources_implement_prost_message() {
    fn check_prost<T: prost::Message>() {}

    check_prost::<StorageVersion>();
    check_prost::<StorageVersionList>();
}

#[test]
fn versioned_object_metadata_access() {
    let resource = StorageVersion::default();
    let meta = resource.metadata();
    assert!(meta.name.is_none());

    let mut resource = StorageVersion::default();
    resource.metadata_mut().name = Some("storage-version".to_string());
    assert_eq!(resource.metadata().name.as_deref(), Some("storage-version"));
}

#[test]
fn apply_default_sets_type_meta() {
    let mut resource = StorageVersion::default();
    resource.apply_default();
    assert_eq!(
        resource.type_meta.api_version,
        "apiserverinternal.k8s.io/v1alpha1"
    );
    assert_eq!(resource.type_meta.kind, "StorageVersion");
}

#[test]
fn internal_resources_implement_required_traits() {
    fn check<T: HasObjectMeta>() {}

    check::<internal::StorageVersion>();
}
