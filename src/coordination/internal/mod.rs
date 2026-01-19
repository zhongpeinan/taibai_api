//! Kubernetes Coordination API Internal Types
//!
//! This module contains type definitions from k8s.io/kubernetes/pkg/apis/coordination/types.go
//! that are used internally by the Kubernetes API.
//!
//! Source: https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/coordination/types.go

use crate::common::{ListMeta, MicroTime, ObjectMeta, TypeMeta};
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
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Lease {
    /// TypeMeta describes the type of this object.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    pub metadata: ObjectMeta,
    /// spec contains the specification of the Lease.
    pub spec: LeaseSpec,
}

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
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct LeaseList {
    /// TypeMeta describes the type of this object.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    pub metadata: ListMeta,
    /// items is a list of schema objects.
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
pub struct LeaseCandidate {
    /// TypeMeta describes the type of this object.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    pub metadata: ObjectMeta,
    /// spec contains the specification of the Lease.
    pub spec: LeaseCandidateSpec,
}

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
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct LeaseCandidateList {
    /// TypeMeta describes the type of this object.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    pub metadata: ListMeta,
    /// items is a list of schema objects.
    pub items: Vec<LeaseCandidate>,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lease_spec_default() {
        let spec = LeaseSpec::default();
        assert!(spec.holder_identity.is_none());
        assert!(spec.lease_duration_seconds.is_none());
        assert!(spec.acquire_time.is_none());
        assert!(spec.renew_time.is_none());
        assert!(spec.lease_transitions.is_none());
        assert!(spec.strategy.is_none());
        assert!(spec.preferred_holder.is_none());
    }

    #[test]
    fn test_lease_candidate_spec_default() {
        let spec = LeaseCandidateSpec::default();
        assert!(spec.lease_name.is_empty());
        assert!(spec.ping_time.is_none());
        assert!(spec.renew_time.is_none());
        assert!(spec.binary_version.is_empty());
        assert!(spec.emulation_version.is_empty());
        assert!(spec.strategy.is_empty());
    }

    #[test]
    fn test_coordinated_lease_strategy_constants() {
        assert_eq!(
            coordinated_lease_strategy::OLDEST_EMULATION_VERSION,
            "OldestEmulationVersion"
        );
    }

    #[test]
    fn test_lease_spec_with_holder_identity() {
        let spec = LeaseSpec {
            holder_identity: Some("node-1".to_string()),
            ..Default::default()
        };
        assert_eq!(spec.holder_identity, Some("node-1".to_string()));
    }

    #[test]
    fn test_lease_spec_with_micro_time() {
        let spec = LeaseSpec {
            acquire_time: Some(MicroTime::from_str("2024-01-15T10:00:00.123456Z").unwrap()),
            renew_time: Some(MicroTime::from_str("2024-01-15T10:00:05.123456Z").unwrap()),
            ..Default::default()
        };
        assert!(spec.acquire_time.is_some());
        assert!(spec.renew_time.is_some());
        assert_eq!(
            &spec.acquire_time.as_ref().unwrap().to_rfc3339(),
            "2024-01-15T10:00:00.123456Z"
        );
    }

    #[test]
    fn test_lease_candidate_spec_with_fields() {
        let spec = LeaseCandidateSpec {
            lease_name: "my-lease".to_string(),
            binary_version: "1.28.0".to_string(),
            emulation_version: "1.27.0".to_string(),
            strategy: coordinated_lease_strategy::OLDEST_EMULATION_VERSION.to_string(),
            ..Default::default()
        };
        assert_eq!(spec.lease_name, "my-lease");
        assert_eq!(spec.binary_version, "1.28.0");
        assert_eq!(spec.emulation_version, "1.27.0");
        assert_eq!(spec.strategy, "OldestEmulationVersion");
    }

    #[test]
    fn test_lease_serialize() {
        let lease = Lease {
            type_meta: TypeMeta::default(),
            metadata: ObjectMeta {
                name: Some("my-lease".to_string()),
                ..Default::default()
            },
            spec: LeaseSpec {
                holder_identity: Some("node-1".to_string()),
                lease_duration_seconds: Some(15),
                ..Default::default()
            },
        };
        let json = serde_json::to_string(&lease).unwrap();
        assert!(json.contains("\"holderIdentity\":\"node-1\""));
        assert!(json.contains("\"leaseDurationSeconds\":15"));
    }

    #[test]
    fn test_lease_list_empty() {
        let list = LeaseList {
            type_meta: TypeMeta::default(),
            metadata: ListMeta::default(),
            items: vec![],
        };
        assert!(list.items.is_empty());
    }

    #[test]
    fn test_lease_list_with_items() {
        let list = LeaseList {
            type_meta: TypeMeta::default(),
            metadata: ListMeta::default(),
            items: vec![Lease {
                type_meta: TypeMeta::default(),
                metadata: ObjectMeta {
                    name: Some("lease-1".to_string()),
                    ..Default::default()
                },
                spec: LeaseSpec::default(),
            }],
        };
        assert_eq!(list.items.len(), 1);
    }

    #[test]
    fn test_lease_candidate_serialize() {
        let candidate = LeaseCandidate {
            type_meta: TypeMeta::default(),
            metadata: ObjectMeta {
                name: Some("candidate-1".to_string()),
                ..Default::default()
            },
            spec: LeaseCandidateSpec {
                lease_name: "my-lease".to_string(),
                binary_version: "1.28.0".to_string(),
                emulation_version: "1.27.0".to_string(),
                strategy: "OldestEmulationVersion".to_string(),
                ..Default::default()
            },
        };
        let json = serde_json::to_string(&candidate).unwrap();
        assert!(json.contains("\"leaseName\":\"my-lease\""));
        assert!(json.contains("\"binaryVersion\":\"1.28.0\""));
    }
}
