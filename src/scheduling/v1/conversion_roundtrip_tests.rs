use super::{PriorityClass, PriorityClassList};
use crate::common::test_utils::assert_conversion_roundtrip;
use crate::common::{ApplyDefault, ListMeta, ObjectMeta, TypeMeta};
use crate::core::internal::PreemptionPolicy;
use crate::scheduling::internal;

fn priority_class_basic() -> PriorityClass {
    PriorityClass {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("high-priority".to_string()),
            ..Default::default()
        }),
        value: Some(1000),
        global_default: false,
        description: "test priority class".to_string(),
        preemption_policy: Some(PreemptionPolicy::PreemptLowerPriority),
    }
}

fn priority_class_list_basic() -> PriorityClassList {
    let mut item = priority_class_basic();
    item.apply_default();
    PriorityClassList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("1".to_string()),
            ..Default::default()
        }),
        items: vec![item],
    }
}

#[test]
fn conversion_roundtrip_priority_class() {
    assert_conversion_roundtrip::<PriorityClass, internal::PriorityClass>(priority_class_basic());
}

#[test]
fn conversion_roundtrip_priority_class_list() {
    assert_conversion_roundtrip::<PriorityClassList, internal::PriorityClassList>(
        priority_class_list_basic(),
    );
}
