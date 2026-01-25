//! ResourceQuota and LimitRange validation

use crate::common::validation::ErrorList;
use crate::core::v1::{LimitRange, ResourceQuota};

/// Validates a ResourceQuota
pub fn validate_resource_quota(_resource_quota: &ResourceQuota) -> ErrorList {
    ErrorList::new()
    // TODO: Implement in Phase 7
}

/// Validates a LimitRange
pub fn validate_limit_range(_limit_range: &LimitRange) -> ErrorList {
    ErrorList::new()
    // TODO: Implement in Phase 7
}
