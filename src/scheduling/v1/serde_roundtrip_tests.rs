use super::{PriorityClass, PriorityClassList};
use crate::common::test_utils::assert_serde_roundtrip;
use crate::common::{ListMeta, ObjectMeta, TypeMeta};
use crate::core::internal::PreemptionPolicy;

fn priority_class_basic() -> PriorityClass {
    PriorityClass {
        type_meta: TypeMeta {
            api_version: "scheduling.k8s.io/v1".to_string(),
            kind: "PriorityClass".to_string(),
        },
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
    PriorityClassList {
        type_meta: TypeMeta {
            api_version: "scheduling.k8s.io/v1".to_string(),
            kind: "PriorityClassList".to_string(),
        },
        metadata: Some(ListMeta {
            resource_version: Some("1".to_string()),
            ..Default::default()
        }),
        items: vec![priority_class_basic()],
    }
}

#[test]
fn serde_roundtrip_priority_class() {
    assert_serde_roundtrip(&priority_class_basic());
}

#[test]
fn serde_roundtrip_priority_class_list() {
    assert_serde_roundtrip(&priority_class_list_basic());
}
