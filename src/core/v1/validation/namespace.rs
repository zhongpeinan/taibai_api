//! Namespace validation for Kubernetes core/v1 API.

use crate::common::ToInternal;
use crate::common::validation::ErrorList;
use crate::core::internal::validation::namespace as internal_namespace_validation;
use crate::core::v1::Namespace;

/// Validates a Namespace.
pub fn validate_namespace(namespace: &Namespace) -> ErrorList {
    let internal_namespace = namespace.clone().to_internal();
    internal_namespace_validation::validate_namespace(&internal_namespace)
}

/// Validates Namespace update.
pub fn validate_namespace_update(new: &Namespace, old: &Namespace) -> ErrorList {
    let internal_new = new.clone().to_internal();
    let internal_old = old.clone().to_internal();
    internal_namespace_validation::validate_namespace_update(&internal_new, &internal_old)
}

/// Validates Namespace status update.
pub fn validate_namespace_status_update(new: &Namespace, old: &Namespace) -> ErrorList {
    let internal_new = new.clone().to_internal();
    let internal_old = old.clone().to_internal();
    internal_namespace_validation::validate_namespace_status_update(&internal_new, &internal_old)
}
