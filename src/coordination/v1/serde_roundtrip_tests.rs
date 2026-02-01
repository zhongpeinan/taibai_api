use super::{Lease, LeaseList, LeaseSpec};
use crate::common::test_utils::assert_serde_roundtrip;
use crate::common::{ListMeta, MicroTime, ObjectMeta, TypeMeta};

fn lease_basic() -> Lease {
    Lease {
        type_meta: TypeMeta {
            api_version: "coordination.k8s.io/v1".to_string(),
            kind: "Lease".to_string(),
        },
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
    LeaseList {
        type_meta: TypeMeta {
            api_version: "coordination.k8s.io/v1".to_string(),
            kind: "LeaseList".to_string(),
        },
        metadata: Some(ListMeta {
            resource_version: Some("1".to_string()),
            ..Default::default()
        }),
        items: vec![lease_basic()],
    }
}

#[test]
fn serde_roundtrip_lease() {
    assert_serde_roundtrip(&lease_basic());
}

#[test]
fn serde_roundtrip_lease_list() {
    assert_serde_roundtrip(&lease_list_basic());
}
