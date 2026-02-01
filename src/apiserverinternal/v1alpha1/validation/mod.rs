//! Validation for Kubernetes APIServerInternal v1alpha1 API types
//!
//! Wrapper around internal apiserverinternal validation.

use crate::apiserverinternal::internal;
use crate::apiserverinternal::v1alpha1;
use crate::common::traits::ToInternal;
use crate::common::validation::{ErrorList, Path};

pub fn validate_storage_version(obj: &v1alpha1::StorageVersion) -> ErrorList {
    let internal = obj.clone().to_internal();
    internal::validation::validate_storage_version(&internal)
}

pub fn validate_storage_version_update(
    old_obj: &v1alpha1::StorageVersion,
    new_obj: &v1alpha1::StorageVersion,
) -> ErrorList {
    let old_internal = old_obj.clone().to_internal();
    let new_internal = new_obj.clone().to_internal();
    internal::validation::validate_storage_version_update(&new_internal, &old_internal)
}

pub fn validate_storage_version_status_update(
    old_obj: &v1alpha1::StorageVersion,
    new_obj: &v1alpha1::StorageVersion,
) -> ErrorList {
    let old_internal = old_obj.clone().to_internal();
    let new_internal = new_obj.clone().to_internal();
    internal::validation::validate_storage_version_status_update(&new_internal, &old_internal)
}

pub fn validate_storage_version_name(name: &str, prefix: bool) -> Vec<String> {
    internal::validation::validate_storage_version_name(name, prefix)
}

pub fn validate_storage_version_status(
    status: &v1alpha1::StorageVersionStatus,
    path: &Path,
) -> ErrorList {
    let internal_status: internal::StorageVersionStatus = status.clone().into();
    internal::validation::validate_storage_version_status(&internal_status, path)
}
