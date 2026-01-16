//! RBAC internal API types
//!
//! This module re-exports the RBAC v1 types, as they are identical
//! to the internal types defined in `k8s.io/kubernetes/pkg/apis/rbac`.
//!
//! In Kubernetes, the internal types (pkg/apis/rbac) and the public v1 API
//! types (api/rbac/v1) have the same structure. The internal types are used
//! within Kubernetes for internal logic, while v1 types are exposed via the API.
//!
//! This module provides the internal types by re-exporting from v1, maintaining
//! a single source of truth for the type definitions.

pub use crate::rbac::v1::{
    AggregationRule, ClusterRole, ClusterRoleBinding, ClusterRoleBindingList, ClusterRoleList,
    PolicyRule, Role, RoleBinding, RoleBindingList, RoleList, RoleRef, Subject,
};
