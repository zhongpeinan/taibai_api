//! Kubernetes Networking API Internal Types
//!
//! This module contains type definitions from k8s.io/kubernetes/pkg/apis/networking/types.go
//! that are used internally by the Kubernetes API.

pub mod network_policy;

pub use network_policy::{
    IPBlock, NetworkPolicy, NetworkPolicyEgressRule, NetworkPolicyIngressRule, NetworkPolicyPeer,
    NetworkPolicyPort, NetworkPolicySpec, PolicyType,
};

pub use network_policy::{policy_type};
