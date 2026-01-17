//! APIServerInternal internal API types
//!
//! This module re-exports the APIServerInternal v1alpha1 types, as they are identical
//! to the internal types defined in `k8s.io/kubernetes/pkg/apis/apiserverinternal`.
//!
//! In Kubernetes, the internal types (pkg/apis/apiserverinternal) and the public v1alpha1 API
//! types (api/apiserverinternal/v1alpha1) have the same structure. The internal types are used
//! within Kubernetes for internal logic, while v1alpha1 types are exposed via the API.
//!
//! This module provides the internal types by re-exporting from v1alpha1, maintaining
//! a single source of truth for the type definitions.

// Core apiserverinternal types
pub use crate::apiserverinternal::v1alpha1::{
    ConditionStatus, ServerStorageVersion, StorageVersion, StorageVersionCondition,
    StorageVersionConditionType, StorageVersionList, StorageVersionSpec, StorageVersionStatus,
};
