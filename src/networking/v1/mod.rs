//! Kubernetes Networking v1 API types
//!
//! This module contains types from the Kubernetes networking.k8s.io/v1 API group.

pub mod network_policy;

pub use network_policy::{
    NetworkPolicy, NetworkPolicyList, NetworkPolicyPeer, NetworkPolicyPort,
    NetworkPolicyEgressRule, NetworkPolicyIngressRule, NetworkPolicySpec,
};
