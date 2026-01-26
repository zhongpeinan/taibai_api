//! Service validation for Kubernetes core/v1 API
//!
//! Ported from k8s.io/kubernetes/pkg/apis/core/validation/validation.go

use crate::common::validation::{
    BadValue, Error, ErrorList, ErrorType, Path, forbidden, invalid, not_supported, required,
};
use crate::core::internal::{ServiceAffinity, ServiceType};
use crate::core::v1::service::{
    CLUSTER_IP_NONE, Service, ServicePort, ServiceSpec, protocol, service_affinity, service_type,
};
use std::collections::HashSet;
use std::sync::LazyLock;

// ============================================================================
// Constants
// ============================================================================

/// Maximum client IP service affinity timeout in seconds (24 hours)
pub const MAX_CLIENT_IP_SERVICE_AFFINITY_SECONDS: i32 = 86400;

/// Minimum port number
pub const MIN_PORT: i32 = 1;

/// Maximum port number
pub const MAX_PORT: i32 = 65535;

/// Node port range (30000-32767 by default in k8s)
pub const MIN_NODE_PORT: i32 = 30000;
pub const MAX_NODE_PORT: i32 = 32767;

static SUPPORTED_SESSION_AFFINITY_TYPES: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| HashSet::from([service_affinity::CLIENT_IP, service_affinity::NONE]));

static SUPPORTED_SERVICE_TYPES: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    HashSet::from([
        service_type::CLUSTER_IP,
        service_type::NODE_PORT,
        service_type::LOAD_BALANCER,
        service_type::EXTERNAL_NAME,
    ])
});

static SUPPORTED_PROTOCOLS: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| HashSet::from([protocol::TCP, protocol::UDP, protocol::SCTP]));

// ============================================================================
// Service Validators
// ============================================================================

/// Validates a Service resource
pub fn validate_service(service: &Service, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate metadata (Service is namespaced)
    if let Some(ref metadata) = service.metadata {
        all_errs.extend(crate::common::validation::validate_object_meta(
            metadata,
            true, // Namespace required
            |name: &str, _prefix: bool| crate::common::validation::is_dns1123_subdomain(name),
            &path.child("metadata"),
        ));
    } else {
        all_errs.push(required(&path.child("metadata"), "metadata is required"));
    }

    // Validate spec
    if let Some(ref spec) = service.spec {
        all_errs.extend(validate_service_spec(spec, &path.child("spec")));
    } else {
        all_errs.push(required(&path.child("spec"), "spec is required"));
    }

    all_errs
}

/// Validates a Service spec
pub fn validate_service_spec(spec: &ServiceSpec, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    let is_headless = is_headless_service(spec);
    let service_type = get_service_type(spec);

    // Ports required unless ExternalName or headless
    if spec.ports.is_empty() && !is_headless && service_type != ServiceType::ExternalName {
        all_errs.push(required(&path.child("ports"), "ports are required"));
    }

    // Type-specific validation
    match service_type {
        ServiceType::LoadBalancer => {
            if is_headless {
                all_errs.push(invalid(
                    &path.child("clusterIP"),
                    BadValue::String(CLUSTER_IP_NONE.to_string()),
                    "may not be set to 'None' for LoadBalancer services",
                ));
            }
        }
        ServiceType::NodePort => {
            if is_headless {
                all_errs.push(invalid(
                    &path.child("clusterIP"),
                    BadValue::String(CLUSTER_IP_NONE.to_string()),
                    "may not be set to 'None' for NodePort services",
                ));
            }
        }
        ServiceType::ExternalName => {
            // ExternalName services must not have ClusterIPs
            if !spec.cluster_ips.is_empty() || !spec.cluster_ip.is_empty() {
                all_errs.push(forbidden(
                    &path.child("clusterIPs"),
                    "may not be set for ExternalName services",
                ));
            }

            // ExternalName services must not have IPFamilies or IPFamilyPolicy
            if !spec.ip_families.is_empty() {
                all_errs.push(forbidden(
                    &path.child("ipFamilies"),
                    "may not be set for ExternalName services",
                ));
            }
            if spec.ip_family_policy.is_some() {
                all_errs.push(forbidden(
                    &path.child("ipFamilyPolicy"),
                    "may not be set for ExternalName services",
                ));
            }

            // ExternalName must be a valid DNS subdomain
            let external_name = spec.external_name.trim_end_matches('.');
            if !external_name.is_empty() {
                let errors = crate::common::validation::is_dns1123_subdomain(external_name);
                if !errors.is_empty() {
                    for err in errors {
                        all_errs.push(invalid(
                            &path.child("externalName"),
                            BadValue::String(spec.external_name.clone()),
                            &err,
                        ));
                    }
                }
            } else {
                all_errs.push(required(
                    &path.child("externalName"),
                    "externalName is required for ExternalName services",
                ));
            }
        }
        _ => {}
    }

    // Validate ports
    let mut all_port_names = HashSet::new();
    let require_port_name = spec.ports.len() > 1;

    for (i, port) in spec.ports.iter().enumerate() {
        all_errs.extend(validate_service_port(
            port,
            require_port_name,
            is_headless,
            &mut all_port_names,
            &path.child("ports").index(i),
        ));
    }

    // Validate selector (optional, but if present must be valid labels)
    if !spec.selector.is_empty() {
        all_errs.extend(crate::common::validation::validate_labels(
            &spec.selector,
            &path.child("selector"),
        ));
    }

    // Validate session affinity
    let session_affinity = &spec.session_affinity;
    if !SUPPORTED_SESSION_AFFINITY_TYPES.contains(affinity_to_str(session_affinity)) {
        let valid: Vec<&str> = SUPPORTED_SESSION_AFFINITY_TYPES.iter().copied().collect();
        all_errs.push(not_supported(
            &path.child("sessionAffinity"),
            BadValue::String(affinity_to_str(session_affinity).to_string()),
            &valid,
        ));
    }

    // Validate session affinity config
    if matches!(session_affinity, ServiceAffinity::ClientIp) {
        all_errs.extend(validate_client_ip_affinity_config(
            spec,
            &path.child("sessionAffinityConfig"),
        ));
    } else if matches!(session_affinity, ServiceAffinity::None) {
        if spec.session_affinity_config.is_some() {
            all_errs.push(forbidden(
                &path.child("sessionAffinityConfig"),
                &format!(
                    "must not be set when session affinity is {}",
                    service_affinity::NONE
                ),
            ));
        }
    }

    // Validate service type
    if !SUPPORTED_SERVICE_TYPES.contains(type_to_str(&service_type)) {
        let valid: Vec<&str> = SUPPORTED_SERVICE_TYPES.iter().copied().collect();
        all_errs.push(not_supported(
            &path.child("type"),
            BadValue::String(type_to_str(&service_type).to_string()),
            &valid,
        ));
    }

    // ClusterIP services cannot have nodePort
    if service_type == ServiceType::ClusterIp {
        for (i, port) in spec.ports.iter().enumerate() {
            if let Some(node_port) = port.node_port {
                if node_port != 0 {
                    all_errs.push(forbidden(
                        &path.child("ports").index(i).child("nodePort"),
                        "may not be used when `type` is 'ClusterIP'",
                    ));
                }
            }
        }
    }

    // Check for duplicate (protocol, port) pairs
    let mut ports_seen = HashSet::new();
    for (i, port) in spec.ports.iter().enumerate() {
        let key = (port.protocol.as_str(), port.port);
        if ports_seen.contains(&key) {
            all_errs.push(Error {
                error_type: ErrorType::Duplicate,
                field: path.child("ports").index(i).to_string(),
                bad_value: Some(BadValue::String(format!(
                    "port {} with protocol {}",
                    port.port, port.protocol
                ))),
                detail: format!(
                    "port {} with protocol {} already exists",
                    port.port, port.protocol
                ),
                origin: None,
                covered_by_declarative: false,
            });
        }
        ports_seen.insert(key);
    }

    // Check for duplicate (protocol, nodePort) pairs
    let mut node_ports_seen = HashSet::new();
    for (i, port) in spec.ports.iter().enumerate() {
        if let Some(node_port) = port.node_port {
            if node_port != 0 {
                let key = (port.protocol.as_str(), node_port);
                if node_ports_seen.contains(&key) {
                    all_errs.push(Error {
                        error_type: ErrorType::Duplicate,
                        field: path.child("ports").index(i).child("nodePort").to_string(),
                        bad_value: Some(BadValue::Int(node_port.into())),
                        detail: format!(
                            "nodePort {} with protocol {} already exists",
                            node_port, port.protocol
                        ),
                        origin: None,
                        covered_by_declarative: false,
                    });
                }
                node_ports_seen.insert(key);
            }
        }
    }

    // LoadBalancerSourceRanges only for LoadBalancer type
    if !spec.load_balancer_source_ranges.is_empty() && service_type != ServiceType::LoadBalancer {
        all_errs.push(forbidden(
            &path.child("loadBalancerSourceRanges"),
            "may only be used when `type` is 'LoadBalancer'",
        ));
    }

    // AllocateLoadBalancerNodePorts validation
    if spec.allocate_load_balancer_node_ports.is_some() && service_type != ServiceType::LoadBalancer
    {
        all_errs.push(forbidden(
            &path.child("allocateLoadBalancerNodePorts"),
            "may only be used when `type` is 'LoadBalancer'",
        ));
    }

    // LoadBalancer type requires allocateLoadBalancerNodePorts to be set
    if service_type == ServiceType::LoadBalancer && spec.allocate_load_balancer_node_ports.is_none()
    {
        all_errs.push(required(
            &path.child("allocateLoadBalancerNodePorts"),
            "must be set for LoadBalancer services",
        ));
    }

    // Validate external IPs (simplified - just check they're not empty)
    for (i, ip) in spec.external_ips.iter().enumerate() {
        if ip.trim().is_empty() {
            all_errs.push(invalid(
                &path.child("externalIPs").index(i),
                BadValue::String(ip.clone()),
                "must not be empty",
            ));
        }
        // TODO: Add IP validation in Phase 6
    }

    // Validate load balancer source ranges (simplified - check format)
    for (i, cidr) in spec.load_balancer_source_ranges.iter().enumerate() {
        let cidr_trimmed = cidr.trim();
        if cidr_trimmed.is_empty() {
            all_errs.push(invalid(
                &path.child("loadBalancerSourceRanges").index(i),
                BadValue::String(cidr.clone()),
                "must not be empty",
            ));
        }
        // TODO: Add CIDR validation in Phase 6
    }

    all_errs
}

/// Validates a single Service port
pub fn validate_service_port(
    port: &ServicePort,
    require_name: bool,
    _is_headless: bool,
    all_port_names: &mut HashSet<String>,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate name
    if require_name && port.name.is_empty() {
        all_errs.push(required(
            &path.child("name"),
            "name is required when multiple ports are specified",
        ));
    } else if !port.name.is_empty() {
        // Name must be a valid DNS label
        let errors = crate::common::validation::is_dns1123_label(&port.name);
        if !errors.is_empty() {
            for err in errors {
                all_errs.push(invalid(
                    &path.child("name"),
                    BadValue::String(port.name.clone()),
                    &err,
                ));
            }
        }

        // Check for duplicate names
        if all_port_names.contains(&port.name) {
            all_errs.push(Error {
                error_type: ErrorType::Duplicate,
                field: path.child("name").to_string(),
                bad_value: Some(BadValue::String(port.name.clone())),
                detail: format!("port name {} already exists", port.name),
                origin: None,
                covered_by_declarative: false,
            });
        } else {
            all_port_names.insert(port.name.clone());
        }
    }

    // Validate port number
    if port.port == 0 {
        all_errs.push(required(&path.child("port"), "port is required"));
    } else if port.port < MIN_PORT || port.port > MAX_PORT {
        all_errs.push(invalid(
            &path.child("port"),
            BadValue::Int(port.port.into()),
            &format!("must be between {} and {}", MIN_PORT, MAX_PORT),
        ));
    }

    // Validate protocol
    if port.protocol.is_empty() {
        all_errs.push(required(&path.child("protocol"), "protocol is required"));
    } else if !SUPPORTED_PROTOCOLS.contains(port.protocol.as_str()) {
        let valid: Vec<&str> = SUPPORTED_PROTOCOLS.iter().copied().collect();
        all_errs.push(not_supported(
            &path.child("protocol"),
            BadValue::String(port.protocol.clone()),
            &valid,
        ));
    }

    // Validate target port (simplified - just check it's set if provided)
    if let Some(ref target_port) = port.target_port {
        match target_port {
            crate::common::IntOrString::Int(val) => {
                if *val < MIN_PORT || *val > MAX_PORT {
                    all_errs.push(invalid(
                        &path.child("targetPort"),
                        BadValue::Int((*val).into()),
                        &format!("must be between {} and {}", MIN_PORT, MAX_PORT),
                    ));
                }
            }
            crate::common::IntOrString::String(name) => {
                if name.is_empty() {
                    all_errs.push(invalid(
                        &path.child("targetPort"),
                        BadValue::String(name.clone()),
                        "must not be empty",
                    ));
                }
                // Named ports are validated when referenced - accept any non-empty string here
            }
        }
    }

    // Validate appProtocol (if present, must be a qualified name)
    if let Some(ref app_protocol) = port.app_protocol {
        let errors = crate::common::validation::is_qualified_name(app_protocol);
        if !errors.is_empty() {
            for err in errors {
                all_errs.push(invalid(
                    &path.child("appProtocol"),
                    BadValue::String(app_protocol.clone()),
                    &err,
                ));
            }
        }
    }

    // Note: NodePort validation is done at the spec level (duplicate check, type check)

    all_errs
}

/// Validates ClientIP session affinity configuration
fn validate_client_ip_affinity_config(spec: &ServiceSpec, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if let Some(ref config) = spec.session_affinity_config {
        if let Some(ref client_ip) = config.client_ip {
            if let Some(timeout) = client_ip.timeout_seconds {
                if timeout <= 0 || timeout > MAX_CLIENT_IP_SERVICE_AFFINITY_SECONDS {
                    all_errs.push(invalid(
                        &path.child("clientIP").child("timeoutSeconds"),
                        BadValue::Int(timeout.into()),
                        &format!(
                            "must be greater than 0 and less than or equal to {}",
                            MAX_CLIENT_IP_SERVICE_AFFINITY_SECONDS
                        ),
                    ));
                }
            } else {
                all_errs.push(required(
                    &path.child("clientIP").child("timeoutSeconds"),
                    "timeoutSeconds is required when session affinity is ClientIP",
                ));
            }
        } else {
            all_errs.push(required(
                &path.child("clientIP"),
                "clientIP config is required when session affinity is ClientIP",
            ));
        }
    } else {
        all_errs.push(required(
            path,
            "sessionAffinityConfig is required when session affinity is ClientIP",
        ));
    }

    all_errs
}

/// Validates a Service update
pub fn validate_service_update(
    new_service: &Service,
    old_service: &Service,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate metadata update
    if let (Some(new_meta), Some(old_meta)) = (&new_service.metadata, &old_service.metadata) {
        all_errs.extend(crate::common::validation::validate_object_meta_update(
            new_meta,
            old_meta,
            &path.child("metadata"),
        ));
    }

    // Validate the new service spec
    if let Some(ref spec) = new_service.spec {
        all_errs.extend(validate_service_spec(spec, &path.child("spec")));
    }

    // TODO: Add immutability checks for ClusterIP, IPFamilies, etc. in future phases

    all_errs
}

/// Validates a Service status update
pub fn validate_service_status_update(
    new_service: &Service,
    old_service: &Service,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate metadata update
    if let (Some(new_meta), Some(old_meta)) = (&new_service.metadata, &old_service.metadata) {
        all_errs.extend(crate::common::validation::validate_object_meta_update(
            new_meta,
            old_meta,
            &path.child("metadata"),
        ));
    }

    // TODO: Add LoadBalancer status validation

    all_errs
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Checks if a service is headless (ClusterIP = "None")
fn is_headless_service(spec: &ServiceSpec) -> bool {
    // Check cluster_ips first (preferred field)
    if !spec.cluster_ips.is_empty() {
        return spec.cluster_ips.len() == 1 && spec.cluster_ips[0] == CLUSTER_IP_NONE;
    }

    // Fallback to cluster_ip (legacy field)
    spec.cluster_ip == CLUSTER_IP_NONE
}

/// Gets the service type from spec (with default to ClusterIP)
fn get_service_type(spec: &ServiceSpec) -> ServiceType {
    spec.type_
        .as_ref()
        .cloned()
        .unwrap_or(ServiceType::ClusterIp)
}

/// Converts ServiceType to &str for validation
fn type_to_str(t: &ServiceType) -> &str {
    match t {
        ServiceType::ClusterIp => service_type::CLUSTER_IP,
        ServiceType::NodePort => service_type::NODE_PORT,
        ServiceType::LoadBalancer => service_type::LOAD_BALANCER,
        ServiceType::ExternalName => service_type::EXTERNAL_NAME,
    }
}

/// Converts ServiceAffinity to &str for validation
fn affinity_to_str(a: &ServiceAffinity) -> &str {
    match a {
        ServiceAffinity::ClientIp => service_affinity::CLIENT_IP,
        ServiceAffinity::None => service_affinity::NONE,
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::meta::ObjectMeta;
    use crate::common::validation::ErrorType;
    use crate::common::{IntOrString, TypeMeta};
    use crate::core::internal::{ServiceAffinity, ServiceType};
    use crate::core::v1::service::{
        ClientIPConfig, Service, ServicePort, ServiceSpec, SessionAffinityConfig,
    };
    use std::collections::BTreeMap;

    #[test]
    fn test_validate_service_valid_cluster_ip() {
        let service = Service {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-service".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(ServiceSpec {
                ports: vec![ServicePort {
                    name: "http".to_string(),
                    port: 80,
                    protocol: protocol::TCP.to_string(),
                    target_port: Some(IntOrString::Int(8080)),
                    ..Default::default()
                }],
                selector: {
                    let mut map = BTreeMap::new();
                    map.insert("app".to_string(), "test".to_string());
                    map
                },
                type_: Some(ServiceType::ClusterIp),
                session_affinity: ServiceAffinity::None,
                ..Default::default()
            }),
            status: None,
        };

        let errs = validate_service(&service, &Path::nil());
        assert!(
            errs.is_empty(),
            "Expected no errors for valid ClusterIP service, got: {:?}",
            errs
        );
    }

    #[test]
    fn test_validate_service_missing_ports() {
        let service = Service {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-service".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(ServiceSpec {
                ports: vec![], // Missing ports
                type_: Some(ServiceType::ClusterIp),
                session_affinity: ServiceAffinity::None,
                ..Default::default()
            }),
            status: None,
        };

        let errs = validate_service(&service, &Path::nil());
        assert!(!errs.is_empty(), "Expected errors for missing ports");
        assert!(
            errs.errors
                .iter()
                .any(|e| e.error_type == ErrorType::Required && e.field.ends_with("ports"))
        );
    }

    #[test]
    fn test_validate_service_invalid_port_range() {
        let service = Service {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-service".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(ServiceSpec {
                ports: vec![ServicePort {
                    port: 70000, // Out of range
                    protocol: protocol::TCP.to_string(),
                    ..Default::default()
                }],
                type_: Some(ServiceType::ClusterIp),
                session_affinity: ServiceAffinity::None,
                ..Default::default()
            }),
            status: None,
        };

        let errs = validate_service(&service, &Path::nil());
        assert!(!errs.is_empty(), "Expected errors for invalid port");
        assert!(
            errs.errors
                .iter()
                .any(|e| e.error_type == ErrorType::Invalid && e.field.ends_with("port"))
        );
    }

    #[test]
    fn test_validate_service_external_name_requires_name() {
        let service = Service {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-service".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(ServiceSpec {
                ports: vec![],
                type_: Some(ServiceType::ExternalName),
                session_affinity: ServiceAffinity::None,
                external_name: String::new(), // Missing external name
                ..Default::default()
            }),
            status: None,
        };

        let errs = validate_service(&service, &Path::nil());
        assert!(!errs.is_empty(), "Expected errors for missing externalName");
        assert!(
            errs.errors
                .iter()
                .any(|e| e.error_type == ErrorType::Required && e.field.ends_with("externalName"))
        );
    }

    #[test]
    fn test_validate_service_external_name_valid() {
        let service = Service {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-service".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(ServiceSpec {
                ports: vec![],
                type_: Some(ServiceType::ExternalName),
                session_affinity: ServiceAffinity::None,
                external_name: "example.com".to_string(),
                ..Default::default()
            }),
            status: None,
        };

        let errs = validate_service(&service, &Path::nil());
        assert!(
            errs.is_empty(),
            "Expected no errors for valid ExternalName service, got: {:?}",
            errs
        );
    }

    #[test]
    fn test_validate_service_headless_cannot_be_load_balancer() {
        let service = Service {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-service".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(ServiceSpec {
                ports: vec![ServicePort {
                    port: 80,
                    protocol: protocol::TCP.to_string(),
                    ..Default::default()
                }],
                cluster_ip: CLUSTER_IP_NONE.to_string(),
                type_: Some(ServiceType::LoadBalancer),
                session_affinity: ServiceAffinity::None,
                ..Default::default()
            }),
            status: None,
        };

        let errs = validate_service(&service, &Path::nil());
        assert!(
            !errs.is_empty(),
            "Expected errors for headless LoadBalancer service"
        );
        assert!(
            errs.errors
                .iter()
                .any(|e| e.error_type == ErrorType::Invalid && e.field.ends_with("clusterIP"))
        );
    }

    #[test]
    fn test_validate_service_duplicate_ports() {
        let service = Service {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-service".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(ServiceSpec {
                ports: vec![
                    ServicePort {
                        name: "http1".to_string(),
                        port: 80,
                        protocol: protocol::TCP.to_string(),
                        ..Default::default()
                    },
                    ServicePort {
                        name: "http2".to_string(),
                        port: 80, // Duplicate port
                        protocol: protocol::TCP.to_string(),
                        ..Default::default()
                    },
                ],
                type_: Some(ServiceType::ClusterIp),
                session_affinity: ServiceAffinity::None,
                ..Default::default()
            }),
            status: None,
        };

        let errs = validate_service(&service, &Path::nil());
        assert!(!errs.is_empty(), "Expected errors for duplicate ports");
        assert!(
            errs.errors
                .iter()
                .any(|e| e.error_type == ErrorType::Duplicate && e.field.contains("ports"))
        );
    }

    #[test]
    fn test_validate_service_client_ip_affinity_requires_config() {
        let service = Service {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-service".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(ServiceSpec {
                ports: vec![ServicePort {
                    port: 80,
                    protocol: protocol::TCP.to_string(),
                    ..Default::default()
                }],
                type_: Some(ServiceType::ClusterIp),
                session_affinity: ServiceAffinity::ClientIp,
                // Missing session_affinity_config
                ..Default::default()
            }),
            status: None,
        };

        let errs = validate_service(&service, &Path::nil());
        assert!(
            !errs.is_empty(),
            "Expected errors for missing ClientIP config"
        );
        assert!(
            errs.errors
                .iter()
                .any(|e| e.error_type == ErrorType::Required
                    && e.field.contains("sessionAffinityConfig"))
        );
    }

    #[test]
    fn test_validate_service_client_ip_affinity_valid() {
        let service = Service {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-service".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(ServiceSpec {
                ports: vec![ServicePort {
                    port: 80,
                    protocol: protocol::TCP.to_string(),
                    ..Default::default()
                }],
                type_: Some(ServiceType::ClusterIp),
                session_affinity: ServiceAffinity::ClientIp,
                session_affinity_config: Some(SessionAffinityConfig {
                    client_ip: Some(ClientIPConfig {
                        timeout_seconds: Some(10800),
                    }),
                }),
                ..Default::default()
            }),
            status: None,
        };

        let errs = validate_service(&service, &Path::nil());
        assert!(
            errs.is_empty(),
            "Expected no errors for valid ClientIP affinity, got: {:?}",
            errs
        );
    }

    #[test]
    fn test_validate_service_cluster_ip_cannot_have_node_port() {
        let service = Service {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-service".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(ServiceSpec {
                ports: vec![ServicePort {
                    port: 80,
                    protocol: protocol::TCP.to_string(),
                    node_port: Some(30000), // Not allowed for ClusterIP
                    ..Default::default()
                }],
                type_: Some(ServiceType::ClusterIp),
                session_affinity: ServiceAffinity::None,
                ..Default::default()
            }),
            status: None,
        };

        let errs = validate_service(&service, &Path::nil());
        assert!(
            !errs.is_empty(),
            "Expected errors for nodePort in ClusterIP service"
        );
        assert!(
            errs.errors
                .iter()
                .any(|e| e.error_type == ErrorType::Forbidden && e.field.contains("nodePort"))
        );
    }

    #[test]
    fn test_validate_service_multiple_ports_require_names() {
        let service = Service {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-service".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(ServiceSpec {
                ports: vec![
                    ServicePort {
                        name: String::new(), // Missing name
                        port: 80,
                        protocol: protocol::TCP.to_string(),
                        ..Default::default()
                    },
                    ServicePort {
                        name: "https".to_string(),
                        port: 443,
                        protocol: protocol::TCP.to_string(),
                        ..Default::default()
                    },
                ],
                type_: Some(ServiceType::ClusterIp),
                session_affinity: ServiceAffinity::None,
                ..Default::default()
            }),
            status: None,
        };

        let errs = validate_service(&service, &Path::nil());
        assert!(!errs.is_empty(), "Expected errors for missing port name");
        assert!(
            errs.errors
                .iter()
                .any(|e| e.error_type == ErrorType::Required && e.field.ends_with("name"))
        );
    }
}
