//! Pod types from the Kubernetes Core v1 API
//!
//! This module contains the Pod type and its associated spec and status types.

use crate::common::{
    ApplyDefault, HasTypeMeta, ListMeta, ObjectMeta, ResourceSchema, Timestamp, TypeMeta,
    VersionedObject,
};
use crate::core::v1::affinity::Affinity;
use crate::core::v1::env::{EnvFromSource, EnvVar};
use crate::core::v1::pod_resources::{PodResourceClaim, PodResourceClaimStatus};
use crate::core::v1::probe::{Lifecycle, Probe};
use crate::core::v1::reference::LocalObjectReference;
use crate::core::v1::resource::ResourceRequirements;
use crate::core::v1::security::{PodSecurityContext, SecurityContext};
use crate::core::v1::toleration::Toleration;
use crate::core::v1::volume::{Volume, VolumeDevice, VolumeMount};
use crate::impl_unimplemented_prost_message;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Pod is a collection of containers that can run on a host.
///
/// Corresponds to [Kubernetes Pod](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5469)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Pod {
    /// TypeMeta describes the type of this object
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// Specification of the desired behavior of the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<PodSpec>,

    /// Most recently observed status of the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PodStatus>,
}

/// PodList is a list of Pods.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodList {
    /// TypeMeta describes the type of this object
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// List of pods.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Pod>,
}

/// PodSpec is a description of a pod.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodSpec {
    /// List of containers belonging to the pod.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub containers: Vec<Container>,

    /// List of initialization containers belonging to the pod.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub init_containers: Vec<Container>,

    /// Restart policy for all containers within the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restart_policy: Option<String>,

    /// Optional duration in seconds the pod needs to terminate gracefully.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub termination_grace_period_seconds: Option<i64>,

    /// Optional duration in seconds the pod may be active on the node relative to StartTime.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_deadline_seconds: Option<i64>,

    /// DNS policy for containers within the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dns_policy: Option<String>,

    /// Pod DNS configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dns_config: Option<PodDNSConfig>,

    /// NodeSelector is a selector which must be true for the pod to fit on a node.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub node_selector: BTreeMap<String, String>,

    /// ServiceAccountName is the name of the ServiceAccount to use to run this pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_account_name: Option<String>,

    /// AutomountServiceAccountToken indicates whether a service account token should be automatically mounted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub automount_service_account_token: Option<bool>,

    /// NodeName indicates in which node this pod is scheduled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,

    /// Host networking requested for this pod.
    #[serde(default)]
    pub host_network: bool,

    /// Use the host's pid namespace.
    #[serde(default)]
    pub host_pid: bool,

    /// Use the host's ipc namespace.
    #[serde(default)]
    pub host_ipc: bool,

    /// Share a single process namespace between all of the containers in a pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub share_process_namespace: Option<bool>,

    /// SecurityContext holds pod-level security attributes and common container settings.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_context: Option<PodSecurityContext>,

    /// ImagePullSecrets is an optional list of references to secrets in the same namespace.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub image_pull_secrets: Vec<LocalObjectReference>,

    /// Specifies the hostname of the Pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,

    /// If specified, the fully qualified Pod hostname will be "<hostname>.<subdomain>.<pod namespace>.svc.<cluster domain>".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subdomain: Option<String>,

    /// If specified, the pod's scheduling constraints.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub affinity: Option<Affinity>,

    /// If specified, the pod will be dispatched by specified scheduler.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheduler_name: Option<String>,

    /// If specified, the pod's tolerations.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tolerations: Vec<Toleration>,

    /// HostAliases is an optional list of hosts and IPs that will be injected into the pod's hosts file.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub host_aliases: Vec<HostAlias>,

    /// If specified, indicates the pod's priority.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority_class_name: Option<String>,

    /// The priority value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,

    /// If specified, all readiness gates will be evaluated for pod readiness.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub readiness_gates: Vec<PodReadinessGate>,

    /// RuntimeClassName refers to a RuntimeClass object in the node.k8s.io group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runtime_class_name: Option<String>,

    /// EnableServiceLinks indicates whether information about services should be injected into pod's environment variables.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable_service_links: Option<bool>,

    /// Specifies the OS of the containers in the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub os: Option<PodOS>,

    /// Use the host's user namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_users: Option<bool>,

    /// SchedulingGates is an opaque list of values that if specified will block scheduling the pod.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scheduling_gates: Vec<PodSchedulingGate>,

    /// List of volumes that can be mounted by containers belonging to the pod.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volumes: Vec<Volume>,

    /// ResourceClaims defines which ResourceClaims must be allocated.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resource_claims: Vec<PodResourceClaim>,

    /// Resources is the total amount of CPU and Memory resources required by all containers in the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overhead: Option<ResourceRequirements>,
}

/// HostIP represents an IP address of a host.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct HostIP {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub ip: String,
}

/// PodIP represents an IP address of a pod.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodIP {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub ip: String,
}

/// PodStatus represents information about the status of a pod.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodStatus {
    /// Current phase of the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,

    /// IP address of the host to which the pod is assigned.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_ip: Option<String>,

    /// IP address allocated to the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pod_ip: Option<String>,

    /// hostIPs holds the IP addresses of the host.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub host_ips: Vec<HostIP>,

    /// Current service state of pod.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<PodCondition>,

    /// The list has one entry per container in the manifest.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub container_statuses: Vec<ContainerStatus>,

    /// The list has one entry per init container in the manifest.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub init_container_statuses: Vec<ContainerStatus>,

    /// The QOS class assigned to the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qos_class: Option<String>,

    /// Time at which the pod was scheduled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<Timestamp>,

    /// IP address of the pod for this host (only for host network pods).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pod_ips: Vec<PodIP>,

    /// Reason for the current pod status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    /// Human-readable message indicating details about why the pod is in this condition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// The list has one entry per ephemeral container.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ephemeral_container_statuses: Vec<ContainerStatus>,

    /// Status of resource claims.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resource_claim_statuses: Vec<PodResourceClaimStatus>,

    /// Status for any resize operations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resize: Option<String>,
}

/// PodCondition contains details for the current state of this pod.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PodCondition {
    /// Type is the type of the condition.
    #[serde(rename = "type")]
    pub type_: String,

    /// Status is the status of the condition.
    pub status: String,

    /// Last time we probed the condition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_probe_time: Option<Timestamp>,

    /// Last time the condition transitioned from one status to another.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<Timestamp>,

    /// Unique, this should be a short, machine understandable string that gives the reason
    /// for condition's last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    /// Human-readable message indicating details about last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// PodDNSConfig defines the DNS parameters of a pod.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PodDNSConfig {
    /// A list of DNS name server IP addresses.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub nameservers: Vec<String>,

    /// A list of DNS search domains for hostname lookup.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub searches: Vec<String>,

    /// A list of DNS resolver options.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<PodDNSConfigOption>,
}

/// PodDNSConfigOption defines DNS resolver options of a pod.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PodDNSConfigOption {
    /// Name of the option.
    pub name: String,

    /// Value of the option.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// PodOS defines the operating system of the containers in a pod.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PodOS {
    /// Name is the name of the operating system (e.g., "linux" or "windows").
    pub name: String,
}

/// PodReadinessGate contains the reference to a pod condition.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PodReadinessGate {
    /// ConditionType refers to a condition in the pod's condition list.
    pub condition_type: String,
}

/// PodSchedulingGate is associated to a Pod to guard its scheduling.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PodSchedulingGate {
    /// Name of the scheduling gate.
    pub name: String,
}

/// HostAlias holds the mapping between IP and hostnames.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct HostAlias {
    /// IP address of the host file entry.
    pub ip: String,

    /// Hostnames for the above IP address.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hostnames: Vec<String>,
}

/// A single application container that you want to run within a pod.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Container {
    /// Name of the container specified as a DNS_LABEL.
    pub name: String,

    /// Container image name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,

    /// Entrypoint array. Not executed within a shell.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub command: Vec<String>,

    /// Arguments to the entrypoint.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,

    /// Container's working directory.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<String>,

    /// List of ports to expose from the container.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<ContainerPort>,

    /// List of environment variables to set in the container.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub env: Vec<EnvVar>,

    /// List of sources to populate environment variables from.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub env_from: Vec<EnvFromSource>,

    /// Compute Resources required by this container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<ResourceRequirements>,

    /// Pod volumes to mount into the container's filesystem.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volume_mounts: Vec<VolumeMount>,

    /// volumeDevices is the list of block devices to be used by the container.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volume_devices: Vec<VolumeDevice>,

    /// Periodic probe of container liveness.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub liveness_probe: Option<Probe>,

    /// Periodic probe of container service readiness.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readiness_probe: Option<Probe>,

    /// StartupProbe indicates that the Pod has successfully initialized.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub startup_probe: Option<Probe>,

    /// Actions that the management system should take in response to container lifecycle events.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<Lifecycle>,

    /// Path at which the file to which the container's termination message will be written.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub termination_message_path: Option<String>,

    /// Indicate how the termination message should be populated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub termination_message_policy: Option<String>,

    /// Image pull policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_pull_policy: Option<String>,

    /// SecurityContext defines the security options the container should be run with.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_context: Option<SecurityContext>,

    /// Whether this container should allocate a buffer for stdin in the container runtime.
    #[serde(default)]
    pub stdin: bool,

    /// Whether the container runtime should close the stdin channel after it has been opened.
    #[serde(default)]
    pub stdin_once: bool,

    /// Whether this container should allocate a TTY for itself.
    #[serde(default)]
    pub tty: bool,

    /// Whether this container should be allocated a read-only root filesystem.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_only_root_filesystem: Option<bool>,
}

/// ContainerStatus contains the current status of a container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ContainerStatus {
    /// Name of the container.
    pub name: String,

    /// The state of the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<ContainerState>,

    /// The last termination state of the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_state: Option<ContainerState>,

    /// Whether the container is currently running.
    #[serde(default)]
    pub running: bool,

    /// The number of times the container has been restarted.
    #[serde(default)]
    pub restart_count: i32,

    /// The image the container is running.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,

    /// Image ID of the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,

    /// Container ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,

    /// Ready specifies whether the container is currently passing its readiness check.
    #[serde(default)]
    pub ready: bool,

    /// Start time of the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<Timestamp>,

    /// Human-readable message indicating details about why container is not yet running.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// Brief reason explaining why the container is not yet running.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// ContainerState holds the current state of a container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ContainerState {
    /// Details about a running container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub running: Option<ContainerStateRunning>,

    /// Details about a terminated container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminated: Option<ContainerStateTerminated>,

    /// Details about a waiting container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub waiting: Option<ContainerStateWaiting>,
}

/// ContainerStateRunning is the running state of a container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ContainerStateRunning {
    /// Time at which the container was last restarted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<Timestamp>,
}

/// ContainerStateTerminated is the terminated state of a container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ContainerStateTerminated {
    /// Exit status from the last termination of the container.
    pub exit_code: i32,

    /// Signal from the last termination of the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signal: Option<i32>,

    /// Time at which the container terminated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<Timestamp>,

    /// Time at which previous execution of the container started.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<Timestamp>,

    /// Message regarding the last termination of the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// Brief reason explaining why the container terminated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// ContainerStateWaiting is the waiting state of a container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ContainerStateWaiting {
    /// Message regarding why the container is not yet running.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// Brief reason explaining why the container is not yet running.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// ContainerPort represents a network port in a single container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ContainerPort {
    /// If specified, this must be an IANA_SVC_NAME and unique within the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Number of port to expose on the pod's IP address.
    pub container_port: i32,

    /// Protocol for port. Must be UDP, TCP, or SCTP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,

    /// Number of port to expose on the host.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_port: Option<i32>,

    /// Host IP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_ip: Option<String>,
}

/// OS name constants for PodOS.
pub mod os_name {
    pub const LINUX: &str = "linux";
    pub const WINDOWS: &str = "windows";
}

/// Pod phase constants.
pub mod pod_phase {
    pub const PENDING: &str = "Pending";
    pub const RUNNING: &str = "Running";
    pub const SUCCEEDED: &str = "Succeeded";
    pub const FAILED: &str = "Failed";
    pub const UNKNOWN: &str = "Unknown";
}

/// Restart policy constants.
pub mod restart_policy {
    pub const ALWAYS: &str = "Always";
    pub const ON_FAILURE: &str = "OnFailure";
    pub const NEVER: &str = "Never";
}

/// DNS policy constants.
pub mod dns_policy {
    pub const CLUSTER_FIRST: &str = "ClusterFirst";
    pub const CLUSTER_FIRST_WITH_HOST_NET: &str = "ClusterFirstWithHostNet";
    pub const DEFAULT: &str = "Default";
    pub const NONE: &str = "None";
}

// ============================================================================
// Trait Implementations for Pod and PodList
// ============================================================================

// ----------------------------------------------------------------------------
// ResourceSchema Implementation
// ----------------------------------------------------------------------------

impl ResourceSchema for Pod {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        ""
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "Pod"
    }
    fn resource(_: &Self::Meta) -> &str {
        "pods"
    }

    fn group_static() -> &'static str {
        ""
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "Pod"
    }
    fn resource_static() -> &'static str {
        "pods"
    }
}

impl ResourceSchema for PodList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        ""
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "PodList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "pods"
    }

    fn group_static() -> &'static str {
        ""
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "PodList"
    }
    fn resource_static() -> &'static str {
        "pods"
    }
}

// ----------------------------------------------------------------------------
// HasTypeMeta Implementation
// ----------------------------------------------------------------------------

impl HasTypeMeta for Pod {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for PodList {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

// ----------------------------------------------------------------------------
// VersionedObject Implementation
// ----------------------------------------------------------------------------

impl VersionedObject for Pod {
    fn metadata(&self) -> &ObjectMeta {
        self.metadata
            .as_ref()
            .unwrap_or_else(|| static_default_object_meta())
    }

    fn metadata_mut(&mut self) -> &mut ObjectMeta {
        self.metadata.get_or_insert_with(ObjectMeta::default)
    }
}

// Helper function for static default ObjectMeta
fn static_default_object_meta() -> &'static ObjectMeta {
    use std::sync::OnceLock;
    static DEFAULT: OnceLock<ObjectMeta> = OnceLock::new();
    DEFAULT.get_or_init(ObjectMeta::default)
}

// Note: PodList does not implement VersionedObject because its metadata is ListMeta

// ----------------------------------------------------------------------------
// ApplyDefaults Implementation
// ----------------------------------------------------------------------------

impl ApplyDefault for Pod {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "Pod".to_string();
        }
        // Apply defaults to spec if present
        if let Some(ref mut spec) = self.spec {
            spec.apply_default();
        }
    }
}

impl ApplyDefault for PodList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "PodList".to_string();
        }
    }
}

impl ApplyDefault for PodSpec {
    fn apply_default(&mut self) {
        // Set default DNS policy to ClusterFirst if not specified
        if self.dns_policy.is_none() {
            self.dns_policy = Some("ClusterFirst".to_string());
        }

        // Set default restart policy to Always if not specified
        if self.restart_policy.is_none() {
            self.restart_policy = Some("Always".to_string());
        }

        // Set default termination grace period to 30 seconds if not specified
        if self.termination_grace_period_seconds.is_none() {
            self.termination_grace_period_seconds = Some(30);
        }

        // Set default scheduler name to "default-scheduler" if not specified
        if self.scheduler_name.is_none() {
            self.scheduler_name = Some("default-scheduler".to_string());
        }

        // Apply defaults to all containers
        for container in &mut self.containers {
            container.apply_default();
        }

        // Apply defaults to all init containers
        for container in &mut self.init_containers {
            container.apply_default();
        }
    }
}

impl ApplyDefault for Container {
    fn apply_default(&mut self) {
        // Set default termination message path if not specified
        if self.termination_message_path.is_none() {
            self.termination_message_path = Some("/dev/termination-log".to_string());
        }

        // Set default termination message policy to "File" if not specified
        if self.termination_message_policy.is_none() {
            self.termination_message_policy = Some("File".to_string());
        }

        // Set default image pull policy based on image tag if not specified
        if self.image_pull_policy.is_none() {
            if let Some(ref image) = self.image {
                // Check if the image tag is "latest" or missing (implies latest)
                let is_latest = if let Some(tag_start) = image.rfind(':') {
                    let tag = &image[tag_start + 1..];
                    tag == "latest" || tag.is_empty()
                } else {
                    // No tag specified, defaults to latest
                    true
                };

                self.image_pull_policy = Some(if is_latest {
                    "Always".to_string()
                } else {
                    "IfNotPresent".to_string()
                });
            } else {
                // No image specified, default to IfNotPresent
                self.image_pull_policy = Some("IfNotPresent".to_string());
            }
        }

        // Apply defaults to probes
        if let Some(ref mut probe) = self.liveness_probe {
            probe.apply_default();
        }
        if let Some(ref mut probe) = self.readiness_probe {
            probe.apply_default();
        }
        if let Some(ref mut probe) = self.startup_probe {
            probe.apply_default();
        }
    }
}

// ----------------------------------------------------------------------------
// Version Conversion Placeholder (using UnimplementedConversion)
// ----------------------------------------------------------------------------

// Conversion implementations in src/core/v1/conversion/pod.rs
// impl UnimplementedConversion for Pod {}

// ----------------------------------------------------------------------------
// Protobuf Placeholder (using macro)
// ----------------------------------------------------------------------------

// Conversion implementations in src/core/v1/conversion/pod.rs
impl_unimplemented_prost_message!(Pod);
impl_unimplemented_prost_message!(PodList);
