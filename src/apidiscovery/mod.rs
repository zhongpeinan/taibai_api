//! Kubernetes API Discovery API types
//!
//! This module contains types from the Kubernetes apidiscovery API group.

pub mod internal;
pub mod v2;
pub mod v2beta1;
pub mod validation;

pub use internal::{DiscoveryFreshness, ResourceScope};
pub use v2::{
    APIGroupDiscovery, APIGroupDiscoveryList, APIResourceDiscovery, APISubresourceDiscovery,
    APIVersionDiscovery,
};
pub use v2beta1::{
    APIGroupDiscovery as APIGroupDiscoveryV2Beta1,
    APIGroupDiscoveryList as APIGroupDiscoveryListV2Beta1,
    APIResourceDiscovery as APIResourceDiscoveryV2Beta1,
    APISubresourceDiscovery as APISubresourceDiscoveryV2Beta1,
    APIVersionDiscovery as APIVersionDiscoveryV2Beta1,
};
