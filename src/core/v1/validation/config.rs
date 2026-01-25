//! ConfigMap, Secret, and ServiceAccount validation

use crate::common::validation::ErrorList;
use crate::core::v1::{ConfigMap, Secret, ServiceAccount};

/// Validates a ConfigMap
pub fn validate_config_map(_config_map: &ConfigMap) -> ErrorList {
    ErrorList::new()
    // TODO: Implement in Phase 6
}

/// Validates a Secret
pub fn validate_secret(_secret: &Secret) -> ErrorList {
    ErrorList::new()
    // TODO: Implement in Phase 6
}

/// Validates a ServiceAccount
pub fn validate_service_account(_service_account: &ServiceAccount) -> ErrorList {
    ErrorList::new()
    // TODO: Implement in Phase 6
}
