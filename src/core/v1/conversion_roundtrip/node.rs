use crate::common::test_utils::assert_conversion_roundtrip;
use crate::common::{ApplyDefault, ListMeta, ObjectMeta, Quantity, TypeMeta};
use crate::core::internal;
use crate::core::v1::{Node, NodeDaemonEndpoints, NodeList, NodeSpec, NodeStatus, NodeSystemInfo};
use std::collections::BTreeMap;

fn node_basic() -> Node {
    Node {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("node-a".to_string()),
            ..Default::default()
        }),
        spec: Some(NodeSpec {
            pod_cidr: Some("10.0.0.0/24".to_string()),
            provider_id: Some("aws:///i-1234567890".to_string()),
            ..Default::default()
        }),
        status: Some(NodeStatus {
            capacity: BTreeMap::from([
                ("cpu".to_string(), Quantity("4".to_string())),
                ("memory".to_string(), Quantity("16Gi".to_string())),
            ]),
            phase: Some("Pending".to_string()),
            daemon_endpoints: Some(NodeDaemonEndpoints::default()),
            node_info: Some(NodeSystemInfo::default()),
            ..Default::default()
        }),
    }
}

fn node_list_basic() -> NodeList {
    let mut item = node_basic();
    item.apply_default();
    NodeList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("3".to_string()),
            ..Default::default()
        }),
        items: vec![item],
    }
}

#[test]
fn conversion_roundtrip_node() {
    assert_conversion_roundtrip::<Node, internal::node::Node>(node_basic());
}

#[test]
fn conversion_roundtrip_node_list() {
    assert_conversion_roundtrip::<NodeList, internal::node::NodeList>(node_list_basic());
}
