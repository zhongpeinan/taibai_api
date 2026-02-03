//! Autoscaling v2 type registrations for the test harness.

use crate::harness::helpers::register_type_no_validate;
use crate::harness::registry::Registry;

pub fn register(registry: &mut Registry) {
    // ---- autoscaling/v2/HorizontalPodAutoscaler ----
    register_type_no_validate::<
        crate::autoscaling::v2::HorizontalPodAutoscaler,
        crate::autoscaling::internal::HorizontalPodAutoscaler,
    >(registry, "autoscaling/v2/HorizontalPodAutoscaler");
}
