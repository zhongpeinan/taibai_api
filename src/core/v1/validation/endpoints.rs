//! Endpoints validation for Kubernetes core/v1 API
//!
//! Ported from k8s.io/kubernetes/pkg/apis/core/validation/validation.go

use crate::common::validation::{BadValue, ErrorList, Path, invalid, required};
use crate::core::v1::{EndpointAddress, EndpointPort, EndpointSubset, Endpoints};

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

    // Validate metadata (Endpoints is namespaced)
    if let Some(ref metadata) = endpoints.metadata {
        all_errs.extend(crate::common::validation::validate_object_meta(
            metadata,
            true, // Endpoints is namespaced
            validate_endpoints_name,
            &path.child("metadata"),
        ));

        all_errs.extend(validate_endpoints_specific_annotations(
            &metadata.annotations,
            &path.child("metadata").child("annotations"),
        ));
    } else {
        all_errs.push(required(&path.child("metadata"), "metadata is required"));
    }

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

    // Validate metadata update
    if let (Some(new_meta), Some(old_meta)) = (&new.metadata, &old.metadata) {
        all_errs.extend(crate::common::validation::validate_object_meta_update(
            new_meta,
            old_meta,
            &path.child("metadata"),
        ));
    }

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
    if let Some(ref node_name) = address.node_name {
        if !node_name.is_empty() {
            let errors = crate::common::validation::is_dns1123_subdomain(node_name);
            for err in errors {
                all_errs.push(invalid(
                    &path.child("nodeName"),
                    BadValue::String(node_name.clone()),
                    &err,
                ));
            }
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
    if port.protocol.is_empty() {
        all_errs.push(required(&path.child("protocol"), "protocol is required"));
    } else if !matches!(port.protocol.as_str(), "TCP" | "UDP" | "SCTP") {
        all_errs.push(crate::common::validation::not_supported(
            &path.child("protocol"),
            BadValue::String(port.protocol.clone()),
            &["TCP", "UDP", "SCTP"],
        ));
    }

    // AppProtocol validation (if present)
    if let Some(ref app_protocol) = port.app_protocol {
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

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{ObjectMeta, TypeMeta};

    fn create_test_endpoints(name: &str) -> Endpoints {
        Endpoints {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some(name.to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            subsets: vec![],
        }
    }

    fn create_test_endpoint_address(ip: &str) -> EndpointAddress {
        EndpointAddress {
            ip: ip.to_string(),
            hostname: String::new(),
            node_name: None,
            target_ref: None,
        }
    }

    fn create_test_endpoint_port(name: &str, port: i32) -> EndpointPort {
        EndpointPort {
            name: name.to_string(),
            port,
            protocol: "TCP".to_string(),
            app_protocol: None,
        }
    }

    #[test]
    fn test_validate_endpoints_valid() {
        let mut endpoints = create_test_endpoints("test-endpoints");
        endpoints.subsets = vec![EndpointSubset {
            addresses: vec![create_test_endpoint_address("192.168.1.1")],
            not_ready_addresses: vec![],
            ports: vec![create_test_endpoint_port("http", 80)],
        }];

        let errs = validate_endpoints(&endpoints);
        assert!(errs.is_empty(), "Expected no errors, got: {:?}", errs);
    }

    #[test]
    fn test_validate_endpoints_missing_ip() {
        let mut endpoints = create_test_endpoints("test-endpoints");
        endpoints.subsets = vec![EndpointSubset {
            addresses: vec![create_test_endpoint_address("")],
            not_ready_addresses: vec![],
            ports: vec![],
        }];

        let errs = validate_endpoints(&endpoints);
        assert!(!errs.is_empty(), "Expected errors for missing IP");
    }

    #[test]
    fn test_validate_endpoints_missing_addresses() {
        let mut endpoints = create_test_endpoints("test-endpoints");
        endpoints.subsets = vec![EndpointSubset {
            addresses: vec![],
            not_ready_addresses: vec![],
            ports: vec![create_test_endpoint_port("http", 80)],
        }];

        let errs = validate_endpoints(&endpoints);
        assert!(!errs.is_empty(), "Expected errors for missing addresses");
    }

    #[test]
    fn test_validate_endpoints_invalid_ip() {
        let mut endpoints = create_test_endpoints("test-endpoints");
        endpoints.subsets = vec![EndpointSubset {
            addresses: vec![create_test_endpoint_address("invalid-ip")],
            not_ready_addresses: vec![],
            ports: vec![],
        }];

        let errs = validate_endpoints(&endpoints);
        assert!(!errs.is_empty(), "Expected errors for invalid IP");
    }

    #[test]
    fn test_validate_endpoints_loopback_ip() {
        let mut endpoints = create_test_endpoints("test-endpoints");
        endpoints.subsets = vec![EndpointSubset {
            addresses: vec![create_test_endpoint_address("127.0.0.1")],
            not_ready_addresses: vec![],
            ports: vec![],
        }];

        let errs = validate_endpoints(&endpoints);
        assert!(!errs.is_empty(), "Expected errors for loopback IP");
    }

    #[test]
    fn test_validate_endpoints_unspecified_ip() {
        let mut endpoints = create_test_endpoints("test-endpoints");
        endpoints.subsets = vec![EndpointSubset {
            addresses: vec![create_test_endpoint_address("0.0.0.0")],
            not_ready_addresses: vec![],
            ports: vec![],
        }];

        let errs = validate_endpoints(&endpoints);
        assert!(!errs.is_empty(), "Expected errors for unspecified IP");
    }

    #[test]
    fn test_validate_endpoints_link_local_ip() {
        let mut endpoints = create_test_endpoints("test-endpoints");
        endpoints.subsets = vec![EndpointSubset {
            addresses: vec![create_test_endpoint_address("169.254.1.1")],
            not_ready_addresses: vec![],
            ports: vec![],
        }];

        let errs = validate_endpoints(&endpoints);
        assert!(!errs.is_empty(), "Expected errors for link-local IP");
    }

    #[test]
    fn test_validate_endpoints_invalid_port() {
        let mut endpoints = create_test_endpoints("test-endpoints");
        endpoints.subsets = vec![EndpointSubset {
            addresses: vec![create_test_endpoint_address("192.168.1.1")],
            not_ready_addresses: vec![],
            ports: vec![create_test_endpoint_port("http", 70000)],
        }];

        let errs = validate_endpoints(&endpoints);
        assert!(!errs.is_empty(), "Expected errors for invalid port");
    }

    #[test]
    fn test_validate_endpoints_missing_protocol() {
        let mut endpoints = create_test_endpoints("test-endpoints");
        endpoints.subsets = vec![EndpointSubset {
            addresses: vec![create_test_endpoint_address("192.168.1.1")],
            not_ready_addresses: vec![],
            ports: vec![EndpointPort {
                name: "http".to_string(),
                port: 80,
                protocol: String::new(),
                app_protocol: None,
            }],
        }];

        let errs = validate_endpoints(&endpoints);
        assert!(!errs.is_empty(), "Expected errors for missing protocol");
    }

    #[test]
    fn test_validate_endpoints_duplicate_port_names() {
        let mut endpoints = create_test_endpoints("test-endpoints");
        endpoints.subsets = vec![EndpointSubset {
            addresses: vec![create_test_endpoint_address("192.168.1.1")],
            not_ready_addresses: vec![],
            ports: vec![
                create_test_endpoint_port("http", 80),
                create_test_endpoint_port("http", 8080),
            ],
        }];

        let errs = validate_endpoints(&endpoints);
        assert!(!errs.is_empty(), "Expected errors for duplicate port names");
    }

    #[test]
    fn test_validate_endpoints_missing_port_name() {
        let mut endpoints = create_test_endpoints("test-endpoints");
        endpoints.subsets = vec![EndpointSubset {
            addresses: vec![create_test_endpoint_address("192.168.1.1")],
            not_ready_addresses: vec![],
            ports: vec![
                create_test_endpoint_port("http", 80),
                create_test_endpoint_port("", 8080), // Missing name for second port
            ],
        }];

        let errs = validate_endpoints(&endpoints);
        assert!(
            !errs.is_empty(),
            "Expected errors for missing port name when multiple ports"
        );
    }

    #[test]
    fn test_validate_endpoints_ipv6_valid() {
        let mut endpoints = create_test_endpoints("test-endpoints");
        endpoints.subsets = vec![EndpointSubset {
            addresses: vec![create_test_endpoint_address("2001:db8::1")],
            not_ready_addresses: vec![],
            ports: vec![create_test_endpoint_port("http", 80)],
        }];

        let errs = validate_endpoints(&endpoints);
        assert!(
            errs.is_empty(),
            "Expected no errors for valid IPv6, got: {:?}",
            errs
        );
    }

    #[test]
    fn test_validate_endpoints_ipv6_loopback() {
        let mut endpoints = create_test_endpoints("test-endpoints");
        endpoints.subsets = vec![EndpointSubset {
            addresses: vec![create_test_endpoint_address("::1")],
            not_ready_addresses: vec![],
            ports: vec![],
        }];

        let errs = validate_endpoints(&endpoints);
        assert!(!errs.is_empty(), "Expected errors for IPv6 loopback");
    }
}
