//! Kubernetes Core v1 API types
//!
//! This module contains types from the Kubernetes core/v1 API group.

pub mod pod;

pub use pod::{Pod, PodSpec, PodStatus};
