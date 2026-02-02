//! Service validation for Kubernetes core internal API
//!
//! Ported from k8s.io/kubernetes/pkg/apis/core/validation/validation.go

use crate::common::validation::{
    BadValue, Error, ErrorList, ErrorType, Path, forbidden, invalid, not_supported, required,
};
use crate::core::internal::{
    IPFamily, IPFamilyPolicy, Protocol, Service, ServiceAffinity, ServicePort, ServiceSpec,
    ServiceType, load_balancer_ip_mode, protocol, service_affinity, service_type,
};
use std::collections::HashSet;
use std::net::IpAddr;
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

/// Cluster IP none constant
pub const CLUSTER_IP_NONE: &str = "None";

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
    all_errs.extend(crate::common::validation::validate_object_meta(
        &service.metadata,
        true, // Namespace required
        |name: &str, _prefix: bool| crate::common::validation::is_dns1123_subdomain(name),
        &path.child("metadata"),
    ));

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
    if spec.ports.is_empty() && !is_headless && service_type != service_type::EXTERNAL_NAME {
        all_errs.push(required(&path.child("ports"), "ports are required"));
    }

    // Type-specific validation
    match service_type {
        service_type::LOAD_BALANCER => {
            if is_headless {
                all_errs.push(invalid(
                    &path.child("clusterIP"),
                    BadValue::String(CLUSTER_IP_NONE.to_string()),
                    "may not be set to 'None' for LoadBalancer services",
                ));
            }
        }
        service_type::NODE_PORT => {
            if is_headless {
                all_errs.push(invalid(
                    &path.child("clusterIP"),
                    BadValue::String(CLUSTER_IP_NONE.to_string()),
                    "may not be set to 'None' for NodePort services",
                ));
            }
        }
        service_type::EXTERNAL_NAME => {
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
            if spec.ip_families_policy.is_some() {
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

    // IP family validation

    // Validate ClusterIP and ClusterIPs
    if !spec.cluster_ip.is_empty() && spec.cluster_ip != CLUSTER_IP_NONE {
        if !is_valid_ip(&spec.cluster_ip) {
            all_errs.push(invalid(
                &path.child("clusterIP"),
                BadValue::String(spec.cluster_ip.clone()),
                "must be a valid IP",
            ));
        }
    }

    if !spec.cluster_ips.is_empty() {
        if spec.cluster_ips.len() > 2 {
            all_errs.push(invalid(
                &path.child("clusterIPs"),
                BadValue::String(format!("{} entries", spec.cluster_ips.len())),
                "may specify no more than two IPs",
            ));
        }

        let has_none = spec.cluster_ips.iter().any(|ip| ip == CLUSTER_IP_NONE);
        if has_none && (spec.cluster_ips.len() != 1 || !is_headless) {
            all_errs.push(invalid(
                &path.child("clusterIPs"),
                BadValue::String(spec.cluster_ips.join(",")),
                "clusterIPs may only be set to [\"None\"] for headless services",
            ));
        }

        for (i, ip) in spec.cluster_ips.iter().enumerate() {
            if ip == CLUSTER_IP_NONE {
                continue;
            }
            if !is_valid_ip(ip) {
                all_errs.push(invalid(
                    &path.child("clusterIPs").index(i),
                    BadValue::String(ip.clone()),
                    "must be a valid IP",
                ));
            }
        }

        if !spec.cluster_ip.is_empty()
            && spec.cluster_ip != CLUSTER_IP_NONE
            && spec.cluster_ip != spec.cluster_ips[0]
        {
            all_errs.push(invalid(
                &path.child("clusterIP"),
                BadValue::String(spec.cluster_ip.clone()),
                "must match first entry of clusterIPs",
            ));
        }
    }

    // Validate IP families and policy
    if !spec.ip_families.is_empty() {
        if spec.ip_families.len() > 2 {
            all_errs.push(invalid(
                &path.child("ipFamilies"),
                BadValue::String(format!("{} entries", spec.ip_families.len())),
                "may specify no more than two IP families",
            ));
        }

        let mut family_seen = HashSet::new();
        for (i, family) in spec.ip_families.iter().enumerate() {
            let family_key = ip_family_to_str(family);
            if !family_seen.insert(family_key) {
                all_errs.push(Error {
                    error_type: ErrorType::Duplicate,
                    field: path.child("ipFamilies").index(i).to_string(),
                    bad_value: Some(BadValue::String(family_key.to_string())),
                    detail: "duplicate IP family".to_string(),
                    origin: None,
                    covered_by_declarative: false,
                });
            }
        }

        if !spec.cluster_ips.is_empty() && !is_headless {
            if spec.cluster_ips.len() != spec.ip_families.len() {
                all_errs.push(invalid(
                    &path.child("clusterIPs"),
                    BadValue::String(format!("{} entries", spec.cluster_ips.len())),
                    "must have same length as ipFamilies",
                ));
            } else {
                for (i, ip) in spec.cluster_ips.iter().enumerate() {
                    if let Some(ip_family) = ip_family_of_ip(ip) {
                        if ip_family != spec.ip_families[i] {
                            all_errs.push(invalid(
                                &path.child("clusterIPs").index(i),
                                BadValue::String(ip.clone()),
                                "IP family does not match ipFamilies entry",
                            ));
                        }
                    }
                }
            }
        }
    }

    if let Some(policy) = spec.ip_families_policy.as_ref() {
        match policy {
            IPFamilyPolicy::SingleStack => {
                if spec.ip_families.len() > 1 {
                    all_errs.push(invalid(
                        &path.child("ipFamilies"),
                        BadValue::String(format!("{} entries", spec.ip_families.len())),
                        "may not specify more than one IP family for SingleStack",
                    ));
                }
            }
            IPFamilyPolicy::RequireDualStack => {
                if !spec.ip_families.is_empty() && spec.ip_families.len() != 2 {
                    all_errs.push(invalid(
                        &path.child("ipFamilies"),
                        BadValue::String(format!("{} entries", spec.ip_families.len())),
                        "must specify two IP families for RequireDualStack",
                    ));
                }
            }
            IPFamilyPolicy::PreferDualStack => {}
        }
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
    let session_affinity = spec.session_affinity.as_ref();
    if let Some(affinity) = session_affinity {
        if !SUPPORTED_SESSION_AFFINITY_TYPES.contains(affinity_to_str(affinity)) {
            let valid: Vec<&str> = SUPPORTED_SESSION_AFFINITY_TYPES.iter().copied().collect();
            all_errs.push(not_supported(
                &path.child("sessionAffinity"),
                BadValue::String(affinity_to_str(affinity).to_string()),
                &valid,
            ));
        }
    }

    // Validate session affinity config
    if matches!(session_affinity, Some(ServiceAffinity::ClientIp)) {
        all_errs.extend(validate_client_ip_affinity_config(
            spec,
            &path.child("sessionAffinityConfig"),
        ));
    } else if matches!(session_affinity, Some(ServiceAffinity::None) | None) {
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
    if !SUPPORTED_SERVICE_TYPES.contains(service_type) {
        let valid: Vec<&str> = SUPPORTED_SERVICE_TYPES.iter().copied().collect();
        all_errs.push(not_supported(
            &path.child("type"),
            BadValue::String(service_type.to_string()),
            &valid,
        ));
    }

    // ClusterIP services cannot have nodePort
    if service_type == service_type::CLUSTER_IP {
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

    if service_type == service_type::EXTERNAL_NAME {
        for (i, port) in spec.ports.iter().enumerate() {
            if let Some(node_port) = port.node_port {
                if node_port != 0 {
                    all_errs.push(forbidden(
                        &path.child("ports").index(i).child("nodePort"),
                        "may not be used when `type` is 'ExternalName'",
                    ));
                }
            }
        }
    }

    // NodePort range validation
    for (i, port) in spec.ports.iter().enumerate() {
        if let Some(node_port) = port.node_port {
            if node_port != 0 && (node_port < MIN_NODE_PORT || node_port > MAX_NODE_PORT) {
                all_errs.push(invalid(
                    &path.child("ports").index(i).child("nodePort"),
                    BadValue::Int(node_port.into()),
                    &format!("must be between {} and {}", MIN_NODE_PORT, MAX_NODE_PORT),
                ));
            }
        }
    }

    // Check for duplicate (protocol, port) pairs
    let mut ports_seen = HashSet::new();
    for (i, port) in spec.ports.iter().enumerate() {
        let protocol_value = protocol_to_str(&port.protocol);
        let key = (protocol_value, port.port);
        if ports_seen.contains(&key) {
            all_errs.push(Error {
                error_type: ErrorType::Duplicate,
                field: path.child("ports").index(i).to_string(),
                bad_value: Some(BadValue::String(format!(
                    "port {} with protocol {}",
                    port.port, protocol_value
                ))),
                detail: format!(
                    "port {} with protocol {} already exists",
                    port.port, protocol_value
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
                let protocol_value = protocol_to_str(&port.protocol);
                let key = (protocol_value, node_port);
                if node_ports_seen.contains(&key) {
                    all_errs.push(Error {
                        error_type: ErrorType::Duplicate,
                        field: path.child("ports").index(i).child("nodePort").to_string(),
                        bad_value: Some(BadValue::Int(node_port.into())),
                        detail: format!(
                            "nodePort {} with protocol {} already exists",
                            node_port, protocol_value
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
    if !spec.load_balancer_source_ranges.is_empty() && service_type != service_type::LOAD_BALANCER {
        all_errs.push(forbidden(
            &path.child("loadBalancerSourceRanges"),
            "may only be used when `type` is 'LoadBalancer'",
        ));
    }

    // AllocateLoadBalancerNodePorts validation
    if spec.allocate_load_balancer_node_ports.is_some()
        && service_type != service_type::LOAD_BALANCER
    {
        all_errs.push(forbidden(
            &path.child("allocateLoadBalancerNodePorts"),
            "may only be used when `type` is 'LoadBalancer'",
        ));
    }

    // LoadBalancer type requires allocateLoadBalancerNodePorts to be set
    if service_type == service_type::LOAD_BALANCER
        && spec.allocate_load_balancer_node_ports.is_none()
    {
        all_errs.push(required(
            &path.child("allocateLoadBalancerNodePorts"),
            "must be set for LoadBalancer services",
        ));
    }

    // Validate external IPs
    for (i, ip) in spec.external_ips.iter().enumerate() {
        if ip.trim().is_empty() {
            all_errs.push(invalid(
                &path.child("externalIPs").index(i),
                BadValue::String(ip.clone()),
                "must not be empty",
            ));
        }
        if !ip.trim().is_empty() && !is_valid_ip(ip) {
            all_errs.push(invalid(
                &path.child("externalIPs").index(i),
                BadValue::String(ip.clone()),
                "must be a valid IP",
            ));
        }
    }

    // Validate load balancer source ranges
    for (i, cidr) in spec.load_balancer_source_ranges.iter().enumerate() {
        let cidr_trimmed = cidr.trim();
        if cidr_trimmed.is_empty() {
            all_errs.push(invalid(
                &path.child("loadBalancerSourceRanges").index(i),
                BadValue::String(cidr.clone()),
                "must not be empty",
            ));
        }
        if !cidr_trimmed.is_empty() && !is_valid_cidr(cidr_trimmed) {
            all_errs.push(invalid(
                &path.child("loadBalancerSourceRanges").index(i),
                BadValue::String(cidr.clone()),
                "must be a valid CIDR",
            ));
        }
    }

    // ExternalTrafficPolicy only valid for NodePort/LoadBalancer
    if spec.external_traffic_policy.is_some()
        && service_type != service_type::NODE_PORT
        && service_type != service_type::LOAD_BALANCER
    {
        all_errs.push(forbidden(
            &path.child("externalTrafficPolicy"),
            "may only be used when `type` is 'NodePort' or 'LoadBalancer'",
        ));
    }

    // InternalTrafficPolicy not valid for ExternalName
    if spec.internal_traffic_policy.is_some() && service_type == service_type::EXTERNAL_NAME {
        all_errs.push(forbidden(
            &path.child("internalTrafficPolicy"),
            "may not be set for ExternalName services",
        ));
    }

    // HealthCheckNodePort validation
    if let Some(node_port) = spec.health_check_node_port {
        if service_type != service_type::LOAD_BALANCER {
            all_errs.push(forbidden(
                &path.child("healthCheckNodePort"),
                "may only be used when `type` is 'LoadBalancer'",
            ));
        } else if node_port < MIN_NODE_PORT || node_port > MAX_NODE_PORT {
            all_errs.push(invalid(
                &path.child("healthCheckNodePort"),
                BadValue::Int(node_port.into()),
                &format!("must be between {} and {}", MIN_NODE_PORT, MAX_NODE_PORT),
            ));
        }
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
    let protocol_value = protocol_to_str(&port.protocol);
    if !SUPPORTED_PROTOCOLS.contains(protocol_value) {
        let valid: Vec<&str> = SUPPORTED_PROTOCOLS.iter().copied().collect();
        all_errs.push(not_supported(
            &path.child("protocol"),
            BadValue::String(protocol_value.to_string()),
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
    if let Some(app_protocol) = &port.app_protocol {
        if !app_protocol.is_empty() {
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
    all_errs.extend(crate::common::validation::validate_object_meta_update(
        &new_service.metadata,
        &old_service.metadata,
        &path.child("metadata"),
    ));

    // Validate the new service spec
    if let Some(ref spec) = new_service.spec {
        all_errs.extend(validate_service_spec(spec, &path.child("spec")));
    }

    // Immutability checks for ClusterIPs/IPFamilies
    if let (Some(new_spec), Some(old_spec)) = (&new_service.spec, &old_service.spec) {
        if !old_spec.cluster_ip.is_empty()
            && old_spec.cluster_ip != CLUSTER_IP_NONE
            && new_spec.cluster_ip != old_spec.cluster_ip
        {
            all_errs.push(forbidden(
                &path.child("spec").child("clusterIP"),
                "field is immutable",
            ));
        }

        if !old_spec.cluster_ips.is_empty() {
            if new_spec.cluster_ips.is_empty() || old_spec.cluster_ips[0] != new_spec.cluster_ips[0]
            {
                all_errs.push(forbidden(
                    &path.child("spec").child("clusterIPs"),
                    "may not change primary ClusterIP",
                ));
            }
        }

        if !old_spec.ip_families.is_empty() {
            if new_spec.ip_families.is_empty() || old_spec.ip_families[0] != new_spec.ip_families[0]
            {
                all_errs.push(forbidden(
                    &path.child("spec").child("ipFamilies"),
                    "may not change primary IP family",
                ));
            }
        }
    }

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
    all_errs.extend(crate::common::validation::validate_object_meta_update(
        &new_service.metadata,
        &old_service.metadata,
        &path.child("metadata"),
    ));

    // TODO: Add LoadBalancer status validation

    let new_status = &new_service.status;
    if let Some(new_lb) = &new_status.load_balancer {
        for (i, ingress) in new_lb.ingress.iter().enumerate() {
            if !ingress.ip.is_empty() && !is_valid_ip(&ingress.ip) {
                all_errs.push(invalid(
                    &path
                        .child("status")
                        .child("loadBalancer")
                        .child("ingress")
                        .index(i),
                    BadValue::String(ingress.ip.clone()),
                    "ingress IP must be a valid IP",
                ));
            }
            if !ingress.hostname.is_empty() {
                let errors = crate::common::validation::is_dns1123_subdomain(&ingress.hostname);
                for err in errors {
                    all_errs.push(invalid(
                        &path
                            .child("status")
                            .child("loadBalancer")
                            .child("ingress")
                            .index(i)
                            .child("hostname"),
                        BadValue::String(ingress.hostname.clone()),
                        &err,
                    ));
                }
            }
            if let Some(ip_mode) = &ingress.ip_mode {
                if !ip_mode.is_empty() {
                    if ingress.ip.is_empty() {
                        all_errs.push(invalid(
                            &path
                                .child("status")
                                .child("loadBalancer")
                                .child("ingress")
                                .index(i)
                                .child("ipMode"),
                            BadValue::String(ip_mode.clone()),
                            "may only be set when ip is specified",
                        ));
                    } else if ip_mode != load_balancer_ip_mode::VIP
                        && ip_mode != load_balancer_ip_mode::PROXY
                    {
                        all_errs.push(not_supported(
                            &path
                                .child("status")
                                .child("loadBalancer")
                                .child("ingress")
                                .index(i)
                                .child("ipMode"),
                            BadValue::String(ip_mode.clone()),
                            &[load_balancer_ip_mode::VIP, load_balancer_ip_mode::PROXY],
                        ));
                    }
                }
            }
            for (j, port_status) in ingress.ports.iter().enumerate() {
                let protocol_value = protocol_to_str(&port_status.protocol);
                if !SUPPORTED_PROTOCOLS.contains(protocol_value) {
                    let valid: Vec<&str> = SUPPORTED_PROTOCOLS.iter().copied().collect();
                    all_errs.push(not_supported(
                        &path
                            .child("status")
                            .child("loadBalancer")
                            .child("ingress")
                            .index(i)
                            .child("ports")
                            .index(j)
                            .child("protocol"),
                        BadValue::String(protocol_value.to_string()),
                        &valid,
                    ));
                }
            }
        }
    }

    // Ensure only status is being updated (spec should not change)
    if new_service.spec != old_service.spec {
        all_errs.push(forbidden(
            &path.child("spec"),
            "spec updates are not allowed via status subresource",
        ));
    }

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
fn get_service_type(spec: &ServiceSpec) -> &str {
    match spec.r#type.as_ref() {
        None => service_type::CLUSTER_IP,
        Some(value) => service_type_to_str(value),
    }
}

fn service_type_to_str(value: &ServiceType) -> &'static str {
    match value {
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

fn is_valid_ip(value: &str) -> bool {
    value.parse::<IpAddr>().is_ok()
}

fn ip_family_of_ip(value: &str) -> Option<IPFamily> {
    let parsed = value.parse::<IpAddr>().ok()?;
    match parsed {
        IpAddr::V4(_) => Some(IPFamily::Ipv4),
        IpAddr::V6(_) => Some(IPFamily::Ipv6),
    }
}

fn ip_family_to_str(value: &IPFamily) -> &'static str {
    match value {
        IPFamily::Ipv4 => "IPv4",
        IPFamily::Ipv6 => "IPv6",
    }
}

fn protocol_to_str(value: &Option<Protocol>) -> &'static str {
    match value {
        Some(Protocol::Tcp) => protocol::TCP,
        Some(Protocol::Udp) => protocol::UDP,
        Some(Protocol::Sctp) => protocol::SCTP,
        None => "",
    }
}

fn is_valid_cidr(value: &str) -> bool {
    let parts: Vec<&str> = value.split('/').collect();
    if parts.len() != 2 {
        return false;
    }
    let addr = parts[0];
    let prefix = match parts[1].parse::<u8>() {
        Ok(value) => value,
        Err(_) => return false,
    };
    let ip = match addr.parse::<IpAddr>() {
        Ok(value) => value,
        Err(_) => return false,
    };
    match ip {
        IpAddr::V4(_) => prefix <= 32,
        IpAddr::V6(_) => prefix <= 128,
    }
}
