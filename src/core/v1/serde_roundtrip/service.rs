use crate::common::test_utils::assert_serde_roundtrip;
use crate::common::{IntOrString, ListMeta, ObjectMeta, TypeMeta};
use crate::core::internal::{ServiceAffinity, ServiceType};
use crate::core::v1::{Service, ServiceList, ServicePort, ServiceSpec};
use std::collections::BTreeMap;

fn service_basic() -> Service {
    Service {
        type_meta: TypeMeta {
            api_version: "v1".to_string(),
            kind: "Service".to_string(),
        },
        metadata: Some(ObjectMeta {
            name: Some("web".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        spec: Some(ServiceSpec {
            ports: vec![ServicePort {
                name: "http".to_string(),
                protocol: "TCP".to_string(),
                app_protocol: None,
                port: 80,
                target_port: Some(IntOrString::Int(8080)),
                node_port: None,
            }],
            selector: BTreeMap::from([("app".to_string(), "web".to_string())]),
            type_: Some(ServiceType::ClusterIp),
            session_affinity: Some(ServiceAffinity::None),
            ..Default::default()
        }),
        status: None,
    }
}

fn service_list_basic() -> ServiceList {
    ServiceList {
        type_meta: TypeMeta {
            api_version: "v1".to_string(),
            kind: "ServiceList".to_string(),
        },
        metadata: Some(ListMeta {
            resource_version: Some("2".to_string()),
            ..Default::default()
        }),
        items: vec![service_basic()],
    }
}

#[test]
fn serde_roundtrip_service() {
    assert_serde_roundtrip(&service_basic());
}

#[test]
fn serde_roundtrip_service_list() {
    assert_serde_roundtrip(&service_list_basic());
}
