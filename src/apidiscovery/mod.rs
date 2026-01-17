//! Kubernetes API Discovery API types
//!
//! This module contains types from the Kubernetes apidiscovery API group.

pub mod internal;
pub mod v1;

pub use internal::{DiscoveryFreshness, ResourceScope};
pub use v1::{
    APIGroupDiscovery, APIGroupDiscoveryList, APIResourceDiscovery, APISubresourceDiscovery,
    APIVersionDiscovery,
};
