//! ReplicationController validation for Kubernetes core/v1 API
//!
//! Ported from k8s.io/kubernetes/pkg/apis/core/validation/validation.go

use crate::common::validation::{
    BadValue, ErrorList, Path, invalid, name_is_dns_subdomain, not_supported, required,
};
use crate::core::v1::replication_controller::{
    ReplicationController, ReplicationControllerSpec, ReplicationControllerStatus,
};
use crate::core::v1::template::PodTemplateSpec;
use crate::core::v1::validation::helpers::validate_nonnegative_field;
use crate::core::v1::validation::template::validate_pod_template_spec;
use std::collections::BTreeMap;

/// Validates a ReplicationController resource.
pub fn validate_replication_controller(controller: &ReplicationController) -> ErrorList {
    validate_replication_controller_with_path(controller, &Path::nil())
}

fn validate_replication_controller_with_path(
    controller: &ReplicationController,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate metadata (ReplicationController is namespaced).
    if let Some(ref metadata) = controller.metadata {
        all_errs.extend(crate::common::validation::validate_object_meta(
            metadata,
            true,
            name_is_dns_subdomain,
            &path.child("metadata"),
        ));
    } else {
        all_errs.push(required(&path.child("metadata"), "metadata is required"));
    }

    // Validate spec
    if let Some(ref spec) = controller.spec {
        all_errs.extend(validate_replication_controller_spec(
            spec,
            &path.child("spec"),
        ));
    } else {
        all_errs.push(required(&path.child("spec"), "spec is required"));
    }

    all_errs
}

/// Validates ReplicationController update.
pub fn validate_replication_controller_update(
    new: &ReplicationController,
    old: &ReplicationController,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if let (Some(new_meta), Some(old_meta)) = (&new.metadata, &old.metadata) {
        all_errs.extend(crate::common::validation::validate_object_meta_update(
            new_meta,
            old_meta,
            &Path::nil().child("metadata"),
        ));
    }

    all_errs.extend(validate_replication_controller_with_path(new, &Path::nil()));
    all_errs
}

/// Validates ReplicationController status update.
pub fn validate_replication_controller_status_update(
    new: &ReplicationController,
    old: &ReplicationController,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if let (Some(new_meta), Some(old_meta)) = (&new.metadata, &old.metadata) {
        all_errs.extend(crate::common::validation::validate_object_meta_update(
            new_meta,
            old_meta,
            &Path::nil().child("metadata"),
        ));
    }

    if let Some(ref status) = new.status {
        all_errs.extend(validate_replication_controller_status(
            status,
            &Path::nil().child("status"),
        ));
    }

    all_errs
}

/// Validates ReplicationControllerStatus.
pub fn validate_replication_controller_status(
    status: &ReplicationControllerStatus,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    all_errs.extend(validate_nonnegative_field(
        status.replicas as i64,
        &path.child("replicas"),
    ));
    all_errs.extend(validate_nonnegative_field(
        status.fully_labeled_replicas as i64,
        &path.child("fullyLabeledReplicas"),
    ));
    all_errs.extend(validate_nonnegative_field(
        status.ready_replicas as i64,
        &path.child("readyReplicas"),
    ));
    all_errs.extend(validate_nonnegative_field(
        status.available_replicas as i64,
        &path.child("availableReplicas"),
    ));
    if let Some(observed) = status.observed_generation {
        all_errs.extend(validate_nonnegative_field(
            observed,
            &path.child("observedGeneration"),
        ));
    }

    if status.fully_labeled_replicas > status.replicas {
        all_errs.push(invalid(
            &path.child("fullyLabeledReplicas"),
            BadValue::Int(status.fully_labeled_replicas as i64),
            "cannot be greater than status.replicas",
        ));
    }
    if status.ready_replicas > status.replicas {
        all_errs.push(invalid(
            &path.child("readyReplicas"),
            BadValue::Int(status.ready_replicas as i64),
            "cannot be greater than status.replicas",
        ));
    }
    if status.available_replicas > status.replicas {
        all_errs.push(invalid(
            &path.child("availableReplicas"),
            BadValue::Int(status.available_replicas as i64),
            "cannot be greater than status.replicas",
        ));
    }
    if status.available_replicas > status.ready_replicas {
        all_errs.push(invalid(
            &path.child("availableReplicas"),
            BadValue::Int(status.available_replicas as i64),
            "cannot be greater than readyReplicas",
        ));
    }

    all_errs
}

/// Validates ReplicationControllerSpec.
pub fn validate_replication_controller_spec(
    spec: &ReplicationControllerSpec,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if let Some(value) = spec.min_ready_seconds {
        all_errs.extend(validate_nonnegative_field(
            value as i64,
            &path.child("minReadySeconds"),
        ));
    }

    all_errs.extend(validate_non_empty_selector(
        &spec.selector,
        &path.child("selector"),
    ));

    if let Some(replicas) = spec.replicas {
        all_errs.extend(validate_nonnegative_field(
            replicas as i64,
            &path.child("replicas"),
        ));
    } else {
        all_errs.push(required(&path.child("replicas"), ""));
    }

    all_errs.extend(validate_pod_template_spec_for_rc(
        spec.template.as_ref(),
        &spec.selector,
        &path.child("template"),
    ));

    all_errs
}

fn validate_non_empty_selector(selector: &BTreeMap<String, String>, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if selector.is_empty() {
        all_errs.push(required(path, ""));
    }
    all_errs
}

fn validate_pod_template_spec_for_rc(
    template: Option<&PodTemplateSpec>,
    selector: &BTreeMap<String, String>,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    let template = if let Some(template) = template {
        template
    } else {
        all_errs.push(required(path, ""));
        return all_errs;
    };

    if !selector.is_empty() {
        let labels = template.metadata.as_ref().map(|meta| &meta.labels);
        let matches = selector
            .iter()
            .all(|(key, value)| labels.and_then(|map| map.get(key)) == Some(value));
        if !matches {
            let label_value = template
                .metadata
                .as_ref()
                .map(|meta| format!("{:?}", meta.labels))
                .unwrap_or_else(|| "{}".to_string());
            all_errs.push(invalid(
                &path.child("metadata").child("labels"),
                BadValue::String(label_value),
                "`selector` does not match template `labels`",
            ));
        }
    }

    all_errs.extend(validate_pod_template_spec(template, path));

    if let Some(ref spec) = template.spec {
        let restart_policy = spec.restart_policy.as_deref().unwrap_or("Always");
        if restart_policy != "Always" {
            all_errs.push(not_supported(
                &path.child("spec").child("restartPolicy"),
                BadValue::String(restart_policy.to_string()),
                &["Always"],
            ));
        }

        if spec.active_deadline_seconds.is_some() {
            all_errs.push(crate::common::validation::forbidden(
                &path.child("spec").child("activeDeadlineSeconds"),
                "activeDeadlineSeconds in ReplicationController is not Supported",
            ));
        }
    }

    all_errs
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ObjectMeta;
    use crate::core::v1::pod::{Container, PodSpec};
    use std::collections::BTreeMap;

    fn base_rc() -> ReplicationController {
        ReplicationController {
            type_meta: crate::common::TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("demo".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(ReplicationControllerSpec {
                replicas: Some(1),
                min_ready_seconds: Some(0),
                selector: BTreeMap::from([("app".to_string(), "demo".to_string())]),
                template: Some(PodTemplateSpec {
                    metadata: Some(ObjectMeta {
                        labels: BTreeMap::from([("app".to_string(), "demo".to_string())]),
                        ..Default::default()
                    }),
                    spec: Some(PodSpec {
                        termination_grace_period_seconds: Some(30),
                        restart_policy: Some("Always".to_string()),
                        dns_policy: Some("ClusterFirst".to_string()),
                        containers: vec![Container {
                            name: "nginx".to_string(),
                            image: Some("nginx:latest".to_string()),
                            termination_message_policy: Some("File".to_string()),
                            ..Default::default()
                        }],
                        ..Default::default()
                    }),
                }),
            }),
            status: None,
        }
    }

    #[test]
    fn test_validate_replication_controller_requires_metadata_and_spec() {
        let rc = ReplicationController::default();
        let errs = validate_replication_controller(&rc);
        assert!(errs.errors.iter().any(|e| e.field.ends_with("metadata")));
        assert!(errs.errors.iter().any(|e| e.field.ends_with("spec")));
    }

    #[test]
    fn test_validate_replication_controller_requires_selector() {
        let mut rc = base_rc();
        rc.spec.as_mut().unwrap().selector.clear();
        let errs = validate_replication_controller(&rc);
        assert!(errs.errors.iter().any(|e| e.field.ends_with("selector")));
    }

    #[test]
    fn test_validate_replication_controller_selector_mismatch() {
        let mut rc = base_rc();
        rc.spec
            .as_mut()
            .unwrap()
            .selector
            .insert("other".to_string(), "value".to_string());
        let errs = validate_replication_controller(&rc);
        assert!(
            errs.errors
                .iter()
                .any(|e| e.detail.contains("selector` does not match"))
        );
    }

    #[test]
    fn test_validate_replication_controller_restart_policy_must_be_always() {
        let mut rc = base_rc();
        if let Some(ref mut spec) = rc.spec.as_mut().unwrap().template.as_mut().unwrap().spec {
            spec.restart_policy = Some("OnFailure".to_string());
        }
        let errs = validate_replication_controller(&rc);
        assert!(
            errs.errors
                .iter()
                .any(|e| e.field.contains("restartPolicy"))
        );
    }

    #[test]
    fn test_validate_replication_controller_active_deadline_seconds_forbidden() {
        let mut rc = base_rc();
        if let Some(ref mut spec) = rc.spec.as_mut().unwrap().template.as_mut().unwrap().spec {
            spec.active_deadline_seconds = Some(10);
        }
        let errs = validate_replication_controller(&rc);
        assert!(
            errs.errors
                .iter()
                .any(|e| e.field.contains("activeDeadlineSeconds"))
        );
    }

    #[test]
    fn test_validate_replication_controller_status_invalid_counts() {
        let status = ReplicationControllerStatus {
            replicas: 1,
            ready_replicas: 2,
            available_replicas: 3,
            fully_labeled_replicas: 4,
            observed_generation: Some(1),
            conditions: Vec::new(),
        };

        let errs = validate_replication_controller_status(&status, &Path::nil());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.field.contains("readyReplicas"))
        );
        assert!(
            errs.errors
                .iter()
                .any(|e| e.field.contains("availableReplicas"))
        );
        assert!(
            errs.errors
                .iter()
                .any(|e| e.field.contains("fullyLabeledReplicas"))
        );
    }
}
