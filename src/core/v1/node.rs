//! Node types from the Kubernetes Core v1 API
//!
//! This module contains types for Kubernetes nodes.

use crate::common::{ListMeta, ObjectMeta, Quantity, Timestamp};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Node Types
// ============================================================================

/// Node is a worker node in Kubernetes.
///
/// Corresponds to [Kubernetes Node](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L6994)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// Spec defines the behavior of a node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<NodeSpec>,

    /// Status describes the current status of a node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<NodeStatus>,
}

/// NodeList is a list of nodes.
///
/// Corresponds to [Kubernetes NodeList](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7018)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct NodeList {
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// List of nodes.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Node>,
}

/// NodeSpec describes the attributes that a node is created with.
///
/// Corresponds to [Kubernetes NodeSpec](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L6507)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NodeSpec {
    /// PodCIDR represents the pod IP range assigned to the node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pod_cidr: Option<String>,

    /// PodCIDRs represents the IP ranges assigned to the node.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pod_cidrs: Vec<String>,

    /// ProviderID is the ID of the cloud provider.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_id: Option<String>,

    /// Unschedulable controls node schedulability of new pods.
    #[serde(default)]
    pub unschedulable: bool,

    /// Taints attached to the node.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub taints: Vec<Taint>,

    /// ConfigSource specifies the source for the node's configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config_source: Option<NodeConfigSource>,

    /// ExternalID is the ID of the node to external systems.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
}

/// NodeStatus is information about the current status of a node.
///
/// Corresponds to [Kubernetes NodeStatus](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L6721)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NodeStatus {
    /// Capacity represents the total resources of a node.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub capacity: HashMap<String, Quantity>,

    /// Allocatable represents the resources of a node that are available for scheduling.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub allocatable: HashMap<String, Quantity>,

    /// Phase is the current phase of the node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,

    /// Conditions is an array of current node conditions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<NodeCondition>,

    /// Addresses is a list of addresses reachable to the node.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub addresses: Vec<NodeAddress>,

    /// DaemonEndpoints lists the endpoints for daemons running on the node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub daemon_endpoints: Option<NodeDaemonEndpoints>,

    /// NodeInfo is information about the node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_info: Option<NodeSystemInfo>,

    /// Images is a list of container images on this node.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<ContainerImage>,

    /// VolumesInUse is a list of volumes that are attached to the node.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volumes_in_use: Vec<String>,

    /// VolumesAttached is a list of volumes attached to the node.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volumes_attached: Vec<AttachedVolume>,

    /// Config describes the config of the node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<NodeConfigStatus>,

    /// RuntimeHandlers is a list of available runtime handlers.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub runtime_handlers: Vec<NodeRuntimeHandler>,

    /// Features is a list of features for the node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub features: Option<NodeFeatures>,
}

/// Taint describes a taint on a node.
///
/// Corresponds to [Kubernetes Taint](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4036)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Taint {
    /// The taint key to be applied to a node.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub key: String,

    /// The taint value corresponding to the taint key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    /// The effect of the taint on pods that do not tolerate the taint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,

    /// TimeAdded represents the time at which the taint was added.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_added: Option<Timestamp>,
}

/// TaintEffect constants
pub mod taint_effect {
    /// Do not allow new pods to schedule onto the node unless they tolerate the taint.
    pub const NO_SCHEDULE: &str = "NoSchedule";

    /// Do not allow new pods to schedule onto the node unless they tolerate the taint.
    pub const PREFER_NO_SCHEDULE: &str = "PreferNoSchedule";

    /// Evict any already-running pods that do not tolerate the taint.
    pub const NO_EXECUTE: &str = "NoExecute";
}

/// NodeConfigSource specifies a source for node configuration.
///
/// Corresponds to [Kubernetes NodeConfigSource](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L6544)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct NodeConfigSource {
    /// ConfigMap is a reference to a ConfigMap.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config_map: Option<ConfigMapNodeConfigSource>,
}

/// ConfigMapNodeConfigSource references a ConfigMap for node configuration.
///
/// Corresponds to [Kubernetes ConfigMapNodeConfigSource](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L6562)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ConfigMapNodeConfigSource {
    /// Namespace is the namespace of the ConfigMap.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    /// Name is the name of the ConfigMap.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// UID is the UID of the ConfigMap.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,

    /// ResourceVersion is the resource version of the ConfigMap.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_version: Option<String>,

    /// KubeletConfigKey is the key in the ConfigMap to select.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kubelet_config_key: Option<String>,
}

/// NodeDaemonEndpoints lists the endpoints for daemons running on a node.
///
/// Corresponds to [Kubernetes NodeDaemonEndpoints](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L6599)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NodeDaemonEndpoints {
    /// KubeletEndpoint is the endpoint for the kubelet.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kubelet_endpoint: Option<DaemonEndpoint>,
}

/// DaemonEndpoint contains information about a single daemon endpoint.
///
/// Corresponds to [Kubernetes DaemonEndpoint](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L6587)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DaemonEndpoint {
    /// Port number of the daemon endpoint.
    pub port: i32,
}

/// NodeRuntimeHandler describes a runtime handler.
///
/// Corresponds to [Kubernetes NodeRuntimeHandler](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L6617)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct NodeRuntimeHandler {
    /// Name is the name of the runtime handler.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    /// Features is a list of features for the runtime handler.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub features: Option<NodeRuntimeHandlerFeatures>,
}

/// NodeRuntimeHandlerFeatures describes the features of a runtime handler.
///
/// Corresponds to [Kubernetes NodeRuntimeHandlerFeatures](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L6606)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NodeRuntimeHandlerFeatures {
    /// RecursiveReadOnlyMounts is whether the runtime supports recursive read-only mounts.
    #[serde(default)]
    pub recursive_read_only_mounts: bool,

    /// UserNamespaces is whether the runtime supports user namespaces.
    #[serde(default)]
    pub user_namespaces: bool,
}

/// NodeFeatures describes the features of a node.
///
/// Corresponds to [Kubernetes NodeFeatures](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L6630)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NodeFeatures {
    /// SupplementalGroupsPolicy is whether the node supports supplemental groups policy.
    #[serde(default)]
    pub supplemental_groups_policy: bool,
}

/// NodeSystemInfo is information about the system.
///
/// Corresponds to [Kubernetes NodeSystemInfo](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L6637)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NodeSystemInfo {
    /// MachineID is the machine ID reported by the node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub machine_id: Option<String>,

    /// SystemUUID is the system UUID reported by the node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_uuid: Option<String>,

    /// BootID is the boot ID reported by the node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boot_id: Option<String>,

    /// KernelVersion is the kernel version reported by the node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kernel_version: Option<String>,

    /// OSImage is the OS image reported by the node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub os_image: Option<String>,

    /// ContainerRuntimeVersion is the container runtime version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container_runtime_version: Option<String>,

    /// KubeletVersion is the kubelet version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kubelet_version: Option<String>,

    /// KubeProxyVersion is the kube-proxy version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kube_proxy_version: Option<String>,

    /// OperatingSystem is the operating system reported by the node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,

    /// Architecture is the architecture reported by the node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,

    /// Swap is the swap size of the node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub swap: Option<NodeSwapStatus>,
}

/// NodeSwapStatus describes the swap status of a node.
///
/// Corresponds to [Kubernetes NodeSwapStatus](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L6667)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct NodeSwapStatus {
    /// Capacity is the total swap capacity in bytes.
    pub capacity: i64,
}

/// NodeConfigStatus describes the config status of a node.
///
/// Corresponds to [Kubernetes NodeConfigStatus](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L6674)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NodeConfigStatus {
    /// Assigned is the config assigned to the node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assigned: Option<NodeConfigSource>,

    /// Active is the config actively used by the node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<NodeConfigSource>,

    /// LastKnownGood is the last known good config.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_known_good: Option<NodeConfigSource>,

    /// Error describes any errors with the config.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// NodePhase constants
pub mod node_phase {
    /// Pending means the node is pending.
    pub const PENDING: &str = "Pending";

    /// Running means the node is running.
    pub const RUNNING: &str = "Running";

    /// Terminated means the node is terminated.
    pub const TERMINATED: &str = "Terminated";
}

/// NodeConditionType constants
pub mod node_condition_type {
    /// Ready means the node is ready.
    pub const READY: &str = "Ready";

    /// MemoryPressure means the node is under memory pressure.
    pub const MEMORY_PRESSURE: &str = "MemoryPressure";

    /// DiskPressure means the node is under disk pressure.
    pub const DISK_PRESSURE: &str = "DiskPressure";

    /// PIDPressure means the node is under PID pressure.
    pub const PID_PRESSURE: &str = "PIDPressure";

    /// NetworkUnavailable means the network is unavailable.
    pub const NETWORK_UNAVAILABLE: &str = "NetworkUnavailable";
}

/// NodeCondition describes the condition of a node.
///
/// Corresponds to [Kubernetes NodeCondition](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L6885)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct NodeCondition {
    /// Type is the type of the condition.
    #[serde(rename = "type")]
    pub type_: String,

    /// Status is the status of the condition.
    pub status: String,

    /// LastHeartbeatTime is the last time the condition was updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_heartbeat_time: Option<Timestamp>,

    /// LastTransitionTime is the last time the condition transitioned.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<Timestamp>,

    /// Reason is the reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    /// Message is a human-readable message about the condition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// NodeAddressType constants
pub mod node_address_type {
    /// Hostname is the hostname.
    pub const HOSTNAME: &str = "Hostname";

    /// InternalIP is the internal IP address.
    pub const INTERNAL_IP: &str = "InternalIP";

    /// ExternalIP is the external IP address.
    pub const EXTERNAL_IP: &str = "ExternalIP";

    /// InternalDNS is the internal DNS name.
    pub const INTERNAL_DNS: &str = "InternalDNS";

    /// ExternalDNS is the external DNS name.
    pub const EXTERNAL_DNS: &str = "ExternalDNS";
}

/// NodeAddress contains information about a node address.
///
/// Corresponds to [Kubernetes NodeAddress](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L6949)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct NodeAddress {
    /// Type is the type of the address.
    #[serde(rename = "type")]
    pub type_: String,

    /// Address is the address.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub address: String,
}

/// AttachedVolume describes a volume attached to a node.
///
/// Corresponds to [Kubernetes AttachedVolume](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L6800)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AttachedVolume {
    /// Name is the name of the volume.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    /// DevicePath is the path to the device.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_path: Option<String>,
}

/// ContainerImage describes a container image on a node.
///
/// Corresponds to [Kubernetes ContainerImage](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L6843)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ContainerImage {
    /// Names are the names of the container image.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub names: Vec<String>,

    /// SizeBytes is the size of the image in bytes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<i64>,
}

/// NodeProxyOptions is the query options to a Node's proxy call.
///
/// Corresponds to [Kubernetes NodeProxyOptions](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7365)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NodeProxyOptions {
    /// Path is the URL path to use.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_default() {
        let node = Node {
            metadata: None,
            spec: None,
            status: None,
        };
        assert!(node.metadata.is_none());
        assert!(node.spec.is_none());
        assert!(node.status.is_none());
    }

    #[test]
    fn test_node_with_spec() {
        let spec = NodeSpec {
            pod_cidr: Some("10.0.0.0/24".to_string()),
            ..Default::default()
        };

        let node = Node {
            metadata: Some(ObjectMeta {
                name: Some("node-1".to_string()),
                ..Default::default()
            }),
            spec: Some(spec),
            status: None,
        };

        assert_eq!(
            node.metadata.as_ref().unwrap().name,
            Some("node-1".to_string())
        );
        assert!(node.spec.is_some());
    }

    #[test]
    fn test_node_serialize() {
        let node = Node {
            metadata: Some(ObjectMeta {
                name: Some("node-1".to_string()),
                ..Default::default()
            }),
            spec: Some(NodeSpec {
                pod_cidr: Some("10.0.0.0/24".to_string()),
                ..Default::default()
            }),
            status: None,
        };

        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains(r#""name":"node-1""#));
        assert!(json.contains(r#""podCidr":"10.0.0.0/24""#));
    }

    #[test]
    fn test_node_deserialize() {
        let json = r#"{
            "metadata": {"name": "node-1"},
            "spec": {
                "podCIDR": "10.0.0.0/24",
                "unschedulable": false
            }
        }"#;

        let node: Node = serde_json::from_str(json).unwrap();
        assert_eq!(
            node.metadata.as_ref().unwrap().name,
            Some("node-1".to_string())
        );
        assert!(node.spec.is_some());
    }

    #[test]
    fn test_node_spec_with_taints() {
        let spec = NodeSpec {
            taints: vec![Taint {
                key: "key1".to_string(),
                value: Some("value1".to_string()),
                effect: Some(taint_effect::NO_SCHEDULE.to_string()),
                time_added: None,
            }],
            ..Default::default()
        };

        assert_eq!(spec.taints.len(), 1);
    }

    #[test]
    fn test_taint_serialize() {
        let taint = Taint {
            key: "key1".to_string(),
            value: Some("value1".to_string()),
            effect: Some(taint_effect::NO_EXECUTE.to_string()),
            time_added: Some(Timestamp::from_str("2024-01-15T10:00:00Z")),
        };

        let json = serde_json::to_string(&taint).unwrap();
        assert!(json.contains(r#""key":"key1""#));
        assert!(json.contains(r#""effect":"NoExecute""#));
    }

    #[test]
    fn test_node_status_with_capacity() {
        let mut capacity = HashMap::new();
        capacity.insert("cpu".to_string(), Quantity::from("4"));
        capacity.insert("memory".to_string(), Quantity::from("16Gi"));

        let status = NodeStatus {
            capacity,
            ..Default::default()
        };

        assert_eq!(status.capacity.len(), 2);
    }

    #[test]
    fn test_node_condition_serialize() {
        let condition = NodeCondition {
            type_: node_condition_type::READY.to_string(),
            status: "True".to_string(),
            last_heartbeat_time: None,
            last_transition_time: None,
            reason: Some("KubeletReady".to_string()),
            message: None,
        };

        let json = serde_json::to_string(&condition).unwrap();
        assert!(json.contains(r#""type":"Ready""#));
        assert!(json.contains(r#""status":"True""#));
        assert!(json.contains(r#""reason":"KubeletReady""#));
    }

    #[test]
    fn test_node_address() {
        let addr = NodeAddress {
            type_: node_address_type::INTERNAL_IP.to_string(),
            address: "192.168.1.1".to_string(),
        };

        assert_eq!(addr.type_, node_address_type::INTERNAL_IP);
        assert_eq!(addr.address, "192.168.1.1");
    }

    #[test]
    fn test_node_system_info() {
        let info = NodeSystemInfo {
            kernel_version: Some("5.4.0".to_string()),
            os_image: Some("Ubuntu 20.04".to_string()),
            container_runtime_version: Some("docker://19.3.8".to_string()),
            kubelet_version: Some("v1.19.0".to_string()),
            ..Default::default()
        };

        assert_eq!(info.kernel_version, Some("5.4.0".to_string()));
        assert_eq!(info.os_image, Some("Ubuntu 20.04".to_string()));
    }

    #[test]
    fn test_node_list_empty() {
        let list = NodeList {
            metadata: None,
            items: vec![],
        };
        assert!(list.items.is_empty());
    }

    #[test]
    fn test_node_list_with_items() {
        let list = NodeList {
            metadata: Some(ListMeta {
                resource_version: Some("12345".to_string()),
                ..Default::default()
            }),
            items: vec![Node {
                metadata: Some(ObjectMeta {
                    name: Some("node-1".to_string()),
                    ..Default::default()
                }),
                spec: None,
                status: None,
            }],
        };

        assert_eq!(list.items.len(), 1);
    }

    #[test]
    fn test_attached_volume() {
        let volume = AttachedVolume {
            name: "pv-1".to_string(),
            device_path: Some("/dev/xvda".to_string()),
        };

        assert_eq!(volume.name, "pv-1");
        assert_eq!(volume.device_path, Some("/dev/xvda".to_string()));
    }

    #[test]
    fn test_container_image() {
        let image = ContainerImage {
            names: vec!["nginx:latest".to_string(), "nginx@sha256:abc".to_string()],
            size_bytes: Some(133000000),
        };

        assert_eq!(image.names.len(), 2);
        assert_eq!(image.size_bytes, Some(133000000));
    }

    #[test]
    fn test_taint_effect_constants() {
        assert_eq!(taint_effect::NO_SCHEDULE, "NoSchedule");
        assert_eq!(taint_effect::PREFER_NO_SCHEDULE, "PreferNoSchedule");
        assert_eq!(taint_effect::NO_EXECUTE, "NoExecute");
    }

    #[test]
    fn test_node_phase_constants() {
        assert_eq!(node_phase::PENDING, "Pending");
        assert_eq!(node_phase::RUNNING, "Running");
        assert_eq!(node_phase::TERMINATED, "Terminated");
    }

    #[test]
    fn test_node_condition_type_constants() {
        assert_eq!(node_condition_type::READY, "Ready");
        assert_eq!(node_condition_type::MEMORY_PRESSURE, "MemoryPressure");
        assert_eq!(node_condition_type::DISK_PRESSURE, "DiskPressure");
        assert_eq!(node_condition_type::PID_PRESSURE, "PIDPressure");
        assert_eq!(
            node_condition_type::NETWORK_UNAVAILABLE,
            "NetworkUnavailable"
        );
    }

    #[test]
    fn test_node_address_type_constants() {
        assert_eq!(node_address_type::HOSTNAME, "Hostname");
        assert_eq!(node_address_type::INTERNAL_IP, "InternalIP");
        assert_eq!(node_address_type::EXTERNAL_IP, "ExternalIP");
        assert_eq!(node_address_type::INTERNAL_DNS, "InternalDNS");
        assert_eq!(node_address_type::EXTERNAL_DNS, "ExternalDNS");
    }

    #[test]
    fn test_node_round_trip() {
        let original = Node {
            metadata: Some(ObjectMeta {
                name: Some("node-1".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(NodeSpec {
                pod_cidr: Some("10.0.0.0/24".to_string()),
                ..Default::default()
            }),
            status: Some(NodeStatus {
                phase: Some(node_phase::RUNNING.to_string()),
                ..Default::default()
            }),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Node = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }
}
