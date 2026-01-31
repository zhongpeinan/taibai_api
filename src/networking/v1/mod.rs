//! Kubernetes Networking v1 API types
//!
//! This module contains types from the Kubernetes networking.k8s.io/v1 API group.

pub mod conversion;
pub mod defaults;
pub mod ingress;
pub mod ingress_class;
pub mod ip_address;
pub mod network_policy;
pub mod service_cidr;
pub mod validation;

pub use ingress::{Ingress, IngressList, IngressSpec, IngressStatus};
pub use ingress_class::{IngressClass, IngressClassList, IngressClassSpec};
pub use ip_address::{IPAddress, IPAddressList, IPAddressSpec, ParentReference};
pub use network_policy::{
    NetworkPolicy, NetworkPolicyEgressRule, NetworkPolicyIngressRule, NetworkPolicyList,
    NetworkPolicyPeer, NetworkPolicyPort, NetworkPolicySpec,
};
pub use service_cidr::{
    ServiceCIDR, ServiceCIDRList, ServiceCIDRSpec, ServiceCIDRStatus, service_cidr_condition,
};

#[cfg(test)]
mod trait_tests;

#[cfg(test)]
mod serde_roundtrip_tests;

#[cfg(test)]
mod conversion_roundtrip_tests;
