//! APIServerInternal v1alpha1 API types
//!
//! This module contains the APIServerInternal v1alpha1 API types.
//!
//! Corresponds to [Kubernetes APIServerInternal v1alpha1](https://github.com/kubernetes/apiserver/blob/master/pkg/apis/apiserverinternal/v1alpha1/types.go)

mod conditions;
mod conversion;
mod defaults;
mod placeholders;
mod schema;
mod storage_version;
mod traits;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod trait_tests;

pub use conditions::{ConditionStatus, StorageVersionCondition, StorageVersionConditionType};
pub use storage_version::{
    ServerStorageVersion, StorageVersion, StorageVersionList, StorageVersionSpec,
    StorageVersionStatus,
};
