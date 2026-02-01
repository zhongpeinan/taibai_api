use crate::common::test_utils::assert_conversion_roundtrip;
use crate::common::{ApplyDefault, IntOrString, ListMeta, ObjectMeta, TypeMeta};
use crate::core::internal::{self, ServiceAffinity, ServiceType};
use crate::core::v1::{Service, ServiceList, ServicePort, ServiceSpec, ServiceStatus};
use std::collections::BTreeMap;

fn service_basic() -> Service {
    Service {
        type_meta: TypeMeta::default(),
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
            session_affinity: ServiceAffinity::None,
            ..Default::default()
        }),
        status: Some(ServiceStatus::default()),
    }
}

fn service_list_basic() -> ServiceList {
    let mut item = service_basic();
    item.apply_default();
    ServiceList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("2".to_string()),
            ..Default::default()
        }),
        items: vec![item],
    }
}

#[test]
fn conversion_roundtrip_service() {
    assert_conversion_roundtrip::<Service, internal::service::Service>(service_basic());
}

#[test]
fn conversion_roundtrip_service_list() {
    assert_conversion_roundtrip::<ServiceList, internal::service::ServiceList>(service_list_basic());
}
