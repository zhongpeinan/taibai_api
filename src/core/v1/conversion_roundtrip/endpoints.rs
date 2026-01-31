use crate::common::test_utils::assert_conversion_roundtrip;
use crate::common::{ApplyDefault, ListMeta, ObjectMeta, TypeMeta};
use crate::core::internal;
use crate::core::v1::reference::ObjectReference;
use crate::core::v1::{EndpointAddress, EndpointPort, EndpointSubset, Endpoints, EndpointsList};

fn endpoints_basic() -> Endpoints {
    Endpoints {
        type_meta: TypeMeta::default(),
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
    let mut item = endpoints_basic();
    item.apply_default();
    EndpointsList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("10".to_string()),
            ..Default::default()
        }),
        items: vec![item],
    }
}

#[test]
fn conversion_roundtrip_endpoints() {
    assert_conversion_roundtrip::<Endpoints, internal::endpoints::Endpoints>(endpoints_basic());
}

#[test]
fn conversion_roundtrip_endpoints_list() {
    assert_conversion_roundtrip::<EndpointsList, internal::endpoints::EndpointsList>(
        endpoints_list_basic(),
    );
}
