use crate::common::test_utils::assert_serde_roundtrip;
use crate::common::{ListMeta, ObjectMeta, TypeMeta};
use crate::core::v1::{Container, ContainerPort, Pod, PodList, PodSpec};
use std::collections::BTreeMap;

fn pod_basic() -> Pod {
    Pod {
        type_meta: TypeMeta {
            api_version: "v1".to_string(),
            kind: "Pod".to_string(),
        },
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
            node_selector: BTreeMap::from([("disk".to_string(), "ssd".to_string())]),
            ..Default::default()
        }),
        status: None,
    }
}

fn pod_list_basic() -> PodList {
    PodList {
        type_meta: TypeMeta {
            api_version: "v1".to_string(),
            kind: "PodList".to_string(),
        },
        metadata: Some(ListMeta {
            resource_version: Some("1".to_string()),
            ..Default::default()
        }),
        items: vec![pod_basic()],
    }
}

#[test]
fn serde_roundtrip_pod() {
    assert_serde_roundtrip(&pod_basic());
}

#[test]
fn serde_roundtrip_pod_list() {
    assert_serde_roundtrip(&pod_list_basic());
}
