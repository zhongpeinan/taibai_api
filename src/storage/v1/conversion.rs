//! Conversions between storage v1 and internal types
//!
//! Based on k8s.io/kubernetes/pkg/apis/storage/v1/conversion.go

use crate::common::{FromInternal, ToInternal, TypeMeta};
use crate::storage::internal;
use crate::storage::v1::{
    CSIDriver, CSIDriverList, CSINode, CSINodeList, CSIStorageCapacity, CSIStorageCapacityList,
    StorageClass, StorageClassList, VolumeAttachment, VolumeAttachmentList, VolumeAttributesClass,
    VolumeAttributesClassList,
};

// ============================================================================
// StorageClass Conversions
// ============================================================================

impl ToInternal<internal::StorageClass> for StorageClass {
    fn to_internal(mut self) -> internal::StorageClass {
        self.type_meta = TypeMeta::default();
        self
    }
}

impl FromInternal<internal::StorageClass> for StorageClass {
    fn from_internal(mut value: internal::StorageClass) -> Self {
        value.type_meta = TypeMeta::default();
        value
    }
}

impl ToInternal<internal::StorageClassList> for StorageClassList {
    fn to_internal(mut self) -> internal::StorageClassList {
        self.type_meta = TypeMeta::default();
        self.items = self
            .items
            .into_iter()
            .map(|item| item.to_internal())
            .collect();
        self
    }
}

impl FromInternal<internal::StorageClassList> for StorageClassList {
    fn from_internal(value: internal::StorageClassList) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: value.metadata,
            items: value
                .items
                .into_iter()
                .map(StorageClass::from_internal)
                .collect(),
        };

        result
    }
}

// ============================================================================
// CSIDriver Conversions
// ============================================================================

impl ToInternal<internal::CSIDriver> for CSIDriver {
    fn to_internal(mut self) -> internal::CSIDriver {
        self.type_meta = TypeMeta::default();
        self
    }
}

impl FromInternal<internal::CSIDriver> for CSIDriver {
    fn from_internal(mut value: internal::CSIDriver) -> Self {
        value.type_meta = TypeMeta::default();
        value
    }
}

impl ToInternal<internal::CSIDriverList> for CSIDriverList {
    fn to_internal(mut self) -> internal::CSIDriverList {
        self.type_meta = TypeMeta::default();
        self.items = self
            .items
            .into_iter()
            .map(|item| item.to_internal())
            .collect();
        self
    }
}

impl FromInternal<internal::CSIDriverList> for CSIDriverList {
    fn from_internal(value: internal::CSIDriverList) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: value.metadata,
            items: value
                .items
                .into_iter()
                .map(CSIDriver::from_internal)
                .collect(),
        };

        result
    }
}

// ============================================================================
// CSINode Conversions
// ============================================================================

impl ToInternal<internal::CSINode> for CSINode {
    fn to_internal(mut self) -> internal::CSINode {
        self.type_meta = TypeMeta::default();
        self
    }
}

impl FromInternal<internal::CSINode> for CSINode {
    fn from_internal(mut value: internal::CSINode) -> Self {
        value.type_meta = TypeMeta::default();
        value
    }
}

impl ToInternal<internal::CSINodeList> for CSINodeList {
    fn to_internal(mut self) -> internal::CSINodeList {
        self.type_meta = TypeMeta::default();
        self.items = self
            .items
            .into_iter()
            .map(|item| item.to_internal())
            .collect();
        self
    }
}

impl FromInternal<internal::CSINodeList> for CSINodeList {
    fn from_internal(value: internal::CSINodeList) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: value.metadata,
            items: value
                .items
                .into_iter()
                .map(CSINode::from_internal)
                .collect(),
        };

        result
    }
}

// ============================================================================
// VolumeAttachment Conversions
// ============================================================================

impl ToInternal<internal::VolumeAttachment> for VolumeAttachment {
    fn to_internal(mut self) -> internal::VolumeAttachment {
        self.type_meta = TypeMeta::default();
        self
    }
}

impl FromInternal<internal::VolumeAttachment> for VolumeAttachment {
    fn from_internal(mut value: internal::VolumeAttachment) -> Self {
        value.type_meta = TypeMeta::default();
        value
    }
}

impl ToInternal<internal::VolumeAttachmentList> for VolumeAttachmentList {
    fn to_internal(mut self) -> internal::VolumeAttachmentList {
        self.type_meta = TypeMeta::default();
        self.items = self
            .items
            .into_iter()
            .map(|item| item.to_internal())
            .collect();
        self
    }
}

impl FromInternal<internal::VolumeAttachmentList> for VolumeAttachmentList {
    fn from_internal(value: internal::VolumeAttachmentList) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: value.metadata,
            items: value
                .items
                .into_iter()
                .map(VolumeAttachment::from_internal)
                .collect(),
        };

        result
    }
}

// ============================================================================
// CSIStorageCapacity Conversions
// ============================================================================

impl ToInternal<internal::CSIStorageCapacity> for CSIStorageCapacity {
    fn to_internal(mut self) -> internal::CSIStorageCapacity {
        self.type_meta = TypeMeta::default();
        self
    }
}

impl FromInternal<internal::CSIStorageCapacity> for CSIStorageCapacity {
    fn from_internal(mut value: internal::CSIStorageCapacity) -> Self {
        value.type_meta = TypeMeta::default();
        value
    }
}

impl ToInternal<internal::CSIStorageCapacityList> for CSIStorageCapacityList {
    fn to_internal(mut self) -> internal::CSIStorageCapacityList {
        self.type_meta = TypeMeta::default();
        self.items = self
            .items
            .into_iter()
            .map(|item| item.to_internal())
            .collect();
        self
    }
}

impl FromInternal<internal::CSIStorageCapacityList> for CSIStorageCapacityList {
    fn from_internal(value: internal::CSIStorageCapacityList) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: value.metadata,
            items: value
                .items
                .into_iter()
                .map(CSIStorageCapacity::from_internal)
                .collect(),
        };

        result
    }
}

// ============================================================================
// VolumeAttributesClass Conversions
// ============================================================================

impl ToInternal<internal::VolumeAttributesClass> for VolumeAttributesClass {
    fn to_internal(mut self) -> internal::VolumeAttributesClass {
        self.type_meta = TypeMeta::default();
        self
    }
}

impl FromInternal<internal::VolumeAttributesClass> for VolumeAttributesClass {
    fn from_internal(mut value: internal::VolumeAttributesClass) -> Self {
        value.type_meta = TypeMeta::default();
        value
    }
}

impl ToInternal<internal::VolumeAttributesClassList> for VolumeAttributesClassList {
    fn to_internal(mut self) -> internal::VolumeAttributesClassList {
        self.type_meta = TypeMeta::default();
        self.items = self
            .items
            .into_iter()
            .map(|item| item.to_internal())
            .collect();
        self
    }
}

impl FromInternal<internal::VolumeAttributesClassList> for VolumeAttributesClassList {
    fn from_internal(value: internal::VolumeAttributesClassList) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: value.metadata,
            items: value
                .items
                .into_iter()
                .map(VolumeAttributesClass::from_internal)
                .collect(),
        };

        result
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{ApplyDefault, ObjectMeta, TypeMeta};

    #[test]
    fn test_storage_class_round_trip() {
        let original = StorageClass {
            type_meta: TypeMeta {
                api_version: "storage.k8s.io/v1".to_string(),
                kind: "StorageClass".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("fast".to_string()),
                ..Default::default()
            }),
            provisioner: "example.com/prov".to_string(),
            parameters: Default::default(),
            reclaim_policy: None,
            mount_options: vec![],
            allow_volume_expansion: None,
            volume_binding_mode: None,
            allowed_topologies: vec![],
        };

        let internal = original.clone().to_internal();
        let mut round_trip = StorageClass::from_internal(internal);
        round_trip.apply_default();

        assert_eq!(round_trip.metadata, original.metadata);
        assert_eq!(round_trip.type_meta.api_version, "storage.k8s.io/v1");
        assert_eq!(round_trip.type_meta.kind, "StorageClass");
    }

    #[test]
    fn test_csi_driver_round_trip() {
        let original = CSIDriver {
            type_meta: TypeMeta {
                api_version: "storage.k8s.io/v1".to_string(),
                kind: "CSIDriver".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("com.example.csi".to_string()),
                ..Default::default()
            }),
            spec: Default::default(),
        };

        let internal = original.clone().to_internal();
        let mut round_trip = CSIDriver::from_internal(internal);
        round_trip.apply_default();

        assert_eq!(round_trip.metadata, original.metadata);
        assert_eq!(round_trip.type_meta.api_version, "storage.k8s.io/v1");
    }

    #[test]
    fn test_volume_attachment_round_trip() {
        let original = VolumeAttachment {
            type_meta: TypeMeta {
                api_version: "storage.k8s.io/v1".to_string(),
                kind: "VolumeAttachment".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("attach".to_string()),
                ..Default::default()
            }),
            spec: Default::default(),
            status: None,
        };

        let internal = original.clone().to_internal();
        let mut round_trip = VolumeAttachment::from_internal(internal);
        round_trip.apply_default();

        assert_eq!(round_trip.metadata, original.metadata);
        assert_eq!(round_trip.type_meta.api_version, "storage.k8s.io/v1");
    }

    #[test]
    fn test_csi_node_round_trip() {
        let original = CSINode {
            type_meta: TypeMeta {
                api_version: "storage.k8s.io/v1".to_string(),
                kind: "CSINode".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("node-1".to_string()),
                ..Default::default()
            }),
            spec: Default::default(),
        };

        let internal = original.clone().to_internal();
        let mut round_trip = CSINode::from_internal(internal);
        round_trip.apply_default();

        assert_eq!(round_trip.metadata, original.metadata);
        assert_eq!(round_trip.type_meta.api_version, "storage.k8s.io/v1");
    }

    #[test]
    fn test_csi_storage_capacity_round_trip() {
        let original = CSIStorageCapacity {
            type_meta: TypeMeta {
                api_version: "storage.k8s.io/v1".to_string(),
                kind: "CSIStorageCapacity".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("cap".to_string()),
                ..Default::default()
            }),
            node_topology: None,
            storage_class_name: "fast".to_string(),
            capacity: None,
            maximum_volume_size: None,
        };

        let internal = original.clone().to_internal();
        let mut round_trip = CSIStorageCapacity::from_internal(internal);
        round_trip.apply_default();

        assert_eq!(round_trip.metadata, original.metadata);
        assert_eq!(round_trip.type_meta.api_version, "storage.k8s.io/v1");
    }

    #[test]
    fn test_volume_attributes_class_round_trip() {
        let original = VolumeAttributesClass {
            type_meta: TypeMeta {
                api_version: "storage.k8s.io/v1".to_string(),
                kind: "VolumeAttributesClass".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("attrs".to_string()),
                ..Default::default()
            }),
            driver_name: "example.com/driver".to_string(),
            parameters: Default::default(),
        };

        let internal = original.clone().to_internal();
        let mut round_trip = VolumeAttributesClass::from_internal(internal);
        round_trip.apply_default();

        assert_eq!(round_trip.metadata, original.metadata);
        assert_eq!(round_trip.type_meta.api_version, "storage.k8s.io/v1");
    }
}
