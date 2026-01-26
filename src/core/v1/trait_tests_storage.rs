use super::*;
use crate::common::{ApplyDefault, FromInternal, ResourceSchema, ToInternal, VersionedObject};
use crate::core::internal;

#[test]
fn top_level_resources_implement_required_traits() {
    fn check_versioned<T: VersionedObject + ApplyDefault>() {}
    fn check_default<T: Default>() {}

    check_versioned::<PersistentVolume>();
    check_versioned::<PersistentVolumeClaim>();

    check_default::<PersistentVolume>();
    check_default::<PersistentVolumeClaim>();
}

#[test]
fn top_level_resources_have_conversion_traits() {
    fn check_conversion<T, I>()
    where
        T: ToInternal<I> + FromInternal<I>,
    {
    }

    check_conversion::<PersistentVolume, internal::PersistentVolume>();
    check_conversion::<PersistentVolumeClaim, internal::PersistentVolumeClaim>();
}

#[test]
fn top_level_resources_implement_prost_message() {
    fn check_prost<T: prost::Message>() {}

    check_prost::<PersistentVolume>();
    check_prost::<PersistentVolumeList>();
    check_prost::<PersistentVolumeClaim>();
    check_prost::<PersistentVolumeClaimList>();
}

#[test]
fn versioned_object_metadata_access() {
    let resource = PersistentVolume::default();
    let meta = resource.metadata();
    assert!(meta.name.is_none());

    let mut resource = PersistentVolume::default();
    resource.metadata_mut().name = Some("test".to_string());
    assert_eq!(resource.metadata().name.as_deref(), Some("test"));
}

#[test]
fn apply_default_sets_type_meta() {
    let mut resource = PersistentVolume::default();
    resource.apply_default();
    let group = <PersistentVolume as ResourceSchema>::group_static();
    let version = <PersistentVolume as ResourceSchema>::version_static();
    let expected_api_version = if group.is_empty() {
        version.to_string()
    } else {
        format!("{}/{}", group, version)
    };
    assert_eq!(resource.type_meta.api_version, expected_api_version);
    assert_eq!(
        resource.type_meta.kind,
        <PersistentVolume as ResourceSchema>::kind_static()
    );
}
