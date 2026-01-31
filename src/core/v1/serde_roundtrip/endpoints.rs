use crate::common::test_utils::assert_serde_roundtrip;
use crate::common::{ListMeta, ObjectMeta, TypeMeta};
use crate::core::v1::reference::ObjectReference;
use crate::core::v1::{EndpointAddress, EndpointPort, EndpointSubset, Endpoints, EndpointsList};

fn endpoints_basic() -> Endpoints {
    Endpoints {
        type_meta: TypeMeta {
            api_version: "v1".to_string(),
            kind: "Endpoints".to_string(),
        },
        metadata: Some(ObjectMeta {
            name: Some("web".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        subsets: vec![EndpointSubset {
            addresses: vec![EndpointAddress {
                ip: "10.0.0.10".to_string(),
                hostname: "web-0".to_string(),
                node_name: Some("node-a".to_string()),
                target_ref: Some(ObjectReference {
                    kind: Some("Pod".to_string()),
                    name: Some("web-0".to_string()),
                    namespace: Some("default".to_string()),
                    ..Default::default()
                }),
            }],
            ports: vec![EndpointPort {
                name: "http".to_string(),
                port: 80,
                protocol: "TCP".to_string(),
                app_protocol: Some("http".to_string()),
            }],
            ..Default::default()
        }],
    }
}

fn endpoints_list_basic() -> EndpointsList {
    EndpointsList {
        type_meta: TypeMeta {
            api_version: "v1".to_string(),
            kind: "EndpointsList".to_string(),
        },
        metadata: Some(ListMeta {
            resource_version: Some("10".to_string()),
            ..Default::default()
        }),
        items: vec![endpoints_basic()],
    }
}

#[test]
fn serde_roundtrip_endpoints() {
    assert_serde_roundtrip(&endpoints_basic());
}

#[test]
fn serde_roundtrip_endpoints_list() {
    assert_serde_roundtrip(&endpoints_list_basic());
}
