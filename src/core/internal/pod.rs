//! Pod types from the Kubernetes Core API
//!
//! This module contains the core Pod types including Pod, PodSpec, PodStatus,
//! PodTemplate, and related condition and configuration types.
//!
//! Source: k8s-pkg/apis/core/types.go

use crate::common::{ListMeta, ObjectMeta, TypeMeta};
use crate::core::internal::{
    Affinity, DNSPolicy, LocalObjectReference, PodDNSConfig, PodOS, PodPhase, PodResizeStatus,
    PodResourceClaim, PodResourceClaimStatus, PodSchedulingGate, PodSecurityContext,
    PreemptionPolicy, ResourceList, RestartPolicy, Toleration,
};
use crate::core::v1;
use crate::impl_has_object_meta;
use serde::{Deserialize, Serialize};

/// Type alias for Container from v1 API
pub type Container = v1::Container;
/// Type alias for EphemeralContainer from v1 API
pub type EphemeralContainer = v1::EphemeralContainer;
/// Type alias for PodReadinessGate from v1 API
pub type PodReadinessGate = v1::PodReadinessGate;
/// Type alias for TopologySpreadConstraint from v1 API
pub type TopologySpreadConstraint = v1::TopologySpreadConstraint;

// ============================================================================
// Pod
// ============================================================================

/// Pod is a collection of containers that can run on a host.
///
/// Corresponds to [Kubernetes Pod](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4583)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Pod {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ObjectMeta,
    /// Spec defines the behavior of a pod.
    #[serde(default)]
    pub spec: PodSpec,
    /// Status represents the current information about a pod.
    #[serde(default)]
    pub status: PodStatus,
}
impl_has_object_meta!(Pod);

/// PodSpec is a description of a pod.
///
/// Corresponds to [Kubernetes PodSpec](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3656)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodSpec {
    /// List of volumes that can be mounted by containers belonging to the pod.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volumes: Vec<crate::core::internal::Volume>,
    /// List of initialization containers belonging to the pod.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub init_containers: Vec<Container>,
    /// List of containers belonging to the pod.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub containers: Vec<Container>,
    /// List of ephemeral containers run in this pod.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ephemeral_containers: Vec<EphemeralContainer>,
    /// Restart policy for all containers within the pod.
    #[serde(default)]
    pub restart_policy: RestartPolicy,
    /// Optional duration in seconds the pod needs to terminate gracefully.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub termination_grace_period_seconds: Option<i64>,
    /// Optional duration in seconds the pod may be active on a node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_deadline_seconds: Option<i64>,
    /// Set DNS policy for the pod.
    #[serde(default)]
    pub dns_policy: DNSPolicy,
    /// NodeSelector is a selector which must be true for the pod to fit on a node.
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub node_selector: std::collections::BTreeMap<String, String>,
    /// ServiceAccountName is the name of the ServiceAccount to use to run this pod.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub service_account_name: String,
    /// AutomountServiceAccountToken indicates whether a service account token should be automatically mounted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub automount_service_account_token: Option<bool>,
    /// NodeName indicates in which node this pod is scheduled.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub node_name: String,
    /// SecurityContext holds pod-level security attributes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_context: Option<PodSecurityContext>,
    /// ImagePullSecrets is an optional list of references to secrets.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub image_pull_secrets: Vec<LocalObjectReference>,
    /// Specifies the hostname of the Pod.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub hostname: String,
    /// If specified, the fully qualified Pod hostname will be "<hostname>.<subdomain>...".
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub subdomain: String,
    /// If true the pod's hostname will be configured as the pod's FQDN.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set_hostname_as_fqdn: Option<bool>,
    /// If specified, the pod's scheduling constraints.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub affinity: Option<Affinity>,
    /// If specified, the pod will be dispatched by specified scheduler.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub scheduler_name: String,
    /// If specified, the pod's tolerations.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tolerations: Vec<Toleration>,
    /// HostAliases is an optional list of hosts and IPs that will be injected into the pod's hosts file.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub host_aliases: Vec<HostAlias>,
    /// If specified, indicates the pod's priority.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub priority_class_name: String,
    /// The priority value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// PreemptionPolicy is the Policy for preempting pods with lower priority.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preemption_policy: Option<PreemptionPolicy>,
    /// Specifies the DNS parameters of a pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dns_config: Option<PodDNSConfig>,
    /// If specified, all readiness gates will be evaluated for pod readiness.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub readiness_gates: Vec<PodReadinessGate>,
    /// RuntimeClassName refers to a RuntimeClass object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runtime_class_name: Option<String>,
    /// Overhead represents the resource overhead associated with running a pod.
    #[serde(default, skip_serializing_if = "ResourceList::is_empty")]
    pub overhead: ResourceList,
    /// EnableServiceLinks indicates whether information about services should be injected.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable_service_links: Option<bool>,
    /// TopologySpreadConstraints describes how a group of pods ought to spread across topology domains.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topology_spread_constraints: Vec<TopologySpreadConstraint>,
    /// Specifies the OS of the containers in the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub os: Option<PodOS>,
    /// SchedulingGates is an opaque list of values that if specified will block scheduling the pod.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scheduling_gates: Vec<PodSchedulingGate>,
    /// ResourceClaims defines which ResourceClaims must be allocated.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resource_claims: Vec<PodResourceClaim>,
    /// Resources is the total amount of CPU and Memory resources required by all containers in the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<crate::core::internal::ResourceRequirements>,
}

/// PodStatus represents the current state of a Pod.
///
/// Corresponds to [Kubernetes PodStatus](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4462)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodStatus {
    /// ObservedGeneration is the .metadata.generation that the pod status was set based upon.
    #[serde(default)]
    pub observed_generation: i64,
    /// Phase is the current phase of the pod.
    #[serde(default)]
    pub phase: PodPhase,
    /// Conditions is an array of current conditions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<PodCondition>,
    /// A human readable message indicating details about why the pod is in this state.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
    /// A brief CamelCase message indicating details about why the pod is in this state.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    /// nominatedNodeName is set when this pod preempts other pods on the node.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub nominated_node_name: String,
    /// HostIP holds the IP address of the host to which the pod is assigned.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub host_ip: String,
    /// HostIPs holds the IP addresses allocated to the host.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub host_ips: Vec<HostIP>,
    /// PodIPs holds all of the known IP addresses allocated to the pod.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pod_ips: Vec<PodIP>,
    /// StartTime is the time when the pod was acknowledged by the Kubelet.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<crate::common::Timestamp>,
    /// QOSClass is the quality of service class.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub qos_class: String,
    /// Statuses of init containers in this pod.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub init_container_statuses: Vec<crate::core::internal::ContainerStatus>,
    /// Statuses of containers in this pod.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub container_statuses: Vec<crate::core::internal::ContainerStatus>,
    /// Statuses for any ephemeral containers that have run in this pod.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ephemeral_container_statuses: Vec<crate::core::internal::ContainerStatus>,
    /// Status of resource claims.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resource_claim_statuses: Vec<PodResourceClaimStatus>,
    /// Extended resource claim status for DRA-backed extended resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extended_resource_claim_status: Option<PodExtendedResourceClaimStatus>,
    /// Status for any resize operations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resize: Option<PodResizeStatus>,
}

/// PodList is a list of Pods.
///
/// Corresponds to [Kubernetes PodList](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3275)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    /// List of pods.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Pod>,
}

// ============================================================================
// Pod Template
// ============================================================================

/// PodTemplateSpec describes the data a pod should have when created from a template.
///
/// Corresponds to [Kubernetes PodTemplateSpec](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4599)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodTemplateSpec {
    /// Metadata of the pods created from this template.
    pub metadata: ObjectMeta,
    /// Spec defines the behavior of a pod.
    #[serde(default)]
    pub spec: PodSpec,
}

/// PodTemplate describes a template for creating copies of a predefined pod.
///
/// Corresponds to [Kubernetes PodTemplate](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4612)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodTemplate {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ObjectMeta,
    /// Template defines the pods that will be created from this pod template.
    #[serde(default)]
    pub template: PodTemplateSpec,
}
impl_has_object_meta!(PodTemplate);

/// PodTemplateList is a list of PodTemplates.
///
/// Corresponds to [Kubernetes PodTemplateList](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4625)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodTemplateList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    /// List of PodTemplates.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<PodTemplate>,
}

// ============================================================================
// Pod Condition
// ============================================================================

/// PodCondition contains details for the current condition of this pod.
///
/// Corresponds to [Kubernetes PodCondition](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3154)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodCondition {
    /// Type is the type of the condition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub r#type: String,
    /// ObservedGeneration is the .metadata.generation that the condition was set based upon.
    #[serde(default)]
    pub observed_generation: i64,
    /// Status is the status of the condition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub status: String,
    /// Last time we probed the condition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_probe_time: Option<crate::common::Timestamp>,
    /// Last time the condition transitioned from one status to another.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<crate::common::Timestamp>,
    /// Unique, one-word, CamelCase reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    /// Human-readable message indicating details about last transition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
}

// ============================================================================
// Pod IP and Host IP
// ============================================================================

/// PodIP represents a single IP address allocated to the pod.
///
/// Corresponds to [Kubernetes PodIP](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4311)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodIP {
    /// IP is the IP address assigned to the pod.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub ip: String,
}

/// HostIP represents a single IP address allocated to the host.
///
/// Corresponds to [Kubernetes HostIP](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4317)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct HostIP {
    /// IP is the IP address assigned to the host.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub ip: String,
}

// ============================================================================
// Host Alias
// ============================================================================

/// HostAlias holds the mapping between IP and hostnames that will be injected
/// as an entry in the pod's hosts file.
///
/// Corresponds to [Kubernetes HostAlias](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3986)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct HostAlias {
    /// IP address of the host file entry.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub ip: String,
    /// Hostnames for the above IP address.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hostnames: Vec<String>,
}

// ============================================================================
// Extended Resource Claim Types
// ============================================================================

/// PodExtendedResourceClaimStatus identifies the mapping of container extended resources to device requests.
///
/// Corresponds to [Kubernetes PodExtendedResourceClaimStatus](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4495)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodExtendedResourceClaimStatus {
    /// RequestMappings identifies the mapping of <container, extended resource backed by DRA> to device request.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub request_mappings: Vec<ContainerExtendedResourceRequest>,
    /// ResourceClaimName is the name of the ResourceClaim that was generated for the Pod.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub resource_claim_name: String,
}

/// ContainerExtendedResourceRequest has the mapping of container name, extended resource name to the device request name.
///
/// Corresponds to [Kubernetes ContainerExtendedResourceRequest](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4508)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContainerExtendedResourceRequest {
    /// The name of the container requesting resources.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub container_name: String,
    /// The name of the extended resource in that container which gets backed by DRA.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub resource_name: String,
    /// The name of the request in the special ResourceClaim which corresponds to the extended resource.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub request_name: String,
}

// ============================================================================
// Pod Resource Claim (already defined in pod_resources.rs)
// ============================================================================

// Preemption Policy (already defined in mod.rs)
// ============================================================================

#[cfg(test)]
mod tests {}
