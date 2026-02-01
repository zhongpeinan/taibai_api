//! Service and Endpoints types from the Kubernetes Core v1 API
//!
//! This module contains types for Kubernetes services and endpoints.

use crate::common::{
    ApplyDefault, Condition, HasTypeMeta, IntOrString, ListMeta, ObjectMeta, ResourceSchema,
    TypeMeta, VersionedObject,
};
use crate::core::internal::{
    IPFamily, IPFamilyPolicy, ServiceAffinity, ServiceExternalTrafficPolicy,
    ServiceInternalTrafficPolicy, ServiceType,
};
use crate::core::v1::reference::ObjectReference;
use crate::impl_unimplemented_prost_message;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// ============================================================================
// Constants
// ============================================================================

/// Service affinity constants
pub mod service_affinity {
    /// ClientIP is the client IP-based session affinity.
    pub const CLIENT_IP: &str = "ClientIP";
    /// None is no session affinity.
    pub const NONE: &str = "None";
}

/// Default timeout for ClientIP service affinity in seconds
pub const DEFAULT_CLIENT_IP_SERVICE_AFFINITY_SECONDS: i32 = 10800;

/// Service type constants
pub mod service_type {
    /// ClusterIP means a service will only be accessible within the cluster.
    pub const CLUSTER_IP: &str = "ClusterIP";
    /// NodePort means service will be exposed on each node's IP at a static port.
    pub const NODE_PORT: &str = "NodePort";
    /// LoadBalancer means service will be exposed via an external load balancer.
    pub const LOAD_BALANCER: &str = "LoadBalancer";
    /// ExternalName means service will be mapped to a DNS name.
    pub const EXTERNAL_NAME: &str = "ExternalName";
}

/// Service internal traffic policy constants
pub mod service_internal_traffic_policy {
    /// Cluster routes traffic to all endpoints.
    pub const CLUSTER: &str = "Cluster";
    /// Local routes traffic only to local endpoints.
    pub const LOCAL: &str = "Local";
}

/// Service external traffic policy constants
pub mod service_external_traffic_policy {
    /// Cluster routes traffic to all endpoints.
    pub const CLUSTER: &str = "Cluster";
    /// Local routes traffic only to local endpoints and drops traffic to other endpoints.
    pub const LOCAL: &str = "Local";
}

/// Traffic distribution constants
pub mod service_traffic_distribution {
    /// PreferSameZone prefers routing traffic to endpoints in the same zone.
    pub const PREFER_SAME_ZONE: &str = "PreferSameZone";
    /// PreferSameNode prefers routing traffic to endpoints on the same node.
    pub const PREFER_SAME_NODE: &str = "PreferSameNode";
}

/// IP family constants
pub mod ip_family {
    /// IPv4 protocol
    pub const IPV4: &str = "IPv4";
    /// IPv6 protocol
    pub const IPV6: &str = "IPv6";
}

/// IP family policy constants
pub mod ip_family_policy {
    /// SingleStack means single IP family.
    pub const SINGLE_STACK: &str = "SingleStack";
    /// PreferDualStack prefers dual stack when possible.
    pub const PREFER_DUAL_STACK: &str = "PreferDualStack";
    /// RequireDualStack requires dual stack.
    pub const REQUIRE_DUAL_STACK: &str = "RequireDualStack";
}

/// Load balancer IP mode constants
pub mod load_balancer_ip_mode {
    /// VIP means virtual IP.
    pub const VIP: &str = "VIP";
    /// Proxy means proxy mode.
    pub const PROXY: &str = "Proxy";
}

/// Protocol constants
pub mod protocol {
    /// TCP protocol
    pub const TCP: &str = "TCP";
    /// UDP protocol
    pub const UDP: &str = "UDP";
    /// SCTP protocol
    pub const SCTP: &str = "SCTP";
}

/// Cluster IP none constant
pub const CLUSTER_IP_NONE: &str = "None";

/// Load balancer condition types
pub mod load_balancer_condition {
    /// LoadBalancerPortsError indicates a ports error.
    pub const PORTS_ERROR: &str = "LoadBalancerPortsError";
}

/// Load balancer condition reasons
pub mod load_balancer_condition_reason {
    /// MixedProtocolNotSupported reason
    pub const MIXED_PROTOCOL_NOT_SUPPORTED: &str = "LoadBalancerMixedProtocolNotSupported";
}

// ============================================================================
// Service Types
// ============================================================================

/// SessionAffinityConfig represents the session affinity configuration.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SessionAffinityConfig {
    /// ClientIP config for session affinity.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clientIP")]
    pub client_ip: Option<ClientIPConfig>,
}

/// ClientIPConfig represents the client IP configuration for session affinity.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClientIPConfig {
    /// TimeoutSeconds is the timeout in seconds for client IP session affinity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i32>,
}

/// PortStatus represents the error status of a port.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PortStatus {
    /// Port is the port number.
    pub port: i32,

    /// Protocol is the protocol of the port.
    pub protocol: String,

    /// Error is the error message if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// LoadBalancerIngress represents the ingress of a load balancer.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct LoadBalancerIngress {
    /// IP is the IP address of the ingress.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub ip: String,

    /// Hostname is the hostname of the ingress.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub hostname: String,

    /// IPMode is the IP mode of the ingress.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_mode: Option<String>,

    /// Ports is the list of port statuses.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<PortStatus>,
}

/// LoadBalancerStatus represents the status of a load balancer.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct LoadBalancerStatus {
    /// Ingress is the list of ingress addresses.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ingress: Vec<LoadBalancerIngress>,
}

/// ServiceStatus represents the current status of a service.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceStatus {
    /// LoadBalancer contains the current status of the load balancer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load_balancer: Option<LoadBalancerStatus>,

    /// Conditions is the list of conditions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<Condition>,
}

/// ServicePort represents the port on which the service is exposed.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ServicePort {
    /// Name is the name of the port.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    /// Protocol is the protocol of the port.
    #[serde(default = "default_protocol")]
    pub protocol: String,

    /// AppProtocol is the application protocol for this port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_protocol: Option<String>,

    /// Port is the port number.
    #[serde(default)]
    pub port: i32,

    /// TargetPort is the target port on the pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_port: Option<IntOrString>,

    /// NodePort is the port on each node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_port: Option<i32>,
}

impl Default for ServicePort {
    fn default() -> Self {
        Self {
            name: String::new(),
            protocol: default_protocol(),
            app_protocol: None,
            port: 0,
            target_port: None,
            node_port: None,
        }
    }
}

fn default_protocol() -> String {
    protocol::TCP.to_string()
}

/// ServiceSpec describes the attributes that a user creates on a service.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceSpec {
    /// Ports is the list of ports.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<ServicePort>,

    /// Selector is the label selector for pods.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub selector: BTreeMap<String, String>,

    /// ClusterIP is the IP address of the service.
    #[serde(
        rename = "clusterIP",
        default,
        skip_serializing_if = "String::is_empty"
    )]
    pub cluster_ip: String,

    /// ClusterIPs is the list of IP addresses.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cluster_ips: Vec<String>,

    /// Type is the type of service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ServiceType>,

    /// ExternalIPs is the list of external IPs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub external_ips: Vec<String>,

    /// SessionAffinity is the session affinity.
    #[serde(default)]
    pub session_affinity: ServiceAffinity,

    /// LoadBalancerIP is the IP address of the load balancer.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        rename = "loadBalancerIP"
    )]
    pub load_balancer_ip: String,

    /// LoadBalancerSourceRanges is the list of allowed source ranges.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub load_balancer_source_ranges: Vec<String>,

    /// ExternalName is the external name of the service.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub external_name: String,

    /// ExternalTrafficPolicy is the external traffic policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_traffic_policy: Option<ServiceExternalTrafficPolicy>,

    /// HealthCheckNodePort is the health check node port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health_check_node_port: Option<i32>,

    /// PublishNotReadyAddresses indicates whether to publish not ready addresses.
    #[serde(default, skip_serializing_if = "crate::common::is_false")]
    pub publish_not_ready_addresses: bool,

    /// SessionAffinityConfig is the session affinity configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_affinity_config: Option<SessionAffinityConfig>,

    /// IPFamilies is the list of IP families.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ip_families: Vec<IPFamily>,

    /// IPFamilyPolicy is the IP family policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_family_policy: Option<IPFamilyPolicy>,

    /// AllocateLoadBalancerNodePorts indicates whether to allocate node ports.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allocate_load_balancer_node_ports: Option<bool>,

    /// LoadBalancerClass is the class of the load balancer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load_balancer_class: Option<String>,

    /// InternalTrafficPolicy is the internal traffic policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub internal_traffic_policy: Option<ServiceInternalTrafficPolicy>,

    /// TrafficDistribution is the traffic distribution policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub traffic_distribution: Option<String>,
}

/// Service is a named abstraction of software service.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    /// TypeMeta describes the type of this object
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// Spec defines the behavior of a service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<ServiceSpec>,

    /// Status represents the current status of a service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ServiceStatus>,
}

/// ServiceList is a list of services.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceList {
    /// TypeMeta describes the type of this object
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// Items is the list of services.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Service>,
}

// ============================================================================
// Endpoints Types
// ============================================================================

/// EndpointAddress is a single address of an endpoint.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct EndpointAddress {
    /// IP is the IP address of the endpoint.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub ip: String,

    /// Hostname is the hostname of the endpoint.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub hostname: String,

    /// NodeName is the name of the node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,

    /// TargetRef is the reference to the target object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_ref: Option<ObjectReference>,
}

/// EndpointPort is a port of an endpoint.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EndpointPort {
    /// Name is the name of the port.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    /// Port is the port number.
    #[serde(default)]
    pub port: i32,

    /// Protocol is the protocol of the port.
    #[serde(default = "default_protocol")]
    pub protocol: String,

    /// AppProtocol is the application protocol for this port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_protocol: Option<String>,
}

impl Default for EndpointPort {
    fn default() -> Self {
        Self {
            name: String::new(),
            port: 0,
            protocol: default_protocol(),
            app_protocol: None,
        }
    }
}

/// EndpointSubset is a group of addresses with a common set of ports.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct EndpointSubset {
    /// Addresses is the list of ready addresses.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub addresses: Vec<EndpointAddress>,

    /// NotReadyAddresses is the list of not ready addresses.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub not_ready_addresses: Vec<EndpointAddress>,

    /// Ports is the list of ports.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<EndpointPort>,
}

/// Endpoints is a collection of endpoints that implement the service.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Endpoints {
    /// TypeMeta describes the type of this object
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// Subsets is the list of endpoint subsets.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subsets: Vec<EndpointSubset>,
}

/// EndpointsList is a list of endpoints.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct EndpointsList {
    /// TypeMeta describes the type of this object
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// Items is the list of endpoints.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Endpoints>,
}

// ============================================================================
// Trait Implementations for Service, ServiceList, Endpoints, and EndpointsList
// ============================================================================

// ----------------------------------------------------------------------------
// ResourceSchema Implementation
// ----------------------------------------------------------------------------

impl ResourceSchema for Service {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        ""
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "Service"
    }
    fn resource(_: &Self::Meta) -> &str {
        "services"
    }

    fn group_static() -> &'static str {
        ""
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "Service"
    }
    fn resource_static() -> &'static str {
        "services"
    }
}

impl ResourceSchema for ServiceList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        ""
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "ServiceList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "services"
    }

    fn group_static() -> &'static str {
        ""
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "ServiceList"
    }
    fn resource_static() -> &'static str {
        "services"
    }
}

impl ResourceSchema for Endpoints {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        ""
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "Endpoints"
    }
    fn resource(_: &Self::Meta) -> &str {
        "endpoints"
    }

    fn group_static() -> &'static str {
        ""
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "Endpoints"
    }
    fn resource_static() -> &'static str {
        "endpoints"
    }
}

impl ResourceSchema for EndpointsList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        ""
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "EndpointsList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "endpoints"
    }

    fn group_static() -> &'static str {
        ""
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "EndpointsList"
    }
    fn resource_static() -> &'static str {
        "endpoints"
    }
}

// ----------------------------------------------------------------------------
// HasTypeMeta Implementation
// ----------------------------------------------------------------------------

impl HasTypeMeta for Service {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for ServiceList {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for Endpoints {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for EndpointsList {
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

impl VersionedObject for Service {
    fn metadata(&self) -> &ObjectMeta {
        use std::sync::OnceLock;
        self.metadata.as_ref().unwrap_or_else(|| {
            static DEFAULT: OnceLock<ObjectMeta> = OnceLock::new();
            DEFAULT.get_or_init(ObjectMeta::default)
        })
    }

    fn metadata_mut(&mut self) -> &mut ObjectMeta {
        self.metadata.get_or_insert_with(ObjectMeta::default)
    }
}

impl VersionedObject for Endpoints {
    fn metadata(&self) -> &ObjectMeta {
        use std::sync::OnceLock;
        self.metadata.as_ref().unwrap_or_else(|| {
            static DEFAULT: OnceLock<ObjectMeta> = OnceLock::new();
            DEFAULT.get_or_init(ObjectMeta::default)
        })
    }

    fn metadata_mut(&mut self) -> &mut ObjectMeta {
        self.metadata.get_or_insert_with(ObjectMeta::default)
    }
}

// Note: ServiceList and EndpointsList do not implement VersionedObject because their metadata is ListMeta

// ----------------------------------------------------------------------------
// ApplyDefaults Implementation
// ----------------------------------------------------------------------------

impl ApplyDefault for Service {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "Service".to_string();
        }
        // Apply defaults to spec if present
        if let Some(ref mut spec) = self.spec {
            spec.apply_default();
        }
        if let (Some(spec), Some(status)) = (self.spec.as_ref(), self.status.as_mut()) {
            if spec.type_ == Some(ServiceType::LoadBalancer) {
                if let Some(load_balancer) = status.load_balancer.as_mut() {
                    for ingress in &mut load_balancer.ingress {
                        if !ingress.ip.is_empty() && ingress.ip_mode.is_none() {
                            ingress.ip_mode = Some(load_balancer_ip_mode::VIP.to_string());
                        }
                    }
                }
            }
        }
    }
}

impl ApplyDefault for ServiceList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "ServiceList".to_string();
        }
        for item in &mut self.items {
            item.apply_default();
        }
    }
}

impl ApplyDefault for ServiceSpec {
    fn apply_default(&mut self) {
        // Set default service type to ClusterIP if not specified
        if self.type_.is_none() {
            self.type_ = Some(ServiceType::ClusterIp);
        }

        // Set default internal traffic policy to Cluster for applicable service types
        if self.internal_traffic_policy.is_none() {
            match self.type_ {
                Some(ServiceType::ClusterIp)
                | Some(ServiceType::NodePort)
                | Some(ServiceType::LoadBalancer) => {
                    self.internal_traffic_policy = Some(ServiceInternalTrafficPolicy::Cluster);
                }
                _ => {}
            }
        }

        // Set default allocate load balancer node ports to true for LoadBalancer type
        if self.type_ == Some(ServiceType::LoadBalancer)
            && self.allocate_load_balancer_node_ports.is_none()
        {
            self.allocate_load_balancer_node_ports = Some(true);
        }

        // Set default external traffic policy to Cluster for externally-accessible services
        if self.external_traffic_policy.is_none() {
            match self.type_ {
                Some(ServiceType::NodePort) | Some(ServiceType::LoadBalancer) => {
                    self.external_traffic_policy = Some(ServiceExternalTrafficPolicy::Cluster);
                }
                _ => {}
            }
        }

        // Clear session affinity config if session affinity is None
        if self.session_affinity == ServiceAffinity::None {
            self.session_affinity_config = None;
        }

        // Set default timeout for ClientIP session affinity
        if self.session_affinity == ServiceAffinity::ClientIp {
            let needs_default = match self.session_affinity_config.as_ref() {
                None => true,
                Some(config) => match config.client_ip.as_ref() {
                    None => true,
                    Some(client_ip) => client_ip.timeout_seconds.is_none(),
                },
            };
            if needs_default {
                self.session_affinity_config = Some(SessionAffinityConfig {
                    client_ip: Some(ClientIPConfig {
                        timeout_seconds: Some(DEFAULT_CLIENT_IP_SERVICE_AFFINITY_SECONDS),
                    }),
                });
            }
        }

        // Apply defaults to all ports
        for port in &mut self.ports {
            port.apply_default();
        }
    }
}

impl ApplyDefault for ServicePort {
    fn apply_default(&mut self) {
        if self.protocol.is_empty() {
            self.protocol = default_protocol();
        }
        // Set default target port to port if not specified
        if self.target_port.is_none()
            || self.target_port == Some(IntOrString::Int(0))
            || matches!(
                self.target_port,
                Some(IntOrString::String(ref value)) if value.is_empty()
            )
        {
            self.target_port = Some(IntOrString::Int(self.port));
        }
    }
}

impl ApplyDefault for Endpoints {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "Endpoints".to_string();
        }
        for subset in &mut self.subsets {
            for port in &mut subset.ports {
                port.apply_default();
            }
        }
    }
}

impl ApplyDefault for EndpointsList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "EndpointsList".to_string();
        }
        for item in &mut self.items {
            item.apply_default();
        }
    }
}

impl ApplyDefault for EndpointPort {
    fn apply_default(&mut self) {
        if self.protocol.is_empty() {
            self.protocol = default_protocol();
        }
    }
}

// ----------------------------------------------------------------------------
// Protobuf Placeholder (using macro)
// ----------------------------------------------------------------------------

impl_unimplemented_prost_message!(Service);
impl_unimplemented_prost_message!(ServiceList);
impl_unimplemented_prost_message!(Endpoints);
impl_unimplemented_prost_message!(EndpointsList);
