//! Kubernetes Core API Internal Types
//!
//! This module contains type definitions from k8s.io/api/core/v1/types.go
//! that are used internally by the Kubernetes API.
//!
//! Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go

use serde::{Deserialize, Serialize};

// ============================================================================
// Pod Related Enums
// ============================================================================

/// PodPhase is a label for the condition of a pod at the current time.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3100
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum PodPhase {
    /// The pod has been accepted by the system, but one or more of the containers
    /// has not been started. This includes time before being bound to a node, as well as time spent
    /// pulling images onto the host.
    #[serde(rename = "Pending")]
    #[default]
    Pending,
    /// The pod has been bound to a node and all of the containers have been started.
    /// At least one container is still running or is in the process of being restarted.
    #[serde(rename = "Running")]
    Running,
    /// All containers in the pod have voluntarily terminated
    /// with a container exit code of 0, and the system is not going to restart any of these containers.
    #[serde(rename = "Succeeded")]
    Succeeded,
    /// All containers in the pod have terminated, and at least one container has
    /// terminated in a failure (exited with a non-zero exit code or was stopped by the system).
    #[serde(rename = "Failed")]
    Failed,
    /// The state of the pod could not be obtained, typically due
    /// to an error in communicating with the host of the pod.
    /// Deprecated in v1.21.
    #[serde(rename = "Unknown")]
    Unknown,
}

pub mod pod_phase {
    pub const PENDING: &str = "Pending";
    pub const RUNNING: &str = "Running";
    pub const SUCCEEDED: &str = "Succeeded";
    pub const FAILED: &str = "Failed";
    pub const UNKNOWN: &str = "Unknown";
}

/// PodConditionType defines the condition of pod.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3124
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum PodConditionType {
    /// Represents status of the scheduling process for this pod.
    #[serde(rename = "PodScheduled")]
    PodScheduled,
    /// The pod is able to service requests and should be added to the
    /// load balancing backends of matching services.
    #[serde(rename = "Ready")]
    Ready,
    /// All init containers have started successfully.
    #[serde(rename = "Initialized")]
    Initialized,
    /// All containers in the pod are ready.
    #[serde(rename = "ContainersReady")]
    ContainersReady,
    /// The pod is being targeted for disruption.
    #[serde(rename = "DisruptionTarget")]
    DisruptionTarget,
    /// Pod resize is pending.
    #[serde(rename = "PodResizePending")]
    PodResizePending,
    /// Pod resize is in progress.
    #[serde(rename = "PodResizeInProgress")]
    PodResizeInProgress,
}

pub mod pod_condition_type {
    pub const POD_SCHEDULED: &str = "PodScheduled";
    pub const READY: &str = "Ready";
    pub const INITIALIZED: &str = "Initialized";
    pub const CONTAINERS_READY: &str = "ContainersReady";
    pub const DISRUPTION_TARGET: &str = "DisruptionTarget";
    pub const POD_RESIZE_PENDING: &str = "PodResizePending";
    pub const POD_RESIZE_IN_PROGRESS: &str = "PodResizeInProgress";
}

/// RestartPolicy defines the behavior for when a container exits.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3203
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum RestartPolicy {
    /// Always restart the container after it exits.
    #[serde(rename = "Always")]
    #[default]
    Always,
    /// Only restart if the container exits with a non-zero exit code.
    #[serde(rename = "OnFailure")]
    OnFailure,
    /// Never restart the container.
    #[serde(rename = "Never")]
    Never,
}

pub mod restart_policy {
    pub const ALWAYS: &str = "Always";
    pub const ON_FAILURE: &str = "OnFailure";
    pub const NEVER: &str = "Never";
}

/// DNSPolicy defines how a pod's DNS will be configured.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3284
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum DNSPolicy {
    /// Indicates that the pod should use cluster DNS
    /// first, if it is available, then fall back on the default
    /// (as determined by kubelet) DNS settings.
    #[serde(rename = "ClusterFirstWithHostNet")]
    ClusterFirstWithHostNet,
    /// Indicates that the pod should use cluster DNS
    /// first unless hostNetwork is true, if it is available, then
    /// fall back on the default (as determined by kubelet) DNS settings.
    #[serde(rename = "ClusterFirst")]
    #[default]
    ClusterFirst,
    /// Indicates that the pod should use the default (as
    /// determined by kubelet) DNS settings.
    #[serde(rename = "Default")]
    Default,
    /// Indicates that the pod should use empty DNS settings. DNS
    /// parameters such as nameservers and search paths should be defined via
    /// DNSConfig.
    #[serde(rename = "None")]
    None,
}

pub mod dns_policy {
    pub const CLUSTER_FIRST_WITH_HOST_NET: &str = "ClusterFirstWithHostNet";
    pub const CLUSTER_FIRST: &str = "ClusterFirst";
    pub const DEFAULT: &str = "Default";
    pub const NONE: &str = "None";
}

// ============================================================================
// Network Related Enums
// ============================================================================

/// Protocol defines network protocols supported for things like container ports.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L901
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum Protocol {
    /// TCP protocol.
    #[default]
    #[serde(rename = "TCP")]
    Tcp,
    /// UDP protocol.
    #[serde(rename = "UDP")]
    Udp,
    /// SCTP protocol.
    #[serde(rename = "SCTP")]
    Sctp,
}

// impl Protocol {
//     fn tcp() -> Self {
//         Self::Tcp
//     }
// }

pub mod protocol {
    pub const TCP: &str = "TCP";
    pub const UDP: &str = "UDP";
    pub const SCTP: &str = "SCTP";
}

// ============================================================================
// Container Related Enums
// ============================================================================

/// PullPolicy describes a policy for if/when to pull a container image.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2484
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum PullPolicy {
    /// Always pull the image.
    #[serde(rename = "Always")]
    Always,
    /// Never pull the image, only use local images.
    #[serde(rename = "Never")]
    Never,
    /// Pull the image if not present locally.
    #[serde(rename = "IfNotPresent")]
    #[default]
    IfNotPresent,
}

pub mod pull_policy {
    pub const ALWAYS: &str = "Always";
    pub const NEVER: &str = "Never";
    pub const IF_NOT_PRESENT: &str = "IfNotPresent";
}

// ============================================================================
// Condition Related Enums
// ============================================================================

/// ConditionStatus is the status of a condition.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2881
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum ConditionStatus {
    /// The condition is true.
    #[serde(rename = "True")]
    True,
    /// The condition is false.
    #[serde(rename = "False")]
    False,
    /// The condition status is unknown.
    #[serde(rename = "Unknown")]
    #[default]
    Unknown,
}

pub mod condition_status {
    pub const TRUE: &str = "True";
    pub const FALSE: &str = "False";
    pub const UNKNOWN: &str = "Unknown";
}

// ============================================================================
// Namespace Related Enums
// ============================================================================

/// NamespacePhase is the current lifecycle phase of a namespace.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5835
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum NamespacePhase {
    /// The namespace is available for use in the system.
    #[serde(rename = "Active")]
    Active,
    /// The namespace is undergoing graceful termination.
    #[serde(rename = "Terminating")]
    Terminating,
}

pub mod namespace_phase {
    pub const ACTIVE: &str = "Active";
    pub const TERMINATING: &str = "Terminating";
}

/// NamespaceConditionType defines constants reporting on status during namespace lifetime and deletion progress.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5846
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum NamespaceConditionType {
    /// Failed to discover resources during namespace deletion.
    #[default]
    #[serde(rename = "NamespaceDeletionDiscoveryFailure")]
    NamespaceDeletionDiscoveryFailure,
    /// Failed to delete content during namespace deletion.
    #[serde(rename = "NamespaceDeletionContentFailure")]
    NamespaceDeletionContentFailure,
    /// Failed to parse GroupVersion during namespace deletion.
    #[serde(rename = "NamespaceDeletionGroupVersionParsingFailure")]
    NamespaceDeletionGroupVersionParsingFailure,
}

// impl NamespaceConditionType {
//     fn discovery_failure() -> Self {
//         Self::NamespaceDeletionDiscoveryFailure
//     }
// }

pub mod namespace_condition_type {
    pub const NAMESPACE_DELETION_DISCOVERY_FAILURE: &str = "NamespaceDeletionDiscoveryFailure";
    pub const NAMESPACE_DELETION_CONTENT_FAILURE: &str = "NamespaceDeletionContentFailure";
    pub const NAMESPACE_DELETION_GV_PARSING_FAILURE: &str =
        "NamespaceDeletionGroupVersionParsingFailure";
}

// ============================================================================
// Service Related Enums
// ============================================================================

/// ServiceType describes how a service is exposed.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4801
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum ServiceType {
    /// Service will only be accessible inside the cluster, via the ClusterIP.
    #[serde(rename = "ClusterIP")]
    ClusterIp,
    /// Service will be exposed on one port of every node, in addition to 'ClusterIP' type.
    #[serde(rename = "NodePort")]
    NodePort,
    /// Service will be exposed via an external load balancer (if the cloud provider supports it),
    /// in addition to 'NodePort' type.
    #[serde(rename = "LoadBalancer")]
    LoadBalancer,
    /// Service consists of only a reference to an external name.
    #[serde(rename = "ExternalName")]
    ExternalName,
}

pub mod service_type {
    pub const CLUSTER_IP: &str = "ClusterIP";
    pub const NODE_PORT: &str = "NodePort";
    pub const LOAD_BALANCER: &str = "LoadBalancer";
    pub const EXTERNAL_NAME: &str = "ExternalName";
}

// ============================================================================
// Node Related Enums
// ============================================================================

/// NodePhase is the current lifecycle phase of a node.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5657
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum NodePhase {
    /// The node has been created/added by the system, but not configured.
    #[serde(rename = "Pending")]
    #[default]
    Pending,
    /// The node has been configured and has Kubernetes components running.
    #[serde(rename = "Running")]
    Running,
    /// The node has been removed from the cluster.
    #[serde(rename = "Terminated")]
    Terminated,
}

pub mod node_phase {
    pub const PENDING: &str = "Pending";
    pub const RUNNING: &str = "Running";
    pub const TERMINATED: &str = "Terminated";
}

/// NodeConditionType defines node's condition.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5670
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum NodeConditionType {
    /// Kubelet is healthy and ready to accept pods.
    #[serde(rename = "Ready")]
    Ready,
    /// Kubelet is under pressure due to insufficient available memory.
    #[serde(rename = "MemoryPressure")]
    MemoryPressure,
    /// Kubelet is under pressure due to insufficient available disk.
    #[serde(rename = "DiskPressure")]
    DiskPressure,
    /// Network for the node is not correctly configured.
    #[serde(rename = "NetworkUnavailable")]
    NetworkUnavailable,
}

pub mod node_condition_type {
    pub const READY: &str = "Ready";
    pub const MEMORY_PRESSURE: &str = "MemoryPressure";
    pub const DISK_PRESSURE: &str = "DiskPressure";
    pub const NETWORK_UNAVAILABLE: &str = "NetworkUnavailable";
}

/// NodeAddressType defines the type of node address.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5700
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum NodeAddressType {
    /// Identifies a name of the node.
    #[default]
    #[serde(rename = "Hostname")]
    Hostname,
    /// Identifies an IP address assigned to one of the node's network interfaces.
    #[serde(rename = "InternalIP")]
    InternalIp,
    /// Identifies an IP address which is routable within an external network.
    #[serde(rename = "ExternalIP")]
    ExternalIp,
    /// Identifies a DNS name which resolves to an internal IP.
    #[serde(rename = "InternalDNS")]
    InternalDns,
    /// Identifies a DNS name which resolves to an external IP.
    #[serde(rename = "ExternalDNS")]
    ExternalDns,
}

// impl NodeAddressType {
//     fn hostname() -> Self {
//         Self::Hostname
//     }
// }

pub mod node_address_type {
    pub const HOSTNAME: &str = "Hostname";
    pub const INTERNAL_IP: &str = "InternalIP";
    pub const EXTERNAL_IP: &str = "ExternalIP";
    pub const INTERNAL_DNS: &str = "InternalDNS";
    pub const EXTERNAL_DNS: &str = "ExternalDNS";
}

// ============================================================================
// Scheduling Related Enums
// ============================================================================

/// TaintEffect describes the effect of a taint on pods.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3586
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum TaintEffect {
    /// Do not allow new pods to schedule onto the node unless they tolerate the taint.
    #[serde(rename = "NoSchedule")]
    NoSchedule,
    /// Try not to schedule new pods onto the node, rather than prohibiting new pods from scheduling
    /// onto the node entirely.
    #[serde(rename = "PreferNoSchedule")]
    PreferNoSchedule,
    /// Evict any already-running pods that do not tolerate the taint.
    #[serde(rename = "NoExecute")]
    NoExecute,
}

pub mod taint_effect {
    pub const NO_SCHEDULE: &str = "NoSchedule";
    pub const PREFER_NO_SCHEDULE: &str = "PreferNoSchedule";
    pub const NO_EXECUTE: &str = "NoExecute";
}

// ============================================================================
// QoS Related Enums
// ============================================================================

/// PodQOSClass defines the quality of service class for a pod.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4269
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum PodQOSClass {
    /// Guaranteed QoS class.
    #[serde(rename = "Guaranteed")]
    Guaranteed,
    /// Burstable QoS class.
    #[serde(rename = "Burstable")]
    Burstable,
    /// BestEffort QoS class.
    #[serde(rename = "BestEffort")]
    BestEffort,
}

pub mod pod_qos_class {
    pub const GUARANTEED: &str = "Guaranteed";
    pub const BURSTABLE: &str = "Burstable";
    pub const BEST_EFFORT: &str = "BestEffort";
}

// ============================================================================
// Service Extended Enums
// ============================================================================

/// ServiceAffinity defines the session affinity for a service.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4765
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum ServiceAffinity {
    /// Client IP based session affinity.
    #[serde(rename = "ClientIP")]
    ClientIp,
    /// No session affinity.
    #[serde(rename = "None")]
    #[default]
    None,
}

pub mod service_affinity {
    pub const CLIENT_IP: &str = "ClientIP";
    pub const NONE: &str = "None";
}

/// ServiceInternalTrafficPolicy describes the endpoint-selection policy for traffic sent to the ClusterIP.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4825
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum ServiceInternalTrafficPolicy {
    /// Routes traffic to all endpoints.
    #[serde(rename = "Cluster")]
    Cluster,
    /// Routes traffic only to endpoints on the same node.
    #[serde(rename = "Local")]
    Local,
}

pub mod service_internal_traffic_policy {
    pub const CLUSTER: &str = "Cluster";
    pub const LOCAL: &str = "Local";
}

/// ServiceExternalTrafficPolicy describes the endpoint-selection policy for traffic to external service entrypoints.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4840
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum ServiceExternalTrafficPolicy {
    /// Routes traffic to all endpoints.
    #[serde(rename = "Cluster")]
    Cluster,
    /// Preserves the source IP by routing only to endpoints on the same node.
    #[serde(rename = "Local")]
    Local,
}

pub mod service_external_traffic_policy {
    pub const CLUSTER: &str = "Cluster";
    pub const LOCAL: &str = "Local";
}

/// IPFamily represents the IP family (IPv4 or IPv6).
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4934
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum IPFamily {
    /// IPv4 protocol.
    #[serde(rename = "IPv4")]
    Ipv4,
    /// IPv6 protocol.
    #[serde(rename = "IPv6")]
    Ipv6,
}

pub mod ip_family {
    pub const IPV4: &str = "IPv4";
    pub const IPV6: &str = "IPv6";
}

/// IPFamilyPolicy represents the dual-stack-ness requested or required by a Service.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4944
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum IPFamilyPolicy {
    /// Service is required to have a single IPFamily.
    #[serde(rename = "SingleStack")]
    SingleStack,
    /// Service prefers dual-stack when the cluster is configured for dual-stack.
    #[serde(rename = "PreferDualStack")]
    PreferDualStack,
    /// Service requires dual-stack.
    #[serde(rename = "RequireDualStack")]
    RequireDualStack,
}

pub mod ip_family_policy {
    pub const SINGLE_STACK: &str = "SingleStack";
    pub const PREFER_DUAL_STACK: &str = "PreferDualStack";
    pub const REQUIRE_DUAL_STACK: &str = "RequireDualStack";
}

/// LoadBalancerIPMode represents the mode of the LoadBalancer IP.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7073
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum LoadBalancerIPMode {
    /// Traffic is delivered to the node with the destination set to the load-balancer's IP and port.
    #[serde(rename = "VIP")]
    Vip,
    /// Traffic is delivered to the node or pod with the destination set to the node's IP and port or the pod's IP and port.
    #[serde(rename = "Proxy")]
    Proxy,
}

pub mod load_balancer_ip_mode {
    pub const VIP: &str = "VIP";
    pub const PROXY: &str = "Proxy";
}

// ============================================================================
// Container Extended Enums
// ============================================================================

/// ContainerRestartPolicy is the restart policy for a single container.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3214
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum ContainerRestartPolicy {
    /// Always restart the container.
    #[serde(rename = "Always")]
    Always,
    /// Never restart the container.
    #[serde(rename = "Never")]
    Never,
    /// Only restart if the container exits with a non-zero exit code.
    #[serde(rename = "OnFailure")]
    OnFailure,
}

pub mod container_restart_policy {
    pub const ALWAYS: &str = "Always";
    pub const NEVER: &str = "Never";
    pub const ON_FAILURE: &str = "OnFailure";
}

/// PreemptionPolicy describes the policy for preempting pods.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2523
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum PreemptionPolicy {
    /// Pod can preempt other pods with lower priority.
    #[serde(rename = "PreemptLowerPriority")]
    PreemptLowerPriority,
    /// Pod never preempts other pods with lower priority.
    #[serde(rename = "Never")]
    Never,
}

pub mod preemption_policy {
    pub const PREEMPT_LOWER_PRIORITY: &str = "PreemptLowerPriority";
    pub const NEVER: &str = "Never";
}

/// TerminationMessagePolicy describes how termination messages are retrieved from a container.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2533
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum TerminationMessagePolicy {
    /// Set the container status message to the contents of the container's terminationMessagePath.
    #[serde(rename = "File")]
    File,
    /// Read the most recent contents of the container logs for the container status message when the container exits with an error.
    #[serde(rename = "FallbackToLogsOnError")]
    FallbackToLogsOnError,
}

pub mod termination_message_policy {
    pub const FILE: &str = "File";
    pub const FALLBACK_TO_LOGS_ON_ERROR: &str = "FallbackToLogsOnError";
}

// ============================================================================
// Scheduling Extended Enums
// ============================================================================

/// TolerationOperator defines the operator for a toleration.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3641
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum TolerationOperator {
    /// The toleration exists (no key/value check required).
    #[serde(rename = "Exists")]
    Exists,
    /// The toleration requires the key to be equal to the value.
    #[serde(rename = "Equal")]
    Equal,
}

pub mod toleration_operator {
    pub const EXISTS: &str = "Exists";
    pub const EQUAL: &str = "Equal";
}

/// NodeSelectorOperator represents a key's relationship to a set of values.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3345
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum NodeSelectorOperator {
    /// The key's value is in the provided set.
    #[serde(rename = "In")]
    In,
    /// The key's value is not in the provided set.
    #[serde(rename = "NotIn")]
    NotIn,
    /// The key exists.
    #[serde(rename = "Exists")]
    Exists,
    /// The key does not exist.
    #[serde(rename = "DoesNotExist")]
    DoesNotExist,
    /// The key's value is greater than the provided value.
    #[serde(rename = "Gt")]
    Gt,
    /// The key's value is less than the provided value.
    #[serde(rename = "Lt")]
    Lt,
}

pub mod node_selector_operator {
    pub const IN: &str = "In";
    pub const NOT_IN: &str = "NotIn";
    pub const EXISTS: &str = "Exists";
    pub const DOES_NOT_EXIST: &str = "DoesNotExist";
    pub const GT: &str = "Gt";
    pub const LT: &str = "Lt";
}

// ============================================================================
// Security Related Enums
// ============================================================================

/// OSName defines the operating system types.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3960
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum OSName {
    /// Linux operating system.
    #[serde(rename = "linux")]
    Linux,
    /// Windows operating system.
    #[serde(rename = "windows")]
    Windows,
}

pub mod os_name {
    pub const LINUX: &str = "linux";
    pub const WINDOWS: &str = "windows";
}

/// PodFSGroupChangePolicy defines how the volume's ownership and permissions are changed.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4001
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum PodFSGroupChangePolicy {
    /// Change ownership and permissions only when root directory doesn't match.
    #[serde(rename = "OnRootMismatch")]
    OnRootMismatch,
    /// Always change ownership and permissions.
    #[serde(rename = "Always")]
    Always,
}

pub mod pod_fs_group_change_policy {
    pub const ON_ROOT_MISMATCH: &str = "OnRootMismatch";
    pub const ALWAYS: &str = "Always";
}

/// SupplementalGroupsPolicy defines how supplemental groups are calculated.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4017
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum SupplementalGroupsPolicy {
    /// Merge container's groups with image's groups.
    #[serde(rename = "Merge")]
    Merge,
    /// Use only container's groups, ignore image's groups.
    #[serde(rename = "Strict")]
    Strict,
}

pub mod supplemental_groups_policy {
    pub const MERGE: &str = "Merge";
    pub const STRICT: &str = "Strict";
}

/// PodSELinuxChangePolicy defines how the container's SELinux label is applied.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4032
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum PodSELinuxChangePolicy {
    /// Recursive relabeling of all Pod volumes.
    #[serde(rename = "Recursive")]
    Recursive,
    /// Mount all volumes with `-o context` mount option.
    #[serde(rename = "MountOption")]
    MountOption,
}

pub mod pod_selinux_change_policy {
    pub const RECURSIVE: &str = "Recursive";
    pub const MOUNT_OPTION: &str = "MountOption";
}

/// SeccompProfileType defines the seccomp profile types.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4225
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum SeccompProfileType {
    /// No seccomp profile is applied (unconfined).
    #[serde(rename = "Unconfined")]
    Unconfined,
    /// The default container runtime seccomp profile.
    #[serde(rename = "RuntimeDefault")]
    RuntimeDefault,
    /// Custom profile stored on the node's disk.
    #[serde(rename = "Localhost")]
    Localhost,
}

pub mod seccomp_profile_type {
    pub const UNCONFINED: &str = "Unconfined";
    pub const RUNTIME_DEFAULT: &str = "RuntimeDefault";
    pub const LOCALHOST: &str = "Localhost";
}

/// AppArmorProfileType defines the AppArmor profile types.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4256
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum AppArmorProfileType {
    /// No AppArmor profile should be enforced.
    #[serde(rename = "Unconfined")]
    Unconfined,
    /// The container runtime's default AppArmor profile.
    #[serde(rename = "RuntimeDefault")]
    RuntimeDefault,
    /// A profile pre-loaded on the node.
    #[serde(rename = "Localhost")]
    Localhost,
}

pub mod app_armor_profile_type {
    pub const UNCONFINED: &str = "Unconfined";
    pub const RUNTIME_DEFAULT: &str = "RuntimeDefault";
    pub const LOCALHOST: &str = "Localhost";
}

/// ProcMountType defines the /proc mount types.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L6811
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum ProcMountType {
    /// Uses the container runtime defaults for /proc.
    #[serde(rename = "Default")]
    Default,
    /// Bypasses the default masking behavior of the container runtime.
    #[serde(rename = "Unmasked")]
    Unmasked,
}

pub mod proc_mount_type {
    pub const DEFAULT: &str = "Default";
    pub const UNMASKED: &str = "Unmasked";
}

// ============================================================================
// Storage Related Enums
// ============================================================================

/// PersistentVolumeAccessMode defines the access modes for persistent volumes.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L784
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum PersistentVolumeAccessMode {
    /// Can be mounted read/write mode to exactly 1 host.
    #[serde(rename = "ReadWriteOnce")]
    ReadWriteOnce,
    /// Can be mounted in read-only mode to many hosts.
    #[serde(rename = "ReadOnlyMany")]
    ReadOnlyMany,
    /// Can be mounted in read/write mode to many hosts.
    #[serde(rename = "ReadWriteMany")]
    ReadWriteMany,
    /// Can be mounted read/write mode to exactly 1 pod.
    #[serde(rename = "ReadWriteOncePod")]
    ReadWriteOncePod,
}

pub mod persistent_volume_access_mode {
    pub const READ_WRITE_ONCE: &str = "ReadWriteOnce";
    pub const READ_ONLY_MANY: &str = "ReadOnlyMany";
    pub const READ_WRITE_MANY: &str = "ReadWriteMany";
    pub const READ_WRITE_ONCE_POD: &str = "ReadWriteOncePod";
}

/// PersistentVolumeReclaimPolicy describes a policy for end-of-life maintenance of persistent volumes.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L421
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum PersistentVolumeReclaimPolicy {
    /// Recycle the volume (deprecated).
    #[serde(rename = "Recycle")]
    Recycle,
    /// Delete the volume from Kubernetes.
    #[serde(rename = "Delete")]
    Delete,
    /// Leave the volume in its current phase for manual reclamation.
    #[serde(rename = "Retain")]
    Retain,
}

pub mod persistent_volume_reclaim_policy {
    pub const RECYCLE: &str = "Recycle";
    pub const DELETE: &str = "Delete";
    pub const RETAIN: &str = "Retain";
}

/// PersistentVolumeMode describes how a volume is intended to be consumed.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L437
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum PersistentVolumeMode {
    /// Raw block device without filesystem.
    #[serde(rename = "Block")]
    Block,
    /// Formatted with a filesystem.
    #[serde(rename = "Filesystem")]
    Filesystem,
}

pub mod persistent_volume_mode {
    pub const BLOCK: &str = "Block";
    pub const FILESYSTEM: &str = "Filesystem";
}

/// PersistentVolumePhase indicates if a volume is available, bound to a claim, or released.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L800
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum PersistentVolumePhase {
    /// The volume is not available.
    #[serde(rename = "Pending")]
    Pending,
    /// The volume is available and not yet bound.
    #[serde(rename = "Available")]
    Available,
    /// The volume is bound to a claim.
    #[serde(rename = "Bound")]
    Bound,
    /// The bound claim was deleted and volume must be recycled.
    #[serde(rename = "Released")]
    Released,
    /// The volume failed to be correctly recycled or deleted.
    #[serde(rename = "Failed")]
    Failed,
}

pub mod persistent_volume_phase {
    pub const PENDING: &str = "Pending";
    pub const AVAILABLE: &str = "Available";
    pub const BOUND: &str = "Bound";
    pub const RELEASED: &str = "Released";
    pub const FAILED: &str = "Failed";
}

/// PersistentVolumeClaimPhase defines the phase of a persistent volume claim.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L820
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum PersistentVolumeClaimPhase {
    /// The claim is not yet bound.
    #[serde(rename = "Pending")]
    Pending,
    /// The claim is bound.
    #[serde(rename = "Bound")]
    Bound,
    /// The claim lost its underlying persistent volume.
    #[serde(rename = "Lost")]
    Lost,
}

pub mod persistent_volume_claim_phase {
    pub const PENDING: &str = "Pending";
    pub const BOUND: &str = "Bound";
    pub const LOST: &str = "Lost";
}

/// HostPathType defines the type of host path for persistent volumes.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L835
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum HostPathType {
    /// Empty default value for backwards compatibility.
    #[serde(rename = "")]
    Unset,
    /// Create directory if it doesn't exist.
    #[serde(rename = "DirectoryOrCreate")]
    DirectoryOrCreate,
    /// A directory must exist at the given path.
    #[serde(rename = "Directory")]
    Directory,
    /// Create file if it doesn't exist.
    #[serde(rename = "FileOrCreate")]
    FileOrCreate,
    /// A file must exist at the given path.
    #[serde(rename = "File")]
    File,
    /// A UNIX socket must exist at the given path.
    #[serde(rename = "Socket")]
    Socket,
    /// A character device must exist at the given path.
    #[serde(rename = "CharDevice")]
    CharDevice,
    /// A block device must exist at the given path.
    #[serde(rename = "BlockDevice")]
    BlockDevice,
}

pub mod host_path_type {
    pub const UNSET: &str = "";
    pub const DIRECTORY_OR_CREATE: &str = "DirectoryOrCreate";
    pub const DIRECTORY: &str = "Directory";
    pub const FILE_OR_CREATE: &str = "FileOrCreate";
    pub const FILE: &str = "File";
    pub const SOCKET: &str = "Socket";
    pub const CHAR_DEVICE: &str = "CharDevice";
    pub const BLOCK_DEVICE: &str = "BlockDevice";
}

/// StorageMedium defines the storage medium types.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L890
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum StorageMedium {
    /// Use whatever the default is for the node.
    #[serde(rename = "")]
    #[default]
    Default,
    /// Use memory (tmpfs).
    #[serde(rename = "Memory")]
    Memory,
    /// Use hugepages.
    #[serde(rename = "HugePages")]
    HugePages,
    /// Prefix for full medium notation HugePages-<size>.
    #[serde(rename = "HugePages-")]
    HugePagesPrefix,
}

pub mod storage_medium {
    pub const DEFAULT: &str = "";
    pub const MEMORY: &str = "Memory";
    pub const HUGE_PAGES: &str = "HugePages";
    pub const HUGE_PAGES_PREFIX: &str = "HugePages-";
}

/// URIScheme defines the URI scheme for HTTP gets.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2414
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum URIScheme {
    /// HTTP scheme.
    #[serde(rename = "HTTP")]
    Http,
    /// HTTPS scheme.
    #[serde(rename = "HTTPS")]
    Https,
}

pub mod uri_scheme {
    pub const HTTP: &str = "HTTP";
    pub const HTTPS: &str = "HTTPS";
}

/// MountPropagationMode describes the propagation mode for mounts.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2185
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum MountPropagationMode {
    /// No mount propagation.
    #[serde(rename = "None")]
    None,
    /// Receive mounts from host to container.
    #[serde(rename = "HostToContainer")]
    HostToContainer,
    /// Bidirectional mount propagation.
    #[serde(rename = "Bidirectional")]
    Bidirectional,
}

pub mod mount_propagation_mode {
    pub const NONE: &str = "None";
    pub const HOST_TO_CONTAINER: &str = "HostToContainer";
    pub const BIDIRECTIONAL: &str = "Bidirectional";
}

/// RecursiveReadOnlyMode describes recursive-readonly mode.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2210
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum RecursiveReadOnlyMode {
    /// Recursive-readonly mode is disabled.
    #[serde(rename = "Disabled")]
    Disabled,
    /// Enable recursive-readonly mode if possible.
    #[serde(rename = "IfPossible")]
    IfPossible,
    /// Enable recursive-readonly mode, or raise an error.
    #[serde(rename = "Enabled")]
    Enabled,
}

pub mod recursive_read_only_mode {
    pub const DISABLED: &str = "Disabled";
    pub const IF_POSSIBLE: &str = "IfPossible";
    pub const ENABLED: &str = "Enabled";
}

/// ResourceResizeRestartPolicy specifies how to handle container resource resize.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2496
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum ResourceResizeRestartPolicy {
    /// Try to resize without restarting if possible.
    #[serde(rename = "NotRequired")]
    NotRequired,
    /// Restart the container when resizing.
    #[serde(rename = "RestartContainer")]
    RestartContainer,
}

pub mod resource_resize_restart_policy {
    pub const NOT_REQUIRED: &str = "NotRequired";
    pub const RESTART_CONTAINER: &str = "RestartContainer";
}

/// PodResizeStatus describes the status of pod resource resize.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3171
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum PodResizeStatus {
    /// Pod resize is being actuated.
    #[serde(rename = "InProgress")]
    InProgress,
    /// Node cannot resize the pod at this time and will keep retrying.
    #[serde(rename = "Deferred")]
    Deferred,
    /// Requested pod resize is not feasible.
    #[serde(rename = "Infeasible")]
    Infeasible,
}

pub mod pod_resize_status {
    pub const IN_PROGRESS: &str = "InProgress";
    pub const DEFERRED: &str = "Deferred";
    pub const INFEASIBLE: &str = "Infeasible";
}

/// AzureDataDiskCachingMode defines the caching mode for Azure data disks.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L1588
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum AzureDataDiskCachingMode {
    /// No caching.
    #[serde(rename = "None")]
    None,
    /// Read-only caching.
    #[serde(rename = "ReadOnly")]
    ReadOnly,
    /// Read-write caching.
    #[serde(rename = "ReadWrite")]
    ReadWrite,
}

pub mod azure_data_disk_caching_mode {
    pub const NONE: &str = "None";
    pub const READ_ONLY: &str = "ReadOnly";
    pub const READ_WRITE: &str = "ReadWrite";
}

/// AzureDataDiskKind defines the kind of Azure data disk.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L1591
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum AzureDataDiskKind {
    /// Shared blob disk.
    #[serde(rename = "Shared")]
    Shared,
    /// Dedicated blob disk.
    #[serde(rename = "Dedicated")]
    Dedicated,
    /// Managed disk.
    #[serde(rename = "Managed")]
    Managed,
}

pub mod azure_data_disk_kind {
    pub const SHARED: &str = "Shared";
    pub const DEDICATED: &str = "Dedicated";
    pub const MANAGED: &str = "Managed";
}

/// LimitType defines the type of limit for LimitRange.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L6250
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum LimitType {
    /// Limit applies to all pods.
    #[serde(rename = "Pod")]
    Pod,
    /// Limit applies to all containers.
    #[serde(rename = "Container")]
    #[default]
    Container,
    /// Limit applies to all persistent volume claims.
    #[serde(rename = "PersistentVolumeClaim")]
    PersistentVolumeClaim,
}

pub mod limit_type {
    pub const POD: &str = "Pod";
    pub const CONTAINER: &str = "Container";
    pub const PERSISTENT_VOLUME_CLAIM: &str = "PersistentVolumeClaim";
}

/// ResourceQuotaScope defines the set of scopes for a ResourceQuota.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L6362
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum ResourceQuotaScope {
    /// Match pods where spec.activeDeadlineSeconds >= 0.
    #[serde(rename = "Terminating")]
    Terminating,
    /// Match pods where spec.activeDeadlineSeconds is nil.
    #[serde(rename = "NotTerminating")]
    NotTerminating,
    /// Match pods with best effort QoS.
    #[serde(rename = "BestEffort")]
    #[default]
    BestEffort,
    /// Match pods without best effort QoS.
    #[serde(rename = "NotBestEffort")]
    NotBestEffort,
    /// Match pods with priority class mentioned.
    #[serde(rename = "PriorityClass")]
    PriorityClass,
    /// Match pods with cross-namespace pod affinity.
    #[serde(rename = "CrossNamespacePodAffinity")]
    CrossNamespacePodAffinity,
    /// Match PVCs with volume attributes class mentioned.
    #[serde(rename = "VolumeAttributesClass")]
    VolumeAttributesClass,
}

pub mod resource_quota_scope {
    pub const TERMINATING: &str = "Terminating";
    pub const NOT_TERMINATING: &str = "NotTerminating";
    pub const BEST_EFFORT: &str = "BestEffort";
    pub const NOT_BEST_EFFORT: &str = "NotBestEffort";
    pub const PRIORITY_CLASS: &str = "PriorityClass";
    pub const CROSS_NAMESPACE_POD_AFFINITY: &str = "CrossNamespacePodAffinity";
    pub const VOLUME_ATTRIBUTES_CLASS: &str = "VolumeAttributesClass";
}

/// SecretType defines the type of secret.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L6504
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum SecretType {
    /// Arbitrary user-defined data (default).
    #[serde(rename = "Opaque")]
    #[default]
    Opaque,
    /// Service account token.
    #[serde(rename = "kubernetes.io/service-account-token")]
    ServiceAccountToken,
    /// Docker config file.
    #[serde(rename = "kubernetes.io/dockercfg")]
    Dockercfg,
    /// Docker config file in JSON format.
    #[serde(rename = "kubernetes.io/dockerconfigjson")]
    DockerConfigJson,
    /// Basic authentication credentials.
    #[serde(rename = "kubernetes.io/basic-auth")]
    BasicAuth,
    /// SSH authentication credentials.
    #[serde(rename = "kubernetes.io/ssh-auth")]
    SshAuth,
    /// TLS credentials.
    #[serde(rename = "kubernetes.io/tls")]
    Tls,
    /// Bootstrap token.
    #[serde(rename = "bootstrap.kubernetes.io/token")]
    BootstrapToken,
}

pub mod secret_type {
    pub const OPAQUE: &str = "Opaque";
    pub const SERVICE_ACCOUNT_TOKEN: &str = "kubernetes.io/service-account-token";
    pub const DOCKERCFG: &str = "kubernetes.io/dockercfg";
    pub const DOCKER_CONFIG_JSON: &str = "kubernetes.io/dockerconfigjson";
    pub const BASIC_AUTH: &str = "kubernetes.io/basic-auth";
    pub const SSH_AUTH: &str = "kubernetes.io/ssh-auth";
    pub const TLS: &str = "kubernetes.io/tls";
    pub const BOOTSTRAP_TOKEN: &str = "bootstrap.kubernetes.io/token";
}

/// ReplicationControllerConditionType defines the condition type for replication controllers.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4691
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum ReplicationControllerConditionType {
    /// Replica failure condition.
    #[serde(rename = "ReplicaFailure")]
    ReplicaFailure,
}

pub mod replication_controller_condition_type {
    pub const REPLICA_FAILURE: &str = "ReplicaFailure";
}

/// ComponentConditionType defines the condition type for components.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L6688
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum ComponentConditionType {
    /// Component is healthy.
    #[serde(rename = "Healthy")]
    #[default]
    Healthy,
}

pub mod component_condition_type {
    pub const HEALTHY: &str = "Healthy";
}

/// Information about the condition of a component.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7994
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComponentCondition {
    /// Type of condition for a component.
    #[serde(default)]
    pub r#type: ComponentConditionType,
    /// Status of the condition for a component.
    #[serde(default)]
    pub status: String,
    /// Message about the condition for a component.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
    /// Condition error code for a component.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub error: String,
}

/// Signal defines a system signal for container lifecycle hooks.
///
/// Source: https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2781
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Signal {
    /// SIGABRT - Abort.
    #[serde(rename = "SIGABRT")]
    Sigabrt,
    /// SIGALRM - Alarm clock.
    #[serde(rename = "SIGALRM")]
    Sigalrm,
    /// SIGBUS - Bus error.
    #[serde(rename = "SIGBUS")]
    Sigbus,
    /// SIGCHLD - Child status changed.
    #[serde(rename = "SIGCHLD")]
    Sigchld,
    /// SIGCLD - Child status changed (alternative).
    #[serde(rename = "SIGCLD")]
    Sigcld,
    /// SIGCONT - Continue executing.
    #[serde(rename = "SIGCONT")]
    Sigcont,
    /// SIGFPE - Floating-point exception.
    #[serde(rename = "SIGFPE")]
    Sigfpe,
    /// SIGHUP - Hangup detected on controlling terminal.
    #[serde(rename = "SIGHUP")]
    Sighup,
    /// SIGILL - Illegal instruction.
    #[serde(rename = "SIGILL")]
    Sigill,
    /// SIGINT - Interrupt from keyboard.
    #[serde(rename = "SIGINT")]
    Sigint,
    /// SIGIO - I/O now possible.
    #[serde(rename = "SIGIO")]
    Sigio,
    /// SIGIOT - IOT trap (alternative for SIGABRT).
    #[serde(rename = "SIGIOT")]
    Sigiot,
    /// SIGKILL - Kill signal (cannot be caught or ignored).
    #[serde(rename = "SIGKILL")]
    Sigkill,
    /// SIGPIPE - Broken pipe.
    #[serde(rename = "SIGPIPE")]
    Sigpipe,
    /// SIGPOLL - Pollable event occurred.
    #[serde(rename = "SIGPOLL")]
    Sigpoll,
    /// SIGPROF - Profiling timer expired.
    #[serde(rename = "SIGPROF")]
    Sigprof,
    /// SIGPWR - Power failure restart.
    #[serde(rename = "SIGPWR")]
    Sigpwr,
    /// SIGQUIT - Quit from keyboard.
    #[serde(rename = "SIGQUIT")]
    Sigquit,
    /// SIGSEGV - Segmentation violation.
    #[serde(rename = "SIGSEGV")]
    Sigsegv,
    /// SIGSTKFLT - Stack fault on coprocessor.
    #[serde(rename = "SIGSTKFLT")]
    Sigstkflt,
    /// SIGSTOP - Stop process (cannot be caught or ignored).
    #[serde(rename = "SIGSTOP")]
    Sigstop,
    /// SIGSYS - Bad system call.
    #[serde(rename = "SIGSYS")]
    Sigsys,
    /// SIGTERM - Termination signal.
    #[serde(rename = "SIGTERM")]
    Sigterm,
    /// SIGTRAP - Trace/breakpoint trap.
    #[serde(rename = "SIGTRAP")]
    Sigtrap,
    /// SIGTSTP - Stop typed at terminal.
    #[serde(rename = "SIGTSTP")]
    Sigtstp,
    /// SIGTTIN - Background read from terminal.
    #[serde(rename = "SIGTTIN")]
    Sigttin,
    /// SIGTTOU - Background write to terminal.
    #[serde(rename = "SIGTTOU")]
    Sigttou,
    /// SIGURG - Urgent condition on socket.
    #[serde(rename = "SIGURG")]
    Sigurg,
    /// SIGUSR1 - User-defined signal 1.
    #[serde(rename = "SIGUSR1")]
    Sigusr1,
    /// SIGUSR2 - User-defined signal 2.
    #[serde(rename = "SIGUSR2")]
    Sigusr2,
    /// SIGVTALRM - Virtual alarm clock.
    #[serde(rename = "SIGVTALRM")]
    Sigvtalrm,
    /// SIGWINCH - Window resize signal.
    #[serde(rename = "SIGWINCH")]
    Sigwinch,
    /// SIGXCPU - CPU time limit exceeded.
    #[serde(rename = "SIGXCPU")]
    Sigxcpu,
    /// SIGXFSZ - File size limit exceeded.
    #[serde(rename = "SIGXFSZ")]
    Sigxfsz,
    /// SIGRTMIN - First real-time signal.
    #[serde(rename = "SIGRTMIN")]
    Sigrtmin,
    /// SIGRTMIN+1
    #[serde(rename = "SIGRTMIN+1")]
    SigrtminPlus1,
    /// SIGRTMIN+2
    #[serde(rename = "SIGRTMIN+2")]
    SigrtminPlus2,
    /// SIGRTMIN+3
    #[serde(rename = "SIGRTMIN+3")]
    SigrtminPlus3,
    /// SIGRTMIN+4
    #[serde(rename = "SIGRTMIN+4")]
    SigrtminPlus4,
    /// SIGRTMIN+5
    #[serde(rename = "SIGRTMIN+5")]
    SigrtminPlus5,
    /// SIGRTMIN+6
    #[serde(rename = "SIGRTMIN+6")]
    SigrtminPlus6,
    /// SIGRTMIN+7
    #[serde(rename = "SIGRTMIN+7")]
    SigrtminPlus7,
    /// SIGRTMIN+8
    #[serde(rename = "SIGRTMIN+8")]
    SigrtminPlus8,
    /// SIGRTMIN+9
    #[serde(rename = "SIGRTMIN+9")]
    SigrtminPlus9,
    /// SIGRTMIN+10
    #[serde(rename = "SIGRTMIN+10")]
    SigrtminPlus10,
    /// SIGRTMIN+11
    #[serde(rename = "SIGRTMIN+11")]
    SigrtminPlus11,
    /// SIGRTMIN+12
    #[serde(rename = "SIGRTMIN+12")]
    SigrtminPlus12,
    /// SIGRTMIN+13
    #[serde(rename = "SIGRTMIN+13")]
    SigrtminPlus13,
    /// SIGRTMIN+14
    #[serde(rename = "SIGRTMIN+14")]
    SigrtminPlus14,
    /// SIGRTMIN+15
    #[serde(rename = "SIGRTMIN+15")]
    SigrtminPlus15,
    /// SIGRTMAX-14
    #[serde(rename = "SIGRTMAX-14")]
    SigrtmaxMinus14,
    /// SIGRTMAX-13
    #[serde(rename = "SIGRTMAX-13")]
    SigrtmaxMinus13,
    /// SIGRTMAX-12
    #[serde(rename = "SIGRTMAX-12")]
    SigrtmaxMinus12,
    /// SIGRTMAX-11
    #[serde(rename = "SIGRTMAX-11")]
    SigrtmaxMinus11,
    /// SIGRTMAX-10
    #[serde(rename = "SIGRTMAX-10")]
    SigrtmaxMinus10,
    /// SIGRTMAX-9
    #[serde(rename = "SIGRTMAX-9")]
    SigrtmaxMinus9,
    /// SIGRTMAX-8
    #[serde(rename = "SIGRTMAX-8")]
    SigrtmaxMinus8,
    /// SIGRTMAX-7
    #[serde(rename = "SIGRTMAX-7")]
    SigrtmaxMinus7,
    /// SIGRTMAX-6
    #[serde(rename = "SIGRTMAX-6")]
    SigrtmaxMinus6,
    /// SIGRTMAX-5
    #[serde(rename = "SIGRTMAX-5")]
    SigrtmaxMinus5,
    /// SIGRTMAX-4
    #[serde(rename = "SIGRTMAX-4")]
    SigrtmaxMinus4,
    /// SIGRTMAX-3
    #[serde(rename = "SIGRTMAX-3")]
    SigrtmaxMinus3,
    /// SIGRTMAX-2
    #[serde(rename = "SIGRTMAX-2")]
    SigrtmaxMinus2,
    /// SIGRTMAX-1
    #[serde(rename = "SIGRTMAX-1")]
    SigrtmaxMinus1,
    /// SIGRTMAX - Last real-time signal.
    #[serde(rename = "SIGRTMAX")]
    Sigrtmax,
}

pub mod signal {
    pub const SIGABRT: &str = "SIGABRT";
    pub const SIGALRM: &str = "SIGALRM";
    pub const SIGBUS: &str = "SIGBUS";
    pub const SIGCHLD: &str = "SIGCHLD";
    pub const SIGCLD: &str = "SIGCLD";
    pub const SIGCONT: &str = "SIGCONT";
    pub const SIGFPE: &str = "SIGFPE";
    pub const SIGHUP: &str = "SIGHUP";
    pub const SIGILL: &str = "SIGILL";
    pub const SIGINT: &str = "SIGINT";
    pub const SIGIO: &str = "SIGIO";
    pub const SIGIOT: &str = "SIGIOT";
    pub const SIGKILL: &str = "SIGKILL";
    pub const SIGPIPE: &str = "SIGPIPE";
    pub const SIGPOLL: &str = "SIGPOLL";
    pub const SIGPROF: &str = "SIGPROF";
    pub const SIGPWR: &str = "SIGPWR";
    pub const SIGQUIT: &str = "SIGQUIT";
    pub const SIGSEGV: &str = "SIGSEGV";
    pub const SIGSTKFLT: &str = "SIGSTKFLT";
    pub const SIGSTOP: &str = "SIGSTOP";
    pub const SIGSYS: &str = "SIGSYS";
    pub const SIGTERM: &str = "SIGTERM";
    pub const SIGTRAP: &str = "SIGTRAP";
    pub const SIGTSTP: &str = "SIGTSTP";
    pub const SIGTTIN: &str = "SIGTTIN";
    pub const SIGTTOU: &str = "SIGTTOU";
    pub const SIGURG: &str = "SIGURG";
    pub const SIGUSR1: &str = "SIGUSR1";
    pub const SIGUSR2: &str = "SIGUSR2";
    pub const SIGVTALRM: &str = "SIGVTALRM";
    pub const SIGWINCH: &str = "SIGWINCH";
    pub const SIGXCPU: &str = "SIGXCPU";
    pub const SIGXFSZ: &str = "SIGXFSZ";
    pub const SIGRTMIN: &str = "SIGRTMIN";
    pub const SIGRTMIN_PLUS_1: &str = "SIGRTMIN+1";
    pub const SIGRTMIN_PLUS_2: &str = "SIGRTMIN+2";
    pub const SIGRTMIN_PLUS_3: &str = "SIGRTMIN+3";
    pub const SIGRTMIN_PLUS_4: &str = "SIGRTMIN+4";
    pub const SIGRTMIN_PLUS_5: &str = "SIGRTMIN+5";
    pub const SIGRTMIN_PLUS_6: &str = "SIGRTMIN+6";
    pub const SIGRTMIN_PLUS_7: &str = "SIGRTMIN+7";
    pub const SIGRTMIN_PLUS_8: &str = "SIGRTMIN+8";
    pub const SIGRTMIN_PLUS_9: &str = "SIGRTMIN+9";
    pub const SIGRTMIN_PLUS_10: &str = "SIGRTMIN+10";
    pub const SIGRTMIN_PLUS_11: &str = "SIGRTMIN+11";
    pub const SIGRTMIN_PLUS_12: &str = "SIGRTMIN+12";
    pub const SIGRTMIN_PLUS_13: &str = "SIGRTMIN+13";
    pub const SIGRTMIN_PLUS_14: &str = "SIGRTMIN+14";
    pub const SIGRTMIN_PLUS_15: &str = "SIGRTMIN+15";
    pub const SIGRTMAX_MINUS_14: &str = "SIGRTMAX-14";
    pub const SIGRTMAX_MINUS_13: &str = "SIGRTMAX-13";
    pub const SIGRTMAX_MINUS_12: &str = "SIGRTMAX-12";
    pub const SIGRTMAX_MINUS_11: &str = "SIGRTMAX-11";
    pub const SIGRTMAX_MINUS_10: &str = "SIGRTMAX-10";
    pub const SIGRTMAX_MINUS_9: &str = "SIGRTMAX-9";
    pub const SIGRTMAX_MINUS_8: &str = "SIGRTMAX-8";
    pub const SIGRTMAX_MINUS_7: &str = "SIGRTMAX-7";
    pub const SIGRTMAX_MINUS_6: &str = "SIGRTMAX-6";
    pub const SIGRTMAX_MINUS_5: &str = "SIGRTMAX-5";
    pub const SIGRTMAX_MINUS_4: &str = "SIGRTMAX-4";
    pub const SIGRTMAX_MINUS_3: &str = "SIGRTMAX-3";
    pub const SIGRTMAX_MINUS_2: &str = "SIGRTMAX-2";
    pub const SIGRTMAX_MINUS_1: &str = "SIGRTMAX-1";
    pub const SIGRTMAX: &str = "SIGRTMAX";
}

// ============================================================================
// Phase 2: Simple Struct Modules
// ============================================================================

pub mod affinity;
pub mod binding;
pub mod component_status;
pub mod config;
pub mod container;
pub mod endpoints;
pub mod env;
pub mod event;
pub mod helper;
pub mod namespace;
pub mod network;
pub mod node;
pub mod persistent_volume;
pub mod pod;
pub mod pod_resources;
pub mod pod_status_result;
pub mod quota;
pub mod replication_controller;
pub mod resource;
pub mod scheduling;
pub mod security;
pub mod selector;
pub mod service;
pub mod validation;
pub mod volume;

// Re-exports for Phase 2 modules
pub use affinity::{
    Affinity, NodeAffinity, PodAffinity, PodAffinityTerm, PodAntiAffinity, PreferredSchedulingTerm,
    WeightedPodAffinityTerm,
};
pub use binding::{Binding, SecretReference};
pub use config::{
    ConfigMap, ConfigMapList, ObjectReference as ConfigObjectReference, Secret, SecretList,
    ServiceAccount, ServiceAccountList,
};
pub use container::{
    ContainerPort, ContainerState, ContainerStateRunning, ContainerStateTerminated,
    ContainerStateWaiting, ContainerStatus, ExecAction, Lifecycle, LifecycleHandler,
    ResourceHealth, ResourceHealthStatus, ResourceID, ResourceStatus,
};
pub use endpoints::{
    EndpointAddress, EndpointPort, EndpointSubset, Endpoints, EndpointsList, ObjectReference,
};
pub use env::{ConfigMapEnvSource, EnvFromSource, EnvVar, EnvVarSource, SecretEnvSource};
pub use event::{Event, EventList, EventSeries, EventSource};
pub use helper::{
    ByteString, GRPCAction, NodeProxyOptions, PodAttachOptions, PodExecOptions, PodLogOptions,
    PodPortForwardOptions, PodProxyOptions, Preconditions, RangeAllocation, SerializedReference,
    ServiceProxyOptions, TypedLocalObjectReference,
};
pub use namespace::{Namespace, NamespaceCondition, NamespaceList, NamespaceSpec, NamespaceStatus};
pub use network::{HTTPGetAction, HTTPHeader, TCPSocketAction};
pub use node::{
    AttachedVolume, AvoidPods, ConfigMapNodeConfigSource, ContainerImage, DaemonEndpoint, Node,
    NodeAddress, NodeCondition, NodeConfigSource, NodeConfigStatus, NodeDaemonEndpoints,
    NodeFeatures, NodeList, NodeRuntimeHandler, NodeRuntimeHandlerFeatures, NodeSpec, NodeStatus,
    NodeSwapStatus, NodeSystemInfo, PodSignature, PreferAvoidPodsEntry,
};
pub use persistent_volume::{
    AzureFilePersistentVolumeSource, CSIPersistentVolumeSource, CephFSPersistentVolumeSource,
    CinderPersistentVolumeSource, FlexPersistentVolumeSource, GlusterfsPersistentVolumeSource,
    ISCSIPersistentVolumeSource, PersistentVolume, PersistentVolumeClaim,
    PersistentVolumeClaimList, PersistentVolumeClaimSpec, PersistentVolumeClaimStatus,
    PersistentVolumeList, PersistentVolumeSource, PersistentVolumeSpec, PersistentVolumeStatus,
    RBDPersistentVolumeSource, ScaleIOPersistentVolumeSource, StorageOSPersistentVolumeSource,
    TypedObjectReferencePV, VolumeNodeAffinity,
};
pub use pod::{
    Container as InternalContainer, HostAlias, HostIP, Pod, PodCondition, PodIP, PodList,
    PodReadinessGate as InternalPodReadinessGate, PodSpec, PodStatus, PodTemplate, PodTemplateList,
    PodTemplateSpec, TopologySpreadConstraint as InternalTopologySpreadConstraint,
};
pub use pod_resources::{
    ContainerResizePolicy, ContainerUser, LinuxContainerUser, PodResourceClaim,
    PodResourceClaimStatus,
};
pub use quota::{
    LimitRange, LimitRangeItem, LimitRangeList, LimitRangeSpec, ResourceQuota, ResourceQuotaList,
    ResourceQuotaSpec, ResourceQuotaStatus, ScopeSelector, ScopeSelectorOperator,
    ScopedResourceSelectorRequirement,
};
pub use replication_controller::{
    ReplicationController, ReplicationControllerCondition, ReplicationControllerList,
    ReplicationControllerSpec, ReplicationControllerStatus,
};
pub use resource::{PortStatus, ResourceList, ResourceRequirements};
pub use scheduling::{
    PodDNSConfig, PodDNSConfigOption, PodOS, PodSchedulingGate, Taint, Toleration,
};
pub use security::{
    AppArmorProfile, Capabilities, PodSecurityContext, SELinuxOptions, SeccompProfile,
    SecurityContext, Sysctl, WindowsSecurityContextOptions,
};
pub use selector::{
    ConfigMapKeySelector, FileKeySelector, LabelSelector, LabelSelectorRequirement,
    LocalObjectReference, NodeSelector, NodeSelectorRequirement, NodeSelectorTerm,
    ObjectFieldSelector, ResourceFieldSelector, SecretKeySelector,
};
pub use service::{
    ClientIPConfig, LoadBalancerIngress, LoadBalancerStatus, PortStatus as ServicePortStatus,
    Service, ServiceList, ServicePort, ServiceSpec, ServiceStatus, SessionAffinityConfig,
};
// SessionAffinityType is an alias to ServiceAffinity
pub type SessionAffinityType = ServiceAffinity;
pub use component_status::ComponentStatus;
pub use component_status::ComponentStatusList;
pub use pod_status_result::PodStatusResult;
pub use volume::{
    AWSElasticBlockStoreVolumeSource, AzureDiskVolumeSource, AzureFileVolumeSource,
    CSIVolumeSource, CephFSVolumeSource, CinderVolumeSource, ClusterTrustBundleProjection,
    ConfigMapProjection, ConfigMapVolumeSource, DownwardAPIProjection, DownwardAPIVolumeFile,
    DownwardAPIVolumeSource, EphemeralVolumeSource, FCVolumeSource, FlexVolumeSource,
    FlockerVolumeSource, GCEPersistentDiskVolumeSource, GitRepoVolumeSource, GlusterfsVolumeSource,
    HostPathVolumeSource, ISCSIVolumeSource, ImageVolumeSource, LocalVolumeSource, NFSVolumeSource,
    PersistentVolumeClaimTemplate, PersistentVolumeClaimVolumeSource,
    PhotonPersistentDiskVolumeSource, PodCertificateProjection, PortworxVolumeSource,
    ProjectedVolumeSource, QuobyteVolumeSource, RBDVolumeSource, ScaleIOVolumeSource,
    SecretProjection, SecretVolumeSource, ServiceAccountTokenProjection, StorageOSVolumeSource,
    Volume, VolumeDevice, VolumeMount, VolumeMountStatus, VolumeProjection, VolumeSource,
};

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod trait_tests_cluster;
#[cfg(test)]
mod trait_tests_config;
#[cfg(test)]
mod trait_tests_misc;
#[cfg(test)]
mod trait_tests_quota;
#[cfg(test)]
mod trait_tests_service;
#[cfg(test)]
mod trait_tests_storage;
#[cfg(test)]
mod trait_tests_workload;
