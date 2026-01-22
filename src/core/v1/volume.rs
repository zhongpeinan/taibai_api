//! Kubernetes Volume types
//!
//! This module contains volume-related types from the Kubernetes core/v1 API.

use std::collections::BTreeMap;
use crate::impl_versioned_object;

use crate::common::meta::ObjectMeta;
use crate::core::v1::persistent_volume::PersistentVolumeClaimSpec;
use crate::core::v1::reference::LocalObjectReference;
use serde::{Deserialize, Serialize};

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
    #[serde(default, skip_serializing_if = "Option::is_none")]
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale_io: Option<serde_json::Value>,
    /// storageOS represents a StorageOS volume attached and mounted on Kubernetes nodes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
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
    #[serde(default)]
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
    #[serde(default)]
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
    #[serde(default)]
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
    #[serde(default)]
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
    pub field_ref: Option<serde_json::Value>,
    /// Selects a resource of the container: only resources limits and requests
    /// (limits.cpu, limits.memory, requests.cpu and requests.memory) are currently supported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_field_ref: Option<serde_json::Value>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
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
    pub label_selector: Option<serde_json::Value>,
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
    #[serde(default)]
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
    #[serde(default)]
    pub read_only: bool,
    /// RecursiveReadOnly is the mode of the recursive read-only mount.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recursive_read_only: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::util::Quantity;
    use serde_json;

    #[test]
    fn test_volume_serialization() {
        let volume = Volume {
            name: "my-volume".to_string(),
            volume_source: VolumeSource {
                empty_dir: Some(EmptyDirVolumeSource {
                    medium: Some(storage_medium::MEMORY.to_string()),
                    size_limit: Some(Quantity::from("1Gi")),
                }),
                ..Default::default()
            },
        };

        let json = serde_json::to_string(&volume).unwrap();
        let deserialized: Volume = serde_json::from_str(&json).unwrap();

        assert_eq!(volume, deserialized);
    }

    #[test]
    fn test_host_path_volume_source() {
        let host_path = HostPathVolumeSource {
            path: "/var/log".to_string(),
            type_: Some(host_path_type::DIRECTORY.to_string()),
        };

        let json = serde_json::to_string(&host_path).unwrap();
        assert!(json.contains("Directory"));

        let deserialized: HostPathVolumeSource = serde_json::from_str(&json).unwrap();
        assert_eq!(host_path.path, deserialized.path);
        assert_eq!(host_path.type_, deserialized.type_);
    }

    #[test]
    fn test_host_path_type_constants() {
        assert_eq!(host_path_type::UNSET, "");
        assert_eq!(host_path_type::DIRECTORY_OR_CREATE, "DirectoryOrCreate");
        assert_eq!(host_path_type::DIRECTORY, "Directory");
        assert_eq!(host_path_type::FILE_OR_CREATE, "FileOrCreate");
        assert_eq!(host_path_type::FILE, "File");
        assert_eq!(host_path_type::SOCKET, "Socket");
        assert_eq!(host_path_type::CHAR_DEVICE, "CharDevice");
        assert_eq!(host_path_type::BLOCK_DEVICE, "BlockDevice");
    }

    #[test]
    fn test_storage_medium_constants() {
        assert_eq!(storage_medium::DEFAULT, "");
        assert_eq!(storage_medium::MEMORY, "Memory");
        assert_eq!(storage_medium::HUGE_PAGES, "HugePages");
        assert_eq!(storage_medium::HUGE_PAGES_PREFIX, "HugePages-");
    }

    #[test]
    fn test_empty_dir_volume_source() {
        let empty_dir = EmptyDirVolumeSource {
            medium: Some(storage_medium::MEMORY.to_string()),
            size_limit: Some(Quantity::from("1Gi")),
        };

        let json = serde_json::to_string(&empty_dir).unwrap();
        let deserialized: EmptyDirVolumeSource = serde_json::from_str(&json).unwrap();

        assert_eq!(empty_dir.medium, deserialized.medium);
        assert_eq!(empty_dir.size_limit, deserialized.size_limit);
    }

    #[test]
    fn test_secret_volume_source() {
        let secret = SecretVolumeSource {
            secret_name: Some("my-secret".to_string()),
            items: vec![],
            default_mode: Some(0o644),
            optional: Some(false),
        };

        let json = serde_json::to_string(&secret).unwrap();
        let deserialized: SecretVolumeSource = serde_json::from_str(&json).unwrap();

        assert_eq!(secret.secret_name, deserialized.secret_name);
        assert_eq!(secret.default_mode, deserialized.default_mode);
    }

    #[test]
    fn test_config_map_volume_source() {
        let config_map = ConfigMapVolumeSource {
            name: Some("my-config".to_string()),
            items: vec![],
            default_mode: Some(0o644),
            optional: Some(false),
        };

        let json = serde_json::to_string(&config_map).unwrap();
        let deserialized: ConfigMapVolumeSource = serde_json::from_str(&json).unwrap();

        assert_eq!(config_map.name, deserialized.name);
        assert_eq!(config_map.default_mode, deserialized.default_mode);
    }

    #[test]
    fn test_downward_api_volume_file() {
        let file = DownwardAPIVolumeFile {
            path: "labels".to_string(),
            field_ref: Some(serde_json::json!({"fieldPath": "metadata.labels"})),
            resource_field_ref: None,
            mode: Some(0o644),
        };

        let json = serde_json::to_string(&file).unwrap();
        let deserialized: DownwardAPIVolumeFile = serde_json::from_str(&json).unwrap();

        assert_eq!(file.path, deserialized.path);
        assert_eq!(file.mode, deserialized.mode);
    }

    #[test]
    fn test_projected_volume_source() {
        let projected = ProjectedVolumeSource {
            sources: vec![VolumeProjection {
                secret: Some(SecretProjection {
                    name: Some("my-secret".to_string()),
                    items: vec![],
                    optional: None,
                }),
                ..Default::default()
            }],
            default_mode: Some(0o644),
        };

        let json = serde_json::to_string(&projected).unwrap();
        let deserialized: ProjectedVolumeSource = serde_json::from_str(&json).unwrap();

        assert_eq!(projected.sources.len(), deserialized.sources.len());
    }

    #[test]
    fn test_key_to_path() {
        let key_to_path = KeyToPath {
            key: "my-key".to_string(),
            path: "my-path".to_string(),
            mode: Some(0o644),
        };

        let json = serde_json::to_string(&key_to_path).unwrap();
        let deserialized: KeyToPath = serde_json::from_str(&json).unwrap();

        assert_eq!(key_to_path.key, deserialized.key);
        assert_eq!(key_to_path.path, deserialized.path);
    }

    #[test]
    fn test_csi_volume_source() {
        let csi = CSIVolumeSource {
            driver: "csi.example.com".to_string(),
            read_only: Some(false),
            fs_type: Some("ext4".to_string()),
            volume_attributes: BTreeMap::new(),
            node_publish_secret_ref: None,
        };

        let json = serde_json::to_string(&csi).unwrap();
        let deserialized: CSIVolumeSource = serde_json::from_str(&json).unwrap();

        assert_eq!(csi.driver, deserialized.driver);
        assert_eq!(csi.fs_type, deserialized.fs_type);
    }

    #[test]
    fn test_volume_mount() {
        let mount = VolumeMount {
            name: "my-volume".to_string(),
            read_only: false,
            mount_path: "/mnt/data".to_string(),
            sub_path: "subdir".to_string(),
            mount_propagation: Some(mount_propagation_mode::HOST_TO_CONTAINER.to_string()),
            sub_path_expr: String::new(),
            recursive_read_only: None,
        };

        let json = serde_json::to_string(&mount).unwrap();
        let deserialized: VolumeMount = serde_json::from_str(&json).unwrap();

        assert_eq!(mount.name, deserialized.name);
        assert_eq!(mount.mount_path, deserialized.mount_path);
    }

    #[test]
    fn test_mount_propagation_mode_constants() {
        assert_eq!(mount_propagation_mode::NONE, "None");
        assert_eq!(mount_propagation_mode::HOST_TO_CONTAINER, "HostToContainer");
        assert_eq!(mount_propagation_mode::BIDIRECTIONAL, "Bidirectional");
    }

    #[test]
    fn test_recursive_read_only_mode_constants() {
        assert_eq!(recursive_read_only_mode::DISABLED, "Disabled");
        assert_eq!(recursive_read_only_mode::IF_POSSIBLE, "IfPossible");
        assert_eq!(recursive_read_only_mode::ENABLED, "Enabled");
    }

    #[test]
    fn test_pull_policy_constants() {
        assert_eq!(pull_policy::ALWAYS, "Always");
        assert_eq!(pull_policy::NEVER, "Never");
        assert_eq!(pull_policy::IF_NOT_PRESENT, "IfNotPresent");
    }

    #[test]
    fn test_volume_device() {
        let device = VolumeDevice {
            name: "my-volume".to_string(),
            device_path: "/dev/xvda".to_string(),
        };

        let json = serde_json::to_string(&device).unwrap();
        let deserialized: VolumeDevice = serde_json::from_str(&json).unwrap();

        assert_eq!(device.name, deserialized.name);
        assert_eq!(device.device_path, deserialized.device_path);
    }

    #[test]
    fn test_nfs_volume_source() {
        let nfs = NFSVolumeSource {
            server: "192.168.1.1".to_string(),
            path: "/exports/data".to_string(),
            read_only: true,
        };

        let json = serde_json::to_string(&nfs).unwrap();
        let deserialized: NFSVolumeSource = serde_json::from_str(&json).unwrap();

        assert_eq!(nfs.server, deserialized.server);
        assert_eq!(nfs.read_only, deserialized.read_only);
    }

    #[test]
    fn test_glusterfs_volume_source() {
        let glusterfs = GlusterfsVolumeSource {
            endpoints: "glusterfs-cluster".to_string(),
            path: "/vol".to_string(),
            read_only: false,
        };

        let json = serde_json::to_string(&glusterfs).unwrap();
        let deserialized: GlusterfsVolumeSource = serde_json::from_str(&json).unwrap();

        assert_eq!(glusterfs.endpoints, deserialized.endpoints);
        assert_eq!(glusterfs.path, deserialized.path);
    }

    #[test]
    fn test_iscsi_volume_source() {
        let iscsi = ISCSIVolumeSource {
            target_portal: "192.168.1.1:3260".to_string(),
            iqn: "iqn.2001-04.com.example:storage".to_string(),
            lun: 0,
            iscsi_interface: "default".to_string(),
            fs_type: "ext4".to_string(),
            read_only: false,
            portals: vec![],
            chap_auth_discovery: false,
            chap_auth_session: false,
            secret_ref: None,
            initiator_name: None,
        };

        let json = serde_json::to_string(&iscsi).unwrap();
        let deserialized: ISCSIVolumeSource = serde_json::from_str(&json).unwrap();

        assert_eq!(iscsi.target_portal, deserialized.target_portal);
        assert_eq!(iscsi.iqn, deserialized.iqn);
        assert_eq!(iscsi.lun, deserialized.lun);
    }

    #[test]
    fn test_persistent_volume_claim_volume_source() {
        let pvc_source = PersistentVolumeClaimVolumeSource {
            claim_name: "my-claim".to_string(),
            read_only: false,
        };

        let json = serde_json::to_string(&pvc_source).unwrap();
        let deserialized: PersistentVolumeClaimVolumeSource = serde_json::from_str(&json).unwrap();

        assert_eq!(pvc_source.claim_name, deserialized.claim_name);
        assert_eq!(pvc_source.read_only, deserialized.read_only);
    }

    #[test]
    fn test_image_volume_source() {
        let image = ImageVolumeSource {
            reference: "nginx:latest".to_string(),
            pull_policy: Some(pull_policy::ALWAYS.to_string()),
        };

        let json = serde_json::to_string(&image).unwrap();
        let deserialized: ImageVolumeSource = serde_json::from_str(&json).unwrap();

        assert_eq!(image.reference, deserialized.reference);
        assert_eq!(image.pull_policy, deserialized.pull_policy);
    }

    #[test]
    fn test_ephemeral_volume_source() {
        let ephemeral = EphemeralVolumeSource {
            volume_claim_template: None,
        };

        let json = serde_json::to_string(&ephemeral).unwrap();
        let deserialized: EphemeralVolumeSource = serde_json::from_str(&json).unwrap();

        assert_eq!(
            ephemeral.volume_claim_template,
            deserialized.volume_claim_template
        );
    }

    #[test]
    fn test_persistent_volume_claim_template() {
        let template = PersistentVolumeClaimTemplate {
            metadata: None,
            spec: None,
        };

        let json = serde_json::to_string(&template).unwrap();
        let deserialized: PersistentVolumeClaimTemplate = serde_json::from_str(&json).unwrap();

        assert_eq!(template.metadata, deserialized.metadata);
        assert_eq!(template.spec, deserialized.spec);
    }

    #[test]
    fn test_volume_round_trip() {
        let volume = Volume {
            name: "test-volume".to_string(),
            volume_source: VolumeSource {
                host_path: Some(HostPathVolumeSource {
                    path: "/tmp".to_string(),
                    type_: Some(host_path_type::DIRECTORY.to_string()),
                }),
                ..Default::default()
            },
        };

        let json = serde_json::to_string(&volume).unwrap();
        let deserialized: Volume = serde_json::from_str(&json).unwrap();

        assert_eq!(volume.name, deserialized.name);
        assert!(deserialized.volume_source.host_path.is_some());
    }

    #[test]
    fn test_service_account_token_projection() {
        let token = ServiceAccountTokenProjection {
            audience: "api".to_string(),
            expiration_seconds: Some(3600),
            path: "token".to_string(),
        };

        let json = serde_json::to_string(&token).unwrap();
        let deserialized: ServiceAccountTokenProjection = serde_json::from_str(&json).unwrap();

        assert_eq!(token.audience, deserialized.audience);
        assert_eq!(token.expiration_seconds, deserialized.expiration_seconds);
        assert_eq!(token.path, deserialized.path);
    }

    #[test]
    fn test_cluster_trust_bundle_projection() {
        let ctb = ClusterTrustBundleProjection {
            name: Some("my-bundle".to_string()),
            signer_name: None,
            label_selector: None,
            optional: Some(false),
            path: "bundle.pem".to_string(),
        };

        let json = serde_json::to_string(&ctb).unwrap();
        let deserialized: ClusterTrustBundleProjection = serde_json::from_str(&json).unwrap();

        assert_eq!(ctb.name, deserialized.name);
        assert_eq!(ctb.path, deserialized.path);
    }

    #[test]
    fn test_pod_certificate_projection() {
        let cert = PodCertificateProjection {
            signer_name: "kubernetes.io/kubelet-serving".to_string(),
            key_type: "RSA2048".to_string(),
            max_expiration_seconds: Some(86400),
            credential_bundle_path: Some("credentials.pem".to_string()),
            key_path: None,
            certificate_chain_path: None,
            user_annotations: BTreeMap::new(),
        };

        let json = serde_json::to_string(&cert).unwrap();
        let deserialized: PodCertificateProjection = serde_json::from_str(&json).unwrap();

        assert_eq!(cert.signer_name, deserialized.signer_name);
        assert_eq!(cert.key_type, deserialized.key_type);
    }

    #[test]
    fn test_volume_mount_status() {
        let status = VolumeMountStatus {
            name: "my-volume".to_string(),
            mount_path: "/mnt/data".to_string(),
            read_only: false,
            recursive_read_only: Some(recursive_read_only_mode::DISABLED.to_string()),
        };

        let json = serde_json::to_string(&status).unwrap();
        let deserialized: VolumeMountStatus = serde_json::from_str(&json).unwrap();

        assert_eq!(status.name, deserialized.name);
        assert_eq!(status.mount_path, deserialized.mount_path);
        assert_eq!(status.read_only, deserialized.read_only);
    }

    // LocalVolumeSource tests
    #[test]
    fn test_local_volume_source_default() {
        let volume = LocalVolumeSource::default();
        assert!(volume.path.is_empty());
        assert!(volume.fs_type.is_none());
    }

    #[test]
    fn test_local_volume_source_with_fields() {
        let volume = LocalVolumeSource {
            path: "/mnt/data".to_string(),
            fs_type: Some("ext4".to_string()),
        };
        assert_eq!(volume.path, "/mnt/data");
        assert_eq!(volume.fs_type, Some("ext4".to_string()));
    }

    #[test]
    fn test_local_volume_source_serialize() {
        let volume = LocalVolumeSource {
            path: "/var/lib/kubelet/pods/1234/volumes/kubernetes.io~local-volume/local-pv"
                .to_string(),
            fs_type: Some("xfs".to_string()),
        };
        let json = serde_json::to_string(&volume).unwrap();
        assert!(json.contains(
            "\"path\":\"/var/lib/kubelet/pods/1234/volumes/kubernetes.io~local-volume/local-pv\""
        ));
        assert!(json.contains("\"fsType\":\"xfs\""));
    }

    #[test]
    fn test_local_volume_source_deserialize() {
        let json = r#"{"path":"/mnt/data","fsType":"ext4"}"#;
        let volume: LocalVolumeSource = serde_json::from_str(json).unwrap();
        assert_eq!(volume.path, "/mnt/data");
        assert_eq!(volume.fs_type, Some("ext4".to_string()));
    }

    #[test]
    fn test_local_volume_source_round_trip() {
        let original = LocalVolumeSource {
            path: "/mnt/disks/ssd1".to_string(),
            fs_type: None,
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: LocalVolumeSource = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_local_volume_source_without_fs_type() {
        let volume = LocalVolumeSource {
            path: "/mnt/data".to_string(),
            fs_type: None,
        };
        let json = serde_json::to_string(&volume).unwrap();
        // fsType should be omitted when None
        assert!(!json.contains("fsType"));
        assert!(json.contains("\"path\":\"/mnt/data\""));
    }
}
