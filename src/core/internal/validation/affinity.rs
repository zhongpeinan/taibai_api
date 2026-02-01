//! Affinity validation for Kubernetes core internal API.
//!
//! Ported from k8s.io/kubernetes/pkg/apis/core/validation/validation.go

use crate::common::validation::{
    BadValue, ErrorList, Path, duplicate, invalid, is_valid_label_value, not_supported, required,
    validate_label_name,
};
use crate::core::internal::selector::LabelSelector as InternalLabelSelector;
use crate::core::internal::{
    Affinity, NodeAffinity, PodAffinity, PodAffinityTerm, PodAntiAffinity, WeightedPodAffinityTerm,
};
use crate::core::v1::validation::helpers::validate_dns1123_label;
use std::collections::HashSet;

// ============================================================================
// Affinity Validation
// ============================================================================

pub fn validate_affinity(affinity: &Affinity, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if let Some(ref node_affinity) = affinity.node_affinity {
        all_errs.extend(validate_node_affinity(
            node_affinity,
            &path.child("nodeAffinity"),
        ));
    }

    if let Some(ref pod_affinity) = affinity.pod_affinity {
        all_errs.extend(validate_pod_affinity(
            pod_affinity,
            &path.child("podAffinity"),
        ));
    }

    if let Some(ref pod_anti_affinity) = affinity.pod_anti_affinity {
        all_errs.extend(validate_pod_anti_affinity(
            pod_anti_affinity,
            &path.child("podAntiAffinity"),
        ));
    }

    all_errs
}

fn validate_node_affinity(node_affinity: &NodeAffinity, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if let Some(ref required) = node_affinity.required_during_scheduling_ignored_during_execution {
        all_errs.extend(validate_node_selector(
            required,
            &path.child("requiredDuringSchedulingIgnoredDuringExecution"),
        ));
    }

    if !node_affinity
        .preferred_during_scheduling_ignored_during_execution
        .is_empty()
    {
        for (i, pref) in node_affinity
            .preferred_during_scheduling_ignored_during_execution
            .iter()
            .enumerate()
        {
            let pref_path = path
                .child("preferredDuringSchedulingIgnoredDuringExecution")
                .index(i);
            if pref.weight < 1 || pref.weight > 100 {
                all_errs.push(invalid(
                    &pref_path.child("weight"),
                    BadValue::Int(pref.weight as i64),
                    "must be in the range 1-100",
                ));
            }
            all_errs.extend(validate_node_selector_term(
                &pref.preference,
                &pref_path.child("preference"),
            ));
        }
    }

    all_errs
}

fn validate_node_selector(
    selector: &crate::core::internal::selector::NodeSelector,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if selector.node_selector_terms.is_empty() {
        all_errs.push(required(path, "must have at least one node selector term"));
        return all_errs;
    }

    for (i, term) in selector.node_selector_terms.iter().enumerate() {
        all_errs.extend(validate_node_selector_term(
            term,
            &path.child("nodeSelectorTerms").index(i),
        ));
    }

    all_errs
}

fn validate_node_selector_term(
    term: &crate::core::internal::selector::NodeSelectorTerm,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if term.match_expressions.is_empty() && term.match_fields.is_empty() {
        all_errs.push(required(
            path,
            "must have at least one match expression or match field",
        ));
    }

    for (i, expr) in term.match_expressions.iter().enumerate() {
        all_errs.extend(validate_node_selector_requirement(
            expr,
            &path.child("matchExpressions").index(i),
            true,
        ));
    }

    for (i, expr) in term.match_fields.iter().enumerate() {
        all_errs.extend(validate_node_selector_requirement(
            expr,
            &path.child("matchFields").index(i),
            false,
        ));
    }

    all_errs
}

fn validate_node_selector_requirement(
    req: &crate::core::internal::selector::NodeSelectorRequirement,
    path: &Path,
    is_label: bool,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if req.key.is_empty() {
        all_errs.push(required(&path.child("key"), "key is required"));
    } else if is_label {
        all_errs.extend(validate_label_name(&req.key, &path.child("key")));
    } else {
        all_errs.extend(crate::common::validation::validate_qualified_name(
            &req.key,
            &path.child("key"),
        ));
    }

    match req.operator.as_str() {
        crate::core::internal::node_selector_operator::IN
        | crate::core::internal::node_selector_operator::NOT_IN => {
            if req.values.is_empty() {
                all_errs.push(required(
                    &path.child("values"),
                    "values are required for In/NotIn",
                ));
            }
        }
        crate::core::internal::node_selector_operator::EXISTS
        | crate::core::internal::node_selector_operator::DOES_NOT_EXIST => {
            if !req.values.is_empty() {
                all_errs.push(invalid(
                    &path.child("values"),
                    BadValue::String(format!("{:?}", req.values)),
                    "values must be empty for Exists/DoesNotExist",
                ));
            }
        }
        crate::core::internal::node_selector_operator::GT
        | crate::core::internal::node_selector_operator::LT => {
            if req.values.len() != 1 {
                all_errs.push(invalid(
                    &path.child("values"),
                    BadValue::String(format!("{:?}", req.values)),
                    "must have exactly one value for Gt/Lt",
                ));
            } else if req.values[0].parse::<i64>().is_err() {
                all_errs.push(invalid(
                    &path.child("values").index(0),
                    BadValue::String(req.values[0].clone()),
                    "must be an integer for Gt/Lt",
                ));
            }
        }
        _ => {
            let valid = vec![
                crate::core::internal::node_selector_operator::IN,
                crate::core::internal::node_selector_operator::NOT_IN,
                crate::core::internal::node_selector_operator::EXISTS,
                crate::core::internal::node_selector_operator::DOES_NOT_EXIST,
                crate::core::internal::node_selector_operator::GT,
                crate::core::internal::node_selector_operator::LT,
            ];
            all_errs.push(not_supported(
                &path.child("operator"),
                BadValue::String(req.operator.clone()),
                &valid,
            ));
        }
    }

    all_errs
}

fn validate_pod_affinity(affinity: &PodAffinity, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (i, term) in affinity
        .required_during_scheduling_ignored_during_execution
        .iter()
        .enumerate()
    {
        all_errs.extend(validate_pod_affinity_term(
            term,
            &path
                .child("requiredDuringSchedulingIgnoredDuringExecution")
                .index(i),
        ));
    }

    for (i, term) in affinity
        .preferred_during_scheduling_ignored_during_execution
        .iter()
        .enumerate()
    {
        all_errs.extend(validate_weighted_pod_affinity_term(
            term,
            &path
                .child("preferredDuringSchedulingIgnoredDuringExecution")
                .index(i),
        ));
    }

    all_errs
}

fn validate_pod_anti_affinity(affinity: &PodAntiAffinity, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (i, term) in affinity
        .required_during_scheduling_ignored_during_execution
        .iter()
        .enumerate()
    {
        all_errs.extend(validate_pod_affinity_term(
            term,
            &path
                .child("requiredDuringSchedulingIgnoredDuringExecution")
                .index(i),
        ));
    }

    for (i, term) in affinity
        .preferred_during_scheduling_ignored_during_execution
        .iter()
        .enumerate()
    {
        all_errs.extend(validate_weighted_pod_affinity_term(
            term,
            &path
                .child("preferredDuringSchedulingIgnoredDuringExecution")
                .index(i),
        ));
    }

    all_errs
}

fn validate_pod_affinity_term(term: &PodAffinityTerm, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if term.topology_key.is_empty() {
        all_errs.push(required(
            &path.child("topologyKey"),
            "topologyKey is required",
        ));
    }

    if let Some(ref selector) = term.label_selector {
        all_errs.extend(validate_label_selector(
            selector,
            &path.child("labelSelector"),
        ));
    }

    if let Some(ref selector) = term.namespace_selector {
        all_errs.extend(validate_label_selector(
            selector,
            &path.child("namespaceSelector"),
        ));
    }

    for (i, namespace) in term.namespaces.iter().enumerate() {
        all_errs.extend(validate_dns1123_label(
            namespace,
            &path.child("namespaces").index(i),
        ));
    }

    all_errs
}

fn validate_weighted_pod_affinity_term(term: &WeightedPodAffinityTerm, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if term.weight < 1 || term.weight > 100 {
        all_errs.push(invalid(
            &path.child("weight"),
            BadValue::Int(term.weight as i64),
            "must be in the range 1-100",
        ));
    }

    all_errs.extend(validate_pod_affinity_term(
        &term.pod_affinity_term,
        &path.child("podAffinityTerm"),
    ));

    all_errs
}

pub fn validate_label_selector(selector: &InternalLabelSelector, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if !selector.match_labels.is_empty() {
        all_errs.extend(crate::common::validation::validate_labels(
            &selector.match_labels,
            &path.child("matchLabels"),
        ));
    }

    for (i, expr) in selector.match_expressions.iter().enumerate() {
        let expr_path = path.child("matchExpressions").index(i);
        all_errs.extend(validate_label_name(&expr.key, &expr_path.child("key")));
        match expr.operator.as_str() {
            crate::core::internal::selector::label_selector_operator::IN
            | crate::core::internal::selector::label_selector_operator::NOT_IN => {
                if expr.values.is_empty() {
                    all_errs.push(required(
                        &expr_path.child("values"),
                        "values are required for In/NotIn",
                    ));
                }
            }
            crate::core::internal::selector::label_selector_operator::EXISTS
            | crate::core::internal::selector::label_selector_operator::DOES_NOT_EXIST => {
                if !expr.values.is_empty() {
                    all_errs.push(invalid(
                        &expr_path.child("values"),
                        BadValue::String(format!("{:?}", expr.values)),
                        "values must be empty for Exists/DoesNotExist",
                    ));
                }
            }
            _ => {
                let valid = vec![
                    crate::core::internal::selector::label_selector_operator::IN,
                    crate::core::internal::selector::label_selector_operator::NOT_IN,
                    crate::core::internal::selector::label_selector_operator::EXISTS,
                    crate::core::internal::selector::label_selector_operator::DOES_NOT_EXIST,
                ];
                all_errs.push(not_supported(
                    &expr_path.child("operator"),
                    BadValue::String(expr.operator.clone()),
                    &valid,
                ));
            }
        }

        let mut seen = HashSet::new();
        for (j, value) in expr.values.iter().enumerate() {
            if !seen.insert(value) {
                all_errs.push(duplicate(
                    &expr_path.child("values").index(j),
                    BadValue::String(value.clone()),
                ));
            }
            for msg in is_valid_label_value(value) {
                all_errs.push(invalid(
                    &expr_path.child("values").index(j),
                    BadValue::String(value.clone()),
                    &msg,
                ));
            }
        }
    }

    all_errs
}
