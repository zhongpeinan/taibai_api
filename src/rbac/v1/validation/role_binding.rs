use crate::common::validation::ErrorList;
use crate::rbac::internal::validation as internal_validation;
use crate::rbac::v1::RoleBinding;

pub fn validate_role_binding(role_binding: &RoleBinding) -> ErrorList {
    internal_validation::validate_role_binding(role_binding)
}

pub fn validate_role_binding_update(
    role_binding: &RoleBinding,
    old_role_binding: &RoleBinding,
) -> ErrorList {
    internal_validation::validate_role_binding_update(role_binding, old_role_binding)
}
