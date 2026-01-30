//! Validation for Kubernetes Networking API v1 types
//!
//! Ported from k8s.io/kubernetes/pkg/apis/networking/validation/validation.go

use crate::common::IntOrString;
use crate::common::meta::label_selector_operator;
use crate::common::validation::{
    BadValue, ErrorList, Path, forbidden, invalid, is_dns1035_label, is_dns1123_label,
    is_dns1123_subdomain, is_valid_label_value, name_is_dns_subdomain, not_supported, required,
    too_long, validate_labels, validate_object_meta, validate_qualified_name,
};
use crate::networking::v1::ingress::{Ingress, IngressBackend, IngressList};
use crate::networking::v1::ingress_class::{IngressClass, IngressClassList};
use crate::networking::v1::network_policy::{
    IPBlock, NetworkPolicy, NetworkPolicyList, NetworkPolicyPort,
};
use std::net::IpAddr;

const MAX_INGRESS_CLASS_CONTROLLER_LEN: usize = 250;
const INVALID_PATH_SEQUENCES: [&str; 5] = ["//", "/./", "/../", "%2f", "%2F"];
const INVALID_PATH_SUFFIXES: [&str; 2] = ["/..", "/."];
const SUPPORTED_POLICY_TYPES: [&str; 2] = ["Ingress", "Egress"];
const SUPPORTED_PROTOCOLS: [&str; 3] = ["TCP", "UDP", "SCTP"];
const INGRESS_CLASS_SCOPE_NAMESPACE: &str = "Namespace";
const INGRESS_CLASS_SCOPE_CLUSTER: &str = "Cluster";

// ============================================================================
// Ingress Validation
// ============================================================================

// ============================================================================
// Helper Functions
// ============================================================================

fn validate_dns1123_subdomain(value: &str, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    for msg in is_dns1123_subdomain(value) {
        all_errs.push(invalid(path, BadValue::String(value.to_string()), &msg));
    }
    all_errs
}

fn validate_wildcard_dns1123_subdomain(value: &str, path: &Path) -> ErrorList {
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

fn is_ip_address(value: &str) -> bool {
    value.parse::<IpAddr>().is_ok()
}

fn validate_port_number(port: i32, path: &Path) -> ErrorList {
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

fn validate_port_name(port: &str, path: &Path) -> ErrorList {
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

fn validate_label_selector(selector: &crate::common::LabelSelector, path: &Path) -> ErrorList {
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
fn validate_ingress_backend(backend: &IngressBackend, path: &Path) -> ErrorList {
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
fn validate_network_policy_port(port: &NetworkPolicyPort, path: &Path) -> ErrorList {
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

fn validate_network_policy_peer(
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

fn validate_ingress_class_parameters(
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
fn validate_ip_block(block: &IPBlock, path: &Path) -> ErrorList {
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

fn parse_cidr(cidr: &str) -> Option<(IpAddr, u8)> {
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

fn cidr_contains(parent: (IpAddr, u8), child: (IpAddr, u8)) -> bool {
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

// ============================================================================
// Ingress Validation
// ============================================================================

/// Validates an Ingress object.
pub fn validate_ingress(ingress: &Ingress) -> ErrorList {
    validate_ingress_with_path(ingress, &Path::nil())
}

fn validate_ingress_with_path(ingress: &Ingress, base_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let default_meta = crate::common::ObjectMeta::default();
    let meta = ingress.metadata.as_ref().unwrap_or(&default_meta);

    all_errs.extend(validate_object_meta(
        meta,
        true,
        name_is_dns_subdomain,
        &base_path.child("metadata"),
    ));

    // Validate spec if present
    if let Some(ref spec) = ingress.spec {
        if spec.rules.is_empty() && spec.default_backend.is_none() {
            all_errs.push(invalid(
                &base_path.child("spec"),
                BadValue::String("missing rules and defaultBackend".to_string()),
                "either defaultBackend or rules must be specified",
            ));
        }

        if let Some(ref class_name) = spec.ingress_class_name {
            all_errs.extend(validate_dns1123_subdomain(
                class_name,
                &base_path.child("spec").child("ingressClassName"),
            ));
        }

        // Validate default backend
        if let Some(ref default_backend) = spec.default_backend {
            all_errs.extend(validate_ingress_backend(
                default_backend,
                &base_path.child("spec").child("defaultBackend"),
            ));
        }

        // Validate rules
        for (i, rule) in spec.rules.iter().enumerate() {
            let rule_path = base_path.child("spec").child("rules").index(i);

            // Validate host if present (should be valid DNS subdomain)
            if !rule.host.is_empty() {
                if is_ip_address(&rule.host) {
                    all_errs.push(invalid(
                        &rule_path.child("host"),
                        BadValue::String(rule.host.clone()),
                        "must be a DNS name, not an IP address",
                    ));
                } else if rule.host.contains('*') {
                    all_errs.extend(validate_wildcard_dns1123_subdomain(
                        &rule.host,
                        &rule_path.child("host"),
                    ));
                } else {
                    all_errs.extend(validate_dns1123_subdomain(
                        &rule.host,
                        &rule_path.child("host"),
                    ));
                }
            }

            // Validate HTTP paths
            if let Some(ref http) = rule.http {
                if http.paths.is_empty() {
                    all_errs.push(required(
                        &rule_path.child("http").child("paths"),
                        "paths is required",
                    ));
                }
                for (j, path) in http.paths.iter().enumerate() {
                    let path_path = rule_path.child("http").child("paths").index(j);

                    // Validate path field
                    if path.path.is_empty() {
                        all_errs.push(required(&path_path.child("path"), "path is required"));
                    } else {
                        // Path must start with '/' for Prefix and Exact types
                        use crate::networking::v1::ingress::PathType;
                        match path.path_type {
                            PathType::Prefix | PathType::Exact => {
                                if !path.path.starts_with('/') {
                                    all_errs.push(invalid(
                                        &path_path.child("path"),
                                        BadValue::String(path.path.clone()),
                                        "must be an absolute path",
                                    ));
                                }
                                for sequence in INVALID_PATH_SEQUENCES {
                                    if path.path.contains(sequence) {
                                        all_errs.push(invalid(
                                            &path_path.child("path"),
                                            BadValue::String(path.path.clone()),
                                            &format!("must not contain '{}'", sequence),
                                        ));
                                    }
                                }
                                for suffix in INVALID_PATH_SUFFIXES {
                                    if path.path.ends_with(suffix) {
                                        all_errs.push(invalid(
                                            &path_path.child("path"),
                                            BadValue::String(path.path.clone()),
                                            &format!("cannot end with '{}'", suffix),
                                        ));
                                    }
                                }
                            }
                            PathType::ImplementationSpecific => {
                                if !path.path.is_empty() && !path.path.starts_with('/') {
                                    all_errs.push(invalid(
                                        &path_path.child("path"),
                                        BadValue::String(path.path.clone()),
                                        "must be an absolute path",
                                    ));
                                }
                            }
                        }
                    }

                    // Validate backend
                    all_errs.extend(validate_ingress_backend(
                        &path.backend,
                        &path_path.child("backend"),
                    ));
                }
            }
        }

        // Validate TLS hosts and secret names if present
        for (i, tls) in spec.tls.iter().enumerate() {
            for (j, host) in tls.hosts.iter().enumerate() {
                if host.contains('*') {
                    all_errs.extend(validate_wildcard_dns1123_subdomain(
                        host,
                        &base_path
                            .child("spec")
                            .child("tls")
                            .index(i)
                            .child("hosts")
                            .index(j),
                    ));
                } else {
                    all_errs.extend(validate_dns1123_subdomain(
                        host,
                        &base_path
                            .child("spec")
                            .child("tls")
                            .index(i)
                            .child("hosts")
                            .index(j),
                    ));
                }
            }

            if let Some(secret_name) = tls.secret_name.as_ref()
                && !secret_name.is_empty()
            {
                all_errs.extend(validate_dns1123_subdomain(
                    secret_name,
                    &base_path
                        .child("spec")
                        .child("tls")
                        .index(i)
                        .child("secretName"),
                ));
            }
        }
    }

    all_errs
}

/// Validates an IngressList object.
pub fn validate_ingress_list(list: &IngressList) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (i, item) in list.items.iter().enumerate() {
        let item_path = Path::new("items").index(i);
        all_errs.extend(validate_ingress_with_path(item, &item_path));
    }

    all_errs
}

// ============================================================================
// IngressClass Validation
// ============================================================================

/// Validates an IngressClass object.
pub fn validate_ingress_class(ingress_class: &IngressClass) -> ErrorList {
    validate_ingress_class_with_path(ingress_class, &Path::nil())
}

fn validate_ingress_class_with_path(ingress_class: &IngressClass, base_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let default_meta = crate::common::ObjectMeta::default();
    let meta = ingress_class.metadata.as_ref().unwrap_or(&default_meta);

    all_errs.extend(validate_object_meta(
        meta,
        false, // IngressClass is not namespaced
        name_is_dns_subdomain,
        &base_path.child("metadata"),
    ));

    // Validate spec - controller is required
    if ingress_class.spec.controller.is_empty() {
        all_errs.push(required(
            &base_path.child("spec").child("controller"),
            "controller is required",
        ));
    } else if ingress_class.spec.controller.len() > MAX_INGRESS_CLASS_CONTROLLER_LEN {
        all_errs.push(too_long(
            &base_path.child("spec").child("controller"),
            MAX_INGRESS_CLASS_CONTROLLER_LEN,
        ));
    }

    if let Some(ref params) = ingress_class.spec.parameters {
        all_errs.extend(validate_ingress_class_parameters(
            params,
            &base_path.child("spec").child("parameters"),
        ));
    }

    all_errs
}

/// Validates an IngressClassList object.
pub fn validate_ingress_class_list(list: &IngressClassList) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (i, item) in list.items.iter().enumerate() {
        let item_path = Path::new("items").index(i);
        all_errs.extend(validate_ingress_class_with_path(item, &item_path));
    }

    all_errs
}

// ============================================================================
// NetworkPolicy Validation
// ============================================================================

/// Validates a NetworkPolicy object.
pub fn validate_network_policy(policy: &NetworkPolicy) -> ErrorList {
    validate_network_policy_with_path(policy, &Path::nil())
}

fn validate_network_policy_with_path(policy: &NetworkPolicy, base_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let default_meta = crate::common::ObjectMeta::default();
    let meta = policy.metadata.as_ref().unwrap_or(&default_meta);

    all_errs.extend(validate_object_meta(
        meta,
        true,
        name_is_dns_subdomain,
        &base_path.child("metadata"),
    ));

    // Validate spec if present
    if let Some(spec) = &policy.spec {
        let spec_path = base_path.child("spec");

        all_errs.extend(validate_label_selector(
            &spec.pod_selector,
            &spec_path.child("podSelector"),
        ));

        // Validate ingress rules
        for (i, rule) in spec.ingress.iter().enumerate() {
            let rule_path = spec_path.child("ingress").index(i);

            // Validate ports
            for (j, port) in rule.ports.iter().enumerate() {
                let port_path = rule_path.child("ports").index(j);
                all_errs.extend(validate_network_policy_port(port, &port_path));
            }

            // Validate peers
            for (k, peer) in rule.from.iter().enumerate() {
                let peer_path = rule_path.child("from").index(k);
                all_errs.extend(validate_network_policy_peer(peer, &peer_path));
            }
        }

        // Validate egress rules
        for (i, rule) in spec.egress.iter().enumerate() {
            let rule_path = spec_path.child("egress").index(i);

            // Validate ports
            for (j, port) in rule.ports.iter().enumerate() {
                let port_path = rule_path.child("ports").index(j);
                all_errs.extend(validate_network_policy_port(port, &port_path));
            }

            // Validate peers
            for (k, peer) in rule.to.iter().enumerate() {
                let peer_path = rule_path.child("to").index(k);
                all_errs.extend(validate_network_policy_peer(peer, &peer_path));
            }
        }

        if spec.policy_types.len() > SUPPORTED_POLICY_TYPES.len() {
            all_errs.push(invalid(
                &spec_path.child("policyTypes"),
                BadValue::String(format!("{:?}", spec.policy_types)),
                "may not specify more than two policyTypes",
            ));
        }
    }

    all_errs
}

/// Validates a NetworkPolicyList object.
pub fn validate_network_policy_list(list: &NetworkPolicyList) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (i, item) in list.items.iter().enumerate() {
        let item_path = Path::new("items").index(i);
        all_errs.extend(validate_network_policy_with_path(item, &item_path));
    }

    all_errs
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{IntOrString, LabelSelector, ObjectMeta, TypeMeta};

    #[test]
    fn test_validate_ingress_valid() {
        let ingress = Ingress {
            type_meta: TypeMeta {
                api_version: "networking.k8s.io/v1".to_string(),
                kind: "Ingress".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("test-ingress".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: None,
            status: None,
        };

        let errors = validate_ingress(&ingress);
        assert!(
            errors.is_empty(),
            "Expected no validation errors, got: {:?}",
            errors.errors
        );
    }

    #[test]
    fn test_validate_ingress_backend_mutual_exclusion() {
        use crate::networking::v1::ingress::{
            HTTPIngressPath, HTTPIngressRuleValue, IngressBackend, IngressRule,
            IngressServiceBackend, IngressSpec, PathType, ServiceBackendPort,
        };

        let ingress = Ingress {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-ingress".to_string()),
                ..Default::default()
            }),
            spec: Some(IngressSpec {
                ingress_class_name: None,
                default_backend: None,
                rules: vec![IngressRule {
                    host: "example.com".to_string(),
                    http: Some(HTTPIngressRuleValue {
                        paths: vec![HTTPIngressPath {
                            path: "/".to_string(),
                            path_type: PathType::Prefix,
                            backend: IngressBackend {
                                // Invalid: both service and resource specified
                                service: Some(IngressServiceBackend {
                                    name: "my-service".to_string(),
                                    port: Some(ServiceBackendPort {
                                        number: Some(80),
                                        name: String::new(),
                                    }),
                                }),
                                resource: Some(crate::core::v1::TypedLocalObjectReference {
                                    kind: Some("StorageBucket".to_string()),
                                    name: Some("my-bucket".to_string()),
                                    api_group: Some("example.com".to_string()),
                                }),
                            },
                        }],
                    }),
                }],
                tls: vec![],
            }),
            status: None,
        };

        let errors = validate_ingress(&ingress);
        assert!(!errors.is_empty(), "Expected validation errors");
        assert!(
            errors
                .errors
                .iter()
                .any(|e| e.detail.contains("cannot set both service and resource")),
            "Expected service/resource exclusivity error"
        );
    }

    #[test]
    fn test_validate_ingress_backend_missing_both() {
        use crate::networking::v1::ingress::{
            HTTPIngressPath, HTTPIngressRuleValue, IngressBackend, IngressRule, IngressSpec,
            PathType,
        };

        let ingress = Ingress {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-ingress".to_string()),
                ..Default::default()
            }),
            spec: Some(IngressSpec {
                ingress_class_name: None,
                default_backend: None,
                rules: vec![IngressRule {
                    host: "example.com".to_string(),
                    http: Some(HTTPIngressRuleValue {
                        paths: vec![HTTPIngressPath {
                            path: "/".to_string(),
                            path_type: PathType::Prefix,
                            backend: IngressBackend {
                                // Invalid: neither service nor resource specified
                                service: None,
                                resource: None,
                            },
                        }],
                    }),
                }],
                tls: vec![],
            }),
            status: None,
        };

        let errors = validate_ingress(&ingress);
        assert!(!errors.is_empty(), "Expected validation errors");
        assert!(
            errors
                .errors
                .iter()
                .any(|e| e.detail.contains("service or resource backend is required")),
            "Expected error about missing backend"
        );
    }

    #[test]
    fn test_validate_ingress_requires_rules_or_default_backend() {
        use crate::networking::v1::ingress::IngressSpec;

        let ingress = Ingress {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-ingress".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(IngressSpec {
                ingress_class_name: None,
                default_backend: None,
                rules: vec![],
                tls: vec![],
            }),
            status: None,
        };

        let errors = validate_ingress(&ingress);
        assert!(!errors.is_empty(), "Expected validation errors");
        assert!(
            errors
                .errors
                .iter()
                .any(|e| e.detail.contains("defaultBackend") || e.detail.contains("rules")),
            "Expected error about missing rules or default backend"
        );
    }

    #[test]
    fn test_validate_ingress_path_invalid_sequence() {
        use crate::networking::v1::ingress::{
            HTTPIngressPath, HTTPIngressRuleValue, IngressBackend, IngressRule,
            IngressServiceBackend, IngressSpec, PathType, ServiceBackendPort,
        };

        let ingress = Ingress {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-ingress".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(IngressSpec {
                ingress_class_name: None,
                default_backend: None,
                rules: vec![IngressRule {
                    host: "example.com".to_string(),
                    http: Some(HTTPIngressRuleValue {
                        paths: vec![HTTPIngressPath {
                            path: "/../".to_string(),
                            path_type: PathType::Prefix,
                            backend: IngressBackend {
                                service: Some(IngressServiceBackend {
                                    name: "svc".to_string(),
                                    port: Some(ServiceBackendPort {
                                        number: Some(80),
                                        name: String::new(),
                                    }),
                                }),
                                resource: None,
                            },
                        }],
                    }),
                }],
                tls: vec![],
            }),
            status: None,
        };

        let errors = validate_ingress(&ingress);
        assert!(!errors.is_empty(), "Expected validation errors");
        assert!(
            errors
                .errors
                .iter()
                .any(|e| e.detail.contains("must not contain")),
            "Expected error about invalid path sequence"
        );
    }

    #[test]
    fn test_validate_ingress_tls_invalid_wildcard() {
        use crate::networking::v1::ingress::IngressSpec;
        use crate::networking::v1::ingress::IngressTLS;

        let ingress = Ingress {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-ingress".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(IngressSpec {
                ingress_class_name: None,
                default_backend: None,
                rules: vec![],
                tls: vec![IngressTLS {
                    hosts: vec!["foo.*.example.com".to_string()],
                    secret_name: None,
                }],
            }),
            status: None,
        };

        let errors = validate_ingress(&ingress);
        assert!(!errors.is_empty(), "Expected validation errors");
        assert!(
            errors.errors.iter().any(|e| e.detail.contains("wildcard")),
            "Expected wildcard validation error"
        );
    }

    #[test]
    fn test_validate_ingress_class_valid() {
        use crate::networking::v1::ingress_class::IngressClassSpec;

        let ingress_class = IngressClass {
            type_meta: TypeMeta {
                api_version: "networking.k8s.io/v1".to_string(),
                kind: "IngressClass".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("test-class".to_string()),
                ..Default::default()
            }),
            spec: IngressClassSpec {
                controller: "example.com/ingress-controller".to_string(),
                parameters: None,
            },
        };

        let errors = validate_ingress_class(&ingress_class);
        assert!(
            errors.is_empty(),
            "Expected no validation errors, got: {:?}",
            errors.errors
        );
    }

    #[test]
    fn test_validate_ingress_class_missing_controller() {
        use crate::networking::v1::ingress_class::IngressClassSpec;

        let ingress_class = IngressClass {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-class".to_string()),
                ..Default::default()
            }),
            spec: IngressClassSpec {
                controller: String::new(), // Invalid: empty
                parameters: None,
            },
        };

        let errors = validate_ingress_class(&ingress_class);
        assert!(!errors.is_empty(), "Expected validation errors");
        assert!(
            errors
                .errors
                .iter()
                .any(|e| e.field.to_string().contains("controller")),
            "Expected error for missing controller"
        );
    }

    #[test]
    fn test_validate_ingress_class_parameters_scope_required() {
        use crate::networking::v1::ingress_class::{
            IngressClassParametersReference, IngressClassSpec,
        };

        let ingress_class = IngressClass {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-class".to_string()),
                ..Default::default()
            }),
            spec: IngressClassSpec {
                controller: "example.com/ingress-controller".to_string(),
                parameters: Some(IngressClassParametersReference {
                    api_group: Some("example.com".to_string()),
                    kind: "BackendConfig".to_string(),
                    name: "default".to_string(),
                    namespace: None,
                    scope: None,
                }),
            },
        };

        let errors = validate_ingress_class(&ingress_class);
        assert!(!errors.is_empty(), "Expected validation errors");
        assert!(
            errors
                .errors
                .iter()
                .any(|e| e.field.to_string().contains("scope")),
            "Expected error for missing parameters.scope"
        );
    }

    #[test]
    fn test_validate_network_policy_valid() {
        use crate::networking::v1::network_policy::NetworkPolicySpec;

        let policy = NetworkPolicy {
            type_meta: TypeMeta {
                api_version: "networking.k8s.io/v1".to_string(),
                kind: "NetworkPolicy".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("test-policy".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(NetworkPolicySpec {
                pod_selector: LabelSelector::default(),
                ingress: vec![],
                egress: vec![],
                policy_types: vec![],
            }),
        };

        let errors = validate_network_policy(&policy);
        assert!(
            errors.is_empty(),
            "Expected no validation errors, got: {:?}",
            errors.errors
        );
    }

    #[test]
    fn test_validate_network_policy_peer_requires_one() {
        use crate::networking::v1::network_policy::{NetworkPolicyIngressRule, NetworkPolicySpec};

        let policy = NetworkPolicy {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-policy".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(NetworkPolicySpec {
                pod_selector: LabelSelector::default(),
                ingress: vec![NetworkPolicyIngressRule {
                    ports: vec![],
                    from: vec![crate::networking::v1::network_policy::NetworkPolicyPeer::default()],
                }],
                egress: vec![],
                policy_types: vec![],
            }),
        };

        let errors = validate_network_policy(&policy);
        assert!(!errors.is_empty(), "Expected validation errors");
        assert!(
            errors
                .errors
                .iter()
                .any(|e| e.detail.contains("must specify a peer")),
            "Expected peer requirement error"
        );
    }

    #[test]
    fn test_validate_network_policy_invalid_protocol() {
        use crate::networking::v1::network_policy::{
            NetworkPolicyIngressRule, NetworkPolicyPort, NetworkPolicySpec,
        };

        let policy = NetworkPolicy {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-policy".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(NetworkPolicySpec {
                pod_selector: LabelSelector::default(),
                ingress: vec![NetworkPolicyIngressRule {
                    ports: vec![NetworkPolicyPort {
                        protocol: Some("ICMP".to_string()),
                        port: Some(IntOrString::Int(80)),
                        end_port: None,
                    }],
                    from: vec![],
                }],
                egress: vec![],
                policy_types: vec![],
            }),
        };

        let errors = validate_network_policy(&policy);
        assert!(!errors.is_empty(), "Expected validation errors");
        assert!(
            errors
                .errors
                .iter()
                .any(|e| e.field.to_string().contains("protocol")),
            "Expected protocol validation error"
        );
    }

    #[test]
    fn test_validate_network_policy_port_invalid_number() {
        use crate::networking::v1::network_policy::{
            NetworkPolicyIngressRule, NetworkPolicyPort, NetworkPolicySpec,
        };

        let policy = NetworkPolicy {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-policy".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(NetworkPolicySpec {
                pod_selector: LabelSelector::default(),
                ingress: vec![NetworkPolicyIngressRule {
                    ports: vec![NetworkPolicyPort {
                        protocol: Some("TCP".to_string()),
                        port: Some(IntOrString::Int(70000)),
                        end_port: None,
                    }],
                    from: vec![],
                }],
                egress: vec![],
                policy_types: vec![],
            }),
        };

        let errors = validate_network_policy(&policy);
        assert!(!errors.is_empty(), "Expected validation errors");
        assert!(
            errors
                .errors
                .iter()
                .any(|e| e.field.to_string().contains("port")),
            "Expected port number validation error"
        );
    }

    #[test]
    fn test_validate_network_policy_policy_types_limit() {
        use crate::networking::v1::network_policy::{NetworkPolicySpec, PolicyType};

        let policy = NetworkPolicy {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-policy".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(NetworkPolicySpec {
                pod_selector: LabelSelector::default(),
                ingress: vec![],
                egress: vec![],
                policy_types: vec![PolicyType::Ingress, PolicyType::Egress, PolicyType::Ingress],
            }),
        };

        let errors = validate_network_policy(&policy);
        assert!(!errors.is_empty(), "Expected validation errors");
        assert!(
            errors
                .errors
                .iter()
                .any(|e| e.detail.contains("policyTypes")),
            "Expected policyTypes length error"
        );
    }

    #[test]
    fn test_validate_network_policy_port_endport_invalid() {
        use crate::networking::v1::network_policy::{
            NetworkPolicyIngressRule, NetworkPolicyPort, NetworkPolicySpec,
        };

        let policy = NetworkPolicy {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-policy".to_string()),
                ..Default::default()
            }),
            spec: Some(NetworkPolicySpec {
                pod_selector: LabelSelector::default(),
                ingress: vec![NetworkPolicyIngressRule {
                    ports: vec![NetworkPolicyPort {
                        protocol: Some("TCP".to_string()),
                        port: Some(IntOrString::Int(80)),
                        end_port: Some(70), // Invalid: endPort < port
                    }],
                    from: vec![],
                }],
                egress: vec![],
                policy_types: vec![],
            }),
        };

        let errors = validate_network_policy(&policy);
        assert!(!errors.is_empty(), "Expected validation errors");
        assert!(
            errors
                .errors
                .iter()
                .any(|e| e.field.to_string().contains("endPort")),
            "Expected error for invalid endPort"
        );
    }

    #[test]
    fn test_validate_network_policy_port_endport_with_named_port() {
        use crate::networking::v1::network_policy::{
            NetworkPolicyIngressRule, NetworkPolicyPort, NetworkPolicySpec,
        };

        let policy = NetworkPolicy {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-policy".to_string()),
                ..Default::default()
            }),
            spec: Some(NetworkPolicySpec {
                pod_selector: LabelSelector::default(),
                ingress: vec![NetworkPolicyIngressRule {
                    ports: vec![NetworkPolicyPort {
                        protocol: Some("TCP".to_string()),
                        port: Some(IntOrString::String("http".to_string())),
                        end_port: Some(8080), // Invalid: endPort with named port
                    }],
                    from: vec![],
                }],
                egress: vec![],
                policy_types: vec![],
            }),
        };

        let errors = validate_network_policy(&policy);
        assert!(!errors.is_empty(), "Expected validation errors");
        assert!(
            errors
                .errors
                .iter()
                .any(|e| e.detail.contains("non-numeric")),
            "Expected error for endPort with named port"
        );
    }

    #[test]
    fn test_validate_ip_block_invalid_cidr() {
        use crate::networking::v1::network_policy::{
            IPBlock, NetworkPolicyIngressRule, NetworkPolicyPeer, NetworkPolicySpec,
        };

        let policy = NetworkPolicy {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-policy".to_string()),
                ..Default::default()
            }),
            spec: Some(NetworkPolicySpec {
                pod_selector: LabelSelector::default(),
                ingress: vec![NetworkPolicyIngressRule {
                    ports: vec![],
                    from: vec![NetworkPolicyPeer {
                        pod_selector: None,
                        namespace_selector: None,
                        ip_block: Some(IPBlock {
                            cidr: "invalid-cidr".to_string(), // Invalid: no / separator
                            except: vec![],
                        }),
                    }],
                }],
                egress: vec![],
                policy_types: vec![],
            }),
        };

        let errors = validate_network_policy(&policy);
        assert!(!errors.is_empty(), "Expected validation errors");
        assert!(
            errors
                .errors
                .iter()
                .any(|e| e.field.to_string().contains("cidr")),
            "Expected error for invalid CIDR"
        );
    }
}
