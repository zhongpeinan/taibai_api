use super::{
    ReplicaSet, ReplicaSetCondition, ReplicaSetConditionType, ReplicaSetList, ReplicaSetSpec,
    ReplicaSetStatus,
};
use crate::apps::internal;
use crate::common::test_utils::assert_conversion_roundtrip;
use crate::common::{ApplyDefault, LabelSelector, ListMeta, ObjectMeta, TypeMeta};
use crate::core::v1::{PodSpec, PodTemplateSpec};

fn replica_set_basic() -> ReplicaSet {
    let mut selector = LabelSelector::default();
    selector
        .match_labels
        .insert("app".to_string(), "demo".to_string());

    ReplicaSet {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("demo-replicaset".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        spec: Some(ReplicaSetSpec {
            replicas: Some(3),
            selector: Some(selector.clone()),
            template: Some(PodTemplateSpec {
                metadata: Some(ObjectMeta {
                    labels: selector.match_labels.clone(),
                    ..Default::default()
                }),
                spec: Some(PodSpec {
                    restart_policy: Some("Always".to_string()),
                    dns_policy: Some("ClusterFirst".to_string()),
                    ..Default::default()
                }),
            }),
            min_ready_seconds: Some(10),
        }),
        status: Some(ReplicaSetStatus {
            replicas: 3,
            fully_labeled_replicas: Some(3),
            ready_replicas: Some(2),
            available_replicas: Some(2),
            terminating_replicas: Some(0),
            observed_generation: Some(2),
            conditions: vec![ReplicaSetCondition {
                r#type: ReplicaSetConditionType::ReplicaFailure,
                status: "True".to_string(),
                last_transition_time: Some("2024-01-01T00:00:00Z".to_string()),
                reason: "ReplicaFailure".to_string(),
                message: "replica failed".to_string(),
            }],
        }),
    }
}

fn replica_set_list_basic() -> ReplicaSetList {
    let mut item = replica_set_basic();
    item.apply_default();

    ReplicaSetList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("1".to_string()),
            ..Default::default()
        }),
        items: vec![item],
    }
}

#[test]
fn conversion_roundtrip_replica_set() {
    assert_conversion_roundtrip::<ReplicaSet, internal::ReplicaSet>(replica_set_basic());
}

#[test]
fn conversion_roundtrip_replica_set_list() {
    assert_conversion_roundtrip::<ReplicaSetList, internal::ReplicaSetList>(
        replica_set_list_basic(),
    );
}
