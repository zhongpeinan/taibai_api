//! RBAC v1 API types
//!
//! This module contains the RBAC v1 API types.

pub mod rbac;
pub mod validation;

pub use rbac::{
    AggregationRule, ClusterRole, ClusterRoleBinding, ClusterRoleBindingList, ClusterRoleList,
    PolicyRule, Role, RoleBinding, RoleBindingList, RoleList, RoleRef, Subject,
};

// Re-export constant modules for use in validation
pub use rbac::{api_group, subject_kind};

#[cfg(test)]
mod trait_tests;
