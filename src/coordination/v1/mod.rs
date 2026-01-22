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
mod tests {
    use super::*;

    // ============================================================================
    // Lease Tests
    // ============================================================================

    #[test]
    fn test_lease_default() {
        let lease = Lease::default();
        assert!(lease.metadata.is_none());
        assert!(lease.spec.is_none());
    }

    #[test]
    fn test_lease_with_metadata() {
        let lease = Lease {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("my-lease".to_string()),
                ..Default::default()
            }),
            spec: Some(LeaseSpec::default()),
        };
        assert_eq!(lease.metadata.unwrap().name.unwrap(), "my-lease");
    }

    #[test]
    fn test_lease_serialize() {
        let lease = Lease {
            type_meta: TypeMeta::default(),
            metadata: None,
            spec: Some(LeaseSpec {
                holder_identity: Some("node-1".to_string()),
                lease_duration_seconds: Some(15),
                ..Default::default()
            }),
        };
        let json = serde_json::to_string(&lease).unwrap();
        assert!(json.contains("\"holderIdentity\":\"node-1\""));
        assert!(json.contains("\"leaseDurationSeconds\":15"));
    }

    #[test]
    fn test_lease_deserialize() {
        let json = r#"{"spec":{"holderIdentity":"node-1","leaseDurationSeconds":15}}"#;
        let lease: Lease = serde_json::from_str(json).unwrap();
        assert!(lease.spec.is_some());
        let spec = lease.spec.unwrap();
        assert_eq!(spec.holder_identity, Some("node-1".to_string()));
        assert_eq!(spec.lease_duration_seconds, Some(15));
    }

    #[test]
    fn test_lease_round_trip() {
        let original = Lease {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("my-lease".to_string()),
                ..Default::default()
            }),
            spec: Some(LeaseSpec {
                holder_identity: Some("node-1".to_string()),
                lease_duration_seconds: Some(15),
                acquire_time: Some(MicroTime::from_str("2024-01-15T10:00:00.123456Z").unwrap()),
                renew_time: Some(MicroTime::from_str("2024-01-15T10:00:05.123456Z").unwrap()),
                lease_transitions: Some(5),
                strategy: Some("OldestEmulationVersion".to_string()),
                preferred_holder: Some("node-2".to_string()),
            }),
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Lease = serde_json::from_str(&json).unwrap();
        assert_eq!(
            original.metadata.unwrap().name,
            deserialized.metadata.unwrap().name
        );
        assert_eq!(
            original.spec.unwrap().holder_identity,
            deserialized.spec.unwrap().holder_identity
        );
    }

    // ============================================================================
    // LeaseSpec Tests
    // ============================================================================

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
    fn test_lease_spec_with_all_fields() {
        let spec = LeaseSpec {
            holder_identity: Some("node-1".to_string()),
            lease_duration_seconds: Some(15),
            acquire_time: Some(MicroTime::from_str("2024-01-15T10:00:00.123456Z").unwrap()),
            renew_time: Some(MicroTime::from_str("2024-01-15T10:00:05.123456Z").unwrap()),
            lease_transitions: Some(5),
            strategy: Some("OldestEmulationVersion".to_string()),
            preferred_holder: Some("node-2".to_string()),
        };
        assert_eq!(spec.holder_identity, Some("node-1".to_string()));
        assert_eq!(spec.lease_duration_seconds, Some(15));
        assert!(spec.acquire_time.is_some());
        assert!(spec.renew_time.is_some());
        assert_eq!(spec.lease_transitions, Some(5));
        assert_eq!(spec.strategy, Some("OldestEmulationVersion".to_string()));
        assert_eq!(spec.preferred_holder, Some("node-2".to_string()));
    }

    #[test]
    fn test_lease_spec_serialize() {
        let spec = LeaseSpec {
            holder_identity: Some("node-1".to_string()),
            lease_duration_seconds: Some(15),
            ..Default::default()
        };
        let json = serde_json::to_string(&spec).unwrap();
        assert!(json.contains("\"holderIdentity\":\"node-1\""));
        assert!(json.contains("\"leaseDurationSeconds\":15"));
    }

    // ============================================================================
    // LeaseList Tests
    // ============================================================================

    #[test]
    fn test_lease_list_default() {
        let list = LeaseList::default();
        assert!(list.metadata.is_none());
        assert!(list.items.is_empty());
    }

    #[test]
    fn test_lease_list_empty() {
        let list = LeaseList {
            type_meta: TypeMeta::default(),
            metadata: None,
            items: vec![],
        };
        assert!(list.items.is_empty());
        // Empty vectors should be skipped during serialization
        let json = serde_json::to_string(&list).unwrap();
        assert!(!json.contains("items"));
    }

    #[test]
    fn test_lease_list_with_items() {
        let list = LeaseList {
            type_meta: TypeMeta::default(),
            metadata: None,
            items: vec![
                Lease {
                    type_meta: TypeMeta::default(),
                    metadata: Some(ObjectMeta {
                        name: Some("lease-1".to_string()),
                        ..Default::default()
                    }),
                    spec: None,
                },
                Lease {
                    type_meta: TypeMeta::default(),
                    metadata: Some(ObjectMeta {
                        name: Some("lease-2".to_string()),
                        ..Default::default()
                    }),
                    spec: None,
                },
            ],
        };
        assert_eq!(list.items.len(), 2);
    }

    #[test]
    fn test_lease_list_serialize() {
        let list = LeaseList {
            type_meta: TypeMeta::default(),
            metadata: None,
            items: vec![Lease {
                type_meta: TypeMeta::default(),
                metadata: Some(ObjectMeta {
                    name: Some("my-lease".to_string()),
                    ..Default::default()
                }),
                spec: None,
            }],
        };
        let json = serde_json::to_string(&list).unwrap();
        assert!(json.contains("\"items\""));
        assert!(json.contains("\"my-lease\""));
    }

    #[test]
    fn test_lease_list_deserialize() {
        let json = r#"{"items":[{"metadata":{"name":"lease-1"}}]}"#;
        let list: LeaseList = serde_json::from_str(json).unwrap();
        assert_eq!(list.items.len(), 1);
        assert_eq!(
            list.items[0]
                .metadata
                .as_ref()
                .unwrap()
                .name
                .as_ref()
                .unwrap(),
            "lease-1"
        );
    }

    // ============================================================================
    // CoordinatedLeaseStrategy Constants Tests
    // ============================================================================

    #[test]
    fn test_coordinated_lease_strategy_constants() {
        assert_eq!(
            coordinated_lease_strategy::OLDEST_EMULATION_VERSION,
            "OldestEmulationVersion"
        );
    }

    // ============================================================================
    // MicroTime Integration Tests
    // ============================================================================

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
        assert_eq!(
            &spec.renew_time.as_ref().unwrap().to_rfc3339(),
            "2024-01-15T10:00:05.123456Z"
        );
    }

    #[test]
    fn test_lease_spec_micro_time_serialization() {
        let spec = LeaseSpec {
            acquire_time: Some(MicroTime::from_str("2024-01-15T10:00:00.123456Z").unwrap()),
            ..Default::default()
        };
        let json = serde_json::to_string(&spec).unwrap();
        assert!(json.contains("\"acquireTime\":\"2024-01-15T10:00:00.123456Z\""));
    }

    #[test]
    fn test_lease_spec_micro_time_deserialization() {
        let json = r#"{"acquireTime":"2024-01-15T10:00:00.123456Z"}"#;
        let spec: LeaseSpec = serde_json::from_str(json).unwrap();
        assert!(spec.acquire_time.is_some());
        assert_eq!(
            &spec.acquire_time.unwrap().to_rfc3339(),
            "2024-01-15T10:00:00.123456Z"
        );
    }

    // ============================================================================
    // Edge Case Tests
    // ============================================================================

    #[test]
    fn test_lease_empty_fields_omitted() {
        let lease = Lease {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("my-lease".to_string()),
                ..Default::default()
            }),
            spec: Some(LeaseSpec {
                holder_identity: Some("node-1".to_string()),
                ..Default::default()
            }),
        };
        let json = serde_json::to_string(&lease).unwrap();
        // Only holderIdentity should be present, other fields should be omitted
        assert!(json.contains("\"holderIdentity\":\"node-1\""));
        // Fields with None values should not be in the JSON
        assert!(!json.contains("leaseDurationSeconds"));
        assert!(!json.contains("acquireTime"));
        assert!(!json.contains("renewTime"));
    }

    #[test]
    fn test_lease_spec_zero_values() {
        let spec = LeaseSpec {
            lease_duration_seconds: Some(0),
            lease_transitions: Some(0),
            ..Default::default()
        };
        let json = serde_json::to_string(&spec).unwrap();
        // Zero values should be present in JSON
        assert!(json.contains("\"leaseDurationSeconds\":0"));
        assert!(json.contains("\"leaseTransitions\":0"));
    }
}
