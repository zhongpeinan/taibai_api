//! Kubernetes Discovery API types
//!
//! This module contains types from the Kubernetes discovery.k8s.io API group.

pub mod internal;
pub mod v1;
pub mod validation;

// Re-export v1 types for convenience
pub use v1::{
    AddressType, Endpoint, EndpointConditions, EndpointHints, EndpointPort, EndpointSlice,
    EndpointSliceList, ForNode, ForZone,
};

// Re-export constants modules from v1
pub mod constants {
    pub use super::v1::{address_type, protocol};
}

// Re-export internal types
pub use internal::{
    AddressType as InternalAddressType, Endpoint as InternalEndpoint,
    EndpointConditions as InternalEndpointConditions, EndpointHints as InternalEndpointHints,
    EndpointPort as InternalEndpointPort, EndpointSlice as InternalEndpointSlice,
    EndpointSliceList as InternalEndpointSliceList, ForNode as InternalForNode,
    ForZone as InternalForZone, address_type as internal_address_type,
};
