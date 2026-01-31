//! Kubernetes Coordination API types
//!
//! This module contains the coordination API types.

pub mod internal;
pub mod v1;
pub mod v1beta1;
pub mod validation;

pub use internal::{
    CoordinatedLeaseStrategy, Lease, LeaseCandidate, LeaseCandidateList, LeaseCandidateSpec,
    LeaseList, LeaseSpec,
};
pub use v1::{
    CoordinatedLeaseStrategy as CoordinatedLeaseStrategyV1, Lease as LeaseV1,
    LeaseList as LeaseListV1, LeaseSpec as LeaseSpecV1,
};
pub use v1beta1::{
    Lease as LeaseV1beta1, LeaseCandidate as LeaseCandidateV1beta1,
    LeaseCandidateList as LeaseCandidateListV1beta1,
    LeaseCandidateSpec as LeaseCandidateSpecV1beta1, LeaseList as LeaseListV1beta1,
    LeaseSpec as LeaseSpecV1beta1,
};

// Re-export constants at module level
pub use internal::coordinated_lease_strategy;
pub use v1::coordinated_lease_strategy as coordinated_lease_strategy_v1;
