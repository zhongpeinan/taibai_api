//! PersistentVolume and PersistentVolumeClaim types from the Kubernetes Core v1 API
//!
//! This module contains types for persistent storage resources.

use crate::common::meta::LabelSelector;
use crate::common::{
    ApplyDefault, HasTypeMeta, ListMeta, ObjectMeta, Quantity, ResourceSchema, Timestamp, TypeMeta,
    VersionedObject,
};
use crate::core::v1::affinity::NodeSelector;
use crate::core::v1::reference::{ObjectReference, TypedLocalObjectReference};
use crate::core::v1::volume::LocalVolumeSource;
use crate::impl_unimplemented_prost_message;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// ============================================================================
// PersistentVolume Types
// ============================================================================

/// PersistentVolume (PV) is a storage resource in the cluster.
///
/// Corresponds to [Kubernetes PersistentVolume](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L366)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PersistentVolume {
    /// TypeMeta describes the type of this object
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// Spec defines a specification of a persistent volume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<PersistentVolumeSpec>,

    /// Status represents the current information about persistent volume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PersistentVolumeStatus>,
}

/// PersistentVolumeList is a list of PersistentVolume items.
///
/// Corresponds to [Kubernetes PersistentVolumeList](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L500)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PersistentVolumeList {
    /// TypeMeta describes the type of this object
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// List of persistent volumes.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<PersistentVolume>,
}

/// PersistentVolumeSpec is the specification for a PersistentVolume.
///
/// Corresponds to [Kubernetes PersistentVolumeSpec](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L388)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PersistentVolumeSpec {
    /// Capacity is the description of the persistent volume's resources.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub capacity: BTreeMap<String, Quantity>,

    /// PersistentVolumeSource is the actual volume backing the PV.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub persistent_volume_source: Option<PersistentVolumeSource>,

    /// AccessModes contains all ways the volume can be mounted.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub access_modes: Vec<String>,

    /// ClaimRef is part of a bi-directional binding between PV and PVC.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_ref: Option<ObjectReference>,

    /// PersistentVolumeReclaimPolicy defines what happens to the PV after release.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub persistent_volume_reclaim_policy: Option<String>,

    /// StorageClassName is the name of StorageClass to which this PV belongs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage_class_name: Option<String>,

    /// MountOptions is the list of mount options.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mount_options: Vec<String>,

    /// VolumeMode defines if volume is used as a filesystem or block device.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume_mode: Option<String>,

    /// NodeAffinity defines constraints for PV node affinity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_affinity: Option<VolumeNodeAffinity>,

    /// VolumeAttributesClassName is the name of the VolumeAttributesClass.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume_attributes_class_name: Option<String>,
}

/// PersistentVolumeStatus is the current status of a PersistentVolume.
///
/// Corresponds to [Kubernetes PersistentVolumeStatus](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L478)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PersistentVolumeStatus {
    /// Phase is the current phase of the persistent volume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,

    /// Message is a human-readable message indicating details.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// Reason is a brief CamelCase string indicating details.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    /// LastPhaseTransitionTime is the time the phase transitioned.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_phase_transition_time: Option<Timestamp>,
}

/// PersistentVolumeReclaimPolicy constants
pub mod persistent_volume_reclaim_policy {
    /// Recycle means the volume will be recycled (deprecated).
    pub const RECYCLE: &str = "Recycle";

    /// Delete means the volume will be deleted from Kubernetes.
    pub const DELETE: &str = "Delete";

    /// Retain means the volume will be retained after release.
    pub const RETAIN: &str = "Retain";
}

/// PersistentVolumeMode constants
pub mod persistent_volume_mode {
    /// Filesystem mode
    pub const FILESYSTEM: &str = "Filesystem";

    /// Block mode
    pub const BLOCK: &str = "Block";
}

/// PersistentVolumePhase constants
pub mod persistent_volume_phase {
    /// The volume is available
    pub const AVAILABLE: &str = "Available";

    /// The volume is bound
    pub const BOUND: &str = "Bound";

    /// The volume is released
    pub const RELEASED: &str = "Released";

    /// The volume has failed
    pub const FAILED: &str = "Failed";
}

/// PersistentVolumeAccessMode constants
pub mod persistent_volume_access_mode {
    /// The volume can be mounted as read-write by a single node.
    pub const READ_WRITE_ONCE: &str = "ReadWriteOnce";

    /// The volume can be mounted as read-only by many nodes.
    pub const READ_ONLY_MANY: &str = "ReadOnlyMany";

    /// The volume can be mounted as read-write by many nodes.
    pub const READ_WRITE_MANY: &str = "ReadWriteMany";

    /// The volume can be mounted as read-write once by a node with support for block devices.
    pub const READ_WRITE_ONCE_POD: &str = "ReadWriteOncePod";
}

// ============================================================================
// PersistentVolumeClaim Types
// ============================================================================

/// PersistentVolumeClaim (PVC) is a user's request for storage.
///
/// Corresponds to [Kubernetes PersistentVolumeClaim](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L516)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PersistentVolumeClaim {
    /// TypeMeta describes the type of this object
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// Spec defines the desired characteristics of a claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<PersistentVolumeClaimSpec>,

    /// Status represents the current information about a claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PersistentVolumeClaimStatus>,
}

/// PersistentVolumeClaimList is a list of PersistentVolumeClaim items.
///
/// Corresponds to [Kubernetes PersistentVolumeClaimList](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L539)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PersistentVolumeClaimList {
    /// TypeMeta describes the type of this object
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// List of persistent volume claims.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<PersistentVolumeClaim>,
}

/// PersistentVolumeClaimSpec is the desired characteristics of a claim.
///
/// Corresponds to [Kubernetes PersistentVolumeClaimSpec](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L552)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PersistentVolumeClaimSpec {
    /// AccessModes contains the desired access modes.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub access_modes: Vec<String>,

    /// Selector is a label query over volumes to consider for binding.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<LabelSelector>,

    /// Resources represents the minimum resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<VolumeResourceRequirements>,

    /// VolumeName is the binding reference to the PersistentVolume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume_name: Option<String>,

    /// StorageClassName is the name of the StorageClass.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage_class_name: Option<String>,

    /// VolumeMode defines the mode of the volume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume_mode: Option<String>,

    /// DataSource is the reference to the PVC to clone from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_source: Option<TypedLocalObjectReference>,

    /// DataSourceRef is an extended reference to the PVC to clone from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_source_ref: Option<TypedObjectReference>,

    /// VolumeAttributesClassName is the name of the VolumeAttributesClass.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume_attributes_class_name: Option<String>,
}

/// PersistentVolumeClaimStatus is the current status of a claim.
///
/// Corresponds to [Kubernetes PersistentVolumeClaimStatus](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L760)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PersistentVolumeClaimStatus {
    /// Phase is the current phase of the claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,

    /// AccessModes contains the actual access modes.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub access_modes: Vec<String>,

    /// Capacity is the actual capacity of the claim.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub capacity: BTreeMap<String, Quantity>,

    /// Conditions is the current condition of the claim.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<PersistentVolumeClaimCondition>,
}

/// PersistentVolumeClaimPhase constants
pub mod persistent_volume_claim_phase {
    /// The claim is pending
    pub const PENDING: &str = "Pending";

    /// The claim is bound
    pub const BOUND: &str = "Bound";

    /// The claim is lost
    pub const LOST: &str = "Lost";
}

/// PersistentVolumeClaimConditionType constants
pub mod persistent_volume_claim_condition_type {
    /// Resizing indicates the claim is resizing.
    pub const RESIZING: &str = "Resizing";

    /// FileSystemResizePending indicates resize is pending.
    pub const FILE_SYSTEM_RESIZE_PENDING: &str = "FileSystemResizePending";

    /// ControllerResizeError indicates a resize error.
    pub const CONTROLLER_RESIZE_ERROR: &str = "ControllerResizeError";

    /// NodeResizeError indicates a node resize error.
    pub const NODE_RESIZE_ERROR: &str = "NodeResizeError";

    /// ModifyVolumeError indicates a modify volume error.
    pub const MODIFY_VOLUME_ERROR: &str = "ModifyVolumeError";

    /// ModifyingVolume indicates the volume is being modified.
    pub const MODIFYING_VOLUME: &str = "ModifyingVolume";
}

/// PersistentVolumeClaimCondition details the condition of a claim.
///
/// Corresponds to [Kubernetes PersistentVolumeClaimCondition](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L735)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PersistentVolumeClaimCondition {
    /// Type is the type of the condition.
    #[serde(rename = "type")]
    pub type_: String,

    /// Status is the status of the condition.
    pub status: String,

    /// LastProbeTime is the last time the condition was probed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_probe_time: Option<Timestamp>,

    /// LastTransitionTime is the last time the condition transitioned.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<Timestamp>,

    /// Reason is the reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    /// Message is a human-readable message about the condition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

// ============================================================================
// Supporting Types
// ============================================================================

/// PersistentVolumeSource is the actual backing storage for the PV.
///
/// Corresponds to [Kubernetes PersistentVolumeSource](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L241)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PersistentVolumeSource {
    // Note: This is a union type in Go - only one field should be set
    // For now, we're using Option fields for each possible source type
    /// GCEPersistentDisk represents a GCE Disk resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gce_persistent_disk: Option<serde_json::Value>,

    /// AWSElasticBlockStore represents an AWS Disk resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aws_elastic_block_store: Option<serde_json::Value>,

    /// HostPath represents a host path directory.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_path: Option<serde_json::Value>,

    /// Glusterfs represents a Glusterfs mount.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub glusterfs: Option<serde_json::Value>,

    /// NFS represents an NFS mount.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nfs: Option<serde_json::Value>,

    /// RBD represents a Ceph RBD image.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rbd: Option<serde_json::Value>,

    /// ISCSI represents an ISCSI disk.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iscsi: Option<serde_json::Value>,

    /// Cinder represents a Cinder volume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cinder: Option<serde_json::Value>,

    /// CephFS represents a Ceph FS mount.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cephfs")]
    pub ceph_fs: Option<serde_json::Value>,

    /// FC represents a Fibre Channel disk.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fc: Option<serde_json::Value>,

    /// Flocker represents a Flocker volume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flocker: Option<serde_json::Value>,

    /// FlexVolume represents a generic volume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flex_volume: Option<serde_json::Value>,

    /// AzureFile represents an Azure File Service mount.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub azure_file: Option<serde_json::Value>,

    /// VsphereVolume represents a vSphere volume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vsphere_volume: Option<serde_json::Value>,

    /// Quobyte represents a Quobyte mount.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quobyte: Option<serde_json::Value>,

    /// AzureDisk represents an Azure Data Disk.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub azure_disk: Option<serde_json::Value>,

    /// PhotonPersistentDisk represents a Photon Controller disk.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub photon_persistent_disk: Option<serde_json::Value>,

    /// PortworxVolume represents a Portworx volume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub portworx_volume: Option<serde_json::Value>,

    /// ScaleIO represents a ScaleIO volume.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scaleIO")]
    pub scale_io: Option<serde_json::Value>,

    /// Local represents a local storage device.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local: Option<LocalVolumeSource>,

    /// StorageOS represents a StorageOS volume.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageos")]
    pub storage_os: Option<serde_json::Value>,

    /// CSI represents a CSI volume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub csi: Option<serde_json::Value>,
}

/// VolumeNodeAffinity defines constraints for persistent volume node affinity.
///
/// Corresponds to [Kubernetes VolumeNodeAffinity](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L445)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct VolumeNodeAffinity {
    /// Required specifies hard node affinity constraints.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<NodeSelector>,
}

/// VolumeResourceRequirements describes the storage resource requirements.
///
/// Corresponds to [Kubernetes VolumeResourceRequirements](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2860)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct VolumeResourceRequirements {
    /// Limits is the maximum storage resources.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub limits: BTreeMap<String, Quantity>,

    /// Requests is the requested storage resources.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub requests: BTreeMap<String, Quantity>,
}

/// TypedObjectReference is a reference to an object with a specific type.
///
/// Corresponds to [Kubernetes TypedObjectReference](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L629)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TypedObjectReference {
    /// APIGroup is the group for the resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_group: Option<String>,

    /// Kind is the kind of resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// Name is the name of resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Namespace is the namespace of resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// PersistentVolumeClaimVolumeSource references a PVC.
///
/// Corresponds to [Kubernetes PersistentVolumeClaimVolumeSource](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L229)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PersistentVolumeClaimVolumeSource {
    /// ClaimName is the name of the PVC.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_name: Option<String>,

    /// ReadOnly is true if the volume is read-only.
    #[serde(default)]
    pub read_only: bool,
}

// ============================================================================
// CSI and Secret Reference Types
// ============================================================================

/// SecretReference represents a Secret Reference.
///
/// Corresponds to [Kubernetes SecretReference](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L1183)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SecretReference {
    /// Name is unique within a namespace to reference a secret resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Namespace defines the space within which the secret name must be unique.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// CSIPersistentVolumeSource represents storage from an external CSI volume driver.
///
/// Corresponds to [Kubernetes CSIPersistentVolumeSource](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2190)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSIPersistentVolumeSource {
    /// Driver is the name of the driver to use for this volume.
    pub driver: String,

    /// VolumeHandle is the unique volume name returned by the CSI volume plugin's CreateVolume.
    pub volume_handle: String,

    /// ReadOnly value to pass to ControllerPublishVolumeRequest.
    #[serde(default)]
    pub read_only: bool,

    /// FSType to mount. Must be a filesystem type supported by the host operating system.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fsType")]
    pub fs_type: Option<String>,

    /// VolumeAttributes of the volume to publish.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume_attributes: Option<std::collections::BTreeMap<String, String>>,

    /// ControllerPublishSecretRef is a reference to the secret for CSI ControllerPublish.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub controller_publish_secret_ref: Option<SecretReference>,

    /// NodeStageSecretRef is a reference to the secret for CSI NodeStage.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_stage_secret_ref: Option<SecretReference>,

    /// NodePublishSecretRef is a reference to the secret for CSI NodePublish.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_publish_secret_ref: Option<SecretReference>,

    /// ControllerExpandSecretRef is a reference to the secret for CSI ControllerExpandVolume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub controller_expand_secret_ref: Option<SecretReference>,

    /// NodeExpandSecretRef is a reference to the secret for CSI NodeExpandVolume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_expand_secret_ref: Option<SecretReference>,
}

/// ModifyVolumeStatus represents the status of a ControllerModifyVolume operation.
///
/// Corresponds to [Kubernetes ModifyVolumeStatus](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L717)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ModifyVolumeStatus {
    /// TargetVolumeAttributesClassName is the name of the VolumeAttributesClass being reconciled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_volume_attributes_class_name: Option<String>,

    /// Status is the status of the ControllerModifyVolume operation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// ModifyVolumeStatus constants.
pub mod modify_volume_status {
    pub const PENDING: &str = "Pending";
    pub const IN_PROGRESS: &str = "InProgress";
    pub const INFEASIBLE: &str = "Infeasible";
}

#[cfg(test)]
mod tests {}

// ============================================================================
// Trait Implementations
// ============================================================================

// ----------------------------------------------------------------------------
// ResourceSchema Implementation
// ----------------------------------------------------------------------------

impl ResourceSchema for PersistentVolume {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        ""
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "PersistentVolume"
    }
    fn resource(_: &Self::Meta) -> &str {
        "persistentvolumes"
    }

    fn group_static() -> &'static str {
        ""
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "PersistentVolume"
    }
    fn resource_static() -> &'static str {
        "persistentvolumes"
    }
}

impl ResourceSchema for PersistentVolumeList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        ""
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "PersistentVolumeList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "persistentvolumes"
    }

    fn group_static() -> &'static str {
        ""
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "PersistentVolumeList"
    }
    fn resource_static() -> &'static str {
        "persistentvolumes"
    }
}

impl ResourceSchema for PersistentVolumeClaim {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        ""
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "PersistentVolumeClaim"
    }
    fn resource(_: &Self::Meta) -> &str {
        "persistentvolumeclaims"
    }

    fn group_static() -> &'static str {
        ""
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "PersistentVolumeClaim"
    }
    fn resource_static() -> &'static str {
        "persistentvolumeclaims"
    }
}

impl ResourceSchema for PersistentVolumeClaimList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        ""
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "PersistentVolumeClaimList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "persistentvolumeclaims"
    }

    fn group_static() -> &'static str {
        ""
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "PersistentVolumeClaimList"
    }
    fn resource_static() -> &'static str {
        "persistentvolumeclaims"
    }
}

// ----------------------------------------------------------------------------
// HasTypeMeta Implementation
// ----------------------------------------------------------------------------

impl HasTypeMeta for PersistentVolume {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for PersistentVolumeList {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for PersistentVolumeClaim {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for PersistentVolumeClaimList {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

// ----------------------------------------------------------------------------
// VersionedObject Implementation
// ----------------------------------------------------------------------------

impl VersionedObject for PersistentVolume {
    fn metadata(&self) -> &ObjectMeta {
        use std::sync::OnceLock;
        self.metadata.as_ref().unwrap_or_else(|| {
            static DEFAULT: OnceLock<ObjectMeta> = OnceLock::new();
            DEFAULT.get_or_init(ObjectMeta::default)
        })
    }

    fn metadata_mut(&mut self) -> &mut ObjectMeta {
        self.metadata.get_or_insert_with(ObjectMeta::default)
    }
}

impl VersionedObject for PersistentVolumeClaim {
    fn metadata(&self) -> &ObjectMeta {
        use std::sync::OnceLock;
        self.metadata.as_ref().unwrap_or_else(|| {
            static DEFAULT: OnceLock<ObjectMeta> = OnceLock::new();
            DEFAULT.get_or_init(ObjectMeta::default)
        })
    }

    fn metadata_mut(&mut self) -> &mut ObjectMeta {
        self.metadata.get_or_insert_with(ObjectMeta::default)
    }
}

// Note: List types do not implement VersionedObject because they have ListMeta, not ObjectMeta

// ----------------------------------------------------------------------------
// ApplyDefaults Implementation
// ----------------------------------------------------------------------------

impl ApplyDefault for PersistentVolume {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "PersistentVolume".to_string();
        }
        // Apply defaults to spec if present
        if let Some(ref mut spec) = self.spec {
            spec.apply_default();
        }
        // Apply defaults to status if present
        if let Some(ref mut status) = self.status {
            status.apply_default();
        }
    }
}

impl ApplyDefault for PersistentVolumeList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "PersistentVolumeList".to_string();
        }
        for item in &mut self.items {
            item.apply_default();
        }
    }
}

impl ApplyDefault for PersistentVolumeClaim {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "PersistentVolumeClaim".to_string();
        }
        // Apply defaults to spec if present
        if let Some(ref mut spec) = self.spec {
            spec.apply_default();
        }
        // Apply defaults to status if present
        if let Some(ref mut status) = self.status {
            status.apply_default();
        }
    }
}

impl ApplyDefault for PersistentVolumeClaimList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "PersistentVolumeClaimList".to_string();
        }
        for item in &mut self.items {
            item.apply_default();
        }
    }
}

impl ApplyDefault for PersistentVolumeSpec {
    fn apply_default(&mut self) {
        // Set default persistent volume reclaim policy to "Retain" if not specified
        if self.persistent_volume_reclaim_policy.is_none() {
            self.persistent_volume_reclaim_policy = Some("Retain".to_string());
        }

        // Set default volume mode to "Filesystem" if not specified
        if self.volume_mode.is_none() {
            self.volume_mode = Some("Filesystem".to_string());
        }
    }
}

impl ApplyDefault for PersistentVolumeStatus {
    fn apply_default(&mut self) {
        // Set default phase to "Pending" if not specified
        if self.phase.is_none() {
            self.phase = Some("Pending".to_string());
        }
    }
}

impl ApplyDefault for PersistentVolumeClaimSpec {
    fn apply_default(&mut self) {
        // Set default volume mode to "Filesystem" if not specified
        if self.volume_mode.is_none() {
            self.volume_mode = Some("Filesystem".to_string());
        }
    }
}

impl ApplyDefault for PersistentVolumeClaimStatus {
    fn apply_default(&mut self) {
        // Set default phase to "Pending" if not specified
        if self.phase.is_none() {
            self.phase = Some("Pending".to_string());
        }
    }
}

// ----------------------------------------------------------------------------
// Protobuf Placeholder
// ----------------------------------------------------------------------------

impl_unimplemented_prost_message!(PersistentVolume);
impl_unimplemented_prost_message!(PersistentVolumeList);
impl_unimplemented_prost_message!(PersistentVolumeClaim);
impl_unimplemented_prost_message!(PersistentVolumeClaimList);
