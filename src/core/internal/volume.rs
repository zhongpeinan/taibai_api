//! Volume-related types from the Kubernetes Core API
//!
//! This module contains types for volume mounts, devices, and volume sources.
//! Source: k8s-pkg/apis/core/types.go

use crate::common::meta::ObjectMeta;
use crate::common::util::Quantity;
use crate::core::internal::{
    AzureDataDiskCachingMode, AzureDataDiskKind, MountPropagationMode, PersistentVolumeClaimSpec,
    PullPolicy, RecursiveReadOnlyMode, StorageMedium,
};
use crate::core::v1::reference::LocalObjectReference;
use crate::core::v1::selector::{ObjectFieldSelector, ResourceFieldSelector};
use crate::core::v1::volume::KeyToPath;
use crate::impl_has_object_meta;
use serde::{Deserialize, Serialize};

/// VolumeMount describes a mounting of a volume within a container.
///
/// Corresponds to [Kubernetes VolumeMount](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2134)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct VolumeMount {
    /// Name of the volume to mount.
    pub name: String,
    /// Mounted read-only if true.
    #[serde(default)]
    pub read_only: bool,
    /// RecursiveReadOnly specifies whether read-only mounts should be recursive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recursive_read_only: Option<RecursiveReadOnlyMode>,
    /// Path within the container at which the volume should be mounted.
    pub mount_path: String,
    /// Path within the volume from which the container's volume should be mounted.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub sub_path: String,
    /// MountPropagationMode describes how mounts are propagated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mount_propagation: Option<MountPropagationMode>,
    /// Expanded path within the volume from which the container's volume should be mounted.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub sub_path_expr: String,
}

/// VolumeDevice describes a mapping of a raw block device within a container.
///
/// Corresponds to [Kubernetes VolumeDevice](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2222)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct VolumeDevice {
    /// Name of the volume to mount.
    pub name: String,
    /// Path inside the container for the device.
    pub device_path: String,
}

/// VolumeMountStatus shows status of volume mount.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct VolumeMountStatus {
    /// Name is the name of the volume mount.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// MountPath is the path of the volume mount.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub mount_path: String,
    /// ReadOnly is true if the volume mount is read-only.
    #[serde(default)]
    pub read_only: bool,
    /// RecursiveReadOnly is the mode of the recursive read-only mount.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recursive_read_only: Option<RecursiveReadOnlyMode>,
}

// ============================================================================
// Volume Source Types
// ============================================================================

/// Volume represents a named volume in a pod.
///
/// Corresponds to [Kubernetes Volume](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L45)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Volume {
    /// Required: This must be a DNS_LABEL. Each volume in a pod must have a unique name.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// The VolumeSource represents the location and type of the volume to mount.
    #[serde(flatten)]
    pub volume_source: VolumeSource,
}

/// VolumeSource represents the source location of a volume to mount.
/// Only one of its members may be specified.
///
/// Corresponds to [Kubernetes VolumeSource](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L58)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct VolumeSource {
    /// hostPath represents file or directory on the host machine.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_path: Option<HostPathVolumeSource>,
    /// emptyDir represents a temporary directory that shares a pod's lifetime.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub empty_dir: Option<EmptyDirVolumeSource>,
    /// gcePersistentDisk represents a GCE Disk resource.
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
    /// gitRepo represents a git repository at a particular revision.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub git_repo: Option<GitRepoVolumeSource>,
    /// secret represents a secret that should populate this volume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<SecretVolumeSource>,
    /// nfs represents an NFS mount on the host.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nfs: Option<NFSVolumeSource>,
    /// iscsi represents an ISCSI Disk resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iscsi: Option<ISCSIVolumeSource>,
    /// glusterfs represents a Glusterfs mount.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub glusterfs: Option<GlusterfsVolumeSource>,
    /// persistentVolumeClaim represents a reference to a PersistentVolumeClaim.
    #[serde(
        rename = "persistentVolumeClaim",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub persistent_volume_claim: Option<PersistentVolumeClaimVolumeSource>,
    /// rbd represents a Rados Block Device mount.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rbd: Option<RBDVolumeSource>,
    /// quobyte represents a Quobyte mount.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quobyte: Option<QuobyteVolumeSource>,
    /// flexVolume represents a generic volume resource.
    #[serde(
        rename = "flexVolume",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub flex_volume: Option<FlexVolumeSource>,
    /// cinder represents a cinder volume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cinder: Option<CinderVolumeSource>,
    /// cephFS represents a Cephfs mount.
    #[serde(rename = "cephfs", default, skip_serializing_if = "Option::is_none")]
    pub ceph_fs: Option<CephFSVolumeSource>,
    /// flocker represents a Flocker volume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flocker: Option<FlockerVolumeSource>,
    /// downwardAPI represents metadata about the pod.
    #[serde(
        rename = "downwardAPI",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub downward_api: Option<DownwardAPIVolumeSource>,
    /// fc represents a Fibre Channel resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fc: Option<FCVolumeSource>,
    /// azureFile represents an Azure File Service mount.
    #[serde(rename = "azureFile", default, skip_serializing_if = "Option::is_none")]
    pub azure_file: Option<AzureFileVolumeSource>,
    /// configMap represents a configMap that should populate this volume.
    #[serde(rename = "configMap", default, skip_serializing_if = "Option::is_none")]
    pub config_map: Option<ConfigMapVolumeSource>,
    /// vsphereVolume represents a vSphere volume.
    #[serde(
        rename = "vsphereVolume",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub vsphere_volume: Option<VsphereVirtualDiskVolumeSource>,
    /// azureDisk represents an Azure Data Disk mount.
    #[serde(rename = "azureDisk", default, skip_serializing_if = "Option::is_none")]
    pub azure_disk: Option<AzureDiskVolumeSource>,
    /// photonPersistentDisk represents a PhotonController persistent disk.
    #[serde(
        rename = "photonPersistentDisk",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub photon_persistent_disk: Option<PhotonPersistentDiskVolumeSource>,
    /// projected represents volume projection for multiple volume sources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub projected: Option<ProjectedVolumeSource>,
    /// portworxVolume represents a portworx volume.
    #[serde(
        rename = "portworxVolume",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub portworx_volume: Option<PortworxVolumeSource>,
    /// scaleIO represents a ScaleIO persistent volume.
    #[serde(rename = "scaleIO", default, skip_serializing_if = "Option::is_none")]
    pub scale_io: Option<ScaleIOVolumeSource>,
    /// storageOS represents a StorageOS volume.
    #[serde(rename = "storageOS", default, skip_serializing_if = "Option::is_none")]
    pub storage_os: Option<StorageOSVolumeSource>,
    /// csi (Container Storage Interface) represents ephemeral storage.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub csi: Option<CSIVolumeSource>,
    /// ephemeral represents a volume handled by a cluster storage driver.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ephemeral: Option<EphemeralVolumeSource>,
    /// image represents an OCI object (container image or artifact).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<ImageVolumeSource>,
}

// ============================================================================
// Individual Volume Source Types
// ============================================================================

/// HostPathVolumeSource represents a host path volume.
///
/// Corresponds to [Kubernetes HostPathVolumeSource](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L861)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct HostPathVolumeSource {
    /// Path of the directory on the host.
    pub path: String,
    /// Type of HostPath volume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

/// EmptyDirVolumeSource represents an empty directory for a pod.
///
/// Corresponds to [Kubernetes EmptyDirVolumeSource](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L870)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct EmptyDirVolumeSource {
    /// Storage medium to use for this directory.
    #[serde(default)]
    pub medium: StorageMedium,
    /// Total amount of local storage required for this EmptyDir volume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size_limit: Option<Quantity>,
}

/// GCEPersistentDiskVolumeSource represents a GCE PD resource.
///
/// Corresponds to [Kubernetes GCEPersistentDiskVolumeSource](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L912)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GCEPersistentDiskVolumeSource {
    /// Unique name of the PD resource.
    #[serde(rename = "pdName")]
    pub pd_name: String,
    /// Filesystem type to mount.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub fs_type: String,
    /// Partition on the disk to mount.
    #[serde(default)]
    pub partition: i32,
    /// Defaults to false (read/write).
    #[serde(default)]
    pub read_only: bool,
}

/// AWSElasticBlockStoreVolumeSource represents an AWS EBS disk.
///
/// Corresponds to [Kubernetes AWSElasticBlockStoreVolumeSource](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L1112)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AWSElasticBlockStoreVolumeSource {
    /// Unique id of the persistent disk resource.
    #[serde(rename = "volumeID")]
    pub volume_id: String,
    /// Filesystem type to mount.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub fs_type: String,
    /// Partition on the disk to mount.
    #[serde(default)]
    pub partition: i32,
    /// Defaults to false (read/write).
    #[serde(default)]
    pub read_only: bool,
}

/// GitRepoVolumeSource represents a git repository volume.
///
/// Corresponds to [Kubernetes GitRepoVolumeSource](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L1138)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GitRepoVolumeSource {
    /// Repository URL.
    pub repository: String,
    /// Commit hash.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub revision: String,
    /// Clone target.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub directory: String,
}

/// SecretVolumeSource adapts a Secret into a volume.
///
/// Corresponds to [Kubernetes SecretVolumeSource](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L1160)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SecretVolumeSource {
    /// Name of the secret in the pod's namespace.
    #[serde(rename = "secretName")]
    pub secret_name: String,
    /// Mode bits to use on created files by default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_mode: Option<i32>,
    /// If unspecified, each key-value pair will be projected.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<KeyToPath>,
    /// Specify whether the Secret or its key must be defined.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// NFSVolumeSource represents an NFS mount.
///
/// Corresponds to [Kubernetes NFSVolumeSource](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L1212)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NFSVolumeSource {
    /// Server is the hostname or IP address of the NFS server.
    pub server: String,
    /// Path is the exported NFS share.
    pub path: String,
    /// Defaults to false (read/write).
    #[serde(default)]
    pub read_only: bool,
}

/// ISCSIVolumeSource represents an ISCSI disk.
///
/// Corresponds to [Kubernetes ISCSIVolumeSource](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L938)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ISCSIVolumeSource {
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
    /// CHAP secret for authentication.
    #[serde(rename = "secretRef", default, skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<LocalObjectReference>,
    /// Custom initiator name per volume.
    #[serde(
        rename = "initiatorName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub initiator_name: Option<String>,
}

/// GlusterfsVolumeSource represents a Glusterfs mount.
///
/// Corresponds to [Kubernetes GlusterfsVolumeSource](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L1259)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GlusterfsVolumeSource {
    /// Required: EndpointsName is the endpoint name.
    #[serde(rename = "endpointsName")]
    pub endpoints_name: String,
    /// Required: Path is the Glusterfs volume path.
    pub path: String,
    /// Defaults to false (read/write).
    #[serde(default)]
    pub read_only: bool,
}

/// PersistentVolumeClaimVolumeSource references a PersistentVolumeClaim.
///
/// Corresponds to [Kubernetes PersistentVolumeClaimVolumeSource](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L332)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PersistentVolumeClaimVolumeSource {
    /// ClaimName is the name of a PersistentVolumeClaim in the same namespace.
    #[serde(rename = "claimName")]
    pub claim_name: String,
    /// Defaults to false (read/write).
    #[serde(default)]
    pub read_only: bool,
}

/// RBDVolumeSource represents a Rados Block Device mount.
///
/// Corresponds to [Kubernetes RBDVolumeSource](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L1298)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RBDVolumeSource {
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
    /// SecretRef is name of the authentication secret.
    #[serde(rename = "secretRef", default, skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<LocalObjectReference>,
    /// Defaults to false (read/write).
    #[serde(default)]
    pub read_only: bool,
}

/// QuobyteVolumeSource represents a Quobyte mount.
///
/// Corresponds to [Kubernetes QuobyteVolumeSource](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L1227)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct QuobyteVolumeSource {
    /// Registry represents a single or multiple Quobyte Registry services.
    pub registry: String,
    /// Volume is a string that references an already created Quobyte volume.
    pub volume: String,
    /// Defaults to false (read/write).
    #[serde(default)]
    pub read_only: bool,
    /// User to map volume access to.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub user: String,
    /// Group to map volume access to.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub group: String,
    /// Tenant owning the given Quobyte volume.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub tenant: String,
}

/// FlexVolumeSource represents a generic volume resource.
///
/// Corresponds to [Kubernetes FlexVolumeSource](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L1086)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct FlexVolumeSource {
    /// Driver is the name of the driver to use for this volume.
    pub driver: String,
    /// Filesystem type to mount.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub fs_type: String,
    /// SecretRef is reference to the secret object.
    #[serde(rename = "secretRef", default, skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<LocalObjectReference>,
    /// Defaults to false (read/write).
    #[serde(default)]
    pub read_only: bool,
    /// Extra driver options.
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub options: std::collections::BTreeMap<String, String>,
}

/// CinderVolumeSource represents a cinder volume resource.
///
/// Corresponds to [Kubernetes CinderVolumeSource](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L1360)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CinderVolumeSource {
    /// Unique id of the volume used to identify the cinder volume.
    #[serde(rename = "volumeID")]
    pub volume_id: String,
    /// Filesystem type to mount.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub fs_type: String,
    /// Defaults to false (read/write).
    #[serde(default)]
    pub read_only: bool,
    /// Points to a secret object containing parameters.
    #[serde(rename = "secretRef", default, skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<LocalObjectReference>,
}

/// CephFSVolumeSource represents a Ceph Filesystem mount.
///
/// Corresponds to [Kubernetes CephFSVolumeSource](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L1404)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CephFSVolumeSource {
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
    /// SecretRef is reference to the authentication secret.
    #[serde(rename = "secretRef", default, skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<LocalObjectReference>,
    /// Defaults to false (read/write).
    #[serde(default)]
    pub read_only: bool,
}

/// FlockerVolumeSource represents a Flocker volume.
///
/// Corresponds to [Kubernetes FlockerVolumeSource](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L1461)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct FlockerVolumeSource {
    /// Name of the dataset stored as metadata.
    #[serde(rename = "datasetName")]
    pub dataset_name: String,
    /// UUID of the dataset.
    #[serde(rename = "datasetUUID")]
    pub dataset_uuid: String,
}

/// DownwardAPIVolumeSource represents downward API info.
///
/// Corresponds to [Kubernetes DownwardAPIVolumeSource](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L1474)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DownwardAPIVolumeSource {
    /// Items is a list of DownwardAPIVolume file.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<DownwardAPIVolumeFile>,
    /// Mode bits to use on created files by default.
    #[serde(
        rename = "defaultMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_mode: Option<i32>,
}

/// DownwardAPIVolumeFile represents a single file containing downward API info.
///
/// Corresponds to [Kubernetes DownwardAPIVolumeFile](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L1489)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DownwardAPIVolumeFile {
    /// Required: Path is the relative path name of the file.
    pub path: String,
    /// Required: Selects a field of the pod.
    #[serde(rename = "fieldRef", default, skip_serializing_if = "Option::is_none")]
    pub field_ref: Option<ObjectFieldSelector>,
    /// Selects a resource of the container.
    #[serde(
        rename = "resourceFieldRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_field_ref: Option<ResourceFieldSelector>,
    /// Optional: mode bits to use on this file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<i32>,
}

/// FCVolumeSource represents a Fibre Channel volume.
///
/// Corresponds to [Kubernetes FCVolumeSource](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L1034)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct FCVolumeSource {
    /// FC target worldwide names (WWNs).
    #[serde(rename = "targetWWNs", default, skip_serializing_if = "Vec::is_empty")]
    pub target_wwns: Vec<String>,
    /// FC target lun number.
    #[serde(rename = "lun", default, skip_serializing_if = "Option::is_none")]
    pub lun: Option<i32>,
    /// Filesystem type to mount.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub fs_type: String,
    /// Defaults to false (read/write).
    #[serde(default)]
    pub read_only: bool,
    /// FC volume World Wide Identifiers (WWIDs).
    #[serde(rename = "wwids", default, skip_serializing_if = "Vec::is_empty")]
    pub wwids: Vec<String>,
}

/// AzureFileVolumeSource represents an Azure File Service mount.
///
/// Corresponds to [Kubernetes AzureFileVolumeSource](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L1517)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AzureFileVolumeSource {
    /// The name of secret that contains Azure Storage Account Name and Key.
    #[serde(rename = "secretName")]
    pub secret_name: String,
    /// Share Name.
    #[serde(rename = "shareName")]
    pub share_name: String,
    /// Defaults to false (read/write).
    #[serde(default)]
    pub read_only: bool,
}

/// ConfigMapVolumeSource adapts a ConfigMap into a volume.
///
/// Corresponds to [Kubernetes ConfigMapVolumeSource](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L1757)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ConfigMapVolumeSource {
    /// This has the same name as the ConfigMap.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Mode bits to use on created files by default.
    #[serde(
        rename = "defaultMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_mode: Option<i32>,
    /// If unspecified, each key-value pair will be projected.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<KeyToPath>,
    /// Specify whether the ConfigMap or its keys must be defined.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// VsphereVirtualDiskVolumeSource represents a vSphere volume.
///
/// Corresponds to [Kubernetes VsphereVirtualDiskVolumeSource](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L1545)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct VsphereVirtualDiskVolumeSource {
    /// Path that identifies vSphere volume vmdk.
    #[serde(rename = "volumePath")]
    pub volume_path: String,
    /// Filesystem type to mount.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub fs_type: String,
    /// Storage Policy Based Management (SPBM) profile name.
    #[serde(
        rename = "storagePolicyName",
        default,
        skip_serializing_if = "String::is_empty"
    )]
    pub storage_policy_name: String,
    /// Storage Policy Based Management (SPBM) profile ID.
    #[serde(
        rename = "storagePolicyID",
        default,
        skip_serializing_if = "String::is_empty"
    )]
    pub storage_policy_id: String,
}

/// PhotonPersistentDiskVolumeSource represents a Photon Controller disk.
///
/// Corresponds to [Kubernetes PhotonPersistentDiskVolumeSource](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L1562)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PhotonPersistentDiskVolumeSource {
    /// ID that identifies Photon Controller persistent disk.
    #[serde(rename = "pdID")]
    pub pd_id: String,
    /// Filesystem type to mount.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub fs_type: String,
}

/// PortworxVolumeSource represents a Portworx volume.
///
/// Corresponds to [Kubernetes PortworxVolumeSource](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L1572)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PortworxVolumeSource {
    /// VolumeID uniquely identifies a Portworx volume.
    #[serde(rename = "volumeID")]
    pub volume_id: String,
    /// FSType represents the filesystem type to mount.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub fs_type: String,
    /// Defaults to false (read/write).
    #[serde(default)]
    pub read_only: bool,
}

/// AzureDiskVolumeSource represents an Azure Data Disk mount.
///
/// Corresponds to [Kubernetes AzureDiskVolumeSource](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L1604)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AzureDiskVolumeSource {
    /// The Name of the data disk in the blob storage.
    #[serde(rename = "diskName")]
    pub disk_name: String,
    /// The URI of the data disk in the blob storage.
    #[serde(rename = "dataDiskURI")]
    pub data_disk_uri: String,
    /// Host Caching mode.
    #[serde(
        rename = "cachingMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub caching_mode: Option<AzureDataDiskCachingMode>,
    /// Filesystem type to mount.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<String>,
    /// Defaults to false (read/write).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// Expected values Shared, Dedicated, Managed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<AzureDataDiskKind>,
}

/// ScaleIOVolumeSource represents a persistent ScaleIO volume.
///
/// Corresponds to [Kubernetes ScaleIOVolumeSource](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L1626)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScaleIOVolumeSource {
    /// The host address of the ScaleIO API Gateway.
    pub gateway: String,
    /// The name of the storage system as configured in ScaleIO.
    pub system: String,
    /// SecretRef references to the secret for ScaleIO user.
    #[serde(rename = "secretRef", default, skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<LocalObjectReference>,
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
    /// The name of a volume already created in the ScaleIO system.
    #[serde(rename = "volumeName")]
    pub volume_name: String,
    /// Filesystem type to mount.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub fs_type: String,
    /// Defaults to false (read/write).
    #[serde(default)]
    pub read_only: bool,
}

/// StorageOSVolumeSource represents a StorageOS volume.
///
/// Corresponds to [Kubernetes StorageOSVolumeSource](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L1701)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StorageOSVolumeSource {
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
    /// SecretRef specifies the secret to use for obtaining the StorageOS API credentials.
    #[serde(rename = "secretRef", default, skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<LocalObjectReference>,
}

/// CSIVolumeSource represents a source location of a volume managed by CSI driver.
///
/// Corresponds to [Kubernetes CSIVolumeSource](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2039)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSIVolumeSource {
    /// Driver is the name of the CSI driver.
    pub driver: String,
    /// Specifies a read-only configuration for the volume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// Filesystem type to mount.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<String>,
    /// VolumeAttributes stores driver-specific properties.
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub volume_attributes: std::collections::BTreeMap<String, String>,
    /// NodePublishSecretRef is a reference to the secret object.
    #[serde(
        rename = "nodePublishSecretRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub node_publish_secret_ref: Option<LocalObjectReference>,
}

/// EphemeralVolumeSource represents a volume handled by a storage driver.
///
/// Corresponds to [Kubernetes EphemeralVolumeSource](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2071)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct EphemeralVolumeSource {
    /// VolumeClaimTemplate will be used to create a stand-alone PVC.
    #[serde(rename = "volumeClaimTemplate")]
    pub volume_claim_template: Option<PersistentVolumeClaimTemplate>,
}

/// PersistentVolumeClaimTemplate is used to produce PVC objects.
///
/// Corresponds to [Kubernetes PersistentVolumeClaimTemplate](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2097)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PersistentVolumeClaimTemplate {
    /// ObjectMeta may contain labels and annotations.
    pub metadata: ObjectMeta,
    /// Spec for the PersistentVolumeClaim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<PersistentVolumeClaimSpec>,
}
impl_has_object_meta!(PersistentVolumeClaimTemplate);

/// ImageVolumeSource represents an OCI object.
///
/// Corresponds to [Kubernetes ImageVolumeSource](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7085)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ImageVolumeSource {
    /// Required: Image or artifact reference to be used.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reference: String,
    /// Policy for pulling OCI objects.
    #[serde(rename = "pullPolicy")]
    pub pull_policy: PullPolicy,
}

/// ProjectedVolumeSource represents several volume sources.
///
/// Corresponds to [Kubernetes ProjectedVolumeSource](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L1910)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProjectedVolumeSource {
    /// list of volume projections.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<VolumeProjection>,
    /// Mode bits to use on created files by default.
    #[serde(
        rename = "defaultMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_mode: Option<i32>,
}

/// VolumeProjection represents a projected volume source.
///
/// Corresponds to [Kubernetes VolumeProjection](https://github.com/kubernetes/api/blob/master/core/v1/types.go)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct VolumeProjection {
    /// SecretProjection adapts a secret into a projected volume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<SecretProjection>,
    /// DownwardAPIProjection represents downward API info.
    #[serde(
        rename = "downwardAPI",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub downward_api: Option<DownwardAPIProjection>,
    /// ConfigMapProjection adapts a ConfigMap into a projected volume.
    #[serde(rename = "configMap", default, skip_serializing_if = "Option::is_none")]
    pub config_map: Option<ConfigMapProjection>,
    /// ServiceAccountTokenProjection represents a service account token.
    #[serde(
        rename = "serviceAccountToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_account_token: Option<ServiceAccountTokenProjection>,
}

/// SecretProjection adapts a secret into a projected volume.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SecretProjection {
    /// This has the same name as the Secret.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// If unspecified, each key-value pair will be projected.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<KeyToPath>,
    /// Specify whether the Secret or its key must be defined.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// DownwardAPIProjection represents downward API info for projecting.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DownwardAPIProjection {
    /// Items is a list of DownwardAPIVolume file.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<DownwardAPIVolumeFile>,
}

/// ConfigMapProjection adapts a ConfigMap into a projected volume.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ConfigMapProjection {
    /// This has the same name as the ConfigMap.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// If unspecified, each key-value pair will be projected.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<KeyToPath>,
    /// Specify whether the ConfigMap or its keys must be defined.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// ServiceAccountTokenProjection represents a service account token.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceAccountTokenProjection {
    /// Audience is the intended audience of the token.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub audience: String,
    /// ExpirationSeconds is the requested duration of validity.
    #[serde(rename = "expirationSeconds")]
    pub expiration_seconds: i64,
    /// Path is the path relative to the mount point.
    pub path: String,
}

/// LocalVolumeSource represents directly-attached storage with node affinity.
///
/// Corresponds to [Kubernetes LocalVolumeSource](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L1958)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct LocalVolumeSource {
    /// The full path to the volume on the node.
    pub path: String,
    /// Filesystem type to mount.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<String>,
}

#[cfg(test)]
mod tests {}
