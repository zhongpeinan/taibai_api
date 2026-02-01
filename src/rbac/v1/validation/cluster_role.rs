use crate::common::validation::ErrorList;
use crate::rbac::internal::validation as internal_validation;
use crate::rbac::v1::ClusterRole;

pub use internal_validation::ClusterRoleValidationOptions;

pub fn validate_cluster_role(
    role: &ClusterRole,
    opts: ClusterRoleValidationOptions,
) -> ErrorList {
    internal_validation::validate_cluster_role(role, opts)
}

pub fn validate_cluster_role_update(
    role: &ClusterRole,
    old_role: &ClusterRole,
    opts: ClusterRoleValidationOptions,
) -> ErrorList {
    internal_validation::validate_cluster_role_update(role, old_role, opts)
}
