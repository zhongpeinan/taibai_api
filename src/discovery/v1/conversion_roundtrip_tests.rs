use super::{
    AddressType, Endpoint, EndpointConditions, EndpointHints, EndpointPort, EndpointSlice,
    EndpointSliceList, ForNode, ForZone,
};
use crate::common::test_utils::assert_conversion_roundtrip;
use crate::common::{ApplyDefault, ListMeta, ObjectMeta, TypeMeta};
use crate::core::v1::ObjectReference;
use crate::discovery::internal;
use std::collections::BTreeMap;

fn endpoint_slice_basic() -> EndpointSlice {
    EndpointSlice {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("endpoint-slice-b".to_string()),
            namespace: Some("default".to_string()),
            labels: BTreeMap::from([("app".to_string(), "backend".to_string())]),
            ..Default::default()
        }),
        address_type: AddressType::FQDN,
        endpoints: vec![Endpoint {
            addresses: vec!["backend.example.local".to_string()],
            conditions: EndpointConditions {
                ready: Some(true),
                serving: Some(true),
                terminating: Some(false),
            },
            hostname: Some("backend-0".to_string()),
            target_ref: Some(ObjectReference {
                kind: Some("Pod".to_string()),
                namespace: Some("default".to_string()),
                name: Some("backend-0".to_string()),
                ..Default::default()
            }),
            deprecated_topology: BTreeMap::from([(
                "topology.kubernetes.io/zone".to_string(),
                "us-east-1b".to_string(),
            )]),
            node_name: Some("node-b".to_string()),
            zone: Some("us-east-1b".to_string()),
            hints: Some(EndpointHints {
                for_zones: vec![ForZone {
                    name: "us-east-1b".to_string(),
                }],
                for_nodes: vec![ForNode {
                    name: "node-b".to_string(),
                }],
            }),
        }],
        ports: vec![EndpointPort {
            name: Some("grpc".to_string()),
            protocol: Some("TCP".to_string()),
            port: Some(9090),
            app_protocol: Some("grpc".to_string()),
        }],
    }
}

fn endpoint_slice_list_basic() -> EndpointSliceList {
    let mut item = endpoint_slice_basic();
    item.apply_default();

    EndpointSliceList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("2".to_string()),
            ..Default::default()
        }),
        items: vec![item],
    }
}

#[test]
fn conversion_roundtrip_endpoint_slice() {
    assert_conversion_roundtrip::<EndpointSlice, internal::EndpointSlice>(endpoint_slice_basic());
}

#[test]
fn conversion_roundtrip_endpoint_slice_list() {
    assert_conversion_roundtrip::<EndpointSliceList, internal::EndpointSliceList>(
        endpoint_slice_list_basic(),
    );
}
