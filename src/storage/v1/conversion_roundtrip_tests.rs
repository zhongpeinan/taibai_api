use super::{
    CSIDriver, CSIDriverList, CSIDriverSpec, CSINode, CSINodeDriver, CSINodeList, CSINodeSpec,
    CSIStorageCapacity, CSIStorageCapacityList, StorageClass, StorageClassList, TokenRequest,
    VolumeAttachment, VolumeAttachmentList, VolumeAttachmentSource, VolumeAttachmentSpec,
    VolumeAttributesClass, VolumeAttributesClassList, VolumeBindingMode, VolumeNodeResources,
};
use crate::common::test_utils::assert_conversion_roundtrip;
use crate::common::util::Quantity;
use crate::common::{
    ApplyDefault, LabelSelector, ListMeta, ObjectMeta, PersistentVolumeReclaimPolicy, TypeMeta,
};
use crate::storage::internal;
use std::collections::BTreeMap;

fn storage_class_basic() -> StorageClass {
    StorageClass {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("standard".to_string()),
            ..Default::default()
        }),
        provisioner: "csi.example.com".to_string(),
        parameters: BTreeMap::from([("type".to_string(), "ssd".to_string())]),
        reclaim_policy: Some(PersistentVolumeReclaimPolicy::Delete),
        mount_options: vec!["discard".to_string()],
        allow_volume_expansion: Some(true),
        volume_binding_mode: Some(VolumeBindingMode::Immediate),
        allowed_topologies: vec![],
    }
}

fn storage_class_list_basic() -> StorageClassList {
    let mut item = storage_class_basic();
    item.apply_default();

    StorageClassList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("1".to_string()),
            ..Default::default()
        }),
        items: vec![item],
    }
}

fn csi_driver_basic() -> CSIDriver {
    CSIDriver {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("csi.example.com".to_string()),
            ..Default::default()
        }),
        spec: CSIDriverSpec {
            attach_required: Some(true),
            pod_info_on_mount: Some(false),
            storage_capacity: Some(true),
            fs_group_policy: None,
            volume_lifecycle_modes: vec![],
            token_requests: vec![TokenRequest {
                audience: "sts".to_string(),
                expiration_seconds: Some(3600),
            }],
            requires_republish: Some(false),
            se_linux_mount: Some(false),
            node_allocatable_update_period_seconds: Some(30),
            service_account_token_in_secrets: Some(true),
        },
    }
}

fn csi_driver_list_basic() -> CSIDriverList {
    let mut item = csi_driver_basic();
    item.apply_default();

    CSIDriverList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("2".to_string()),
            ..Default::default()
        }),
        items: vec![item],
    }
}

fn csi_node_basic() -> CSINode {
    CSINode {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("node-1".to_string()),
            ..Default::default()
        }),
        spec: CSINodeSpec {
            drivers: vec![CSINodeDriver {
                name: "csi.example.com".to_string(),
                node_id: "node-1".to_string(),
                topology_keys: vec!["topology.kubernetes.io/zone".to_string()],
                allocatable: Some(VolumeNodeResources { count: Some(10) }),
            }],
        },
    }
}

fn csi_node_list_basic() -> CSINodeList {
    let mut item = csi_node_basic();
    item.apply_default();

    CSINodeList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("3".to_string()),
            ..Default::default()
        }),
        items: vec![item],
    }
}

fn csi_storage_capacity_basic() -> CSIStorageCapacity {
    CSIStorageCapacity {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("capacity-b".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        node_topology: Some(LabelSelector {
            match_labels: BTreeMap::from([(
                "topology.kubernetes.io/zone".to_string(),
                "us-east-1b".to_string(),
            )]),
            ..Default::default()
        }),
        storage_class_name: "fast".to_string(),
        capacity: Some(Quantity("10Gi".to_string())),
        maximum_volume_size: Some(Quantity("1Ti".to_string())),
    }
}

fn csi_storage_capacity_list_basic() -> CSIStorageCapacityList {
    let mut item = csi_storage_capacity_basic();
    item.apply_default();

    CSIStorageCapacityList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("4".to_string()),
            ..Default::default()
        }),
        items: vec![item],
    }
}

fn volume_attachment_basic() -> VolumeAttachment {
    VolumeAttachment {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("attach-b".to_string()),
            ..Default::default()
        }),
        spec: VolumeAttachmentSpec {
            attacher: "csi.example.com".to_string(),
            source: VolumeAttachmentSource {
                persistent_volume_name: Some("pv-b".to_string()),
                inline_volume_spec: None,
            },
            node_name: "node-1".to_string(),
        },
        status: None,
    }
}

fn volume_attachment_list_basic() -> VolumeAttachmentList {
    let mut item = volume_attachment_basic();
    item.apply_default();

    VolumeAttachmentList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("5".to_string()),
            ..Default::default()
        }),
        items: vec![item],
    }
}

fn volume_attributes_class_basic() -> VolumeAttributesClass {
    VolumeAttributesClass {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("attrs-b".to_string()),
            ..Default::default()
        }),
        driver_name: "csi.example.com".to_string(),
        parameters: BTreeMap::from([("iops".to_string(), "2000".to_string())]),
    }
}

fn volume_attributes_class_list_basic() -> VolumeAttributesClassList {
    let mut item = volume_attributes_class_basic();
    item.apply_default();

    VolumeAttributesClassList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("6".to_string()),
            ..Default::default()
        }),
        items: vec![item],
    }
}

#[test]
fn conversion_roundtrip_storage_class() {
    assert_conversion_roundtrip::<StorageClass, internal::StorageClass>(storage_class_basic());
}

#[test]
fn conversion_roundtrip_storage_class_list() {
    assert_conversion_roundtrip::<StorageClassList, internal::StorageClassList>(
        storage_class_list_basic(),
    );
}

#[test]
fn conversion_roundtrip_csi_driver() {
    assert_conversion_roundtrip::<CSIDriver, internal::CSIDriver>(csi_driver_basic());
}

#[test]
fn conversion_roundtrip_csi_driver_list() {
    assert_conversion_roundtrip::<CSIDriverList, internal::CSIDriverList>(csi_driver_list_basic());
}

#[test]
fn conversion_roundtrip_csi_node() {
    assert_conversion_roundtrip::<CSINode, internal::CSINode>(csi_node_basic());
}

#[test]
fn conversion_roundtrip_csi_node_list() {
    assert_conversion_roundtrip::<CSINodeList, internal::CSINodeList>(csi_node_list_basic());
}

#[test]
fn conversion_roundtrip_csi_storage_capacity() {
    assert_conversion_roundtrip::<CSIStorageCapacity, internal::CSIStorageCapacity>(
        csi_storage_capacity_basic(),
    );
}

#[test]
fn conversion_roundtrip_csi_storage_capacity_list() {
    assert_conversion_roundtrip::<CSIStorageCapacityList, internal::CSIStorageCapacityList>(
        csi_storage_capacity_list_basic(),
    );
}

#[test]
fn conversion_roundtrip_volume_attachment() {
    assert_conversion_roundtrip::<VolumeAttachment, internal::VolumeAttachment>(
        volume_attachment_basic(),
    );
}

#[test]
fn conversion_roundtrip_volume_attachment_list() {
    assert_conversion_roundtrip::<VolumeAttachmentList, internal::VolumeAttachmentList>(
        volume_attachment_list_basic(),
    );
}

#[test]
fn conversion_roundtrip_volume_attributes_class() {
    assert_conversion_roundtrip::<VolumeAttributesClass, internal::VolumeAttributesClass>(
        volume_attributes_class_basic(),
    );
}

#[test]
fn conversion_roundtrip_volume_attributes_class_list() {
    assert_conversion_roundtrip::<VolumeAttributesClassList, internal::VolumeAttributesClassList>(
        volume_attributes_class_list_basic(),
    );
}
