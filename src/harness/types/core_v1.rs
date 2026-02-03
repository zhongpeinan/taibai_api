//! Core v1 type registrations for the test harness.

use crate::harness::helpers::{register_type, register_type_with_path};
use crate::harness::registry::Registry;

pub fn register(registry: &mut Registry) {
    // ---- core/v1/Pod ----
    register_type::<crate::core::v1::Pod, crate::core::internal::Pod, _>(
        registry,
        "core/v1/Pod",
        crate::core::v1::validation::pod::validate_pod,
    );

    // ---- core/v1/Service ----
    register_type_with_path::<
        crate::core::v1::service::Service,
        crate::core::internal::service::Service,
        _,
    >(
        registry,
        "core/v1/Service",
        crate::core::v1::validation::service::validate_service,
    );

    // ---- core/v1/ConfigMap ----
    register_type::<crate::core::v1::config::ConfigMap, crate::core::internal::config::ConfigMap, _>(
        registry,
        "core/v1/ConfigMap",
        crate::core::v1::validation::config::validate_config_map,
    );

    // ---- core/v1/Secret ----
    register_type::<crate::core::v1::config::Secret, crate::core::internal::config::Secret, _>(
        registry,
        "core/v1/Secret",
        crate::core::v1::validation::config::validate_secret,
    );

    // ---- core/v1/Namespace ----
    register_type::<crate::core::v1::namespace::Namespace, crate::core::internal::Namespace, _>(
        registry,
        "core/v1/Namespace",
        crate::core::v1::validation::namespace::validate_namespace,
    );

    // ---- core/v1/PersistentVolume ----
    register_type_with_path::<
        crate::core::v1::persistent_volume::PersistentVolume,
        crate::core::internal::persistent_volume::PersistentVolume,
        _,
    >(
        registry,
        "core/v1/PersistentVolume",
        crate::core::v1::validation::storage::validate_persistent_volume,
    );

    // ---- core/v1/PersistentVolumeClaim ----
    register_type_with_path::<
        crate::core::v1::persistent_volume::PersistentVolumeClaim,
        crate::core::internal::persistent_volume::PersistentVolumeClaim,
        _,
    >(
        registry,
        "core/v1/PersistentVolumeClaim",
        crate::core::v1::validation::storage::validate_persistent_volume_claim,
    );

    // ---- core/v1/ServiceAccount ----
    register_type::<
        crate::core::v1::config::ServiceAccount,
        crate::core::internal::config::ServiceAccount,
        _,
    >(
        registry,
        "core/v1/ServiceAccount",
        crate::core::v1::validation::config::validate_service_account,
    );

    // ---- core/v1/Endpoints ----
    register_type::<
        crate::core::v1::service::Endpoints,
        crate::core::internal::endpoints::Endpoints,
        _,
    >(
        registry,
        "core/v1/Endpoints",
        crate::core::v1::validation::endpoints::validate_endpoints,
    );

    // ---- core/v1/Node ----
    register_type::<crate::core::v1::node::Node, crate::core::internal::Node, _>(
        registry,
        "core/v1/Node",
        crate::core::v1::validation::node::validate_node,
    );

    // ---- core/v1/LimitRange ----
    register_type::<crate::core::v1::resource::LimitRange, crate::core::internal::LimitRange, _>(
        registry,
        "core/v1/LimitRange",
        crate::core::v1::validation::resource_quota::validate_limit_range,
    );

    // ---- core/v1/ResourceQuota ----
    register_type::<
        crate::core::v1::resource::ResourceQuota,
        crate::core::internal::ResourceQuota,
        _,
    >(
        registry,
        "core/v1/ResourceQuota",
        crate::core::v1::validation::resource_quota::validate_resource_quota,
    );

    // ---- core/v1/ReplicationController ----
    register_type::<
        crate::core::v1::replication_controller::ReplicationController,
        crate::core::internal::ReplicationController,
        _,
    >(
        registry,
        "core/v1/ReplicationController",
        crate::core::v1::validation::replication_controller::validate_replication_controller,
    );
}
