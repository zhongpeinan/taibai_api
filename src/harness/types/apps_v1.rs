//! Apps v1 type registrations for the test harness.

use crate::harness::helpers::register_type;
use crate::harness::registry::Registry;

pub fn register(registry: &mut Registry) {
    // ---- apps/v1/Deployment ----
    register_type::<crate::apps::v1::Deployment, crate::apps::internal::Deployment, _>(
        registry,
        "apps/v1/Deployment",
        crate::apps::v1::validation::validate_deployment,
    );

    // ---- apps/v1/DaemonSet ----
    register_type::<crate::apps::v1::DaemonSet, crate::apps::internal::DaemonSet, _>(
        registry,
        "apps/v1/DaemonSet",
        crate::apps::v1::validation::validate_daemon_set,
    );

    // ---- apps/v1/StatefulSet ----
    register_type::<crate::apps::v1::StatefulSet, crate::apps::internal::StatefulSet, _>(
        registry,
        "apps/v1/StatefulSet",
        crate::apps::v1::validation::validate_stateful_set,
    );
}
