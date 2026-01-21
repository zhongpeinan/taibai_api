//! Service and Endpoints types from the Kubernetes Core v1 API
//!
//! This module contains types for Kubernetes services and endpoints.

use crate::common::{
    ApplyDefaults, Condition, HasTypeMeta, IntOrString, ListMeta, ObjectMeta, ResourceSchema,
    TypeMeta, UnimplementedConversion, VersionedObject,
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
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SessionAffinityConfig {
    /// ClientIP config for session affinity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
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
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
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
    pub type_: Option<String>,

    /// ExternalIPs is the list of external IPs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub external_ips: Vec<String>,

    /// SessionAffinity is the session affinity.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub session_affinity: String,

    /// LoadBalancerIP is the IP address of the load balancer.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub load_balancer_ip: String,

    /// LoadBalancerSourceRanges is the list of allowed source ranges.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub load_balancer_source_ranges: Vec<String>,

    /// ExternalName is the external name of the service.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub external_name: String,

    /// ExternalTrafficPolicy is the external traffic policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_traffic_policy: Option<String>,

    /// HealthCheckNodePort is the health check node port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health_check_node_port: Option<i32>,

    /// PublishNotReadyAddresses indicates whether to publish not ready addresses.
    #[serde(default)]
    pub publish_not_ready_addresses: bool,

    /// SessionAffinityConfig is the session affinity configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_affinity_config: Option<SessionAffinityConfig>,

    /// IPFamilies is the list of IP families.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ip_families: Vec<String>,

    /// IPFamilyPolicy is the IP family policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_family_policy: Option<String>,

    /// AllocateLoadBalancerNodePorts indicates whether to allocate node ports.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allocate_load_balancer_node_ports: Option<bool>,

    /// LoadBalancerClass is the class of the load balancer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load_balancer_class: Option<String>,

    /// InternalTrafficPolicy is the internal traffic policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub internal_traffic_policy: Option<String>,

    /// TrafficDistribution is the traffic distribution policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub traffic_distribution: Option<String>,
}

/// Service is a named abstraction of software service.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
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
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
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
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
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
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    // Constants tests
    #[test]
    fn test_service_affinity_constants() {
        assert_eq!(service_affinity::CLIENT_IP, "ClientIP");
        assert_eq!(service_affinity::NONE, "None");
    }

    #[test]
    fn test_service_type_constants() {
        assert_eq!(service_type::CLUSTER_IP, "ClusterIP");
        assert_eq!(service_type::NODE_PORT, "NodePort");
        assert_eq!(service_type::LOAD_BALANCER, "LoadBalancer");
        assert_eq!(service_type::EXTERNAL_NAME, "ExternalName");
    }

    #[test]
    fn test_protocol_constants() {
        assert_eq!(protocol::TCP, "TCP");
        assert_eq!(protocol::UDP, "UDP");
        assert_eq!(protocol::SCTP, "SCTP");
    }

    // SessionAffinityConfig tests
    #[test]
    fn test_session_affinity_config_with_client_ip() {
        let config = SessionAffinityConfig {
            client_ip: Some(ClientIPConfig {
                timeout_seconds: Some(3600),
            }),
        };
        assert!(config.client_ip.is_some());
        assert_eq!(
            config.client_ip.as_ref().unwrap().timeout_seconds,
            Some(3600)
        );
    }

    #[test]
    fn test_client_ip_config_default() {
        let config = ClientIPConfig::default();
        assert!(config.timeout_seconds.is_none());
    }

    // LoadBalancerStatus tests
    #[test]
    fn test_load_balancer_ingress_default() {
        let ingress = LoadBalancerIngress::default();
        assert!(ingress.ip.is_empty());
        assert!(ingress.hostname.is_empty());
        assert!(ingress.ports.is_empty());
    }

    #[test]
    fn test_load_balancer_ingress_with_ip() {
        let ingress = LoadBalancerIngress {
            ip: "192.168.1.1".to_string(),
            ..Default::default()
        };
        assert_eq!(ingress.ip, "192.168.1.1");
    }

    #[test]
    fn test_load_balancer_status_default() {
        let status = LoadBalancerStatus::default();
        assert!(status.ingress.is_empty());
    }

    #[test]
    fn test_load_balancer_status_with_ingress() {
        let status = LoadBalancerStatus {
            ingress: vec![LoadBalancerIngress {
                ip: "192.168.1.1".to_string(),
                ..Default::default()
            }],
        };
        assert_eq!(status.ingress.len(), 1);
        assert_eq!(status.ingress[0].ip, "192.168.1.1");
    }

    // ServicePort tests
    #[test]
    fn test_service_port_default() {
        let port = ServicePort::default();
        assert_eq!(port.protocol, "TCP");
        assert_eq!(port.port, 0);
    }

    #[test]
    fn test_service_port_with_fields() {
        let port = ServicePort {
            name: "http".to_string(),
            protocol: "TCP".to_string(),
            port: 80,
            target_port: Some(IntOrString::Int(8080)),
            ..Default::default()
        };
        assert_eq!(port.name, "http");
        assert_eq!(port.port, 80);
    }

    #[test]
    fn test_service_port_serialize() {
        let port = ServicePort {
            name: "http".to_string(),
            protocol: "TCP".to_string(),
            port: 80,
            target_port: Some(IntOrString::Int(8080)),
            ..Default::default()
        };
        let json = serde_json::to_string(&port).unwrap();
        assert!(json.contains("\"name\":\"http\""));
        assert!(json.contains("\"port\":80"));
    }

    // ServiceSpec tests
    #[test]
    fn test_service_spec_default() {
        let spec = ServiceSpec::default();
        assert!(spec.ports.is_empty());
        assert!(spec.selector.is_empty());
        assert!(spec.cluster_ip.is_empty());
        assert!(!spec.publish_not_ready_addresses);
    }

    #[test]
    fn test_service_spec_with_selector() {
        let mut selector = BTreeMap::new();
        selector.insert("app".to_string(), "nginx".to_string());

        let spec = ServiceSpec {
            selector,
            cluster_ip: "10.0.0.1".to_string(),
            ..Default::default()
        };
        assert_eq!(spec.cluster_ip, "10.0.0.1");
        assert_eq!(spec.selector.len(), 1);
    }

    #[test]
    fn test_service_spec_serialize() {
        let mut selector = BTreeMap::new();
        selector.insert("app".to_string(), "nginx".to_string());

        let spec = ServiceSpec {
            cluster_ip: "10.0.0.1".to_string(),
            selector,
            type_: Some(service_type::CLUSTER_IP.to_string()),
            ..Default::default()
        };
        let json = serde_json::to_string(&spec).unwrap();
        assert!(json.contains("\"clusterIP\":\"10.0.0.1\""));
        assert!(json.contains("\"type\":\"ClusterIP\""));
        assert!(json.contains("\"app\":\"nginx\""));
    }

    // Service tests
    #[test]
    fn test_service_with_metadata() {
        let service = Service {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("my-service".to_string()),
                ..Default::default()
            }),
            spec: None,
            status: None,
        };
        assert_eq!(
            service.metadata.as_ref().unwrap().name,
            Some("my-service".to_string())
        );
    }

    #[test]
    fn test_service_serialize() {
        let mut selector = BTreeMap::new();
        selector.insert("app".to_string(), "nginx".to_string());

        let service = Service {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("my-service".to_string()),
                ..Default::default()
            }),
            spec: Some(ServiceSpec {
                cluster_ip: "10.0.0.1".to_string(),
                selector,
                type_: Some(service_type::CLUSTER_IP.to_string()),
                ports: vec![ServicePort {
                    name: "http".to_string(),
                    port: 80,
                    ..Default::default()
                }],
                ..Default::default()
            }),
            status: None,
        };
        let json = serde_json::to_string(&service).unwrap();
        assert!(json.contains("\"name\":\"my-service\""));
        assert!(json.contains("\"clusterIP\":\"10.0.0.1\""));
        assert!(json.contains("\"port\":80"));
    }

    #[test]
    fn test_service_deserialize() {
        let json = r#"{
            "metadata": {"name": "my-service"},
            "spec": {
                "clusterIP": "10.0.0.1",
                "selector": {"app": "nginx"}
            }
        }"#;
        let service: Service = serde_json::from_str(json).unwrap();
        assert_eq!(
            service.metadata.as_ref().unwrap().name,
            Some("my-service".to_string())
        );
        assert_eq!(service.spec.as_ref().unwrap().cluster_ip, "10.0.0.1");
    }

    #[test]
    fn test_service_list_default() {
        let list = ServiceList {
            type_meta: TypeMeta::default(),
            metadata: None,
            items: vec![],
        };
        assert!(list.metadata.is_none());
        assert!(list.items.is_empty());
    }

    #[test]
    fn test_service_list_with_items() {
        let list = ServiceList {
            type_meta: TypeMeta::default(),
            metadata: Some(ListMeta {
                resource_version: Some("12345".to_string()),
                ..Default::default()
            }),
            items: vec![Service {
                type_meta: TypeMeta::default(),
                metadata: Some(ObjectMeta {
                    name: Some("my-service".to_string()),
                    ..Default::default()
                }),
                spec: None,
                status: None,
            }],
        };
        assert_eq!(list.items.len(), 1);
        assert_eq!(
            list.metadata.as_ref().unwrap().resource_version,
            Some("12345".to_string())
        );
    }

    // EndpointAddress tests
    #[test]
    fn test_endpoint_address_default() {
        let addr = EndpointAddress::default();
        assert!(addr.ip.is_empty());
        assert!(addr.hostname.is_empty());
    }

    #[test]
    fn test_endpoint_address_with_ip() {
        let addr = EndpointAddress {
            ip: "10.0.0.1".to_string(),
            hostname: "pod-1".to_string(),
            ..Default::default()
        };
        assert_eq!(addr.ip, "10.0.0.1");
        assert_eq!(addr.hostname, "pod-1");
    }

    // EndpointPort tests
    #[test]
    fn test_endpoint_port_default() {
        let port = EndpointPort::default();
        assert_eq!(port.protocol, "TCP");
        assert_eq!(port.port, 0);
    }

    #[test]
    fn test_endpoint_port_with_fields() {
        let port = EndpointPort {
            name: "http".to_string(),
            port: 8080,
            protocol: "TCP".to_string(),
            ..Default::default()
        };
        assert_eq!(port.name, "http");
        assert_eq!(port.port, 8080);
    }

    // EndpointSubset tests
    #[test]
    fn test_endpoint_subset_default() {
        let subset = EndpointSubset::default();
        assert!(subset.addresses.is_empty());
        assert!(subset.ports.is_empty());
    }

    #[test]
    fn test_endpoint_subset_with_addresses() {
        let subset = EndpointSubset {
            addresses: vec![EndpointAddress {
                ip: "10.0.0.1".to_string(),
                ..Default::default()
            }],
            ports: vec![EndpointPort {
                port: 8080,
                ..Default::default()
            }],
            ..Default::default()
        };
        assert_eq!(subset.addresses.len(), 1);
        assert_eq!(subset.ports.len(), 1);
    }

    // Endpoints tests
    #[test]
    fn test_endpoints_with_metadata() {
        let endpoints = Endpoints {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("my-endpoints".to_string()),
                ..Default::default()
            }),
            subsets: vec![],
        };
        assert_eq!(
            endpoints.metadata.as_ref().unwrap().name,
            Some("my-endpoints".to_string())
        );
    }

    #[test]
    fn test_endpoints_serialize() {
        let endpoints = Endpoints {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("my-endpoints".to_string()),
                ..Default::default()
            }),
            subsets: vec![EndpointSubset {
                addresses: vec![EndpointAddress {
                    ip: "10.0.0.1".to_string(),
                    ..Default::default()
                }],
                ports: vec![EndpointPort {
                    name: "http".to_string(),
                    port: 8080,
                    ..Default::default()
                }],
                ..Default::default()
            }],
        };
        let json = serde_json::to_string(&endpoints).unwrap();
        assert!(json.contains("\"name\":\"my-endpoints\""));
        assert!(json.contains("\"ip\":\"10.0.0.1\""));
        assert!(json.contains("\"port\":8080"));
    }

    #[test]
    fn test_endpoints_deserialize() {
        let json = r#"{
            "metadata": {"name": "my-endpoints"},
            "subsets": [{
                "addresses": [{"ip": "10.0.0.1"}],
                "ports": [{"port": 8080}]
            }]
        }"#;
        let endpoints: Endpoints = serde_json::from_str(json).unwrap();
        assert_eq!(
            endpoints.metadata.as_ref().unwrap().name,
            Some("my-endpoints".to_string())
        );
        assert_eq!(endpoints.subsets.len(), 1);
        assert_eq!(endpoints.subsets[0].addresses[0].ip, "10.0.0.1");
    }

    #[test]
    fn test_endpoints_list_with_items() {
        let list = EndpointsList {
            type_meta: TypeMeta::default(),
            metadata: Some(ListMeta {
                resource_version: Some("12345".to_string()),
                ..Default::default()
            }),
            items: vec![Endpoints {
                type_meta: TypeMeta::default(),
                metadata: Some(ObjectMeta {
                    name: Some("my-endpoints".to_string()),
                    ..Default::default()
                }),
                subsets: vec![],
            }],
        };
        assert_eq!(list.items.len(), 1);
        assert_eq!(
            list.metadata.as_ref().unwrap().resource_version,
            Some("12345".to_string())
        );
    }

    // Round trip tests
    #[test]
    fn test_service_round_trip() {
        let mut selector = BTreeMap::new();
        selector.insert("app".to_string(), "nginx".to_string());

        let original = Service {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("my-service".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(ServiceSpec {
                cluster_ip: "10.0.0.1".to_string(),
                selector,
                type_: Some(service_type::LOAD_BALANCER.to_string()),
                ports: vec![ServicePort {
                    name: "http".to_string(),
                    port: 80,
                    target_port: Some(IntOrString::Int(8080)),
                    ..Default::default()
                }],
                session_affinity: service_affinity::CLIENT_IP.to_string(),
                ..Default::default()
            }),
            status: Some(ServiceStatus {
                load_balancer: Some(LoadBalancerStatus {
                    ingress: vec![LoadBalancerIngress {
                        ip: "192.168.1.1".to_string(),
                        ..Default::default()
                    }],
                }),
                ..Default::default()
            }),
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Service = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_endpoints_round_trip() {
        let original = Endpoints {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("my-endpoints".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            subsets: vec![EndpointSubset {
                addresses: vec![EndpointAddress {
                    ip: "10.0.0.1".to_string(),
                    hostname: "pod-1".to_string(),
                    target_ref: Some(ObjectReference {
                        kind: "Pod".to_string(),
                        name: Some("pod-1".to_string()),
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                ports: vec![EndpointPort {
                    name: "http".to_string(),
                    port: 8080,
                    protocol: protocol::TCP.to_string(),
                    ..Default::default()
                }],
                ..Default::default()
            }],
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Endpoints = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }
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

impl ApplyDefaults for Service {
    fn apply_defaults(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "Service".to_string();
        }
    }
}

impl ApplyDefaults for ServiceList {
    fn apply_defaults(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "ServiceList".to_string();
        }
    }
}

impl ApplyDefaults for Endpoints {
    fn apply_defaults(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "Endpoints".to_string();
        }
    }
}

impl ApplyDefaults for EndpointsList {
    fn apply_defaults(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "EndpointsList".to_string();
        }
    }
}

// ----------------------------------------------------------------------------
// Version Conversion Placeholder (using UnimplementedConversion)
// ----------------------------------------------------------------------------

impl UnimplementedConversion for Service {}
impl UnimplementedConversion for Endpoints {}

// ----------------------------------------------------------------------------
// Protobuf Placeholder (using macro)
// ----------------------------------------------------------------------------

impl_unimplemented_prost_message!(Service);
impl_unimplemented_prost_message!(ServiceList);
impl_unimplemented_prost_message!(Endpoints);
impl_unimplemented_prost_message!(EndpointsList);
