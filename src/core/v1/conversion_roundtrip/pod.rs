use crate::common::test_utils::assert_conversion_roundtrip;
use crate::common::{ApplyDefault, ListMeta, ObjectMeta, TypeMeta};
use crate::core::internal;
use crate::core::v1::{Container, ContainerPort, Pod, PodList, PodSpec, PodStatus};
use std::collections::BTreeMap;

fn pod_basic() -> Pod {
    Pod {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("demo-pod".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        spec: Some(PodSpec {
            containers: vec![Container {
                name: "app".to_string(),
                image: Some("nginx:1.25".to_string()),
                ports: vec![ContainerPort {
                    name: Some("http".to_string()),
                    container_port: 80,
                    protocol: Some("TCP".to_string()),
                    host_port: None,
                    host_ip: None,
                }],
                ..Default::default()
            }],
            service_account_name: Some("default".to_string()),
            deprecated_service_account: Some("default".to_string()),
            node_selector: BTreeMap::from([("disk".to_string(), "ssd".to_string())]),
            ..Default::default()
        }),
        status: Some(PodStatus {
            phase: Some("Pending".to_string()),
            ..Default::default()
        }),
    }
}

fn pod_list_basic() -> PodList {
    let mut item = pod_basic();
    item.apply_default();
    PodList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("1".to_string()),
            ..Default::default()
        }),
        items: vec![item],
    }
}

#[test]
fn conversion_roundtrip_pod() {
    assert_conversion_roundtrip::<Pod, internal::pod::Pod>(pod_basic());
}

#[test]
fn conversion_roundtrip_pod_list() {
    assert_conversion_roundtrip::<PodList, internal::pod::PodList>(pod_list_basic());
}
