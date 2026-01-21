//! PersistentVolume and PersistentVolumeClaim types from the Kubernetes Core v1 API
//!
//! This module contains types for persistent storage resources.

use crate::common::{
    ApplyDefaults, HasTypeMeta, ListMeta, ObjectMeta, Quantity, ResourceSchema, Timestamp,
    TypeMeta, UnimplementedConversion, VersionedObject,
};
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
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
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
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
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
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
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
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
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
    pub selector: Option<serde_json::Value>,

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
    #[serde(default, skip_serializing_if = "Option::is_none")]
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale_io: Option<serde_json::Value>,

    /// Local represents a local storage device.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local: Option<LocalVolumeSource>,

    /// StorageOS represents a StorageOS volume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
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
    pub required: Option<serde_json::Value>,
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

#[cfg(test)]
mod tests {
    use super::*;

    // PersistentVolume tests
    #[test]
    fn test_persistent_volume_default() {
        let pv = PersistentVolume {
            type_meta: TypeMeta::default(),
            metadata: None,
            spec: None,
            status: None,
        };
        assert!(pv.metadata.is_none());
        assert!(pv.spec.is_none());
        assert!(pv.status.is_none());
    }

    #[test]
    fn test_persistent_volume_with_spec() {
        let mut capacity = BTreeMap::new();
        capacity.insert("storage".to_string(), Quantity::from("10Gi"));

        let spec = PersistentVolumeSpec {
            capacity,
            storage_class_name: Some("standard".to_string()),
            ..Default::default()
        };

        let pv = PersistentVolume {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("pv-1".to_string()),
                ..Default::default()
            }),
            spec: Some(spec),
            status: None,
        };

        assert_eq!(pv.metadata.as_ref().unwrap().name, Some("pv-1".to_string()));
        assert!(pv.spec.is_some());
    }

    #[test]
    fn test_persistent_volume_serialize() {
        let mut capacity = BTreeMap::new();
        capacity.insert("storage".to_string(), Quantity::from("5Gi"));

        let pv = PersistentVolume {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("pv-1".to_string()),
                ..Default::default()
            }),
            spec: Some(PersistentVolumeSpec {
                capacity,
                ..Default::default()
            }),
            status: None,
        };

        let json = serde_json::to_string(&pv).unwrap();
        assert!(json.contains(r#""name":"pv-1""#));
        assert!(json.contains(r#""storage":"5Gi""#));
    }

    #[test]
    fn test_persistent_volume_status() {
        let status = PersistentVolumeStatus {
            phase: Some(persistent_volume_phase::BOUND.to_string()),
            ..Default::default()
        };

        assert_eq!(
            status.phase,
            Some(persistent_volume_phase::BOUND.to_string())
        );
    }

    #[test]
    fn test_persistent_volume_list() {
        let list = PersistentVolumeList {
            type_meta: TypeMeta::default(),
            metadata: None,
            items: vec![],
        };
        assert!(list.items.is_empty());
    }

    // PersistentVolumeClaim tests
    #[test]
    fn test_persistent_volume_claim_default() {
        let pvc = PersistentVolumeClaim {
            type_meta: TypeMeta::default(),
            metadata: None,
            spec: None,
            status: None,
        };
        assert!(pvc.metadata.is_none());
        assert!(pvc.spec.is_none());
        assert!(pvc.status.is_none());
    }

    #[test]
    fn test_persistent_volume_claim_with_spec() {
        let spec = PersistentVolumeClaimSpec {
            storage_class_name: Some("standard".to_string()),
            access_modes: vec![persistent_volume_access_mode::READ_WRITE_ONCE.to_string()],
            ..Default::default()
        };

        let pvc = PersistentVolumeClaim {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("pvc-1".to_string()),
                ..Default::default()
            }),
            spec: Some(spec),
            status: None,
        };

        assert_eq!(
            pvc.metadata.as_ref().unwrap().name,
            Some("pvc-1".to_string())
        );
        assert!(pvc.spec.is_some());
    }

    #[test]
    fn test_persistent_volume_claim_serialize() {
        let pvc = PersistentVolumeClaim {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("pvc-1".to_string()),
                ..Default::default()
            }),
            spec: Some(PersistentVolumeClaimSpec {
                storage_class_name: Some("standard".to_string()),
                ..Default::default()
            }),
            status: None,
        };

        let json = serde_json::to_string(&pvc).unwrap();
        assert!(json.contains(r#""name":"pvc-1""#));
        assert!(json.contains(r#""storageClassName":"standard""#));
    }

    #[test]
    fn test_persistent_volume_claim_status() {
        let mut capacity = BTreeMap::new();
        capacity.insert("storage".to_string(), Quantity::from("5Gi"));

        let status = PersistentVolumeClaimStatus {
            phase: Some(persistent_volume_claim_phase::BOUND.to_string()),
            capacity,
            ..Default::default()
        };

        assert_eq!(
            status.phase,
            Some(persistent_volume_claim_phase::BOUND.to_string())
        );
        assert_eq!(status.capacity.len(), 1);
    }

    #[test]
    fn test_persistent_volume_claim_list() {
        let list = PersistentVolumeClaimList {
            type_meta: TypeMeta::default(),
            metadata: None,
            items: vec![],
        };
        assert!(list.items.is_empty());
    }

    #[test]
    fn test_persistent_volume_claim_condition() {
        let condition = PersistentVolumeClaimCondition {
            type_: persistent_volume_claim_condition_type::RESIZING.to_string(),
            status: "True".to_string(),
            last_probe_time: None,
            last_transition_time: None,
            reason: Some("Resizing".to_string()),
            message: None,
        };

        assert_eq!(
            condition.type_,
            persistent_volume_claim_condition_type::RESIZING
        );
    }

    // Supporting types tests
    #[test]
    fn test_typed_object_reference() {
        let ref_obj = TypedObjectReference {
            api_group: Some("".to_string()),
            kind: "PersistentVolumeClaim".to_string(),
            name: Some("pvc-1".to_string()),
            namespace: Some("default".to_string()),
        };

        assert_eq!(ref_obj.kind, Some("PersistentVolumeClaim".to_string()));
    }

    #[test]
    fn test_persistent_volume_claim_volume_source() {
        let source = PersistentVolumeClaimVolumeSource {
            claim_name: Some("pvc-1".to_string()),
            read_only: false,
        };

        assert_eq!(source.claim_name, Some("pvc-1".to_string()));
        assert!(!source.read_only);
    }

    #[test]
    fn test_volume_resource_requirements() {
        let mut requests = BTreeMap::new();
        requests.insert("storage".to_string(), Quantity::from("5Gi"));

        let resources = VolumeResourceRequirements {
            requests,
            ..Default::default()
        };

        assert_eq!(resources.requests.len(), 1);
    }

    // Constants tests
    #[test]
    fn test_persistent_volume_reclaim_policy_constants() {
        assert_eq!(persistent_volume_reclaim_policy::RECYCLE, "Recycle");
        assert_eq!(persistent_volume_reclaim_policy::DELETE, "Delete");
        assert_eq!(persistent_volume_reclaim_policy::RETAIN, "Retain");
    }

    #[test]
    fn test_persistent_volume_mode_constants() {
        assert_eq!(persistent_volume_mode::FILESYSTEM, "Filesystem");
        assert_eq!(persistent_volume_mode::BLOCK, "Block");
    }

    #[test]
    fn test_persistent_volume_phase_constants() {
        assert_eq!(persistent_volume_phase::AVAILABLE, "Available");
        assert_eq!(persistent_volume_phase::BOUND, "Bound");
        assert_eq!(persistent_volume_phase::RELEASED, "Released");
        assert_eq!(persistent_volume_phase::FAILED, "Failed");
    }

    #[test]
    fn test_persistent_volume_access_mode_constants() {
        assert_eq!(
            persistent_volume_access_mode::READ_WRITE_ONCE,
            "ReadWriteOnce"
        );
        assert_eq!(
            persistent_volume_access_mode::READ_ONLY_MANY,
            "ReadOnlyMany"
        );
        assert_eq!(
            persistent_volume_access_mode::READ_WRITE_MANY,
            "ReadWriteMany"
        );
        assert_eq!(
            persistent_volume_access_mode::READ_WRITE_ONCE_POD,
            "ReadWriteOncePod"
        );
    }

    #[test]
    fn test_persistent_volume_claim_phase_constants() {
        assert_eq!(persistent_volume_claim_phase::PENDING, "Pending");
        assert_eq!(persistent_volume_claim_phase::BOUND, "Bound");
        assert_eq!(persistent_volume_claim_phase::LOST, "Lost");
    }

    #[test]
    fn test_persistent_volume_claim_condition_type_constants() {
        assert_eq!(persistent_volume_claim_condition_type::RESIZING, "Resizing");
        assert_eq!(
            persistent_volume_claim_condition_type::FILE_SYSTEM_RESIZE_PENDING,
            "FileSystemResizePending"
        );
    }

    #[test]
    fn test_persistent_volume_round_trip() {
        let mut capacity = BTreeMap::new();
        capacity.insert("storage".to_string(), Quantity::from("10Gi"));

        let original = PersistentVolume {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("pv-1".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(PersistentVolumeSpec {
                capacity,
                ..Default::default()
            }),
            status: Some(PersistentVolumeStatus {
                phase: Some(persistent_volume_phase::AVAILABLE.to_string()),
                ..Default::default()
            }),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: PersistentVolume = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_persistent_volume_claim_round_trip() {
        let original = PersistentVolumeClaim {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("pvc-1".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(PersistentVolumeClaimSpec {
                storage_class_name: Some("standard".to_string()),
                access_modes: vec![persistent_volume_access_mode::READ_WRITE_ONCE.to_string()],
                selector: None,
                resources: None,
                volume_name: None,
                volume_mode: None,
                data_source: None,
                data_source_ref: None,
                volume_attributes_class_name: None,
            }),
            status: Some(PersistentVolumeClaimStatus {
                phase: Some(persistent_volume_claim_phase::BOUND.to_string()),
                ..Default::default()
            }),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: PersistentVolumeClaim = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }
}

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

impl ApplyDefaults for PersistentVolume {
    fn apply_defaults(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "PersistentVolume".to_string();
        }
    }
}

impl ApplyDefaults for PersistentVolumeList {
    fn apply_defaults(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "PersistentVolumeList".to_string();
        }
    }
}

impl ApplyDefaults for PersistentVolumeClaim {
    fn apply_defaults(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "PersistentVolumeClaim".to_string();
        }
    }
}

impl ApplyDefaults for PersistentVolumeClaimList {
    fn apply_defaults(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "PersistentVolumeClaimList".to_string();
        }
    }
}

// ----------------------------------------------------------------------------
// Version Conversion Placeholder
// ----------------------------------------------------------------------------

impl UnimplementedConversion for PersistentVolume {}
impl UnimplementedConversion for PersistentVolumeList {}
impl UnimplementedConversion for PersistentVolumeClaim {}
impl UnimplementedConversion for PersistentVolumeClaimList {}

// ----------------------------------------------------------------------------
// Protobuf Placeholder
// ----------------------------------------------------------------------------

impl_unimplemented_prost_message!(PersistentVolume);
impl_unimplemented_prost_message!(PersistentVolumeList);
impl_unimplemented_prost_message!(PersistentVolumeClaim);
impl_unimplemented_prost_message!(PersistentVolumeClaimList);
