use super::{
    AddressType, Endpoint, EndpointConditions, EndpointHints, EndpointPort, EndpointSlice,
    EndpointSliceList, ForNode, ForZone,
};
use crate::common::test_utils::assert_serde_roundtrip;
use crate::common::{ListMeta, ObjectMeta, TypeMeta};
use crate::core::v1::ObjectReference;
use std::collections::BTreeMap;

fn endpoint_slice_basic() -> EndpointSlice {
    EndpointSlice {
        type_meta: TypeMeta {
            api_version: "discovery.k8s.io/v1".to_string(),
            kind: "EndpointSlice".to_string(),
        },
        metadata: Some(ObjectMeta {
            name: Some("endpoint-slice-a".to_string()),
            namespace: Some("default".to_string()),
            labels: BTreeMap::from([(
                "discovery.k8s.io/service-name".to_string(),
                "example".to_string(),
            )]),
            ..Default::default()
        }),
        address_type: AddressType::IPv6,
        endpoints: vec![Endpoint {
            addresses: vec!["fd00::1".to_string()],
            conditions: EndpointConditions {
                ready: Some(true),
                serving: Some(true),
                terminating: Some(false),
            },
            hostname: Some("node-a".to_string()),
            target_ref: Some(ObjectReference {
                kind: Some("Pod".to_string()),
                namespace: Some("default".to_string()),
                name: Some("pod-a".to_string()),
                ..Default::default()
            }),
            deprecated_topology: BTreeMap::from([(
                "topology.kubernetes.io/zone".to_string(),
                "us-east-1a".to_string(),
            )]),
            node_name: Some("node-a".to_string()),
            zone: Some("us-east-1a".to_string()),
            hints: Some(EndpointHints {
                for_zones: vec![ForZone {
                    name: "us-east-1a".to_string(),
                }],
                for_nodes: vec![ForNode {
                    name: "node-a".to_string(),
                }],
            }),
        }],
        ports: vec![EndpointPort {
            name: Some("http".to_string()),
            protocol: Some("TCP".to_string()),
            port: Some(80),
            app_protocol: Some("http".to_string()),
        }],
    }
}

fn endpoint_slice_list_basic() -> EndpointSliceList {
    EndpointSliceList {
        type_meta: TypeMeta {
            api_version: "discovery.k8s.io/v1".to_string(),
            kind: "EndpointSliceList".to_string(),
        },
        metadata: Some(ListMeta {
            resource_version: Some("1".to_string()),
            ..Default::default()
        }),
        items: vec![endpoint_slice_basic()],
    }
}

#[test]
fn serde_roundtrip_endpoint_slice() {
    assert_serde_roundtrip(&endpoint_slice_basic());
}

#[test]
fn serde_roundtrip_endpoint_slice_list() {
    assert_serde_roundtrip(&endpoint_slice_list_basic());
}
