use crate::common::test_utils::assert_conversion_roundtrip;
use crate::common::{ApplyDefault, ListMeta, ObjectMeta, TypeMeta};
use crate::core::internal;
use crate::core::v1::{
    Container, PodSpec, PodTemplateSpec, ReplicationController, ReplicationControllerList,
    ReplicationControllerSpec, ReplicationControllerStatus,
};
use std::collections::BTreeMap;

fn replication_controller_basic() -> ReplicationController {
    let labels = BTreeMap::from([("app".to_string(), "web".to_string())]);

    ReplicationController {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("web-rc".to_string()),
            namespace: Some("default".to_string()),
            labels: labels.clone(),
            ..Default::default()
        }),
        spec: Some(ReplicationControllerSpec {
            replicas: Some(2),
            min_ready_seconds: Some(5),
            selector: labels.clone(),
            template: Some(PodTemplateSpec {
                metadata: Some(ObjectMeta {
                    labels,
                    ..Default::default()
                }),
                spec: Some(PodSpec {
                    containers: vec![Container {
                        name: "app".to_string(),
                        image: Some("nginx:1.25".to_string()),
                        ..Default::default()
                    }],
                    restart_policy: Some("Always".to_string()),
                    dns_policy: Some("ClusterFirst".to_string()),
                    ..Default::default()
                }),
            }),
        }),
        status: Some(ReplicationControllerStatus {
            replicas: 1,
            ready_replicas: 1,
            ..Default::default()
        }),
    }
}

fn replication_controller_list_basic() -> ReplicationControllerList {
    let mut item = replication_controller_basic();
    item.apply_default();
    ReplicationControllerList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("11".to_string()),
            ..Default::default()
        }),
        items: vec![item],
    }
}

#[test]
fn conversion_roundtrip_replication_controller() {
    assert_conversion_roundtrip::<
        ReplicationController,
        internal::replication_controller::ReplicationController,
    >(replication_controller_basic());
}

#[test]
fn conversion_roundtrip_replication_controller_list() {
    assert_conversion_roundtrip::<
        ReplicationControllerList,
        internal::replication_controller::ReplicationControllerList,
    >(replication_controller_list_basic());
}
