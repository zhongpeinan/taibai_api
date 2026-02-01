use crate::common::validation::ErrorList;
use crate::rbac::internal::validation as internal_validation;
use crate::rbac::v1::Role;

pub fn validate_role(role: &Role) -> ErrorList {
    internal_validation::validate_role(role)
}

pub fn validate_role_update(role: &Role, old_role: &Role) -> ErrorList {
    internal_validation::validate_role_update(role, old_role)
}
