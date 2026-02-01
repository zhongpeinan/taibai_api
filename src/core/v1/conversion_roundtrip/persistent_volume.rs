use crate::common::test_utils::assert_conversion_roundtrip;
use crate::common::{ApplyDefault, ListMeta, ObjectMeta, Quantity, TypeMeta};
use crate::core::internal;
use crate::core::v1::volume::LocalVolumeSource;
use crate::core::v1::{
    PersistentVolume, PersistentVolumeClaim, PersistentVolumeClaimList, PersistentVolumeClaimSpec,
    PersistentVolumeClaimStatus, PersistentVolumeList, PersistentVolumeSource,
    PersistentVolumeSpec, PersistentVolumeStatus, VolumeResourceRequirements,
    persistent_volume_access_mode,
};
use std::collections::BTreeMap;

fn persistent_volume_basic() -> PersistentVolume {
    PersistentVolume {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("pv-data".to_string()),
            ..Default::default()
        }),
        spec: Some(PersistentVolumeSpec {
            capacity: BTreeMap::from([("storage".to_string(), Quantity("10Gi".to_string()))]),
            access_modes: vec![persistent_volume_access_mode::READ_WRITE_ONCE.to_string()],
            persistent_volume_source: Some(PersistentVolumeSource {
                local: Some(LocalVolumeSource {
                    path: "/data".to_string(),
                    fs_type: Some("ext4".to_string()),
                }),
                ..Default::default()
            }),
            storage_class_name: Some("standard".to_string()),
            ..Default::default()
        }),
        status: Some(PersistentVolumeStatus {
            phase: Some("Available".to_string()),
            ..Default::default()
        }),
    }
}

fn persistent_volume_list_basic() -> PersistentVolumeList {
    let mut item = persistent_volume_basic();
    item.apply_default();
    PersistentVolumeList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("8".to_string()),
            ..Default::default()
        }),
        items: vec![item],
    }
}

fn persistent_volume_claim_basic() -> PersistentVolumeClaim {
    PersistentVolumeClaim {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("data-claim".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        spec: Some(PersistentVolumeClaimSpec {
            access_modes: vec![persistent_volume_access_mode::READ_WRITE_ONCE.to_string()],
            resources: Some(VolumeResourceRequirements {
                requests: BTreeMap::from([("storage".to_string(), Quantity("1Gi".to_string()))]),
                ..Default::default()
            }),
            storage_class_name: Some("standard".to_string()),
            ..Default::default()
        }),
        status: Some(PersistentVolumeClaimStatus {
            phase: Some("Bound".to_string()),
            ..Default::default()
        }),
    }
}

fn persistent_volume_claim_list_basic() -> PersistentVolumeClaimList {
    let mut item = persistent_volume_claim_basic();
    item.apply_default();
    PersistentVolumeClaimList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("9".to_string()),
            ..Default::default()
        }),
        items: vec![item],
    }
}

#[test]
fn conversion_roundtrip_persistent_volume() {
    assert_conversion_roundtrip::<PersistentVolume, internal::persistent_volume::PersistentVolume>(
        persistent_volume_basic(),
    );
}

#[test]
fn conversion_roundtrip_persistent_volume_list() {
    assert_conversion_roundtrip::<
        PersistentVolumeList,
        internal::persistent_volume::PersistentVolumeList,
    >(persistent_volume_list_basic());
}

#[test]
fn conversion_roundtrip_persistent_volume_claim() {
    assert_conversion_roundtrip::<
        PersistentVolumeClaim,
        internal::persistent_volume::PersistentVolumeClaim,
    >(persistent_volume_claim_basic());
}

#[test]
fn conversion_roundtrip_persistent_volume_claim_list() {
    assert_conversion_roundtrip::<
        PersistentVolumeClaimList,
        internal::persistent_volume::PersistentVolumeClaimList,
    >(persistent_volume_claim_list_basic());
}
