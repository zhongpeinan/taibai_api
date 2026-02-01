use super::{Lease, LeaseList, LeaseSpec};
use crate::common::test_utils::assert_conversion_roundtrip;
use crate::common::{ApplyDefault, ListMeta, MicroTime, ObjectMeta, TypeMeta};
use crate::coordination::internal;

fn lease_basic() -> Lease {
    Lease {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("lease-a".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        spec: Some(LeaseSpec {
            holder_identity: Some("holder-1".to_string()),
            lease_duration_seconds: Some(15),
            acquire_time: Some(
                MicroTime::from_str("2024-01-15T10:00:00.123456Z").expect("parse microtime"),
            ),
            renew_time: Some(
                MicroTime::from_str("2024-01-15T10:00:01.123456Z").expect("parse microtime"),
            ),
            lease_transitions: Some(2),
            ..Default::default()
        }),
    }
}

fn lease_list_basic() -> LeaseList {
    let mut item = lease_basic();
    item.apply_default();
    LeaseList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("1".to_string()),
            ..Default::default()
        }),
        items: vec![item],
    }
}

#[test]
fn conversion_roundtrip_lease() {
    assert_conversion_roundtrip::<Lease, internal::Lease>(lease_basic());
}

#[test]
fn conversion_roundtrip_lease_list() {
    assert_conversion_roundtrip::<LeaseList, internal::LeaseList>(lease_list_basic());
}
