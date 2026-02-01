//! DaemonSet validation for Kubernetes apps internal API

use crate::apps::internal::{
    DaemonSet, DaemonSetSpec, DaemonSetStatus, DaemonSetUpdateStrategy,
    DaemonSetUpdateStrategyType, RollingUpdateDaemonSet,
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
// DaemonSet validation
// =============================================================================

pub fn validate_daemon_set(daemon_set: &DaemonSet) -> ErrorList {
    let mut all_errs = ErrorList::new();

    all_errs.extend(validate_object_meta(
        &daemon_set.metadata,
        true,
        name_is_dns_subdomain,
        &Path::new("metadata"),
    ));

    if let Some(ref spec) = daemon_set.spec {
        all_errs.extend(validate_daemon_set_spec(spec, &Path::new("spec")));
    } else {
        all_errs.push(required(&Path::new("spec"), "spec is required"));
    }

    all_errs
}

pub fn validate_daemon_set_update(daemon_set: &DaemonSet, old_daemon_set: &DaemonSet) -> ErrorList {
    let mut all_errs = ErrorList::new();

    all_errs.extend(validate_object_meta_update(
        &daemon_set.metadata,
        &old_daemon_set.metadata,
        &Path::new("metadata"),
    ));

    if let Some(ref spec) = daemon_set.spec {
        all_errs.extend(validate_daemon_set_spec(spec, &Path::new("spec")));
        if let (Some(new_selector), Some(old_selector)) = (
            daemon_set.spec.as_ref().and_then(|s| s.selector.as_ref()),
            old_daemon_set
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

pub fn validate_daemon_set_status_update(
    daemon_set: &DaemonSet,
    old_daemon_set: &DaemonSet,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if let Some(ref status) = daemon_set.status {
        all_errs.extend(validate_daemon_set_status(status, &Path::new("status")));
    } else {
        all_errs.push(required(&Path::new("status"), "status is required"));
    }

    all_errs.extend(validate_object_meta_update(
        &daemon_set.metadata,
        &old_daemon_set.metadata,
        &Path::new("metadata"),
    ));

    all_errs
}

fn validate_daemon_set_spec(spec: &DaemonSetSpec, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

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
                    "empty selector is invalid for daemonset",
                ));
            }
        }
    }

    all_errs.extend(validate_template_common(
        &spec.template,
        spec.selector.as_ref(),
        &path.child("template"),
    ));

    all_errs.extend(validate_nonnegative_field(
        spec.min_ready_seconds as i64,
        &path.child("minReadySeconds"),
    ));

    all_errs.extend(validate_daemon_set_update_strategy(
        &spec.update_strategy,
        &path.child("updateStrategy"),
    ));

    if let Some(revision_history_limit) = spec.revision_history_limit {
        all_errs.extend(validate_nonnegative_field(
            revision_history_limit as i64,
            &path.child("revisionHistoryLimit"),
        ));
    }

    all_errs
}

fn validate_daemon_set_status(status: &DaemonSetStatus, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    all_errs.extend(validate_nonnegative_field(
        status.current_number_scheduled as i64,
        &path.child("currentNumberScheduled"),
    ));
    all_errs.extend(validate_nonnegative_field(
        status.number_misscheduled as i64,
        &path.child("numberMisscheduled"),
    ));
    all_errs.extend(validate_nonnegative_field(
        status.desired_number_scheduled as i64,
        &path.child("desiredNumberScheduled"),
    ));
    all_errs.extend(validate_nonnegative_field(
        status.number_ready as i64,
        &path.child("numberReady"),
    ));
    all_errs.extend(validate_nonnegative_field(
        status.observed_generation,
        &path.child("observedGeneration"),
    ));
    all_errs.extend(validate_nonnegative_field(
        status.updated_number_scheduled as i64,
        &path.child("updatedNumberScheduled"),
    ));
    all_errs.extend(validate_nonnegative_field(
        status.number_available as i64,
        &path.child("numberAvailable"),
    ));
    all_errs.extend(validate_nonnegative_field(
        status.number_unavailable as i64,
        &path.child("numberUnavailable"),
    ));

    if let Some(count) = status.collision_count {
        all_errs.extend(validate_nonnegative_field(
            count as i64,
            &path.child("collisionCount"),
        ));
    }

    all_errs
}

fn validate_daemon_set_update_strategy(
    strategy: &DaemonSetUpdateStrategy,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    match strategy.r#type {
        DaemonSetUpdateStrategyType::OnDelete => {}
        DaemonSetUpdateStrategyType::RollingUpdate => {
            if let Some(ref rolling_update) = strategy.rolling_update {
                all_errs.extend(validate_rolling_update_daemon_set(
                    rolling_update,
                    &path.child("rollingUpdate"),
                ));
            } else {
                all_errs.push(required(&path.child("rollingUpdate"), ""));
            }
        }
    }
    all_errs
}

fn validate_rolling_update_daemon_set(
    rolling_update: &RollingUpdateDaemonSet,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    all_errs.extend(validate_positive_int_or_percent(
        &rolling_update.max_unavailable,
        &path.child("maxUnavailable"),
    ));
    all_errs.extend(is_not_more_than_100_percent(
        &rolling_update.max_unavailable,
        &path.child("maxUnavailable"),
    ));

    all_errs.extend(validate_positive_int_or_percent(
        &rolling_update.max_surge,
        &path.child("maxSurge"),
    ));
    all_errs.extend(is_not_more_than_100_percent(
        &rolling_update.max_surge,
        &path.child("maxSurge"),
    ));

    let has_unavailable = get_int_or_percent_value(&rolling_update.max_unavailable) != 0;
    let has_surge = get_int_or_percent_value(&rolling_update.max_surge) != 0;
    match (has_unavailable, has_surge) {
        (true, true) => {
            all_errs.push(invalid(
                &path.child("maxSurge"),
                BadValue::String("maxSurge".to_string()),
                "may not be set when maxUnavailable is non-zero",
            ));
        }
        (false, false) => {
            all_errs.push(required(
                &path.child("maxUnavailable"),
                "cannot be 0 when maxSurge is 0",
            ));
        }
        _ => {}
    }

    all_errs
}
