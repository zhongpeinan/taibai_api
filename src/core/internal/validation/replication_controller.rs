//! ReplicationController validation for Kubernetes core internal API.
//!
//! Ported from k8s.io/kubernetes/pkg/apis/core/validation/validation.go

use crate::common::validation::{
    BadValue, ErrorList, Path, forbidden, invalid, name_is_dns_subdomain, not_supported, required,
    validate_labels, validate_object_meta,
};
use crate::core::internal::replication_controller::{
    PodTemplateSpec, ReplicationController, ReplicationControllerSpec, ReplicationControllerStatus,
};
use crate::core::internal::validation::pod_spec::validate_pod_spec;
use crate::core::internal::{RestartPolicy, restart_policy};
use std::collections::BTreeMap;

const IS_NEGATIVE_ERROR_MSG: &str = "must be greater than or equal to 0";

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
    all_errs.extend(validate_object_meta(
        &controller.metadata,
        true,
        name_is_dns_subdomain,
        &path.child("metadata"),
    ));

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

    all_errs.extend(crate::common::validation::validate_object_meta_update(
        &new.metadata,
        &old.metadata,
        &Path::nil().child("metadata"),
    ));

    all_errs.extend(validate_replication_controller_with_path(new, &Path::nil()));
    all_errs
}

/// Validates ReplicationController status update.
pub fn validate_replication_controller_status_update(
    new: &ReplicationController,
    old: &ReplicationController,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    all_errs.extend(crate::common::validation::validate_object_meta_update(
        &new.metadata,
        &old.metadata,
        &Path::nil().child("metadata"),
    ));

    all_errs.extend(validate_replication_controller_status(
        &new.status,
        &Path::nil().child("status"),
    ));

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

    all_errs.extend(validate_nonnegative_field(
        spec.replicas as i64,
        &path.child("replicas"),
    ));

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
        let labels = &template.metadata.labels;
        let matches = selector
            .iter()
            .all(|(key, value)| labels.get(key) == Some(value));
        if !matches {
            let label_value = format!("{:?}", template.metadata.labels);
            all_errs.push(invalid(
                &path.child("metadata").child("labels"),
                BadValue::String(label_value),
                "`selector` does not match template `labels`",
            ));
        }
    }

    all_errs.extend(validate_pod_template_spec(template, path));

    if let Some(ref spec) = template.spec {
        if spec.restart_policy != RestartPolicy::Always {
            all_errs.push(not_supported(
                &path.child("spec").child("restartPolicy"),
                BadValue::String(format!("{:?}", spec.restart_policy)),
                &[restart_policy::ALWAYS],
            ));
        }

        if spec.active_deadline_seconds.is_some() {
            all_errs.push(forbidden(
                &path.child("spec").child("activeDeadlineSeconds"),
                "activeDeadlineSeconds in ReplicationController is not Supported",
            ));
        }
    }

    all_errs
}

fn validate_nonnegative_field(value: i64, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if value < 0 {
        all_errs.push(invalid(path, BadValue::Int(value), IS_NEGATIVE_ERROR_MSG));
    }
    all_errs
}

fn validate_pod_template_spec(spec: &PodTemplateSpec, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if !spec.metadata.labels.is_empty() {
        all_errs.extend(validate_labels(
            &spec.metadata.labels,
            &path.child("metadata").child("labels"),
        ));
    }

    if let Some(ref pod_spec) = spec.spec {
        all_errs.extend(validate_pod_spec(pod_spec, &path.child("spec")));
        if !pod_spec.ephemeral_containers.is_empty() {
            all_errs.push(forbidden(
                &path.child("spec").child("ephemeralContainers"),
                "ephemeral containers not allowed in pod template",
            ));
        }
    } else {
        all_errs.push(required(&path.child("spec"), "spec is required"));
    }

    all_errs
}
