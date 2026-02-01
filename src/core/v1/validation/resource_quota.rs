//! ResourceQuota and LimitRange validation for Kubernetes core/v1 API.

use crate::common::ToInternal;
use crate::common::validation::ErrorList;
use crate::core::internal::validation::resource_quota as internal_resource_quota_validation;
use crate::core::v1::{LimitRange, ResourceQuota};

/// Validates a ResourceQuota.
pub fn validate_resource_quota(resource_quota: &ResourceQuota) -> ErrorList {
    let internal_quota = resource_quota.clone().to_internal();
    internal_resource_quota_validation::validate_resource_quota(&internal_quota)
}

/// Validates ResourceQuota update.
pub fn validate_resource_quota_update(new: &ResourceQuota, old: &ResourceQuota) -> ErrorList {
    let internal_new = new.clone().to_internal();
    let internal_old = old.clone().to_internal();
    internal_resource_quota_validation::validate_resource_quota_update(&internal_new, &internal_old)
}

/// Validates a LimitRange.
pub fn validate_limit_range(limit_range: &LimitRange) -> ErrorList {
    let internal_limit_range = limit_range.clone().to_internal();
    internal_resource_quota_validation::validate_limit_range(&internal_limit_range)
}
