//! Service validation

use crate::common::validation::{ErrorList, Path};
use crate::core::v1::{Service, ServiceSpec};

/// Validates a Service
pub fn validate_service(_service: &Service) -> ErrorList {
    ErrorList::new()
    // TODO: Implement in Phase 5
}

/// Validates a ServiceSpec
pub fn validate_service_spec(_spec: &ServiceSpec, _path: &Path) -> ErrorList {
    ErrorList::new()
    // TODO: Implement in Phase 5
}

/// Validates Service update
pub fn validate_service_update(_new: &Service, _old: &Service) -> ErrorList {
    ErrorList::new()
    // TODO: Implement in Phase 5
}
