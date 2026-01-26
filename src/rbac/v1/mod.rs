//! RBAC v1 API types
//!
//! This module contains the RBAC v1 API types.

pub mod rbac;

pub use rbac::{
    AggregationRule, ClusterRole, ClusterRoleBinding, ClusterRoleBindingList, ClusterRoleList,
    PolicyRule, Role, RoleBinding, RoleBindingList, RoleList, RoleRef, Subject,
};

#[cfg(test)]
mod trait_tests;
