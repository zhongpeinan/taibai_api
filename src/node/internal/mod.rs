//! Node internal API types
//!
//! This module re-exports the node v1 types, as they are identical
//! to the internal types defined in `k8s.io/kubernetes/pkg/apis/node`.
//!
//! In Kubernetes, the internal types (pkg/apis/node) and the public v1 API
//! types (api/node/v1) have the same structure. The internal types are used
//! within Kubernetes for internal logic, while v1 types are exposed via the API.
//!
//! This module provides the internal types by re-exporting from v1, maintaining
//! a single source of truth for the type definitions.

pub use crate::node::v1::{Overhead, RuntimeClass, RuntimeClassList, Scheduling};
