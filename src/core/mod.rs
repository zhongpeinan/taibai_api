//! Kubernetes Core API types
//!
//! This module contains types from the Kubernetes core API group.

pub mod v1;

pub use v1::{Pod, PodSpec, PodStatus};
