//! Networking v1 type registrations for the test harness.

use crate::harness::helpers::register_type;
use crate::harness::registry::Registry;

pub fn register(registry: &mut Registry) {
    // ---- networking/v1/Ingress ----
    register_type::<crate::networking::v1::Ingress, crate::networking::internal::Ingress, _>(
        registry,
        "networking/v1/Ingress",
        crate::networking::v1::validation::validate_ingress,
    );

    // ---- networking/v1/IngressClass ----
    register_type::<
        crate::networking::v1::IngressClass,
        crate::networking::internal::IngressClass,
        _,
    >(
        registry,
        "networking/v1/IngressClass",
        crate::networking::v1::validation::validate_ingress_class,
    );

    // ---- networking/v1/NetworkPolicy ----
    register_type::<
        crate::networking::v1::NetworkPolicy,
        crate::networking::internal::NetworkPolicy,
        _,
    >(
        registry,
        "networking/v1/NetworkPolicy",
        crate::networking::v1::validation::validate_network_policy,
    );
}
