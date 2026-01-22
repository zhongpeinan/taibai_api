//! Kubernetes Networking v1alpha1 API types
//!
//! This module contains types from the Kubernetes networking.k8s.io/v1alpha1 API group.

pub mod ip_address;
pub mod service_cidr;

pub use ip_address::{IPAddress, IPAddressList, IPAddressSpec};
pub use service_cidr::{ServiceCIDR, ServiceCIDRList, ServiceCIDRSpec};
