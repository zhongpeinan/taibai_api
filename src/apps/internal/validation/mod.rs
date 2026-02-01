//! Validation for Kubernetes Apps internal API types
//!
//! Ported from k8s.io/kubernetes/pkg/apis/apps/validation/validation.go

use std::collections::BTreeMap;

use crate::common::FromInternal;
use crate::common::meta::label_selector_operator;
use crate::common::validation::{
    BadValue, ErrorList, Path, forbidden, invalid, not_supported, required, validate_labels,
    validate_qualified_name,
};
use crate::common::{IntOrString, LabelSelector};
use crate::core::v1::pod::restart_policy;
use crate::core::v1::validation::helpers::validate_nonnegative_field;
use crate::core::v1::validation::template::validate_pod_template_spec;

pub mod controllerrevision;
pub mod daemonset;
pub mod deployment;
pub mod replicaset;
pub mod statefulset;

pub use controllerrevision::{
    validate_controller_revision_create, validate_controller_revision_update,
};
pub use daemonset::{
    validate_daemon_set, validate_daemon_set_status_update, validate_daemon_set_update,
};
pub use deployment::{
    validate_deployment, validate_deployment_status_update, validate_deployment_update,
};
pub use replicaset::{
    validate_replica_set, validate_replica_set_status_update, validate_replica_set_update,
};
pub use statefulset::{
    validate_stateful_set, validate_stateful_set_status_update, validate_stateful_set_update,
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

fn validate_restart_policy_always(spec: &crate::core::v1::pod::PodSpec, path: &Path) -> ErrorList {
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
    template: &crate::core::internal::PodTemplateSpec,
    selector: Option<&LabelSelector>,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if let Some(selector) = selector {
        if !selector.match_labels.is_empty() || !selector.match_expressions.is_empty() {
            let labels = &template.metadata.labels;
            if !selector_matches_labels(selector, labels) {
                all_errs.push(invalid(
                    &path.child("metadata").child("labels"),
                    BadValue::String(format!("{labels:?}")),
                    "`selector` does not match template `labels`",
                ));
            }
        }
    }

    let v1_template = crate::core::v1::PodTemplateSpec::from_internal(template.clone());
    all_errs.extend(validate_pod_template_spec(&v1_template, path));
    if let Some(ref pod_spec) = v1_template.spec {
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
