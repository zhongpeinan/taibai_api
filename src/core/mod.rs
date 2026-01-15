//! Kubernetes Core API types
//!
//! This module contains types from the Kubernetes core API group.

pub mod internal;
pub mod v1;

pub use v1::{
    Container, ContainerPort, ContainerState, ContainerStatus, Pod, PodCondition, PodDNSConfig,
    PodList, PodSpec, PodStatus,
};
