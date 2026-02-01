use crate::common::test_utils::assert_serde_roundtrip;
use crate::common::{ListMeta, ObjectMeta, Quantity, TypeMeta};
use crate::core::v1::{Node, NodeList, NodeSpec, NodeStatus};
use std::collections::BTreeMap;

fn node_basic() -> Node {
    Node {
        type_meta: TypeMeta {
            api_version: "v1".to_string(),
            kind: "Node".to_string(),
        },
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
            ..Default::default()
        }),
    }
}

fn node_list_basic() -> NodeList {
    NodeList {
        type_meta: TypeMeta {
            api_version: "v1".to_string(),
            kind: "NodeList".to_string(),
        },
        metadata: Some(ListMeta {
            resource_version: Some("3".to_string()),
            ..Default::default()
        }),
        items: vec![node_basic()],
    }
}

#[test]
fn serde_roundtrip_node() {
    assert_serde_roundtrip(&node_basic());
}

#[test]
fn serde_roundtrip_node_list() {
    assert_serde_roundtrip(&node_list_basic());
}
