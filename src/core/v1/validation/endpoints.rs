//! Endpoints validation

use crate::common::validation::ErrorList;
use crate::core::v1::Endpoints;

/// Validates Endpoints
pub fn validate_endpoints(_endpoints: &Endpoints) -> ErrorList {
    ErrorList::new()
    // TODO: Implement in Phase 7
}
