//! Service types from the Kubernetes Core API
//!
//! This module contains types for Kubernetes Service resources,
//! which provide network access to applications.
//!
//! Source: k8s-pkg/apis/core/types.go

use crate::common::{ListMeta, ObjectMeta, TypeMeta};
use crate::core::internal::{Protocol, SessionAffinityType};
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// Spec defines the behavior of a service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<ServiceSpec>,
    /// Status represents the current status of a service.
    #[serde(default)]
    pub status: ServiceStatus,
}

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
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub r#type: String,
    /// SessionAffinity defines the affinity settings for client traffic.
    #[serde(default)]
    pub session_affinity: SessionAffinityType,
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
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub external_traffic_policy: String,
    /// HealthCheckNodePort specifies the healthcheck nodePort for the service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health_check_node_port: Option<i32>,
    /// IPFamily specifies whether this service has a preference for a particular IP family.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub ip_families: String,
    /// IPFamiliesPolicy is a set of policies for IP families.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub ip_families_policy: String,
    /// LoadBalancerClass is the class of the load balancer.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub load_balancer_class: String,
    /// InternalTrafficPolicy describes how nodes distribute service traffic.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub internal_traffic_policy: String,
    /// SupportsDualStack determines if a service allows dual stack.
    #[serde(default)]
    pub supports_dual_stack: bool,
    /// LoadBalancerIP will be used for load balancer IP.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub load_balancer_ip: String,
    /// LoadBalancerSourceRanges is a list of CIDR ranges.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub load_balancer_source_ranges: Vec<String>,
    /// PublishNotReadyAddresses indicates that any agent which deals with endpoints.
    #[serde(default)]
    pub publish_not_ready_addresses: bool,
    /// Selector is a label query over pods.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub selector_change: String,
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
    #[serde(default)]
    pub protocol: Protocol,
    /// The port number that is exposed.
    #[serde(default)]
    pub port: i32,
    /// The port on each node on which this service is exposed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_port: Option<i32>,
    /// The target port on pods selected by this service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_port: Option<i32>,
    /// The application protocol for this port.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub app_protocol: String,
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
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub ip_mode: String,
}

/// PortStatus represents the status of a port.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PortStatus {
    /// Port is the port number of the service port.
    #[serde(default)]
    pub port: i32,
    /// Protocol is the protocol of the service port.
    #[serde(default)]
    pub protocol: Protocol,
    /// Error is to record the problem with the service port.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub error: String,
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
mod tests {
    use super::*;

    // Service tests
    #[test]
    fn test_service_default() {
        let service = Service {
            type_meta: TypeMeta::default(),
            metadata: None,
            spec: None,
            status: ServiceStatus::default(),
        };
        assert!(service.spec.is_none());
        assert!(service.status.load_balancer.is_none());
    }

    #[test]
    fn test_service_spec_default() {
        let spec = ServiceSpec::default();
        assert!(spec.ports.is_empty());
        assert!(spec.selector.is_empty());
        assert_eq!(spec.session_affinity, SessionAffinityType::None);
    }

    #[test]
    fn test_service_spec_with_ports() {
        let spec = ServiceSpec {
            ports: vec![ServicePort {
                name: "http".to_string(),
                port: 80,
                ..Default::default()
            }],
            ..Default::default()
        };
        assert_eq!(spec.ports.len(), 1);
        assert_eq!(spec.ports[0].name, "http");
    }

    #[test]
    fn test_service_spec_serialize() {
        let spec = ServiceSpec {
            cluster_ip: "10.0.0.1".to_string(),
            r#type: "ClusterIP".to_string(),
            ..Default::default()
        };
        let json = serde_json::to_string(&spec).unwrap();
        assert!(json.contains(r#""clusterIp":"10.0.0.1""#));
        assert!(json.contains(r#""type":"ClusterIP""#));
    }

    #[test]
    fn test_service_status_default() {
        let status = ServiceStatus::default();
        assert!(status.load_balancer.is_none());
    }

    #[test]
    fn test_service_list_default() {
        let list = ServiceList::default();
        assert!(list.items.is_empty());
    }

    // ServicePort tests
    #[test]
    fn test_service_port_default() {
        let port = ServicePort::default();
        assert!(port.name.is_empty());
        assert_eq!(port.protocol, Protocol::Tcp);
        assert_eq!(port.port, 0);
    }

    #[test]
    fn test_service_port_with_fields() {
        let port = ServicePort {
            name: "https".to_string(),
            port: 443,
            node_port: Some(30443),
            target_port: Some(8443),
            ..Default::default()
        };
        assert_eq!(port.name, "https");
        assert_eq!(port.port, 443);
        assert_eq!(port.node_port, Some(30443));
        assert_eq!(port.target_port, Some(8443));
    }

    #[test]
    fn test_service_port_serialize() {
        let port = ServicePort {
            name: "metrics".to_string(),
            port: 9090,
            protocol: Protocol::Tcp,
            ..Default::default()
        };
        let json = serde_json::to_string(&port).unwrap();
        assert!(json.contains("\"name\":\"metrics\""));
        assert!(json.contains("\"port\":9090"));
        assert!(json.contains("\"protocol\":\"TCP\""));
    }

    #[test]
    fn test_service_port_round_trip() {
        let original = ServicePort {
            name: "web".to_string(),
            port: 8080,
            node_port: Some(30080),
            target_port: Some(80),
            ..Default::default()
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: ServicePort = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    // LoadBalancer tests
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
            ip: "10.0.0.100".to_string(),
            ..Default::default()
        };
        assert_eq!(ingress.ip, "10.0.0.100");
    }

    #[test]
    fn test_load_balancer_ingress_with_hostname() {
        let ingress = LoadBalancerIngress {
            hostname: "example.lb.com".to_string(),
            ..Default::default()
        };
        assert_eq!(ingress.hostname, "example.lb.com");
    }

    #[test]
    fn test_load_balancer_ingress_serialize() {
        let ingress = LoadBalancerIngress {
            ip: "1.2.3.4".to_string(),
            ..Default::default()
        };
        let json = serde_json::to_string(&ingress).unwrap();
        assert!(json.contains("\"ip\":\"1.2.3.4\""));
    }

    #[test]
    fn test_port_status_default() {
        let status = PortStatus::default();
        assert_eq!(status.port, 0);
        assert_eq!(status.protocol, Protocol::Tcp);
    }

    #[test]
    fn test_port_status_with_fields() {
        let status = PortStatus {
            port: 8080,
            protocol: Protocol::Tcp,
            error: "connection refused".to_string(),
        };
        assert_eq!(status.port, 8080);
        assert_eq!(status.error, "connection refused");
    }

    // SessionAffinity tests
    #[test]
    fn test_session_affinity_config_default() {
        let config = SessionAffinityConfig::default();
        assert!(config.client_ip.is_none());
    }

    #[test]
    fn test_session_affinity_config_with_timeout() {
        let config = SessionAffinityConfig {
            client_ip: Some(ClientIPConfig {
                timeout_seconds: Some(3600),
            }),
        };
        assert!(config.client_ip.is_some());
        assert_eq!(config.client_ip.unwrap().timeout_seconds, Some(3600));
    }

    #[test]
    fn test_client_ip_config_default() {
        let config = ClientIPConfig::default();
        assert!(config.timeout_seconds.is_none());
    }

    #[test]
    fn test_client_ip_config_with_timeout() {
        let config = ClientIPConfig {
            timeout_seconds: Some(10800),
        };
        assert_eq!(config.timeout_seconds, Some(10800));
    }

    #[test]
    fn test_session_affinity_config_serialize() {
        let config = SessionAffinityConfig {
            client_ip: Some(ClientIPConfig {
                timeout_seconds: Some(7200),
            }),
        };
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains(r#""clientIp""#));
        assert!(json.contains(r#""timeoutSeconds":7200"#));
    }

    // Integration tests
    #[test]
    fn test_service_with_multiple_ports() {
        let spec = ServiceSpec {
            ports: vec![
                ServicePort {
                    name: "http".to_string(),
                    port: 80,
                    ..Default::default()
                },
                ServicePort {
                    name: "https".to_string(),
                    port: 443,
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        assert_eq!(spec.ports.len(), 2);
    }

    #[test]
    fn test_service_with_load_balancer() {
        let status = ServiceStatus {
            load_balancer: Some(LoadBalancerStatus {
                ingress: vec![
                    LoadBalancerIngress {
                        ip: "10.0.0.1".to_string(),
                        ..Default::default()
                    },
                    LoadBalancerIngress {
                        ip: "10.0.0.2".to_string(),
                        ..Default::default()
                    },
                ],
            }),
        };
        assert!(status.load_balancer.is_some());
        assert_eq!(status.load_balancer.as_ref().unwrap().ingress.len(), 2);
    }

    #[test]
    fn test_service_list_with_services() {
        let list = ServiceList {
            items: vec![Service {
                type_meta: TypeMeta {
                    kind: Some("Service".to_string()),
                    api_version: Some("v1".to_string()),
                },
                ..Default::default()
            }],
            ..Default::default()
        };
        assert_eq!(list.items.len(), 1);
    }

    #[test]
    fn test_service_spec_with_selector() {
        let mut selector = std::collections::BTreeMap::new();
        selector.insert("app".to_string(), "nginx".to_string());
        selector.insert("tier".to_string(), "frontend".to_string());

        let spec = ServiceSpec {
            selector,
            ..Default::default()
        };
        assert_eq!(spec.selector.len(), 2);
    }
}
