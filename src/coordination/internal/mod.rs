//! Kubernetes Coordination API Internal Types
//!
//! This module contains type definitions from k8s.io/kubernetes/pkg/apis/coordination/types.go
//! that are used internally by the Kubernetes API.
//!
//! Source: https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/coordination/types.go

use crate::common::{ListMeta, MicroTime, ObjectMeta, ResourceSchema, TypeMeta};
use crate::impl_has_object_meta;
use serde::{Deserialize, Serialize};

/// CoordinatedLeaseStrategy defines the strategy for picking the leader for coordinated leader election.
pub type CoordinatedLeaseStrategy = String;

/// CoordinatedLeaseStrategy constants
pub mod coordinated_lease_strategy {
    /// OldestEmulationVersion picks the oldest LeaseCandidate, where "oldest" is defined as follows:
    /// 1) Select the candidate(s) with the lowest emulation version
    /// 2) If multiple candidates have the same emulation version, select the candidate(s) with the lowest binary version.
    /// 3) If multiple candidates have the same binary version, select the candidate with the oldest creationTimestamp.
    pub const OLDEST_EMULATION_VERSION: &str = "OldestEmulationVersion";
}

// ============================================================================
// Lease
// ============================================================================

/// Lease defines a lease concept.
///
/// Corresponds to [Kubernetes Lease](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/coordination/types.go#L37)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Lease {
    /// TypeMeta describes the type of this object.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    pub metadata: ObjectMeta,
    /// spec contains the specification of the Lease.
    pub spec: LeaseSpec,
}
impl_has_object_meta!(Lease);

// ============================================================================
// LeaseSpec
// ============================================================================

/// LeaseSpec is a specification of a Lease.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct LeaseSpec {
    /// holderIdentity contains the identity of the holder of a current lease.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub holder_identity: Option<String>,
    /// leaseDurationSeconds is a duration that candidates for a lease need
    /// to wait to force acquire it. This is measure against time of last
    /// observed renewTime.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lease_duration_seconds: Option<i32>,
    /// acquireTime is a time when the current lease was acquired.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acquire_time: Option<MicroTime>,
    /// renewTime is a time when the current holder of a lease has last
    /// updated the lease.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renew_time: Option<MicroTime>,
    /// leaseTransitions is the number of transitions of a lease between
    /// holders.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lease_transitions: Option<i32>,
    /// Strategy indicates the strategy for picking the leader for coordinated leader election.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strategy: Option<CoordinatedLeaseStrategy>,
    /// PreferredHolder signals to a lease holder that the lease has a
    /// more optimal holder and should be given up.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_holder: Option<String>,
}

// ============================================================================
// LeaseList
// ============================================================================

/// LeaseList is a list of Lease objects.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct LeaseList {
    /// TypeMeta describes the type of this object.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(default)]
    pub metadata: ListMeta,
    /// items is a list of schema objects.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Lease>,
}

// ============================================================================
// LeaseCandidate
// ============================================================================

/// LeaseCandidate defines a candidate for a Lease object.
///
/// Candidates are created such that coordinated leader election will pick the best
/// leader from the list of candidates.
///
/// Corresponds to [Kubernetes LeaseCandidate](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/coordination/types.go#L99)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LeaseCandidate {
    /// TypeMeta describes the type of this object.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    pub metadata: ObjectMeta,
    /// spec contains the specification of the Lease.
    pub spec: LeaseCandidateSpec,
}
impl_has_object_meta!(LeaseCandidate);

// ============================================================================
// LeaseCandidateSpec
// ============================================================================

/// LeaseCandidateSpec is a specification of a LeaseCandidate.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct LeaseCandidateSpec {
    /// LeaseName is the name of the lease for which this candidate is contending.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub lease_name: String,
    /// PingTime is the last time that the server has requested the LeaseCandidate
    /// to renew.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ping_time: Option<MicroTime>,
    /// RenewTime is the time that the LeaseCandidate was last updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renew_time: Option<MicroTime>,
    /// BinaryVersion is the binary version. It must be in a semver format without leading `v`.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub binary_version: String,
    /// EmulationVersion is the emulation version. It must be in a semver format without leading `v`.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub emulation_version: String,
    /// Strategy is the strategy that coordinated leader election will use for picking the leader.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub strategy: CoordinatedLeaseStrategy,
}

// ============================================================================
// LeaseCandidateList
// ============================================================================

/// LeaseCandidateList is a list of LeaseCandidate objects.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct LeaseCandidateList {
    /// TypeMeta describes the type of this object.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(default)]
    pub metadata: ListMeta,
    /// items is a list of schema objects.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<LeaseCandidate>,
}

// ============================================================================
// Trait Implementations
// ============================================================================

impl ResourceSchema for Lease {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "coordination.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "__internal"
    }
    fn kind(_: &Self::Meta) -> &str {
        "Lease"
    }
    fn resource(_: &Self::Meta) -> &str {
        "leases"
    }

    fn group_static() -> &'static str {
        "coordination.k8s.io"
    }
    fn version_static() -> &'static str {
        "__internal"
    }
    fn kind_static() -> &'static str {
        "Lease"
    }
    fn resource_static() -> &'static str {
        "leases"
    }
}

impl ResourceSchema for LeaseList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "coordination.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "__internal"
    }
    fn kind(_: &Self::Meta) -> &str {
        "LeaseList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "leases"
    }

    fn group_static() -> &'static str {
        "coordination.k8s.io"
    }
    fn version_static() -> &'static str {
        "__internal"
    }
    fn kind_static() -> &'static str {
        "LeaseList"
    }
    fn resource_static() -> &'static str {
        "leases"
    }
}

impl ResourceSchema for LeaseCandidate {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "coordination.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "__internal"
    }
    fn kind(_: &Self::Meta) -> &str {
        "LeaseCandidate"
    }
    fn resource(_: &Self::Meta) -> &str {
        "leasecandidates"
    }

    fn group_static() -> &'static str {
        "coordination.k8s.io"
    }
    fn version_static() -> &'static str {
        "__internal"
    }
    fn kind_static() -> &'static str {
        "LeaseCandidate"
    }
    fn resource_static() -> &'static str {
        "leasecandidates"
    }
}

impl ResourceSchema for LeaseCandidateList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "coordination.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "__internal"
    }
    fn kind(_: &Self::Meta) -> &str {
        "LeaseCandidateList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "leasecandidates"
    }

    fn group_static() -> &'static str {
        "coordination.k8s.io"
    }
    fn version_static() -> &'static str {
        "__internal"
    }
    fn kind_static() -> &'static str {
        "LeaseCandidateList"
    }
    fn resource_static() -> &'static str {
        "leasecandidates"
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {}
