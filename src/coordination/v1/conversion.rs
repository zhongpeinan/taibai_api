//! Conversions between v1 and internal coordination types

use crate::common::{ApplyDefault, FromInternal, ListMeta, ObjectMeta, ToInternal, TypeMeta};
use crate::coordination::internal;

use super::{Lease, LeaseList, LeaseSpec};

// ============================================================================
// Conversion Helper Functions
// ============================================================================

fn is_empty_object_meta(meta: &ObjectMeta) -> bool {
    meta.name.is_none()
        && meta.generate_name.is_none()
        && meta.namespace.is_none()
        && meta.uid.is_none()
        && meta.resource_version.is_none()
        && meta.generation.is_none()
        && meta.self_link.is_none()
        && meta.labels.is_empty()
        && meta.annotations.is_empty()
        && meta.owner_references.is_empty()
        && meta.finalizers.is_empty()
        && meta.managed_fields.is_empty()
        && meta.creation_timestamp.is_none()
        && meta.deletion_timestamp.is_none()
        && meta.deletion_grace_period_seconds.is_none()
}

fn is_empty_list_meta(meta: &ListMeta) -> bool {
    meta.continue_.is_none()
        && meta.remaining_item_count.is_none()
        && meta.resource_version.is_none()
        && meta.self_link.is_none()
}

fn is_empty_lease_spec(spec: &LeaseSpec) -> bool {
    spec.holder_identity.is_none()
        && spec.lease_duration_seconds.is_none()
        && spec.acquire_time.is_none()
        && spec.renew_time.is_none()
        && spec.lease_transitions.is_none()
        && spec.strategy.is_none()
        && spec.preferred_holder.is_none()
}

fn option_object_meta_to_meta(meta: Option<ObjectMeta>) -> ObjectMeta {
    meta.unwrap_or_default()
}

fn meta_to_option_object_meta(meta: ObjectMeta) -> Option<ObjectMeta> {
    if is_empty_object_meta(&meta) {
        None
    } else {
        Some(meta)
    }
}

fn option_list_meta_to_meta(meta: Option<ListMeta>) -> ListMeta {
    meta.unwrap_or_default()
}

fn meta_to_option_list_meta(meta: ListMeta) -> Option<ListMeta> {
    if is_empty_list_meta(&meta) {
        None
    } else {
        Some(meta)
    }
}

fn option_lease_spec_to_spec(spec: Option<LeaseSpec>) -> internal::LeaseSpec {
    spec.map(v1_lease_spec_to_internal).unwrap_or_default()
}

fn lease_spec_to_option_spec(spec: internal::LeaseSpec) -> Option<LeaseSpec> {
    let v1_spec = internal_lease_spec_to_v1(spec);
    if is_empty_lease_spec(&v1_spec) {
        None
    } else {
        Some(v1_spec)
    }
}

fn v1_lease_spec_to_internal(spec: LeaseSpec) -> internal::LeaseSpec {
    internal::LeaseSpec {
        holder_identity: spec.holder_identity,
        lease_duration_seconds: spec.lease_duration_seconds,
        acquire_time: spec.acquire_time,
        renew_time: spec.renew_time,
        lease_transitions: spec.lease_transitions,
        strategy: spec.strategy,
        preferred_holder: spec.preferred_holder,
    }
}

fn internal_lease_spec_to_v1(spec: internal::LeaseSpec) -> LeaseSpec {
    LeaseSpec {
        holder_identity: spec.holder_identity,
        lease_duration_seconds: spec.lease_duration_seconds,
        acquire_time: spec.acquire_time,
        renew_time: spec.renew_time,
        lease_transitions: spec.lease_transitions,
        strategy: spec.strategy,
        preferred_holder: spec.preferred_holder,
    }
}

// ============================================================================
// Lease Conversions
// ============================================================================

impl ToInternal<internal::Lease> for Lease {
    fn to_internal(self) -> internal::Lease {
        internal::Lease {
            type_meta: TypeMeta::default(),
            metadata: option_object_meta_to_meta(self.metadata),
            spec: option_lease_spec_to_spec(self.spec),
        }
    }
}

impl FromInternal<internal::Lease> for Lease {
    fn from_internal(value: internal::Lease) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: meta_to_option_object_meta(value.metadata),
            spec: lease_spec_to_option_spec(value.spec),
        };
        result.apply_default();
        result
    }
}

// ============================================================================
// LeaseList Conversions
// ============================================================================

impl ToInternal<internal::LeaseList> for LeaseList {
    fn to_internal(self) -> internal::LeaseList {
        internal::LeaseList {
            type_meta: TypeMeta::default(),
            metadata: option_list_meta_to_meta(self.metadata),
            items: self.items.into_iter().map(Lease::to_internal).collect(),
        }
    }
}

impl FromInternal<internal::LeaseList> for LeaseList {
    fn from_internal(value: internal::LeaseList) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: meta_to_option_list_meta(value.metadata),
            items: value.items.into_iter().map(Lease::from_internal).collect(),
        };
        result.apply_default();
        result
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lease_round_trip() {
        let v1 = Lease {
            type_meta: TypeMeta {
                api_version: "coordination.k8s.io/v1".to_string(),
                kind: "Lease".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("lease-a".to_string()),
                ..Default::default()
            }),
            spec: Some(LeaseSpec {
                holder_identity: Some("holder".to_string()),
                lease_duration_seconds: Some(15),
                ..Default::default()
            }),
        };

        let internal = v1.clone().to_internal();
        let back = Lease::from_internal(internal);

        assert_eq!(
            back.metadata.as_ref().and_then(|m| m.name.clone()),
            Some("lease-a".to_string())
        );
        assert_eq!(
            back.spec.as_ref().and_then(|s| s.holder_identity.clone()),
            Some("holder".to_string())
        );
        assert_eq!(back.type_meta.api_version, "coordination.k8s.io/v1");
        assert_eq!(back.type_meta.kind, "Lease");
    }

    #[test]
    fn test_lease_list_round_trip() {
        let v1 = LeaseList {
            type_meta: TypeMeta {
                api_version: "coordination.k8s.io/v1".to_string(),
                kind: "LeaseList".to_string(),
            },
            metadata: None,
            items: vec![Lease {
                type_meta: TypeMeta::default(),
                metadata: Some(ObjectMeta {
                    name: Some("lease-a".to_string()),
                    ..Default::default()
                }),
                spec: None,
            }],
        };

        let internal = v1.clone().to_internal();
        let back = LeaseList::from_internal(internal);

        assert_eq!(back.items.len(), 1);
        assert_eq!(
            back.items[0].metadata.as_ref().and_then(|m| m.name.clone()),
            Some("lease-a".to_string())
        );
        assert_eq!(back.type_meta.api_version, "coordination.k8s.io/v1");
        assert_eq!(back.type_meta.kind, "LeaseList");
    }

    #[test]
    fn test_lease_spec_empty_round_trip() {
        let v1 = Lease {
            type_meta: TypeMeta::default(),
            metadata: None,
            spec: None,
        };

        let internal = v1.to_internal();
        let back = Lease::from_internal(internal);

        assert!(back.spec.is_none());
    }
}
