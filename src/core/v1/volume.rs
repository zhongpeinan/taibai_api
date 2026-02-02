//! Kubernetes Volume types
//!
//! This module contains volume-related types from the Kubernetes core/v1 API.

use crate::common::ApplyDefault;
use crate::common::meta::{LabelSelector, ObjectMeta};
use crate::core::v1::persistent_volume::PersistentVolumeClaimSpec;
use crate::core::v1::reference::LocalObjectReference;
use crate::core::v1::selector::{ObjectFieldSelector, ResourceFieldSelector};
use crate::impl_versioned_object;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Volume represents a named volume in a pod that may be accessed by any container in the pod.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct Volume {
    /// name of the volume.
    /// Must be a DNS_LABEL and unique within the pod.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// volumeSource represents the location and type of the mounted volume.
    #[serde(flatten)]
    pub volume_source: VolumeSource,
}

/// Represents the source of a volume to mount.
/// Only one of its members may be specified.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct VolumeSource {
    /// hostPath represents a pre-existing file or directory on the host
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_path: Option<HostPathVolumeSource>,
    /// emptyDir represents a temporary directory that shares a pod's lifetime.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub empty_dir: Option<EmptyDirVolumeSource>,
    /// gcePersistentDisk represents a GCE Disk resource that is attached to the
    /// kubelet's host machine and then exposed to the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gce_persistent_disk: Option<serde_json::Value>,
    /// awsElasticBlockStore represents an AWS Disk resource that is attached to a
    /// kubelet's host machine and then exposed to the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aws_elastic_block_store: Option<serde_json::Value>,
    /// gitRepo represents a git repository at a particular revision.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub git_repo: Option<serde_json::Value>,
    /// secret represents a secret that should populate this volume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<SecretVolumeSource>,
    /// nfs represents an NFS mount on the host that shares a pod's lifetime
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nfs: Option<NFSVolumeSource>,
    /// iscsi represents an ISCSI Disk resource that is attached to a
    /// kubelet's host machine and then exposed to the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iscsi: Option<ISCSIVolumeSource>,
    /// glusterfs represents a Glusterfs mount on the host that shares a pod's lifetime.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub glusterfs: Option<GlusterfsVolumeSource>,
    /// persistentVolumeClaimVolumeSource represents a reference to a
    /// PersistentVolumeClaim in the same namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub persistent_volume_claim: Option<PersistentVolumeClaimVolumeSource>,
    /// rbd represents a Rados Block Device mount on the host that shares a pod's lifetime.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rbd: Option<serde_json::Value>,
    /// flexVolume represents a generic volume resource that is
    /// provisioned/attached using an exec based plugin.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flex_volume: Option<serde_json::Value>,
    /// cinder represents a cinder volume attached and mounted on kubelets host machine.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cinder: Option<serde_json::Value>,
    /// cephFS represents a Ceph FS mount on the host that shares a pod's lifetime.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cephfs: Option<serde_json::Value>,
    /// flocker represents a Flocker volume attached to a kubelet's host machine.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flocker: Option<serde_json::Value>,
    /// downwardAPI represents downward API about the pod that should populate this volume
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "downwardAPI"
    )]
    pub downward_api: Option<DownwardAPIVolumeSource>,
    /// fc represents a Fibre Channel resource that is attached to a kubelet's host machine.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fc: Option<serde_json::Value>,
    /// azureFile represents an Azure File Service mount on the host and bind mount to the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub azure_file: Option<serde_json::Value>,
    /// configMap represents a configMap that should populate this volume
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config_map: Option<ConfigMapVolumeSource>,
    /// vsphereVolume represents a vSphere volume attached and mounted on kubelets host machine.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vsphere_volume: Option<serde_json::Value>,
    /// quobyte represents a Quobyte mount on the host that shares a pod's lifetime.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quobyte: Option<serde_json::Value>,
    /// azureDisk represents an Azure Data Disk mount on the host and bind mount to the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub azure_disk: Option<serde_json::Value>,
    /// photonPersistentDisk represents a PhotonController persistent disk.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub photon_persistent_disk: Option<serde_json::Value>,
    /// projected items for all in one resources secrets, configmaps, and downward API
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub projected: Option<ProjectedVolumeSource>,
    /// portworxVolume represents a portworx volume attached and mounted on kubelets host machine.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub portworx_volume: Option<serde_json::Value>,
    /// scaleIO represents a ScaleIO persistent volume attached and mounted on Kubernetes nodes.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scaleIO")]
    pub scale_io: Option<serde_json::Value>,
    /// storageOS represents a StorageOS volume attached and mounted on Kubernetes nodes.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageos")]
    pub storage_os: Option<serde_json::Value>,
    /// csi (Container Storage Interface) represents ephemeral storage that is handled by certain external CSI drivers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub csi: Option<CSIVolumeSource>,
    /// ephemeral represents a volume that is handled by a cluster storage driver.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ephemeral: Option<EphemeralVolumeSource>,
    /// image represents an OCI object (a container image or artifact) pulled and mounted on the kubelet's host machine.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<ImageVolumeSource>,
}

/// HostPathType string
pub type HostPathType = String;

/// Constants for HostPathType
pub mod host_path_type {
    pub const UNSET: &str = "";
    pub const DIRECTORY_OR_CREATE: &str = "DirectoryOrCreate";
    pub const DIRECTORY: &str = "Directory";
    pub const FILE_OR_CREATE: &str = "FileOrCreate";
    pub const FILE: &str = "File";
    pub const SOCKET: &str = "Socket";
    pub const CHAR_DEVICE: &str = "CharDevice";
    pub const BLOCK_DEVICE: &str = "BlockDevice";
}

/// Represents a host path mapped into a pod.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct HostPathVolumeSource {
    /// path of the directory on the host.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub path: String,
    /// type for HostPath Volume
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// StorageMedium defines ways that storage can be allocated to a volume.
pub type StorageMedium = String;

/// Constants for StorageMedium
pub mod storage_medium {
    /// use whatever the default is for the node
    pub const DEFAULT: &str = "";
    /// use memory (e.g. tmpfs on linux)
    pub const MEMORY: &str = "Memory";
    /// use hugepages
    pub const HUGE_PAGES: &str = "HugePages";
    /// prefix for full medium notation HugePages-<size>
    pub const HUGE_PAGES_PREFIX: &str = "HugePages-";
}

/// Represents an empty directory for a pod.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct EmptyDirVolumeSource {
    /// medium represents what type of storage medium should back this directory.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub medium: Option<String>,
    /// sizeLimit is the total amount of local storage required for this EmptyDir volume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size_limit: Option<crate::common::util::Quantity>,
}

/// Represents a Glusterfs mount that lasts the lifetime of a pod.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GlusterfsVolumeSource {
    /// endpoints is the endpoint name that details Glusterfs topology.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub endpoints: String,
    /// path is the Glusterfs volume path.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub path: String,
    /// readOnly here will force the Glusterfs volume to be mounted with read-only permissions.
    #[serde(default, skip_serializing_if = "crate::common::is_false")]
    pub read_only: bool,
}

/// PersistentVolumeClaimVolumeSource references the user's PVC in the same namespace.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PersistentVolumeClaimVolumeSource {
    /// claimName is the name of a PersistentVolumeClaim in the same namespace as the pod using this volume.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub claim_name: String,
    /// readOnly Will force the ReadOnly setting in VolumeMounts.
    #[serde(default, skip_serializing_if = "crate::common::is_false")]
    pub read_only: bool,
}

/// Represents an ISCSI disk.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ISCSIVolumeSource {
    /// targetPortal is iSCSI Target Portal.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub target_portal: String,
    /// iqn is the target iSCSI Qualified Name.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub iqn: String,
    /// lun represents iSCSI Target Lun number.
    #[serde(default)]
    pub lun: i32,
    /// iscsiInterface is the interface Name that uses an iSCSI transport.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub iscsi_interface: String,
    /// fsType is the filesystem type of the volume that you want to mount.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub fs_type: String,
    /// readOnly here will force the ReadOnly setting in VolumeMounts.
    #[serde(default, skip_serializing_if = "crate::common::is_false")]
    pub read_only: bool,
    /// portals is the iSCSI Target Portal List.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub portals: Vec<String>,
    /// chapAuthDiscovery defines whether support iSCSI Discovery CHAP authentication
    #[serde(default)]
    pub chap_auth_discovery: bool,
    /// chapAuthSession defines whether support iSCSI Session CHAP authentication
    #[serde(default)]
    pub chap_auth_session: bool,
    /// secretRef is the CHAP Secret for iSCSI target and initiator authentication
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<LocalObjectReference>,
    /// initiatorName is the custom iSCSI Initiator Name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initiator_name: Option<String>,
}

/// Represents an NFS mount that lasts the lifetime of a pod.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NFSVolumeSource {
    /// server is the hostname or IP address of the NFS server.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub server: String,
    /// path that is exported by the NFS server.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub path: String,
    /// readOnly here will force the NFS export to be mounted with read-only permissions.
    #[serde(default, skip_serializing_if = "crate::common::is_false")]
    pub read_only: bool,
}

/// SecretVolumeSource adapts a Secret into a volume.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SecretVolumeSource {
    /// secretName is the name of the secret in the pod's namespace to use.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_name: Option<String>,
    /// items If unspecified, each key-value pair in the Data field of the referenced
    /// Secret will be projected into the volume as a file whose name is the
    /// key and content is the value.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<KeyToPath>,
    /// defaultMode is optional: mode bits used to set permissions on created files by default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_mode: Option<i32>,
    /// optional field specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// ConfigMapVolumeSource adapts a ConfigMap into a volume.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ConfigMapVolumeSource {
    /// name of the configMap in the pod's namespace to use.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// items if unspecified, each key-value pair in the Data field of the referenced
    /// ConfigMap will be projected into the volume as a file whose name is the
    /// key and content is the value.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<KeyToPath>,
    /// defaultMode is optional: mode bits used to set permissions on created files by default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_mode: Option<i32>,
    /// optional specify whether the ConfigMap or its keys must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// DownwardAPIVolumeSource represents a volume containing downward API info.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DownwardAPIVolumeSource {
    /// Items is a list of DownwardAPIVolume file
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<DownwardAPIVolumeFile>,
    /// DefaultMode is the mode bits for created files by default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_mode: Option<i32>,
}

/// DownwardAPIVolumeFile represents information to create the file containing the pod field
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct DownwardAPIVolumeFile {
    /// Path is  the relative path name of the file to be created.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub path: String,
    /// Required: Selects a field of the pod: only annotations, labels, name, namespace and uid are supported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_ref: Option<ObjectFieldSelector>,
    /// Selects a resource of the container: only resources limits and requests
    /// (limits.cpu, limits.memory, requests.cpu and requests.memory) are currently supported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_field_ref: Option<ResourceFieldSelector>,
    /// Optional: mode bits used to set permissions on this file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<i32>,
}

/// Represents a projected volume source
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProjectedVolumeSource {
    /// sources is the list of volume projections.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<VolumeProjection>,
    /// defaultMode are the mode bits used to set permissions on created files by default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_mode: Option<i32>,
}

/// Projection that may be projected along with other supported volume types.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct VolumeProjection {
    /// secret information about the secret data to project
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<SecretProjection>,
    /// downwardAPI information about the downwardAPI data to project
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "downwardAPI"
    )]
    pub downward_api: Option<DownwardAPIProjection>,
    /// configMap information about the configMap data to project
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config_map: Option<ConfigMapProjection>,
    /// serviceAccountToken is information about the serviceAccountToken data to project
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_account_token: Option<ServiceAccountTokenProjection>,
    /// ClusterTrustBundle allows a pod to access the `.spec.trustBundle` field
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster_trust_bundle: Option<ClusterTrustBundleProjection>,
    /// Projects an auto-rotating credential bundle
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pod_certificate: Option<PodCertificateProjection>,
}

/// Adapts a secret into a projected volume.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SecretProjection {
    /// Name of the secret in the pod's namespace to use.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// items if unspecified, each key-value pair in the Data field of the referenced
    /// Secret will be projected into the volume as a file whose name is the
    /// key and content is the value.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<KeyToPath>,
    /// optional field specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// Adapts a ConfigMap into a projected volume.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ConfigMapProjection {
    /// Name of the configMap in the pod's namespace to use.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// items if unspecified, each key-value pair in the Data field of the referenced
    /// ConfigMap will be projected into the volume as a file whose name is the
    /// key and content is the value.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<KeyToPath>,
    /// optional specify whether the ConfigMap or its keys must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// Represents downward API info for projecting into a projected volume.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DownwardAPIProjection {
    /// Items is a list of DownwardAPIVolume file
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<DownwardAPIVolumeFile>,
}

/// ServiceAccountTokenProjection represents a projected service account token volume.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct ServiceAccountTokenProjection {
    /// audience is the intended audience of the token.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub audience: String,
    /// expirationSeconds is the requested duration of validity of the service account token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_seconds: Option<i64>,
    /// path is the path relative to the mount point of the file to project the token into.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub path: String,
}

/// ClusterTrustBundleProjection describes how to select a set of ClusterTrustBundle objects.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct ClusterTrustBundleProjection {
    /// Select a single ClusterTrustBundle by object name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Select all ClusterTrustBundles that match this signer name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signer_name: Option<String>,
    /// Select all ClusterTrustBundles that match this label selector.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label_selector: Option<LabelSelector>,
    /// If true, don't block pod startup if the referenced ClusterTrustBundle(s) aren't available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
    /// Relative path from the volume root to write the bundle.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub path: String,
}

/// PodCertificateProjection provides a private key and X.509 certificate in the pod filesystem.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct PodCertificateProjection {
    /// Kubelet's generated CSRs will be addressed to this signer.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub signer_name: String,
    /// The type of keypair Kubelet will generate for the pod.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub key_type: String,
    /// maxExpirationSeconds is the maximum lifetime permitted for the certificate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_expiration_seconds: Option<i32>,
    /// Write the credential bundle at this path in the projected volume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential_bundle_path: Option<String>,
    /// Write the key at this path in the projected volume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_path: Option<String>,
    /// Write the certificate chain at this path in the projected volume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate_chain_path: Option<String>,
    /// userAnnotations allow pod authors to pass additional information to the signer implementation.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub user_annotations: BTreeMap<String, String>,
}

/// Maps a string key to a path within a volume.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct KeyToPath {
    /// key is the key to project.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub key: String,
    /// path is the relative path of the file to map the key to.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub path: String,
    /// mode is Optional: mode bits used to set permissions on this file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<i32>,
}

/// Represents a source location of a volume to mount, managed by an external CSI driver
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSIVolumeSource {
    /// driver is the name of the CSI driver that handles this volume.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub driver: String,
    /// readOnly specifies a read-only configuration for the volume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// fsType to mount. Ex. "ext4", "xfs", "ntfs".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<String>,
    /// volumeAttributes stores driver-specific properties that are passed to the CSI driver.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub volume_attributes: BTreeMap<String, String>,
    /// nodePublishSecretRef is a reference to the secret object containing sensitive information.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_publish_secret_ref: Option<LocalObjectReference>,
}

/// Represents an ephemeral volume that is handled by a normal storage driver.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct EphemeralVolumeSource {
    /// Will be used to create a stand-alone PVC to provision the volume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume_claim_template: Option<PersistentVolumeClaimTemplate>,
}

/// PersistentVolumeClaimTemplate is used to produce PersistentVolumeClaim objects.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct PersistentVolumeClaimTemplate {
    /// May contain labels and annotations that will be copied into the PVC.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// The specification for the PersistentVolumeClaim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<PersistentVolumeClaimSpec>,
}
impl_versioned_object!(PersistentVolumeClaimTemplate);

/// ImageVolumeSource represents a image volume resource.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ImageVolumeSource {
    /// Required: Image or artifact reference to be used.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reference: String,
    /// Policy for pulling OCI objects.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull_policy: Option<String>,
}

/// PullPolicy describes a policy for if/when to pull a container image
pub type PullPolicy = String;

/// LocalVolumeSource represents directly-attached storage with node affinity.
///
/// Corresponds to [Kubernetes LocalVolumeSource](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L1959)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct LocalVolumeSource {
    /// The full path to the volume on the node.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub path: String,

    /// Filesystem type to mount. It applies only when the Path is a block device.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<String>,
}

/// Constants for PullPolicy
pub mod pull_policy {
    pub const ALWAYS: &str = "Always";
    pub const NEVER: &str = "Never";
    pub const IF_NOT_PRESENT: &str = "IfNotPresent";
}

/// VolumeMount describes a mounting of a Volume within a container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct VolumeMount {
    /// This must match the Name of a Volume.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// Mounted read-only if true, read-write otherwise (false or unspecified).
    #[serde(default, skip_serializing_if = "crate::common::is_false")]
    pub read_only: bool,
    /// Path within the container at which the volume should be mounted.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub mount_path: String,
    /// Path within the volume from which the container's volume should be mounted.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub sub_path: String,
    /// mountPropagation determines how mounts are propagated from the host to container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mount_propagation: Option<String>,
    /// Expanded path within the volume from which the container's volume should be mounted.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub sub_path_expr: String,
    /// RecursiveReadOnly specifies whether read-only mounts should be handled recursively.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recursive_read_only: Option<String>,
}

/// MountPropagationMode describes mount propagation.
pub type MountPropagationMode = String;

/// Constants for MountPropagationMode
pub mod mount_propagation_mode {
    pub const NONE: &str = "None";
    pub const HOST_TO_CONTAINER: &str = "HostToContainer";
    pub const BIDIRECTIONAL: &str = "Bidirectional";
}

/// RecursiveReadOnlyMode describes recursive-readonly mode.
pub type RecursiveReadOnlyMode = String;

/// Constants for RecursiveReadOnlyMode
pub mod recursive_read_only_mode {
    pub const DISABLED: &str = "Disabled";
    pub const IF_POSSIBLE: &str = "IfPossible";
    pub const ENABLED: &str = "Enabled";
}

/// volumeDevice describes a mapping of a raw block device within a container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct VolumeDevice {
    /// name must match the name of a persistentVolumeClaim in the pod
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// devicePath is the path inside of the container that the device will be mapped to.
    #[serde(default, skip_serializing_if = "String::is_empty")]
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
    #[serde(default, skip_serializing_if = "crate::common::is_false")]
    pub read_only: bool,
    /// RecursiveReadOnly is the mode of the recursive read-only mount.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recursive_read_only: Option<String>,
}

#[cfg(test)]
mod tests {}

// ============================================================================
// Defaults
// ============================================================================

const DEFAULT_VOLUME_MODE: i32 = 0o644;
const DEFAULT_SERVICE_ACCOUNT_TOKEN_EXPIRATION_SECONDS: i64 = 3600;

/// Apply defaults to a list of volumes.
pub(crate) fn apply_volume_defaults(volumes: &mut [Volume]) {
    for volume in volumes {
        volume.apply_default();
    }
}

impl ApplyDefault for Volume {
    fn apply_default(&mut self) {
        if volume_source_is_empty(&self.volume_source) {
            self.volume_source.empty_dir = Some(Default::default());
        }
        self.volume_source.apply_default();
    }
}

impl ApplyDefault for VolumeSource {
    fn apply_default(&mut self) {
        if let Some(ref mut host_path) = self.host_path {
            host_path.apply_default();
        }
        if let Some(ref mut secret) = self.secret {
            secret.apply_default();
        }
        if let Some(ref mut iscsi) = self.iscsi {
            if iscsi.iscsi_interface.is_empty() {
                iscsi.iscsi_interface = "default".to_string();
            }
        }
        if let Some(ref mut downward_api) = self.downward_api {
            downward_api.apply_default();
        }
        if let Some(ref mut config_map) = self.config_map {
            config_map.apply_default();
        }
        if let Some(ref mut projected) = self.projected {
            projected.apply_default();
        }
        if let Some(ref mut ephemeral) = self.ephemeral {
            ephemeral.apply_default();
        }
        if let Some(ref mut image) = self.image {
            image.apply_default();
        }
    }
}

impl ApplyDefault for HostPathVolumeSource {
    fn apply_default(&mut self) {
        if self.type_.is_none() {
            self.type_ = Some(host_path_type::UNSET.to_string());
        }
    }
}

impl ApplyDefault for SecretVolumeSource {
    fn apply_default(&mut self) {
        if self.default_mode.is_none() {
            self.default_mode = Some(DEFAULT_VOLUME_MODE);
        }
    }
}

impl ApplyDefault for ConfigMapVolumeSource {
    fn apply_default(&mut self) {
        if self.default_mode.is_none() {
            self.default_mode = Some(DEFAULT_VOLUME_MODE);
        }
    }
}

impl ApplyDefault for DownwardAPIVolumeSource {
    fn apply_default(&mut self) {
        if self.default_mode.is_none() {
            self.default_mode = Some(DEFAULT_VOLUME_MODE);
        }
        for item in &mut self.items {
            if let Some(ref mut field_ref) = item.field_ref {
                field_ref.apply_default();
            }
        }
    }
}

impl ApplyDefault for ProjectedVolumeSource {
    fn apply_default(&mut self) {
        if self.default_mode.is_none() {
            self.default_mode = Some(DEFAULT_VOLUME_MODE);
        }
        for source in &mut self.sources {
            source.apply_default();
        }
    }
}

impl ApplyDefault for VolumeProjection {
    fn apply_default(&mut self) {
        if let Some(ref mut downward_api) = self.downward_api {
            for item in &mut downward_api.items {
                if let Some(ref mut field_ref) = item.field_ref {
                    field_ref.apply_default();
                }
            }
        }
        if let Some(ref mut token) = self.service_account_token {
            token.apply_default();
        }
    }
}

impl ApplyDefault for ServiceAccountTokenProjection {
    fn apply_default(&mut self) {
        if self.expiration_seconds.is_none() {
            self.expiration_seconds = Some(DEFAULT_SERVICE_ACCOUNT_TOKEN_EXPIRATION_SECONDS);
        }
    }
}

impl ApplyDefault for EphemeralVolumeSource {
    fn apply_default(&mut self) {
        if let Some(ref mut template) = self.volume_claim_template {
            if let Some(ref mut spec) = template.spec {
                spec.apply_default();
            }
        }
    }
}

impl ApplyDefault for ImageVolumeSource {
    fn apply_default(&mut self) {
        let pull_policy = self.pull_policy.as_deref().unwrap_or("");
        if pull_policy.is_empty() {
            let is_latest = image_tag_or_latest(&self.reference) == "latest";
            self.pull_policy = Some(if is_latest {
                "Always".to_string()
            } else {
                "IfNotPresent".to_string()
            });
        }
    }
}

fn volume_source_is_empty(source: &VolumeSource) -> bool {
    source.host_path.is_none()
        && source.empty_dir.is_none()
        && source.gce_persistent_disk.is_none()
        && source.aws_elastic_block_store.is_none()
        && source.git_repo.is_none()
        && source.secret.is_none()
        && source.nfs.is_none()
        && source.iscsi.is_none()
        && source.glusterfs.is_none()
        && source.persistent_volume_claim.is_none()
        && source.rbd.is_none()
        && source.flex_volume.is_none()
        && source.cinder.is_none()
        && source.cephfs.is_none()
        && source.flocker.is_none()
        && source.downward_api.is_none()
        && source.fc.is_none()
        && source.azure_file.is_none()
        && source.config_map.is_none()
        && source.vsphere_volume.is_none()
        && source.quobyte.is_none()
        && source.azure_disk.is_none()
        && source.photon_persistent_disk.is_none()
        && source.projected.is_none()
        && source.portworx_volume.is_none()
        && source.scale_io.is_none()
        && source.storage_os.is_none()
        && source.csi.is_none()
        && source.ephemeral.is_none()
        && source.image.is_none()
}

fn image_tag_or_latest(image: &str) -> &str {
    let (name, _) = image.split_once('@').unwrap_or((image, ""));
    let last_slash = name.rfind('/');
    if let Some(colon) = name.rfind(':') {
        if last_slash.map(|slash| colon > slash).unwrap_or(true) {
            let tag = &name[colon + 1..];
            if !tag.is_empty() {
                return tag;
            }
        }
    }
    "latest"
}
