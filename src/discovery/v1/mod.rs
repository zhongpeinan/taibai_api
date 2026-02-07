//! Kubernetes Discovery v1 API types
//!
//! This module contains types from the Kubernetes discovery.k8s.io/v1 API group.

use crate::common::{
    ApplyDefault, HasTypeMeta, ListMeta, ObjectMeta, ResourceSchema, TypeMeta, VersionedObject,
};
use crate::core::v1::ObjectReference;
use crate::impl_unimplemented_prost_message;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::OnceLock;

pub mod validation;

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

/// EndpointSlice represents a set of service endpoints.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct EndpointSlice {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// addressType specifies the type of address carried by this EndpointSlice.
    #[serde(default)]
    pub address_type: AddressType,
    /// endpoints is a list of unique endpoints in this slice.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub endpoints: Vec<Endpoint>,
    /// ports specifies the list of network ports exposed by each endpoint.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub ports: Vec<EndpointPort>,
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
    /// items is the list of endpoint slices.
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
    /// deprecatedTopology contains topology information part of the v1beta1 API.
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
    /// ready indicates that this endpoint is ready to receive traffic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready: Option<bool>,
    /// serving indicates that this endpoint is able to receive traffic.
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
    /// name represents the name of this port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// protocol represents the IP protocol for this port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// port represents the port number of the endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// The application protocol for this port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_protocol: Option<String>,
}

// ============================================================================
// Constants
// ============================================================================

/// Protocol constants for endpoint ports
pub mod protocol {
    pub const TCP: &str = "TCP";
    pub const UDP: &str = "UDP";
    pub const SCTP: &str = "SCTP";
}

// ============================================================================
// Trait Implementations for EndpointSlice and EndpointSliceList
// ============================================================================

// ----------------------------------------------------------------------------
// ResourceSchema Implementation
// ----------------------------------------------------------------------------

impl ResourceSchema for EndpointSlice {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "discovery.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "EndpointSlice"
    }
    fn resource(_: &Self::Meta) -> &str {
        "endpointslices"
    }

    fn group_static() -> &'static str {
        "discovery.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "EndpointSlice"
    }
    fn resource_static() -> &'static str {
        "endpointslices"
    }
}

impl ResourceSchema for EndpointSliceList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "discovery.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "EndpointSliceList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "endpointslices"
    }

    fn group_static() -> &'static str {
        "discovery.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "EndpointSliceList"
    }
    fn resource_static() -> &'static str {
        "endpointslices"
    }
}

// ----------------------------------------------------------------------------
// HasTypeMeta Implementation
// ----------------------------------------------------------------------------

impl HasTypeMeta for EndpointSlice {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for EndpointSliceList {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

// ----------------------------------------------------------------------------
// VersionedObject Implementation
// ----------------------------------------------------------------------------

impl VersionedObject for EndpointSlice {
    fn metadata(&self) -> &ObjectMeta {
        self.metadata
            .as_ref()
            .unwrap_or_else(|| static_default_object_meta())
    }

    fn metadata_mut(&mut self) -> &mut ObjectMeta {
        self.metadata.get_or_insert_with(ObjectMeta::default)
    }
}

// Helper function for static default ObjectMeta
fn static_default_object_meta() -> &'static ObjectMeta {
    static DEFAULT: OnceLock<ObjectMeta> = OnceLock::new();
    DEFAULT.get_or_init(ObjectMeta::default)
}

// Note: EndpointSliceList does not implement VersionedObject because its metadata is ListMeta

// ----------------------------------------------------------------------------
// ApplyDefaults Implementation
// ----------------------------------------------------------------------------

impl ApplyDefault for EndpointSlice {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "discovery.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "EndpointSlice".to_string();
        }
    }
}

impl ApplyDefault for EndpointSliceList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "discovery.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "EndpointSliceList".to_string();
        }
        for item in &mut self.items {
            item.apply_default();
        }
    }
}

// ----------------------------------------------------------------------------
// Protobuf Placeholder (using macro)
// ----------------------------------------------------------------------------

impl_unimplemented_prost_message!(EndpointSlice);
impl_unimplemented_prost_message!(EndpointSliceList);

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {}

#[cfg(test)]
mod trait_tests;

#[cfg(test)]
mod serde_roundtrip_tests;

mod conversion;

#[cfg(test)]
mod conversion_roundtrip_tests;

// AsRefStr / AsRef<str> implementations for enums
crate::impl_as_str_ref!(AddressType, {
    IPv4 => address_type::IPV4,
    IPv6 => address_type::IPV6,
    FQDN => address_type::FQDN,
});
