use super::{Overhead, RuntimeClass, RuntimeClassList, Scheduling};
use crate::common::test_utils::assert_conversion_roundtrip;
use crate::common::util::Quantity;
use crate::common::{ApplyDefault, ListMeta, ObjectMeta, TypeMeta};
use crate::core::v1::Toleration;
use crate::node::internal;
use std::collections::BTreeMap;

fn runtime_class_basic() -> RuntimeClass {
    let mut overhead = Overhead::default();
    overhead
        .pod_fixed
        .insert("memory".to_string(), Quantity("256Mi".to_string()));

    RuntimeClass {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("kata".to_string()),
            annotations: BTreeMap::from([("runtime".to_string(), "kata".to_string())]),
            ..Default::default()
        }),
        handler: "kata".to_string(),
        overhead: Some(overhead),
        scheduling: Some(Scheduling {
            node_selector: BTreeMap::from([(
                "node.kubernetes.io/instance-type".to_string(),
                "m5".to_string(),
            )]),
            tolerations: vec![Toleration {
                key: "runtime".to_string(),
                operator: "Exists".to_string(),
                value: "".to_string(),
                effect: "PreferNoSchedule".to_string(),
                toleration_seconds: Some(30),
            }],
        }),
    }
}

fn runtime_class_list_basic() -> RuntimeClassList {
    let mut item = runtime_class_basic();
    item.apply_default();

    RuntimeClassList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("2".to_string()),
            ..Default::default()
        }),
        items: vec![item],
    }
}

#[test]
fn conversion_roundtrip_runtime_class() {
    assert_conversion_roundtrip::<RuntimeClass, internal::RuntimeClass>(runtime_class_basic());
}

#[test]
fn conversion_roundtrip_runtime_class_list() {
    assert_conversion_roundtrip::<RuntimeClassList, internal::RuntimeClassList>(
        runtime_class_list_basic(),
    );
}
