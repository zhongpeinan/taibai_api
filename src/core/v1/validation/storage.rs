//! PersistentVolume and PersistentVolumeClaim validation for Kubernetes core/v1 API
//!
//! This module implements validation for persistent storage resources.

use super::constants::SUPPORTED_HOST_PATH_TYPES;
use super::helpers::{validate_absolute_path, validate_nonnegative_field};
use super::resources::validate_resource_quantity_value;
use crate::common::meta::{LabelSelector, label_selector_operator};
use crate::common::validation::{
    BadValue, ErrorList, Path, forbidden, invalid, is_dns1123_label, is_dns1123_subdomain,
    is_valid_label_value, not_supported, required, validate_labels, validate_qualified_name,
};
use crate::core::internal::persistent_volume as internal_pv;
use crate::core::v1::affinity::{
    NodeSelector, NodeSelectorRequirement, NodeSelectorTerm, node_selector_operator,
};
use crate::core::v1::persistent_volume::{
    PersistentVolume, PersistentVolumeClaim, PersistentVolumeClaimSpec, PersistentVolumeSource,
    PersistentVolumeSpec, TypedObjectReference, VolumeNodeAffinity, persistent_volume_access_mode,
    persistent_volume_mode, persistent_volume_reclaim_policy,
};
use crate::core::v1::reference::TypedLocalObjectReference;
use serde::de::DeserializeOwned;
use std::collections::HashSet;
use std::sync::LazyLock;

// ============================================================================
// Constants
// ============================================================================

/// Storage resource name
const RESOURCE_STORAGE: &str = "storage";

/// Supported access modes
static SUPPORTED_ACCESS_MODES: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    HashSet::from([
        persistent_volume_access_mode::READ_WRITE_ONCE,
        persistent_volume_access_mode::READ_ONLY_MANY,
        persistent_volume_access_mode::READ_WRITE_MANY,
        persistent_volume_access_mode::READ_WRITE_ONCE_POD,
    ])
});

/// Supported reclaim policies
static SUPPORTED_RECLAIM_POLICIES: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    HashSet::from([
        persistent_volume_reclaim_policy::DELETE,
        persistent_volume_reclaim_policy::RECYCLE,
        persistent_volume_reclaim_policy::RETAIN,
    ])
});

/// Supported volume modes
static SUPPORTED_VOLUME_MODES: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    HashSet::from([
        persistent_volume_mode::BLOCK,
        persistent_volume_mode::FILESYSTEM,
    ])
});

// ============================================================================
// PersistentVolume Validation
// ============================================================================

/// Validates a PersistentVolume.
///
/// Validates:
/// - ObjectMeta (name required, no namespace)
/// - PersistentVolumeSpec
pub fn validate_persistent_volume(pv: &PersistentVolume, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate metadata (PV is cluster-scoped, so no namespace)
    if let Some(ref metadata) = pv.metadata {
        all_errs.extend(crate::common::validation::validate_object_meta(
            metadata,
            false, // No namespace required (cluster-scoped)
            |name: &str, _prefix: bool| crate::common::validation::is_dns1123_subdomain(name),
            &path.child("metadata"),
        ));
    } else {
        all_errs.push(required(&path.child("metadata"), "metadata is required"));
    }

    // Validate spec
    if let Some(ref spec) = pv.spec {
        all_errs.extend(validate_persistent_volume_spec(spec, &path.child("spec")));
    } else {
        all_errs.push(required(&path.child("spec"), "spec is required"));
    }

    all_errs
}

/// Validates a PersistentVolumeSpec.
///
/// Validates:
/// - Access modes (required, at least one, supported modes)
/// - Capacity (required, storage capacity must be positive)
/// - PersistentVolumeReclaimPolicy (must be supported)
/// - VolumeMode (must be supported)
/// - StorageClassName (DNS subdomain if specified)
/// - Exactly one volume source type specified
pub fn validate_persistent_volume_spec(spec: &PersistentVolumeSpec, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate access modes (required)
    if spec.access_modes.is_empty() {
        all_errs.push(required(
            &path.child("accessModes"),
            "at least 1 access mode is required",
        ));
    }

    let mut found_read_write_once_pod = false;
    let mut found_non_read_write_once_pod = false;

    for mode in &spec.access_modes {
        if !SUPPORTED_ACCESS_MODES.contains(mode.as_str()) {
            let valid: Vec<&str> = SUPPORTED_ACCESS_MODES.iter().copied().collect();
            all_errs.push(not_supported(
                &path.child("accessModes"),
                BadValue::String(mode.clone()),
                &valid,
            ));
        }

        if mode == persistent_volume_access_mode::READ_WRITE_ONCE_POD {
            found_read_write_once_pod = true;
        } else if SUPPORTED_ACCESS_MODES.contains(mode.as_str()) {
            found_non_read_write_once_pod = true;
        }
    }

    // ReadWriteOncePod cannot be mixed with other access modes
    if found_read_write_once_pod && found_non_read_write_once_pod {
        all_errs.push(forbidden(
            &path.child("accessModes"),
            "may not use ReadWriteOncePod with other access modes",
        ));
    }

    // Validate capacity (required, storage must be positive)
    if spec.capacity.is_empty() {
        all_errs.push(required(&path.child("capacity"), "capacity is required"));
    }

    // Must have "storage" key
    if !spec.capacity.contains_key(RESOURCE_STORAGE) || spec.capacity.len() > 1 {
        all_errs.push(not_supported(
            &path.child("capacity"),
            BadValue::String(format!("{:?}", spec.capacity.keys().collect::<Vec<_>>())),
            &[RESOURCE_STORAGE],
        ));
    }

    // Validate storage quantity is positive
    for (resource, quantity) in &spec.capacity {
        all_errs.extend(validate_resource_quantity_value(
            resource,
            quantity,
            &path.child("capacity").key(resource),
        ));
        all_errs.extend(validate_positive_quantity_value(
            quantity,
            &path.child("capacity").key(resource),
        ));
    }

    // Validate reclaim policy
    if let Some(ref policy) = spec.persistent_volume_reclaim_policy {
        if !policy.is_empty() && !SUPPORTED_RECLAIM_POLICIES.contains(policy.as_str()) {
            let valid: Vec<&str> = SUPPORTED_RECLAIM_POLICIES.iter().copied().collect();
            all_errs.push(not_supported(
                &path.child("persistentVolumeReclaimPolicy"),
                BadValue::String(policy.clone()),
                &valid,
            ));
        }
    }

    // Validate volume mode
    if let Some(ref mode) = spec.volume_mode {
        if !SUPPORTED_VOLUME_MODES.contains(mode.as_str()) {
            let valid: Vec<&str> = SUPPORTED_VOLUME_MODES.iter().copied().collect();
            all_errs.push(not_supported(
                &path.child("volumeMode"),
                BadValue::String(mode.clone()),
                &valid,
            ));
        }
    }

    // Validate storage class name (DNS subdomain if specified)
    if let Some(ref sc_name) = spec.storage_class_name {
        if !sc_name.is_empty() {
            for err_msg in is_dns1123_subdomain(sc_name) {
                all_errs.push(invalid(
                    &path.child("storageClassName"),
                    BadValue::String(sc_name.clone()),
                    &err_msg,
                ));
            }
        }
    }

    // Validate exactly one volume source type
    if let Some(ref source) = spec.persistent_volume_source {
        all_errs.extend(validate_persistent_volume_source(source, path));
    } else {
        all_errs.push(required(&path, "must specify a volume source"));
    }

    // Validate node affinity
    let mut node_affinity_specified = false;
    if let Some(node_affinity) = &spec.node_affinity {
        node_affinity_specified = true;
        all_errs.extend(validate_volume_node_affinity(
            node_affinity,
            &path.child("nodeAffinity"),
        ));
    }

    // Special rule: hostPath of '/' cannot have 'recycle' reclaim policy
    if let Some(ref source) = spec.persistent_volume_source {
        if let Some(ref host_path) = source.host_path {
            if let Some(ref reclaim_policy) = spec.persistent_volume_reclaim_policy {
                if reclaim_policy == persistent_volume_reclaim_policy::RECYCLE
                    && host_path_is_root(host_path)
                {
                    all_errs.push(forbidden(
                        &path.child("persistentVolumeReclaimPolicy"),
                        "may not be 'recycle' for a hostPath mount of '/'",
                    ));
                }
            }
        }
    }

    // Local volume requires node affinity
    if let Some(ref source) = spec.persistent_volume_source {
        if source.local.is_some() && !node_affinity_specified {
            all_errs.push(required(
                &path.child("nodeAffinity"),
                "Local volume requires node affinity",
            ));
        }
    }

    // Validate volume attributes class name (requires CSI source)
    if let Some(ref class_name) = spec.volume_attributes_class_name {
        if class_name.is_empty() {
            all_errs.push(required(
                &path.child("volumeAttributesClassName"),
                "an empty string is disallowed",
            ));
        } else {
            for err_msg in is_dns1123_subdomain(class_name) {
                all_errs.push(invalid(
                    &path.child("volumeAttributesClassName"),
                    BadValue::String(class_name.clone()),
                    &err_msg,
                ));
            }
        }

        if spec
            .persistent_volume_source
            .as_ref()
            .and_then(|source| source.csi.as_ref())
            .is_none()
        {
            all_errs.push(required(
                &path.child("csi"),
                "has to be specified when using volumeAttributesClassName",
            ));
        }
    }

    all_errs
}

/// Validates that exactly one volume source type is specified.
fn validate_persistent_volume_source(source: &PersistentVolumeSource, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let mut num_volumes = 0;

    // Count how many volume sources are specified
    if source.host_path.is_some() {
        num_volumes += 1;
    }
    if source.gce_persistent_disk.is_some() {
        num_volumes += 1;
    }
    if source.aws_elastic_block_store.is_some() {
        num_volumes += 1;
    }
    if source.glusterfs.is_some() {
        num_volumes += 1;
    }
    if source.nfs.is_some() {
        num_volumes += 1;
    }
    if source.rbd.is_some() {
        num_volumes += 1;
    }
    if source.iscsi.is_some() {
        num_volumes += 1;
    }
    if source.cinder.is_some() {
        num_volumes += 1;
    }
    if source.ceph_fs.is_some() {
        num_volumes += 1;
    }
    if source.fc.is_some() {
        num_volumes += 1;
    }
    if source.flocker.is_some() {
        num_volumes += 1;
    }
    if source.flex_volume.is_some() {
        num_volumes += 1;
    }
    if source.azure_file.is_some() {
        num_volumes += 1;
    }
    if source.vsphere_volume.is_some() {
        num_volumes += 1;
    }
    if source.quobyte.is_some() {
        num_volumes += 1;
    }
    if source.azure_disk.is_some() {
        num_volumes += 1;
    }
    if source.photon_persistent_disk.is_some() {
        num_volumes += 1;
    }
    if source.portworx_volume.is_some() {
        num_volumes += 1;
    }
    if source.scale_io.is_some() {
        num_volumes += 1;
    }
    if source.local.is_some() {
        num_volumes += 1;
    }
    if source.storage_os.is_some() {
        num_volumes += 1;
    }
    if source.csi.is_some() {
        num_volumes += 1;
    }

    if num_volumes == 0 {
        all_errs.push(required(path, "must specify a volume type"));
    } else if num_volumes > 1 {
        all_errs.push(forbidden(path, "may not specify more than 1 volume type"));
    }

    // Per-backend validation for non-deprecated sources.
    if let Some(ref host_path) = source.host_path {
        all_errs.extend(validate_host_path_pv_source(
            host_path,
            &path.child("hostPath"),
        ));
    }

    if let Some(ref nfs) = source.nfs {
        all_errs.extend(validate_nfs_pv_source(nfs, &path.child("nfs")));
    }

    if let Some(ref iscsi) = source.iscsi {
        all_errs.extend(validate_iscsi_pv_source(iscsi, &path.child("iscsi")));
    }

    if let Some(ref fc) = source.fc {
        all_errs.extend(validate_fc_pv_source(fc, &path.child("fc")));
    }

    if let Some(ref local) = source.local {
        all_errs.extend(validate_local_volume_source(local, &path.child("local")));
    }

    if let Some(ref csi) = source.csi {
        all_errs.extend(validate_csi_pv_source(csi, &path.child("csi")));
    }

    // Validate local volume requires node affinity (checked at spec level)
    // This is handled in validate_persistent_volume_spec by checking both conditions

    all_errs
}

/// Validates a PersistentVolume update.
///
/// Most PersistentVolume fields are immutable after creation.
pub fn validate_persistent_volume_update(
    new_pv: &PersistentVolume,
    old_pv: &PersistentVolume,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // First validate the new PV
    all_errs.extend(validate_persistent_volume(new_pv, path));

    // Check metadata update
    if let (Some(new_meta), Some(old_meta)) = (&new_pv.metadata, &old_pv.metadata) {
        all_errs.extend(crate::common::validation::validate_object_meta_update(
            new_meta,
            old_meta,
            &path.child("metadata"),
        ));
    }

    // PersistentVolumeSource should be immutable after creation.
    if let (Some(new_spec), Some(old_spec)) = (&new_pv.spec, &old_pv.spec) {
        if new_spec.persistent_volume_source != old_spec.persistent_volume_source {
            all_errs.push(forbidden(
                &path.child("spec").child("persistentVolumeSource"),
                "persistentVolumeSource is immutable after creation",
            ));
        }

        // VolumeMode is immutable
        if new_spec.volume_mode != old_spec.volume_mode {
            all_errs.push(forbidden(
                &path.child("spec").child("volumeMode"),
                "field is immutable",
            ));
        }

        // NodeAffinity updates are allowed only when the old value is unset
        if old_spec.node_affinity.is_some() && new_spec.node_affinity != old_spec.node_affinity {
            all_errs.push(forbidden(
                &path.child("spec").child("nodeAffinity"),
                "field is immutable once set",
            ));
        }
    }

    all_errs
}

// ============================================================================
// PersistentVolumeClaim Validation
// ============================================================================

/// Validates a PersistentVolumeClaim.
///
/// Validates:
/// - ObjectMeta (name and namespace required)
/// - PersistentVolumeClaimSpec
pub fn validate_persistent_volume_claim(pvc: &PersistentVolumeClaim, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate metadata (PVC is namespaced)
    if let Some(ref metadata) = pvc.metadata {
        all_errs.extend(crate::common::validation::validate_object_meta(
            metadata,
            true, // Namespace required
            |name: &str, _prefix: bool| crate::common::validation::is_dns1123_subdomain(name),
            &path.child("metadata"),
        ));
    } else {
        all_errs.push(required(&path.child("metadata"), "metadata is required"));
    }

    // Validate spec
    if let Some(ref spec) = pvc.spec {
        all_errs.extend(validate_persistent_volume_claim_spec(
            spec,
            &path.child("spec"),
        ));
    } else {
        all_errs.push(required(&path.child("spec"), "spec is required"));
    }

    all_errs
}

/// Validates a PersistentVolumeClaimSpec.
///
/// Validates:
/// - Access modes (required, at least one)
/// - Resources (storage request required and positive)
/// - Selector (label selector if specified)
/// - StorageClassName (DNS subdomain if specified)
/// - VolumeMode (must be supported)
pub fn validate_persistent_volume_claim_spec(
    spec: &PersistentVolumeClaimSpec,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate access modes (required)
    if spec.access_modes.is_empty() {
        all_errs.push(required(
            &path.child("accessModes"),
            "at least 1 access mode is required",
        ));
    }

    let mut found_read_write_once_pod = false;
    let mut found_non_read_write_once_pod = false;

    for mode in &spec.access_modes {
        if !SUPPORTED_ACCESS_MODES.contains(mode.as_str()) {
            let valid: Vec<&str> = SUPPORTED_ACCESS_MODES.iter().copied().collect();
            all_errs.push(not_supported(
                &path.child("accessModes"),
                BadValue::String(mode.clone()),
                &valid,
            ));
        }

        if mode == persistent_volume_access_mode::READ_WRITE_ONCE_POD {
            found_read_write_once_pod = true;
        } else if SUPPORTED_ACCESS_MODES.contains(mode.as_str()) {
            found_non_read_write_once_pod = true;
        }
    }

    // ReadWriteOncePod cannot be mixed with other access modes
    if found_read_write_once_pod && found_non_read_write_once_pod {
        all_errs.push(forbidden(
            &path.child("accessModes"),
            "may not use ReadWriteOncePod with other access modes",
        ));
    }

    // Validate resources (storage request required)
    if let Some(ref resources) = spec.resources {
        if !resources.requests.contains_key(RESOURCE_STORAGE) {
            all_errs.push(required(
                &path.child("resources").key(RESOURCE_STORAGE),
                "storage request is required",
            ));
        } else if let Some(storage_qty) = resources.requests.get(RESOURCE_STORAGE) {
            all_errs.extend(validate_positive_quantity_value(
                storage_qty,
                &path.child("resources").key(RESOURCE_STORAGE),
            ));
            all_errs.extend(validate_resource_quantity_value(
                RESOURCE_STORAGE,
                storage_qty,
                &path.child("resources").key(RESOURCE_STORAGE),
            ));
        }
    } else {
        all_errs.push(required(&path.child("resources"), "resources is required"));
    }

    // Validate selector (label selector if specified)
    if let Some(ref selector) = spec.selector {
        all_errs.extend(validate_label_selector(selector, &path.child("selector")));
    }

    // Validate storage class name (DNS subdomain if specified)
    if let Some(ref sc_name) = spec.storage_class_name {
        if !sc_name.is_empty() {
            for err_msg in is_dns1123_subdomain(sc_name) {
                all_errs.push(invalid(
                    &path.child("storageClassName"),
                    BadValue::String(sc_name.clone()),
                    &err_msg,
                ));
            }
        }
    }

    // Validate volume mode
    if let Some(ref mode) = spec.volume_mode {
        if !SUPPORTED_VOLUME_MODES.contains(mode.as_str()) {
            let valid: Vec<&str> = SUPPORTED_VOLUME_MODES.iter().copied().collect();
            all_errs.push(not_supported(
                &path.child("volumeMode"),
                BadValue::String(mode.clone()),
                &valid,
            ));
        }
    }

    // Validate data source (if specified)
    if let Some(ref data_source) = spec.data_source {
        all_errs.extend(validate_data_source(data_source, &path.child("dataSource")));
    }

    // Validate data source ref (if specified)
    if let Some(ref data_source_ref) = spec.data_source_ref {
        all_errs.extend(validate_data_source_ref(
            data_source_ref,
            &path.child("dataSourceRef"),
        ));
    }

    if let Some(ref data_source_ref) = spec.data_source_ref {
        if data_source_ref
            .namespace
            .as_deref()
            .map_or(false, |value| !value.is_empty())
        {
            if spec.data_source.is_some() {
                all_errs.push(invalid(
                    path,
                    BadValue::String("dataSource".to_string()),
                    "may not be specified when dataSourceRef.namespace is specified",
                ));
            }
        } else if let (Some(data_source), Some(data_source_ref)) =
            (&spec.data_source, &spec.data_source_ref)
        {
            if !is_data_source_equal_data_source_ref(data_source, data_source_ref) {
                all_errs.push(invalid(
                    path,
                    BadValue::String("dataSource".to_string()),
                    "must match dataSourceRef",
                ));
            }
        }
    }

    if let Some(ref class_name) = spec.volume_attributes_class_name {
        if !class_name.is_empty() {
            for err_msg in is_dns1123_subdomain(class_name) {
                all_errs.push(invalid(
                    &path.child("volumeAttributesClassName"),
                    BadValue::String(class_name.clone()),
                    &err_msg,
                ));
            }
        }
    }

    all_errs
}

/// Validates a PersistentVolumeClaim update.
///
/// Most PersistentVolumeClaim fields are immutable after creation.
pub fn validate_persistent_volume_claim_update(
    new_pvc: &PersistentVolumeClaim,
    old_pvc: &PersistentVolumeClaim,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // First validate the new PVC
    all_errs.extend(validate_persistent_volume_claim(new_pvc, path));

    // Check metadata update
    if let (Some(new_meta), Some(old_meta)) = (&new_pvc.metadata, &old_pvc.metadata) {
        all_errs.extend(crate::common::validation::validate_object_meta_update(
            new_meta,
            old_meta,
            &path.child("metadata"),
        ));
    }

    if let (Some(new_spec), Some(old_spec)) = (&new_pvc.spec, &old_pvc.spec) {
        // VolumeName can be set once (from empty to a value)
        if old_spec.volume_name.is_some() && new_spec.volume_name != old_spec.volume_name {
            all_errs.push(forbidden(
                &path.child("spec").child("volumeName"),
                "field is immutable once set",
            ));
        }

        // Storage request cannot be decreased
        if let (Some(new_resources), Some(old_resources)) =
            (&new_spec.resources, &old_spec.resources)
        {
            if let (Some(new_qty), Some(old_qty)) = (
                new_resources.requests.get(RESOURCE_STORAGE),
                old_resources.requests.get(RESOURCE_STORAGE),
            ) {
                if new_qty
                    .cmp(old_qty)
                    .unwrap_or(std::cmp::Ordering::Less)
                    .is_lt()
                {
                    all_errs.push(forbidden(
                        &path
                            .child("spec")
                            .child("resources")
                            .child("requests")
                            .key(RESOURCE_STORAGE),
                        "field can not be less than previous value",
                    ));
                }
            }
        }

        // VolumeMode is immutable
        if new_spec.volume_mode != old_spec.volume_mode {
            all_errs.push(forbidden(
                &path.child("spec").child("volumeMode"),
                "field is immutable",
            ));
        }

        // StorageClassName is immutable
        if new_spec.storage_class_name != old_spec.storage_class_name {
            all_errs.push(forbidden(
                &path.child("spec").child("storageClassName"),
                "field is immutable",
            ));
        }

        // AccessModes are immutable
        if new_spec.access_modes != old_spec.access_modes {
            all_errs.push(forbidden(
                &path.child("spec").child("accessModes"),
                "field is immutable",
            ));
        }

        // Selector is immutable
        if new_spec.selector != old_spec.selector {
            all_errs.push(forbidden(
                &path.child("spec").child("selector"),
                "field is immutable",
            ));
        }

        // DataSource and DataSourceRef are immutable
        if new_spec.data_source != old_spec.data_source {
            all_errs.push(forbidden(
                &path.child("spec").child("dataSource"),
                "field is immutable",
            ));
        }
        if new_spec.data_source_ref != old_spec.data_source_ref {
            all_errs.push(forbidden(
                &path.child("spec").child("dataSourceRef"),
                "field is immutable",
            ));
        }

        if new_spec.volume_attributes_class_name != old_spec.volume_attributes_class_name {
            all_errs.push(forbidden(
                &path.child("spec").child("volumeAttributesClassName"),
                "field is immutable",
            ));
        }
    }

    all_errs
}

// ========================================================================
// Helper validation functions
// ========================================================================

fn validate_positive_quantity_value(quantity: &crate::common::Quantity, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if let Ok(sign) = quantity.sign() {
        if !sign.is_gt() {
            all_errs.push(invalid(
                path,
                BadValue::String(quantity.to_string()),
                "must be a positive quantity",
            ));
        }
    }
    all_errs
}

fn validate_label_selector(selector: &LabelSelector, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    all_errs.extend(validate_labels(
        &selector.match_labels,
        &path.child("matchLabels"),
    ));

    for (i, requirement) in selector.match_expressions.iter().enumerate() {
        let req_path = path.child("matchExpressions").index(i);
        if requirement.key.is_empty() {
            all_errs.push(required(&req_path.child("key"), "key is required"));
        } else {
            all_errs.extend(validate_qualified_name(
                &requirement.key,
                &req_path.child("key"),
            ));
        }

        match requirement.operator.as_str() {
            label_selector_operator::IN | label_selector_operator::NOT_IN => {
                if requirement.values.is_empty() {
                    all_errs.push(required(
                        &req_path.child("values"),
                        "values must be non-empty for In/NotIn operators",
                    ));
                }
            }
            label_selector_operator::EXISTS | label_selector_operator::DOES_NOT_EXIST => {
                if !requirement.values.is_empty() {
                    all_errs.push(invalid(
                        &req_path.child("values"),
                        BadValue::String(format!("{:?}", requirement.values)),
                        "values must be empty for Exists/DoesNotExist operators",
                    ));
                }
            }
            _ => {
                all_errs.push(not_supported(
                    &req_path.child("operator"),
                    BadValue::String(requirement.operator.clone()),
                    &[
                        label_selector_operator::IN,
                        label_selector_operator::NOT_IN,
                        label_selector_operator::EXISTS,
                        label_selector_operator::DOES_NOT_EXIST,
                    ],
                ));
            }
        }

        for (j, value) in requirement.values.iter().enumerate() {
            if value.is_empty() {
                all_errs.push(required(
                    &req_path.child("values").index(j),
                    "value must be non-empty",
                ));
                continue;
            }
            for msg in is_valid_label_value(value) {
                all_errs.push(invalid(
                    &req_path.child("values").index(j),
                    BadValue::String(value.clone()),
                    &msg,
                ));
            }
        }
    }

    all_errs
}

fn validate_volume_node_affinity(node_affinity: &VolumeNodeAffinity, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if let Some(ref required) = node_affinity.required {
        all_errs.extend(validate_node_selector(required, &path.child("required")));
    } else {
        all_errs.push(required(
            &path.child("required"),
            "must specify required node constraints",
        ));
    }
    all_errs
}

fn validate_node_selector(selector: &NodeSelector, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let terms_path = path.child("nodeSelectorTerms");

    if selector.node_selector_terms.is_empty() {
        all_errs.push(required(
            &terms_path,
            "must have at least one node selector term",
        ));
        return all_errs;
    }

    for (i, term) in selector.node_selector_terms.iter().enumerate() {
        all_errs.extend(validate_node_selector_term(term, &terms_path.index(i)));
    }

    all_errs
}

fn validate_node_selector_term(term: &NodeSelectorTerm, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (i, req) in term.match_expressions.iter().enumerate() {
        all_errs.extend(validate_node_selector_requirement(
            req,
            &path.child("matchExpressions").index(i),
            true,
        ));
    }

    for (i, req) in term.match_fields.iter().enumerate() {
        all_errs.extend(validate_node_selector_requirement(
            req,
            &path.child("matchFields").index(i),
            false,
        ));
    }

    all_errs
}

fn validate_node_selector_requirement(
    req: &NodeSelectorRequirement,
    path: &Path,
    is_label: bool,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if req.key.is_empty() {
        all_errs.push(required(&path.child("key"), "key is required"));
    } else if is_label {
        all_errs.extend(crate::common::validation::validate_label_name(
            &req.key,
            &path.child("key"),
        ));
    } else {
        all_errs.extend(validate_qualified_name(&req.key, &path.child("key")));
    }

    match req.operator.as_str() {
        node_selector_operator::IN | node_selector_operator::NOT_IN => {
            if req.values.is_empty() {
                all_errs.push(required(
                    &path.child("values"),
                    "values are required for In/NotIn",
                ));
            }
        }
        node_selector_operator::EXISTS | node_selector_operator::DOES_NOT_EXIST => {
            if !req.values.is_empty() {
                all_errs.push(invalid(
                    &path.child("values"),
                    BadValue::String(format!("{:?}", req.values)),
                    "values must be empty for Exists/DoesNotExist",
                ));
            }
        }
        node_selector_operator::GT | node_selector_operator::LT => {
            if req.values.len() != 1 {
                all_errs.push(invalid(
                    &path.child("values"),
                    BadValue::String(format!("{:?}", req.values)),
                    "must have exactly one value for Gt/Lt",
                ));
            } else if req.values[0].parse::<i64>().is_err() {
                all_errs.push(invalid(
                    &path.child("values").index(0),
                    BadValue::String(req.values[0].clone()),
                    "must be an integer for Gt/Lt",
                ));
            }
        }
        _ => {
            all_errs.push(not_supported(
                &path.child("operator"),
                BadValue::String(req.operator.clone()),
                &[
                    node_selector_operator::IN,
                    node_selector_operator::NOT_IN,
                    node_selector_operator::EXISTS,
                    node_selector_operator::DOES_NOT_EXIST,
                    node_selector_operator::GT,
                    node_selector_operator::LT,
                ],
            ));
        }
    }

    all_errs
}

fn parse_source<T: DeserializeOwned>(
    value: &serde_json::Value,
    path: &Path,
    kind: &str,
) -> Result<T, ErrorList> {
    serde_json::from_value::<T>(value.clone()).map_err(|_| {
        let mut errs = ErrorList::new();
        errs.push(invalid(
            path,
            BadValue::String(value.to_string()),
            &format!("must be a valid {kind} volume source"),
        ));
        errs
    })
}

fn validate_host_path_pv_source(value: &serde_json::Value, path: &Path) -> ErrorList {
    let host_path = match parse_source::<internal_pv::HostPathVolumeSource>(value, path, "hostPath")
    {
        Ok(parsed) => parsed,
        Err(errs) => return errs,
    };

    let mut all_errs = ErrorList::new();
    if host_path.path.is_empty() {
        all_errs.push(required(&path.child("path"), "path is required"));
        return all_errs;
    }

    all_errs.extend(validate_no_backstep_segments(
        &host_path.path,
        &path.child("path"),
    ));

    if let Some(ref type_) = host_path.r#type {
        if !SUPPORTED_HOST_PATH_TYPES.contains(type_.as_str()) {
            all_errs.push(not_supported(
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

fn validate_nfs_pv_source(value: &serde_json::Value, path: &Path) -> ErrorList {
    let nfs = match parse_source::<internal_pv::NFSVolumeSource>(value, path, "nfs") {
        Ok(parsed) => parsed,
        Err(errs) => return errs,
    };

    let mut all_errs = ErrorList::new();
    if nfs.server.is_empty() {
        all_errs.push(required(&path.child("server"), "server is required"));
    }
    if nfs.path.is_empty() {
        all_errs.push(required(&path.child("path"), "path is required"));
    } else {
        all_errs.extend(validate_absolute_path(&nfs.path, &path.child("path")));
    }

    all_errs
}

fn validate_iscsi_pv_source(value: &serde_json::Value, path: &Path) -> ErrorList {
    let iscsi = match parse_source::<internal_pv::ISCSIPersistentVolumeSource>(value, path, "iscsi")
    {
        Ok(parsed) => parsed,
        Err(errs) => return errs,
    };

    let mut all_errs = ErrorList::new();
    if iscsi.target_portal.is_empty() {
        all_errs.push(required(
            &path.child("targetPortal"),
            "targetPortal is required",
        ));
    }

    if iscsi.iqn.is_empty() {
        all_errs.push(required(&path.child("iqn"), "iqn is required"));
    } else if !iscsi.iqn.starts_with("iqn")
        && !iscsi.iqn.starts_with("eui")
        && !iscsi.iqn.starts_with("naa")
    {
        all_errs.push(invalid(
            &path.child("iqn"),
            BadValue::String(iscsi.iqn.clone()),
            "must be valid format starting with iqn, eui, or naa",
        ));
    }

    all_errs.extend(validate_nonnegative_field(
        iscsi.lun as i64,
        &path.child("lun"),
    ));
    if iscsi.lun < 0 || iscsi.lun > 255 {
        all_errs.push(invalid(
            &path.child("lun"),
            BadValue::Int(iscsi.lun as i64),
            "must be in the range 0-255",
        ));
    }

    if (iscsi.chap_auth_discovery || iscsi.chap_auth_session) && iscsi.secret_ref.is_none() {
        all_errs.push(required(&path.child("secretRef"), "secretRef is required"));
    }

    if let Some(ref secret_ref) = iscsi.secret_ref {
        all_errs.extend(validate_pv_secret_reference(
            secret_ref,
            &path.child("secretRef"),
        ));
    }

    if let Some(ref initiator) = iscsi.initiator_name {
        if !initiator.starts_with("iqn")
            && !initiator.starts_with("eui")
            && !initiator.starts_with("naa")
        {
            all_errs.push(invalid(
                &path.child("initiatorName"),
                BadValue::String(initiator.clone()),
                "must be valid format starting with iqn, eui, or naa",
            ));
        }
    }

    all_errs
}

fn validate_fc_pv_source(value: &serde_json::Value, path: &Path) -> ErrorList {
    let fc = match parse_source::<internal_pv::FCVolumeSource>(value, path, "fc") {
        Ok(parsed) => parsed,
        Err(errs) => return errs,
    };

    let mut all_errs = ErrorList::new();
    if fc.target_wwns.is_empty() && fc.wwids.is_empty() {
        all_errs.push(required(
            &path.child("targetWWNs"),
            "must specify either targetWWNs or wwids, but not both",
        ));
    }

    if !fc.target_wwns.is_empty() && !fc.wwids.is_empty() {
        all_errs.push(invalid(
            &path.child("targetWWNs"),
            BadValue::String(format!("{:?}", fc.target_wwns)),
            "targetWWNs and wwids can not be specified simultaneously",
        ));
    }

    if !fc.target_wwns.is_empty() {
        if fc.lun.is_none() {
            all_errs.push(required(
                &path.child("lun"),
                "lun is required if targetWWNs is specified",
            ));
        } else if let Some(lun) = fc.lun {
            if lun < 0 || lun > 255 {
                all_errs.push(invalid(
                    &path.child("lun"),
                    BadValue::Int(lun as i64),
                    "must be in the range 0-255",
                ));
            }
        }
    }

    all_errs
}

fn validate_local_volume_source(
    local: &crate::core::v1::volume::LocalVolumeSource,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if local.path.is_empty() {
        all_errs.push(required(&path.child("path"), "path is required"));
        return all_errs;
    }

    all_errs.extend(validate_no_backstep_segments(
        &local.path,
        &path.child("path"),
    ));

    all_errs
}

fn validate_no_backstep_segments(path_value: &str, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if path_value
        .split('/')
        .filter(|segment| !segment.is_empty())
        .any(|segment| segment == "..")
    {
        all_errs.push(invalid(
            path,
            BadValue::String(path_value.to_string()),
            "must not contain '..' path segments",
        ));
    }
    all_errs
}

fn validate_csi_pv_source(value: &serde_json::Value, path: &Path) -> ErrorList {
    let csi = match parse_source::<internal_pv::CSIPersistentVolumeSource>(value, path, "csi") {
        Ok(parsed) => parsed,
        Err(errs) => return errs,
    };

    let mut all_errs = ErrorList::new();
    all_errs.extend(validate_csi_driver_name(&csi.driver, &path.child("driver")));

    if csi.volume_handle.is_empty() {
        all_errs.push(required(
            &path.child("volumeHandle"),
            "volumeHandle is required",
        ));
    }

    if let Some(ref secret) = csi.controller_publish_secret_ref {
        all_errs.extend(validate_pv_secret_reference(
            secret,
            &path.child("controllerPublishSecretRef"),
        ));
    }
    if let Some(ref secret) = csi.controller_expand_secret_ref {
        all_errs.extend(validate_pv_secret_reference(
            secret,
            &path.child("controllerExpandSecretRef"),
        ));
    }
    if let Some(ref secret) = csi.node_publish_secret_ref {
        all_errs.extend(validate_pv_secret_reference(
            secret,
            &path.child("nodePublishSecretRef"),
        ));
    }
    if let Some(ref secret) = csi.node_expand_secret_ref {
        all_errs.extend(validate_pv_secret_reference(
            secret,
            &path.child("nodeExpandSecretRef"),
        ));
    }

    all_errs
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

fn validate_pv_secret_reference(
    secret_ref: &crate::core::internal::SecretReference,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if secret_ref.name.is_empty() {
        all_errs.push(required(&path.child("name"), "name is required"));
    } else {
        for err_msg in is_dns1123_subdomain(&secret_ref.name) {
            all_errs.push(invalid(
                &path.child("name"),
                BadValue::String(secret_ref.name.clone()),
                &err_msg,
            ));
        }
    }

    if secret_ref.namespace.is_empty() {
        all_errs.push(required(&path.child("namespace"), "namespace is required"));
    } else {
        for err_msg in is_dns1123_label(&secret_ref.namespace) {
            all_errs.push(invalid(
                &path.child("namespace"),
                BadValue::String(secret_ref.namespace.clone()),
                &err_msg,
            ));
        }
    }

    all_errs
}

fn validate_data_source(data_source: &TypedLocalObjectReference, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if data_source.name.as_deref().unwrap_or("").is_empty() {
        all_errs.push(required(&path.child("name"), "name is required"));
    }
    if data_source.kind.as_deref().unwrap_or("").is_empty() {
        all_errs.push(required(&path.child("kind"), "kind is required"));
    }

    let api_group = data_source.api_group.as_deref().unwrap_or("");
    if api_group.is_empty() && data_source.kind.as_deref().unwrap_or("") != "PersistentVolumeClaim"
    {
        all_errs.push(invalid(
            path,
            BadValue::String(data_source.kind.clone().unwrap_or_default()),
            "must be 'PersistentVolumeClaim' when referencing the default apiGroup",
        ));
    }
    if !api_group.is_empty() {
        for err_msg in is_dns1123_subdomain(api_group) {
            all_errs.push(invalid(
                &path.child("apiGroup"),
                BadValue::String(api_group.to_string()),
                &err_msg,
            ));
        }
    }

    all_errs
}

fn validate_data_source_ref(data_source_ref: &TypedObjectReference, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if data_source_ref.name.as_deref().unwrap_or("").is_empty() {
        all_errs.push(required(&path.child("name"), "name is required"));
    }
    if data_source_ref.kind.as_deref().unwrap_or("").is_empty() {
        all_errs.push(required(&path.child("kind"), "kind is required"));
    }

    let api_group = data_source_ref.api_group.as_deref().unwrap_or("");
    if api_group.is_empty()
        && data_source_ref.kind.as_deref().unwrap_or("") != "PersistentVolumeClaim"
    {
        all_errs.push(invalid(
            path,
            BadValue::String(data_source_ref.kind.clone().unwrap_or_default()),
            "must be 'PersistentVolumeClaim' when referencing the default apiGroup",
        ));
    }
    if !api_group.is_empty() {
        for err_msg in is_dns1123_subdomain(api_group) {
            all_errs.push(invalid(
                &path.child("apiGroup"),
                BadValue::String(api_group.to_string()),
                &err_msg,
            ));
        }
    }

    if let Some(ref namespace) = data_source_ref.namespace {
        if !namespace.is_empty() {
            for err_msg in is_dns1123_label(namespace) {
                all_errs.push(invalid(
                    &path.child("namespace"),
                    BadValue::String(namespace.clone()),
                    &err_msg,
                ));
            }
        }
    }

    all_errs
}

fn is_data_source_equal_data_source_ref(
    data_source: &TypedLocalObjectReference,
    data_source_ref: &TypedObjectReference,
) -> bool {
    data_source.api_group == data_source_ref.api_group
        && data_source.kind == data_source_ref.kind
        && data_source.name == data_source_ref.name
}

fn host_path_is_root(value: &serde_json::Value) -> bool {
    value
        .get("path")
        .and_then(|path| path.as_str())
        .map_or(false, |path| path == "/")
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::meta::{LabelSelector, ObjectMeta};
    use crate::common::validation::ErrorType;
    use crate::common::{Quantity, TypeMeta};
    use crate::core::v1::persistent_volume::{
        PersistentVolume, PersistentVolumeClaim, TypedObjectReference, VolumeResourceRequirements,
    };
    use crate::core::v1::reference::TypedLocalObjectReference;
    use crate::core::v1::volume::LocalVolumeSource;
    use std::collections::BTreeMap;

    #[test]
    fn test_validate_pv_missing_access_modes() {
        let pv = PersistentVolume {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-pv".to_string()),
                ..Default::default()
            }),
            spec: Some(PersistentVolumeSpec {
                access_modes: vec![], // Missing
                capacity: {
                    let mut map = BTreeMap::new();
                    map.insert("storage".to_string(), Quantity::from_str("1Gi"));
                    map
                },
                persistent_volume_source: Some(PersistentVolumeSource {
                    host_path: Some(serde_json::json!({"path": "/data"})),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            status: None,
        };

        let errs = validate_persistent_volume(&pv, &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.detail.contains("at least 1 access mode is required"))
        );
    }

    #[test]
    fn test_validate_pv_missing_capacity() {
        let pv = PersistentVolume {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-pv".to_string()),
                ..Default::default()
            }),
            spec: Some(PersistentVolumeSpec {
                access_modes: vec!["ReadWriteOnce".to_string()],
                capacity: BTreeMap::new(), // Missing
                persistent_volume_source: Some(PersistentVolumeSource {
                    host_path: Some(serde_json::json!({"path": "/data"})),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            status: None,
        };

        let errs = validate_persistent_volume(&pv, &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.detail.contains("capacity is required"))
        );
    }

    #[test]
    fn test_validate_pv_invalid_access_mode() {
        let pv = PersistentVolume {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-pv".to_string()),
                ..Default::default()
            }),
            spec: Some(PersistentVolumeSpec {
                access_modes: vec!["InvalidMode".to_string()],
                capacity: {
                    let mut map = BTreeMap::new();
                    map.insert("storage".to_string(), Quantity::from_str("1Gi"));
                    map
                },
                persistent_volume_source: Some(PersistentVolumeSource {
                    host_path: Some(serde_json::json!({"path": "/data"})),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            status: None,
        };

        let errs = validate_persistent_volume(&pv, &Path::nil());
        assert!(!errs.is_empty());
        assert!(errs
            .errors
            .iter()
            .any(|e| e.field.contains("accessModes") && e.error_type == ErrorType::NotSupported));
    }

    #[test]
    fn test_validate_pv_read_write_once_pod_with_others() {
        let pv = PersistentVolume {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-pv".to_string()),
                ..Default::default()
            }),
            spec: Some(PersistentVolumeSpec {
                access_modes: vec!["ReadWriteOncePod".to_string(), "ReadWriteOnce".to_string()],
                capacity: {
                    let mut map = BTreeMap::new();
                    map.insert("storage".to_string(), Quantity::from_str("1Gi"));
                    map
                },
                persistent_volume_source: Some(PersistentVolumeSource {
                    host_path: Some(serde_json::json!({"path": "/data"})),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            status: None,
        };

        let errs = validate_persistent_volume(&pv, &Path::nil());
        assert!(!errs.is_empty());
        assert!(errs.errors.iter().any(|e| {
            e.detail
                .contains("may not use ReadWriteOncePod with other access modes")
        }));
    }

    #[test]
    fn test_validate_pv_no_volume_source() {
        let pv = PersistentVolume {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-pv".to_string()),
                ..Default::default()
            }),
            spec: Some(PersistentVolumeSpec {
                access_modes: vec!["ReadWriteOnce".to_string()],
                capacity: {
                    let mut map = BTreeMap::new();
                    map.insert("storage".to_string(), Quantity::from_str("1Gi"));
                    map
                },
                persistent_volume_source: None, // Missing
                ..Default::default()
            }),
            status: None,
        };

        let errs = validate_persistent_volume(&pv, &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.detail.contains("must specify a volume source"))
        );
    }

    #[test]
    fn test_validate_pv_multiple_volume_sources() {
        let pv = PersistentVolume {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-pv".to_string()),
                ..Default::default()
            }),
            spec: Some(PersistentVolumeSpec {
                access_modes: vec!["ReadWriteOnce".to_string()],
                capacity: {
                    let mut map = BTreeMap::new();
                    map.insert("storage".to_string(), Quantity::from_str("1Gi"));
                    map
                },
                persistent_volume_source: Some(PersistentVolumeSource {
                    host_path: Some(serde_json::json!({"path": "/data"})),
                    nfs: Some(serde_json::json!({"server": "server", "path": "/path"})),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            status: None,
        };

        let errs = validate_persistent_volume(&pv, &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.detail.contains("may not specify more than 1 volume type"))
        );
    }

    #[test]
    fn test_validate_pv_invalid_reclaim_policy() {
        let pv = PersistentVolume {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-pv".to_string()),
                ..Default::default()
            }),
            spec: Some(PersistentVolumeSpec {
                access_modes: vec!["ReadWriteOnce".to_string()],
                capacity: {
                    let mut map = BTreeMap::new();
                    map.insert("storage".to_string(), Quantity::from_str("1Gi"));
                    map
                },
                persistent_volume_source: Some(PersistentVolumeSource {
                    host_path: Some(serde_json::json!({"path": "/data"})),
                    ..Default::default()
                }),
                persistent_volume_reclaim_policy: Some("InvalidPolicy".to_string()),
                ..Default::default()
            }),
            status: None,
        };

        let errs = validate_persistent_volume(&pv, &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.field.contains("persistentVolumeReclaimPolicy")
                    && e.error_type == ErrorType::NotSupported)
        );
    }

    #[test]
    fn test_validate_pv_valid() {
        let pv = PersistentVolume {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-pv".to_string()),
                ..Default::default()
            }),
            spec: Some(PersistentVolumeSpec {
                access_modes: vec!["ReadWriteOnce".to_string()],
                capacity: {
                    let mut map = BTreeMap::new();
                    map.insert("storage".to_string(), Quantity::from_str("1Gi"));
                    map
                },
                persistent_volume_source: Some(PersistentVolumeSource {
                    host_path: Some(serde_json::json!({"path": "/data"})),
                    ..Default::default()
                }),
                persistent_volume_reclaim_policy: Some("Retain".to_string()),
                volume_mode: Some("Filesystem".to_string()),
                ..Default::default()
            }),
            status: None,
        };

        let errs = validate_persistent_volume(&pv, &Path::nil());
        assert!(errs.is_empty(), "Valid PV should not produce errors");
    }

    #[test]
    fn test_validate_pv_host_path_root_recycle_forbidden() {
        let pv = PersistentVolume {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-pv".to_string()),
                ..Default::default()
            }),
            spec: Some(PersistentVolumeSpec {
                access_modes: vec!["ReadWriteOnce".to_string()],
                capacity: {
                    let mut map = BTreeMap::new();
                    map.insert("storage".to_string(), Quantity::from_str("1Gi"));
                    map
                },
                persistent_volume_source: Some(PersistentVolumeSource {
                    host_path: Some(serde_json::json!({"path": "/"})),
                    ..Default::default()
                }),
                persistent_volume_reclaim_policy: Some("Recycle".to_string()),
                ..Default::default()
            }),
            status: None,
        };

        let errs = validate_persistent_volume(&pv, &Path::nil());
        assert!(!errs.is_empty());
        assert!(errs.errors.iter().any(|e| {
            e.field.contains("persistentVolumeReclaimPolicy")
                && e.error_type == ErrorType::Forbidden
        }));
    }

    #[test]
    fn test_validate_pv_local_requires_node_affinity() {
        let pv = PersistentVolume {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-pv".to_string()),
                ..Default::default()
            }),
            spec: Some(PersistentVolumeSpec {
                access_modes: vec!["ReadWriteOnce".to_string()],
                capacity: {
                    let mut map = BTreeMap::new();
                    map.insert("storage".to_string(), Quantity::from_str("1Gi"));
                    map
                },
                persistent_volume_source: Some(PersistentVolumeSource {
                    local: Some(LocalVolumeSource {
                        path: "/data".to_string(),
                        fs_type: None,
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            status: None,
        };

        let errs = validate_persistent_volume(&pv, &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.field.contains("nodeAffinity") && e.error_type == ErrorType::Required)
        );
    }

    #[test]
    fn test_validate_pvc_missing_access_modes() {
        let pvc = PersistentVolumeClaim {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-pvc".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(PersistentVolumeClaimSpec {
                access_modes: vec![], // Missing
                resources: Some(VolumeResourceRequirements {
                    requests: {
                        let mut map = BTreeMap::new();
                        map.insert("storage".to_string(), Quantity::from_str("1Gi"));
                        map
                    },
                    ..Default::default()
                }),
                ..Default::default()
            }),
            status: None,
        };

        let errs = validate_persistent_volume_claim(&pvc, &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.detail.contains("at least 1 access mode is required"))
        );
    }

    #[test]
    fn test_validate_pvc_missing_storage_request() {
        let pvc = PersistentVolumeClaim {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-pvc".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(PersistentVolumeClaimSpec {
                access_modes: vec!["ReadWriteOnce".to_string()],
                resources: Some(VolumeResourceRequirements {
                    requests: BTreeMap::new(), // Missing storage
                    ..Default::default()
                }),
                ..Default::default()
            }),
            status: None,
        };

        let errs = validate_persistent_volume_claim(&pvc, &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.detail.contains("storage request is required"))
        );
    }

    #[test]
    fn test_validate_pvc_valid() {
        let pvc = PersistentVolumeClaim {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-pvc".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(PersistentVolumeClaimSpec {
                access_modes: vec!["ReadWriteOnce".to_string()],
                resources: Some(VolumeResourceRequirements {
                    requests: {
                        let mut map = BTreeMap::new();
                        map.insert("storage".to_string(), Quantity::from_str("1Gi"));
                        map
                    },
                    ..Default::default()
                }),
                ..Default::default()
            }),
            status: None,
        };

        let errs = validate_persistent_volume_claim(&pvc, &Path::nil());
        assert!(errs.is_empty(), "Valid PVC should not produce errors");
    }

    #[test]
    fn test_validate_pvc_selector_match_expression_requires_values() {
        let pvc = PersistentVolumeClaim {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-pvc".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(PersistentVolumeClaimSpec {
                access_modes: vec!["ReadWriteOnce".to_string()],
                selector: Some(LabelSelector {
                    match_labels: BTreeMap::new(),
                    match_expressions: vec![crate::common::meta::LabelSelectorRequirement {
                        key: "disk".to_string(),
                        operator: "In".to_string(),
                        values: vec![],
                    }],
                }),
                resources: Some(VolumeResourceRequirements {
                    requests: {
                        let mut map = BTreeMap::new();
                        map.insert("storage".to_string(), Quantity::from_str("1Gi"));
                        map
                    },
                    ..Default::default()
                }),
                ..Default::default()
            }),
            status: None,
        };

        let errs = validate_persistent_volume_claim(&pvc, &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.field.contains("matchExpressions"))
        );
    }

    #[test]
    fn test_validate_pvc_selector_match_expression_invalid_value() {
        let pvc = PersistentVolumeClaim {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-pvc".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(PersistentVolumeClaimSpec {
                access_modes: vec!["ReadWriteOnce".to_string()],
                selector: Some(LabelSelector {
                    match_labels: BTreeMap::new(),
                    match_expressions: vec![crate::common::meta::LabelSelectorRequirement {
                        key: "disk".to_string(),
                        operator: "In".to_string(),
                        values: vec!["bad*value".to_string()],
                    }],
                }),
                resources: Some(VolumeResourceRequirements {
                    requests: {
                        let mut map = BTreeMap::new();
                        map.insert("storage".to_string(), Quantity::from_str("1Gi"));
                        map
                    },
                    ..Default::default()
                }),
                ..Default::default()
            }),
            status: None,
        };

        let errs = validate_persistent_volume_claim(&pvc, &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.field.contains("matchExpressions") && e.detail.contains("valid label"))
        );
    }

    #[test]
    fn test_validate_pvc_data_source_ref_namespace_conflict() {
        let pvc = PersistentVolumeClaim {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-pvc".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(PersistentVolumeClaimSpec {
                access_modes: vec!["ReadWriteOnce".to_string()],
                resources: Some(VolumeResourceRequirements {
                    requests: {
                        let mut map = BTreeMap::new();
                        map.insert("storage".to_string(), Quantity::from_str("1Gi"));
                        map
                    },
                    ..Default::default()
                }),
                data_source: Some(TypedLocalObjectReference {
                    api_group: None,
                    kind: Some("PersistentVolumeClaim".to_string()),
                    name: Some("source".to_string()),
                }),
                data_source_ref: Some(TypedObjectReference {
                    api_group: None,
                    kind: Some("PersistentVolumeClaim".to_string()),
                    name: Some("source".to_string()),
                    namespace: Some("default".to_string()),
                }),
                ..Default::default()
            }),
            status: None,
        };

        let errs = validate_persistent_volume_claim(&pvc, &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.detail.contains("dataSourceRef.namespace"))
        );
    }

    #[test]
    fn test_validate_pv_nfs_requires_absolute_path() {
        let pv = PersistentVolume {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-pv".to_string()),
                ..Default::default()
            }),
            spec: Some(PersistentVolumeSpec {
                access_modes: vec!["ReadWriteOnce".to_string()],
                capacity: {
                    let mut map = BTreeMap::new();
                    map.insert("storage".to_string(), Quantity::from_str("1Gi"));
                    map
                },
                persistent_volume_source: Some(PersistentVolumeSource {
                    nfs: Some(serde_json::json!({"server": "nfs", "path": "data"})),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            status: None,
        };

        let errs = validate_persistent_volume(&pv, &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.field.contains("path") && e.error_type == ErrorType::Invalid)
        );
    }

    #[test]
    fn test_validate_pv_iscsi_chap_requires_secret_ref() {
        let pv = PersistentVolume {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-pv".to_string()),
                ..Default::default()
            }),
            spec: Some(PersistentVolumeSpec {
                access_modes: vec!["ReadWriteOnce".to_string()],
                capacity: {
                    let mut map = BTreeMap::new();
                    map.insert("storage".to_string(), Quantity::from_str("1Gi"));
                    map
                },
                persistent_volume_source: Some(PersistentVolumeSource {
                    iscsi: Some(serde_json::json!({
                        "targetPortal": "10.0.0.1:3260",
                        "iqn": "iqn.2001-04.com.example:storage",
                        "lun": 1,
                        "chapAuthDiscovery": true
                    })),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            status: None,
        };

        let errs = validate_persistent_volume(&pv, &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.field.contains("secretRef") && e.error_type == ErrorType::Required)
        );
    }

    #[test]
    fn test_validate_pv_csi_driver_required() {
        let pv = PersistentVolume {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-pv".to_string()),
                ..Default::default()
            }),
            spec: Some(PersistentVolumeSpec {
                access_modes: vec!["ReadWriteOnce".to_string()],
                capacity: {
                    let mut map = BTreeMap::new();
                    map.insert("storage".to_string(), Quantity::from_str("1Gi"));
                    map
                },
                persistent_volume_source: Some(PersistentVolumeSource {
                    csi: Some(serde_json::json!({
                        "driver": "",
                        "volumeHandle": "vol-1"
                    })),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            status: None,
        };

        let errs = validate_persistent_volume(&pv, &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.field.contains("driver") && e.error_type == ErrorType::Required)
        );
    }
}
