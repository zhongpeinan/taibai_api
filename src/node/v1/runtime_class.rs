//! RuntimeClass types from the Kubernetes Node v1 API
//!
//! This module contains types for defining container runtime classes.

use crate::common::{ListMeta, ObjectMeta};
use crate::core::v1::{ResourceList, Toleration};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// RuntimeClass defines a class of container runtime supported in the cluster.
///
/// The RuntimeClass is used to determine which container runtime is used to run
/// all containers in a pod. RuntimeClasses are manually defined by a
/// user or cluster provisioner, and referenced in the PodSpec. The Kubelet is
/// responsible for resolving the RuntimeClassName reference before running the
/// pod. For more details, see
/// https://kubernetes.io/docs/concepts/containers/runtime-class/
///
/// Corresponds to [Kubernetes RuntimeClass](https://github.com/kubernetes/api/blob/master/node/v1/types.go#L29)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeClass {
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// Handler specifies the underlying runtime and configuration that the CRI
    /// implementation will use to handle pods of this class.
    ///
    /// The possible values are specific to the node & CRI configuration. It is
    /// assumed that all handlers are available on every node, and handlers of
    /// the same name are equivalent on every node.
    ///
    /// For example, a handler called "runc" might specify that the runc OCI
    /// runtime (using native Linux containers) will be used to run the containers
    /// in a pod.
    ///
    /// The Handler must be lowercase, conform to the DNS Label (RFC 1123) requirements,
    /// and is immutable.
    pub handler: String,

    /// Overhead represents the resource overhead associated with running a pod for a
    /// given RuntimeClass.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overhead: Option<Overhead>,

    /// Scheduling holds the scheduling constraints to ensure that pods running
    /// with this RuntimeClass are scheduled to nodes that support it.
    ///
    /// If scheduling is nil, this RuntimeClass is assumed to be supported by all
    /// nodes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheduling: Option<Scheduling>,
}

/// RuntimeClassList is a list of RuntimeClass objects.
///
/// Corresponds to [Kubernetes RuntimeClassList](https://github.com/kubernetes/api/blob/master/node/v1/types.go#L103)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeClassList {
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// Items is a list of schema objects.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<RuntimeClass>,
}

/// Overhead structure represents the resource overhead associated with running a pod.
///
/// Corresponds to [Kubernetes Overhead](https://github.com/kubernetes/api/blob/master/node/v1/types.go#L73)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Overhead {
    /// PodFixed represents the fixed resource overhead associated with running a pod.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub pod_fixed: ResourceList,
}

/// Scheduling specifies the scheduling constraints for nodes supporting a
/// RuntimeClass.
///
/// Corresponds to [Kubernetes Scheduling](https://github.com/kubernetes/api/blob/master/node/v1/types.go#L80)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Scheduling {
    /// NodeSelector lists labels that must be present on nodes that support this
    /// RuntimeClass. Pods using this RuntimeClass can only be scheduled to a
    /// node matched by this selector. The RuntimeClass nodeSelector is merged
    /// with a pod's existing nodeSelector. Any conflicts will cause the pod to
    /// be rejected in admission.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub node_selector: BTreeMap<String, String>,

    /// Tolerations are appended (excluding duplicates) to pods running with this
    /// RuntimeClass during admission, effectively unioning the set of nodes
    /// tolerated by the pod and the RuntimeClass.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tolerations: Vec<Toleration>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::util::Quantity;

    #[test]
    fn test_runtime_class_default_fields() {
        let rc = RuntimeClass {
            metadata: None,
            handler: "runc".to_string(),
            overhead: None,
            scheduling: None,
        };
        assert_eq!(rc.handler, "runc");
        assert!(rc.metadata.is_none());
        assert!(rc.overhead.is_none());
        assert!(rc.scheduling.is_none());
    }

    #[test]
    fn test_runtime_class_with_metadata() {
        let metadata = ObjectMeta {
            name: Some("my-runtime-class".to_string()),
            ..Default::default()
        };
        let rc = RuntimeClass {
            metadata: Some(metadata),
            handler: "runc".to_string(),
            overhead: None,
            scheduling: None,
        };
        assert_eq!(
            rc.metadata.as_ref().unwrap().name,
            Some("my-runtime-class".to_string())
        );
    }

    #[test]
    fn test_runtime_class_with_overhead() {
        let mut resource_list = ResourceList::new();
        resource_list.insert("cpu".to_string(), Quantity::from("100m"));
        resource_list.insert("memory".to_string(), Quantity::from("100Mi"));

        let overhead = Overhead {
            pod_fixed: resource_list,
        };

        let rc = RuntimeClass {
            metadata: None,
            handler: "runc".to_string(),
            overhead: Some(overhead),
            scheduling: None,
        };
        assert!(rc.overhead.is_some());
        assert_eq!(rc.overhead.as_ref().unwrap().pod_fixed.len(), 2);
    }

    #[test]
    fn test_runtime_class_with_scheduling() {
        let mut node_selector = BTreeMap::new();
        node_selector.insert("node-role.kubernetes.io/worker".to_string(), "".to_string());

        let scheduling = Scheduling {
            node_selector,
            tolerations: vec![],
        };

        let rc = RuntimeClass {
            metadata: None,
            handler: "runc".to_string(),
            overhead: None,
            scheduling: Some(scheduling),
        };
        assert!(rc.scheduling.is_some());
        assert_eq!(
            rc.scheduling
                .as_ref()
                .unwrap()
                .node_selector
                .get("node-role.kubernetes.io/worker")
                .unwrap(),
            ""
        );
    }

    #[test]
    fn test_runtime_class_serialize() {
        let rc = RuntimeClass {
            metadata: Some(ObjectMeta {
                name: Some("my-runtime-class".to_string()),
                ..Default::default()
            }),
            handler: "runc".to_string(),
            overhead: None,
            scheduling: None,
        };

        let json = serde_json::to_string(&rc).unwrap();
        assert!(json.contains("\"name\":\"my-runtime-class\""));
        assert!(json.contains("\"handler\":\"runc\""));
    }

    #[test]
    fn test_runtime_class_deserialize() {
        let json = r#"{
            "metadata": {"name": "my-runtime-class"},
            "handler": "runc"
        }"#;
        let rc: RuntimeClass = serde_json::from_str(json).unwrap();
        assert_eq!(rc.handler, "runc");
        assert_eq!(
            rc.metadata.as_ref().unwrap().name,
            Some("my-runtime-class".to_string())
        );
    }

    #[test]
    fn test_runtime_class_round_trip() {
        let original = RuntimeClass {
            metadata: Some(ObjectMeta {
                name: Some("test-class".to_string()),
                ..Default::default()
            }),
            handler: "gvisor".to_string(),
            overhead: Some(Overhead {
                pod_fixed: {
                    let mut map = ResourceList::new();
                    map.insert("cpu".to_string(), Quantity::from("100m"));
                    map
                },
            }),
            scheduling: Some(Scheduling {
                node_selector: {
                    let mut map = BTreeMap::new();
                    map.insert("key".to_string(), "value".to_string());
                    map
                },
                tolerations: vec![],
            }),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: RuntimeClass = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_runtime_class_list_default() {
        let list = RuntimeClassList {
            metadata: None,
            items: vec![],
        };
        assert!(list.metadata.is_none());
        assert!(list.items.is_empty());
    }

    #[test]
    fn test_runtime_class_list_with_items() {
        let rc1 = RuntimeClass {
            metadata: Some(ObjectMeta {
                name: Some("runc".to_string()),
                ..Default::default()
            }),
            handler: "runc".to_string(),
            overhead: None,
            scheduling: None,
        };
        let rc2 = RuntimeClass {
            metadata: Some(ObjectMeta {
                name: Some("gvisor".to_string()),
                ..Default::default()
            }),
            handler: "gvisor".to_string(),
            overhead: None,
            scheduling: None,
        };

        let list = RuntimeClassList {
            metadata: Some(ListMeta {
                resource_version: Some("12345".to_string()),
                ..Default::default()
            }),
            items: vec![rc1, rc2],
        };
        assert_eq!(list.items.len(), 2);
        assert_eq!(
            list.metadata.as_ref().unwrap().resource_version,
            Some("12345".to_string())
        );
    }

    #[test]
    fn test_runtime_class_list_serialize() {
        let list = RuntimeClassList {
            metadata: Some(ListMeta {
                resource_version: Some("12345".to_string()),
                ..Default::default()
            }),
            items: vec![RuntimeClass {
                metadata: Some(ObjectMeta {
                    name: Some("runc".to_string()),
                    ..Default::default()
                }),
                handler: "runc".to_string(),
                overhead: None,
                scheduling: None,
            }],
        };
        let json = serde_json::to_string(&list).unwrap();
        assert!(json.contains("\"resourceVersion\":\"12345\""));
        assert!(json.contains("\"name\":\"runc\""));
    }

    #[test]
    fn test_overhead_default() {
        let overhead = Overhead::default();
        assert!(overhead.pod_fixed.is_empty());
    }

    #[test]
    fn test_overhead_with_resources() {
        let mut resources = ResourceList::new();
        resources.insert("cpu".to_string(), Quantity::from("200m"));
        resources.insert("memory".to_string(), Quantity::from("200Mi"));

        let overhead = Overhead {
            pod_fixed: resources.clone(),
        };
        assert_eq!(overhead.pod_fixed.len(), 2);
        assert_eq!(overhead.pod_fixed.get("cpu"), Some(&Quantity::from("200m")));
        assert_eq!(
            overhead.pod_fixed.get("memory"),
            Some(&Quantity::from("200Mi"))
        );
    }

    #[test]
    fn test_overhead_serialize() {
        let mut resources = ResourceList::new();
        resources.insert("cpu".to_string(), Quantity::from("100m"));

        let overhead = Overhead {
            pod_fixed: resources,
        };
        let json = serde_json::to_string(&overhead).unwrap();
        assert!(json.contains("\"podFixed\""));
        assert!(json.contains("\"cpu\""));
        assert!(json.contains("\"100m\""));
    }

    #[test]
    fn test_overhead_deserialize() {
        let json = r#"{"podFixed":{"cpu":"100m","memory":"100Mi"}}"#;
        let overhead: Overhead = serde_json::from_str(json).unwrap();
        assert_eq!(overhead.pod_fixed.len(), 2);
        assert_eq!(overhead.pod_fixed.get("cpu"), Some(&Quantity::from("100m")));
        assert_eq!(
            overhead.pod_fixed.get("memory"),
            Some(&Quantity::from("100Mi"))
        );
    }

    #[test]
    fn test_overhead_round_trip() {
        let mut resources = ResourceList::new();
        resources.insert("cpu".to_string(), Quantity::from("100m"));
        resources.insert("memory".to_string(), Quantity::from("100Mi"));

        let original = Overhead {
            pod_fixed: resources,
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Overhead = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_scheduling_default() {
        let scheduling = Scheduling::default();
        assert!(scheduling.node_selector.is_empty());
        assert!(scheduling.tolerations.is_empty());
    }

    #[test]
    fn test_scheduling_with_node_selector() {
        let mut node_selector = BTreeMap::new();
        node_selector.insert("node-role.kubernetes.io/worker".to_string(), "".to_string());
        node_selector.insert("disktype".to_string(), "ssd".to_string());

        let scheduling = Scheduling {
            node_selector,
            tolerations: vec![],
        };
        assert_eq!(scheduling.node_selector.len(), 2);
        assert_eq!(
            scheduling.node_selector.get("disktype"),
            Some(&"ssd".to_string())
        );
    }

    #[test]
    fn test_scheduling_with_tolerations() {
        let scheduling = Scheduling {
            node_selector: BTreeMap::new(),
            tolerations: vec![Toleration {
                ..Default::default()
            }],
        };
        assert_eq!(scheduling.tolerations.len(), 1);
    }

    #[test]
    fn test_scheduling_serialize() {
        let mut node_selector = BTreeMap::new();
        node_selector.insert("key".to_string(), "value".to_string());

        let scheduling = Scheduling {
            node_selector,
            tolerations: vec![],
        };
        let json = serde_json::to_string(&scheduling).unwrap();
        assert!(json.contains("\"nodeSelector\""));
        assert!(json.contains("\"key\""));
        assert!(json.contains("\"value\""));
    }

    #[test]
    fn test_scheduling_deserialize() {
        let json = r#"{
            "nodeSelector": {"disktype": "ssd"},
            "tolerations": []
        }"#;
        let scheduling: Scheduling = serde_json::from_str(json).unwrap();
        assert_eq!(
            scheduling.node_selector.get("disktype"),
            Some(&"ssd".to_string())
        );
        assert!(scheduling.tolerations.is_empty());
    }

    #[test]
    fn test_scheduling_round_trip() {
        let mut node_selector = BTreeMap::new();
        node_selector.insert("key1".to_string(), "value1".to_string());
        node_selector.insert("key2".to_string(), "value2".to_string());

        let original = Scheduling {
            node_selector,
            tolerations: vec![],
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Scheduling = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_runtime_class_with_all_fields() {
        let mut resource_list = ResourceList::new();
        resource_list.insert("cpu".to_string(), Quantity::from("100m"));

        let mut node_selector = BTreeMap::new();
        node_selector.insert("node-role".to_string(), "worker".to_string());

        let rc = RuntimeClass {
            metadata: Some(ObjectMeta {
                name: Some("complete-class".to_string()),
                ..Default::default()
            }),
            handler: "runc".to_string(),
            overhead: Some(Overhead {
                pod_fixed: resource_list,
            }),
            scheduling: Some(Scheduling {
                node_selector,
                tolerations: vec![],
            }),
        };

        assert_eq!(rc.handler, "runc");
        assert!(rc.overhead.is_some());
        assert!(rc.scheduling.is_some());
        assert_eq!(
            rc.metadata.as_ref().unwrap().name,
            Some("complete-class".to_string())
        );
    }
}
