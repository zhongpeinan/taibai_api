//! Apps v1 type registrations for the test harness.

use crate::harness::helpers::{register_type, register_type_no_validate};
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

    // ---- apps/v1/ReplicaSet ----
    register_type::<crate::apps::v1::ReplicaSet, crate::apps::internal::ReplicaSet, _>(
        registry,
        "apps/v1/ReplicaSet",
        crate::apps::v1::validation::validate_replica_set,
    );

    // ---- apps/v1/ControllerRevision ----
    register_type_no_validate::<
        crate::apps::v1::ControllerRevision,
        crate::apps::internal::ControllerRevision,
    >(registry, "apps/v1/ControllerRevision");
}
