//! Validation for Kubernetes Storage API types
//!
//! Ported from k8s.io/kubernetes/pkg/apis/storage/validation/validation.go

use std::collections::{BTreeMap, BTreeSet, HashSet};

use crate::common::meta::label_selector_operator;
use crate::common::validation::{
    BadValue, ErrorList, Path, duplicate, invalid, name_is_dns_subdomain, not_supported, required,
    too_long, validate_labels, validate_object_meta, validate_qualified_name,
};
use crate::common::{
    LabelSelector, ObjectMeta, PersistentVolumeReclaimPolicy, TopologySelectorTerm,
};
use crate::storage::v1 as storage_v1;
use crate::storage::v1alpha1 as storage_v1alpha1;
use crate::storage::v1beta1 as storage_v1beta1;

const MAX_PROVISIONER_PARAMETER_SIZE: usize = 256 * 1024;
const MAX_PROVISIONER_PARAMETER_LEN: usize = 512;

const MAX_ATTACHED_VOLUME_METADATA_SIZE: usize = 256 * 1024;
const MAX_VOLUME_ERROR_MESSAGE_SIZE: usize = 1024;

const CSI_NODE_ID_MAX_LENGTH: usize = 192;
const CSI_NODE_ID_LONGER_MAX_LENGTH: usize = 256;

// ============================================================================
// Helper Types
// ============================================================================

/// Options for CSINode validation.
#[derive(Clone, Copy, Debug, Default)]
pub struct CSINodeValidationOptions {
    pub allow_long_node_id: bool,
}

// ============================================================================
// Shared Validation Helpers
// ============================================================================

fn validate_provisioner(provisioner: &str, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if provisioner.is_empty() {
        all_errs.push(required(path, "provisioner is required"));
        return all_errs;
    }

    let lower = provisioner.to_lowercase();
    all_errs.extend(validate_qualified_name(&lower, path));
    all_errs
}

fn validate_parameters(
    params: &BTreeMap<String, String>,
    allow_empty: bool,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if params.len() > MAX_PROVISIONER_PARAMETER_LEN {
        all_errs.push(too_long(path, MAX_PROVISIONER_PARAMETER_LEN));
        return all_errs;
    }

    let mut total_size = 0usize;
    for (key, value) in params {
        if key.is_empty() {
            all_errs.push(invalid(
                path,
                BadValue::String(key.clone()),
                "key must not be empty",
            ));
        }
        total_size += key.len() + value.len();
    }

    if total_size > MAX_PROVISIONER_PARAMETER_SIZE {
        all_errs.push(too_long(path, MAX_PROVISIONER_PARAMETER_SIZE));
    }

    if !allow_empty && params.is_empty() {
        all_errs.push(required(path, "must contain at least one key/value pair"));
    }

    all_errs
}

fn validate_reclaim_policy(
    policy: &Option<PersistentVolumeReclaimPolicy>,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let Some(_policy) = policy else {
        all_errs.push(required(path, "reclaimPolicy is required"));
        return all_errs;
    };

    all_errs
}

fn validate_volume_binding_mode(mode: &Option<String>, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let Some(mode) = mode else {
        all_errs.push(required(path, "volumeBindingMode is required"));
        return all_errs;
    };

    let supported = ["Immediate", "WaitForFirstConsumer"];
    if !supported.contains(&mode.as_str()) {
        all_errs.push(not_supported(
            path,
            BadValue::String(mode.clone()),
            &supported,
        ));
    }
    all_errs
}

fn validate_topology_selector_term(term: &TopologySelectorTerm, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    for (i, expr) in term.match_label_expressions.iter().enumerate() {
        let expr_path = path.child("matchLabelExpressions").index(i);
        if expr.key.is_empty() {
            all_errs.push(required(&expr_path.child("key"), "key is required"));
        } else {
            all_errs.extend(validate_qualified_name(&expr.key, &expr_path.child("key")));
        }

        if expr.values.is_empty() {
            all_errs.push(required(
                &expr_path.child("values"),
                "values must be non-empty",
            ));
        }

        for (j, value) in expr.values.iter().enumerate() {
            if value.is_empty() {
                all_errs.push(required(
                    &expr_path.child("values").index(j),
                    "value must be non-empty",
                ));
            }
        }
    }
    all_errs
}

fn validate_allowed_topologies(topologies: &[TopologySelectorTerm], path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if topologies.is_empty() {
        return all_errs;
    }

    let mut seen = HashSet::new();
    for (i, term) in topologies.iter().enumerate() {
        let term_path = path.index(i);
        all_errs.extend(validate_topology_selector_term(term, &term_path));

        let mut normalized = BTreeMap::new();
        for expr in &term.match_label_expressions {
            let mut values = BTreeSet::new();
            for value in &expr.values {
                values.insert(value.clone());
            }
            normalized.insert(expr.key.clone(), values);
        }

        let key = format!("{normalized:?}");
        if !seen.insert(key) {
            all_errs.push(duplicate(
                &term_path.child("matchLabelExpressions"),
                BadValue::String(String::new()),
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

        let operator = requirement.operator.as_str();
        match operator {
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
            }
        }
    }

    all_errs
}
fn validate_volume_attachment_source(
    source: &storage_v1::VolumeAttachmentSource,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let has_inline = source.inline_volume_spec.is_some();
    let has_pv = source.persistent_volume_name.is_some();

    match (has_inline, has_pv) {
        (false, false) => {
            all_errs.push(required(
                path,
                "must specify exactly one of inlineVolumeSpec and persistentVolumeName",
            ));
        }
        (true, true) => {
            all_errs.push(invalid(
                path,
                BadValue::String("both inline and persistent volume set".to_string()),
                "must specify exactly one of inlineVolumeSpec and persistentVolumeName",
            ));
        }
        _ => {}
    }

    if let Some(name) = source.persistent_volume_name.as_deref() {
        if name.is_empty() {
            all_errs.push(required(
                &path.child("persistentVolumeName"),
                "must specify non empty persistentVolumeName",
            ));
        }
    }

    all_errs
}

fn validate_node_name(node_name: &str, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if node_name.is_empty() {
        all_errs.push(required(path, "nodeName is required"));
        return all_errs;
    }
    for msg in crate::common::validation::is_dns1123_subdomain(node_name) {
        all_errs.push(invalid(path, BadValue::String(node_name.to_string()), &msg));
    }
    all_errs
}

fn validate_attachment_metadata(metadata: &BTreeMap<String, String>, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let size: usize = metadata.iter().map(|(k, v)| k.len() + v.len()).sum();
    if size > MAX_ATTACHED_VOLUME_METADATA_SIZE {
        all_errs.push(too_long(path, MAX_ATTACHED_VOLUME_METADATA_SIZE));
    }
    all_errs
}

fn validate_volume_error(error: &Option<storage_v1::VolumeError>, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let Some(error) = error else {
        return all_errs;
    };

    if error.message.len() > MAX_VOLUME_ERROR_MESSAGE_SIZE {
        all_errs.push(too_long(
            &path.child("message"),
            MAX_VOLUME_ERROR_MESSAGE_SIZE,
        ));
    }

    if let Some(code) = error.error_code {
        if code < 0 {
            all_errs.push(invalid(
                &path.child("errorCode"),
                BadValue::Int(code as i64),
                "must be greater than or equal to 0",
            ));
        }
    }

    all_errs
}

fn volume_error_v1beta1_to_v1(error: &storage_v1beta1::VolumeError) -> storage_v1::VolumeError {
    storage_v1::VolumeError {
        time: error.time.clone(),
        message: error.message.clone(),
        error_code: error.error_code,
    }
}

fn volume_error_v1alpha1_to_v1(error: &storage_v1alpha1::VolumeError) -> storage_v1::VolumeError {
    storage_v1::VolumeError {
        time: error.time.clone(),
        message: error.message.clone(),
        error_code: error.error_code,
    }
}

fn validate_csi_node_driver_node_id(node_id: &str, path: &Path, allow_long: bool) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if node_id.is_empty() {
        all_errs.push(required(path, "nodeID is required"));
        return all_errs;
    }
    let max = if allow_long {
        CSI_NODE_ID_LONGER_MAX_LENGTH
    } else {
        CSI_NODE_ID_MAX_LENGTH
    };
    if node_id.len() > max {
        all_errs.push(invalid(
            path,
            BadValue::String(node_id.to_string()),
            &format!("must be {max} characters or less"),
        ));
    }
    all_errs
}

// ============================================================================
// StorageClass Validation
// ============================================================================

pub fn validate_storage_class_v1(obj: &storage_v1::StorageClass) -> ErrorList {
    let base_path = Path::nil();
    validate_storage_class_common(
        obj.metadata.as_ref().unwrap_or(&ObjectMeta::default()),
        &obj.provisioner,
        &obj.parameters,
        &obj.reclaim_policy,
        obj.volume_binding_mode.as_ref().map(|mode| match mode {
            storage_v1::VolumeBindingMode::Immediate => "Immediate".to_string(),
            storage_v1::VolumeBindingMode::WaitForFirstConsumer => {
                "WaitForFirstConsumer".to_string()
            }
        }),
        &obj.allowed_topologies,
        &base_path,
    )
}

pub fn validate_storage_class_v1beta1(obj: &storage_v1beta1::StorageClass) -> ErrorList {
    let base_path = Path::nil();
    validate_storage_class_common(
        obj.metadata.as_ref().unwrap_or(&ObjectMeta::default()),
        &obj.provisioner,
        &obj.parameters,
        &obj.reclaim_policy,
        obj.volume_binding_mode.as_ref().map(|mode| match mode {
            storage_v1beta1::VolumeBindingMode::Immediate => "Immediate".to_string(),
            storage_v1beta1::VolumeBindingMode::WaitForFirstConsumer => {
                "WaitForFirstConsumer".to_string()
            }
        }),
        &obj.allowed_topologies,
        &base_path,
    )
}

fn validate_storage_class_common(
    metadata: &ObjectMeta,
    provisioner: &str,
    parameters: &BTreeMap<String, String>,
    reclaim_policy: &Option<PersistentVolumeReclaimPolicy>,
    volume_binding_mode: Option<String>,
    allowed_topologies: &[TopologySelectorTerm],
    base_path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    all_errs.extend(validate_object_meta(
        metadata,
        false,
        name_is_dns_subdomain,
        &base_path.child("metadata"),
    ));

    all_errs.extend(validate_provisioner(
        provisioner,
        &base_path.child("provisioner"),
    ));
    all_errs.extend(validate_parameters(
        parameters,
        true,
        &base_path.child("parameters"),
    ));
    all_errs.extend(validate_reclaim_policy(
        reclaim_policy,
        &base_path.child("reclaimPolicy"),
    ));
    all_errs.extend(validate_volume_binding_mode(
        &volume_binding_mode,
        &base_path.child("volumeBindingMode"),
    ));
    all_errs.extend(validate_allowed_topologies(
        allowed_topologies,
        &base_path.child("allowedTopologies"),
    ));

    all_errs
}
// ============================================================================
// VolumeAttachment Validation
// ============================================================================

pub fn validate_volume_attachment_v1(obj: &storage_v1::VolumeAttachment) -> ErrorList {
    let mut all_errs = validate_volume_attachment_common(
        obj.metadata.as_ref().unwrap_or(&ObjectMeta::default()),
        &obj.spec.attacher,
        &obj.spec.source,
        &obj.spec.node_name,
        &obj.status,
        &Path::nil(),
    );

    // v1 adds CSI driver name validation for attacher.
    all_errs.extend(validate_csi_driver_name(
        &obj.spec.attacher,
        &Path::nil().child("spec").child("attacher"),
    ));

    all_errs
}

pub fn validate_volume_attachment_v1beta1(obj: &storage_v1beta1::VolumeAttachment) -> ErrorList {
    validate_volume_attachment_common(
        obj.metadata.as_ref().unwrap_or(&ObjectMeta::default()),
        &obj.spec.attacher,
        &storage_v1::VolumeAttachmentSource {
            persistent_volume_name: obj.spec.source.persistent_volume_name.clone(),
            inline_volume_spec: obj.spec.source.inline_volume_spec.clone(),
        },
        &obj.spec.node_name,
        &obj.status
            .as_ref()
            .map(|status| storage_v1::VolumeAttachmentStatus {
                attached: status.attached,
                attachment_metadata: status.attachment_metadata.clone(),
                attach_error: status.attach_error.as_ref().map(volume_error_v1beta1_to_v1),
                detach_error: status.detach_error.as_ref().map(volume_error_v1beta1_to_v1),
            }),
        &Path::nil(),
    )
}

pub fn validate_volume_attachment_v1alpha1(obj: &storage_v1alpha1::VolumeAttachment) -> ErrorList {
    validate_volume_attachment_common(
        obj.metadata.as_ref().unwrap_or(&ObjectMeta::default()),
        &obj.spec.attacher,
        &storage_v1::VolumeAttachmentSource {
            persistent_volume_name: obj.spec.source.persistent_volume_name.clone(),
            inline_volume_spec: obj.spec.source.inline_volume_spec.clone(),
        },
        &obj.spec.node_name,
        &obj.status
            .as_ref()
            .map(|status| storage_v1::VolumeAttachmentStatus {
                attached: status.attached,
                attachment_metadata: status.attachment_metadata.clone(),
                attach_error: status
                    .attach_error
                    .as_ref()
                    .map(volume_error_v1alpha1_to_v1),
                detach_error: status
                    .detach_error
                    .as_ref()
                    .map(volume_error_v1alpha1_to_v1),
            }),
        &Path::nil(),
    )
}

fn validate_volume_attachment_common(
    metadata: &ObjectMeta,
    attacher: &str,
    source: &storage_v1::VolumeAttachmentSource,
    node_name: &str,
    status: &Option<storage_v1::VolumeAttachmentStatus>,
    base_path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    all_errs.extend(validate_object_meta(
        metadata,
        false,
        name_is_dns_subdomain,
        &base_path.child("metadata"),
    ));

    if attacher.is_empty() {
        all_errs.push(required(
            &base_path.child("spec").child("attacher"),
            "attacher is required",
        ));
    }

    all_errs.extend(validate_volume_attachment_source(
        source,
        &base_path.child("spec").child("source"),
    ));
    all_errs.extend(validate_node_name(
        node_name,
        &base_path.child("spec").child("nodeName"),
    ));

    if let Some(status) = status {
        all_errs.extend(validate_attachment_metadata(
            &status.attachment_metadata,
            &base_path.child("status").child("attachmentMetadata"),
        ));
        all_errs.extend(validate_volume_error(
            &status.attach_error,
            &base_path.child("status").child("attachError"),
        ));
        all_errs.extend(validate_volume_error(
            &status.detach_error,
            &base_path.child("status").child("detachError"),
        ));
    }

    all_errs
}

// ============================================================================
// CSIDriver Validation
// ============================================================================

pub fn validate_csi_driver_v1(obj: &storage_v1::CSIDriver) -> ErrorList {
    validate_csi_driver_common(obj)
}

pub fn validate_csi_driver_v1beta1(obj: &storage_v1beta1::CSIDriver) -> ErrorList {
    validate_csi_driver_common(&storage_v1::CSIDriver {
        type_meta: obj.type_meta.clone(),
        metadata: obj.metadata.clone(),
        spec: storage_v1::CSIDriverSpec {
            attach_required: obj.spec.attach_required,
            pod_info_on_mount: obj.spec.pod_info_on_mount,
            volume_lifecycle_modes: obj
                .spec
                .volume_lifecycle_modes
                .iter()
                .map(|mode| match mode {
                    storage_v1beta1::VolumeLifecycleMode::Persistent => {
                        storage_v1::VolumeLifecycleMode::Persistent
                    }
                    storage_v1beta1::VolumeLifecycleMode::Ephemeral => {
                        storage_v1::VolumeLifecycleMode::Ephemeral
                    }
                })
                .collect(),
            storage_capacity: obj.spec.storage_capacity,
            fs_group_policy: obj
                .spec
                .fs_group_policy
                .as_ref()
                .map(|policy| match policy {
                    storage_v1beta1::FSGroupPolicy::ReadWriteOnceWithFSType => {
                        storage_v1::FSGroupPolicy::ReadWriteOnceWithFSType
                    }
                    storage_v1beta1::FSGroupPolicy::File => storage_v1::FSGroupPolicy::File,
                    storage_v1beta1::FSGroupPolicy::None => storage_v1::FSGroupPolicy::None,
                }),
            token_requests: obj
                .spec
                .token_requests
                .iter()
                .map(|req| storage_v1::TokenRequest {
                    audience: req.audience.clone(),
                    expiration_seconds: req.expiration_seconds,
                })
                .collect(),
            requires_republish: obj.spec.requires_republish,
            se_linux_mount: obj.spec.se_linux_mount,
            node_allocatable_update_period_seconds: obj.spec.node_allocatable_update_period_seconds,
            service_account_token_in_secrets: None,
        },
    })
}

fn validate_csi_driver_common(obj: &storage_v1::CSIDriver) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let base_path = Path::nil();
    let default_meta = ObjectMeta::default();
    let metadata = obj.metadata.as_ref().unwrap_or(&default_meta);

    all_errs.extend(validate_object_meta(
        metadata,
        false,
        name_is_dns_subdomain,
        &base_path.child("metadata"),
    ));
    all_errs.extend(validate_csi_driver_name(
        metadata.name.as_deref().unwrap_or(""),
        &base_path.child("metadata").child("name"),
    ));
    all_errs.extend(validate_csi_driver_spec(
        &obj.spec,
        &base_path.child("spec"),
    ));

    all_errs
}

fn validate_csi_driver_name(name: &str, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    for msg in crate::common::validation::is_dns1123_subdomain(name) {
        all_errs.push(invalid(path, BadValue::String(name.to_string()), &msg));
    }
    all_errs
}

fn validate_csi_driver_spec(spec: &storage_v1::CSIDriverSpec, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if spec.attach_required.is_none() {
        all_errs.push(required(&path.child("attachRequired"), ""));
    }
    if spec.pod_info_on_mount.is_none() {
        all_errs.push(required(&path.child("podInfoOnMount"), ""));
    }
    if spec.storage_capacity.is_none() {
        all_errs.push(required(&path.child("storageCapacity"), ""));
    }

    let mut audiences = HashSet::new();
    for (i, request) in spec.token_requests.iter().enumerate() {
        let req_path = path.child("tokenRequests").index(i);
        if !audiences.insert(request.audience.clone()) {
            all_errs.push(duplicate(
                &req_path.child("audience"),
                BadValue::String(request.audience.clone()),
            ));
        }
        if let Some(exp) = request.expiration_seconds {
            if exp < 600 {
                all_errs.push(invalid(
                    &req_path.child("expirationSeconds"),
                    BadValue::Int(exp),
                    "may not specify a duration less than 10 minutes",
                ));
            }
            if exp > (1u64 << 32) as i64 {
                all_errs.push(invalid(
                    &req_path.child("expirationSeconds"),
                    BadValue::Int(exp),
                    "may not specify a duration larger than 2^32 seconds",
                ));
            }
        }
    }

    if let Some(period) = spec.node_allocatable_update_period_seconds {
        if period < 10 {
            all_errs.push(invalid(
                &path.child("nodeAllocatableUpdatePeriodSeconds"),
                BadValue::Int(period),
                "must be greater than or equal to 10 seconds",
            ));
        }
    }

    all_errs
}
// ============================================================================
// CSINode Validation
// ============================================================================

pub fn validate_csi_node_v1(
    obj: &storage_v1::CSINode,
    opts: CSINodeValidationOptions,
) -> ErrorList {
    validate_csi_node_common(
        obj.metadata.as_ref().unwrap_or(&ObjectMeta::default()),
        &obj.spec,
        opts,
    )
}

pub fn validate_csi_node_v1beta1(
    obj: &storage_v1beta1::CSINode,
    opts: CSINodeValidationOptions,
) -> ErrorList {
    let spec = storage_v1::CSINodeSpec {
        drivers: obj
            .spec
            .drivers
            .iter()
            .map(|driver| storage_v1::CSINodeDriver {
                name: driver.name.clone(),
                node_id: driver.node_id.clone(),
                topology_keys: driver.topology_keys.clone(),
                allocatable: driver
                    .allocatable
                    .as_ref()
                    .map(|alloc| storage_v1::VolumeNodeResources { count: alloc.count }),
            })
            .collect(),
    };
    validate_csi_node_common(
        obj.metadata.as_ref().unwrap_or(&ObjectMeta::default()),
        &spec,
        opts,
    )
}

fn validate_csi_node_common(
    metadata: &ObjectMeta,
    spec: &storage_v1::CSINodeSpec,
    opts: CSINodeValidationOptions,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let base_path = Path::nil();

    all_errs.extend(validate_object_meta(
        metadata,
        false,
        name_is_dns_subdomain,
        &base_path.child("metadata"),
    ));

    let mut driver_names = HashSet::new();
    for (i, driver) in spec.drivers.iter().enumerate() {
        let driver_path = base_path.child("spec").child("drivers").index(i);
        all_errs.extend(validate_csi_driver_name(
            &driver.name,
            &driver_path.child("name"),
        ));
        all_errs.extend(validate_csi_node_driver_node_id(
            &driver.node_id,
            &driver_path.child("nodeID"),
            opts.allow_long_node_id,
        ));
        if driver_names.contains(&driver.name) {
            all_errs.push(duplicate(
                &driver_path.child("name"),
                BadValue::String(driver.name.clone()),
            ));
        }
        driver_names.insert(driver.name.clone());

        let mut topo_keys = HashSet::new();
        for key in &driver.topology_keys {
            if key.is_empty() {
                all_errs.push(required(&driver_path.child("topologyKeys"), ""));
            }
            if !topo_keys.insert(key.clone()) {
                all_errs.push(duplicate(
                    &driver_path.child("topologyKeys"),
                    BadValue::String(key.clone()),
                ));
            }
            all_errs.extend(validate_qualified_name(
                key,
                &driver_path.child("topologyKeys"),
            ));
        }

        if let Some(alloc) = &driver.allocatable {
            if let Some(count) = alloc.count {
                if count < 0 {
                    all_errs.push(invalid(
                        &driver_path.child("allocatable").child("count"),
                        BadValue::Int(count as i64),
                        "must be non-negative",
                    ));
                }
            }
        }
    }

    all_errs
}

// ============================================================================
// CSIStorageCapacity Validation
// ============================================================================

pub fn validate_csi_storage_capacity_v1(obj: &storage_v1::CSIStorageCapacity) -> ErrorList {
    validate_csi_storage_capacity_common(obj)
}

pub fn validate_csi_storage_capacity_v1beta1(
    obj: &storage_v1beta1::CSIStorageCapacity,
) -> ErrorList {
    validate_csi_storage_capacity_common(&storage_v1::CSIStorageCapacity {
        type_meta: obj.type_meta.clone(),
        metadata: obj.metadata.clone(),
        node_topology: obj.node_topology.clone(),
        storage_class_name: obj.storage_class_name.clone(),
        capacity: obj.capacity.clone(),
        maximum_volume_size: obj.maximum_volume_size.clone(),
    })
}

pub fn validate_csi_storage_capacity_v1alpha1(
    obj: &storage_v1alpha1::CSIStorageCapacity,
) -> ErrorList {
    validate_csi_storage_capacity_common(&storage_v1::CSIStorageCapacity {
        type_meta: obj.type_meta.clone(),
        metadata: obj.metadata.clone(),
        node_topology: obj.node_topology.clone(),
        storage_class_name: obj.storage_class_name.clone(),
        capacity: obj.capacity.clone(),
        maximum_volume_size: obj.maximum_volume_size.clone(),
    })
}

fn validate_csi_storage_capacity_common(obj: &storage_v1::CSIStorageCapacity) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let base_path = Path::nil();
    let default_meta = ObjectMeta::default();
    let metadata = obj.metadata.as_ref().unwrap_or(&default_meta);

    all_errs.extend(validate_object_meta(
        metadata,
        true,
        name_is_dns_subdomain,
        &base_path.child("metadata"),
    ));

    if let Some(selector) = obj.node_topology.as_ref() {
        all_errs.extend(validate_label_selector(
            selector,
            &base_path.child("nodeTopology"),
        ));
    }

    if obj.storage_class_name.is_empty() {
        all_errs.push(required(
            &base_path.child("storageClassName"),
            "storageClassName is required",
        ));
    } else {
        for msg in crate::common::validation::is_dns1123_subdomain(&obj.storage_class_name) {
            all_errs.push(invalid(
                &base_path.child("storageClassName"),
                BadValue::String(obj.storage_class_name.clone()),
                &msg,
            ));
        }
    }

    all_errs
}

// ============================================================================
// VolumeAttributesClass Validation
// ============================================================================

pub fn validate_volume_attributes_class_v1(obj: &storage_v1::VolumeAttributesClass) -> ErrorList {
    validate_volume_attributes_class_common(obj)
}

pub fn validate_volume_attributes_class_v1beta1(
    obj: &storage_v1beta1::VolumeAttributesClass,
) -> ErrorList {
    validate_volume_attributes_class_common(&storage_v1::VolumeAttributesClass {
        type_meta: obj.type_meta.clone(),
        metadata: obj.metadata.clone(),
        driver_name: obj.driver_name.clone(),
        parameters: obj.parameters.clone(),
    })
}

pub fn validate_volume_attributes_class_v1alpha1(
    obj: &storage_v1alpha1::VolumeAttributesClass,
) -> ErrorList {
    validate_volume_attributes_class_common(&storage_v1::VolumeAttributesClass {
        type_meta: obj.type_meta.clone(),
        metadata: obj.metadata.clone(),
        driver_name: obj.driver_name.clone(),
        parameters: obj.parameters.clone(),
    })
}

fn validate_volume_attributes_class_common(obj: &storage_v1::VolumeAttributesClass) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let base_path = Path::nil();
    let default_meta = ObjectMeta::default();
    let metadata = obj.metadata.as_ref().unwrap_or(&default_meta);

    all_errs.extend(validate_object_meta(
        metadata,
        false,
        name_is_dns_subdomain,
        &base_path.child("metadata"),
    ));

    all_errs.extend(validate_provisioner(
        &obj.driver_name,
        &base_path.child("driverName"),
    ));

    all_errs.extend(validate_parameters(
        &obj.parameters,
        false,
        &base_path.child("parameters"),
    ));

    all_errs
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{ObjectMeta, TypeMeta};

    #[test]
    fn test_validate_storage_class_missing_provisioner() {
        let storage_class = storage_v1::StorageClass {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("fast".to_string()),
                ..Default::default()
            }),
            provisioner: String::new(),
            parameters: Default::default(),
            reclaim_policy: Some(PersistentVolumeReclaimPolicy::Delete),
            mount_options: vec![],
            allow_volume_expansion: None,
            volume_binding_mode: Some(storage_v1::VolumeBindingMode::Immediate),
            allowed_topologies: vec![],
        };

        let errors = validate_storage_class_v1(&storage_class);
        assert!(!errors.is_empty(), "expected validation errors");
    }

    #[test]
    fn test_validate_volume_attachment_requires_one_source() {
        let attachment = storage_v1::VolumeAttachment {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("attach".to_string()),
                ..Default::default()
            }),
            spec: storage_v1::VolumeAttachmentSpec {
                attacher: "example.com/driver".to_string(),
                source: storage_v1::VolumeAttachmentSource {
                    persistent_volume_name: None,
                    inline_volume_spec: None,
                },
                node_name: "node-1".to_string(),
            },
            status: None,
        };

        let errors = validate_volume_attachment_v1(&attachment);
        assert!(!errors.is_empty(), "expected validation errors");
    }

    #[test]
    fn test_validate_volume_attributes_class_requires_parameters() {
        let volume_class = storage_v1::VolumeAttributesClass {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("attrs".to_string()),
                ..Default::default()
            }),
            driver_name: "example.com/driver".to_string(),
            parameters: Default::default(),
        };

        let errors = validate_volume_attributes_class_v1(&volume_class);
        assert!(!errors.is_empty(), "expected validation errors");
    }
}
