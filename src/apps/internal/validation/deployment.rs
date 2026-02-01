//! Deployment validation for Kubernetes apps internal API

use crate::apps::internal::{
    Deployment, DeploymentSpec, DeploymentStatus, DeploymentStrategyType, RollbackConfig,
    RollingUpdateDeployment,
};
use crate::common::validation::{
    BadValue, ErrorList, Path, forbidden, invalid, name_is_dns_subdomain, required,
    validate_object_meta, validate_object_meta_update,
};
use crate::core::v1::validation::helpers::validate_nonnegative_field;

use super::{
    get_int_or_percent_value, is_not_more_than_100_percent, validate_label_selector,
    validate_positive_int_or_percent, validate_template_common,
};

// =============================================================================
// Deployment validation
// =============================================================================

pub fn validate_deployment(deployment: &Deployment) -> ErrorList {
    let mut all_errs = ErrorList::new();

    all_errs.extend(validate_object_meta(
        &deployment.metadata,
        true,
        name_is_dns_subdomain,
        &Path::new("metadata"),
    ));

    if let Some(ref spec) = deployment.spec {
        all_errs.extend(validate_deployment_spec(spec, &Path::new("spec")));
    } else {
        all_errs.push(required(&Path::new("spec"), "spec is required"));
    }

    all_errs
}

pub fn validate_deployment_update(
    deployment: &Deployment,
    old_deployment: &Deployment,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    all_errs.extend(validate_object_meta_update(
        &deployment.metadata,
        &old_deployment.metadata,
        &Path::new("metadata"),
    ));

    if let Some(ref spec) = deployment.spec {
        all_errs.extend(validate_deployment_spec(spec, &Path::new("spec")));
        if let (Some(new_selector), Some(old_selector)) = (
            deployment.spec.as_ref().and_then(|s| s.selector.as_ref()),
            old_deployment
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

pub fn validate_deployment_status_update(
    deployment: &Deployment,
    old_deployment: &Deployment,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if let Some(ref status) = deployment.status {
        all_errs.extend(validate_deployment_status(status, &Path::new("status")));
    } else {
        all_errs.push(required(&Path::new("status"), "status is required"));
    }

    all_errs.extend(validate_object_meta_update(
        &deployment.metadata,
        &old_deployment.metadata,
        &Path::new("metadata"),
    ));

    all_errs
}

fn validate_deployment_spec(spec: &DeploymentSpec, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    all_errs.extend(validate_nonnegative_field(
        spec.replicas as i64,
        &path.child("replicas"),
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
                    "empty selector is invalid for deployment",
                ));
            }
        }
    }

    all_errs.extend(validate_template_common(
        &spec.template,
        spec.selector.as_ref(),
        &path.child("template"),
    ));

    all_errs.extend(validate_deployment_strategy(
        &spec.strategy,
        &path.child("strategy"),
    ));

    all_errs.extend(validate_nonnegative_field(
        spec.min_ready_seconds as i64,
        &path.child("minReadySeconds"),
    ));

    if let Some(revision_history_limit) = spec.revision_history_limit {
        all_errs.extend(validate_nonnegative_field(
            revision_history_limit as i64,
            &path.child("revisionHistoryLimit"),
        ));
    }

    if let Some(ref rollback_to) = spec.rollback_to {
        all_errs.extend(validate_rollback(rollback_to, &path.child("rollback")));
    }

    if let Some(progress_deadline_seconds) = spec.progress_deadline_seconds {
        all_errs.extend(validate_nonnegative_field(
            progress_deadline_seconds as i64,
            &path.child("progressDeadlineSeconds"),
        ));
        if progress_deadline_seconds <= spec.min_ready_seconds {
            all_errs.push(invalid(
                &path.child("progressDeadlineSeconds"),
                BadValue::Int(progress_deadline_seconds as i64),
                "must be greater than minReadySeconds",
            ));
        }
    }

    all_errs
}

fn validate_deployment_status(status: &DeploymentStatus, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    all_errs.extend(validate_nonnegative_field(
        status.observed_generation,
        &path.child("observedGeneration"),
    ));
    all_errs.extend(validate_nonnegative_field(
        status.replicas as i64,
        &path.child("replicas"),
    ));
    all_errs.extend(validate_nonnegative_field(
        status.updated_replicas as i64,
        &path.child("updatedReplicas"),
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
        status.unavailable_replicas as i64,
        &path.child("unavailableReplicas"),
    ));

    if let Some(rep) = status.terminating_replicas {
        all_errs.extend(validate_nonnegative_field(
            rep as i64,
            &path.child("terminatingReplicas"),
        ));
    }

    if let Some(count) = status.collision_count {
        all_errs.extend(validate_nonnegative_field(
            count as i64,
            &path.child("collisionCount"),
        ));
    }

    let msg = "cannot be greater than status.replicas";
    if status.updated_replicas > status.replicas {
        all_errs.push(invalid(
            &path.child("updatedReplicas"),
            BadValue::Int(status.updated_replicas as i64),
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

fn validate_deployment_strategy(
    strategy: &crate::apps::internal::DeploymentStrategy,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    match strategy.r#type {
        DeploymentStrategyType::Recreate => {
            if strategy.rolling_update.is_some() {
                all_errs.push(forbidden(
                    &path.child("rollingUpdate"),
                    "may not be specified when strategy `type` is 'Recreate'",
                ));
            }
        }
        DeploymentStrategyType::RollingUpdate => {
            if let Some(ref rolling_update) = strategy.rolling_update {
                all_errs.extend(validate_rolling_update_deployment(
                    rolling_update,
                    &path.child("rollingUpdate"),
                ));
            } else {
                all_errs.push(required(
                    &path.child("rollingUpdate"),
                    "this should be defaulted and never be nil",
                ));
            }
        }
    }
    all_errs
}

fn validate_rolling_update_deployment(
    rolling_update: &RollingUpdateDeployment,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    all_errs.extend(validate_positive_int_or_percent(
        &rolling_update.max_unavailable,
        &path.child("maxUnavailable"),
    ));
    all_errs.extend(validate_positive_int_or_percent(
        &rolling_update.max_surge,
        &path.child("maxSurge"),
    ));

    if get_int_or_percent_value(&rolling_update.max_unavailable) == 0
        && get_int_or_percent_value(&rolling_update.max_surge) == 0
    {
        all_errs.push(invalid(
            &path.child("maxUnavailable"),
            BadValue::String(format!("{:?}", rolling_update.max_unavailable)),
            "may not be 0 when `maxSurge` is 0",
        ));
    }

    all_errs.extend(is_not_more_than_100_percent(
        &rolling_update.max_unavailable,
        &path.child("maxUnavailable"),
    ));

    all_errs
}

fn validate_rollback(rollback: &RollbackConfig, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    all_errs.extend(validate_nonnegative_field(
        rollback.revision,
        &path.child("version"),
    ));
    all_errs
}
