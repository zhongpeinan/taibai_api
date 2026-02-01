use super::{
    CSIDriver, CSIDriverList, CSIDriverSpec, CSINode, CSINodeDriver, CSINodeList, CSINodeSpec,
    CSIStorageCapacity, CSIStorageCapacityList, FSGroupPolicy, StorageClass, StorageClassList,
    TokenRequest, VolumeAttachment, VolumeAttachmentList, VolumeAttachmentSource,
    VolumeAttachmentSpec, VolumeAttachmentStatus, VolumeAttributesClass, VolumeAttributesClassList,
    VolumeBindingMode, VolumeError, VolumeLifecycleMode, VolumeNodeResources,
};
use crate::common::test_utils::assert_serde_roundtrip;
use crate::common::util::Quantity;
use crate::common::{
    LabelSelector, ListMeta, ObjectMeta, PersistentVolumeReclaimPolicy, Timestamp,
    TopologySelectorLabelRequirement, TopologySelectorTerm, TypeMeta,
};
use std::collections::BTreeMap;

fn storage_class_basic() -> StorageClass {
    StorageClass {
        type_meta: TypeMeta {
            api_version: "storage.k8s.io/v1".to_string(),
            kind: "StorageClass".to_string(),
        },
        metadata: Some(ObjectMeta {
            name: Some("fast".to_string()),
            ..Default::default()
        }),
        provisioner: "csi.example.com".to_string(),
        parameters: BTreeMap::from([("type".to_string(), "ssd".to_string())]),
        reclaim_policy: Some(PersistentVolumeReclaimPolicy::Retain),
        mount_options: vec!["discard".to_string()],
        allow_volume_expansion: Some(true),
        volume_binding_mode: Some(VolumeBindingMode::WaitForFirstConsumer),
        allowed_topologies: vec![TopologySelectorTerm {
            match_label_expressions: vec![TopologySelectorLabelRequirement {
                key: "topology.kubernetes.io/zone".to_string(),
                values: vec!["us-east-1a".to_string()],
            }],
        }],
    }
}

fn storage_class_list_basic() -> StorageClassList {
    StorageClassList {
        type_meta: TypeMeta {
            api_version: "storage.k8s.io/v1".to_string(),
            kind: "StorageClassList".to_string(),
        },
        metadata: Some(ListMeta {
            resource_version: Some("1".to_string()),
            ..Default::default()
        }),
        items: vec![storage_class_basic()],
    }
}

fn csi_driver_basic() -> CSIDriver {
    CSIDriver {
        type_meta: TypeMeta {
            api_version: "storage.k8s.io/v1".to_string(),
            kind: "CSIDriver".to_string(),
        },
        metadata: Some(ObjectMeta {
            name: Some("csi.example.com".to_string()),
            ..Default::default()
        }),
        spec: CSIDriverSpec {
            attach_required: Some(true),
            pod_info_on_mount: Some(false),
            volume_lifecycle_modes: vec![VolumeLifecycleMode::Persistent],
            storage_capacity: Some(true),
            fs_group_policy: Some(FSGroupPolicy::File),
            token_requests: vec![TokenRequest {
                audience: "sts".to_string(),
                expiration_seconds: Some(3600),
            }],
            requires_republish: Some(false),
            se_linux_mount: Some(true),
            node_allocatable_update_period_seconds: Some(60),
            service_account_token_in_secrets: Some(true),
        },
    }
}

fn csi_driver_list_basic() -> CSIDriverList {
    CSIDriverList {
        type_meta: TypeMeta {
            api_version: "storage.k8s.io/v1".to_string(),
            kind: "CSIDriverList".to_string(),
        },
        metadata: Some(ListMeta {
            resource_version: Some("2".to_string()),
            ..Default::default()
        }),
        items: vec![csi_driver_basic()],
    }
}

fn csi_node_basic() -> CSINode {
    CSINode {
        type_meta: TypeMeta {
            api_version: "storage.k8s.io/v1".to_string(),
            kind: "CSINode".to_string(),
        },
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
    CSINodeList {
        type_meta: TypeMeta {
            api_version: "storage.k8s.io/v1".to_string(),
            kind: "CSINodeList".to_string(),
        },
        metadata: Some(ListMeta {
            resource_version: Some("3".to_string()),
            ..Default::default()
        }),
        items: vec![csi_node_basic()],
    }
}

fn csi_storage_capacity_basic() -> CSIStorageCapacity {
    CSIStorageCapacity {
        type_meta: TypeMeta {
            api_version: "storage.k8s.io/v1".to_string(),
            kind: "CSIStorageCapacity".to_string(),
        },
        metadata: Some(ObjectMeta {
            name: Some("capacity-a".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        node_topology: Some(LabelSelector {
            match_labels: BTreeMap::from([(
                "topology.kubernetes.io/zone".to_string(),
                "us-east-1a".to_string(),
            )]),
            ..Default::default()
        }),
        storage_class_name: "fast".to_string(),
        capacity: Some(Quantity("10Gi".to_string())),
        maximum_volume_size: Some(Quantity("1Ti".to_string())),
    }
}

fn csi_storage_capacity_list_basic() -> CSIStorageCapacityList {
    CSIStorageCapacityList {
        type_meta: TypeMeta {
            api_version: "storage.k8s.io/v1".to_string(),
            kind: "CSIStorageCapacityList".to_string(),
        },
        metadata: Some(ListMeta {
            resource_version: Some("4".to_string()),
            ..Default::default()
        }),
        items: vec![csi_storage_capacity_basic()],
    }
}

fn volume_attachment_basic() -> VolumeAttachment {
    VolumeAttachment {
        type_meta: TypeMeta {
            api_version: "storage.k8s.io/v1".to_string(),
            kind: "VolumeAttachment".to_string(),
        },
        metadata: Some(ObjectMeta {
            name: Some("attach-a".to_string()),
            ..Default::default()
        }),
        spec: VolumeAttachmentSpec {
            attacher: "csi.example.com".to_string(),
            source: VolumeAttachmentSource {
                persistent_volume_name: Some("pv-a".to_string()),
                inline_volume_spec: None,
            },
            node_name: "node-1".to_string(),
        },
        status: Some(VolumeAttachmentStatus {
            attached: true,
            attachment_metadata: BTreeMap::from([(
                "devicePath".to_string(),
                "/dev/sdb".to_string(),
            )]),
            attach_error: Some(VolumeError {
                time: Some(Timestamp::from_str("2024-01-01T00:00:00Z").unwrap()),
                message: "attach failed".to_string(),
                error_code: Some(5),
            }),
            detach_error: None,
        }),
    }
}

fn volume_attachment_list_basic() -> VolumeAttachmentList {
    VolumeAttachmentList {
        type_meta: TypeMeta {
            api_version: "storage.k8s.io/v1".to_string(),
            kind: "VolumeAttachmentList".to_string(),
        },
        metadata: Some(ListMeta {
            resource_version: Some("5".to_string()),
            ..Default::default()
        }),
        items: vec![volume_attachment_basic()],
    }
}

fn volume_attributes_class_basic() -> VolumeAttributesClass {
    VolumeAttributesClass {
        type_meta: TypeMeta {
            api_version: "storage.k8s.io/v1".to_string(),
            kind: "VolumeAttributesClass".to_string(),
        },
        metadata: Some(ObjectMeta {
            name: Some("attrs-a".to_string()),
            ..Default::default()
        }),
        driver_name: "csi.example.com".to_string(),
        parameters: BTreeMap::from([("iops".to_string(), "1000".to_string())]),
    }
}

fn volume_attributes_class_list_basic() -> VolumeAttributesClassList {
    VolumeAttributesClassList {
        type_meta: TypeMeta {
            api_version: "storage.k8s.io/v1".to_string(),
            kind: "VolumeAttributesClassList".to_string(),
        },
        metadata: Some(ListMeta {
            resource_version: Some("6".to_string()),
            ..Default::default()
        }),
        items: vec![volume_attributes_class_basic()],
    }
}

#[test]
fn serde_roundtrip_storage_class() {
    assert_serde_roundtrip(&storage_class_basic());
}

#[test]
fn serde_roundtrip_storage_class_list() {
    assert_serde_roundtrip(&storage_class_list_basic());
}

#[test]
fn serde_roundtrip_csi_driver() {
    assert_serde_roundtrip(&csi_driver_basic());
}

#[test]
fn serde_roundtrip_csi_driver_list() {
    assert_serde_roundtrip(&csi_driver_list_basic());
}

#[test]
fn serde_roundtrip_csi_node() {
    assert_serde_roundtrip(&csi_node_basic());
}

#[test]
fn serde_roundtrip_csi_node_list() {
    assert_serde_roundtrip(&csi_node_list_basic());
}

#[test]
fn serde_roundtrip_csi_storage_capacity() {
    assert_serde_roundtrip(&csi_storage_capacity_basic());
}

#[test]
fn serde_roundtrip_csi_storage_capacity_list() {
    assert_serde_roundtrip(&csi_storage_capacity_list_basic());
}

#[test]
fn serde_roundtrip_volume_attachment() {
    assert_serde_roundtrip(&volume_attachment_basic());
}

#[test]
fn serde_roundtrip_volume_attachment_list() {
    assert_serde_roundtrip(&volume_attachment_list_basic());
}

#[test]
fn serde_roundtrip_volume_attributes_class() {
    assert_serde_roundtrip(&volume_attributes_class_basic());
}

#[test]
fn serde_roundtrip_volume_attributes_class_list() {
    assert_serde_roundtrip(&volume_attributes_class_list_basic());
}
