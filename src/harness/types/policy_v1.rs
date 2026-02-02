//! Policy v1 type registrations for the test harness.

use crate::harness::helpers::register_type;
use crate::harness::registry::Registry;

pub fn register(registry: &mut Registry) {
    // ---- policy/v1/PodDisruptionBudget ----
    register_type::<
        crate::policy::v1::PodDisruptionBudget,
        crate::policy::internal::PodDisruptionBudget,
        _,
    >(
        registry,
        "policy/v1/PodDisruptionBudget",
        crate::policy::v1::validation::validate_pod_disruption_budget,
    );
}
