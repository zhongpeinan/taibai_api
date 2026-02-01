//! Volume validation for core internal API

use crate::common::validation::{
    BadValue, ErrorList, Path, forbidden, invalid, is_dns1123_label, is_dns1123_subdomain,
    not_found, not_supported, required,
};
use crate::core::internal::InternalContainer as Container;
use crate::core::internal::{
    CSIVolumeSource, ClusterTrustBundleProjection, ConfigMapProjection, ConfigMapVolumeSource,
    DownwardAPIProjection, DownwardAPIVolumeFile, DownwardAPIVolumeSource, EphemeralVolumeSource,
    GlusterfsVolumeSource, HostPathVolumeSource, ISCSIVolumeSource, ImageVolumeSource,
    NFSVolumeSource, PersistentVolumeClaimVolumeSource, PodCertificateProjection,
    ProjectedVolumeSource, SecretProjection, SecretVolumeSource, ServiceAccountTokenProjection,
    Volume, VolumeProjection, VolumeSource,
};
use crate::core::v1::volume::{KeyToPath, VolumeDevice, VolumeMount};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

use crate::core::v1::validation::constants::*;
use crate::core::v1::validation::helpers::*;

// ============================================================================
// Public API
// ============================================================================

/// Validates a list of volumes
///
/// Returns a map of volume names to their sources for reference by volume mounts.
pub fn validate_volumes(
    volumes: &[Volume],
    path: &Path,
) -> (HashMap<String, VolumeSource>, ErrorList) {
    let mut all_errs = ErrorList::new();
    let mut all_names = HashSet::new();
    let mut volume_map = HashMap::new();

    // Check volume count limit
    if volumes.len() > MAX_VOLUMES_PER_POD {
        all_errs.push(crate::common::validation::too_many(
            path,
            Some(volumes.len()),
            MAX_VOLUMES_PER_POD,
        ));
    }

    for (i, vol) in volumes.iter().enumerate() {
        let idx_path = path.index(i);
        let name_path = idx_path.child("name");

        // Validate volume name
        let mut vol_errs = ErrorList::new();
        if vol.name.is_empty() {
            vol_errs.push(required(&name_path, "name is required"));
        } else {
            for msg in is_dns1123_label(&vol.name) {
                vol_errs.push(invalid(
                    &name_path,
                    BadValue::String(vol.name.clone()),
                    &msg,
                ));
            }
        }

        // Check for duplicate names
        if all_names.contains(&vol.name) {
            vol_errs.push(crate::common::validation::duplicate(
                &name_path,
                BadValue::String(vol.name.clone()),
            ));
        }

        // Validate volume source
        vol_errs.extend(validate_volume_source(
            &vol.volume_source,
            &idx_path,
            &vol.name,
        ));

        // If no errors, add to map and name set
        if vol_errs.is_empty() {
            all_names.insert(vol.name.clone());
            volume_map.insert(vol.name.clone(), vol.volume_source.clone());
        } else {
            all_errs.extend(vol_errs);
        }
    }

    (volume_map, all_errs)
}

/// Validates a single volume
pub fn validate_volume(volume: &Volume, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate name
    if volume.name.is_empty() {
        all_errs.push(required(&path.child("name"), "name is required"));
    } else {
        all_errs.extend(validate_dns1123_label(&volume.name, &path.child("name")));
    }

    // Validate volume source
    all_errs.extend(validate_volume_source(
        &volume.volume_source,
        path,
        &volume.name,
    ));

    all_errs
}

/// Validates that exactly one volume source is specified
fn validate_volume_source(source: &VolumeSource, path: &Path, vol_name: &str) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let mut num_volumes = 0;

    // EmptyDir
    if let Some(ref empty_dir) = source.empty_dir {
        num_volumes += 1;
        all_errs.extend(validate_empty_dir_volume_source(
            empty_dir,
            &path.child("emptyDir"),
        ));
    }

    // HostPath
    if let Some(ref host_path) = source.host_path {
        if num_volumes > 0 {
            all_errs.push(forbidden(
                &path.child("hostPath"),
                "may not specify more than 1 volume type",
            ));
        } else {
            num_volumes += 1;
            all_errs.extend(validate_host_path_volume_source(
                host_path,
                &path.child("hostPath"),
            ));
        }
    }

    // GCEPersistentDisk
    if source.gce_persistent_disk.is_some() {
        if num_volumes > 0 {
            all_errs.push(forbidden(
                &path.child("gcePersistentDisk"),
                "may not specify more than 1 volume type",
            ));
        } else {
            num_volumes += 1;
            // TODO: Add specific validation when type is fully defined
        }
    }

    // AWSElasticBlockStore
    if source.aws_elastic_block_store.is_some() {
        if num_volumes > 0 {
            all_errs.push(forbidden(
                &path.child("awsElasticBlockStore"),
                "may not specify more than 1 volume type",
            ));
        } else {
            num_volumes += 1;
            // TODO: Add specific validation when type is fully defined
        }
    }

    // GitRepo
    if source.git_repo.is_some() {
        if num_volumes > 0 {
            all_errs.push(forbidden(
                &path.child("gitRepo"),
                "may not specify more than 1 volume type",
            ));
        } else {
            num_volumes += 1;
            // TODO: Add specific validation when type is fully defined
        }
    }

    // Secret
    if let Some(ref secret) = source.secret {
        if num_volumes > 0 {
            all_errs.push(forbidden(
                &path.child("secret"),
                "may not specify more than 1 volume type",
            ));
        } else {
            num_volumes += 1;
            all_errs.extend(validate_secret_volume_source(secret, &path.child("secret")));
        }
    }

    // NFS
    if let Some(ref nfs) = source.nfs {
        if num_volumes > 0 {
            all_errs.push(forbidden(
                &path.child("nfs"),
                "may not specify more than 1 volume type",
            ));
        } else {
            num_volumes += 1;
            all_errs.extend(validate_nfs_volume_source(nfs, &path.child("nfs")));
        }
    }

    // ISCSI
    if let Some(ref iscsi) = source.iscsi {
        if num_volumes > 0 {
            all_errs.push(forbidden(
                &path.child("iscsi"),
                "may not specify more than 1 volume type",
            ));
        } else {
            num_volumes += 1;
            all_errs.extend(validate_iscsi_volume_source(iscsi, &path.child("iscsi")));

            // Special validation: volume name + target portal length
            if iscsi.initiator_name.is_some() {
                let combined_len = vol_name.len() + 1 + iscsi.target_portal.len(); // +1 for ':'
                if combined_len > 64 {
                    all_errs.push(invalid(
                        &path.child("name"),
                        BadValue::String(vol_name.to_string()),
                        "Total length of <volume name>:<iscsi.targetPortal> must be under 64 characters if iscsi.initiatorName is specified",
                    ));
                }
            }
        }
    }

    // Glusterfs
    if let Some(ref glusterfs) = source.glusterfs {
        if num_volumes > 0 {
            all_errs.push(forbidden(
                &path.child("glusterfs"),
                "may not specify more than 1 volume type",
            ));
        } else {
            num_volumes += 1;
            all_errs.extend(validate_glusterfs_volume_source(
                glusterfs,
                &path.child("glusterfs"),
            ));
        }
    }

    // PersistentVolumeClaim
    if let Some(ref pvc) = source.persistent_volume_claim {
        if num_volumes > 0 {
            all_errs.push(forbidden(
                &path.child("persistentVolumeClaim"),
                "may not specify more than 1 volume type",
            ));
        } else {
            num_volumes += 1;
            all_errs.extend(validate_persistent_volume_claim_volume_source(
                pvc,
                &path.child("persistentVolumeClaim"),
            ));
        }
    }

    // RBD
    if source.rbd.is_some() {
        if num_volumes > 0 {
            all_errs.push(forbidden(
                &path.child("rbd"),
                "may not specify more than 1 volume type",
            ));
        } else {
            num_volumes += 1;
            // TODO: Add specific validation when type is fully defined
        }
    }

    // FlexVolume
    if source.flex_volume.is_some() {
        if num_volumes > 0 {
            all_errs.push(forbidden(
                &path.child("flexVolume"),
                "may not specify more than 1 volume type",
            ));
        } else {
            num_volumes += 1;
            // TODO: Add specific validation when type is fully defined
        }
    }

    // Cinder
    if source.cinder.is_some() {
        if num_volumes > 0 {
            all_errs.push(forbidden(
                &path.child("cinder"),
                "may not specify more than 1 volume type",
            ));
        } else {
            num_volumes += 1;
            // TODO: Add specific validation when type is fully defined
        }
    }

    // CephFS
    if source.ceph_fs.is_some() {
        if num_volumes > 0 {
            all_errs.push(forbidden(
                &path.child("cephFS"),
                "may not specify more than 1 volume type",
            ));
        } else {
            num_volumes += 1;
            // TODO: Add specific validation when type is fully defined
        }
    }

    // Flocker
    if source.flocker.is_some() {
        if num_volumes > 0 {
            all_errs.push(forbidden(
                &path.child("flocker"),
                "may not specify more than 1 volume type",
            ));
        } else {
            num_volumes += 1;
            // TODO: Add specific validation when type is fully defined
        }
    }

    // DownwardAPI
    if let Some(ref downward_api) = source.downward_api {
        if num_volumes > 0 {
            all_errs.push(forbidden(
                &path.child("downwardAPI"),
                "may not specify more than 1 volume type",
            ));
        } else {
            num_volumes += 1;
            all_errs.extend(validate_downward_api_volume_source(
                downward_api,
                &path.child("downwardAPI"),
            ));
        }
    }

    // FC
    if source.fc.is_some() {
        if num_volumes > 0 {
            all_errs.push(forbidden(
                &path.child("fc"),
                "may not specify more than 1 volume type",
            ));
        } else {
            num_volumes += 1;
            // TODO: Add specific validation when type is fully defined
        }
    }

    // AzureFile
    if source.azure_file.is_some() {
        if num_volumes > 0 {
            all_errs.push(forbidden(
                &path.child("azureFile"),
                "may not specify more than 1 volume type",
            ));
        } else {
            num_volumes += 1;
            // TODO: Add specific validation when type is fully defined
        }
    }

    // ConfigMap
    if let Some(ref config_map) = source.config_map {
        if num_volumes > 0 {
            all_errs.push(forbidden(
                &path.child("configMap"),
                "may not specify more than 1 volume type",
            ));
        } else {
            num_volumes += 1;
            all_errs.extend(validate_config_map_volume_source(
                config_map,
                &path.child("configMap"),
            ));
        }
    }

    // VsphereVolume
    if source.vsphere_volume.is_some() {
        if num_volumes > 0 {
            all_errs.push(forbidden(
                &path.child("vsphereVolume"),
                "may not specify more than 1 volume type",
            ));
        } else {
            num_volumes += 1;
            // TODO: Add specific validation when type is fully defined
        }
    }

    // Quobyte
    if source.quobyte.is_some() {
        if num_volumes > 0 {
            all_errs.push(forbidden(
                &path.child("quobyte"),
                "may not specify more than 1 volume type",
            ));
        } else {
            num_volumes += 1;
            // TODO: Add specific validation when type is fully defined
        }
    }

    // AzureDisk
    if source.azure_disk.is_some() {
        if num_volumes > 0 {
            all_errs.push(forbidden(
                &path.child("azureDisk"),
                "may not specify more than 1 volume type",
            ));
        } else {
            num_volumes += 1;
            // TODO: Add specific validation when type is fully defined
        }
    }

    // PhotonPersistentDisk
    if source.photon_persistent_disk.is_some() {
        if num_volumes > 0 {
            all_errs.push(forbidden(
                &path.child("photonPersistentDisk"),
                "may not specify more than 1 volume type",
            ));
        } else {
            num_volumes += 1;
            // TODO: Add specific validation when type is fully defined
        }
    }

    // PortworxVolume
    if source.portworx_volume.is_some() {
        if num_volumes > 0 {
            all_errs.push(forbidden(
                &path.child("portworxVolume"),
                "may not specify more than 1 volume type",
            ));
        } else {
            num_volumes += 1;
            // TODO: Add specific validation when type is fully defined
        }
    }

    // ScaleIO
    if source.scale_io.is_some() {
        if num_volumes > 0 {
            all_errs.push(forbidden(
                &path.child("scaleIO"),
                "may not specify more than 1 volume type",
            ));
        } else {
            num_volumes += 1;
            // TODO: Add specific validation when type is fully defined
        }
    }

    // StorageOS
    if source.storage_os.is_some() {
        if num_volumes > 0 {
            all_errs.push(forbidden(
                &path.child("storageos"),
                "may not specify more than 1 volume type",
            ));
        } else {
            num_volumes += 1;
            // TODO: Add specific validation when type is fully defined
        }
    }

    // Projected
    if let Some(ref projected) = source.projected {
        if num_volumes > 0 {
            all_errs.push(forbidden(
                &path.child("projected"),
                "may not specify more than 1 volume type",
            ));
        } else {
            num_volumes += 1;
            all_errs.extend(validate_projected_volume_source(
                projected,
                &path.child("projected"),
            ));
        }
    }

    // CSI
    if let Some(ref csi) = source.csi {
        if num_volumes > 0 {
            all_errs.push(forbidden(
                &path.child("csi"),
                "may not specify more than 1 volume type",
            ));
        } else {
            num_volumes += 1;
            all_errs.extend(validate_csi_volume_source(csi, &path.child("csi")));
        }
    }

    // Ephemeral
    if let Some(ref ephemeral) = source.ephemeral {
        if num_volumes > 0 {
            all_errs.push(forbidden(
                &path.child("ephemeral"),
                "may not specify more than 1 volume type",
            ));
        } else {
            num_volumes += 1;
            all_errs.extend(validate_ephemeral_volume_source(
                ephemeral,
                &path.child("ephemeral"),
            ));
        }
    }

    // Image
    if let Some(ref image) = source.image {
        if num_volumes > 0 {
            all_errs.push(forbidden(
                &path.child("image"),
                "may not specify more than 1 volume type",
            ));
        } else {
            num_volumes += 1;
            all_errs.extend(validate_image_volume_source(image, &path.child("image")));
        }
    }

    // Must have exactly one volume source
    if num_volumes == 0 {
        all_errs.push(required(path, "must specify a volume type"));
    }

    all_errs
}

// ============================================================================
// Volume Source Validators
// ============================================================================

fn validate_empty_dir_volume_source(
    empty_dir: &crate::core::internal::volume::EmptyDirVolumeSource,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate size limit if specified
    if let Some(ref size_limit) = empty_dir.size_limit {
        // Size limit must be non-negative
        // TODO: Add Quantity validation when available
        if size_limit.0.starts_with('-') {
            all_errs.push(forbidden(
                &path.child("sizeLimit"),
                "sizeLimit field must be a valid resource quantity",
            ));
        }
    }

    all_errs
}

fn validate_host_path_volume_source(host_path: &HostPathVolumeSource, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Path is required
    if host_path.path.is_empty() {
        all_errs.push(required(&path.child("path"), "path is required"));
    } else {
        all_errs.extend(validate_path_no_backsteps(
            &host_path.path,
            &path.child("path"),
        ));
    }

    // Validate type if specified
    if let Some(ref type_) = host_path.r#type {
        if !SUPPORTED_HOST_PATH_TYPES.contains(type_.as_str()) {
            all_errs.push(crate::common::validation::not_supported(
                &path.child("type"),
                BadValue::String(type_.clone()),
                &SUPPORTED_HOST_PATH_TYPES
                    .iter()
                    .copied()
                    .collect::<Vec<_>>(),
            ));
        }
    }

    all_errs
}

fn validate_secret_volume_source(secret: &SecretVolumeSource, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // secretName is required (or at least should be set)
    if secret.secret_name.is_empty() {
        all_errs.push(required(
            &path.child("secretName"),
            "secretName is required",
        ));
    } else {
        for msg in is_dns1123_subdomain(&secret.secret_name) {
            all_errs.push(invalid(
                &path.child("secretName"),
                BadValue::String(secret.secret_name.clone()),
                &msg,
            ));
        }
    }

    // Validate items (KeyToPath)
    all_errs.extend(validate_key_to_paths(&secret.items, &path.child("items")));

    // Validate defaultMode if specified
    if let Some(mode) = secret.default_mode {
        all_errs.extend(validate_file_mode(mode, &path.child("defaultMode")));
    }

    all_errs
}

fn validate_config_map_volume_source(config_map: &ConfigMapVolumeSource, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // name is required (or at least should be set)
    if let Some(ref name) = config_map.name {
        if name.is_empty() {
            all_errs.push(required(&path.child("name"), "name is required"));
        } else {
            for msg in is_dns1123_subdomain(name) {
                all_errs.push(invalid(
                    &path.child("name"),
                    BadValue::String(name.clone()),
                    &msg,
                ));
            }
        }
    } else {
        all_errs.push(required(&path.child("name"), "name is required"));
    }

    // Validate items (KeyToPath)
    all_errs.extend(validate_key_to_paths(
        &config_map.items,
        &path.child("items"),
    ));

    // Validate defaultMode if specified
    if let Some(mode) = config_map.default_mode {
        all_errs.extend(validate_file_mode(mode, &path.child("defaultMode")));
    }

    all_errs
}

fn validate_nfs_volume_source(nfs: &NFSVolumeSource, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Server is required
    if nfs.server.is_empty() {
        all_errs.push(required(&path.child("server"), "server is required"));
    }

    // Path is required
    if nfs.path.is_empty() {
        all_errs.push(required(&path.child("path"), "path is required"));
    } else if !nfs.path.starts_with('/') {
        all_errs.push(invalid(
            &path.child("path"),
            BadValue::String(nfs.path.clone()),
            "must be an absolute path",
        ));
    }

    all_errs
}

fn validate_iscsi_volume_source(iscsi: &ISCSIVolumeSource, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // targetPortal is required
    if iscsi.target_portal.is_empty() {
        all_errs.push(required(
            &path.child("targetPortal"),
            "targetPortal is required",
        ));
    }

    // iqn is required
    if iscsi.iqn.is_empty() {
        all_errs.push(required(&path.child("iqn"), "iqn is required"));
    } else {
        all_errs.extend(validate_iscsi_qualified_name(
            &iscsi.iqn,
            &path.child("iqn"),
        ));
    }

    // lun must be between 0 and 255
    if iscsi.lun < 0 || iscsi.lun > 255 {
        all_errs.push(invalid(
            &path.child("lun"),
            BadValue::Int(iscsi.lun as i64),
            "must be between 0 and 255",
        ));
    }

    if (iscsi.chap_auth_discovery || iscsi.chap_auth_session) && iscsi.secret_ref.is_none() {
        all_errs.push(required(&path.child("secretRef"), "secretRef is required"));
    }

    if let Some(ref secret_ref) = iscsi.secret_ref {
        if secret_ref.name.as_deref().unwrap_or("").is_empty() {
            all_errs.push(required(
                &path.child("secretRef").child("name"),
                "name is required",
            ));
        } else {
            for msg in is_dns1123_subdomain(secret_ref.name.as_deref().unwrap_or("")) {
                all_errs.push(invalid(
                    &path.child("secretRef").child("name"),
                    BadValue::String(secret_ref.name.as_deref().unwrap_or("").to_string()),
                    &msg,
                ));
            }
        }
    }

    if let Some(ref initiator) = iscsi.initiator_name {
        all_errs.extend(validate_iscsi_qualified_name(
            initiator,
            &path.child("initiatorName"),
        ));
    }

    all_errs
}

fn validate_glusterfs_volume_source(glusterfs: &GlusterfsVolumeSource, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // endpointsName is required
    if glusterfs.endpoints_name.is_empty() {
        all_errs.push(required(
            &path.child("endpointsName"),
            "endpointsName is required",
        ));
    }

    // path is required
    if glusterfs.path.is_empty() {
        all_errs.push(required(&path.child("path"), "path is required"));
    }

    all_errs
}

fn validate_persistent_volume_claim_volume_source(
    pvc: &PersistentVolumeClaimVolumeSource,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // claimName is required
    if pvc.claim_name.is_empty() {
        all_errs.push(required(&path.child("claimName"), "claimName is required"));
    }

    all_errs
}

fn validate_downward_api_volume_source(
    downward_api: &DownwardAPIVolumeSource,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate items
    for (i, item) in downward_api.items.iter().enumerate() {
        all_errs.extend(validate_downward_api_volume_file(
            item,
            &path.child("items").index(i),
        ));
    }

    // Validate defaultMode if specified
    if let Some(mode) = downward_api.default_mode {
        all_errs.extend(validate_file_mode(mode, &path.child("defaultMode")));
    }

    all_errs
}

fn validate_downward_api_volume_file(file: &DownwardAPIVolumeFile, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // path is required
    if file.path.is_empty() {
        all_errs.push(required(&path.child("path"), "path is required"));
    } else {
        // path must be a local descending path (no .., no absolute)
        all_errs.extend(validate_local_descending_path(
            &file.path,
            &path.child("path"),
        ));
    }

    // Must have exactly one source: fieldRef or resourceFieldRef
    let mut num_sources = 0;

    if file.field_ref.is_some() {
        num_sources += 1;
        if let Some(ref field_ref) = file.field_ref {
            all_errs.extend(validate_volume_object_field_selector(
                field_ref,
                &path.child("fieldRef"),
            ));
        }
    }

    if file.resource_field_ref.is_some() {
        num_sources += 1;
        if let Some(ref resource_field_ref) = file.resource_field_ref {
            all_errs.extend(validate_container_resource_field_selector(
                resource_field_ref,
                &path.child("resourceFieldRef"),
                true,
            ));
        }
    }

    if num_sources == 0 {
        all_errs.push(required(
            path,
            "must specify one of: fieldRef, resourceFieldRef",
        ));
    } else if num_sources > 1 {
        all_errs.push(forbidden(
            path,
            "may not specify more than one of: fieldRef, resourceFieldRef",
        ));
    }

    // Validate mode if specified
    if let Some(mode) = file.mode {
        all_errs.extend(validate_file_mode(mode, &path.child("mode")));
    }

    all_errs
}

fn validate_projected_volume_source(projected: &ProjectedVolumeSource, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let mut all_paths = HashSet::new();

    if projected.sources.is_empty() {
        all_errs.push(required(&path.child("sources"), "sources is required"));
        return all_errs;
    }

    // Validate sources
    for (i, source) in projected.sources.iter().enumerate() {
        all_errs.extend(validate_volume_projection(
            source,
            &path.child("sources").index(i),
            &mut all_paths,
        ));
    }

    // Validate defaultMode if specified
    if let Some(mode) = projected.default_mode {
        all_errs.extend(validate_file_mode(mode, &path.child("defaultMode")));
    }

    all_errs
}

fn validate_volume_projection(
    projection: &VolumeProjection,
    path: &Path,
    all_paths: &mut HashSet<String>,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let mut num_sources = 0;

    // Secret
    if let Some(ref secret) = projection.secret {
        num_sources += 1;
        all_errs.extend(validate_secret_projection(
            secret,
            &path.child("secret"),
            all_paths,
        ));
    }

    // DownwardAPI
    if let Some(ref downward_api) = projection.downward_api {
        if num_sources > 0 {
            all_errs.push(forbidden(
                &path.child("downwardAPI"),
                "may not specify more than 1 projection type",
            ));
        } else {
            num_sources += 1;
            all_errs.extend(validate_downward_api_projection(
                downward_api,
                &path.child("downwardAPI"),
                all_paths,
            ));
        }
    }

    // ConfigMap
    if let Some(ref config_map) = projection.config_map {
        if num_sources > 0 {
            all_errs.push(forbidden(
                &path.child("configMap"),
                "may not specify more than 1 projection type",
            ));
        } else {
            num_sources += 1;
            all_errs.extend(validate_config_map_projection(
                config_map,
                &path.child("configMap"),
                all_paths,
            ));
        }
    }

    // ServiceAccountToken
    if let Some(ref sa_token) = projection.service_account_token {
        if num_sources > 0 {
            all_errs.push(forbidden(
                &path.child("serviceAccountToken"),
                "may not specify more than 1 projection type",
            ));
        } else {
            num_sources += 1;
            all_errs.extend(validate_service_account_token_projection(
                sa_token,
                &path.child("serviceAccountToken"),
                all_paths,
            ));
        }
    }

    // ClusterTrustBundle
    if let Some(ref ctb) = projection.cluster_trust_bundle {
        if num_sources > 0 {
            all_errs.push(forbidden(
                &path.child("clusterTrustBundle"),
                "may not specify more than 1 projection type",
            ));
        } else {
            num_sources += 1;
            all_errs.extend(validate_cluster_trust_bundle_projection(
                ctb,
                &path.child("clusterTrustBundle"),
                all_paths,
            ));
        }
    }

    // PodCertificate
    if let Some(ref pod_cert) = projection.pod_certificate {
        if num_sources > 0 {
            all_errs.push(forbidden(
                &path.child("podCertificate"),
                "may not specify more than 1 projection type",
            ));
        } else {
            num_sources += 1;
            all_errs.extend(validate_pod_certificate_projection(
                pod_cert,
                &path.child("podCertificate"),
                all_paths,
            ));
        }
    }

    // Must have exactly one source
    if num_sources == 0 {
        all_errs.push(required(path, "must specify a projection type"));
    }

    all_errs
}

fn validate_secret_projection(
    secret: &SecretProjection,
    path: &Path,
    all_paths: &mut HashSet<String>,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if secret.name.as_deref().unwrap_or("").is_empty() {
        all_errs.push(required(&path.child("name"), "name is required"));
    } else {
        for msg in is_dns1123_subdomain(secret.name.as_deref().unwrap_or("")) {
            all_errs.push(invalid(
                &path.child("name"),
                BadValue::String(secret.name.as_deref().unwrap_or("").to_string()),
                &msg,
            ));
        }
    }

    // Validate items
    all_errs.extend(validate_key_to_paths(&secret.items, &path.child("items")));
    for item in &secret.items {
        if !item.path.is_empty() && !all_paths.insert(item.path.clone()) {
            all_errs.push(invalid(
                path,
                BadValue::String(item.path.clone()),
                "conflicting duplicate paths",
            ));
        }
    }

    all_errs
}

fn validate_downward_api_projection(
    downward_api: &DownwardAPIProjection,
    path: &Path,
    all_paths: &mut HashSet<String>,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate items
    for (i, item) in downward_api.items.iter().enumerate() {
        all_errs.extend(validate_downward_api_volume_file(
            item,
            &path.child("items").index(i),
        ));
        if !item.path.is_empty() && !all_paths.insert(item.path.clone()) {
            all_errs.push(invalid(
                path,
                BadValue::String(item.path.clone()),
                "conflicting duplicate paths",
            ));
        }
    }

    all_errs
}

fn validate_config_map_projection(
    config_map: &ConfigMapProjection,
    path: &Path,
    all_paths: &mut HashSet<String>,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if config_map.name.as_deref().unwrap_or("").is_empty() {
        all_errs.push(required(&path.child("name"), "name is required"));
    } else {
        for msg in is_dns1123_subdomain(config_map.name.as_deref().unwrap_or("")) {
            all_errs.push(invalid(
                &path.child("name"),
                BadValue::String(config_map.name.as_deref().unwrap_or("").to_string()),
                &msg,
            ));
        }
    }

    // Validate items
    all_errs.extend(validate_key_to_paths(
        &config_map.items,
        &path.child("items"),
    ));
    for item in &config_map.items {
        if !item.path.is_empty() && !all_paths.insert(item.path.clone()) {
            all_errs.push(invalid(
                path,
                BadValue::String(item.path.clone()),
                "conflicting duplicate paths",
            ));
        }
    }

    all_errs
}

fn validate_service_account_token_projection(
    sa_token: &ServiceAccountTokenProjection,
    path: &Path,
    all_paths: &mut HashSet<String>,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // path is required
    if sa_token.path.is_empty() {
        all_errs.push(required(&path.child("path"), "path is required"));
    } else {
        all_errs.extend(validate_local_descending_path(
            &sa_token.path,
            &path.child("path"),
        ));
        if !all_paths.insert(sa_token.path.clone()) {
            all_errs.push(invalid(
                path,
                BadValue::String(sa_token.path.clone()),
                "conflicting duplicate paths",
            ));
        }
    }

    // expirationSeconds must be positive
    let expiration = sa_token.expiration_seconds;
    if expiration < 600 {
        all_errs.push(invalid(
            &path.child("expirationSeconds"),
            BadValue::Int(expiration),
            "may not specify a duration less than 10 minutes",
        ));
    } else if expiration > u32::MAX as i64 {
        all_errs.push(invalid(
            &path.child("expirationSeconds"),
            BadValue::Int(expiration),
            "may not specify a duration larger than 2^32 seconds",
        ));
    } else {
        all_errs.extend(validate_positive_field(
            expiration,
            &path.child("expirationSeconds"),
        ));
    }

    all_errs
}

fn validate_cluster_trust_bundle_projection(
    ctb: &ClusterTrustBundleProjection,
    path: &Path,
    all_paths: &mut HashSet<String>,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // path is required
    if ctb.path.is_empty() {
        all_errs.push(required(&path.child("path"), "path is required"));
    } else {
        all_errs.extend(validate_local_descending_path(
            &ctb.path,
            &path.child("path"),
        ));
        if !all_paths.insert(ctb.path.clone()) {
            all_errs.push(invalid(
                path,
                BadValue::String(ctb.path.clone()),
                "conflicting duplicate paths",
            ));
        }
    }

    // Either name or signerName should be specified, not both
    let has_name = ctb.name.as_deref().map_or(false, |value| !value.is_empty());
    let has_signer = ctb
        .signer_name
        .as_deref()
        .map_or(false, |value| !value.is_empty());

    if has_name && has_signer {
        all_errs.push(invalid(
            path,
            BadValue::String("clusterTrustBundle".to_string()),
            "only one of name and signerName may be used",
        ));
    } else if !has_name && !has_signer {
        all_errs.push(required(
            path,
            "either name or signerName must be specified",
        ));
    }

    all_errs
}

fn validate_pod_certificate_projection(
    pod_cert: &PodCertificateProjection,
    path: &Path,
    all_paths: &mut HashSet<String>,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // signerName is required
    if pod_cert.signer_name.is_empty() {
        all_errs.push(required(
            &path.child("signerName"),
            "signerName is required",
        ));
    }

    // At least one of the paths must be specified
    let has_cred = pod_cert.credential_bundle_path.is_some()
        && !pod_cert.credential_bundle_path.as_ref().unwrap().is_empty();
    let has_key = pod_cert.key_path.is_some() && !pod_cert.key_path.as_ref().unwrap().is_empty();
    let has_cert = pod_cert.certificate_chain_path.is_some()
        && !pod_cert.certificate_chain_path.as_ref().unwrap().is_empty();

    if !has_cred && !has_key && !has_cert {
        all_errs.push(required(
            path,
            "must specify at least one of: credentialBundlePath, keyPath, certificateChainPath",
        ));
    }

    // Validate paths if specified
    if let Some(ref cred_path) = pod_cert.credential_bundle_path {
        all_errs.extend(validate_local_descending_path(
            cred_path,
            &path.child("credentialBundlePath"),
        ));
        if !all_paths.insert(cred_path.clone()) {
            all_errs.push(invalid(
                path,
                BadValue::String(cred_path.clone()),
                "conflicting duplicate paths",
            ));
        }
    }
    if let Some(ref key_path) = pod_cert.key_path {
        all_errs.extend(validate_local_descending_path(
            key_path,
            &path.child("keyPath"),
        ));
        if !all_paths.insert(key_path.clone()) {
            all_errs.push(invalid(
                path,
                BadValue::String(key_path.clone()),
                "conflicting duplicate paths",
            ));
        }
    }
    if let Some(ref cert_path) = pod_cert.certificate_chain_path {
        all_errs.extend(validate_local_descending_path(
            cert_path,
            &path.child("certificateChainPath"),
        ));
        if !all_paths.insert(cert_path.clone()) {
            all_errs.push(invalid(
                path,
                BadValue::String(cert_path.clone()),
                "conflicting duplicate paths",
            ));
        }
    }

    all_errs
}

fn validate_csi_volume_source(csi: &CSIVolumeSource, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // driver is required
    all_errs.extend(validate_csi_driver_name(&csi.driver, &path.child("driver")));

    if let Some(ref secret_ref) = csi.node_publish_secret_ref {
        if secret_ref.name.as_deref().unwrap_or("").is_empty() {
            all_errs.push(required(
                &path.child("nodePublishSecretRef").child("name"),
                "name is required",
            ));
        } else {
            for msg in is_dns1123_subdomain(secret_ref.name.as_deref().unwrap_or("")) {
                all_errs.push(invalid(
                    &path.child("nodePublishSecretRef").child("name"),
                    BadValue::String(secret_ref.name.as_deref().unwrap_or("").to_string()),
                    &msg,
                ));
            }
        }
    }

    all_errs
}

fn validate_ephemeral_volume_source(ephemeral: &EphemeralVolumeSource, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    let Some(ref template) = ephemeral.volume_claim_template else {
        all_errs.push(required(
            &path.child("volumeClaimTemplate"),
            "volumeClaimTemplate is required",
        ));
        return all_errs;
    };

    if let Some(ref spec) = template.spec {
        all_errs.extend(
            crate::core::internal::validation::storage::validate_persistent_volume_claim_spec(
                spec,
                &path.child("volumeClaimTemplate").child("spec"),
            ),
        );
    } else {
        all_errs.push(required(
            &path.child("volumeClaimTemplate").child("spec"),
            "spec is required",
        ));
    }

    all_errs
}

fn validate_image_volume_source(image: &ImageVolumeSource, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // reference is required
    if image.reference.is_empty() {
        all_errs.push(required(&path.child("reference"), "reference is required"));
    }

    // pullPolicy is validated by enum deserialization; no additional checks required.

    all_errs
}

// ============================================================================
// Volume Mount & Device Validation
// ============================================================================

/// Validates volume mounts
pub fn validate_volume_mounts(
    mounts: &[VolumeMount],
    vol_devices: &HashMap<String, String>,
    volumes: &HashMap<String, VolumeSource>,
    container: &Container,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let mut mount_points = HashSet::new();

    // Check mount count limit
    if mounts.len() > MAX_VOLUME_MOUNTS_PER_CONTAINER {
        all_errs.push(crate::common::validation::too_many(
            path,
            Some(mounts.len()),
            MAX_VOLUME_MOUNTS_PER_CONTAINER,
        ));
    }

    for (i, mnt) in mounts.iter().enumerate() {
        let idx_path = path.index(i);

        // name is required
        if mnt.name.is_empty() {
            all_errs.push(required(&idx_path.child("name"), "name is required"));
        }

        // name must exist in volumes
        if !volumes.contains_key(&mnt.name) {
            all_errs.push(not_found(
                &idx_path.child("name"),
                BadValue::String(mnt.name.clone()),
            ));
        }

        // mountPath is required
        if mnt.mount_path.is_empty() {
            all_errs.push(required(
                &idx_path.child("mountPath"),
                "mountPath is required",
            ));
        }

        // mountPath must be unique
        if mount_points.contains(&mnt.mount_path) {
            all_errs.push(invalid(
                &idx_path.child("mountPath"),
                BadValue::String(mnt.mount_path.clone()),
                "must be unique",
            ));
        }
        mount_points.insert(mnt.mount_path.clone());

        // Check for overlap with volumeDevices
        if vol_devices.contains_key(&mnt.name) {
            all_errs.push(invalid(
                &idx_path.child("name"),
                BadValue::String(mnt.name.clone()),
                "must not already exist in volumeDevices",
            ));
        }
        if vol_devices.values().any(|path| path == &mnt.mount_path) {
            all_errs.push(invalid(
                &idx_path.child("mountPath"),
                BadValue::String(mnt.mount_path.clone()),
                "must not already exist as a path in volumeDevices",
            ));
        }

        // Validate subPath if specified
        if !mnt.sub_path.is_empty() {
            all_errs.extend(validate_local_descending_path(
                &mnt.sub_path,
                &idx_path.child("subPath"),
            ));
        }

        // Validate subPathExpr if specified
        if !mnt.sub_path_expr.is_empty() {
            // subPath and subPathExpr are mutually exclusive
            if !mnt.sub_path.is_empty() {
                all_errs.push(invalid(
                    &idx_path.child("subPathExpr"),
                    BadValue::String(mnt.sub_path_expr.clone()),
                    "subPathExpr and subPath are mutually exclusive",
                ));
            }

            all_errs.extend(validate_local_descending_path(
                &mnt.sub_path_expr,
                &idx_path.child("subPathExpr"),
            ));
        }

        if let Some(ref propagation) = mnt.mount_propagation {
            all_errs.extend(validate_mount_propagation(
                propagation,
                container,
                &idx_path.child("mountPropagation"),
            ));
        }
        all_errs.extend(validate_mount_recursive_read_only(
            mnt,
            &idx_path.child("recursiveReadOnly"),
        ));
    }

    all_errs
}

/// Validates volume devices
pub fn validate_volume_devices(
    devices: &[VolumeDevice],
    vol_mounts: &HashMap<String, String>,
    volumes: &HashMap<String, VolumeSource>,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let mut device_paths = HashSet::new();
    let mut device_names = HashSet::new();

    // Check device count limit
    if devices.len() > MAX_VOLUME_DEVICES_PER_CONTAINER {
        all_errs.push(crate::common::validation::too_many(
            path,
            Some(devices.len()),
            MAX_VOLUME_DEVICES_PER_CONTAINER,
        ));
    }

    for (i, dev) in devices.iter().enumerate() {
        let idx_path = path.index(i);

        // name is required
        if dev.name.is_empty() {
            all_errs.push(required(&idx_path.child("name"), "name is required"));
        }

        // name must be unique
        if device_names.contains(&dev.name) {
            all_errs.push(invalid(
                &idx_path.child("name"),
                BadValue::String(dev.name.clone()),
                "must be unique",
            ));
        }
        device_names.insert(dev.name.clone());

        // name must exist in volumes
        let is_matched = volumes.contains_key(&dev.name);
        let is_pvc = if let Some(source) = volumes.get(&dev.name) {
            source.persistent_volume_claim.is_some() || source.ephemeral.is_some()
        } else {
            false
        };

        if !is_matched {
            all_errs.push(not_found(
                &idx_path.child("name"),
                BadValue::String(dev.name.clone()),
            ));
        } else if !is_pvc {
            // Block mode can only use PersistentVolumeClaim or Ephemeral sources
            all_errs.push(invalid(
                &idx_path.child("name"),
                BadValue::String(dev.name.clone()),
                "can only use volume source type of PersistentVolumeClaim or Ephemeral for block mode",
            ));
        }

        // devicePath is required
        if dev.device_path.is_empty() {
            all_errs.push(required(
                &idx_path.child("devicePath"),
                "devicePath is required",
            ));
        }

        // devicePath must be unique
        if device_paths.contains(&dev.device_path) {
            all_errs.push(invalid(
                &idx_path.child("devicePath"),
                BadValue::String(dev.device_path.clone()),
                "must be unique",
            ));
        }

        // devicePath must not contain ..
        if !dev.device_path.is_empty()
            && !validate_path_no_backsteps(&dev.device_path, &idx_path.child("devicePath"))
                .is_empty()
        {
            all_errs.push(invalid(
                &idx_path.child("devicePath"),
                BadValue::String(dev.device_path.clone()),
                "can not contain backsteps ('..')",
            ));
        } else {
            device_paths.insert(dev.device_path.clone());
        }

        // Check for overlap with volumeMounts
        if vol_mounts.contains_key(&dev.name) {
            all_errs.push(invalid(
                &idx_path.child("name"),
                BadValue::String(dev.name.clone()),
                "must not already exist in volumeMounts",
            ));
        }
        if vol_mounts.values().any(|path| path == &dev.device_path) {
            all_errs.push(invalid(
                &idx_path.child("devicePath"),
                BadValue::String(dev.device_path.clone()),
                "must not already exist as a path in volumeMounts",
            ));
        }
    }

    all_errs
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Validates KeyToPath items
fn validate_key_to_paths(items: &[KeyToPath], path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (i, item) in items.iter().enumerate() {
        let idx_path = path.index(i);

        // key is required
        if item.key.is_empty() {
            all_errs.push(required(&idx_path.child("key"), "key is required"));
        }

        // path is required
        if item.path.is_empty() {
            all_errs.push(required(&idx_path.child("path"), "path is required"));
        } else {
            // path must be a local descending path
            all_errs.extend(validate_local_descending_path(
                &item.path,
                &idx_path.child("path"),
            ));
        }

        // Validate mode if specified
        if let Some(mode) = item.mode {
            all_errs.extend(validate_file_mode(mode, &idx_path.child("mode")));
        }
    }

    all_errs
}

fn validate_volume_object_field_selector(
    selector: &crate::core::v1::selector::ObjectFieldSelector,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if selector.api_version.is_empty() {
        all_errs.push(required(
            &path.child("apiVersion"),
            "apiVersion is required",
        ));
        return all_errs;
    }

    if selector.field_path.is_empty() {
        all_errs.push(required(&path.child("fieldPath"), "fieldPath is required"));
        return all_errs;
    }

    let field_path = &selector.field_path;
    let base_path = if let Some(bracket_pos) = field_path.find('[') {
        &field_path[..bracket_pos]
    } else {
        field_path.as_str()
    };

    if !VALID_VOLUME_DOWNWARD_API_FIELD_PATH_EXPRESSIONS.contains(base_path) {
        if !base_path.starts_with("metadata.labels")
            && !base_path.starts_with("metadata.annotations")
        {
            let valid: Vec<&str> = VALID_VOLUME_DOWNWARD_API_FIELD_PATH_EXPRESSIONS
                .iter()
                .copied()
                .collect();
            all_errs.push(not_supported(
                &path.child("fieldPath"),
                BadValue::String(field_path.clone()),
                &valid,
            ));
        }
    }

    all_errs
}

fn validate_container_resource_field_selector(
    selector: &crate::core::v1::selector::ResourceFieldSelector,
    path: &Path,
    volume: bool,
) -> ErrorList {
    crate::core::v1::validation::selector::validate_container_resource_field_selector(
        selector, path, volume,
    )
}

fn validate_csi_driver_name(driver: &str, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if driver.is_empty() {
        all_errs.push(required(path, "driver is required"));
        return all_errs;
    }

    if driver.len() > 63 {
        all_errs.push(invalid(
            path,
            BadValue::String(driver.to_string()),
            "must be no more than 63 characters",
        ));
    }

    for err_msg in is_dns1123_subdomain(&driver.to_lowercase()) {
        all_errs.push(invalid(
            path,
            BadValue::String(driver.to_string()),
            &err_msg,
        ));
    }

    all_errs
}

fn validate_mount_propagation(
    mount_propagation: &str,
    container: &Container,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let supported = [
        crate::core::internal::mount_propagation_mode::BIDIRECTIONAL,
        crate::core::internal::mount_propagation_mode::HOST_TO_CONTAINER,
        crate::core::internal::mount_propagation_mode::NONE,
    ];

    if !supported.contains(&mount_propagation) {
        all_errs.push(not_supported(
            path,
            BadValue::String(mount_propagation.to_string()),
            &supported,
        ));
    }

    let privileged = container
        .security_context
        .as_ref()
        .and_then(|ctx| ctx.privileged)
        .unwrap_or(false);
    if mount_propagation == crate::core::internal::mount_propagation_mode::BIDIRECTIONAL
        && !privileged
    {
        all_errs.push(forbidden(
            path,
            "Bidirectional mount propagation is available only to privileged containers",
        ));
    }

    all_errs
}

fn validate_mount_recursive_read_only(mount: &VolumeMount, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let Some(ref mode) = mount.recursive_read_only else {
        return all_errs;
    };

    match mode.as_str() {
        crate::core::internal::recursive_read_only_mode::DISABLED => {}
        crate::core::internal::recursive_read_only_mode::ENABLED
        | crate::core::internal::recursive_read_only_mode::IF_POSSIBLE => {
            if !mount.read_only {
                all_errs.push(forbidden(
                    path,
                    "may only be specified when readOnly is true",
                ));
            }
            if let Some(ref propagation) = mount.mount_propagation {
                if propagation != crate::core::internal::mount_propagation_mode::NONE {
                    all_errs.push(forbidden(
                        path,
                        "may only be specified when mountPropagation is None or not specified",
                    ));
                }
            }
        }
        _ => {
            let supported = [
                crate::core::internal::recursive_read_only_mode::DISABLED,
                crate::core::internal::recursive_read_only_mode::IF_POSSIBLE,
                crate::core::internal::recursive_read_only_mode::ENABLED,
            ];
            all_errs.push(not_supported(
                path,
                BadValue::String(mode.clone()),
                &supported,
            ));
        }
    }

    all_errs
}

fn validate_iscsi_qualified_name(value: &str, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if !(value.starts_with("iqn") || value.starts_with("eui") || value.starts_with("naa")) {
        all_errs.push(invalid(
            path,
            BadValue::String(value.to_string()),
            "must be valid format starting with iqn, eui, or naa",
        ));
        return all_errs;
    }

    if value.starts_with("iqn") && !ISCSI_IQN_RE.is_match(value) {
        all_errs.push(invalid(
            path,
            BadValue::String(value.to_string()),
            "must be valid format",
        ));
    } else if value.starts_with("eui") && !ISCSI_EUI_RE.is_match(value) {
        all_errs.push(invalid(
            path,
            BadValue::String(value.to_string()),
            "must be valid format",
        ));
    } else if value.starts_with("naa") && !ISCSI_NAA_RE.is_match(value) {
        all_errs.push(invalid(
            path,
            BadValue::String(value.to_string()),
            "must be valid format",
        ));
    }

    all_errs
}

static ISCSI_IQN_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^iqn\.\d{4}-\d{2}\.([[:alnum:]-.]+)(:[^,;*&$|\s]+)$").unwrap());
static ISCSI_EUI_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^eui.[[:alnum:]]{16}$").unwrap());
static ISCSI_NAA_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^naa.[[:alnum:]]{32}$").unwrap());

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::core::internal::host_path_type;

    #[test]
    fn test_validate_volume_valid() {
        let vol = Volume {
            name: "test-vol".to_string(),
            volume_source: VolumeSource {
                empty_dir: Some(Default::default()),
                ..Default::default()
            },
        };

        let errs = validate_volume(&vol, &Path::nil());
        assert!(errs.is_empty(), "Expected no errors, got: {:?}", errs);
    }

    #[test]
    fn test_validate_volume_missing_name() {
        let vol = Volume {
            name: String::new(),
            volume_source: VolumeSource {
                empty_dir: Some(Default::default()),
                ..Default::default()
            },
        };

        let errs = validate_volume(&vol, &Path::nil());
        assert!(!errs.is_empty(), "Expected error for missing name");
    }

    #[test]
    fn test_validate_volume_no_source() {
        let vol = Volume {
            name: "test-vol".to_string(),
            volume_source: Default::default(),
        };

        let errs = validate_volume(&vol, &Path::nil());
        assert!(!errs.is_empty(), "Expected error for missing volume source");
    }

    #[test]
    fn test_validate_volume_multiple_sources() {
        let vol = Volume {
            name: "test-vol".to_string(),
            volume_source: VolumeSource {
                empty_dir: Some(Default::default()),
                host_path: Some(HostPathVolumeSource {
                    path: "/foo".to_string(),
                    r#type: None,
                }),
                ..Default::default()
            },
        };

        let errs = validate_volume(&vol, &Path::nil());
        assert!(
            !errs.is_empty(),
            "Expected error for multiple volume sources"
        );
        assert!(
            errs.errors
                .iter()
                .any(|e| { e.detail.contains("may not specify more than 1 volume type") })
        );
    }

    #[test]
    fn test_validate_volumes_duplicate_names() {
        let vols = vec![
            Volume {
                name: "test-vol".to_string(),
                volume_source: VolumeSource {
                    empty_dir: Some(Default::default()),
                    ..Default::default()
                },
            },
            Volume {
                name: "test-vol".to_string(),
                volume_source: VolumeSource {
                    host_path: Some(HostPathVolumeSource {
                        path: "/foo".to_string(),
                        r#type: None,
                    }),
                    ..Default::default()
                },
            },
        ];

        let (_, errs) = validate_volumes(&vols, &Path::nil());
        assert!(!errs.is_empty(), "Expected error for duplicate names");
        assert!(
            errs.errors
                .iter()
                .any(|e| e.error_type == crate::common::validation::ErrorType::Duplicate)
        );
    }

    #[test]
    fn test_validate_host_path() {
        let vol = Volume {
            name: "hostpath-vol".to_string(),
            volume_source: VolumeSource {
                host_path: Some(HostPathVolumeSource {
                    path: "/data".to_string(),
                    r#type: Some(host_path_type::DIRECTORY.to_string()),
                }),
                ..Default::default()
            },
        };

        let errs = validate_volume(&vol, &Path::nil());
        assert!(errs.is_empty(), "Expected no errors, got: {:?}", errs);
    }

    #[test]
    fn test_validate_host_path_invalid_type() {
        let vol = Volume {
            name: "hostpath-vol".to_string(),
            volume_source: VolumeSource {
                host_path: Some(HostPathVolumeSource {
                    path: "/data".to_string(),
                    r#type: Some("InvalidType".to_string()),
                }),
                ..Default::default()
            },
        };

        let errs = validate_volume(&vol, &Path::nil());
        assert!(!errs.is_empty(), "Expected error for invalid type");
    }

    #[test]
    fn test_validate_volume_mounts() {
        let mounts = vec![VolumeMount {
            name: "vol1".to_string(),
            mount_path: "/mnt/vol1".to_string(),
            ..Default::default()
        }];

        let mut volumes = HashMap::new();
        volumes.insert("vol1".to_string(), Default::default());
        let vol_devices = HashMap::new();

        let errs = validate_volume_mounts(
            &mounts,
            &vol_devices,
            &volumes,
            &crate::core::internal::InternalContainer::default(),
            &Path::nil(),
        );
        assert!(errs.is_empty(), "Expected no errors, got: {:?}", errs);
    }

    #[test]
    fn test_validate_volume_mounts_duplicate_paths() {
        let mounts = vec![
            VolumeMount {
                name: "vol1".to_string(),
                mount_path: "/mnt/vol".to_string(),
                ..Default::default()
            },
            VolumeMount {
                name: "vol2".to_string(),
                mount_path: "/mnt/vol".to_string(),
                ..Default::default()
            },
        ];

        let mut volumes = HashMap::new();
        volumes.insert("vol1".to_string(), Default::default());
        volumes.insert("vol2".to_string(), Default::default());
        let vol_devices = HashMap::new();

        let errs = validate_volume_mounts(
            &mounts,
            &vol_devices,
            &volumes,
            &crate::core::internal::InternalContainer::default(),
            &Path::nil(),
        );
        assert!(!errs.is_empty(), "Expected error for duplicate mount paths");
    }

    #[test]
    fn test_validate_volume_mounts_not_found() {
        let mounts = vec![VolumeMount {
            name: "nonexistent".to_string(),
            mount_path: "/mnt/vol".to_string(),
            ..Default::default()
        }];

        let volumes = HashMap::new();
        let vol_devices = HashMap::new();

        let errs = validate_volume_mounts(
            &mounts,
            &vol_devices,
            &volumes,
            &crate::core::internal::InternalContainer::default(),
            &Path::nil(),
        );
        assert!(!errs.is_empty(), "Expected error for nonexistent volume");
        assert!(
            errs.errors
                .iter()
                .any(|e| e.error_type == crate::common::validation::ErrorType::NotFound)
        );
    }
}
