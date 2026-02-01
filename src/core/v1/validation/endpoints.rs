//! Endpoints validation for Kubernetes core/v1 API.
//!
//! Delegates to internal validation and retains v1-only protocol checks.

use crate::common::ToInternal;
use crate::common::validation::{BadValue, ErrorList, Path, not_supported, required};
use crate::core::internal::validation::endpoints as internal_endpoints_validation;
use crate::core::v1::{EndpointPort, Endpoints};

/// Validates Endpoints
pub fn validate_endpoints(endpoints: &Endpoints) -> ErrorList {
    let mut all_errs = validate_endpoints_v1_protocols(endpoints, &Path::nil());
    let internal_endpoints = endpoints.clone().to_internal();
    all_errs.extend(internal_endpoints_validation::validate_endpoints(
        &internal_endpoints,
    ));
    all_errs
}

/// Validates Endpoints update
pub fn validate_endpoints_update(new: &Endpoints, old: &Endpoints) -> ErrorList {
    let mut all_errs = validate_endpoints_v1_protocols(new, &Path::nil());
    let internal_new = new.clone().to_internal();
    let internal_old = old.clone().to_internal();
    all_errs.extend(internal_endpoints_validation::validate_endpoints_update(
        &internal_new,
        &internal_old,
    ));
    all_errs
}

fn validate_endpoints_v1_protocols(endpoints: &Endpoints, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let subsets_path = path.child("subsets");

    for (i, subset) in endpoints.subsets.iter().enumerate() {
        let ports_path = subsets_path.index(i).child("ports");
        for (j, port) in subset.ports.iter().enumerate() {
            all_errs.extend(validate_endpoint_port_protocol(port, &ports_path.index(j)));
        }
    }

    all_errs
}

fn validate_endpoint_port_protocol(port: &EndpointPort, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if port.protocol.is_empty() {
        all_errs.push(required(&path.child("protocol"), "protocol is required"));
    } else if !matches!(port.protocol.as_str(), "TCP" | "UDP" | "SCTP") {
        all_errs.push(not_supported(
            &path.child("protocol"),
            BadValue::String(port.protocol.clone()),
            &["TCP", "UDP", "SCTP"],
        ));
    }

    all_errs
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{ObjectMeta, TypeMeta};
    use crate::core::v1::{EndpointAddress, EndpointPort, EndpointSubset};

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
