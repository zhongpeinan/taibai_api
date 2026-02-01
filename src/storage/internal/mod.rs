//! Storage internal API types
//!
//! This module re-exports the Storage v1 types, as they are identical
//! to the internal types defined in `k8s.io/kubernetes/pkg/apis/storage`.
//!
//! In Kubernetes, the internal types (pkg/apis/storage) and the public v1 API
//! types (api/storage/v1) have the same structure. The internal types are used
//! within Kubernetes for internal logic, while v1 types are exposed via the API.
//!
//! This module provides the internal types by re-exporting from v1, maintaining
//! a single source of truth for the type definitions.

// Core storage types
pub use crate::storage::v1::{
    CSIDriver, CSIDriverList, CSIDriverSpec, CSINode, CSINodeDriver, CSINodeList, CSINodeSpec,
    CSIStorageCapacity, CSIStorageCapacityList, FSGroupPolicy, StorageClass, StorageClassList,
    TokenRequest, VolumeAttachment, VolumeAttachmentList, VolumeAttachmentSource,
    VolumeAttachmentSpec, VolumeAttachmentStatus, VolumeAttributesClass, VolumeAttributesClassList,
    VolumeBindingMode, VolumeError, VolumeLifecycleMode, VolumeNodeResources,
};

// Constants
pub use crate::storage::v1::{fs_group_policy, volume_binding_mode, volume_lifecycle_mode};

pub mod validation;
