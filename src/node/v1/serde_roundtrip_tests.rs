use super::{Overhead, RuntimeClass, RuntimeClassList, Scheduling};
use crate::common::test_utils::assert_serde_roundtrip;
use crate::common::util::Quantity;
use crate::common::{ListMeta, ObjectMeta, TypeMeta};
use crate::core::v1::Toleration;
use std::collections::BTreeMap;

fn runtime_class_basic() -> RuntimeClass {
    let mut overhead = Overhead::default();
    overhead
        .pod_fixed
        .insert("cpu".to_string(), Quantity("100m".to_string()));

    RuntimeClass {
        type_meta: TypeMeta {
            api_version: "node.k8s.io/v1".to_string(),
            kind: "RuntimeClass".to_string(),
        },
        metadata: Some(ObjectMeta {
            name: Some("runc".to_string()),
            labels: BTreeMap::from([("tier".to_string(), "node".to_string())]),
            ..Default::default()
        }),
        handler: "runc".to_string(),
        overhead: Some(overhead),
        scheduling: Some(Scheduling {
            node_selector: BTreeMap::from([("kubernetes.io/os".to_string(), "linux".to_string())]),
            tolerations: vec![Toleration {
                key: "workload".to_string(),
                operator: "Equal".to_string(),
                value: "batch".to_string(),
                effect: "NoSchedule".to_string(),
                toleration_seconds: None,
            }],
        }),
    }
}

fn runtime_class_list_basic() -> RuntimeClassList {
    RuntimeClassList {
        type_meta: TypeMeta {
            api_version: "node.k8s.io/v1".to_string(),
            kind: "RuntimeClassList".to_string(),
        },
        metadata: Some(ListMeta {
            resource_version: Some("1".to_string()),
            ..Default::default()
        }),
        items: vec![runtime_class_basic()],
    }
}

#[test]
fn serde_roundtrip_runtime_class() {
    assert_serde_roundtrip(&runtime_class_basic());
}

#[test]
fn serde_roundtrip_runtime_class_list() {
    assert_serde_roundtrip(&runtime_class_list_basic());
}
