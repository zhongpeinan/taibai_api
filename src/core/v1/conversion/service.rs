//! Service and Endpoints conversion implementations
//!
//! Includes: Service, ServiceList, ServiceSpec, ServiceStatus, ServicePort,
//! LoadBalancerStatus, LoadBalancerIngress, PortStatus, SessionAffinityConfig,
//! ClientIPConfig, Endpoints, EndpointsList, EndpointSubset, EndpointAddress, EndpointPort

use crate::common::traits::{ApplyDefault, FromInternal, ToInternal};
use crate::core::internal;
use crate::core::v1::{reference, service};

use super::helpers::*;

// ============================================================================
// Helper Functions - Enum Conversions
// ============================================================================

/// Convert Option<ServiceType> to String
fn service_type_to_string(t: Option<internal::ServiceType>) -> String {
    match t {
        Some(st) => serde_json::to_value(&st)
            .ok()
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or_default(),
        None => String::new(),
    }
}

/// Convert String to Option<ServiceType>
fn string_to_service_type(s: String) -> Option<internal::ServiceType> {
    if s.is_empty() {
        return None;
    }
    serde_json::from_value(serde_json::Value::String(s)).ok()
}

/// Convert ServiceAffinity to SessionAffinityType (direct enum mapping)
fn service_affinity_to_session_affinity(
    affinity: internal::ServiceAffinity,
) -> internal::SessionAffinityType {
    match affinity {
        internal::ServiceAffinity::ClientIp => internal::SessionAffinityType::ClientIp,
        internal::ServiceAffinity::None => internal::SessionAffinityType::None,
    }
}

/// Convert SessionAffinityType to ServiceAffinity (direct enum mapping)
fn session_affinity_to_service_affinity(
    affinity: internal::SessionAffinityType,
) -> internal::ServiceAffinity {
    match affinity {
        internal::SessionAffinityType::ClientIp => internal::ServiceAffinity::ClientIp,
        internal::SessionAffinityType::None => internal::ServiceAffinity::None,
    }
}

/// Convert Option<ServiceExternalTrafficPolicy> to String
fn external_traffic_policy_to_string(
    policy: Option<internal::ServiceExternalTrafficPolicy>,
) -> String {
    match policy {
        Some(p) => serde_json::to_value(&p)
            .ok()
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or_default(),
        None => String::new(),
    }
}

/// Convert String to Option<ServiceExternalTrafficPolicy>
fn string_to_external_traffic_policy(s: String) -> Option<internal::ServiceExternalTrafficPolicy> {
    if s.is_empty() {
        return None;
    }
    serde_json::from_value(serde_json::Value::String(s)).ok()
}

/// Convert Protocol enum to String
fn protocol_to_string(protocol: internal::Protocol) -> String {
    serde_json::to_value(&protocol)
        .ok()
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_else(|| "TCP".to_string())
}

/// Convert String to Protocol enum
fn string_to_protocol(s: String) -> internal::Protocol {
    serde_json::from_value(serde_json::Value::String(s)).unwrap_or_default()
}

// ============================================================================
// Service
// ============================================================================

impl ToInternal<internal::service::Service> for service::Service {
    fn to_internal(self) -> internal::service::Service {
        internal::service::Service {
            type_meta: crate::common::TypeMeta::default(),
            metadata: option_object_meta_to_meta(self.metadata),
            spec: self.spec.map(|s| s.to_internal()),
            status: self.status.map(|s| s.to_internal()).unwrap_or_default(),
        }
    }
}

impl FromInternal<internal::service::Service> for service::Service {
    fn from_internal(value: internal::service::Service) -> Self {
        let mut result = Self {
            type_meta: crate::common::TypeMeta::default(),
            metadata: meta_to_option_object_meta(value.metadata),
            spec: value.spec.map(service::ServiceSpec::from_internal),
            status: Some(service::ServiceStatus::from_internal(value.status)),
        };
        result.apply_default();
        result
    }
}

// ============================================================================
// ServiceList
// ============================================================================

impl ToInternal<internal::service::ServiceList> for service::ServiceList {
    fn to_internal(self) -> internal::service::ServiceList {
        internal::service::ServiceList {
            type_meta: crate::common::TypeMeta::default(),
            metadata: self.metadata.unwrap_or_default(),
            items: self.items.into_iter().map(|i| i.to_internal()).collect(),
        }
    }
}

impl FromInternal<internal::service::ServiceList> for service::ServiceList {
    fn from_internal(value: internal::service::ServiceList) -> Self {
        let mut result = Self {
            type_meta: crate::common::TypeMeta::default(),
            metadata: Some(value.metadata),
            items: value
                .items
                .into_iter()
                .map(service::Service::from_internal)
                .collect(),
        };
        result.apply_default();
        result
    }
}

// ============================================================================
// ServiceSpec
// ============================================================================

impl ToInternal<internal::service::ServiceSpec> for service::ServiceSpec {
    fn to_internal(self) -> internal::service::ServiceSpec {
        internal::service::ServiceSpec {
            ports: self.ports.into_iter().map(|p| p.to_internal()).collect(),
            selector: self.selector,
            cluster_ip: self.cluster_ip,
            cluster_ips: self.cluster_ips,
            r#type: service_type_to_string(self.type_),
            session_affinity: service_affinity_to_session_affinity(self.session_affinity),
            session_affinity_config: self.session_affinity_config.map(|c| c.to_internal()),
            external_ips: self.external_ips,
            external_name: self.external_name,
            external_traffic_policy: external_traffic_policy_to_string(
                self.external_traffic_policy,
            ),
            health_check_node_port: self.health_check_node_port,
            ip_families: self.ip_families,
            ip_families_policy: self.ip_family_policy,
            load_balancer_class: self.load_balancer_class,
            internal_traffic_policy: self.internal_traffic_policy,
            load_balancer_ip: self.load_balancer_ip,
            load_balancer_source_ranges: self.load_balancer_source_ranges,
            publish_not_ready_addresses: self.publish_not_ready_addresses,
            allocate_load_balancer_node_ports: self.allocate_load_balancer_node_ports,
            traffic_distribution: self.traffic_distribution,
        }
    }
}

impl FromInternal<internal::service::ServiceSpec> for service::ServiceSpec {
    fn from_internal(value: internal::service::ServiceSpec) -> Self {
        Self {
            ports: value
                .ports
                .into_iter()
                .map(service::ServicePort::from_internal)
                .collect(),
            selector: value.selector,
            cluster_ip: value.cluster_ip,
            cluster_ips: value.cluster_ips,
            type_: string_to_service_type(value.r#type),
            external_ips: value.external_ips,
            session_affinity: session_affinity_to_service_affinity(value.session_affinity),
            load_balancer_ip: value.load_balancer_ip,
            load_balancer_source_ranges: value.load_balancer_source_ranges,
            external_name: value.external_name,
            external_traffic_policy: string_to_external_traffic_policy(
                value.external_traffic_policy,
            ),
            health_check_node_port: value.health_check_node_port,
            publish_not_ready_addresses: value.publish_not_ready_addresses,
            session_affinity_config: value
                .session_affinity_config
                .map(service::SessionAffinityConfig::from_internal),
            ip_families: value.ip_families,
            ip_family_policy: value.ip_families_policy,
            allocate_load_balancer_node_ports: value.allocate_load_balancer_node_ports,
            load_balancer_class: value.load_balancer_class,
            internal_traffic_policy: value.internal_traffic_policy,
            traffic_distribution: value.traffic_distribution,
        }
    }
}

// ============================================================================
// ServiceStatus
// ============================================================================

impl ToInternal<internal::service::ServiceStatus> for service::ServiceStatus {
    fn to_internal(self) -> internal::service::ServiceStatus {
        internal::service::ServiceStatus {
            load_balancer: self.load_balancer.map(|lb| lb.to_internal()),
            conditions: self.conditions,
        }
    }
}

impl FromInternal<internal::service::ServiceStatus> for service::ServiceStatus {
    fn from_internal(value: internal::service::ServiceStatus) -> Self {
        Self {
            load_balancer: value
                .load_balancer
                .map(service::LoadBalancerStatus::from_internal),
            conditions: value.conditions,
        }
    }
}

// ============================================================================
// ServicePort
// ============================================================================

impl ToInternal<internal::service::ServicePort> for service::ServicePort {
    fn to_internal(self) -> internal::service::ServicePort {
        internal::service::ServicePort {
            name: self.name,
            protocol: string_to_protocol(self.protocol),
            port: self.port,
            node_port: self.node_port,
            target_port: self.target_port,
            app_protocol: self.app_protocol.unwrap_or_default(),
        }
    }
}

impl FromInternal<internal::service::ServicePort> for service::ServicePort {
    fn from_internal(value: internal::service::ServicePort) -> Self {
        Self {
            name: value.name,
            protocol: protocol_to_string(value.protocol),
            app_protocol: if value.app_protocol.is_empty() {
                None
            } else {
                Some(value.app_protocol)
            },
            port: value.port,
            target_port: value.target_port,
            node_port: value.node_port,
        }
    }
}

// ============================================================================
// LoadBalancerStatus
// ============================================================================

impl ToInternal<internal::service::LoadBalancerStatus> for service::LoadBalancerStatus {
    fn to_internal(self) -> internal::service::LoadBalancerStatus {
        internal::service::LoadBalancerStatus {
            ingress: self.ingress.into_iter().map(|i| i.to_internal()).collect(),
        }
    }
}

impl FromInternal<internal::service::LoadBalancerStatus> for service::LoadBalancerStatus {
    fn from_internal(value: internal::service::LoadBalancerStatus) -> Self {
        Self {
            ingress: value
                .ingress
                .into_iter()
                .map(service::LoadBalancerIngress::from_internal)
                .collect(),
        }
    }
}

// ============================================================================
// LoadBalancerIngress
// ============================================================================

impl ToInternal<internal::service::LoadBalancerIngress> for service::LoadBalancerIngress {
    fn to_internal(self) -> internal::service::LoadBalancerIngress {
        internal::service::LoadBalancerIngress {
            ip: self.ip,
            hostname: self.hostname,
            ip_mode: self.ip_mode.unwrap_or_default(),
            ports: self.ports.into_iter().map(|p| p.to_internal()).collect(),
        }
    }
}

impl FromInternal<internal::service::LoadBalancerIngress> for service::LoadBalancerIngress {
    fn from_internal(value: internal::service::LoadBalancerIngress) -> Self {
        Self {
            ip: value.ip,
            hostname: value.hostname,
            ip_mode: if value.ip_mode.is_empty() {
                None
            } else {
                Some(value.ip_mode)
            },
            ports: value
                .ports
                .into_iter()
                .map(service::PortStatus::from_internal)
                .collect(),
        }
    }
}

// ============================================================================
// PortStatus
// ============================================================================

impl ToInternal<internal::service::PortStatus> for service::PortStatus {
    fn to_internal(self) -> internal::service::PortStatus {
        internal::service::PortStatus {
            port: self.port,
            protocol: string_to_protocol(self.protocol),
            error: self.error.unwrap_or_default(),
        }
    }
}

impl FromInternal<internal::service::PortStatus> for service::PortStatus {
    fn from_internal(value: internal::service::PortStatus) -> Self {
        Self {
            port: value.port,
            protocol: protocol_to_string(value.protocol),
            error: if value.error.is_empty() {
                None
            } else {
                Some(value.error)
            },
        }
    }
}

// ============================================================================
// SessionAffinityConfig
// ============================================================================

impl ToInternal<internal::service::SessionAffinityConfig> for service::SessionAffinityConfig {
    fn to_internal(self) -> internal::service::SessionAffinityConfig {
        internal::service::SessionAffinityConfig {
            client_ip: self.client_ip.map(|c| c.to_internal()),
        }
    }
}

impl FromInternal<internal::service::SessionAffinityConfig> for service::SessionAffinityConfig {
    fn from_internal(value: internal::service::SessionAffinityConfig) -> Self {
        Self {
            client_ip: value.client_ip.map(service::ClientIPConfig::from_internal),
        }
    }
}

// ============================================================================
// ClientIPConfig
// ============================================================================

impl ToInternal<internal::service::ClientIPConfig> for service::ClientIPConfig {
    fn to_internal(self) -> internal::service::ClientIPConfig {
        internal::service::ClientIPConfig {
            timeout_seconds: self.timeout_seconds,
        }
    }
}

impl FromInternal<internal::service::ClientIPConfig> for service::ClientIPConfig {
    fn from_internal(value: internal::service::ClientIPConfig) -> Self {
        Self {
            timeout_seconds: value.timeout_seconds,
        }
    }
}

// ============================================================================
// Endpoints
// ============================================================================

impl ToInternal<internal::endpoints::Endpoints> for service::Endpoints {
    fn to_internal(self) -> internal::endpoints::Endpoints {
        internal::endpoints::Endpoints {
            type_meta: crate::common::TypeMeta::default(),
            metadata: option_object_meta_to_meta(self.metadata),
            subsets: self
                .subsets
                .into_iter()
                .map(|s: service::EndpointSubset| s.to_internal())
                .collect(),
        }
    }
}

impl FromInternal<internal::endpoints::Endpoints> for service::Endpoints {
    fn from_internal(value: internal::endpoints::Endpoints) -> Self {
        let mut result = Self {
            type_meta: crate::common::TypeMeta::default(),
            metadata: meta_to_option_object_meta(value.metadata),
            subsets: value
                .subsets
                .into_iter()
                .map(service::EndpointSubset::from_internal)
                .collect(),
        };
        result.apply_default();
        result
    }
}

// ============================================================================
// EndpointsList
// ============================================================================

impl ToInternal<internal::endpoints::EndpointsList> for service::EndpointsList {
    fn to_internal(self) -> internal::endpoints::EndpointsList {
        internal::endpoints::EndpointsList {
            type_meta: crate::common::TypeMeta::default(),
            metadata: self.metadata.unwrap_or_default(),
            items: self
                .items
                .into_iter()
                .map(|i: service::Endpoints| i.to_internal())
                .collect(),
        }
    }
}

impl FromInternal<internal::endpoints::EndpointsList> for service::EndpointsList {
    fn from_internal(value: internal::endpoints::EndpointsList) -> Self {
        let mut result = Self {
            type_meta: crate::common::TypeMeta::default(),
            metadata: Some(value.metadata),
            items: value
                .items
                .into_iter()
                .map(service::Endpoints::from_internal)
                .collect(),
        };
        result.apply_default();
        result
    }
}

// ============================================================================
// EndpointSubset
// ============================================================================

impl ToInternal<internal::endpoints::EndpointSubset> for service::EndpointSubset {
    fn to_internal(self) -> internal::endpoints::EndpointSubset {
        internal::endpoints::EndpointSubset {
            addresses: self
                .addresses
                .into_iter()
                .map(|a: service::EndpointAddress| a.to_internal())
                .collect(),
            not_ready_addresses: self
                .not_ready_addresses
                .into_iter()
                .map(|a: service::EndpointAddress| a.to_internal())
                .collect(),
            ports: self
                .ports
                .into_iter()
                .map(|p: service::EndpointPort| p.to_internal())
                .collect(),
        }
    }
}

impl FromInternal<internal::endpoints::EndpointSubset> for service::EndpointSubset {
    fn from_internal(value: internal::endpoints::EndpointSubset) -> Self {
        Self {
            addresses: value
                .addresses
                .into_iter()
                .map(service::EndpointAddress::from_internal)
                .collect(),
            not_ready_addresses: value
                .not_ready_addresses
                .into_iter()
                .map(service::EndpointAddress::from_internal)
                .collect(),
            ports: value
                .ports
                .into_iter()
                .map(service::EndpointPort::from_internal)
                .collect(),
        }
    }
}

// ============================================================================
// EndpointAddress
// ============================================================================

impl ToInternal<internal::endpoints::EndpointAddress> for service::EndpointAddress {
    fn to_internal(self) -> internal::endpoints::EndpointAddress {
        internal::endpoints::EndpointAddress {
            ip: self.ip,
            hostname: self.hostname,
            node_name: self.node_name.unwrap_or_default(),
            target_ref: self.target_ref.map(|r| r.to_internal()),
        }
    }
}

impl FromInternal<internal::endpoints::EndpointAddress> for service::EndpointAddress {
    fn from_internal(value: internal::endpoints::EndpointAddress) -> Self {
        Self {
            ip: value.ip,
            hostname: value.hostname,
            node_name: if value.node_name.is_empty() {
                None
            } else {
                Some(value.node_name)
            },
            target_ref: value
                .target_ref
                .map(reference::ObjectReference::from_internal),
        }
    }
}

// ============================================================================
// EndpointPort
// ============================================================================

impl ToInternal<internal::endpoints::EndpointPort> for service::EndpointPort {
    fn to_internal(self) -> internal::endpoints::EndpointPort {
        internal::endpoints::EndpointPort {
            name: self.name,
            port: self.port,
            protocol: string_to_protocol(self.protocol),
            app_protocol: self.app_protocol.unwrap_or_default(),
        }
    }
}

impl FromInternal<internal::endpoints::EndpointPort> for service::EndpointPort {
    fn from_internal(value: internal::endpoints::EndpointPort) -> Self {
        Self {
            name: value.name,
            port: value.port,
            protocol: protocol_to_string(value.protocol),
            app_protocol: if value.app_protocol.is_empty() {
                None
            } else {
                Some(value.app_protocol)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_roundtrip() {
        let v1_service = service::Service {
            type_meta: crate::common::TypeMeta::default(),
            metadata: Some(crate::common::ObjectMeta {
                name: Some("my-service".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(service::ServiceSpec {
                type_: Some(internal::ServiceType::ClusterIp),
                ports: vec![service::ServicePort {
                    name: "http".to_string(),
                    protocol: "TCP".to_string(),
                    port: 80,
                    target_port: Some(crate::common::IntOrString::Int(8080)),
                    node_port: None,
                    app_protocol: Some("http".to_string()),
                }],
                selector: std::collections::BTreeMap::from([(
                    "app".to_string(),
                    "my-app".to_string(),
                )]),
                ..Default::default()
            }),
            status: Some(service::ServiceStatus::default()),
        };

        let internal_service = v1_service.clone().to_internal();
        assert_eq!(
            internal_service.metadata.name,
            Some("my-service".to_string())
        );
        assert_eq!(internal_service.spec.as_ref().unwrap().ports.len(), 1);
        assert_eq!(
            internal_service.spec.as_ref().unwrap().ports[0].protocol,
            internal::Protocol::Tcp
        );

        let roundtrip = service::Service::from_internal(internal_service);
        assert_eq!(
            roundtrip.metadata.as_ref().unwrap().name,
            Some("my-service".to_string())
        );
        assert_eq!(roundtrip.spec.as_ref().unwrap().ports[0].name, "http");
    }

    #[test]
    fn test_endpoints_roundtrip() {
        let v1_endpoints = service::Endpoints {
            type_meta: crate::common::TypeMeta::default(),
            metadata: Some(crate::common::ObjectMeta {
                name: Some("my-endpoints".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            subsets: vec![service::EndpointSubset {
                addresses: vec![service::EndpointAddress {
                    ip: "192.168.1.1".to_string(),
                    hostname: "host1".to_string(),
                    node_name: Some("node1".to_string()),
                    target_ref: None,
                }],
                not_ready_addresses: vec![],
                ports: vec![service::EndpointPort {
                    name: "http".to_string(),
                    port: 8080,
                    protocol: "TCP".to_string(),
                    app_protocol: None,
                }],
            }],
        };

        let internal_endpoints = v1_endpoints.clone().to_internal();
        assert_eq!(
            internal_endpoints.metadata.name,
            Some("my-endpoints".to_string())
        );
        assert_eq!(internal_endpoints.subsets.len(), 1);
        assert_eq!(internal_endpoints.subsets[0].addresses[0].ip, "192.168.1.1");

        let roundtrip = service::Endpoints::from_internal(internal_endpoints);
        assert_eq!(
            roundtrip.metadata.as_ref().unwrap().name,
            Some("my-endpoints".to_string())
        );
        assert_eq!(roundtrip.subsets[0].addresses[0].ip, "192.168.1.1");
    }

    #[test]
    fn test_service_type_enum_conversion() {
        let v1_spec = service::ServiceSpec {
            type_: Some(internal::ServiceType::LoadBalancer),
            ..Default::default()
        };

        let internal_spec = v1_spec.to_internal();
        assert_eq!(internal_spec.r#type, "LoadBalancer");

        let roundtrip = service::ServiceSpec::from_internal(internal_spec);
        assert_eq!(roundtrip.type_, Some(internal::ServiceType::LoadBalancer));
    }

    #[test]
    fn test_protocol_string_enum_conversion() {
        let v1_port = service::ServicePort {
            protocol: "UDP".to_string(),
            port: 53,
            ..Default::default()
        };

        let internal_port = v1_port.to_internal();
        assert_eq!(internal_port.protocol, internal::Protocol::Udp);

        let roundtrip = service::ServicePort::from_internal(internal_port);
        assert_eq!(roundtrip.protocol, "UDP");
    }

    #[test]
    fn test_endpoint_address_node_name_conversion() {
        let v1_address = service::EndpointAddress {
            ip: "10.0.0.1".to_string(),
            node_name: Some("my-node".to_string()),
            ..Default::default()
        };

        let internal_address = v1_address.clone().to_internal();
        assert_eq!(internal_address.node_name, "my-node");

        let roundtrip = service::EndpointAddress::from_internal(internal_address);
        assert_eq!(roundtrip.node_name, Some("my-node".to_string()));

        // Test empty node_name
        let v1_empty = service::EndpointAddress {
            ip: "10.0.0.2".to_string(),
            node_name: None,
            ..Default::default()
        };

        let internal_empty = v1_empty.to_internal();
        assert_eq!(internal_empty.node_name, "");

        let roundtrip_empty = service::EndpointAddress::from_internal(internal_empty);
        assert_eq!(roundtrip_empty.node_name, None);
    }
}
