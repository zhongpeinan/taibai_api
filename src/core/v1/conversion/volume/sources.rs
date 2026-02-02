//! Basic volume source conversions
//!
//! Includes: Volume, VolumeSource, HostPath, EmptyDir, NFS, Glusterfs, ISCSI,
//! Secret, ConfigMap, PersistentVolumeClaim

use crate::common::traits::{FromInternal, ToInternal};
use crate::core::internal::volume as internal_volume;
use crate::core::v1::selector as v1_selector;
use crate::core::v1::volume;

use super::helpers::*;

// ============================================================================
// Volume
// ============================================================================

impl ToInternal<internal_volume::Volume> for volume::Volume {
    fn to_internal(self) -> internal_volume::Volume {
        internal_volume::Volume {
            name: self.name,
            volume_source: self.volume_source.to_internal(),
        }
    }
}

impl FromInternal<internal_volume::Volume> for volume::Volume {
    fn from_internal(value: internal_volume::Volume) -> Self {
        Self {
            name: value.name,
            volume_source: volume::VolumeSource::from_internal(value.volume_source),
        }
    }
}

// ============================================================================
// VolumeSource
// ============================================================================

impl ToInternal<internal_volume::VolumeSource> for volume::VolumeSource {
    fn to_internal(self) -> internal_volume::VolumeSource {
        internal_volume::VolumeSource {
            host_path: self.host_path.map(|h| h.to_internal()),
            empty_dir: self.empty_dir.map(|e| e.to_internal()),
            gce_persistent_disk: None, // TODO: Implement GCE volume conversion
            aws_elastic_block_store: None, // TODO: Implement AWS volume conversion
            git_repo: None,            // TODO: Implement GitRepo volume conversion
            secret: self.secret.map(|s| s.to_internal()),
            nfs: self.nfs.map(|n| n.to_internal()),
            iscsi: self.iscsi.map(|i| i.to_internal()),
            glusterfs: self.glusterfs.map(|g| g.to_internal()),
            persistent_volume_claim: self.persistent_volume_claim.map(|p| p.to_internal()),
            rbd: None,         // TODO: Implement RBD volume conversion
            quobyte: None,     // TODO: Implement Quobyte volume conversion
            flex_volume: None, // TODO: Implement FlexVolume conversion
            cinder: None,      // TODO: Implement Cinder volume conversion
            ceph_fs: None,     // TODO: Implement CephFS volume conversion
            flocker: None,     // TODO: Implement Flocker volume conversion
            downward_api: self.downward_api.map(|d| d.to_internal()),
            fc: None,         // TODO: Implement FC volume conversion
            azure_file: None, // TODO: Implement AzureFile volume conversion
            config_map: self.config_map.map(|c| c.to_internal()),
            vsphere_volume: None, // TODO: Implement vSphere volume conversion
            azure_disk: None,     // TODO: Implement AzureDisk volume conversion
            photon_persistent_disk: None, // TODO: Implement PhotonPersistentDisk conversion
            projected: self.projected.map(|p| p.to_internal()),
            portworx_volume: None, // TODO: Implement Portworx volume conversion
            scale_io: None,        // TODO: Implement ScaleIO volume conversion
            storage_os: None,      // TODO: Implement StorageOS volume conversion
            csi: self.csi.map(|c| c.to_internal()),
            ephemeral: self.ephemeral.map(|e| e.to_internal()),
            image: self.image.map(|i| i.to_internal()),
        }
    }
}

impl FromInternal<internal_volume::VolumeSource> for volume::VolumeSource {
    fn from_internal(value: internal_volume::VolumeSource) -> Self {
        Self {
            host_path: value
                .host_path
                .map(volume::HostPathVolumeSource::from_internal),
            empty_dir: value
                .empty_dir
                .map(volume::EmptyDirVolumeSource::from_internal),
            gce_persistent_disk: None, // TODO: Implement GCE volume conversion
            aws_elastic_block_store: None, // TODO: Implement AWS volume conversion
            git_repo: None,            // TODO: Implement GitRepo volume conversion
            secret: value.secret.map(volume::SecretVolumeSource::from_internal),
            nfs: value.nfs.map(volume::NFSVolumeSource::from_internal),
            iscsi: value.iscsi.map(volume::ISCSIVolumeSource::from_internal),
            glusterfs: value
                .glusterfs
                .map(volume::GlusterfsVolumeSource::from_internal),
            persistent_volume_claim: value
                .persistent_volume_claim
                .map(volume::PersistentVolumeClaimVolumeSource::from_internal),
            rbd: None,         // TODO: Implement RBD volume conversion
            flex_volume: None, // TODO: Implement FlexVolume conversion
            cinder: None,      // TODO: Implement Cinder volume conversion
            cephfs: None,      // TODO: Implement CephFS volume conversion
            flocker: None,     // TODO: Implement Flocker volume conversion
            downward_api: value
                .downward_api
                .map(volume::DownwardAPIVolumeSource::from_internal),
            fc: None,         // TODO: Implement FC volume conversion
            azure_file: None, // TODO: Implement AzureFile volume conversion
            config_map: value
                .config_map
                .map(volume::ConfigMapVolumeSource::from_internal),
            vsphere_volume: None, // TODO: Implement vSphere volume conversion
            quobyte: None,        // TODO: Implement Quobyte volume conversion
            azure_disk: None,     // TODO: Implement AzureDisk volume conversion
            photon_persistent_disk: None, // TODO: Implement PhotonPersistentDisk conversion
            projected: value
                .projected
                .map(volume::ProjectedVolumeSource::from_internal),
            portworx_volume: None, // TODO: Implement Portworx volume conversion
            scale_io: None,        // TODO: Implement ScaleIO volume conversion
            storage_os: None,      // TODO: Implement StorageOS volume conversion
            csi: value.csi.map(volume::CSIVolumeSource::from_internal),
            ephemeral: value
                .ephemeral
                .map(volume::EphemeralVolumeSource::from_internal),
            image: value.image.map(volume::ImageVolumeSource::from_internal),
        }
    }
}

// ============================================================================
// Simple Volume Sources
// ============================================================================

// HostPathVolumeSource
impl ToInternal<internal_volume::HostPathVolumeSource> for volume::HostPathVolumeSource {
    fn to_internal(self) -> internal_volume::HostPathVolumeSource {
        internal_volume::HostPathVolumeSource {
            path: self.path,
            r#type: self.type_,
        }
    }
}

impl FromInternal<internal_volume::HostPathVolumeSource> for volume::HostPathVolumeSource {
    fn from_internal(value: internal_volume::HostPathVolumeSource) -> Self {
        Self {
            path: value.path,
            type_: value.r#type,
        }
    }
}

// EmptyDirVolumeSource
impl ToInternal<internal_volume::EmptyDirVolumeSource> for volume::EmptyDirVolumeSource {
    fn to_internal(self) -> internal_volume::EmptyDirVolumeSource {
        internal_volume::EmptyDirVolumeSource {
            medium: option_string_to_storage_medium(self.medium),
            size_limit: self.size_limit,
        }
    }
}

impl FromInternal<internal_volume::EmptyDirVolumeSource> for volume::EmptyDirVolumeSource {
    fn from_internal(value: internal_volume::EmptyDirVolumeSource) -> Self {
        Self {
            medium: storage_medium_to_option_string(value.medium),
            size_limit: value.size_limit,
        }
    }
}

// NFSVolumeSource
impl ToInternal<internal_volume::NFSVolumeSource> for volume::NFSVolumeSource {
    fn to_internal(self) -> internal_volume::NFSVolumeSource {
        internal_volume::NFSVolumeSource {
            server: self.server,
            path: self.path,
            read_only: self.read_only,
        }
    }
}

impl FromInternal<internal_volume::NFSVolumeSource> for volume::NFSVolumeSource {
    fn from_internal(value: internal_volume::NFSVolumeSource) -> Self {
        Self {
            server: value.server,
            path: value.path,
            read_only: value.read_only,
        }
    }
}

// GlusterfsVolumeSource
impl ToInternal<internal_volume::GlusterfsVolumeSource> for volume::GlusterfsVolumeSource {
    fn to_internal(self) -> internal_volume::GlusterfsVolumeSource {
        internal_volume::GlusterfsVolumeSource {
            endpoints_name: self.endpoints,
            path: self.path,
            read_only: self.read_only,
        }
    }
}

impl FromInternal<internal_volume::GlusterfsVolumeSource> for volume::GlusterfsVolumeSource {
    fn from_internal(value: internal_volume::GlusterfsVolumeSource) -> Self {
        Self {
            endpoints: value.endpoints_name,
            path: value.path,
            read_only: value.read_only,
        }
    }
}

// ISCSIVolumeSource
impl ToInternal<internal_volume::ISCSIVolumeSource> for volume::ISCSIVolumeSource {
    fn to_internal(self) -> internal_volume::ISCSIVolumeSource {
        internal_volume::ISCSIVolumeSource {
            target_portal: self.target_portal,
            iqn: self.iqn,
            lun: self.lun,
            iscsi_interface: self.iscsi_interface,
            fs_type: self.fs_type,
            read_only: self.read_only,
            portals: self.portals,
            chap_auth_discovery: self.chap_auth_discovery,
            chap_auth_session: self.chap_auth_session,
            secret_ref: self.secret_ref.map(|s| s.to_internal()),
            initiator_name: self.initiator_name,
        }
    }
}

impl FromInternal<internal_volume::ISCSIVolumeSource> for volume::ISCSIVolumeSource {
    fn from_internal(value: internal_volume::ISCSIVolumeSource) -> Self {
        Self {
            target_portal: value.target_portal,
            iqn: value.iqn,
            lun: value.lun,
            iscsi_interface: value.iscsi_interface,
            fs_type: value.fs_type,
            read_only: value.read_only,
            portals: value.portals,
            chap_auth_discovery: value.chap_auth_discovery,
            chap_auth_session: value.chap_auth_session,
            secret_ref: value
                .secret_ref
                .map(crate::core::v1::LocalObjectReference::from_internal),
            initiator_name: value.initiator_name,
        }
    }
}

// PersistentVolumeClaimVolumeSource
impl ToInternal<internal_volume::PersistentVolumeClaimVolumeSource>
    for volume::PersistentVolumeClaimVolumeSource
{
    fn to_internal(self) -> internal_volume::PersistentVolumeClaimVolumeSource {
        internal_volume::PersistentVolumeClaimVolumeSource {
            claim_name: self.claim_name,
            read_only: self.read_only,
        }
    }
}

impl FromInternal<internal_volume::PersistentVolumeClaimVolumeSource>
    for volume::PersistentVolumeClaimVolumeSource
{
    fn from_internal(value: internal_volume::PersistentVolumeClaimVolumeSource) -> Self {
        Self {
            claim_name: value.claim_name,
            read_only: value.read_only,
        }
    }
}

// ============================================================================
// Secret/ConfigMap Volume Sources
// ============================================================================

// KeyToPath
impl ToInternal<internal_volume::KeyToPath> for volume::KeyToPath {
    fn to_internal(self) -> internal_volume::KeyToPath {
        internal_volume::KeyToPath {
            key: self.key,
            path: self.path,
            mode: self.mode,
        }
    }
}

impl FromInternal<internal_volume::KeyToPath> for volume::KeyToPath {
    fn from_internal(value: internal_volume::KeyToPath) -> Self {
        Self {
            key: value.key,
            path: value.path,
            mode: value.mode,
        }
    }
}

// SecretVolumeSource
impl ToInternal<internal_volume::SecretVolumeSource> for volume::SecretVolumeSource {
    fn to_internal(self) -> internal_volume::SecretVolumeSource {
        internal_volume::SecretVolumeSource {
            secret_name: self.secret_name.unwrap_or_default(),
            items: self.items.into_iter().map(|i| i.to_internal()).collect(),
            default_mode: self.default_mode,
            optional: self.optional,
        }
    }
}

impl FromInternal<internal_volume::SecretVolumeSource> for volume::SecretVolumeSource {
    fn from_internal(value: internal_volume::SecretVolumeSource) -> Self {
        Self {
            secret_name: if value.secret_name.is_empty() {
                None
            } else {
                Some(value.secret_name)
            },
            items: value
                .items
                .into_iter()
                .map(volume::KeyToPath::from_internal)
                .collect(),
            default_mode: value.default_mode,
            optional: value.optional,
        }
    }
}

// ConfigMapVolumeSource
impl ToInternal<internal_volume::ConfigMapVolumeSource> for volume::ConfigMapVolumeSource {
    fn to_internal(self) -> internal_volume::ConfigMapVolumeSource {
        internal_volume::ConfigMapVolumeSource {
            name: self.name,
            items: self.items.into_iter().map(|i| i.to_internal()).collect(),
            default_mode: self.default_mode,
            optional: self.optional,
        }
    }
}

impl FromInternal<internal_volume::ConfigMapVolumeSource> for volume::ConfigMapVolumeSource {
    fn from_internal(value: internal_volume::ConfigMapVolumeSource) -> Self {
        Self {
            name: value.name,
            items: value
                .items
                .into_iter()
                .map(volume::KeyToPath::from_internal)
                .collect(),
            default_mode: value.default_mode,
            optional: value.optional,
        }
    }
}

// ============================================================================
// DownwardAPI Volume Source
// ============================================================================

// DownwardAPIVolumeSource
impl ToInternal<internal_volume::DownwardAPIVolumeSource> for volume::DownwardAPIVolumeSource {
    fn to_internal(self) -> internal_volume::DownwardAPIVolumeSource {
        internal_volume::DownwardAPIVolumeSource {
            items: self.items.into_iter().map(|i| i.to_internal()).collect(),
            default_mode: self.default_mode,
        }
    }
}

impl FromInternal<internal_volume::DownwardAPIVolumeSource> for volume::DownwardAPIVolumeSource {
    fn from_internal(value: internal_volume::DownwardAPIVolumeSource) -> Self {
        Self {
            items: value
                .items
                .into_iter()
                .map(volume::DownwardAPIVolumeFile::from_internal)
                .collect(),
            default_mode: value.default_mode,
        }
    }
}

// DownwardAPIVolumeFile
impl ToInternal<internal_volume::DownwardAPIVolumeFile> for volume::DownwardAPIVolumeFile {
    fn to_internal(self) -> internal_volume::DownwardAPIVolumeFile {
        internal_volume::DownwardAPIVolumeFile {
            path: self.path,
            field_ref: self.field_ref.map(|f| f.to_internal()),
            resource_field_ref: self.resource_field_ref.map(|r| r.to_internal()),
            mode: self.mode,
        }
    }
}

impl FromInternal<internal_volume::DownwardAPIVolumeFile> for volume::DownwardAPIVolumeFile {
    fn from_internal(value: internal_volume::DownwardAPIVolumeFile) -> Self {
        Self {
            path: value.path,
            field_ref: value
                .field_ref
                .map(v1_selector::ObjectFieldSelector::from_internal),
            resource_field_ref: value
                .resource_field_ref
                .map(v1_selector::ResourceFieldSelector::from_internal),
            mode: value.mode,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::internal;

    #[test]
    fn test_empty_dir_roundtrip() {
        let v1_empty_dir = volume::EmptyDirVolumeSource {
            medium: Some("Memory".to_string()),
            size_limit: Some(crate::common::util::Quantity("1Gi".to_string())),
        };

        let internal_empty_dir = v1_empty_dir.clone().to_internal();
        assert!(matches!(
            internal_empty_dir.medium,
            internal::StorageMedium::Memory
        ));
        assert_eq!(
            internal_empty_dir.size_limit,
            Some(crate::common::util::Quantity("1Gi".to_string()))
        );

        let roundtrip = volume::EmptyDirVolumeSource::from_internal(internal_empty_dir);
        assert_eq!(roundtrip.medium, v1_empty_dir.medium);
        assert_eq!(roundtrip.size_limit, v1_empty_dir.size_limit);
    }

    #[test]
    fn test_secret_volume_source_roundtrip() {
        let v1_secret = volume::SecretVolumeSource {
            secret_name: Some("my-secret".to_string()),
            items: vec![],
            default_mode: Some(0o644),
            optional: Some(true),
        };

        let internal_secret = v1_secret.clone().to_internal();
        assert_eq!(internal_secret.secret_name, "my-secret");
        assert_eq!(internal_secret.default_mode, Some(0o644));
        assert_eq!(internal_secret.optional, Some(true));

        let roundtrip = volume::SecretVolumeSource::from_internal(internal_secret);
        assert_eq!(roundtrip, v1_secret);
    }

    #[test]
    fn test_volume_roundtrip() {
        let v1_volume = volume::Volume {
            name: "data-volume".to_string(),
            volume_source: volume::VolumeSource {
                empty_dir: Some(volume::EmptyDirVolumeSource {
                    medium: None,
                    size_limit: None,
                }),
                ..Default::default()
            },
        };

        let internal_volume = v1_volume.clone().to_internal();
        assert_eq!(internal_volume.name, "data-volume");
        assert!(internal_volume.volume_source.empty_dir.is_some());

        let roundtrip = volume::Volume::from_internal(internal_volume);
        assert_eq!(roundtrip.name, v1_volume.name);
        assert!(roundtrip.volume_source.empty_dir.is_some());
    }
}
