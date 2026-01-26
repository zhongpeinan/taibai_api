//! Volume validation for core v1 API

use crate::common::validation::{
    BadValue, ErrorList, Path, forbidden, invalid, is_dns1123_label, not_found, required,
};
use crate::core::v1::volume::{
    CSIVolumeSource, ClusterTrustBundleProjection, ConfigMapProjection, ConfigMapVolumeSource,
    DownwardAPIProjection, DownwardAPIVolumeFile, DownwardAPIVolumeSource, EphemeralVolumeSource,
    GlusterfsVolumeSource, HostPathVolumeSource, ISCSIVolumeSource, ImageVolumeSource, KeyToPath,
    NFSVolumeSource, PersistentVolumeClaimVolumeSource, PodCertificateProjection,
    ProjectedVolumeSource, SecretProjection, SecretVolumeSource, ServiceAccountTokenProjection,
    Volume, VolumeDevice, VolumeMount, VolumeProjection, VolumeSource,
};
use std::collections::{HashMap, HashSet};

use super::constants::*;
use super::helpers::*;

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
    if source.cephfs.is_some() {
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
    empty_dir: &crate::core::v1::volume::EmptyDirVolumeSource,
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
    }

    // Validate type if specified
    if let Some(ref type_) = host_path.type_ {
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
    if let Some(ref secret_name) = secret.secret_name {
        if secret_name.is_empty() {
            all_errs.push(required(
                &path.child("secretName"),
                "secretName is required",
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
        }
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
    }

    // lun must be non-negative
    all_errs.extend(validate_nonnegative_field(
        iscsi.lun as i64,
        &path.child("lun"),
    ));

    all_errs
}

fn validate_glusterfs_volume_source(glusterfs: &GlusterfsVolumeSource, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // endpoints is required
    if glusterfs.endpoints.is_empty() {
        all_errs.push(required(&path.child("endpoints"), "endpoints is required"));
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
        // TODO: Validate field_ref when ObjectFieldSelector validation is available
    }

    if file.resource_field_ref.is_some() {
        num_sources += 1;
        // TODO: Validate resource_field_ref when ResourceFieldSelector validation is available
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

    // Validate sources
    for (i, source) in projected.sources.iter().enumerate() {
        all_errs.extend(validate_volume_projection(
            source,
            &path.child("sources").index(i),
        ));
    }

    // Validate defaultMode if specified
    if let Some(mode) = projected.default_mode {
        all_errs.extend(validate_file_mode(mode, &path.child("defaultMode")));
    }

    all_errs
}

fn validate_volume_projection(projection: &VolumeProjection, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let mut num_sources = 0;

    // Secret
    if let Some(ref secret) = projection.secret {
        num_sources += 1;
        all_errs.extend(validate_secret_projection(secret, &path.child("secret")));
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
            ));
        }
    }

    // Must have exactly one source
    if num_sources == 0 {
        all_errs.push(required(path, "must specify a projection type"));
    }

    all_errs
}

fn validate_secret_projection(secret: &SecretProjection, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate items
    all_errs.extend(validate_key_to_paths(&secret.items, &path.child("items")));

    all_errs
}

fn validate_downward_api_projection(
    downward_api: &DownwardAPIProjection,
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

    all_errs
}

fn validate_config_map_projection(config_map: &ConfigMapProjection, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate items
    all_errs.extend(validate_key_to_paths(
        &config_map.items,
        &path.child("items"),
    ));

    all_errs
}

fn validate_service_account_token_projection(
    sa_token: &ServiceAccountTokenProjection,
    path: &Path,
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
    }

    // expirationSeconds must be positive if specified
    if let Some(expiration) = sa_token.expiration_seconds {
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
    }

    // Either name or signerName should be specified, not both
    let has_name = ctb.name.is_some() && !ctb.name.as_ref().unwrap().is_empty();
    let has_signer = ctb.signer_name.is_some() && !ctb.signer_name.as_ref().unwrap().is_empty();

    if has_name && has_signer {
        all_errs.push(forbidden(path, "may not specify both name and signerName"));
    }

    all_errs
}

fn validate_pod_certificate_projection(
    pod_cert: &PodCertificateProjection,
    path: &Path,
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
    }
    if let Some(ref key_path) = pod_cert.key_path {
        all_errs.extend(validate_local_descending_path(
            key_path,
            &path.child("keyPath"),
        ));
    }
    if let Some(ref cert_path) = pod_cert.certificate_chain_path {
        all_errs.extend(validate_local_descending_path(
            cert_path,
            &path.child("certificateChainPath"),
        ));
    }

    all_errs
}

fn validate_csi_volume_source(csi: &CSIVolumeSource, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // driver is required
    if csi.driver.is_empty() {
        all_errs.push(required(&path.child("driver"), "driver is required"));
    }

    all_errs
}

fn validate_ephemeral_volume_source(_ephemeral: &EphemeralVolumeSource, _path: &Path) -> ErrorList {
    let all_errs = ErrorList::new();

    // volumeClaimTemplate validation
    // TODO: Add PersistentVolumeClaimSpec validation when storage.rs is implemented

    all_errs
}

fn validate_image_volume_source(image: &ImageVolumeSource, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // reference is required
    if image.reference.is_empty() {
        all_errs.push(required(&path.child("reference"), "reference is required"));
    }

    // Validate pullPolicy if specified
    if let Some(ref pull_policy) = image.pull_policy {
        if !SUPPORTED_PULL_POLICIES.contains(pull_policy.as_str()) {
            all_errs.push(crate::common::validation::not_supported(
                &path.child("pullPolicy"),
                BadValue::String(pull_policy.clone()),
                &SUPPORTED_PULL_POLICIES.iter().copied().collect::<Vec<_>>(),
            ));
        }
    }

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

        // TODO: Validate mountPropagation when supported
        // TODO: Validate recursiveReadOnly when supported
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
        if dev.device_path.contains("..") {
            all_errs.push(invalid(
                &idx_path.child("devicePath"),
                BadValue::String(dev.device_path.clone()),
                "can not contain backsteps (\'..\')",
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

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::validation::ErrorType;
    use crate::core::v1::volume::host_path_type;

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
                    type_: None,
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
                        type_: None,
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
                    type_: Some(host_path_type::DIRECTORY.to_string()),
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
                    type_: Some("InvalidType".to_string()),
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

        let errs = validate_volume_mounts(&mounts, &vol_devices, &volumes, &Path::nil());
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

        let errs = validate_volume_mounts(&mounts, &vol_devices, &volumes, &Path::nil());
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

        let errs = validate_volume_mounts(&mounts, &vol_devices, &volumes, &Path::nil());
        assert!(!errs.is_empty(), "Expected error for nonexistent volume");
        assert!(
            errs.errors
                .iter()
                .any(|e| e.error_type == crate::common::validation::ErrorType::NotFound)
        );
    }
}
