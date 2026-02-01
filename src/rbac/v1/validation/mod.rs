//! Validation for Kubernetes RBAC v1 API types

mod cluster_role;
mod cluster_role_binding;
mod role;
mod role_binding;

pub use cluster_role::{
    validate_cluster_role, validate_cluster_role_update, ClusterRoleValidationOptions,
};
pub use cluster_role_binding::{
    validate_cluster_role_binding, validate_cluster_role_binding_update,
};
pub use role::{validate_role, validate_role_update};
pub use role_binding::{validate_role_binding, validate_role_binding_update};
