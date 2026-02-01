//! StatefulSet validation for Kubernetes apps internal API

use crate::apps::internal::{
    PersistentVolumeClaimRetentionPolicyType, RollingUpdateStatefulSetStrategy, StatefulSet,
    StatefulSetPersistentVolumeClaimRetentionPolicy, StatefulSetSpec, StatefulSetStatus,
    StatefulSetUpdateStrategyType,
};
use crate::common::FromInternal;
use crate::common::validation::{
    BadValue, ErrorList, Path, invalid, name_is_dns_subdomain, required, validate_object_meta,
    validate_object_meta_update,
};
use crate::core::v1::PersistentVolumeClaimSpec;
use crate::core::v1::validation::helpers::{validate_dns1123_label, validate_nonnegative_field};
use crate::core::v1::validation::storage::validate_persistent_volume_claim_spec;

use super::{
    get_int_or_percent_value, is_not_more_than_100_percent, validate_label_selector,
    validate_positive_int_or_percent, validate_template_common,
};

// =============================================================================
// StatefulSet validation
// =============================================================================

#[derive(Clone, Copy, Debug, Default)]
pub struct StatefulSetValidationOptions {
    pub allow_invalid_service_name: bool,
    pub skip_validate_pod_template_spec: bool,
    pub skip_validate_volume_claim_templates: bool,
}

pub fn validate_stateful_set(stateful_set: &StatefulSet) -> ErrorList {
    let mut all_errs = ErrorList::new();

    all_errs.extend(validate_object_meta(
        &stateful_set.metadata,
        true,
        name_is_dns_subdomain,
        &Path::new("metadata"),
    ));

    if let Some(ref spec) = stateful_set.spec {
        all_errs.extend(validate_stateful_set_spec(
            spec,
            &Path::new("spec"),
            StatefulSetValidationOptions {
                allow_invalid_service_name: false,
                ..Default::default()
            },
        ));
    } else {
        all_errs.push(required(&Path::new("spec"), "spec is required"));
    }

    all_errs
}

pub fn validate_stateful_set_update(
    stateful_set: &StatefulSet,
    old_stateful_set: &StatefulSet,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    all_errs.extend(validate_object_meta_update(
        &stateful_set.metadata,
        &old_stateful_set.metadata,
        &Path::new("metadata"),
    ));

    if let Some(ref spec) = stateful_set.spec {
        all_errs.extend(validate_stateful_set_spec(
            spec,
            &Path::new("spec"),
            StatefulSetValidationOptions {
                allow_invalid_service_name: true,
                skip_validate_volume_claim_templates: true,
                ..Default::default()
            },
        ));
    }

    all_errs
}

pub fn validate_stateful_set_status_update(
    stateful_set: &StatefulSet,
    old_stateful_set: &StatefulSet,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if let Some(ref status) = stateful_set.status {
        all_errs.extend(validate_stateful_set_status(status, &Path::new("status")));
    } else {
        all_errs.push(required(&Path::new("status"), "status is required"));
    }

    all_errs.extend(validate_object_meta_update(
        &stateful_set.metadata,
        &old_stateful_set.metadata,
        &Path::new("metadata"),
    ));

    all_errs
}

fn validate_stateful_set_spec(
    spec: &StatefulSetSpec,
    path: &Path,
    opts: StatefulSetValidationOptions,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    match spec.update_strategy.r#type {
        StatefulSetUpdateStrategyType::OnDelete => {
            if spec.update_strategy.rolling_update.is_some() {
                all_errs.push(invalid(
                    &path.child("updateStrategy").child("rollingUpdate"),
                    BadValue::String("rollingUpdate".to_string()),
                    "only allowed for updateStrategy 'RollingUpdate'",
                ));
            }
        }
        StatefulSetUpdateStrategyType::RollingUpdate => {
            if let Some(ref rolling_update) = spec.update_strategy.rolling_update {
                all_errs.extend(validate_rolling_update_stateful_set(
                    rolling_update,
                    &path.child("updateStrategy").child("rollingUpdate"),
                ));
            }
        }
    }

    if let Some(policy) = &spec.persistent_volume_claim_retention_policy {
        all_errs.extend(validate_persistent_volume_claim_retention_policy(
            policy,
            &path.child("persistentVolumeClaimRetentionPolicy"),
        ));
    }

    if !opts.skip_validate_volume_claim_templates {
        for (i, pvc) in spec.volume_claim_templates.iter().enumerate() {
            if let Some(ref pvc_spec) = pvc.spec {
                let v1_spec = PersistentVolumeClaimSpec::from_internal(pvc_spec.clone());
                all_errs.extend(validate_persistent_volume_claim_spec(
                    &v1_spec,
                    &path.child("volumeClaimTemplates").index(i).child("spec"),
                ));
            } else {
                all_errs.push(required(
                    &path.child("volumeClaimTemplates").index(i).child("spec"),
                    "spec is required",
                ));
            }
        }
    }

    all_errs.extend(validate_nonnegative_field(
        spec.replicas as i64,
        &path.child("replicas"),
    ));
    all_errs.extend(validate_nonnegative_field(
        spec.min_ready_seconds as i64,
        &path.child("minReadySeconds"),
    ));

    if let Some(ordinals) = &spec.ordinals {
        all_errs.extend(validate_nonnegative_field(
            ordinals.start as i64,
            &path.child("ordinals").child("start"),
        ));
    }

    if !opts.allow_invalid_service_name && !spec.service_name.is_empty() {
        all_errs.extend(validate_dns1123_label(
            &spec.service_name,
            &path.child("serviceName"),
        ));
    }

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
                    "empty selector is invalid for statefulset",
                ));
            }
        }
    }

    if !opts.skip_validate_pod_template_spec {
        all_errs.extend(validate_template_common(
            &spec.template,
            spec.selector.as_ref(),
            &path.child("template"),
        ));
    }

    all_errs
}

fn validate_stateful_set_status(status: &StatefulSetStatus, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    all_errs.extend(validate_nonnegative_field(
        status.replicas as i64,
        &path.child("replicas"),
    ));
    all_errs.extend(validate_nonnegative_field(
        status.ready_replicas as i64,
        &path.child("readyReplicas"),
    ));
    all_errs.extend(validate_nonnegative_field(
        status.current_replicas as i64,
        &path.child("currentReplicas"),
    ));
    all_errs.extend(validate_nonnegative_field(
        status.updated_replicas as i64,
        &path.child("updatedReplicas"),
    ));
    all_errs.extend(validate_nonnegative_field(
        status.available_replicas as i64,
        &path.child("availableReplicas"),
    ));

    if let Some(observed_generation) = status.observed_generation {
        all_errs.extend(validate_nonnegative_field(
            observed_generation,
            &path.child("observedGeneration"),
        ));
    }
    if let Some(count) = status.collision_count {
        all_errs.extend(validate_nonnegative_field(
            count as i64,
            &path.child("collisionCount"),
        ));
    }

    let msg = "cannot be greater than status.replicas";
    if status.ready_replicas > status.replicas {
        all_errs.push(invalid(
            &path.child("readyReplicas"),
            BadValue::Int(status.ready_replicas as i64),
            msg,
        ));
    }
    if status.current_replicas > status.replicas {
        all_errs.push(invalid(
            &path.child("currentReplicas"),
            BadValue::Int(status.current_replicas as i64),
            msg,
        ));
    }
    if status.updated_replicas > status.replicas {
        all_errs.push(invalid(
            &path.child("updatedReplicas"),
            BadValue::Int(status.updated_replicas as i64),
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
            "cannot be greater than status.readyReplicas",
        ));
    }

    all_errs
}

fn validate_rolling_update_stateful_set(
    rolling_update: &RollingUpdateStatefulSetStrategy,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    all_errs.extend(validate_nonnegative_field(
        rolling_update.partition as i64,
        &path.child("partition"),
    ));

    if let Some(ref max_unavailable) = rolling_update.max_unavailable {
        all_errs.extend(validate_positive_int_or_percent(
            max_unavailable,
            &path.child("maxUnavailable"),
        ));
        if get_int_or_percent_value(max_unavailable) == 0 {
            all_errs.push(invalid(
                &path.child("maxUnavailable"),
                BadValue::String(format!("{:?}", max_unavailable)),
                "cannot be 0",
            ));
        }
        all_errs.extend(is_not_more_than_100_percent(
            max_unavailable,
            &path.child("maxUnavailable"),
        ));
    }

    all_errs
}

fn validate_persistent_volume_claim_retention_policy(
    policy: &StatefulSetPersistentVolumeClaimRetentionPolicy,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    all_errs.extend(validate_persistent_volume_claim_retention_policy_type(
        policy.when_deleted.clone(),
        &path.child("whenDeleted"),
    ));
    all_errs.extend(validate_persistent_volume_claim_retention_policy_type(
        policy.when_scaled.clone(),
        &path.child("whenScaled"),
    ));

    all_errs
}

fn validate_persistent_volume_claim_retention_policy_type(
    policy_type: PersistentVolumeClaimRetentionPolicyType,
    _path: &Path,
) -> ErrorList {
    match policy_type {
        PersistentVolumeClaimRetentionPolicyType::Retain
        | PersistentVolumeClaimRetentionPolicyType::Delete => ErrorList::new(),
    }
}
