//! Shared helpers for networking validation.

use std::net::IpAddr;

use crate::common::IntOrString;
use crate::common::meta::label_selector_operator;
use crate::common::validation::*;
use crate::networking::v1::ingress::IngressBackend;
use crate::networking::v1::network_policy::{IPBlock, NetworkPolicyPort};

pub(crate) const MAX_INGRESS_CLASS_CONTROLLER_LEN: usize = 250;
pub(crate) const INVALID_PATH_SEQUENCES: [&str; 5] = ["//", "/./", "/../", "%2f", "%2F"];
pub(crate) const INVALID_PATH_SUFFIXES: [&str; 2] = ["/..", "/."];
pub(crate) const SUPPORTED_POLICY_TYPES: [&str; 2] = ["Ingress", "Egress"];
pub(crate) const SUPPORTED_PROTOCOLS: [&str; 3] = ["TCP", "UDP", "SCTP"];
pub(crate) const INGRESS_CLASS_SCOPE_NAMESPACE: &str = "Namespace";
pub(crate) const INGRESS_CLASS_SCOPE_CLUSTER: &str = "Cluster";

// ============================================================================
// ============================================================================

// ============================================================================
// Helper Functions
// ============================================================================

pub(crate) fn validate_dns1123_subdomain(value: &str, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    for msg in is_dns1123_subdomain(value) {
        all_errs.push(invalid(path, BadValue::String(value.to_string()), &msg));
    }
    all_errs
}

pub(crate) fn validate_wildcard_dns1123_subdomain(value: &str, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let wildcard_count = value.matches('*').count();
    if wildcard_count != 1 || !value.starts_with("*.") {
        all_errs.push(invalid(
            path,
            BadValue::String(value.to_string()),
            "wildcard must be the leftmost label like \"*.example.com\"",
        ));
        return all_errs;
    }

    let suffix = &value[2..];
    if suffix.is_empty() {
        all_errs.push(required(path, "wildcard host must include a DNS suffix"));
        return all_errs;
    }

    all_errs.extend(validate_dns1123_subdomain(suffix, path));
    all_errs
}

pub(crate) fn is_ip_address(value: &str) -> bool {
    value.parse::<IpAddr>().is_ok()
}

pub(crate) fn validate_path_segment_name(value: &str, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if value == "." || value == ".." {
        all_errs.push(invalid(
            path,
            BadValue::String(value.to_string()),
            &format!("may not be '{}'", value),
        ));
        return all_errs;
    }

    for illegal in ["/", "%"] {
        if value.contains(illegal) {
            all_errs.push(invalid(
                path,
                BadValue::String(value.to_string()),
                &format!("may not contain '{}'", illegal),
            ));
        }
    }

    all_errs
}

pub(crate) fn validate_ip_address_name(name: &str, _prefix: bool) -> Vec<String> {
    let mut errs = Vec::new();
    let ip: IpAddr = match name.parse() {
        Ok(ip) => ip,
        Err(_) => {
            errs.push("must be a valid IP address, (e.g. 10.9.8.7 or 2001:db8::ffff)".to_string());
            return errs;
        }
    };

    if let IpAddr::V6(addr) = ip {
        // Check for IPv4-mapped IPv6 address (::ffff:a.b.c.d)
        if is_ipv4_mapped_ipv6(&addr) {
            errs.push("must not be an IPv4-mapped IPv6 address".to_string());
        }
    }

    let canonical = ip.to_string();
    if name != canonical {
        errs.push(format!("must be in canonical form (\"{}\")", canonical));
    }

    errs
}

pub(crate) fn is_ipv4_mapped_ipv6(addr: &std::net::Ipv6Addr) -> bool {
    matches!(addr.segments(), [0, 0, 0, 0, 0, 0xffff, _, _])
}

pub(crate) fn validate_port_number(port: i32, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if !(1..=65535).contains(&port) {
        all_errs.push(invalid(
            path,
            BadValue::Int(port as i64),
            "must be in the range 1-65535",
        ));
    }
    all_errs
}

pub(crate) fn validate_port_name(port: &str, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if port.is_empty() {
        return all_errs;
    }
    let is_valid = port.len() <= 15
        && port
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        && port.chars().any(|c| c.is_ascii_lowercase())
        && !port.starts_with('-')
        && !port.ends_with('-')
        && !port.contains("--");
    if !is_valid {
        all_errs.push(invalid(
            path,
            BadValue::String(port.to_string()),
            "must be a valid port name (lowercase alphanumeric or '-', 1-15 chars, no consecutive hyphens)",
        ));
    }
    all_errs
}

pub(crate) fn validate_label_selector(
    selector: &crate::common::LabelSelector,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    all_errs.extend(validate_labels(
        &selector.match_labels,
        &path.child("matchLabels"),
    ));

    for (i, requirement) in selector.match_expressions.iter().enumerate() {
        let req_path = path.child("matchExpressions").index(i);
        if requirement.key.is_empty() {
            all_errs.push(required(&req_path.child("key"), "key is required"));
        } else {
            all_errs.extend(validate_qualified_name(
                &requirement.key,
                &req_path.child("key"),
            ));
        }

        let operator = requirement.operator.as_str();
        match operator {
            label_selector_operator::IN | label_selector_operator::NOT_IN => {
                if requirement.values.is_empty() {
                    all_errs.push(required(
                        &req_path.child("values"),
                        "values must be non-empty for In/NotIn operators",
                    ));
                }
            }
            label_selector_operator::EXISTS | label_selector_operator::DOES_NOT_EXIST => {
                if !requirement.values.is_empty() {
                    all_errs.push(invalid(
                        &req_path.child("values"),
                        BadValue::String(format!("{:?}", requirement.values)),
                        "values must be empty for Exists/DoesNotExist operators",
                    ));
                }
            }
            _ => {
                all_errs.push(not_supported(
                    &req_path.child("operator"),
                    BadValue::String(requirement.operator.clone()),
                    &[
                        label_selector_operator::IN,
                        label_selector_operator::NOT_IN,
                        label_selector_operator::EXISTS,
                        label_selector_operator::DOES_NOT_EXIST,
                    ],
                ));
            }
        }

        for (j, value) in requirement.values.iter().enumerate() {
            for msg in is_valid_label_value(value) {
                all_errs.push(invalid(
                    &req_path.child("values").index(j),
                    BadValue::String(value.to_string()),
                    &msg,
                ));
            }
        }
    }

    all_errs
}

/// Validates an IngressBackend
pub(crate) fn validate_ingress_backend(backend: &IngressBackend, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // service and resource are mutually exclusive
    if backend.service.is_some() && backend.resource.is_some() {
        all_errs.push(invalid(
            path,
            BadValue::String("both service and resource specified".to_string()),
            "cannot set both service and resource backends",
        ));
    }

    // At least one must be specified
    if backend.service.is_none() && backend.resource.is_none() {
        all_errs.push(required(path, "service or resource backend is required"));
    }

    // Validate service if present
    if let Some(ref service) = backend.service {
        if service.name.is_empty() {
            all_errs.push(required(
                &path.child("service").child("name"),
                "service name is required",
            ));
        } else {
            for msg in is_dns1035_label(&service.name) {
                all_errs.push(invalid(
                    &path.child("service").child("name"),
                    BadValue::String(service.name.clone()),
                    &msg,
                ));
            }
        }

        // Validate port if present
        if let Some(ref port) = service.port {
            let has_name = !port.name.is_empty();
            let has_number = port.number.is_some();
            if has_name && has_number {
                all_errs.push(invalid(
                    &path.child("service").child("port"),
                    BadValue::String("both name and number specified".to_string()),
                    "cannot set both port name and port number",
                ));
            } else if has_name {
                all_errs.extend(validate_port_name(
                    &port.name,
                    &path.child("service").child("port").child("name"),
                ));
            } else if let Some(number) = port.number {
                all_errs.extend(validate_port_number(
                    number,
                    &path.child("service").child("port").child("number"),
                ));
            } else {
                all_errs.push(required(
                    &path.child("service").child("port"),
                    "port name or number is required",
                ));
            }
        } else {
            all_errs.push(required(
                &path.child("service").child("port"),
                "port name or number is required",
            ));
        }
    }

    // Validate resource if present
    if let Some(ref resource) = backend.resource {
        if resource.kind.as_ref().map_or(true, |k| k.is_empty()) {
            all_errs.push(required(
                &path.child("resource").child("kind"),
                "resource kind is required",
            ));
        }
        if resource.name.as_ref().map_or(true, |n| n.is_empty()) {
            all_errs.push(required(
                &path.child("resource").child("name"),
                "resource name is required",
            ));
        }
        if let Some(api_group) = resource.api_group.as_ref()
            && !api_group.is_empty()
        {
            all_errs.extend(validate_dns1123_subdomain(
                api_group,
                &path.child("resource").child("apiGroup"),
            ));
        }
    }

    all_errs
}

/// Validates a NetworkPolicyPort
pub(crate) fn validate_network_policy_port(port: &NetworkPolicyPort, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if let Some(protocol) = port.protocol.as_deref()
        && !SUPPORTED_PROTOCOLS.contains(&protocol)
    {
        all_errs.push(not_supported(
            &path.child("protocol"),
            BadValue::String(protocol.to_string()),
            &SUPPORTED_PROTOCOLS,
        ));
    }

    if let Some(IntOrString::Int(port_num)) = port.port {
        all_errs.extend(validate_port_number(port_num, &path.child("port")));
    }

    if let Some(IntOrString::String(ref port_name)) = port.port {
        if port_name.is_empty() {
            all_errs.push(required(&path.child("port"), "port name is required"));
        } else {
            all_errs.extend(validate_port_name(port_name, &path.child("port")));
        }
    }

    // Validate endPort requirements
    if let Some(end_port) = port.end_port {
        if let Some(ref port_value) = port.port {
            // endPort only valid with numeric ports
            match port_value {
                IntOrString::String(_) => {
                    all_errs.push(invalid(
                        &path.child("endPort"),
                        BadValue::Int(end_port as i64),
                        "endPort may not be specified when port is non-numeric",
                    ));
                }
                IntOrString::Int(port_num) => {
                    if end_port < *port_num {
                        all_errs.push(invalid(
                            &path.child("endPort"),
                            BadValue::Int(end_port as i64),
                            "endPort must be greater than or equal to port",
                        ));
                    }
                    all_errs.extend(validate_port_number(end_port, &path.child("endPort")));
                }
            }
        } else {
            all_errs.push(invalid(
                &path.child("endPort"),
                BadValue::Int(end_port as i64),
                "endPort may not be specified when port is not specified",
            ));
        }
    }

    all_errs
}

pub(crate) fn validate_network_policy_peer(
    peer: &crate::networking::v1::network_policy::NetworkPolicyPeer,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let mut peer_fields = 0;

    if let Some(ref selector) = peer.pod_selector {
        peer_fields += 1;
        all_errs.extend(validate_label_selector(
            selector,
            &path.child("podSelector"),
        ));
    }
    if let Some(ref selector) = peer.namespace_selector {
        peer_fields += 1;
        all_errs.extend(validate_label_selector(
            selector,
            &path.child("namespaceSelector"),
        ));
    }
    if let Some(ref ip_block) = peer.ip_block {
        peer_fields += 1;
        all_errs.extend(validate_ip_block(ip_block, &path.child("ipBlock")));
    }

    if peer_fields == 0 {
        all_errs.push(required(path, "must specify a peer"));
    } else if peer_fields > 1 && peer.ip_block.is_some() {
        all_errs.push(forbidden(
            path,
            "may not specify both ipBlock and another peer",
        ));
    }

    all_errs
}

pub(crate) fn validate_ingress_class_parameters(
    params: &crate::networking::v1::ingress_class::IngressClassParametersReference,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if params.kind.is_empty() {
        all_errs.push(required(&path.child("kind"), "kind is required"));
    }
    if params.name.is_empty() {
        all_errs.push(required(&path.child("name"), "name is required"));
    } else {
        all_errs.extend(validate_dns1123_subdomain(
            &params.name,
            &path.child("name"),
        ));
    }

    if let Some(api_group) = params.api_group.as_ref()
        && !api_group.is_empty()
    {
        all_errs.extend(validate_dns1123_subdomain(
            api_group,
            &path.child("apiGroup"),
        ));
    }

    let scope = match params.scope.as_ref() {
        Some(scope) if !scope.is_empty() => scope.as_str(),
        _ => {
            all_errs.push(required(&path.child("scope"), "scope is required"));
            return all_errs;
        }
    };

    match scope {
        INGRESS_CLASS_SCOPE_NAMESPACE => {
            if let Some(namespace) = params.namespace.as_ref() {
                for msg in is_dns1123_label(namespace) {
                    all_errs.push(invalid(
                        &path.child("namespace"),
                        BadValue::String(namespace.clone()),
                        &msg,
                    ));
                }
            } else {
                all_errs.push(required(
                    &path.child("namespace"),
                    "`parameters.scope` is set to 'Namespace'",
                ));
            }
        }
        INGRESS_CLASS_SCOPE_CLUSTER => {
            if params.namespace.is_some() {
                all_errs.push(forbidden(
                    &path.child("namespace"),
                    "`parameters.scope` is set to 'Cluster'",
                ));
            }
        }
        _ => {
            all_errs.push(not_supported(
                &path.child("scope"),
                BadValue::String(scope.to_string()),
                &[INGRESS_CLASS_SCOPE_NAMESPACE, INGRESS_CLASS_SCOPE_CLUSTER],
            ));
        }
    }

    all_errs
}

/// Validates an IPBlock CIDR
pub(crate) fn validate_ip_block(block: &IPBlock, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate CIDR format
    if block.cidr.is_empty() {
        all_errs.push(required(&path.child("cidr"), "CIDR is required"));
    } else if parse_cidr(&block.cidr).is_none() {
        all_errs.push(invalid(
            &path.child("cidr"),
            BadValue::String(block.cidr.clone()),
            "invalid CIDR format",
        ));
    }

    // Validate except CIDRs
    for (i, except_cidr) in block.except.iter().enumerate() {
        let except_path = &path.child("except").index(i);
        let except = match parse_cidr(except_cidr) {
            Some(value) => value,
            None => {
                all_errs.push(invalid(
                    except_path,
                    BadValue::String(except_cidr.clone()),
                    "invalid CIDR format",
                ));
                continue;
            }
        };
        let parent = match parse_cidr(&block.cidr) {
            Some(value) => value,
            None => continue,
        };

        if !cidr_contains(parent, except) || except.1 <= parent.1 {
            all_errs.push(invalid(
                except_path,
                BadValue::String(except_cidr.clone()),
                "must be a strict subset of cidr",
            ));
        }
    }

    all_errs
}

pub(crate) fn parse_cidr(cidr: &str) -> Option<(IpAddr, u8)> {
    let (ip_str, prefix_str) = cidr.split_once('/')?;
    let ip: IpAddr = ip_str.parse().ok()?;
    let prefix: u8 = prefix_str.parse().ok()?;
    let max = match ip {
        IpAddr::V4(_) => 32,
        IpAddr::V6(_) => 128,
    };
    if prefix > max {
        return None;
    }
    Some((ip, prefix))
}

pub(crate) fn cidr_contains(parent: (IpAddr, u8), child: (IpAddr, u8)) -> bool {
    match (parent.0, child.0) {
        (IpAddr::V4(parent_ip), IpAddr::V4(child_ip)) => {
            let parent_prefix = parent.1;
            let child_prefix = child.1;
            if child_prefix < parent_prefix {
                return false;
            }
            let parent_bits = u32::from(parent_ip);
            let child_bits = u32::from(child_ip);
            let mask = if parent_prefix == 0 {
                0
            } else {
                u32::MAX << (32 - parent_prefix)
            };
            (parent_bits & mask) == (child_bits & mask)
        }
        (IpAddr::V6(parent_ip), IpAddr::V6(child_ip)) => {
            let parent_prefix = parent.1;
            let child_prefix = child.1;
            if child_prefix < parent_prefix {
                return false;
            }
            let parent_bits = u128::from(parent_ip);
            let child_bits = u128::from(child_ip);
            let mask = if parent_prefix == 0 {
                0
            } else {
                u128::MAX << (128 - parent_prefix)
            };
            (parent_bits & mask) == (child_bits & mask)
        }
        _ => false,
    }
}

pub(crate) fn validate_cidr_strict(cidr: &str, path: &Path) -> (ErrorList, Option<(IpAddr, u8)>) {
    let mut all_errs = ErrorList::new();
    let (ip_str, prefix_str) = match cidr.split_once('/') {
        Some(parts) => parts,
        None => {
            all_errs.push(invalid(
                path,
                BadValue::String(cidr.to_string()),
                "must be a valid CIDR value, (e.g. 10.9.8.0/24 or 2001:db8::/64)",
            ));
            return (all_errs, None);
        }
    };

    let ip: IpAddr = match ip_str.parse() {
        Ok(ip) => ip,
        Err(_) => {
            all_errs.push(invalid(
                path,
                BadValue::String(cidr.to_string()),
                "must be a valid CIDR value, (e.g. 10.9.8.0/24 or 2001:db8::/64)",
            ));
            return (all_errs, None);
        }
    };

    let prefix: u8 = match prefix_str.parse() {
        Ok(prefix) => prefix,
        Err(_) => {
            all_errs.push(invalid(
                path,
                BadValue::String(cidr.to_string()),
                "must be a valid CIDR value, (e.g. 10.9.8.0/24 or 2001:db8::/64)",
            ));
            return (all_errs, None);
        }
    };

    let max = match ip {
        IpAddr::V4(_) => 32,
        IpAddr::V6(_) => 128,
    };
    if prefix > max {
        all_errs.push(invalid(
            path,
            BadValue::String(cidr.to_string()),
            "must be a valid CIDR value, (e.g. 10.9.8.0/24 or 2001:db8::/64)",
        ));
        return (all_errs, None);
    }

    if let IpAddr::V6(addr) = ip {
        // Check for IPv4-mapped IPv6 address (::ffff:a.b.c.d)
        if is_ipv4_mapped_ipv6(&addr) {
            all_errs.push(invalid(
                path,
                BadValue::String(cidr.to_string()),
                "must not have an IPv4-mapped IPv6 address",
            ));
        }
    }

    let network_ip = match ip {
        IpAddr::V4(addr) => {
            let value = u32::from(addr);
            let mask = if prefix == 0 {
                0
            } else {
                u32::MAX << (32 - prefix)
            };
            IpAddr::V4((value & mask).into())
        }
        IpAddr::V6(addr) => {
            let value = u128::from(addr);
            let mask = if prefix == 0 {
                0
            } else {
                u128::MAX << (128 - prefix)
            };
            IpAddr::V6((value & mask).into())
        }
    };

    if ip != network_ip {
        all_errs.push(invalid(
            path,
            BadValue::String(cidr.to_string()),
            "must not have bits set beyond the prefix length",
        ));
    }

    let canonical = format!("{}/{}", network_ip, prefix);
    if cidr != canonical {
        all_errs.push(invalid(
            path,
            BadValue::String(cidr.to_string()),
            &format!("must be in canonical form (\"{}\")", canonical),
        ));
    }

    (all_errs, Some((ip, prefix)))
}

// ============================================================================
