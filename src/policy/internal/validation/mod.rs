//! Validation for Kubernetes Policy API internal types
//!
//! Ported from k8s.io/kubernetes/pkg/apis/policy/validation/validation.go

use crate::common::meta::{LabelSelector, label_selector_operator};
use crate::common::util::IntOrString;
use crate::common::validation::{
    BadValue, ErrorList, Path, invalid, is_valid_label_value, not_supported, required,
    validate_labels, validate_object_meta, validate_object_meta_update, validate_qualified_name,
};
use crate::policy::internal::{
    PodDisruptionBudget, PodDisruptionBudgetList, PodDisruptionBudgetSpec,
    PodDisruptionBudgetStatus,
};

// ============================================================================
// PodDisruptionBudget Validation
// ============================================================================

/// Validates a PodDisruptionBudget.
pub fn validate_pod_disruption_budget(obj: &PodDisruptionBudget) -> ErrorList {
    let mut all_errs = validate_object_meta(
        &obj.metadata,
        true,
        crate::common::validation::name_is_dns_subdomain,
        &Path::new("metadata"),
    );

    all_errs.extend(validate_pod_disruption_budget_spec(
        &obj.spec,
        &Path::new("spec"),
    ));

    all_errs
}

/// Validates a PodDisruptionBudgetList.
pub fn validate_pod_disruption_budget_list(obj: &PodDisruptionBudgetList) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (i, item) in obj.items.iter().enumerate() {
        all_errs.extend(validate_pod_disruption_budget_with_path(
            item,
            &Path::new("items").index(i),
        ));
    }

    all_errs
}

fn validate_pod_disruption_budget_with_path(
    obj: &PodDisruptionBudget,
    base_path: &Path,
) -> ErrorList {
    let mut all_errs = validate_object_meta(
        &obj.metadata,
        true,
        crate::common::validation::name_is_dns_subdomain,
        &base_path.child("metadata"),
    );

    all_errs.extend(validate_pod_disruption_budget_spec(
        &obj.spec,
        &base_path.child("spec"),
    ));

    all_errs
}

/// Validates updates to a PodDisruptionBudget.
pub fn validate_pod_disruption_budget_update(
    obj: &PodDisruptionBudget,
    old: &PodDisruptionBudget,
) -> ErrorList {
    let mut all_errs =
        validate_object_meta_update(&obj.metadata, &old.metadata, &Path::new("metadata"));

    all_errs.extend(validate_pod_disruption_budget_spec(
        &obj.spec,
        &Path::new("spec"),
    ));

    all_errs
}

/// Validates PodDisruptionBudget status fields.
pub fn validate_pod_disruption_budget_status(
    status: &PodDisruptionBudgetStatus,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if status.observed_generation < 0 {
        all_errs.push(invalid(
            &path.child("observedGeneration"),
            BadValue::Int(status.observed_generation),
            "must be greater than or equal to 0",
        ));
    }

    if status.disruptions_allowed < 0 {
        all_errs.push(invalid(
            &path.child("disruptionsAllowed"),
            BadValue::Int(status.disruptions_allowed as i64),
            "must be greater than or equal to 0",
        ));
    }

    if status.current_healthy < 0 {
        all_errs.push(invalid(
            &path.child("currentHealthy"),
            BadValue::Int(status.current_healthy as i64),
            "must be greater than or equal to 0",
        ));
    }

    if status.desired_healthy < 0 {
        all_errs.push(invalid(
            &path.child("desiredHealthy"),
            BadValue::Int(status.desired_healthy as i64),
            "must be greater than or equal to 0",
        ));
    }

    if status.expected_pods < 0 {
        all_errs.push(invalid(
            &path.child("expectedPods"),
            BadValue::Int(status.expected_pods as i64),
            "must be greater than or equal to 0",
        ));
    }

    all_errs
}

/// Validates PodDisruptionBudget status updates.
pub fn validate_pod_disruption_budget_status_update(
    obj: &PodDisruptionBudget,
    old: &PodDisruptionBudget,
) -> ErrorList {
    let mut all_errs =
        validate_object_meta_update(&obj.metadata, &old.metadata, &Path::new("metadata"));

    all_errs.extend(validate_pod_disruption_budget_status(
        &obj.status,
        &Path::new("status"),
    ));

    all_errs
}

// ============================================================================
// PodDisruptionBudgetSpec Validation
// ============================================================================

fn validate_pod_disruption_budget_spec(spec: &PodDisruptionBudgetSpec, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if let Some(ref selector) = spec.selector {
        all_errs.extend(validate_label_selector(selector, &path.child("selector")));
    } else {
        all_errs.push(required(&path.child("selector"), ""));
    }

    let min_set = spec.min_available.is_some();
    let max_set = spec.max_unavailable.is_some();

    if min_set && max_set {
        all_errs.push(invalid(
            path,
            BadValue::String("minAvailable and maxUnavailable".to_string()),
            "may not both be set",
        ));
    }

    if !min_set && !max_set {
        all_errs.push(required(path, "minAvailable or maxUnavailable is required"));
    }

    if let Some(ref value) = spec.min_available {
        all_errs.extend(validate_int_or_percent(value, &path.child("minAvailable")));
    }

    if let Some(ref value) = spec.max_unavailable {
        all_errs.extend(validate_int_or_percent(
            value,
            &path.child("maxUnavailable"),
        ));
    }

    all_errs
}

fn validate_int_or_percent(value: &IntOrString, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    match value {
        IntOrString::Int(int_value) => {
            if *int_value < 0 {
                all_errs.push(invalid(
                    path,
                    BadValue::Int(*int_value as i64),
                    "must be greater than or equal to 0",
                ));
            }
        }
        IntOrString::String(raw) => {
            if let Some(stripped) = raw.strip_suffix('%') {
                if stripped.is_empty() {
                    all_errs.push(invalid(
                        path,
                        BadValue::String(raw.clone()),
                        "must be a percentage",
                    ));
                } else if let Ok(value) = stripped.parse::<i32>() {
                    if value < 0 || value > 100 {
                        all_errs.push(invalid(
                            path,
                            BadValue::String(raw.clone()),
                            "must be between 0 and 100",
                        ));
                    }
                } else {
                    all_errs.push(invalid(
                        path,
                        BadValue::String(raw.clone()),
                        "must be a percentage",
                    ));
                }
            } else {
                all_errs.push(invalid(
                    path,
                    BadValue::String(raw.clone()),
                    "must be a percentage",
                ));
            }
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

        match requirement.operator.as_str() {
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
                continue;
            }
            for msg in is_valid_label_value(value) {
                all_errs.push(invalid(
                    &req_path.child("values").index(j),
                    BadValue::String(value.clone()),
                    &msg,
                ));
            }
        }
    }

    all_errs
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ListMeta;
    use crate::common::TypeMeta;

    fn base_pdb() -> PodDisruptionBudget {
        PodDisruptionBudget {
            type_meta: TypeMeta::default(),
            metadata: crate::common::ObjectMeta {
                name: Some("pdb".to_string()),
                namespace: Some("default".to_string()),
                resource_version: Some("1".to_string()),
                ..Default::default()
            },
            spec: PodDisruptionBudgetSpec {
                min_available: Some(IntOrString::Int(1)),
                selector: Some(LabelSelector::default()),
                max_unavailable: None,
                unhealthy_pod_eviction_policy: None,
            },
            status: PodDisruptionBudgetStatus::default(),
        }
    }

    #[test]
    fn test_validate_pdb_valid() {
        let obj = base_pdb();
        let errs = validate_pod_disruption_budget(&obj);
        assert!(errs.is_empty(), "expected no errors, got {errs:?}");
    }

    #[test]
    fn test_validate_pdb_missing_selector() {
        let mut obj = base_pdb();
        obj.spec.selector = None;
        let errs = validate_pod_disruption_budget(&obj);
        assert!(errs.errors.iter().any(|e| e.field == "spec.selector"));
    }

    #[test]
    fn test_validate_pdb_min_max_mutual_exclusive() {
        let mut obj = base_pdb();
        obj.spec.max_unavailable = Some(IntOrString::Int(1));
        let errs = validate_pod_disruption_budget(&obj);
        assert!(errs.errors.iter().any(|e| e.field == "spec"));
    }

    #[test]
    fn test_validate_pdb_percent_invalid() {
        let mut obj = base_pdb();
        obj.spec.min_available = Some(IntOrString::String("200%".to_string()));
        let errs = validate_pod_disruption_budget(&obj);
        assert!(errs.errors.iter().any(|e| e.field == "spec.minAvailable"));
    }

    #[test]
    fn test_validate_pdb_list_item_index() {
        let mut list = PodDisruptionBudgetList {
            type_meta: TypeMeta::default(),
            metadata: ListMeta::default(),
            items: vec![base_pdb()],
        };
        list.items[0].spec.selector = None;
        let errs = validate_pod_disruption_budget_list(&list);
        assert!(
            errs.errors
                .iter()
                .any(|e| e.field.contains("items[0].spec.selector"))
        );
    }
}
