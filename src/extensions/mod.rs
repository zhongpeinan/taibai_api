//! Kubernetes Extensions API types
//!
//! This module provides Rust representations of Kubernetes Extensions API types.
//!
//! **DEPRECATION NOTICE**: The `extensions` API group is deprecated in Kubernetes.
//! Most types have been moved to other API groups:
//! - `apps/v1beta2` and `apps/v1` for Deployment, DaemonSet, ReplicaSet
//! - `networking.k8s.io/v1beta1` and `networking.k8s.io/v1` for Ingress, NetworkPolicy
//!
//! ## Migration Guide
//!
//! When writing new code, prefer the modern API groups:
//! - Use `crate::apps::v1::Deployment` instead of `extensions::v1beta1::Deployment`
//! - Use `crate::apps::v1::DaemonSet` instead of `extensions::v1beta1::DaemonSet`
//! - Use `crate::apps::v1::ReplicaSet` instead of `extensions::v1beta1::ReplicaSet`
//! - Use `crate::networking::v1::Ingress` instead of `extensions::v1beta1::Ingress` (when available)
//!
//! This module is provided for backward compatibility when working with
//! older Kubernetes clusters or existing manifests that use the extensions API.
//!
//! The module is organized into:
//! - `v1beta1`: The v1beta1 version of the Extensions API (all types are deprecated)
//! - `internal`: Internal types used within Kubernetes

pub mod internal;
pub mod v1beta1;

// Re-export commonly used v1beta1 types
pub use v1beta1::{
    DaemonSet, DaemonSetList, DaemonSetSpec, DaemonSetStatus, Deployment, DeploymentList,
    DeploymentSpec, DeploymentStatus, Ingress, IngressList, IngressSpec, IngressStatus,
    NetworkPolicy, NetworkPolicyList, NetworkPolicySpec, ReplicaSet, ReplicaSetList,
    ReplicaSetSpec, ReplicaSetStatus, Scale, ScaleSpec, ScaleStatus,
};
