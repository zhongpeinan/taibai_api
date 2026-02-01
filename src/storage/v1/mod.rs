//! Storage v1 API types
//!
//! This module contains the Storage v1 API types.

pub mod conversion;
pub mod csi_driver;
pub mod csi_node;
pub mod csi_storage_capacity;
pub mod defaults;
pub mod storage_class;
pub mod validation;
pub mod volume_attachment;
pub mod volume_attributes_class;

pub use csi_driver::{
    CSIDriver, CSIDriverList, CSIDriverSpec, FSGroupPolicy, TokenRequest, VolumeLifecycleMode,
};
pub use csi_node::{CSINode, CSINodeDriver, CSINodeList, CSINodeSpec, VolumeNodeResources};
pub use csi_storage_capacity::{CSIStorageCapacity, CSIStorageCapacityList};
pub use storage_class::{StorageClass, StorageClassList, VolumeBindingMode};
pub use volume_attachment::{
    VolumeAttachment, VolumeAttachmentList, VolumeAttachmentSource, VolumeAttachmentSpec,
    VolumeAttachmentStatus, VolumeError,
};
pub use volume_attributes_class::{VolumeAttributesClass, VolumeAttributesClassList};

// Re-export constants
pub use csi_driver::{fs_group_policy, volume_lifecycle_mode};
pub use storage_class::volume_binding_mode;

#[cfg(test)]
mod trait_tests;

#[cfg(test)]
mod serde_roundtrip_tests;

#[cfg(test)]
mod conversion_roundtrip_tests;
