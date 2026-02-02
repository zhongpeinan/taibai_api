//! Conversions between storage v1beta1 and internal types
//!
//! Based on k8s.io/kubernetes/pkg/apis/storage/v1beta1/conversion.go

use crate::common::{ApplyDefault, FromInternal, ToInternal, TypeMeta};
use crate::storage::internal;
use crate::storage::v1beta1::{
    CSIDriver, CSIDriverList, CSIDriverSpec, CSINode, CSINodeDriver, CSINodeList, CSINodeSpec,
    CSIStorageCapacity, CSIStorageCapacityList, FSGroupPolicy, StorageClass, StorageClassList,
    TokenRequest, VolumeAttachment, VolumeAttachmentList, VolumeAttachmentSource,
    VolumeAttachmentSpec, VolumeAttachmentStatus, VolumeAttributesClass, VolumeAttributesClassList,
    VolumeBindingMode, VolumeError, VolumeLifecycleMode, VolumeNodeResources,
};

// ============================================================================
// Enum Conversion Helpers
// ============================================================================

fn convert_volume_binding_mode_v1beta1_to_internal(
    mode: VolumeBindingMode,
) -> internal::VolumeBindingMode {
    match mode {
        VolumeBindingMode::Immediate => internal::VolumeBindingMode::Immediate,
        VolumeBindingMode::WaitForFirstConsumer => {
            internal::VolumeBindingMode::WaitForFirstConsumer
        }
    }
}

fn convert_volume_binding_mode_internal_to_v1beta1(
    mode: internal::VolumeBindingMode,
) -> VolumeBindingMode {
    match mode {
        internal::VolumeBindingMode::Immediate => VolumeBindingMode::Immediate,
        internal::VolumeBindingMode::WaitForFirstConsumer => {
            VolumeBindingMode::WaitForFirstConsumer
        }
    }
}

fn convert_fs_group_policy_v1beta1_to_internal(policy: FSGroupPolicy) -> internal::FSGroupPolicy {
    match policy {
        FSGroupPolicy::ReadWriteOnceWithFSType => internal::FSGroupPolicy::ReadWriteOnceWithFSType,
        FSGroupPolicy::File => internal::FSGroupPolicy::File,
        FSGroupPolicy::None => internal::FSGroupPolicy::None,
    }
}

fn convert_fs_group_policy_internal_to_v1beta1(policy: internal::FSGroupPolicy) -> FSGroupPolicy {
    match policy {
        internal::FSGroupPolicy::ReadWriteOnceWithFSType => FSGroupPolicy::ReadWriteOnceWithFSType,
        internal::FSGroupPolicy::File => FSGroupPolicy::File,
        internal::FSGroupPolicy::None => FSGroupPolicy::None,
    }
}

fn convert_volume_lifecycle_mode_v1beta1_to_internal(
    mode: VolumeLifecycleMode,
) -> internal::VolumeLifecycleMode {
    match mode {
        VolumeLifecycleMode::Persistent => internal::VolumeLifecycleMode::Persistent,
        VolumeLifecycleMode::Ephemeral => internal::VolumeLifecycleMode::Ephemeral,
    }
}

fn convert_volume_lifecycle_mode_internal_to_v1beta1(
    mode: internal::VolumeLifecycleMode,
) -> VolumeLifecycleMode {
    match mode {
        internal::VolumeLifecycleMode::Persistent => VolumeLifecycleMode::Persistent,
        internal::VolumeLifecycleMode::Ephemeral => VolumeLifecycleMode::Ephemeral,
    }
}

// ============================================================================
// Spec Conversion Helpers
// ============================================================================

fn convert_token_request_v1beta1_to_internal(request: TokenRequest) -> internal::TokenRequest {
    internal::TokenRequest {
        audience: request.audience,
        expiration_seconds: request.expiration_seconds,
    }
}

fn convert_token_request_internal_to_v1beta1(request: internal::TokenRequest) -> TokenRequest {
    TokenRequest {
        audience: request.audience,
        expiration_seconds: request.expiration_seconds,
    }
}

fn convert_csi_driver_spec_v1beta1_to_internal(spec: CSIDriverSpec) -> internal::CSIDriverSpec {
    internal::CSIDriverSpec {
        attach_required: spec.attach_required,
        pod_info_on_mount: spec.pod_info_on_mount,
        volume_lifecycle_modes: spec
            .volume_lifecycle_modes
            .into_iter()
            .map(convert_volume_lifecycle_mode_v1beta1_to_internal)
            .collect(),
        storage_capacity: spec.storage_capacity,
        fs_group_policy: spec
            .fs_group_policy
            .map(convert_fs_group_policy_v1beta1_to_internal),
        token_requests: spec
            .token_requests
            .into_iter()
            .map(convert_token_request_v1beta1_to_internal)
            .collect(),
        requires_republish: spec.requires_republish,
        se_linux_mount: spec.se_linux_mount,
        node_allocatable_update_period_seconds: spec.node_allocatable_update_period_seconds,
        service_account_token_in_secrets: None,
    }
}

fn convert_csi_driver_spec_internal_to_v1beta1(spec: internal::CSIDriverSpec) -> CSIDriverSpec {
    CSIDriverSpec {
        attach_required: spec.attach_required,
        pod_info_on_mount: spec.pod_info_on_mount,
        volume_lifecycle_modes: spec
            .volume_lifecycle_modes
            .into_iter()
            .map(convert_volume_lifecycle_mode_internal_to_v1beta1)
            .collect(),
        storage_capacity: spec.storage_capacity,
        fs_group_policy: spec
            .fs_group_policy
            .map(convert_fs_group_policy_internal_to_v1beta1),
        token_requests: spec
            .token_requests
            .into_iter()
            .map(convert_token_request_internal_to_v1beta1)
            .collect(),
        requires_republish: spec.requires_republish,
        se_linux_mount: spec.se_linux_mount,
        node_allocatable_update_period_seconds: spec.node_allocatable_update_period_seconds,
    }
}

fn convert_csi_node_spec_v1beta1_to_internal(spec: CSINodeSpec) -> internal::CSINodeSpec {
    internal::CSINodeSpec {
        drivers: spec
            .drivers
            .into_iter()
            .map(convert_csi_node_driver_v1beta1_to_internal)
            .collect(),
    }
}

fn convert_csi_node_spec_internal_to_v1beta1(spec: internal::CSINodeSpec) -> CSINodeSpec {
    CSINodeSpec {
        drivers: spec
            .drivers
            .into_iter()
            .map(convert_csi_node_driver_internal_to_v1beta1)
            .collect(),
    }
}

fn convert_csi_node_driver_v1beta1_to_internal(driver: CSINodeDriver) -> internal::CSINodeDriver {
    internal::CSINodeDriver {
        name: driver.name,
        node_id: driver.node_id,
        topology_keys: driver.topology_keys,
        allocatable: driver
            .allocatable
            .map(convert_volume_node_resources_v1beta1_to_internal),
    }
}

fn convert_csi_node_driver_internal_to_v1beta1(driver: internal::CSINodeDriver) -> CSINodeDriver {
    CSINodeDriver {
        name: driver.name,
        node_id: driver.node_id,
        topology_keys: driver.topology_keys,
        allocatable: driver
            .allocatable
            .map(convert_volume_node_resources_internal_to_v1beta1),
    }
}

fn convert_volume_node_resources_v1beta1_to_internal(
    resources: VolumeNodeResources,
) -> internal::VolumeNodeResources {
    internal::VolumeNodeResources {
        count: resources.count,
    }
}

fn convert_volume_node_resources_internal_to_v1beta1(
    resources: internal::VolumeNodeResources,
) -> VolumeNodeResources {
    VolumeNodeResources {
        count: resources.count,
    }
}

fn convert_volume_attachment_spec_v1beta1_to_internal(
    spec: VolumeAttachmentSpec,
) -> internal::VolumeAttachmentSpec {
    internal::VolumeAttachmentSpec {
        attacher: spec.attacher,
        source: convert_volume_attachment_source_v1beta1_to_internal(spec.source),
        node_name: spec.node_name,
    }
}

fn convert_volume_attachment_spec_internal_to_v1beta1(
    spec: internal::VolumeAttachmentSpec,
) -> VolumeAttachmentSpec {
    VolumeAttachmentSpec {
        attacher: spec.attacher,
        source: convert_volume_attachment_source_internal_to_v1beta1(spec.source),
        node_name: spec.node_name,
    }
}

fn convert_volume_attachment_source_v1beta1_to_internal(
    source: VolumeAttachmentSource,
) -> internal::VolumeAttachmentSource {
    internal::VolumeAttachmentSource {
        persistent_volume_name: source.persistent_volume_name,
        inline_volume_spec: source.inline_volume_spec,
    }
}

fn convert_volume_attachment_source_internal_to_v1beta1(
    source: internal::VolumeAttachmentSource,
) -> VolumeAttachmentSource {
    VolumeAttachmentSource {
        persistent_volume_name: source.persistent_volume_name,
        inline_volume_spec: source.inline_volume_spec,
    }
}

fn convert_volume_attachment_status_v1beta1_to_internal(
    status: VolumeAttachmentStatus,
) -> internal::VolumeAttachmentStatus {
    internal::VolumeAttachmentStatus {
        attached: status.attached,
        attachment_metadata: status.attachment_metadata,
        attach_error: status
            .attach_error
            .map(convert_volume_error_v1beta1_to_internal),
        detach_error: status
            .detach_error
            .map(convert_volume_error_v1beta1_to_internal),
    }
}

fn convert_volume_attachment_status_internal_to_v1beta1(
    status: internal::VolumeAttachmentStatus,
) -> VolumeAttachmentStatus {
    VolumeAttachmentStatus {
        attached: status.attached,
        attachment_metadata: status.attachment_metadata,
        attach_error: status
            .attach_error
            .map(convert_volume_error_internal_to_v1beta1),
        detach_error: status
            .detach_error
            .map(convert_volume_error_internal_to_v1beta1),
    }
}

fn convert_volume_error_v1beta1_to_internal(error: VolumeError) -> internal::VolumeError {
    internal::VolumeError {
        time: error.time,
        message: error.message,
        error_code: error.error_code,
    }
}

fn convert_volume_error_internal_to_v1beta1(error: internal::VolumeError) -> VolumeError {
    VolumeError {
        time: error.time,
        message: error.message,
        error_code: error.error_code,
    }
}

// ============================================================================
// StorageClass Conversions
// ============================================================================

impl ToInternal<internal::StorageClass> for StorageClass {
    fn to_internal(self) -> internal::StorageClass {
        internal::StorageClass {
            type_meta: TypeMeta::default(),
            metadata: self.metadata,
            provisioner: self.provisioner,
            parameters: self.parameters,
            reclaim_policy: self.reclaim_policy,
            mount_options: self.mount_options,
            allow_volume_expansion: self.allow_volume_expansion,
            volume_binding_mode: self
                .volume_binding_mode
                .map(convert_volume_binding_mode_v1beta1_to_internal),
            allowed_topologies: self.allowed_topologies,
        }
    }
}

impl FromInternal<internal::StorageClass> for StorageClass {
    fn from_internal(value: internal::StorageClass) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: value.metadata,
            provisioner: value.provisioner,
            parameters: value.parameters,
            reclaim_policy: value.reclaim_policy,
            mount_options: value.mount_options,
            allow_volume_expansion: value.allow_volume_expansion,
            volume_binding_mode: value
                .volume_binding_mode
                .map(convert_volume_binding_mode_internal_to_v1beta1),
            allowed_topologies: value.allowed_topologies,
        };

        result
    }
}

impl ToInternal<internal::StorageClassList> for StorageClassList {
    fn to_internal(self) -> internal::StorageClassList {
        internal::StorageClassList {
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
    fn to_internal(self) -> internal::CSIDriver {
        internal::CSIDriver {
            type_meta: TypeMeta::default(),
            metadata: self.metadata,
            spec: convert_csi_driver_spec_v1beta1_to_internal(self.spec),
        }
    }
}

impl FromInternal<internal::CSIDriver> for CSIDriver {
    fn from_internal(value: internal::CSIDriver) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: value.metadata,
            spec: convert_csi_driver_spec_internal_to_v1beta1(value.spec),
        };

        result
    }
}

impl ToInternal<internal::CSIDriverList> for CSIDriverList {
    fn to_internal(self) -> internal::CSIDriverList {
        internal::CSIDriverList {
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
    fn to_internal(self) -> internal::CSINode {
        internal::CSINode {
            type_meta: TypeMeta::default(),
            metadata: self.metadata,
            spec: convert_csi_node_spec_v1beta1_to_internal(self.spec),
        }
    }
}

impl FromInternal<internal::CSINode> for CSINode {
    fn from_internal(value: internal::CSINode) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: value.metadata,
            spec: convert_csi_node_spec_internal_to_v1beta1(value.spec),
        };

        result
    }
}

impl ToInternal<internal::CSINodeList> for CSINodeList {
    fn to_internal(self) -> internal::CSINodeList {
        internal::CSINodeList {
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
    fn to_internal(self) -> internal::VolumeAttachment {
        internal::VolumeAttachment {
            type_meta: TypeMeta::default(),
            metadata: self.metadata,
            spec: convert_volume_attachment_spec_v1beta1_to_internal(self.spec),
            status: self
                .status
                .map(convert_volume_attachment_status_v1beta1_to_internal),
        }
    }
}

impl FromInternal<internal::VolumeAttachment> for VolumeAttachment {
    fn from_internal(value: internal::VolumeAttachment) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: value.metadata,
            spec: convert_volume_attachment_spec_internal_to_v1beta1(value.spec),
            status: value
                .status
                .map(convert_volume_attachment_status_internal_to_v1beta1),
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
        let mut result = Self {
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
        let mut result = Self {
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
    use crate::common::{ObjectMeta, TypeMeta};

    #[test]
    fn test_storage_class_round_trip() {
        let original = StorageClass {
            type_meta: TypeMeta {
                api_version: "storage.k8s.io/v1beta1".to_string(),
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
        assert_eq!(round_trip.type_meta.api_version, "storage.k8s.io/v1beta1");
    }

    #[test]
    fn test_volume_attachment_round_trip() {
        let original = VolumeAttachment {
            type_meta: TypeMeta {
                api_version: "storage.k8s.io/v1beta1".to_string(),
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
        assert_eq!(round_trip.type_meta.api_version, "storage.k8s.io/v1beta1");
    }

    #[test]
    fn test_csi_driver_round_trip() {
        let original = CSIDriver {
            type_meta: TypeMeta {
                api_version: "storage.k8s.io/v1beta1".to_string(),
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
        assert_eq!(round_trip.type_meta.api_version, "storage.k8s.io/v1beta1");
    }
}
