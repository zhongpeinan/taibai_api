//! Validation wrappers for Kubernetes Policy API v1 types.
//!
//! These wrappers convert v1 types to internal types before validation.

use crate::common::validation::{ErrorList, Path, required};
use crate::common::TypeMeta;
use crate::policy::internal;
use crate::policy::v1::{
    PodDisruptionBudget, PodDisruptionBudgetList, PodDisruptionBudgetSpec,
    PodDisruptionBudgetStatus, UnhealthyPodEvictionPolicyType,
};

// ============================================================================
// PodDisruptionBudget Validation
// ============================================================================

/// Validates a v1 PodDisruptionBudget by converting to internal and delegating validation.
pub fn validate_pod_disruption_budget(obj: &PodDisruptionBudget) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if obj.spec.is_none() {
        all_errs.push(required(&Path::new("spec"), "spec is required"));
    }

    let internal_obj = to_internal_pdb(obj);
    all_errs.extend(internal::validation::validate_pod_disruption_budget(&internal_obj));

    all_errs
}

/// Validates a v1 PodDisruptionBudgetList by converting to internal and delegating validation.
pub fn validate_pod_disruption_budget_list(obj: &PodDisruptionBudgetList) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (i, item) in obj.items.iter().enumerate() {
        if item.spec.is_none() {
            all_errs.push(required(
                &Path::new("items").index(i).child("spec"),
                "spec is required",
            ));
        }
    }

    let internal_list = internal::PodDisruptionBudgetList {
        type_meta: TypeMeta::default(),
        metadata: obj.metadata.clone().unwrap_or_default(),
        items: obj.items.iter().map(to_internal_pdb).collect(),
    };

    all_errs.extend(internal::validation::validate_pod_disruption_budget_list(
        &internal_list,
    ));

    all_errs
}

/// Validates a v1 PodDisruptionBudget update by converting to internal and delegating validation.
pub fn validate_pod_disruption_budget_update(
    obj: &PodDisruptionBudget,
    old: &PodDisruptionBudget,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if obj.spec.is_none() {
        all_errs.push(required(&Path::new("spec"), "spec is required"));
    }

    let internal_obj = to_internal_pdb(obj);
    let internal_old = to_internal_pdb(old);
    all_errs.extend(internal::validation::validate_pod_disruption_budget_update(
        &internal_obj,
        &internal_old,
    ));

    all_errs
}

/// Validates a v1 PodDisruptionBudget status update by converting to internal and delegating validation.
pub fn validate_pod_disruption_budget_status_update(
    obj: &PodDisruptionBudget,
    old: &PodDisruptionBudget,
) -> ErrorList {
    let internal_obj = to_internal_pdb(obj);
    let internal_old = to_internal_pdb(old);
    internal::validation::validate_pod_disruption_budget_status_update(
        &internal_obj,
        &internal_old,
    )
}

// ============================================================================
// Conversion Helpers
// ============================================================================

fn to_internal_pdb(obj: &PodDisruptionBudget) -> internal::PodDisruptionBudget {
    internal::PodDisruptionBudget {
        type_meta: TypeMeta::default(),
        metadata: obj.metadata.clone().unwrap_or_default(),
        spec: obj
            .spec
            .clone()
            .map(to_internal_spec)
            .unwrap_or_default(),
        status: obj
            .status
            .clone()
            .map(to_internal_status)
            .unwrap_or_default(),
    }
}

fn to_internal_spec(spec: PodDisruptionBudgetSpec) -> internal::PodDisruptionBudgetSpec {
    internal::PodDisruptionBudgetSpec {
        min_available: spec.min_available,
        selector: spec.selector,
        max_unavailable: spec.max_unavailable,
        unhealthy_pod_eviction_policy: spec
            .unhealthy_pod_eviction_policy
            .map(to_internal_unhealthy_policy),
    }
}

fn to_internal_status(status: PodDisruptionBudgetStatus) -> internal::PodDisruptionBudgetStatus {
    internal::PodDisruptionBudgetStatus {
        observed_generation: status.observed_generation.unwrap_or(0),
        disrupted_pods: status.disrupted_pods,
        disruptions_allowed: status.disruptions_allowed.unwrap_or(0),
        current_healthy: status.current_healthy.unwrap_or(0),
        desired_healthy: status.desired_healthy.unwrap_or(0),
        expected_pods: status.expected_pods.unwrap_or(0),
        conditions: status.conditions,
    }
}

fn to_internal_unhealthy_policy(
    value: UnhealthyPodEvictionPolicyType,
) -> internal::UnhealthyPodEvictionPolicyType {
    match value {
        UnhealthyPodEvictionPolicyType::IfHealthyBudget => {
            internal::UnhealthyPodEvictionPolicyType::IfHealthyBudget
        }
        UnhealthyPodEvictionPolicyType::AlwaysAllow => {
            internal::UnhealthyPodEvictionPolicyType::AlwaysAllow
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_internal_defaults() {
        let pdb = PodDisruptionBudget::default();
        let internal = to_internal_pdb(&pdb);
        assert!(internal.metadata.name.is_none());
    }
}
