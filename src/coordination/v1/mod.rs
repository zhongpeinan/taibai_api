//! Kubernetes Coordination API v1 types
//!
//! This module contains the coordination v1 API types.
//!
//! Source: https://github.com/kubernetes/api/blob/master/coordination/v1/types.go

use crate::common::{
    ApplyDefault, HasTypeMeta, ListMeta, MicroTime, ObjectMeta, ResourceSchema, TypeMeta,
    UnimplementedConversion, VersionedObject,
};
use crate::impl_unimplemented_prost_message;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

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
/// Corresponds to [Kubernetes Lease](https://github.com/kubernetes/api/blob/master/coordination/v1/types.go#L39)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Lease {
    /// TypeMeta for this resource
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// spec contains the specification of the Lease.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<LeaseSpec>,
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
    /// to wait to force acquire it. This is measured against the time of last
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
    /// TypeMeta for this resource
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,
    /// items is a list of schema objects.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Lease>,
}

// ============================================================================
// Trait Implementations for Lease and LeaseList
// ============================================================================

// ----------------------------------------------------------------------------
// ResourceSchema Implementation
// ----------------------------------------------------------------------------

impl ResourceSchema for Lease {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "coordination.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
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
        "v1"
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
        "v1"
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
        "v1"
    }
    fn kind_static() -> &'static str {
        "LeaseList"
    }
    fn resource_static() -> &'static str {
        "leases"
    }
}

// ----------------------------------------------------------------------------
// HasTypeMeta Implementation
// ----------------------------------------------------------------------------

impl HasTypeMeta for Lease {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for LeaseList {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

// ----------------------------------------------------------------------------
// VersionedObject Implementation
// ----------------------------------------------------------------------------

impl VersionedObject for Lease {
    fn metadata(&self) -> &ObjectMeta {
        self.metadata
            .as_ref()
            .unwrap_or_else(|| static_default_object_meta())
    }

    fn metadata_mut(&mut self) -> &mut ObjectMeta {
        self.metadata.get_or_insert_with(ObjectMeta::default)
    }
}

// Helper function for static default ObjectMeta
fn static_default_object_meta() -> &'static ObjectMeta {
    static DEFAULT: OnceLock<ObjectMeta> = OnceLock::new();
    DEFAULT.get_or_init(ObjectMeta::default)
}

// Note: LeaseList does not implement VersionedObject because its metadata is ListMeta

// ----------------------------------------------------------------------------
// ApplyDefaults Implementation
// ----------------------------------------------------------------------------

impl ApplyDefault for Lease {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "coordination.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "Lease".to_string();
        }
    }
}

impl ApplyDefault for LeaseList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "coordination.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "LeaseList".to_string();
        }
    }
}

// ----------------------------------------------------------------------------
// Version Conversion Placeholder (using UnimplementedConversion)
// ----------------------------------------------------------------------------

impl UnimplementedConversion for Lease {}

// ----------------------------------------------------------------------------
// Protobuf Placeholder (using macro)
// ----------------------------------------------------------------------------

impl_unimplemented_prost_message!(Lease);
impl_unimplemented_prost_message!(LeaseList);

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {}
