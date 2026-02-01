//! ReplicaSet validation for Kubernetes apps internal API

use crate::apps::internal::{ReplicaSet, ReplicaSetSpec, ReplicaSetStatus};
use crate::common::validation::{
    BadValue, ErrorList, Path, forbidden, invalid, name_is_dns_subdomain, required,
    validate_object_meta, validate_object_meta_update,
};
use crate::core::v1::validation::helpers::validate_nonnegative_field;

use super::{validate_label_selector, validate_template_common};

// =============================================================================
// ReplicaSet validation
// =============================================================================

pub fn validate_replica_set(replica_set: &ReplicaSet) -> ErrorList {
    let mut all_errs = ErrorList::new();

    all_errs.extend(validate_object_meta(
        &replica_set.metadata,
        true,
        name_is_dns_subdomain,
        &Path::new("metadata"),
    ));

    if let Some(ref spec) = replica_set.spec {
        all_errs.extend(validate_replica_set_spec(spec, &Path::new("spec")));
    } else {
        all_errs.push(required(&Path::new("spec"), "spec is required"));
    }

    all_errs
}

pub fn validate_replica_set_update(
    replica_set: &ReplicaSet,
    old_replica_set: &ReplicaSet,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    all_errs.extend(validate_object_meta_update(
        &replica_set.metadata,
        &old_replica_set.metadata,
        &Path::new("metadata"),
    ));

    if let Some(ref spec) = replica_set.spec {
        all_errs.extend(validate_replica_set_spec(spec, &Path::new("spec")));
        if let (Some(new_selector), Some(old_selector)) = (
            replica_set.spec.as_ref().and_then(|s| s.selector.as_ref()),
            old_replica_set
                .spec
                .as_ref()
                .and_then(|s| s.selector.as_ref()),
        ) {
            if new_selector != old_selector {
                all_errs.push(forbidden(
                    &Path::new("spec").child("selector"),
                    "field is immutable",
                ));
            }
        }
    }

    all_errs
}

pub fn validate_replica_set_status_update(
    replica_set: &ReplicaSet,
    old_replica_set: &ReplicaSet,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if let Some(ref status) = replica_set.status {
        all_errs.extend(validate_replica_set_status(status, &Path::new("status")));
    } else {
        all_errs.push(required(&Path::new("status"), "status is required"));
    }

    all_errs.extend(validate_object_meta_update(
        &replica_set.metadata,
        &old_replica_set.metadata,
        &Path::new("metadata"),
    ));

    all_errs
}

fn validate_replica_set_spec(spec: &ReplicaSetSpec, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    all_errs.extend(validate_nonnegative_field(
        spec.replicas as i64,
        &path.child("replicas"),
    ));
    all_errs.extend(validate_nonnegative_field(
        spec.min_ready_seconds as i64,
        &path.child("minReadySeconds"),
    ));

    match spec.selector.as_ref() {
        None => {
            all_errs.push(required(&path.child("selector"), ""));
        }
        Some(selector) => {
            all_errs.extend(validate_label_selector(selector, &path.child("selector")));
            if selector.match_labels.is_empty() && selector.match_expressions.is_empty() {
                all_errs.push(invalid(
                    &path.child("selector"),
                    BadValue::String(format!("{selector:?}")),
                    "empty selector is invalid for replica set",
                ));
            }
        }
    }

    if let Some(ref template) = spec.template {
        all_errs.extend(validate_template_common(
            template,
            spec.selector.as_ref(),
            &path.child("template"),
        ));
    } else {
        all_errs.push(required(&path.child("template"), "template is required"));
    }

    all_errs
}

fn validate_replica_set_status(status: &ReplicaSetStatus, path: &Path) -> ErrorList {
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
    all_errs.extend(validate_nonnegative_field(
        status.observed_generation,
        &path.child("observedGeneration"),
    ));

    if let Some(rep) = status.terminating_replicas {
        all_errs.extend(validate_nonnegative_field(
            rep as i64,
            &path.child("terminatingReplicas"),
        ));
    }

    let msg = "cannot be greater than status.replicas";
    if status.fully_labeled_replicas > status.replicas {
        all_errs.push(invalid(
            &path.child("fullyLabeledReplicas"),
            BadValue::Int(status.fully_labeled_replicas as i64),
            msg,
        ));
    }
    if status.ready_replicas > status.replicas {
        all_errs.push(invalid(
            &path.child("readyReplicas"),
            BadValue::Int(status.ready_replicas as i64),
            msg,
        ));
    }
    if status.available_replicas > status.replicas {
        all_errs.push(invalid(
            &path.child("availableReplicas"),
            BadValue::Int(status.available_replicas as i64),
            msg,
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
