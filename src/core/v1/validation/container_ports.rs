//! Container port validation for Kubernetes core/v1 API
//!
//! This module implements validation for container ports and host port conflicts.

use crate::common::ToInternal;
use crate::common::validation::{BadValue, ErrorList, Path, not_supported, required};
use crate::core::internal;
use crate::core::internal::validation::container_ports as internal_container_ports;
use crate::core::v1::pod::ContainerPort;
use std::collections::HashSet;
use std::sync::LazyLock;

// ============================================================================
// Constants
// ============================================================================

/// Supported port protocols
static SUPPORTED_PORT_PROTOCOLS: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| HashSet::from(["TCP", "UDP", "SCTP"]));

// ============================================================================
// Container Port Validation
// ============================================================================

/// Validates a list of container ports.
///
/// Validates:
/// - Port names are unique and valid
/// - Container ports are required and in valid range (1-65535)
/// - Host ports are in valid range if specified
/// - Protocols are required and supported (TCP, UDP, SCTP)
pub fn validate_container_ports(ports: &[ContainerPort], path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (i, port) in ports.iter().enumerate() {
        let idx_path = path.index(i);

        // Validate protocol
        if let Some(ref protocol) = port.protocol {
            if protocol.is_empty() {
                all_errs.push(required(
                    &idx_path.child("protocol"),
                    "protocol is required",
                ));
            } else if !SUPPORTED_PORT_PROTOCOLS.contains(protocol.as_str()) {
                let valid: Vec<&str> = SUPPORTED_PORT_PROTOCOLS.iter().copied().collect();
                all_errs.push(not_supported(
                    &idx_path.child("protocol"),
                    BadValue::String(protocol.clone()),
                    &valid,
                ));
            }
        } else {
            all_errs.push(required(
                &idx_path.child("protocol"),
                "protocol is required",
            ));
        }
    }

    let internal_ports: Vec<internal::ContainerPort> =
        ports.iter().cloned().map(ToInternal::to_internal).collect();
    all_errs.extend(internal_container_ports::validate_container_ports(
        &internal_ports,
        path,
    ));

    all_errs
}

/// Accumulates unique host ports across containers and checks for conflicts.
///
/// Host ports are unique per protocol+hostIP combination.
pub fn accumulate_unique_host_ports(containers: &[&[ContainerPort]], path: &Path) -> ErrorList {
    let internal_containers: Vec<Vec<internal::ContainerPort>> = containers
        .iter()
        .map(|ports| ports.iter().cloned().map(ToInternal::to_internal).collect())
        .collect();
    let internal_refs: Vec<&[internal::ContainerPort]> = internal_containers
        .iter()
        .map(|ports| ports.as_slice())
        .collect();

    internal_container_ports::accumulate_unique_host_ports(&internal_refs, path)
}

/// Checks for host port conflicts within a single container list.
pub fn check_host_port_conflicts(ports: &[ContainerPort], path: &Path) -> ErrorList {
    let internal_ports: Vec<internal::ContainerPort> =
        ports.iter().cloned().map(ToInternal::to_internal).collect();
    internal_container_ports::check_host_port_conflicts(&internal_ports, path)
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::validation::ErrorType;

    fn is_valid_port_num(port: i32) -> bool {
        (1..=65535).contains(&port)
    }

    #[test]
    fn test_validate_container_ports_missing_container_port() {
        let ports = vec![ContainerPort {
            name: None,
            container_port: 0, // Invalid - required
            protocol: Some("TCP".to_string()),
            host_port: None,
            host_ip: None,
        }];

        let errs = validate_container_ports(&ports, &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.detail.contains("containerPort is required"))
        );
    }

    #[test]
    fn test_validate_container_ports_invalid_port_range() {
        let ports = vec![ContainerPort {
            name: None,
            container_port: 70000, // Invalid - out of range
            protocol: Some("TCP".to_string()),
            host_port: None,
            host_ip: None,
        }];

        let errs = validate_container_ports(&ports, &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.detail.contains("must be between 1 and 65535"))
        );
    }

    #[test]
    fn test_validate_container_ports_invalid_protocol() {
        let ports = vec![ContainerPort {
            name: None,
            container_port: 8080,
            protocol: Some("HTTP".to_string()), // Invalid protocol
            host_port: None,
            host_ip: None,
        }];

        let errs = validate_container_ports(&ports, &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.field.contains("protocol") && e.error_type == ErrorType::NotSupported)
        );
    }

    #[test]
    fn test_validate_container_ports_duplicate_names() {
        let ports = vec![
            ContainerPort {
                name: Some("http".to_string()),
                container_port: 8080,
                protocol: Some("TCP".to_string()),
                host_port: None,
                host_ip: None,
            },
            ContainerPort {
                name: Some("http".to_string()), // Duplicate name
                container_port: 9090,
                protocol: Some("TCP".to_string()),
                host_port: None,
                host_ip: None,
            },
        ];

        let errs = validate_container_ports(&ports, &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.error_type == crate::common::validation::ErrorType::Duplicate)
        );
    }

    #[test]
    fn test_check_host_port_conflicts() {
        let ports = vec![
            ContainerPort {
                name: None,
                container_port: 8080,
                protocol: Some("TCP".to_string()),
                host_port: Some(80),
                host_ip: None,
            },
            ContainerPort {
                name: None,
                container_port: 9090,
                protocol: Some("TCP".to_string()),
                host_port: Some(80), // Duplicate host port
                host_ip: None,
            },
        ];

        let errs = check_host_port_conflicts(&ports, &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.error_type == crate::common::validation::ErrorType::Duplicate)
        );
    }

    #[test]
    fn test_check_host_port_no_conflict_different_protocol() {
        let ports = vec![
            ContainerPort {
                name: None,
                container_port: 8080,
                protocol: Some("TCP".to_string()),
                host_port: Some(80),
                host_ip: None,
            },
            ContainerPort {
                name: None,
                container_port: 9090,
                protocol: Some("UDP".to_string()),
                host_port: Some(80), // Same port but different protocol - OK
                host_ip: None,
            },
        ];

        let errs = check_host_port_conflicts(&ports, &Path::nil());
        assert!(errs.is_empty(), "Different protocols should not conflict");
    }

    #[test]
    fn test_check_host_port_no_conflict_different_host_ip() {
        let ports = vec![
            ContainerPort {
                name: None,
                container_port: 8080,
                protocol: Some("TCP".to_string()),
                host_port: Some(80),
                host_ip: Some("192.168.1.1".to_string()),
            },
            ContainerPort {
                name: None,
                container_port: 9090,
                protocol: Some("TCP".to_string()),
                host_port: Some(80), // Same port but different host IP - OK
                host_ip: Some("192.168.1.2".to_string()),
            },
        ];

        let errs = check_host_port_conflicts(&ports, &Path::nil());
        assert!(errs.is_empty(), "Different host IPs should not conflict");
    }

    #[test]
    fn test_is_valid_port_num() {
        assert!(is_valid_port_num(1));
        assert!(is_valid_port_num(8080));
        assert!(is_valid_port_num(65535));

        assert!(!is_valid_port_num(0));
        assert!(!is_valid_port_num(-1));
        assert!(!is_valid_port_num(65536));
        assert!(!is_valid_port_num(100000));
    }

    #[test]
    fn test_validate_container_ports_valid() {
        let ports = vec![
            ContainerPort {
                name: Some("http".to_string()),
                container_port: 8080,
                protocol: Some("TCP".to_string()),
                host_port: Some(80),
                host_ip: None,
            },
            ContainerPort {
                name: Some("metrics".to_string()),
                container_port: 9090,
                protocol: Some("TCP".to_string()),
                host_port: None,
                host_ip: None,
            },
        ];

        let errs = validate_container_ports(&ports, &Path::nil());
        assert!(errs.is_empty(), "Valid ports should not produce errors");
    }
}
