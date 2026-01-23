//! Kubernetes Discovery API Internal Types
//!
//! This module contains type definitions from k8s-pkg/apis/discovery/types.go
//! that are used internally by the Kubernetes API.
//!
//! Source: k8s.io/kubernetes/pkg/apis/discovery

use crate::common::{ListMeta, ObjectMeta, TypeMeta};
use crate::core::internal::{ObjectReference, Protocol};
use crate::impl_has_object_meta;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// ============================================================================
// AddressType
// ============================================================================

/// AddressType represents the type of address referred to by an endpoint.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum AddressType {
    /// IPv4 represents an IPv4 Address.
    #[serde(rename = "IPv4")]
    #[default]
    IPv4,
    /// IPv6 represents an IPv6 Address.
    #[serde(rename = "IPv6")]
    IPv6,
    /// FQDN represents a Fully Qualified Domain Name.
    #[serde(rename = "FQDN")]
    FQDN,
}

pub mod address_type {
    pub const IPV4: &str = "IPv4";
    pub const IPV6: &str = "IPv6";
    pub const FQDN: &str = "FQDN";
}

// ============================================================================
// EndpointSlice
// ============================================================================

/// EndpointSlice represents a subset of the endpoints that implement a service.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EndpointSlice {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    pub metadata: ObjectMeta,
    /// addressType specifies the type of address carried by this EndpointSlice.
    pub address_type: AddressType,
    /// endpoints is a list of unique endpoints in this slice.
    #[serde(default)]
    pub endpoints: Vec<Endpoint>,
    /// ports specifies the list of network ports exposed by each endpoint.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub ports: Vec<EndpointPort>,
}
impl_has_object_meta!(EndpointSlice);

impl Default for EndpointSlice {
    fn default() -> Self {
        Self {
            type_meta: TypeMeta::default(),
            metadata: ObjectMeta::default(),
            address_type: AddressType::IPv4,
            endpoints: Vec::new(),
            ports: Vec::new(),
        }
    }
}

/// EndpointSliceList represents a list of endpoint slices.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct EndpointSliceList {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,
    /// List of endpoint slices.
    #[serde(default)]
    pub items: Vec<EndpointSlice>,
}

// ============================================================================
// Endpoint
// ============================================================================

/// Endpoint represents a single logical "backend" implementing a service.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Endpoint {
    /// addresses of this endpoint.
    #[serde(default)]
    pub addresses: Vec<String>,
    /// conditions contains information about the current status of the endpoint.
    #[serde(default)]
    pub conditions: EndpointConditions,
    /// hostname of this endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// targetRef is a reference to a Kubernetes object that represents this endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_ref: Option<ObjectReference>,
    /// deprecatedTopology is deprecated and only retained for round-trip compatibility.
    #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
    pub deprecated_topology: BTreeMap<String, String>,
    /// nodeName represents the name of the Node hosting this endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,
    /// zone is the name of the Zone this endpoint exists in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
    /// hints contains information associated with how an endpoint should be consumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hints: Option<EndpointHints>,
}

/// EndpointConditions represents the current condition of an endpoint.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct EndpointConditions {
    /// ready indicates that this endpoint is prepared to receive traffic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready: Option<bool>,
    /// serving is identical to ready except that it is set regardless of the terminating state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serving: Option<bool>,
    /// terminating indicates that this endpoint is terminating.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminating: Option<bool>,
}

/// EndpointHints provides hints describing how an endpoint should be consumed.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct EndpointHints {
    /// forZones indicates the zone(s) this endpoint should be consumed by.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub for_zones: Vec<ForZone>,
    /// forNodes indicates the node(s) this endpoint should be consumed by.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub for_nodes: Vec<ForNode>,
}

/// ForZone provides information about which zones should consume this endpoint.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct ForZone {
    /// name represents the name of the zone.
    pub name: String,
}

/// ForNode provides information about which nodes should consume this endpoint.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct ForNode {
    /// name represents the name of the node.
    pub name: String,
}

// ============================================================================
// EndpointPort
// ============================================================================

/// EndpointPort represents a Port used by an EndpointSlice.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct EndpointPort {
    /// The name of this port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The IP protocol for this port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<Protocol>,
    /// The port number of the endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// The application protocol for this port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_protocol: Option<String>,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {}
