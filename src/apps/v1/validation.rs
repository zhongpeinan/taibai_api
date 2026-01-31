//! Validation for Kubernetes Apps v1 API types
//!
//! Ported from k8s.io/kubernetes/pkg/apis/apps/validation/validation.go

use std::collections::BTreeMap;

use crate::common::meta::label_selector_operator;
use crate::common::validation::{
    BadValue, ErrorList, Path, forbidden, invalid, name_is_dns_subdomain, not_supported, required,
    validate_labels, validate_object_meta, validate_object_meta_update, validate_qualified_name,
};
use crate::common::{IntOrString, LabelSelector};
use crate::core::v1::pod::{PodSpec, restart_policy};
use crate::core::v1::validation::helpers::{validate_dns1123_label, validate_nonnegative_field};
use crate::core::v1::validation::storage::validate_persistent_volume_claim_spec;
use crate::core::v1::validation::template::validate_pod_template_spec;

use super::{
    ControllerRevision, DaemonSet, DaemonSetSpec, DaemonSetUpdateStrategyType, Deployment,
    DeploymentSpec, DeploymentStrategyType, ReplicaSet, ReplicaSetSpec, RollingUpdateDaemonSet,
    RollingUpdateDeployment, RollingUpdateStatefulSetStrategy, StatefulSet, StatefulSetSpec,
    StatefulSetUpdateStrategyType, persistent_volume_claim_retention_policy_type,
};

// =============================================================================
// Selector helpers
// =============================================================================

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

fn selector_matches_labels(selector: &LabelSelector, labels: &BTreeMap<String, String>) -> bool {
    for (key, value) in &selector.match_labels {
        if labels.get(key) != Some(value) {
            return false;
        }
    }

    for requirement in &selector.match_expressions {
        let operator = requirement.operator.as_str();
        match operator {
            label_selector_operator::IN => {
                let Some(label_value) = labels.get(&requirement.key) else {
                    return false;
                };
                if !requirement.values.iter().any(|v| v == label_value) {
                    return false;
                }
            }
            label_selector_operator::NOT_IN => {
                let Some(label_value) = labels.get(&requirement.key) else {
                    return false;
                };
                if requirement.values.iter().any(|v| v == label_value) {
                    return false;
                }
            }
            label_selector_operator::EXISTS => {
                if !labels.contains_key(&requirement.key) {
                    return false;
                }
            }
            label_selector_operator::DOES_NOT_EXIST => {
                if labels.contains_key(&requirement.key) {
                    return false;
                }
            }
            _ => return false,
        }
    }

    true
}

// =============================================================================
// IntOrString helpers
// =============================================================================

fn parse_percent(value: &str) -> Option<i32> {
    let trimmed = value.trim();
    if !trimmed.ends_with('%') {
        return None;
    }
    let number = trimmed.trim_end_matches('%');
    if number.is_empty() || !number.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    number.parse::<i32>().ok()
}

fn validate_positive_int_or_percent(value: &IntOrString, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    match value {
        IntOrString::String(text) => {
            if parse_percent(text).is_none() {
                all_errs.push(invalid(
                    path,
                    BadValue::String(text.clone()),
                    "must be an integer or percentage (e.g '5%')",
                ));
            }
        }
        IntOrString::Int(int_value) => {
            all_errs.extend(validate_nonnegative_field(*int_value as i64, path));
        }
    }
    all_errs
}

fn get_int_or_percent_value(value: &IntOrString) -> i32 {
    match value {
        IntOrString::Int(v) => *v,
        IntOrString::String(text) => parse_percent(text).unwrap_or(0),
    }
}

fn is_not_more_than_100_percent(value: &IntOrString, path: &Path) -> ErrorList {
    let percent_value = match value {
        IntOrString::String(text) => parse_percent(text),
        IntOrString::Int(_) => None,
    };
    let Some(percent_value) = percent_value else {
        return ErrorList::new();
    };

    if percent_value <= 100 {
        return ErrorList::new();
    }

    let mut all_errs = ErrorList::new();
    all_errs.push(invalid(
        path,
        BadValue::String(format!("{percent_value}%")),
        "must not be greater than 100%",
    ));
    all_errs
}

// =============================================================================
// Pod template helpers
// =============================================================================

fn validate_restart_policy_always(spec: &PodSpec, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if let Some(policy) = spec.restart_policy.as_deref() {
        if policy != restart_policy::ALWAYS {
            all_errs.push(not_supported(
                path,
                BadValue::String(policy.to_string()),
                &[restart_policy::ALWAYS],
            ));
        }
    }
    all_errs
}

fn validate_template_common(
    template: &crate::core::v1::PodTemplateSpec,
    selector: Option<&LabelSelector>,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if let Some(selector) = selector {
        if !selector.match_labels.is_empty() || !selector.match_expressions.is_empty() {
            let empty_labels = BTreeMap::new();
            let labels = template
                .metadata
                .as_ref()
                .map(|meta| &meta.labels)
                .unwrap_or(&empty_labels);
            if !selector_matches_labels(selector, labels) {
                all_errs.push(invalid(
                    &path.child("metadata").child("labels"),
                    BadValue::String(format!("{labels:?}")),
                    "`selector` does not match template `labels`",
                ));
            }
        }
    }

    all_errs.extend(validate_pod_template_spec(template, path));
    if let Some(ref pod_spec) = template.spec {
        all_errs.extend(validate_restart_policy_always(
            pod_spec,
            &path.child("spec").child("restartPolicy"),
        ));
        if pod_spec.active_deadline_seconds.is_some() {
            all_errs.push(forbidden(
                &path.child("spec").child("activeDeadlineSeconds"),
                "activeDeadlineSeconds in controller is not supported",
            ));
        }
    }

    all_errs
}

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
    let default_meta = crate::common::ObjectMeta::default();
    let meta = stateful_set.metadata.as_ref().unwrap_or(&default_meta);

    all_errs.extend(validate_object_meta(
        meta,
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

    if let (Some(new_meta), Some(old_meta)) = (
        stateful_set.metadata.as_ref(),
        old_stateful_set.metadata.as_ref(),
    ) {
        all_errs.extend(validate_object_meta_update(
            new_meta,
            old_meta,
            &Path::new("metadata"),
        ));
    }

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

    if let (Some(new_meta), Some(old_meta)) = (
        stateful_set.metadata.as_ref(),
        old_stateful_set.metadata.as_ref(),
    ) {
        all_errs.extend(validate_object_meta_update(
            new_meta,
            old_meta,
            &Path::new("metadata"),
        ));
    }

    all_errs
}

fn validate_stateful_set_spec(
    spec: &StatefulSetSpec,
    path: &Path,
    opts: StatefulSetValidationOptions,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    match spec.pod_management_policy.as_ref() {
        None => {
            all_errs.push(required(&path.child("podManagementPolicy"), ""));
        }
        Some(policy)
            if matches!(
                policy,
                super::PodManagementPolicyType::OrderedReady
                    | super::PodManagementPolicyType::Parallel
            ) => {}
        Some(policy) => {
            all_errs.push(invalid(
                &path.child("podManagementPolicy"),
                BadValue::String(format!("{policy:?}")),
                "must be 'OrderedReady' or 'Parallel'",
            ));
        }
    }

    match spec.update_strategy.as_ref() {
        None => {
            all_errs.push(required(&path.child("updateStrategy"), ""));
        }
        Some(strategy) => match strategy.r#type {
            Some(StatefulSetUpdateStrategyType::OnDelete) => {
                if strategy.rolling_update.is_some() {
                    all_errs.push(invalid(
                        &path.child("updateStrategy").child("rollingUpdate"),
                        BadValue::String("rollingUpdate".to_string()),
                        "only allowed for updateStrategy 'RollingUpdate'",
                    ));
                }
            }
            Some(StatefulSetUpdateStrategyType::RollingUpdate) => {
                if let Some(ref rolling_update) = strategy.rolling_update {
                    all_errs.extend(validate_rolling_update_stateful_set(
                        rolling_update,
                        &path.child("updateStrategy").child("rollingUpdate"),
                    ));
                }
            }
            None => {
                all_errs.push(required(&path.child("updateStrategy").child("type"), ""));
            }
        },
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
                all_errs.extend(validate_persistent_volume_claim_spec(
                    pvc_spec,
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

    if let Some(replicas) = spec.replicas {
        all_errs.extend(validate_nonnegative_field(
            replicas as i64,
            &path.child("replicas"),
        ));
    }
    if let Some(min_ready_seconds) = spec.min_ready_seconds {
        all_errs.extend(validate_nonnegative_field(
            min_ready_seconds as i64,
            &path.child("minReadySeconds"),
        ));
    }
    if let Some(ordinals) = &spec.ordinals {
        if let Some(start) = ordinals.start {
            all_errs.extend(validate_nonnegative_field(
                start as i64,
                &path.child("ordinals").child("start"),
            ));
        }
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
        if let Some(ref template) = spec.template {
            all_errs.extend(validate_template_common(
                template,
                spec.selector.as_ref(),
                &path.child("template"),
            ));
        } else {
            all_errs.push(required(&path.child("template"), "template is required"));
        }
    }

    all_errs
}

fn validate_stateful_set_status(status: &super::StatefulSetStatus, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    all_errs.extend(validate_nonnegative_field(
        status.replicas as i64,
        &path.child("replicas"),
    ));
    if let Some(ready) = status.ready_replicas {
        all_errs.extend(validate_nonnegative_field(
            ready as i64,
            &path.child("readyReplicas"),
        ));
        if ready > status.replicas {
            all_errs.push(invalid(
                &path.child("readyReplicas"),
                BadValue::Int(ready as i64),
                "cannot be greater than status.replicas",
            ));
        }
    }
    if let Some(current) = status.current_replicas {
        all_errs.extend(validate_nonnegative_field(
            current as i64,
            &path.child("currentReplicas"),
        ));
        if current > status.replicas {
            all_errs.push(invalid(
                &path.child("currentReplicas"),
                BadValue::Int(current as i64),
                "cannot be greater than status.replicas",
            ));
        }
    }
    if let Some(updated) = status.updated_replicas {
        all_errs.extend(validate_nonnegative_field(
            updated as i64,
            &path.child("updatedReplicas"),
        ));
        if updated > status.replicas {
            all_errs.push(invalid(
                &path.child("updatedReplicas"),
                BadValue::Int(updated as i64),
                "cannot be greater than status.replicas",
            ));
        }
    }
    all_errs.extend(validate_nonnegative_field(
        status.available_replicas as i64,
        &path.child("availableReplicas"),
    ));
    if status.available_replicas > status.replicas {
        all_errs.push(invalid(
            &path.child("availableReplicas"),
            BadValue::Int(status.available_replicas as i64),
            "cannot be greater than status.replicas",
        ));
    }
    if let Some(ready) = status.ready_replicas {
        if status.available_replicas > ready {
            all_errs.push(invalid(
                &path.child("availableReplicas"),
                BadValue::Int(status.available_replicas as i64),
                "cannot be greater than status.readyReplicas",
            ));
        }
    }
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
    all_errs
}

fn validate_rolling_update_stateful_set(
    rolling_update: &RollingUpdateStatefulSetStrategy,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if let Some(partition) = rolling_update.partition {
        all_errs.extend(validate_nonnegative_field(
            partition as i64,
            &path.child("partition"),
        ));
    }
    if let Some(ref max_unavailable) = rolling_update.max_unavailable {
        all_errs.extend(validate_positive_int_or_percent(
            max_unavailable,
            &path.child("maxUnavailable"),
        ));
        if get_int_or_percent_value(max_unavailable) == 0 {
            all_errs.push(invalid(
                &path.child("maxUnavailable"),
                BadValue::String(format!("{max_unavailable:?}")),
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
    policy: &super::StatefulSetPersistentVolumeClaimRetentionPolicy,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if let Some(policy_type) = &policy.when_deleted {
        if !matches!(
            policy_type,
            super::PersistentVolumeClaimRetentionPolicyType::Retain
                | super::PersistentVolumeClaimRetentionPolicyType::Delete
        ) {
            all_errs.push(not_supported(
                &path.child("whenDeleted"),
                BadValue::String(format!("{policy_type:?}")),
                &[
                    persistent_volume_claim_retention_policy_type::RETAIN,
                    persistent_volume_claim_retention_policy_type::DELETE,
                ],
            ));
        }
    }
    if let Some(policy_type) = &policy.when_scaled {
        if !matches!(
            policy_type,
            super::PersistentVolumeClaimRetentionPolicyType::Retain
                | super::PersistentVolumeClaimRetentionPolicyType::Delete
        ) {
            all_errs.push(not_supported(
                &path.child("whenScaled"),
                BadValue::String(format!("{policy_type:?}")),
                &[
                    persistent_volume_claim_retention_policy_type::RETAIN,
                    persistent_volume_claim_retention_policy_type::DELETE,
                ],
            ));
        }
    }
    all_errs
}

// =============================================================================
// Deployment validation
// =============================================================================

pub fn validate_deployment(deployment: &Deployment) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let default_meta = crate::common::ObjectMeta::default();
    let meta = deployment.metadata.as_ref().unwrap_or(&default_meta);

    all_errs.extend(validate_object_meta(
        meta,
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
    if let (Some(new_meta), Some(old_meta)) = (
        deployment.metadata.as_ref(),
        old_deployment.metadata.as_ref(),
    ) {
        all_errs.extend(validate_object_meta_update(
            new_meta,
            old_meta,
            &Path::new("metadata"),
        ));
    }

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

    if let (Some(new_meta), Some(old_meta)) = (
        deployment.metadata.as_ref(),
        old_deployment.metadata.as_ref(),
    ) {
        all_errs.extend(validate_object_meta_update(
            new_meta,
            old_meta,
            &Path::new("metadata"),
        ));
    }

    all_errs
}

fn validate_deployment_spec(spec: &DeploymentSpec, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if let Some(replicas) = spec.replicas {
        all_errs.extend(validate_nonnegative_field(
            replicas as i64,
            &path.child("replicas"),
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
                    "empty selector is invalid for deployment",
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

    if let Some(ref strategy) = spec.strategy {
        all_errs.extend(validate_deployment_strategy(
            strategy,
            &path.child("strategy"),
        ));
    } else {
        all_errs.push(required(&path.child("strategy"), ""));
    }

    if let Some(min_ready_seconds) = spec.min_ready_seconds {
        all_errs.extend(validate_nonnegative_field(
            min_ready_seconds as i64,
            &path.child("minReadySeconds"),
        ));
    }
    if let Some(revision_history_limit) = spec.revision_history_limit {
        all_errs.extend(validate_nonnegative_field(
            revision_history_limit as i64,
            &path.child("revisionHistoryLimit"),
        ));
    }
    if let Some(progress_deadline_seconds) = spec.progress_deadline_seconds {
        all_errs.extend(validate_nonnegative_field(
            progress_deadline_seconds as i64,
            &path.child("progressDeadlineSeconds"),
        ));
        if let Some(min_ready_seconds) = spec.min_ready_seconds {
            if progress_deadline_seconds <= min_ready_seconds {
                all_errs.push(invalid(
                    &path.child("progressDeadlineSeconds"),
                    BadValue::Int(progress_deadline_seconds as i64),
                    "must be greater than minReadySeconds",
                ));
            }
        }
    }

    all_errs
}

fn validate_deployment_status(status: &super::DeploymentStatus, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if let Some(rep) = status.replicas {
        all_errs.extend(validate_nonnegative_field(
            rep as i64,
            &path.child("replicas"),
        ));
    }
    if let Some(rep) = status.updated_replicas {
        all_errs.extend(validate_nonnegative_field(
            rep as i64,
            &path.child("updatedReplicas"),
        ));
    }
    if let Some(rep) = status.ready_replicas {
        all_errs.extend(validate_nonnegative_field(
            rep as i64,
            &path.child("readyReplicas"),
        ));
    }
    if let Some(rep) = status.available_replicas {
        all_errs.extend(validate_nonnegative_field(
            rep as i64,
            &path.child("availableReplicas"),
        ));
    }
    if let Some(rep) = status.unavailable_replicas {
        all_errs.extend(validate_nonnegative_field(
            rep as i64,
            &path.child("unavailableReplicas"),
        ));
    }
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
    all_errs
}

fn validate_deployment_strategy(strategy: &super::DeploymentStrategy, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    match strategy.r#type {
        Some(DeploymentStrategyType::Recreate) => {
            if strategy.rolling_update.is_some() {
                all_errs.push(forbidden(
                    &path.child("rollingUpdate"),
                    "may not be specified when strategy `type` is 'Recreate'",
                ));
            }
        }
        Some(DeploymentStrategyType::RollingUpdate) => {
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
        None => {
            all_errs.push(required(&path.child("type"), ""));
        }
    }
    all_errs
}

fn validate_rolling_update_deployment(
    rolling_update: &RollingUpdateDeployment,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if let Some(ref max_unavailable) = rolling_update.max_unavailable {
        all_errs.extend(validate_positive_int_or_percent(
            max_unavailable,
            &path.child("maxUnavailable"),
        ));
        all_errs.extend(is_not_more_than_100_percent(
            max_unavailable,
            &path.child("maxUnavailable"),
        ));
    }
    if let Some(ref max_surge) = rolling_update.max_surge {
        all_errs.extend(validate_positive_int_or_percent(
            max_surge,
            &path.child("maxSurge"),
        ));
        all_errs.extend(is_not_more_than_100_percent(
            max_surge,
            &path.child("maxSurge"),
        ));
    }

    if let (Some(ref max_unavailable), Some(ref max_surge)) = (
        rolling_update.max_unavailable.as_ref(),
        rolling_update.max_surge.as_ref(),
    ) {
        if get_int_or_percent_value(max_unavailable) == 0
            && get_int_or_percent_value(max_surge) == 0
        {
            all_errs.push(invalid(
                &path.child("maxUnavailable"),
                BadValue::String(format!("{max_unavailable:?}")),
                "may not be 0 when `maxSurge` is 0",
            ));
        }
    }

    all_errs
}

// =============================================================================
// ReplicaSet validation
// =============================================================================

pub fn validate_replica_set(replica_set: &ReplicaSet) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let default_meta = crate::common::ObjectMeta::default();
    let meta = replica_set.metadata.as_ref().unwrap_or(&default_meta);

    all_errs.extend(validate_object_meta(
        meta,
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
    if let (Some(new_meta), Some(old_meta)) = (
        replica_set.metadata.as_ref(),
        old_replica_set.metadata.as_ref(),
    ) {
        all_errs.extend(validate_object_meta_update(
            new_meta,
            old_meta,
            &Path::new("metadata"),
        ));
    }

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

    if let (Some(new_meta), Some(old_meta)) = (
        replica_set.metadata.as_ref(),
        old_replica_set.metadata.as_ref(),
    ) {
        all_errs.extend(validate_object_meta_update(
            new_meta,
            old_meta,
            &Path::new("metadata"),
        ));
    }

    all_errs
}

fn validate_replica_set_spec(spec: &ReplicaSetSpec, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if let Some(replicas) = spec.replicas {
        all_errs.extend(validate_nonnegative_field(
            replicas as i64,
            &path.child("replicas"),
        ));
    }
    if let Some(min_ready_seconds) = spec.min_ready_seconds {
        all_errs.extend(validate_nonnegative_field(
            min_ready_seconds as i64,
            &path.child("minReadySeconds"),
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

fn validate_replica_set_status(status: &super::ReplicaSetStatus, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    all_errs.extend(validate_nonnegative_field(
        status.replicas as i64,
        &path.child("replicas"),
    ));
    if let Some(rep) = status.fully_labeled_replicas {
        all_errs.extend(validate_nonnegative_field(
            rep as i64,
            &path.child("fullyLabeledReplicas"),
        ));
        if rep > status.replicas {
            all_errs.push(invalid(
                &path.child("fullyLabeledReplicas"),
                BadValue::Int(rep as i64),
                "cannot be greater than status.replicas",
            ));
        }
    }
    if let Some(rep) = status.ready_replicas {
        all_errs.extend(validate_nonnegative_field(
            rep as i64,
            &path.child("readyReplicas"),
        ));
        if rep > status.replicas {
            all_errs.push(invalid(
                &path.child("readyReplicas"),
                BadValue::Int(rep as i64),
                "cannot be greater than status.replicas",
            ));
        }
    }
    if let Some(rep) = status.available_replicas {
        all_errs.extend(validate_nonnegative_field(
            rep as i64,
            &path.child("availableReplicas"),
        ));
        if rep > status.replicas {
            all_errs.push(invalid(
                &path.child("availableReplicas"),
                BadValue::Int(rep as i64),
                "cannot be greater than status.replicas",
            ));
        }
        if let Some(ready) = status.ready_replicas {
            if rep > ready {
                all_errs.push(invalid(
                    &path.child("availableReplicas"),
                    BadValue::Int(rep as i64),
                    "cannot be greater than readyReplicas",
                ));
            }
        }
    }
    if let Some(observed_generation) = status.observed_generation {
        all_errs.extend(validate_nonnegative_field(
            observed_generation,
            &path.child("observedGeneration"),
        ));
    }
    if let Some(rep) = status.terminating_replicas {
        all_errs.extend(validate_nonnegative_field(
            rep as i64,
            &path.child("terminatingReplicas"),
        ));
    }
    all_errs
}

// =============================================================================
// DaemonSet validation
// =============================================================================

pub fn validate_daemon_set(daemon_set: &DaemonSet) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let default_meta = crate::common::ObjectMeta::default();
    let meta = daemon_set.metadata.as_ref().unwrap_or(&default_meta);

    all_errs.extend(validate_object_meta(
        meta,
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
    if let (Some(new_meta), Some(old_meta)) = (
        daemon_set.metadata.as_ref(),
        old_daemon_set.metadata.as_ref(),
    ) {
        all_errs.extend(validate_object_meta_update(
            new_meta,
            old_meta,
            &Path::new("metadata"),
        ));
    }

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

    if let (Some(new_meta), Some(old_meta)) = (
        daemon_set.metadata.as_ref(),
        old_daemon_set.metadata.as_ref(),
    ) {
        all_errs.extend(validate_object_meta_update(
            new_meta,
            old_meta,
            &Path::new("metadata"),
        ));
    }

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

    if let Some(ref template) = spec.template {
        all_errs.extend(validate_template_common(
            template,
            spec.selector.as_ref(),
            &path.child("template"),
        ));
    } else {
        all_errs.push(required(&path.child("template"), "template is required"));
    }

    if let Some(min_ready_seconds) = spec.min_ready_seconds {
        all_errs.extend(validate_nonnegative_field(
            min_ready_seconds as i64,
            &path.child("minReadySeconds"),
        ));
    }

    if let Some(ref update_strategy) = spec.update_strategy {
        all_errs.extend(validate_daemon_set_update_strategy(
            update_strategy,
            &path.child("updateStrategy"),
        ));
    } else {
        all_errs.push(required(&path.child("updateStrategy"), ""));
    }

    if let Some(revision_history_limit) = spec.revision_history_limit {
        all_errs.extend(validate_nonnegative_field(
            revision_history_limit as i64,
            &path.child("revisionHistoryLimit"),
        ));
    }

    all_errs
}

fn validate_daemon_set_status(status: &super::DaemonSetStatus, path: &Path) -> ErrorList {
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
    if let Some(observed_generation) = status.observed_generation {
        all_errs.extend(validate_nonnegative_field(
            observed_generation,
            &path.child("observedGeneration"),
        ));
    }
    if let Some(updated) = status.updated_number_scheduled {
        all_errs.extend(validate_nonnegative_field(
            updated as i64,
            &path.child("updatedNumberScheduled"),
        ));
    }
    if let Some(available) = status.number_available {
        all_errs.extend(validate_nonnegative_field(
            available as i64,
            &path.child("numberAvailable"),
        ));
    }
    if let Some(unavailable) = status.number_unavailable {
        all_errs.extend(validate_nonnegative_field(
            unavailable as i64,
            &path.child("numberUnavailable"),
        ));
    }
    if let Some(count) = status.collision_count {
        all_errs.extend(validate_nonnegative_field(
            count as i64,
            &path.child("collisionCount"),
        ));
    }
    all_errs
}

fn validate_daemon_set_update_strategy(
    strategy: &super::DaemonSetUpdateStrategy,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    match strategy.r#type {
        Some(DaemonSetUpdateStrategyType::OnDelete) => {}
        Some(DaemonSetUpdateStrategyType::RollingUpdate) => {
            if let Some(ref rolling_update) = strategy.rolling_update {
                all_errs.extend(validate_rolling_update_daemon_set(
                    rolling_update,
                    &path.child("rollingUpdate"),
                ));
            } else {
                all_errs.push(required(&path.child("rollingUpdate"), ""));
            }
        }
        None => {
            all_errs.push(required(&path.child("type"), ""));
        }
    }
    all_errs
}

fn validate_rolling_update_daemon_set(
    rolling_update: &RollingUpdateDaemonSet,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if let Some(ref max_unavailable) = rolling_update.max_unavailable {
        all_errs.extend(validate_positive_int_or_percent(
            max_unavailable,
            &path.child("maxUnavailable"),
        ));
        all_errs.extend(is_not_more_than_100_percent(
            max_unavailable,
            &path.child("maxUnavailable"),
        ));
    }
    if let Some(ref max_surge) = rolling_update.max_surge {
        all_errs.extend(validate_positive_int_or_percent(
            max_surge,
            &path.child("maxSurge"),
        ));
        all_errs.extend(is_not_more_than_100_percent(
            max_surge,
            &path.child("maxSurge"),
        ));
    }

    let has_unavailable = rolling_update
        .max_unavailable
        .as_ref()
        .map(get_int_or_percent_value)
        .unwrap_or(0)
        != 0;
    let has_surge = rolling_update
        .max_surge
        .as_ref()
        .map(get_int_or_percent_value)
        .unwrap_or(0)
        != 0;
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

// =============================================================================
// ControllerRevision validation
// =============================================================================

pub fn validate_controller_revision_create(revision: &ControllerRevision) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let default_meta = crate::common::ObjectMeta::default();
    let meta = revision.metadata.as_ref().unwrap_or(&default_meta);

    all_errs.extend(validate_object_meta(
        meta,
        true,
        name_is_dns_subdomain,
        &Path::new("metadata"),
    ));
    all_errs.extend(validate_nonnegative_field(
        revision.revision,
        &Path::new("revision"),
    ));

    match revision.data.as_ref() {
        None => {
            all_errs.push(required(&Path::new("data"), "data is mandatory"));
        }
        Some(value) => {
            if !value.is_object() {
                all_errs.push(required(
                    &Path::new("data"),
                    "data must be a valid JSON object",
                ));
            }
        }
    }

    all_errs
}

pub fn validate_controller_revision_update(
    new_revision: &ControllerRevision,
    old_revision: &ControllerRevision,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if let (Some(new_meta), Some(old_meta)) = (
        new_revision.metadata.as_ref(),
        old_revision.metadata.as_ref(),
    ) {
        all_errs.extend(validate_object_meta_update(
            new_meta,
            old_meta,
            &Path::new("metadata"),
        ));
    }

    all_errs.extend(validate_controller_revision_create(new_revision));
    if new_revision.data != old_revision.data {
        all_errs.push(forbidden(&Path::new("data"), "field is immutable"));
    }

    all_errs
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ObjectMeta;
    use crate::core::v1::PodTemplateSpec;
    use crate::core::v1::pod::Container;

    fn base_template(labels: BTreeMap<String, String>) -> PodTemplateSpec {
        PodTemplateSpec {
            metadata: Some(ObjectMeta {
                labels,
                ..Default::default()
            }),
            spec: Some(PodSpec {
                termination_grace_period_seconds: Some(30),
                restart_policy: Some(restart_policy::ALWAYS.to_string()),
                dns_policy: Some("ClusterFirst".to_string()),
                containers: vec![Container {
                    name: "nginx".to_string(),
                    image: Some("nginx:latest".to_string()),
                    termination_message_policy: Some("File".to_string()),
                    ..Default::default()
                }],
                ..Default::default()
            }),
        }
    }

    #[test]
    fn deployment_requires_selector() {
        let deployment = Deployment {
            metadata: Some(ObjectMeta {
                name: Some("demo".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(DeploymentSpec::default()),
            ..Default::default()
        };
        let errs = validate_deployment(&deployment);
        assert!(!errs.is_empty());
    }

    #[test]
    fn replica_set_selector_must_match_template() {
        let selector = LabelSelector {
            match_labels: [("app".to_string(), "demo".to_string())].into(),
            match_expressions: Vec::new(),
        };
        let template_labels = [("app".to_string(), "other".to_string())].into();
        let rs = ReplicaSet {
            metadata: Some(ObjectMeta {
                name: Some("rs".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(ReplicaSetSpec {
                replicas: Some(1),
                selector: Some(selector),
                template: Some(base_template(template_labels)),
                min_ready_seconds: None,
            }),
            ..Default::default()
        };
        let errs = validate_replica_set(&rs);
        assert!(!errs.is_empty());
    }
}
