//! Kubernetes Networking v1beta1 API types
//!
//! This module contains types from the Kubernetes networking.k8s.io/v1beta1 API group.

pub mod ip_address;
pub mod service_cidr;

pub use ip_address::{IPAddress, IPAddressList, IPAddressSpec, ParentReference};
pub use service_cidr::{
    ServiceCIDR, ServiceCIDRList, ServiceCIDRSpec, ServiceCIDRStatus, service_cidr_condition,
};

#[cfg(test)]
mod trait_tests;
