//! Conversions between storage v1alpha1 and internal types
//!
//! Based on k8s.io/kubernetes/pkg/apis/storage/v1alpha1/conversion.go

#[allow(unused_imports)]
use crate::common::{ApplyDefault, FromInternal, ToInternal, TypeMeta};
use crate::storage::internal;
use crate::storage::v1alpha1::{
    CSIStorageCapacity, CSIStorageCapacityList, VolumeAttachment, VolumeAttachmentList,
    VolumeAttachmentSource, VolumeAttachmentSpec, VolumeAttachmentStatus, VolumeAttributesClass,
    VolumeAttributesClassList, VolumeError,
};

// ============================================================================
// Spec Conversion Helpers
// ============================================================================

fn convert_volume_attachment_spec_v1alpha1_to_internal(
    spec: VolumeAttachmentSpec,
) -> internal::VolumeAttachmentSpec {
    internal::VolumeAttachmentSpec {
        attacher: spec.attacher,
        source: convert_volume_attachment_source_v1alpha1_to_internal(spec.source),
        node_name: spec.node_name,
    }
}

fn convert_volume_attachment_spec_internal_to_v1alpha1(
    spec: internal::VolumeAttachmentSpec,
) -> VolumeAttachmentSpec {
    VolumeAttachmentSpec {
        attacher: spec.attacher,
        source: convert_volume_attachment_source_internal_to_v1alpha1(spec.source),
        node_name: spec.node_name,
    }
}

fn convert_volume_attachment_source_v1alpha1_to_internal(
    source: VolumeAttachmentSource,
) -> internal::VolumeAttachmentSource {
    internal::VolumeAttachmentSource {
        persistent_volume_name: source.persistent_volume_name,
        inline_volume_spec: source.inline_volume_spec,
    }
}

fn convert_volume_attachment_source_internal_to_v1alpha1(
    source: internal::VolumeAttachmentSource,
) -> VolumeAttachmentSource {
    VolumeAttachmentSource {
        persistent_volume_name: source.persistent_volume_name,
        inline_volume_spec: source.inline_volume_spec,
    }
}

fn convert_volume_attachment_status_v1alpha1_to_internal(
    status: VolumeAttachmentStatus,
) -> internal::VolumeAttachmentStatus {
    internal::VolumeAttachmentStatus {
        attached: status.attached,
        attachment_metadata: status.attachment_metadata,
        attach_error: status
            .attach_error
            .map(convert_volume_error_v1alpha1_to_internal),
        detach_error: status
            .detach_error
            .map(convert_volume_error_v1alpha1_to_internal),
    }
}

fn convert_volume_attachment_status_internal_to_v1alpha1(
    status: internal::VolumeAttachmentStatus,
) -> VolumeAttachmentStatus {
    VolumeAttachmentStatus {
        attached: status.attached,
        attachment_metadata: status.attachment_metadata,
        attach_error: status
            .attach_error
            .map(convert_volume_error_internal_to_v1alpha1),
        detach_error: status
            .detach_error
            .map(convert_volume_error_internal_to_v1alpha1),
    }
}

fn convert_volume_error_v1alpha1_to_internal(error: VolumeError) -> internal::VolumeError {
    internal::VolumeError {
        time: error.time,
        message: error.message,
        error_code: error.error_code,
    }
}

fn convert_volume_error_internal_to_v1alpha1(error: internal::VolumeError) -> VolumeError {
    VolumeError {
        time: error.time,
        message: error.message,
        error_code: error.error_code,
    }
}

// ============================================================================
// VolumeAttachment Conversions
// ============================================================================

impl ToInternal<internal::VolumeAttachment> for VolumeAttachment {
    fn to_internal(self) -> internal::VolumeAttachment {
        internal::VolumeAttachment {
            type_meta: TypeMeta::default(),
            metadata: self.metadata,
            spec: convert_volume_attachment_spec_v1alpha1_to_internal(self.spec),
            status: self
                .status
                .map(convert_volume_attachment_status_v1alpha1_to_internal),
        }
    }
}

impl FromInternal<internal::VolumeAttachment> for VolumeAttachment {
    fn from_internal(value: internal::VolumeAttachment) -> Self {
        let result = Self {
            type_meta: TypeMeta::default(),
            metadata: value.metadata,
            spec: convert_volume_attachment_spec_internal_to_v1alpha1(value.spec),
            status: value
                .status
                .map(convert_volume_attachment_status_internal_to_v1alpha1),
        };

        result
    }
}

impl ToInternal<internal::VolumeAttachmentList> for VolumeAttachmentList {
    fn to_internal(self) -> internal::VolumeAttachmentList {
        internal::VolumeAttachmentList {
            type_meta: TypeMeta::default(),
            metadata: self.metadata,
            items: self
                .items
                .into_iter()
                .map(|item| item.to_internal())
                .collect(),
        }
    }
}

impl FromInternal<internal::VolumeAttachmentList> for VolumeAttachmentList {
    fn from_internal(value: internal::VolumeAttachmentList) -> Self {
        let result = Self {
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
    fn to_internal(self) -> internal::CSIStorageCapacity {
        internal::CSIStorageCapacity {
            type_meta: TypeMeta::default(),
            metadata: self.metadata,
            node_topology: self.node_topology,
            storage_class_name: self.storage_class_name,
            capacity: self.capacity,
            maximum_volume_size: self.maximum_volume_size,
        }
    }
}

impl FromInternal<internal::CSIStorageCapacity> for CSIStorageCapacity {
    fn from_internal(value: internal::CSIStorageCapacity) -> Self {
        let result = Self {
            type_meta: TypeMeta::default(),
            metadata: value.metadata,
            node_topology: value.node_topology,
            storage_class_name: value.storage_class_name,
            capacity: value.capacity,
            maximum_volume_size: value.maximum_volume_size,
        };

        result
    }
}

impl ToInternal<internal::CSIStorageCapacityList> for CSIStorageCapacityList {
    fn to_internal(self) -> internal::CSIStorageCapacityList {
        internal::CSIStorageCapacityList {
            type_meta: TypeMeta::default(),
            metadata: self.metadata,
            items: self
                .items
                .into_iter()
                .map(|item| item.to_internal())
                .collect(),
        }
    }
}

impl FromInternal<internal::CSIStorageCapacityList> for CSIStorageCapacityList {
    fn from_internal(value: internal::CSIStorageCapacityList) -> Self {
        let result = Self {
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
    fn to_internal(self) -> internal::VolumeAttributesClass {
        internal::VolumeAttributesClass {
            type_meta: TypeMeta::default(),
            metadata: self.metadata,
            driver_name: self.driver_name,
            parameters: self.parameters,
        }
    }
}

impl FromInternal<internal::VolumeAttributesClass> for VolumeAttributesClass {
    fn from_internal(value: internal::VolumeAttributesClass) -> Self {
        let result = Self {
            type_meta: TypeMeta::default(),
            metadata: value.metadata,
            driver_name: value.driver_name,
            parameters: value.parameters,
        };

        result
    }
}

impl ToInternal<internal::VolumeAttributesClassList> for VolumeAttributesClassList {
    fn to_internal(self) -> internal::VolumeAttributesClassList {
        internal::VolumeAttributesClassList {
            type_meta: TypeMeta::default(),
            metadata: self.metadata,
            items: self
                .items
                .into_iter()
                .map(|item| item.to_internal())
                .collect(),
        }
    }
}

impl FromInternal<internal::VolumeAttributesClassList> for VolumeAttributesClassList {
    fn from_internal(value: internal::VolumeAttributesClassList) -> Self {
        let result = Self {
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
    use crate::common::{ObjectMeta, TypeMeta};

    #[test]
    fn test_volume_attachment_round_trip() {
        let original = VolumeAttachment {
            type_meta: TypeMeta {
                api_version: "storage.k8s.io/v1alpha1".to_string(),
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
        assert_eq!(round_trip.type_meta.api_version, "storage.k8s.io/v1alpha1");
    }

    #[test]
    fn test_csi_storage_capacity_round_trip() {
        let original = CSIStorageCapacity {
            type_meta: TypeMeta {
                api_version: "storage.k8s.io/v1alpha1".to_string(),
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
        assert_eq!(round_trip.type_meta.api_version, "storage.k8s.io/v1alpha1");
    }
}
