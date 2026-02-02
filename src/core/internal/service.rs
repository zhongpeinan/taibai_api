//! Service types from the Kubernetes Core API
//!
//! This module contains types for Kubernetes Service resources,
//! which provide network access to applications.
//!
//! Source: k8s-pkg/apis/core/types.go

use crate::common::{Condition, IntOrString, ListMeta, ObjectMeta, TypeMeta};
use crate::core::internal::{
    IPFamily, IPFamilyPolicy, Protocol, ServiceAffinity, ServiceExternalTrafficPolicy,
    ServiceInternalTrafficPolicy, ServiceType,
};
use crate::impl_has_object_meta;
use serde::{Deserialize, Serialize};

// ============================================================================
// Service
// ============================================================================

/// Service is a named abstraction of software service (for example, mysql) consisting of local port
/// (for example 3306) that the proxy listens to, and the selector that determines which pods will
/// answer requests sent through the proxy.
///
/// Corresponds to [Kubernetes Service](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3846)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ObjectMeta,
    /// Spec defines the behavior of a service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<ServiceSpec>,
    /// Status represents the current status of a service.
    #[serde(default)]
    pub status: ServiceStatus,
}
impl_has_object_meta!(Service);

/// ServiceSpec describes the attributes that a user creates on a service.
///
/// Corresponds to [Kubernetes ServiceSpec](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3977)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceSpec {
    /// The list of ports that are exposed by this service.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<ServicePort>,
    /// Route service traffic to pods with label keys and values matching this selector.
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub selector: std::collections::BTreeMap<String, String>,
    /// ClusterIP is the IP address of the service and is usually assigned randomly.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub cluster_ip: String,
    /// ClusterIPs is a list of IP addresses assigned to this service.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cluster_ips: Vec<String>,
    /// Type determines how the Service is exposed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ServiceType>,
    /// SessionAffinity defines the affinity settings for client traffic.
    #[serde(default)]
    pub session_affinity: ServiceAffinity,
    /// SessionAffinityConfig represents the configuration of session affinity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_affinity_config: Option<SessionAffinityConfig>,
    /// ExternalIPs is a list of IP addresses for which nodes in the cluster will also accept traffic.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub external_ips: Vec<String>,
    /// ExternalName is the external reference that kubedns or equivalent will return.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub external_name: String,
    /// ExternalTrafficPolicy describes how nodes distribute service traffic.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_traffic_policy: Option<ServiceExternalTrafficPolicy>,
    /// HealthCheckNodePort specifies the healthcheck nodePort for the service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health_check_node_port: Option<i32>,
    /// IPFamily specifies whether this service has a preference for a particular IP family.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ip_families: Vec<IPFamily>,
    /// IPFamiliesPolicy is a set of policies for IP families.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_families_policy: Option<IPFamilyPolicy>,
    /// LoadBalancerClass is the class of the load balancer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load_balancer_class: Option<String>,
    /// InternalTrafficPolicy describes how nodes distribute service traffic.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub internal_traffic_policy: Option<ServiceInternalTrafficPolicy>,
    /// LoadBalancerIP will be used for load balancer IP.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub load_balancer_ip: String,
    /// LoadBalancerSourceRanges is a list of CIDR ranges.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub load_balancer_source_ranges: Vec<String>,
    /// PublishNotReadyAddresses indicates that any agent which deals with endpoints.
    #[serde(default)]
    pub publish_not_ready_addresses: bool,
    /// AllocateLoadBalancerNodePorts defines if NodePorts will be automatically
    /// allocated for services with type LoadBalancer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allocate_load_balancer_node_ports: Option<bool>,
    /// TrafficDistribution offers a way to express preferences for how traffic
    /// is distributed to Service endpoints.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub traffic_distribution: Option<String>,
}

/// ServiceStatus represents the current status of a service.
///
/// Corresponds to [Kubernetes ServiceStatus](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4096)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceStatus {
    /// LoadBalancer contains the current status of the load-balancer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load_balancer: Option<LoadBalancerStatus>,
    /// Conditions is the list of conditions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<Condition>,
}

/// ServiceList is a list of Services.
///
/// Corresponds to [Kubernetes ServiceList](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3856)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    /// List of services.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Service>,
}

// ============================================================================
// Service Port
// ============================================================================

/// ServicePort represents the port on which the service is exposed.
///
/// Corresponds to [Kubernetes ServicePort](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3912)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServicePort {
    /// The name of this port within the service.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// The IP protocol for this port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<Protocol>,
    /// The port number that is exposed.
    #[serde(default)]
    pub port: i32,
    /// The port on each node on which this service is exposed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_port: Option<i32>,
    /// The target port on pods selected by this service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_port: Option<IntOrString>,
    /// The application protocol for this port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_protocol: Option<String>,
}

// ============================================================================
// Load Balancer
// ============================================================================

/// LoadBalancerStatus represents the status of a load-balancer.
///
/// Corresponds to [Kubernetes LoadBalancerStatus](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4102)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct LoadBalancerStatus {
    /// Ingress is a list containing ingress points for the load-balancer.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ingress: Vec<LoadBalancerIngress>,
}

/// LoadBalancerIngress represents the status of a load-balancer ingress point.
///
/// Corresponds to [Kubernetes LoadBalancerIngress](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4108)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct LoadBalancerIngress {
    /// IP is set for load-balancer ingress points that are IP based.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub ip: String,
    /// Hostname is set for load-balancer ingress points that are DNS based.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub hostname: String,
    /// Ports provides information about the ports exposed by this LoadBalancer.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<PortStatus>,
    /// IPMode specifies how the load-balancer IP behaves.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_mode: Option<String>,
}

/// PortStatus represents the status of a port.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PortStatus {
    /// Port is the port number of the service port.
    #[serde(default)]
    pub port: i32,
    /// Protocol is the protocol of the service port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<Protocol>,
    /// Error is to record the problem with the service port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

// ============================================================================
// Session Affinity
// ============================================================================

/// SessionAffinityConfig represents the configuration of session affinity.
///
/// Corresponds to [Kubernetes SessionAffinityConfig](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4142)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SessionAffinityConfig {
    /// ClientIP contains the configurations of Client IP based session affinity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_ip: Option<ClientIPConfig>,
}

/// ClientIPConfig represents the configurations of Client IP based session affinity.
///
/// Corresponds to [Kubernetes ClientIPConfig](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4148)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClientIPConfig {
    /// TimeoutSeconds specifies the seconds of ClientIP session affinity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i32>,
}

#[cfg(test)]
mod tests {}
