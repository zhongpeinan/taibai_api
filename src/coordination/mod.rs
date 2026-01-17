//! Kubernetes Coordination API types
//!
//! This module contains the coordination API types.

pub mod internal;
pub mod v1;

pub use internal::{
    CoordinatedLeaseStrategy, Lease, LeaseCandidate, LeaseCandidateList, LeaseCandidateSpec,
    LeaseList, LeaseSpec,
};
pub use v1::{
    CoordinatedLeaseStrategy as CoordinatedLeaseStrategyV1, Lease as LeaseV1,
    LeaseList as LeaseListV1, LeaseSpec as LeaseSpecV1,
};

// Re-export constants at module level
pub use internal::coordinated_lease_strategy;
pub use v1::coordinated_lease_strategy as coordinated_lease_strategy_v1;
