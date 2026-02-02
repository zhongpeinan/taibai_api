//! RBAC v1 type registrations for the test harness.

use crate::harness::helpers::register_type;
use crate::harness::registry::Registry;

pub fn register(registry: &mut Registry) {
    // ---- rbac/v1/Role ----
    register_type::<crate::rbac::v1::Role, crate::rbac::internal::Role, _>(
        registry,
        "rbac/v1/Role",
        crate::rbac::v1::validation::validate_role,
    );

    // Note: ClusterRole validation requires ClusterRoleValidationOptions, so we skip it for now
    // TODO: Add ClusterRole when we have a way to handle validation options

    // ---- rbac/v1/RoleBinding ----
    register_type::<crate::rbac::v1::RoleBinding, crate::rbac::internal::RoleBinding, _>(
        registry,
        "rbac/v1/RoleBinding",
        crate::rbac::v1::validation::validate_role_binding,
    );

    // ---- rbac/v1/ClusterRoleBinding ----
    register_type::<
        crate::rbac::v1::ClusterRoleBinding,
        crate::rbac::internal::ClusterRoleBinding,
        _,
    >(
        registry,
        "rbac/v1/ClusterRoleBinding",
        crate::rbac::v1::validation::validate_cluster_role_binding,
    );
}
