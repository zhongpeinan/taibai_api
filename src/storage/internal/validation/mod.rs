//! Validation for Kubernetes Storage internal API types
//!
//! Ported from k8s.io/kubernetes/pkg/apis/storage/validation/validation.go

use std::collections::{BTreeMap, BTreeSet, HashSet};

use crate::common::meta::label_selector_operator;
use crate::common::validation::{
    BadValue, ErrorList, Path, duplicate, invalid, not_supported, required, too_long,
    validate_labels, validate_qualified_name,
};
use crate::common::{LabelSelector, PersistentVolumeReclaimPolicy, TopologySelectorTerm};

mod csi_driver;
mod csi_node;
mod csi_storage_capacity;
mod storage_class;
mod volume_attachment;
mod volume_attributes_class;

pub use csi_driver::{validate_csi_driver_v1, validate_csi_driver_v1beta1};
pub use csi_node::{CSINodeValidationOptions, validate_csi_node_v1, validate_csi_node_v1beta1};
pub use csi_storage_capacity::{
    validate_csi_storage_capacity_v1, validate_csi_storage_capacity_v1alpha1,
    validate_csi_storage_capacity_v1beta1,
};
pub use storage_class::{validate_storage_class_v1, validate_storage_class_v1beta1};
pub use volume_attachment::{
    validate_volume_attachment_v1, validate_volume_attachment_v1alpha1,
    validate_volume_attachment_v1beta1,
};
pub use volume_attributes_class::{
    validate_volume_attributes_class_v1, validate_volume_attributes_class_v1alpha1,
    validate_volume_attributes_class_v1beta1,
};

const MAX_PROVISIONER_PARAMETER_SIZE: usize = 256 * 1024;
const MAX_PROVISIONER_PARAMETER_LEN: usize = 512;

const MAX_ATTACHED_VOLUME_METADATA_SIZE: usize = 256 * 1024;
const MAX_VOLUME_ERROR_MESSAGE_SIZE: usize = 1024;

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
    source: &crate::storage::v1::VolumeAttachmentSource,
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

fn validate_volume_error(
    error: &Option<crate::storage::v1::VolumeError>,
    path: &Path,
) -> ErrorList {
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

fn validate_csi_driver_name(name: &str, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    for msg in crate::common::validation::is_dns1123_subdomain(name) {
        all_errs.push(invalid(path, BadValue::String(name.to_string()), &msg));
    }
    all_errs
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{ObjectMeta, TypeMeta};
    use crate::storage::v1 as storage_v1;

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
