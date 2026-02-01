//! Container ports validation wrappers for internal core API types.

use crate::common::validation::{ErrorList, Path};
use crate::core::v1::pod::ContainerPort;
use crate::core::v1::validation::container_ports as v1_container_ports;

pub fn validate_container_ports(ports: &[ContainerPort], path: &Path) -> ErrorList {
    v1_container_ports::validate_container_ports(ports, path)
}

pub fn accumulate_unique_host_ports(containers: &[&[ContainerPort]], path: &Path) -> ErrorList {
    v1_container_ports::accumulate_unique_host_ports(containers, path)
}

pub fn check_host_port_conflicts(ports: &[ContainerPort], path: &Path) -> ErrorList {
    v1_container_ports::check_host_port_conflicts(ports, path)
}
