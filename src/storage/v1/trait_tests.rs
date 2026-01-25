use super::*;
use crate::common::{ApplyDefault, FromInternal, HasObjectMeta, ToInternal, VersionedObject};
use crate::storage::internal;

#[test]
fn top_level_resources_implement_required_traits() {
    fn check_versioned<T: VersionedObject + ApplyDefault>() {}

    check_versioned::<StorageClass>();
    check_versioned::<VolumeAttachment>();
    check_versioned::<CSIDriver>();
    check_versioned::<CSINode>();
    check_versioned::<CSIStorageCapacity>();
    check_versioned::<VolumeAttributesClass>();
}

#[test]
fn top_level_resources_have_conversion_traits() {
    fn check_conversion<T, I>()
    where
        T: ToInternal<I> + FromInternal<I>,
    {
    }

    check_conversion::<StorageClass, internal::StorageClass>();
    check_conversion::<StorageClassList, internal::StorageClassList>();
    check_conversion::<VolumeAttachment, internal::VolumeAttachment>();
    check_conversion::<VolumeAttachmentList, internal::VolumeAttachmentList>();
    check_conversion::<CSIDriver, internal::CSIDriver>();
    check_conversion::<CSIDriverList, internal::CSIDriverList>();
    check_conversion::<CSINode, internal::CSINode>();
    check_conversion::<CSINodeList, internal::CSINodeList>();
    check_conversion::<CSIStorageCapacity, internal::CSIStorageCapacity>();
    check_conversion::<CSIStorageCapacityList, internal::CSIStorageCapacityList>();
    check_conversion::<VolumeAttributesClass, internal::VolumeAttributesClass>();
    check_conversion::<VolumeAttributesClassList, internal::VolumeAttributesClassList>();
}

#[test]
fn top_level_resources_implement_prost_message() {
    fn check_prost<T: prost::Message>() {}

    check_prost::<StorageClass>();
    check_prost::<StorageClassList>();
    check_prost::<VolumeAttachment>();
    check_prost::<VolumeAttachmentList>();
    check_prost::<CSIDriver>();
    check_prost::<CSIDriverList>();
    check_prost::<CSINode>();
    check_prost::<CSINodeList>();
    check_prost::<CSIStorageCapacity>();
    check_prost::<CSIStorageCapacityList>();
    check_prost::<VolumeAttributesClass>();
    check_prost::<VolumeAttributesClassList>();
}

#[test]
fn versioned_object_metadata_access() {
    let resource = StorageClass::default();
    let meta = resource.metadata();
    assert!(meta.name.is_none());

    let mut resource = StorageClass::default();
    resource.metadata_mut().name = Some("storage-class".to_string());
    assert_eq!(resource.metadata().name.as_deref(), Some("storage-class"));
}

#[test]
fn apply_default_sets_type_meta() {
    let mut resource = StorageClass::default();
    resource.apply_default();
    assert_eq!(resource.type_meta.api_version, "storage.k8s.io/v1");
    assert_eq!(resource.type_meta.kind, "StorageClass");
}

#[test]
fn internal_resources_implement_required_traits() {
    fn check<T: HasObjectMeta>() {}

    check::<internal::StorageClass>();
    check::<internal::VolumeAttachment>();
    check::<internal::CSIDriver>();
    check::<internal::CSINode>();
    check::<internal::CSIStorageCapacity>();
    check::<internal::VolumeAttributesClass>();
}
