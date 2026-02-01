//! Node-related types from the Kubernetes Core API
//!
//! This module contains types for node addresses, configuration,
//! and complete Node resource definitions.

use crate::common::{ListMeta, ObjectMeta, TypeMeta};
use crate::core::internal::{ConditionStatus, NodeAddressType, ResourceList};
use crate::impl_has_object_meta;
use serde::{Deserialize, Serialize};

/// NodeAddress contains information for the node's address.
///
/// Corresponds to [Kubernetes NodeAddress](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5745)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NodeAddress {
    /// Node address type.
    pub r#type: NodeAddressType,
    /// The node address.
    pub address: String,
}

/// NodeConfigSource specifies a source of node configuration.
///
/// Corresponds to [Kubernetes NodeConfigSource](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5391)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NodeConfigSource {
    /// ConfigMap is a reference to a ConfigMap for node configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config_map: Option<ConfigMapNodeConfigSource>,
}

/// ConfigMapNodeConfigSource specifies a ConfigMap for node configuration.
///
/// Corresponds to [Kubernetes ConfigMapNodeConfigSource](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5396)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ConfigMapNodeConfigSource {
    /// Namespace of the ConfigMap.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Name of the ConfigMap.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// UID of the ConfigMap.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    /// Resource version of the ConfigMap.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_version: Option<String>,
    /// Kubelet config key to fetch from the ConfigMap.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kubelet_config_key: Option<String>,
}

/// NodeConfigStatus describes the status of config assigned to the node.
///
/// Corresponds to [Kubernetes NodeConfigStatus](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5509)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NodeConfigStatus {
    /// Assigned is the config the node will try to use.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assigned: Option<NodeConfigSource>,
    /// Active is the config the node is actively using.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<NodeConfigSource>,
    /// LastKnownGood is the config the node will fall back to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_known_good: Option<NodeConfigSource>,
    /// Error describes any problems reconciling the config.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub error: String,
}

// ============================================================================
// Node Condition
// ============================================================================

/// NodeCondition contains condition information for a node.
///
/// Corresponds to [Kubernetes NodeCondition](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5683)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NodeCondition {
    /// Type of node condition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub r#type: String,
    /// Status of the condition, one of True, False, Unknown.
    #[serde(default)]
    pub status: ConditionStatus,
    /// Last time we got an update on a given condition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_heartbeat_time: Option<crate::common::Timestamp>,
    /// Last time the condition transitioned from one status to another.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<crate::common::Timestamp>,
    /// The reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    /// Human-readable message indicating details about last transition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
}

// ============================================================================
// Node System Info
// ============================================================================

/// NodeSystemInfo is a set of ids/uuids to uniquely identify the node.
///
/// Corresponds to [Kubernetes NodeSystemInfo](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5781)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NodeSystemInfo {
    /// MachineID reported by the node.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub machine_id: String,
    /// SystemUUID reported by the node.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub system_uuid: String,
    /// BootID reported by the node.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub boot_id: String,
    /// Kernel Version reported by the node.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub kernel_version: String,
    /// OS Image reported by the node.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub os_image: String,
    /// ContainerRuntime Version reported by the node.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub container_runtime_version: String,
    /// Kubelet Version reported by the node.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub kubelet_version: String,
    /// KubeProxy Version reported by the node.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub kube_proxy_version: String,
    /// Operating System reported by the node.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub operating_system: String,
    /// The Architecture reported by the node.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub architecture: String,
    /// Swap Info reported by the node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub swap: Option<NodeSwapStatus>,
}

// ============================================================================
// Node Daemon Endpoints
// ============================================================================

/// NodeDaemonEndpoints lists ports opened by daemons running on the Node.
///
/// Corresponds to [Kubernetes NodeDaemonEndpoints](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5761)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NodeDaemonEndpoints {
    /// Endpoint on which Kubelet is listening.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kubelet_endpoint: Option<DaemonEndpoint>,
}

/// DaemonEndpoint contains information about a single daemon endpoint.
///
/// Corresponds to [Kubernetes DaemonEndpoint](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5767)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DaemonEndpoint {
    /// Port number of the given endpoint.
    #[serde(default)]
    pub port: i32,
}

// ============================================================================
// Attached Volume
// ============================================================================

/// AttachedVolume describes a volume attached to a node.
///
/// Corresponds to [Kubernetes AttachedVolume](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5815)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AttachedVolume {
    /// Name of the attached volume.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// DevicePath represents the device path where the volume should be available.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub device_path: String,
}

// ============================================================================
// Avoid Pods
// ============================================================================

/// AvoidPods describes pods that should avoid this node.
///
/// Corresponds to [Kubernetes AvoidPods](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5825)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AvoidPods {
    /// Bounded-sized list of pods that should avoid this node.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub prefer_avoid_pods: Vec<PreferAvoidPodsEntry>,
}

/// PreferAvoidPodsEntry describes a set of pods to avoid.
///
/// Corresponds to [Kubernetes PreferAvoidPodsEntry](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5832)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PreferAvoidPodsEntry {
    /// The class of the entry.
    #[serde(default)]
    pub pod_signature: PodSignature,
    /// Time at which this entry was added.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
}

/// PodSignature represents a pod's signature for avoidance.
///
/// Corresponds to [Kubernetes PodSignature](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5840)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodSignature {
    /// PodController is a reference to the controller that manages this pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pod_controller: Option<i64>,
}

// ============================================================================
// Container Image
// ============================================================================

/// ContainerImage represents a container image.
///
/// Corresponds to [Kubernetes ContainerImage](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5848)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContainerImage {
    /// Names by which this image is known.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub names: Vec<String>,
    /// The size of the image in bytes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<i64>,
}

// ============================================================================
// Node Runtime Handler
// ============================================================================

/// NodeRuntimeHandler features describe the set of features implemented by a CRI runtime.
///
/// Corresponds to [Kubernetes NodeRuntimeHandlerFeatures](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5860)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NodeRuntimeHandlerFeatures {
    /// true if the runtime supports recursive read-only mounts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recursive_read_only_mounts: Option<bool>,
    /// true if the runtime supports user namespaces.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_namespaces: Option<bool>,
}

/// NodeRuntimeHandler describes the runtime handler.
///
/// Corresponds to [Kubernetes NodeRuntimeHandler](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5868)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NodeRuntimeHandler {
    /// The name of the runtime handler.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// The features supported by the runtime handler.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub features: Option<NodeRuntimeHandlerFeatures>,
}

/// NodeFeatures describes the set of features implemented by the CRI implementation.
///
/// Corresponds to [Kubernetes NodeFeatures](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5878)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NodeFeatures {
    /// true if the runtime supports SupplementalGroupsPolicy and ContainerUser.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supplemental_groups_policy: Option<bool>,
}

/// NodeSwapStatus represents the swap status of a node.
///
/// Corresponds to [Kubernetes NodeSwapStatus](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5886)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NodeSwapStatus {
    /// Total amount of swap memory in bytes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
}

// ============================================================================
// Node Spec
// ============================================================================

/// NodeSpec describes the attributes that a node is created with.
///
/// Corresponds to [Kubernetes NodeSpec](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5550)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NodeSpec {
    /// PodCIDR represents the pod IP range assigned to the node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pod_cidr: Option<String>,
    /// PodCIDRs represents multiple IP ranges assigned to the node.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pod_cidrs: Vec<String>,
    /// ID of the node assigned by the cloud provider.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_id: Option<String>,
    /// Unschedulable controls node schedulability of new pods.
    #[serde(default)]
    pub unschedulable: bool,
    /// If specified, the node's taints.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub taints: Vec<crate::core::internal::Taint>,
    /// Deprecated: Previously used to specify the source of the node's configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config_source: Option<NodeConfigSource>,
    /// Deprecated: Previously used to specify the amount of memory.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
}

// ============================================================================
// Node Status
// ============================================================================

/// NodeStatus is information about the current status of a node.
///
/// Corresponds to [Kubernetes NodeStatus](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5584)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NodeStatus {
    /// Capacity represents the total resources of a node.
    #[serde(default, skip_serializing_if = "ResourceList::is_empty")]
    pub capacity: ResourceList,
    /// Allocatable represents the resources of a node.
    #[serde(default, skip_serializing_if = "ResourceList::is_empty")]
    pub allocatable: ResourceList,
    /// Phase is the current lifecycle phase of the node.
    #[serde(default)]
    pub phase: crate::core::internal::NodePhase,
    /// Conditions is an array of current node conditions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<NodeCondition>,
    /// List of addresses reachable to the node.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub addresses: Vec<NodeAddress>,
    /// DaemonEndpoints lists the ports opened by daemons running on the Node.
    #[serde(default)]
    pub daemon_endpoints: NodeDaemonEndpoints,
    /// Set of ids/uuids to uniquely identify the node.
    #[serde(default)]
    pub node_info: NodeSystemInfo,
    /// List of container images on this node.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<ContainerImage>,
    /// Volumes in use on this node.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volumes_in_use: Vec<String>,
    /// Deprecated: List of attachable volumes on this node.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volumes_attached: Vec<AttachedVolume>,
    /// Status of the config assigned to the node via the dynamic Kubelet config feature.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<NodeConfigStatus>,
    /// The available runtime handlers.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub runtime_handlers: Vec<NodeRuntimeHandler>,
    /// The available features of the kubelet and the runtime handler.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub features: Option<NodeFeatures>,
}

// ============================================================================
// Node
// ============================================================================

/// Node is a worker node in Kubernetes.
///
/// Corresponds to [Kubernetes Node](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5467)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ObjectMeta,
    /// Spec defines the behavior of a node.
    #[serde(default)]
    pub spec: NodeSpec,
    /// Status describes the current status of a node.
    #[serde(default)]
    pub status: NodeStatus,
}
impl_has_object_meta!(Node);

/// NodeList is a list of nodes.
///
/// Corresponds to [Kubernetes NodeList](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5478)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NodeList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    /// List of nodes.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Node>,
}

#[cfg(test)]
mod tests {}
