use crate::common::validation::ErrorList;
use crate::rbac::internal::validation as internal_validation;
use crate::rbac::v1::ClusterRoleBinding;

pub fn validate_cluster_role_binding(role_binding: &ClusterRoleBinding) -> ErrorList {
    internal_validation::validate_cluster_role_binding(role_binding)
}

pub fn validate_cluster_role_binding_update(
    role_binding: &ClusterRoleBinding,
    old_role_binding: &ClusterRoleBinding,
) -> ErrorList {
    internal_validation::validate_cluster_role_binding_update(role_binding, old_role_binding)
}
