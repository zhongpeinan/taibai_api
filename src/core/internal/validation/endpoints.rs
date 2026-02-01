//! Endpoints validation for Kubernetes core internal API.
//!
//! Ported from k8s.io/kubernetes/pkg/apis/core/validation/validation.go

use crate::common::validation::{BadValue, ErrorList, Path, invalid, not_supported, required};
use crate::core::internal::endpoints::{EndpointAddress, EndpointPort, EndpointSubset, Endpoints};
use crate::core::internal::{Protocol, protocol};

/// Validates Endpoints
pub fn validate_endpoints(endpoints: &Endpoints) -> ErrorList {
    validate_endpoints_with_path(endpoints, None, &Path::nil())
}

fn validate_endpoints_with_path(
    endpoints: &Endpoints,
    old_endpoints: Option<&Endpoints>,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    all_errs.extend(crate::common::validation::validate_object_meta(
        &endpoints.metadata,
        true, // Endpoints is namespaced
        validate_endpoints_name,
        &path.child("metadata"),
    ));

    all_errs.extend(validate_endpoints_specific_annotations(
        &endpoints.metadata.annotations,
        &path.child("metadata").child("annotations"),
    ));

    // Validate subsets
    let subsets_path = path.child("subsets");
    let mut subset_errs = ErrorList::new();
    for (i, subset) in endpoints.subsets.iter().enumerate() {
        subset_errs.extend(validate_endpoint_subset(subset, &subsets_path.index(i)));
    }
    if !subset_errs.is_empty() {
        if let Some(old) = old_endpoints {
            if old.subsets == endpoints.subsets {
                subset_errs = ErrorList::new();
            }
        }
    }
    all_errs.extend(subset_errs);

    all_errs
}

/// Validates Endpoints update
pub fn validate_endpoints_update(new: &Endpoints, old: &Endpoints) -> ErrorList {
    validate_endpoints_update_with_path(new, old, &Path::nil())
}

fn validate_endpoints_update_with_path(new: &Endpoints, old: &Endpoints, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    all_errs.extend(crate::common::validation::validate_object_meta_update(
        &new.metadata,
        &old.metadata,
        &path.child("metadata"),
    ));

    // Validate the new endpoints
    all_errs.extend(validate_endpoints_with_path(new, Some(old), path));

    all_errs
}

fn validate_endpoint_subset(subset: &EndpointSubset, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Require at least one address across ready/not-ready subsets
    if subset.addresses.is_empty() && subset.not_ready_addresses.is_empty() {
        all_errs.push(required(
            path,
            "must specify `addresses` or `notReadyAddresses`",
        ));
    }

    // Determine if port names are required (when there are multiple ports)
    let require_port_name = subset.ports.len() > 1;

    // Validate addresses
    let addresses_path = path.child("addresses");
    for (i, address) in subset.addresses.iter().enumerate() {
        all_errs.extend(validate_endpoint_address(address, &addresses_path.index(i)));
    }

    // Validate not ready addresses
    let not_ready_path = path.child("notReadyAddresses");
    for (i, address) in subset.not_ready_addresses.iter().enumerate() {
        all_errs.extend(validate_endpoint_address(address, &not_ready_path.index(i)));
    }

    // Validate ports
    let ports_path = path.child("ports");
    let mut port_names = std::collections::HashSet::new();

    for (i, port) in subset.ports.iter().enumerate() {
        let port_path = ports_path.index(i);
        all_errs.extend(validate_endpoint_port(port, require_port_name, &port_path));

        // Check for duplicate port names
        if !port.name.is_empty() {
            if port_names.contains(&port.name) {
                all_errs.push(invalid(
                    &port_path.child("name"),
                    BadValue::String(port.name.clone()),
                    "duplicate port name",
                ));
            }
            port_names.insert(port.name.clone());
        }
    }

    all_errs
}

fn validate_endpoint_address(address: &EndpointAddress, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // IP is required
    if address.ip.is_empty() {
        all_errs.push(required(&path.child("ip"), "ip address is required"));
    } else {
        // Validate IP address format
        all_errs.extend(validate_endpoint_ip(&address.ip, &path.child("ip")));
    }

    // Hostname validation (if present)
    if !address.hostname.is_empty() {
        let errors = crate::common::validation::is_dns1123_label(&address.hostname);
        for err in errors {
            all_errs.push(invalid(
                &path.child("hostname"),
                BadValue::String(address.hostname.clone()),
                &err,
            ));
        }
    }

    // NodeName validation (if present)
    if !address.node_name.is_empty() {
        let errors = crate::common::validation::is_dns1123_subdomain(&address.node_name);
        for err in errors {
            all_errs.push(invalid(
                &path.child("nodeName"),
                BadValue::String(address.node_name.clone()),
                &err,
            ));
        }
    }

    all_errs
}

fn validate_endpoint_port(port: &EndpointPort, require_name: bool, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Name validation
    if require_name && port.name.is_empty() {
        all_errs.push(required(
            &path.child("name"),
            "port name is required when multiple ports are defined",
        ));
    } else if !port.name.is_empty() {
        let errors = crate::common::validation::is_dns1123_label(&port.name);
        for err in errors {
            all_errs.push(invalid(
                &path.child("name"),
                BadValue::String(port.name.clone()),
                &err,
            ));
        }
    }

    // Port number validation
    if port.port <= 0 || port.port > 65535 {
        all_errs.push(invalid(
            &path.child("port"),
            BadValue::Int(port.port as i64),
            "must be between 1 and 65535",
        ));
    }

    // Protocol validation
    let protocol_value = protocol_to_str(&port.protocol);
    if !matches!(
        protocol_value,
        protocol::TCP | protocol::UDP | protocol::SCTP
    ) {
        all_errs.push(not_supported(
            &path.child("protocol"),
            BadValue::String(protocol_value.to_string()),
            &[protocol::TCP, protocol::UDP, protocol::SCTP],
        ));
    }

    // AppProtocol validation (if present)
    if let Some(app_protocol) = &port.app_protocol {
        if !app_protocol.is_empty() {
            let errors = crate::common::validation::is_qualified_name(app_protocol);
            for err in errors {
                all_errs.push(invalid(
                    &path.child("appProtocol"),
                    BadValue::String(app_protocol.clone()),
                    &err,
                ));
            }
        }
    }

    all_errs
}

fn validate_endpoints_specific_annotations(
    _annotations: &std::collections::BTreeMap<String, String>,
    _path: &Path,
) -> ErrorList {
    ErrorList::new()
}

/// Validates an endpoint IP address
///
/// This implements basic IP validation following upstream patterns.
/// Disallows unspecified, loopback, and link-local addresses.
fn validate_endpoint_ip(ip_address: &str, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Try to parse as IP address
    if let Ok(ip) = ip_address.parse::<std::net::IpAddr>() {
        match ip {
            std::net::IpAddr::V4(ipv4) => {
                // Unspecified (0.0.0.0)
                if ipv4.is_unspecified() {
                    all_errs.push(invalid(
                        path,
                        BadValue::String(ip_address.to_string()),
                        "may not be unspecified (0.0.0.0)",
                    ));
                }
                // Loopback (127.0.0.0/8)
                if ipv4.is_loopback() {
                    all_errs.push(invalid(
                        path,
                        BadValue::String(ip_address.to_string()),
                        "may not be in the loopback range (127.0.0.0/8)",
                    ));
                }
                // Link-local (169.254.0.0/16)
                if ipv4.octets()[0] == 169 && ipv4.octets()[1] == 254 {
                    all_errs.push(invalid(
                        path,
                        BadValue::String(ip_address.to_string()),
                        "may not be in the link-local range (169.254.0.0/16)",
                    ));
                }
                // Link-local multicast (224.0.0.0/24)
                if ipv4.is_multicast()
                    && ipv4.octets()[0] == 224
                    && ipv4.octets()[1] == 0
                    && ipv4.octets()[2] == 0
                {
                    all_errs.push(invalid(
                        path,
                        BadValue::String(ip_address.to_string()),
                        "may not be in the link-local multicast range (224.0.0.0/24)",
                    ));
                }
            }
            std::net::IpAddr::V6(ipv6) => {
                // Unspecified (::)
                if ipv6.is_unspecified() {
                    all_errs.push(invalid(
                        path,
                        BadValue::String(ip_address.to_string()),
                        "may not be unspecified (::)",
                    ));
                }
                // Loopback (::1)
                if ipv6.is_loopback() {
                    all_errs.push(invalid(
                        path,
                        BadValue::String(ip_address.to_string()),
                        "may not be in the loopback range (::1/128)",
                    ));
                }
                // Link-local unicast (fe80::/10)
                if ipv6.segments()[0] >= 0xfe80 && ipv6.segments()[0] <= 0xfebf {
                    all_errs.push(invalid(
                        path,
                        BadValue::String(ip_address.to_string()),
                        "may not be in the link-local range (fe80::/10)",
                    ));
                }
                // Link-local multicast (ff02::/10)
                if ipv6.is_multicast() && ipv6.segments()[0] == 0xff02 {
                    all_errs.push(invalid(
                        path,
                        BadValue::String(ip_address.to_string()),
                        "may not be in the link-local multicast range (ff02::/10)",
                    ));
                }
            }
        }
    } else {
        all_errs.push(invalid(
            path,
            BadValue::String(ip_address.to_string()),
            "must be a valid IP address",
        ));
    }

    all_errs
}

// ============================================================================
// Helper Functions
// ============================================================================

fn validate_endpoints_name(name: &str, _prefix: bool) -> Vec<String> {
    crate::common::validation::is_dns1123_subdomain(name)
}

fn protocol_to_str(value: &Protocol) -> &'static str {
    match value {
        Protocol::Tcp => protocol::TCP,
        Protocol::Udp => protocol::UDP,
        Protocol::Sctp => protocol::SCTP,
    }
}
