//! Persistent Volume types from the Kubernetes Core API
//!
//! This module contains types for persistent volumes and persistent volume claims.
//! Source: k8s-pkg/apis/core/types.go

use crate::common::{ListMeta, ObjectMeta, Quantity};
use crate::core::internal::binding::SecretReference;
use crate::core::internal::{LabelSelector, NodeSelector, ResourceRequirements};
use crate::core::v1::PodCondition;
use crate::core::v1::reference::{ObjectReference, TypedLocalObjectReference};
use crate::impl_has_object_meta;
use serde::{Deserialize, Serialize};

// ============================================================================
// Re-exported Types from volume.rs
// ============================================================================

// Re-export the volume source types that are identical between VolumeSource and PersistentVolumeSource
pub use super::volume::{
    AWSElasticBlockStoreVolumeSource, AzureDiskVolumeSource, AzureFileVolumeSource,
    CephFSVolumeSource, CinderVolumeSource, FCVolumeSource, FlockerVolumeSource,
    GCEPersistentDiskVolumeSource, GitRepoVolumeSource, GlusterfsVolumeSource,
    HostPathVolumeSource, ISCSIVolumeSource, LocalVolumeSource, NFSVolumeSource,
    PhotonPersistentDiskVolumeSource, PortworxVolumeSource, QuobyteVolumeSource,
    ScaleIOVolumeSource, StorageOSVolumeSource, VsphereVirtualDiskVolumeSource,
};

// ============================================================================
// PV-specific Volume Source Types (with SecretReference)
// ============================================================================

/// GlusterfsPersistentVolumeSource represents a Glusterfs mount for PV.
/// Includes EndpointsNamespace for cross-namespace endpoint access.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GlusterfsPersistentVolumeSource {
    /// Required: EndpointsName is the endpoint name.
    #[serde(rename = "endpointsName")]
    pub endpoints_name: String,
    /// Required: Path is the Glusterfs volume path.
    pub path: String,
    /// Defaults to false (read/write).
    #[serde(default)]
    pub read_only: bool,
    /// EndpointsNamespace is the namespace that contains Glusterfs endpoint.
    #[serde(
        rename = "endpointsNamespace",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub endpoints_namespace: Option<String>,
}

/// RBDPersistentVolumeSource represents a Rados Block Device mount for PV.
/// Uses SecretReference instead of LocalObjectReference.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RBDPersistentVolumeSource {
    /// Required: CephMonitors is a collection of Ceph monitors.
    #[serde(rename = "cephMonitors")]
    pub ceph_monitors: Vec<String>,
    /// Required: RBDImage is the rados image name.
    #[serde(rename = "rbdImage")]
    pub rbd_image: String,
    /// Filesystem type to mount.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub fs_type: String,
    /// RadosPool is the rados pool name.
    #[serde(rename = "rbdPool", default, skip_serializing_if = "String::is_empty")]
    pub rbd_pool: String,
    /// RBDUser is the rados user name.
    #[serde(
        rename = "radosUser",
        default,
        skip_serializing_if = "String::is_empty"
    )]
    pub rados_user: String,
    /// Keyring is the path to key ring.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub keyring: String,
    /// SecretRef is reference to the authentication secret (SecretReference for PV).
    #[serde(rename = "secretRef", default, skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<SecretReference>,
    /// Defaults to false (read/write).
    #[serde(default)]
    pub read_only: bool,
}

/// FlexPersistentVolumeSource represents a generic volume resource for PV.
/// Uses SecretReference instead of LocalObjectReference.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct FlexPersistentVolumeSource {
    /// Driver is the name of the driver to use for this volume.
    pub driver: String,
    /// Filesystem type to mount.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub fs_type: String,
    /// SecretRef is reference to the secret object (SecretReference for PV).
    #[serde(rename = "secretRef", default, skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<SecretReference>,
    /// Defaults to false (read/write).
    #[serde(default)]
    pub read_only: bool,
    /// Extra driver options.
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub options: std::collections::BTreeMap<String, String>,
}

/// ISCSIPersistentVolumeSource represents an ISCSI disk for PV.
/// Uses SecretReference instead of LocalObjectReference.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ISCSIPersistentVolumeSource {
    /// Required: iSCSI target portal.
    #[serde(rename = "targetPortal")]
    pub target_portal: String,
    /// Required: target iSCSI Qualified Name.
    #[serde(rename = "iqn")]
    pub iqn: String,
    /// Required: iSCSI target lun number.
    #[serde(rename = "lun")]
    pub lun: i32,
    /// iSCSI interface name.
    #[serde(
        rename = "iscsiInterface",
        default,
        skip_serializing_if = "String::is_empty"
    )]
    pub iscsi_interface: String,
    /// Filesystem type to mount.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub fs_type: String,
    /// Defaults to false (read/write).
    #[serde(default)]
    pub read_only: bool,
    /// List of iSCSI target portal ips for high availability.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub portals: Vec<String>,
    /// Whether support iSCSI Discovery CHAP authentication.
    #[serde(rename = "chapAuthDiscovery", default)]
    pub chap_auth_discovery: bool,
    /// Whether support iSCSI Session CHAP authentication.
    #[serde(rename = "chapAuthSession", default)]
    pub chap_auth_session: bool,
    /// CHAP secret for authentication (SecretReference for PV).
    #[serde(rename = "secretRef", default, skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<SecretReference>,
    /// Custom initiator name per volume.
    #[serde(
        rename = "initiatorName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub initiator_name: Option<String>,
}

/// CephFSPersistentVolumeSource represents a Ceph Filesystem mount for PV.
/// Uses SecretReference instead of LocalObjectReference.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CephFSPersistentVolumeSource {
    /// Required: Monitors is a collection of Ceph monitors.
    pub monitors: Vec<String>,
    /// Used as the mounted root.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub path: String,
    /// User is the rados user name.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub user: String,
    /// SecretFile is the path to key ring.
    #[serde(
        rename = "secretFile",
        default,
        skip_serializing_if = "String::is_empty"
    )]
    pub secret_file: String,
    /// SecretRef is reference to the authentication secret (SecretReference for PV).
    #[serde(rename = "secretRef", default, skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<SecretReference>,
    /// Defaults to false (read/write).
    #[serde(default)]
    pub read_only: bool,
}

/// CinderPersistentVolumeSource represents a cinder volume resource for PV.
/// Uses SecretReference instead of LocalObjectReference.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CinderPersistentVolumeSource {
    /// Unique id of the volume used to identify the cinder volume.
    #[serde(rename = "volumeID")]
    pub volume_id: String,
    /// Filesystem type to mount.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub fs_type: String,
    /// Defaults to false (read/write).
    #[serde(default)]
    pub read_only: bool,
    /// Points to a secret object containing parameters (SecretReference for PV).
    #[serde(rename = "secretRef", default, skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<SecretReference>,
}

/// AzureFilePersistentVolumeSource represents an Azure File Service mount for PV.
/// Includes SecretNamespace for cross-namespace secret access.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AzureFilePersistentVolumeSource {
    /// The name of secret that contains Azure Storage Account Name and Key.
    #[serde(rename = "secretName")]
    pub secret_name: String,
    /// Share Name.
    #[serde(rename = "shareName")]
    pub share_name: String,
    /// Defaults to false (read/write).
    #[serde(default)]
    pub read_only: bool,
    /// The namespace of the secret that contains Azure Storage Account Name and Key.
    #[serde(
        rename = "secretNamespace",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub secret_namespace: Option<String>,
}

/// ScaleIOPersistentVolumeSource represents a persistent ScaleIO volume.
/// Uses SecretReference instead of LocalObjectReference.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScaleIOPersistentVolumeSource {
    /// The host address of the ScaleIO API Gateway.
    pub gateway: String,
    /// The name of the storage system as configured in ScaleIO.
    pub system: String,
    /// SecretRef references to the secret for ScaleIO user (SecretReference for PV).
    #[serde(rename = "secretRef", default, skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<SecretReference>,
    /// Flag to enable/disable SSL communication with Gateway.
    #[serde(rename = "sslEnabled", default)]
    pub ssl_enabled: bool,
    /// The name of the ScaleIO Protection Domain.
    #[serde(
        rename = "protectionDomain",
        default,
        skip_serializing_if = "String::is_empty"
    )]
    pub protection_domain: String,
    /// The ScaleIO Storage Pool associated with the protection domain.
    #[serde(
        rename = "storagePool",
        default,
        skip_serializing_if = "String::is_empty"
    )]
    pub storage_pool: String,
    /// Indicates whether the storage should be ThickProvisioned or ThinProvisioned.
    #[serde(
        rename = "storageMode",
        default,
        skip_serializing_if = "String::is_empty"
    )]
    pub storage_mode: String,
    /// The name of a volume created in the ScaleIO system.
    #[serde(rename = "volumeName")]
    pub volume_name: String,
    /// Filesystem type to mount.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub fs_type: String,
    /// Defaults to false (read/write).
    #[serde(default)]
    pub read_only: bool,
}

/// StorageOSPersistentVolumeSource represents a StorageOS volume.
/// Uses ObjectReference for secret access.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StorageOSPersistentVolumeSource {
    /// VolumeName is the human-readable name of the StorageOS volume.
    #[serde(rename = "volumeName")]
    pub volume_name: String,
    /// VolumeNamespace specifies the scope of the volume.
    #[serde(
        rename = "volumeNamespace",
        default,
        skip_serializing_if = "String::is_empty"
    )]
    pub volume_namespace: String,
    /// Filesystem type to mount.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub fs_type: String,
    /// Defaults to false (read/write).
    #[serde(default)]
    pub read_only: bool,
    /// SecretRef specifies the secret to use for obtaining the StorageOS API credentials (ObjectReference).
    #[serde(rename = "secretRef", default, skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<ObjectReference>,
}

/// CSIPersistentVolumeSource represents storage managed by an external CSI volume driver.
/// Uses multiple SecretReference fields for different CSI operations.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSIPersistentVolumeSource {
    /// Driver is the name of the driver to use for this volume.
    pub driver: String,
    /// VolumeHandle is the unique volume name returned by the CSI volume plugin.
    #[serde(rename = "volumeHandle")]
    pub volume_handle: String,
    /// Optional: ReadOnly for the volume.
    #[serde(default)]
    pub read_only: bool,
    /// Filesystem type to mount.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub fs_type: String,
    /// Attributes of the volume to publish.
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub volume_attributes: std::collections::BTreeMap<String, String>,
    /// ControllerPublishSecretRef is a reference to the secret object for CSI ControllerPublishVolume.
    #[serde(
        rename = "controllerPublishSecretRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub controller_publish_secret_ref: Option<SecretReference>,
    /// NodeStageSecretRef is a reference to the secret object for CSI NodeStageVolume.
    #[serde(
        rename = "nodeStageSecretRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub node_stage_secret_ref: Option<SecretReference>,
    /// NodePublishSecretRef is a reference to the secret object for CSI NodePublishVolume.
    #[serde(
        rename = "nodePublishSecretRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub node_publish_secret_ref: Option<SecretReference>,
    /// ControllerExpandSecretRef is a reference to the secret object for CSI ControllerExpandVolume.
    #[serde(
        rename = "controllerExpandSecretRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub controller_expand_secret_ref: Option<SecretReference>,
    /// NodeExpandSecretRef is a reference to the secret object for CSI NodeExpandVolume.
    #[serde(
        rename = "nodeExpandSecretRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub node_expand_secret_ref: Option<SecretReference>,
}

// ============================================================================
// PersistentVolumeSource
// ============================================================================

/// PersistentVolumeSource represents the source location of a volume to mount.
/// Similar to VolumeSource but uses SecretReference for cross-namespace secret access.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PersistentVolumeSource {
    /// hostPath represents a host path volume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_path: Option<HostPathVolumeSource>,
    /// gcePersistentDisk represents a GCE PD resource.
    #[serde(
        rename = "gcePersistentDisk",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub gce_persistent_disk: Option<GCEPersistentDiskVolumeSource>,
    /// awsElasticBlockStore represents an AWS EBS disk.
    #[serde(
        rename = "awsElasticBlockStore",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub aws_elastic_block_store: Option<AWSElasticBlockStoreVolumeSource>,
    /// gitRepo represents a git repository volume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub git_repo: Option<GitRepoVolumeSource>,
    /// nfs represents an NFS mount.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nfs: Option<NFSVolumeSource>,
    /// iscsi represents an ISCSI disk.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iscsi: Option<ISCSIPersistentVolumeSource>,
    /// glusterfs represents a Glusterfs mount.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub glusterfs: Option<GlusterfsPersistentVolumeSource>,
    /// rbd represents a Rados Block Device mount.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rbd: Option<RBDPersistentVolumeSource>,
    /// flexVolume represents a generic volume resource.
    #[serde(
        rename = "flexVolume",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub flex_volume: Option<FlexPersistentVolumeSource>,
    /// cinder represents a cinder volume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cinder: Option<CinderPersistentVolumeSource>,
    /// cephFS represents a Ceph Filesystem mount.
    #[serde(rename = "cephfs", default, skip_serializing_if = "Option::is_none")]
    pub ceph_fs: Option<CephFSPersistentVolumeSource>,
    /// flocker represents a Flocker volume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flocker: Option<FlockerVolumeSource>,
    /// fc represents a Fibre Channel resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fc: Option<FCVolumeSource>,
    /// azureFile represents an Azure File Service mount.
    #[serde(rename = "azureFile", default, skip_serializing_if = "Option::is_none")]
    pub azure_file: Option<AzureFilePersistentVolumeSource>,
    /// vsphereVolume represents a vSphere volume.
    #[serde(
        rename = "vsphereVolume",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub vsphere_volume: Option<VsphereVirtualDiskVolumeSource>,
    /// quobyte represents a Quobyte mount.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quobyte: Option<QuobyteVolumeSource>,
    /// azureDisk represents an Azure Data Disk mount.
    #[serde(rename = "azureDisk", default, skip_serializing_if = "Option::is_none")]
    pub azure_disk: Option<AzureDiskVolumeSource>,
    /// photonPersistentDisk represents a Photon Controller disk.
    #[serde(
        rename = "photonPersistentDisk",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub photon_persistent_disk: Option<PhotonPersistentDiskVolumeSource>,
    /// portworxVolume represents a portworx volume.
    #[serde(
        rename = "portworxVolume",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub portworx_volume: Option<PortworxVolumeSource>,
    /// scaleIO represents a ScaleIO persistent volume.
    #[serde(rename = "scaleIO", default, skip_serializing_if = "Option::is_none")]
    pub scale_io: Option<ScaleIOPersistentVolumeSource>,
    /// local represents directly-attached storage.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local: Option<LocalVolumeSource>,
    /// storageOS represents a StorageOS volume.
    #[serde(rename = "storageOS", default, skip_serializing_if = "Option::is_none")]
    pub storage_os: Option<StorageOSPersistentVolumeSource>,
    /// csi (Container Storage Interface) represents persistent storage.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub csi: Option<CSIPersistentVolumeSource>,
}

// ============================================================================
// PersistentVolume and Related Types
// ============================================================================

/// PersistentVolume (PV) is a storage resource in the cluster.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PersistentVolume {
    /// Standard object metadata.
    pub metadata: ObjectMeta,
    /// Spec defines a specification of a persistent volume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<PersistentVolumeSpec>,
    /// Status represents the current information about persistent volume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PersistentVolumeStatus>,
}
impl_has_object_meta!(PersistentVolume);

/// PersistentVolumeSpec is the specification of a persistent volume.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PersistentVolumeSpec {
    /// The underlying volume source.
    #[serde(flatten)]
    pub persistent_volume_source: PersistentVolumeSource,
    /// Capacity represents the capacity of the volume.
    #[serde(
        rename = "capacity",
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub capacity: std::collections::BTreeMap<String, Quantity>,
    /// Source of the volume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// AccessModes contains all ways the volume can be mounted.
    #[serde(rename = "accessModes", default, skip_serializing_if = "Vec::is_empty")]
    pub access_modes: Vec<String>,
    /// ClaimRef is the binding reference to a PersistentVolumeClaim.
    #[serde(rename = "claimRef", default, skip_serializing_if = "Option::is_none")]
    pub claim_ref: Option<ObjectReference>,
    /// PersistentVolumeReclaimPolicy defines what happens to the volume when released.
    #[serde(
        rename = "persistentVolumeReclaimPolicy",
        default,
        skip_serializing_if = "String::is_empty"
    )]
    pub persistent_volume_reclaim_policy: String,
    /// StorageClassName is the name of StorageClass to which this PV belongs.
    #[serde(
        rename = "storageClassName",
        default,
        skip_serializing_if = "String::is_empty"
    )]
    pub storage_class_name: String,
    /// MountOptions is the list of mount options.
    #[serde(
        rename = "mountOptions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub mount_options: Vec<String>,
    /// VolumeMode defines if volume is intended to be used with a formatted filesystem.
    #[serde(
        rename = "volumeMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub volume_mode: Option<String>,
    /// NodeAffinity defines constraints for volume node scheduling.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_affinity: Option<VolumeNodeAffinity>,
}

/// VolumeNodeAffinity defines constraints for volume scheduling.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct VolumeNodeAffinity {
    /// Required specifies hard node constraints.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<NodeSelector>,
}

/// PersistentVolumeStatus is the current status of a persistent volume.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PersistentVolumeStatus {
    /// Phase indicates if a volume is available, bound, or released.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub phase: String,
    /// Reason is a brief description of why the volume is in this state.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    /// Message is a human-readable message indicating details about why the volume is in this state.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
}

/// PersistentVolumeList is a list of PersistentVolume items.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PersistentVolumeList {
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,
    /// List of persistent volumes.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<PersistentVolume>,
}

// ============================================================================
// PersistentVolumeClaim and Related Types
// ============================================================================

/// PersistentVolumeClaim (PVC) is a user's request for storage.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PersistentVolumeClaim {
    /// Standard object metadata.
    pub metadata: ObjectMeta,
    /// Spec defines the desired characteristics of a claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<PersistentVolumeClaimSpec>,
    /// Status represents the current information about a claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PersistentVolumeClaimStatus>,
}
impl_has_object_meta!(PersistentVolumeClaim);

/// PersistentVolumeClaimSpec is the desired characteristics of a claim.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PersistentVolumeClaimSpec {
    /// AccessModes contains the desired access modes.
    #[serde(rename = "accessModes", default, skip_serializing_if = "Vec::is_empty")]
    pub access_modes: Vec<String>,
    /// Selector is a label query over volumes to consider for binding.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<LabelSelector>,
    /// Resources represents the minimum resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<ResourceRequirements>,
    /// VolumeName is the binding reference to the PersistentVolume.
    #[serde(
        rename = "volumeName",
        default,
        skip_serializing_if = "String::is_empty"
    )]
    pub volume_name: String,
    /// StorageClassName is the name of the StorageClass required by the claim.
    #[serde(
        rename = "storageClassName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_class_name: Option<String>,
    /// VolumeMode defines what type of volume is required.
    #[serde(
        rename = "volumeMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub volume_mode: Option<String>,
    /// DataSource is the field to specify the source of the volume.
    #[serde(
        rename = "dataSource",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub data_source: Option<TypedLocalObjectReference>,
    /// DataSourceRef is the field to specify the source of the volume.
    #[serde(
        rename = "dataSourceRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub data_source_ref: Option<TypedLocalObjectReference>,
}

/// PersistentVolumeClaimStatus is the current status of a claim.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PersistentVolumeClaimStatus {
    /// Phase represents the current phase of PersistentVolumeClaim.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub phase: String,
    /// AccessModes contains the actual access modes.
    #[serde(rename = "accessModes", default, skip_serializing_if = "Vec::is_empty")]
    pub access_modes: Vec<String>,
    /// Capacity represents the actual resources.
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub capacity: std::collections::BTreeMap<String, Quantity>,
    /// Conditions is the current condition of the claim.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<PodCondition>,
}

/// PersistentVolumeClaimList is a list of PersistentVolumeClaim items.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PersistentVolumeClaimList {
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,
    /// List of persistent volume claims.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<PersistentVolumeClaim>,
}

/// TypedObjectReferencePV references an object by name and optionally namespace and kind.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TypedObjectReferencePV {
    /// APIGroup is the group for the resource being referenced.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_group: Option<String>,
    /// Kind is the type of resource being referenced.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub kind: String,
    /// Name is the name of resource being referenced.
    pub name: String,
    /// Namespace is the namespace of resource being referenced.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub namespace: String,
}

#[cfg(test)]
mod tests {
}
