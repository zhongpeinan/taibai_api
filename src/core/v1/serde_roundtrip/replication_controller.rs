use crate::common::test_utils::assert_serde_roundtrip;
use crate::common::{ListMeta, ObjectMeta, TypeMeta};
use crate::core::v1::{
    Container, PodSpec, PodTemplateSpec, ReplicationController, ReplicationControllerList,
    ReplicationControllerSpec, ReplicationControllerStatus,
};
use std::collections::BTreeMap;

fn replication_controller_basic() -> ReplicationController {
    let labels = BTreeMap::from([("app".to_string(), "web".to_string())]);

    ReplicationController {
        type_meta: TypeMeta {
            api_version: "v1".to_string(),
            kind: "ReplicationController".to_string(),
        },
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
    ReplicationControllerList {
        type_meta: TypeMeta {
            api_version: "v1".to_string(),
            kind: "ReplicationControllerList".to_string(),
        },
        metadata: Some(ListMeta {
            resource_version: Some("11".to_string()),
            ..Default::default()
        }),
        items: vec![replication_controller_basic()],
    }
}

#[test]
fn serde_roundtrip_replication_controller() {
    assert_serde_roundtrip(&replication_controller_basic());
}

#[test]
fn serde_roundtrip_replication_controller_list() {
    assert_serde_roundtrip(&replication_controller_list_basic());
}
