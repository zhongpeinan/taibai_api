//! Container ports validation for Kubernetes core internal API types.

use crate::common::validation::{BadValue, ErrorList, Path, invalid, required};
use crate::core::internal::{ContainerPort, Protocol, protocol};
use std::collections::HashSet;

/// Supported port protocols
fn protocol_to_str(protocol_value: &Protocol) -> &'static str {
    match protocol_value {
        Protocol::Tcp => protocol::TCP,
        Protocol::Udp => protocol::UDP,
        Protocol::Sctp => protocol::SCTP,
    }
}

/// Validates a list of container ports.
pub fn validate_container_ports(ports: &[ContainerPort], path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let mut all_names = HashSet::new();

    for (i, port) in ports.iter().enumerate() {
        let idx_path = path.index(i);

        if !port.name.is_empty() {
            all_errs.extend(validate_port_name(&port.name, &idx_path.child("name")));

            if all_names.contains(&port.name) {
                all_errs.push(crate::common::validation::duplicate(
                    &idx_path.child("name"),
                    BadValue::String(port.name.clone()),
                ));
            } else {
                all_names.insert(port.name.clone());
            }
        }

        if port.container_port == 0 {
            all_errs.push(required(
                &idx_path.child("containerPort"),
                "containerPort is required",
            ));
        } else if !is_valid_port_num(port.container_port) {
            all_errs.push(invalid(
                &idx_path.child("containerPort"),
                BadValue::Int(port.container_port.into()),
                "must be between 1 and 65535",
            ));
        }

        if let Some(host_port) = port.host_port {
            if host_port != 0 && !is_valid_port_num(host_port) {
                all_errs.push(invalid(
                    &idx_path.child("hostPort"),
                    BadValue::Int(host_port.into()),
                    "must be between 1 and 65535",
                ));
            }
        }
    }

    all_errs
}

/// Accumulates unique host ports across containers and checks for conflicts.
pub fn accumulate_unique_host_ports(containers: &[&[ContainerPort]], path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let mut host_ports = HashSet::new();

    for (ci, container_ports) in containers.iter().enumerate() {
        let idx_path = path.index(ci);
        let ports_path = idx_path.child("ports");

        for (pi, port) in container_ports.iter().enumerate() {
            let port_idx_path = ports_path.index(pi);

            let host_port = port.host_port.unwrap_or(0);
            if host_port == 0 {
                continue;
            }

            let protocol_value = protocol_to_str(&port.protocol);
            let host_ip = port.host_ip.as_str();
            let key = format!("{}/{}/{}", protocol_value, host_ip, host_port);

            if host_ports.contains(&key) {
                all_errs.push(crate::common::validation::duplicate(
                    &port_idx_path.child("hostPort"),
                    BadValue::String(key),
                ));
            } else {
                host_ports.insert(key);
            }
        }
    }

    all_errs
}

/// Checks for host port conflicts within a single container list.
pub fn check_host_port_conflicts(ports: &[ContainerPort], path: &Path) -> ErrorList {
    accumulate_unique_host_ports(&[ports], path)
}

// ============================================================================
// Helper Functions
// ============================================================================

fn is_valid_port_num(port: i32) -> bool {
    (1..=65535).contains(&port)
}

fn validate_port_name(name: &str, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if name.is_empty() {
        all_errs.push(required(path, "port name must not be empty"));
        return all_errs;
    }

    let is_valid = name.len() <= 15
        && name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        && name.chars().any(|c| c.is_ascii_lowercase())
        && !name.starts_with('-')
        && !name.ends_with('-')
        && !name.contains("--");

    if !is_valid {
        all_errs.push(invalid(
            path,
            BadValue::String(name.to_string()),
            "must be a valid port name (lowercase alphanumeric or '-', 1-15 chars, must contain at least one letter, no consecutive hyphens)",
        ));
    }

    all_errs
}
