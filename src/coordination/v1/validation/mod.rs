//! Validation wrappers for Kubernetes Coordination API v1 types.
//!
//! These wrappers convert v1 types to internal types before validation.

use crate::common::ToInternal;
use crate::common::validation::{ErrorList, Path};
use crate::coordination::internal;

use super::{CoordinatedLeaseStrategy, Lease, LeaseSpec};

// ============================================================================
// Lease Validation
// ============================================================================

/// Validates a v1 Lease by converting to internal and delegating validation.
pub fn validate_lease(obj: &Lease) -> ErrorList {
    let internal_obj = obj.clone().to_internal();
    internal::validation::validate_lease(&internal_obj)
}

/// Validates a v1 Lease update by converting to internal and delegating validation.
pub fn validate_lease_update(obj: &Lease, old: &Lease) -> ErrorList {
    let internal_obj = obj.clone().to_internal();
    let internal_old = old.clone().to_internal();
    internal::validation::validate_lease_update(&internal_obj, &internal_old)
}

/// Validates a v1 LeaseSpec by converting to internal and delegating validation.
pub fn validate_lease_spec(spec: &LeaseSpec, fld_path: &Path) -> ErrorList {
    let internal_spec = internal::LeaseSpec {
        holder_identity: spec.holder_identity.clone(),
        lease_duration_seconds: spec.lease_duration_seconds,
        acquire_time: spec.acquire_time.clone(),
        renew_time: spec.renew_time.clone(),
        lease_transitions: spec.lease_transitions,
        strategy: spec.strategy.clone(),
        preferred_holder: spec.preferred_holder.clone(),
    };
    internal::validation::validate_lease_spec(&internal_spec, fld_path)
}

// ============================================================================
// Strategy Validation
// ============================================================================

/// Validates the Strategy field in a v1 Lease.
pub fn validate_coordinated_lease_strategy(
    strategy: &CoordinatedLeaseStrategy,
    fld_path: &Path,
) -> ErrorList {
    internal::validation::validate_coordinated_lease_strategy(strategy, fld_path)
}
