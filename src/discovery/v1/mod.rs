//! Kubernetes Discovery v1 API types
//!
//! This module contains types from the Kubernetes discovery.k8s.io/v1 API group.

use crate::common::{
    ApplyDefaults, HasTypeMeta, ListMeta, ObjectMeta, ResourceSchema, TypeMeta,
    UnimplementedConversion, VersionedObject,
};
use crate::core::v1::ObjectReference;
use crate::impl_unimplemented_prost_message;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::OnceLock;

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

impl ApplyDefaults for EndpointSlice {
    fn apply_defaults(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "discovery.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "EndpointSlice".to_string();
        }
    }
}

impl ApplyDefaults for EndpointSliceList {
    fn apply_defaults(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "discovery.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "EndpointSliceList".to_string();
        }
    }
}

// ----------------------------------------------------------------------------
// Version Conversion Placeholder (using UnimplementedConversion)
// ----------------------------------------------------------------------------

impl UnimplementedConversion for EndpointSlice {}

// ----------------------------------------------------------------------------
// Protobuf Placeholder (using macro)
// ----------------------------------------------------------------------------

impl_unimplemented_prost_message!(EndpointSlice);
impl_unimplemented_prost_message!(EndpointSliceList);

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // AddressType tests
    #[test]
    fn test_address_type_serialize() {
        assert_eq!(
            serde_json::to_string(&AddressType::IPv4).unwrap(),
            r#""IPv4""#
        );
    }

    #[test]
    fn test_address_type_deserialize() {
        assert_eq!(
            serde_json::from_str::<AddressType>(r#""IPv4""#).unwrap(),
            AddressType::IPv4
        );
    }

    #[test]
    fn test_address_type_ipv6() {
        assert_eq!(
            serde_json::to_string(&AddressType::IPv6).unwrap(),
            r#""IPv6""#
        );
    }

    #[test]
    fn test_address_type_fqdn() {
        assert_eq!(
            serde_json::to_string(&AddressType::FQDN).unwrap(),
            r#""FQDN""#
        );
    }

    #[test]
    fn test_address_type_constants() {
        assert_eq!(address_type::IPV4, "IPv4");
        assert_eq!(address_type::IPV6, "IPv6");
        assert_eq!(address_type::FQDN, "FQDN");
    }

    // EndpointSlice tests
    #[test]
    fn test_endpoint_slice_default() {
        let es = EndpointSlice::default();
        assert!(es.metadata.is_none());
        assert!(es.endpoints.is_empty());
        assert!(es.ports.is_empty());
    }

    #[test]
    fn test_endpoint_slice_round_trip() {
        let original = EndpointSlice {
            type_meta: TypeMeta {
                api_version: "discovery.k8s.io/v1".to_string(),
                kind: "EndpointSlice".to_string(),
            },
            metadata: None,
            address_type: AddressType::IPv4,
            endpoints: vec![],
            ports: vec![],
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: EndpointSlice = serde_json::from_str(&json).unwrap();
        assert_eq!(
            original.type_meta.api_version,
            deserialized.type_meta.api_version
        );
        assert_eq!(original.type_meta.kind, deserialized.type_meta.kind);
    }

    #[test]
    fn test_endpoint_slice_with_endpoints() {
        let es = EndpointSlice {
            type_meta: TypeMeta::default(),
            metadata: None,
            address_type: AddressType::IPv6,
            endpoints: vec![Endpoint {
                addresses: vec!["2001:db8::1".to_string()],
                ..Default::default()
            }],
            ports: vec![],
        };
        let json = serde_json::to_string(&es).unwrap();
        assert!(json.contains("IPv6"));
    }

    #[test]
    fn test_endpoint_slice_serialization_format() {
        let es = EndpointSlice {
            type_meta: TypeMeta::default(),
            metadata: None,
            address_type: AddressType::IPv4,
            endpoints: vec![Endpoint {
                addresses: vec!["10.0.0.1".to_string()],
                ..Default::default()
            }],
            ports: vec![EndpointPort {
                name: Some("http".to_string()),
                port: Some(8080),
                app_protocol: Some("HTTP".to_string()),
                ..Default::default()
            }],
        };
        let json = serde_json::to_string(&es).unwrap();
        // Verify camelCase serialization
        assert!(json.contains("addressType"));
        assert!(json.contains("appProtocol"));
    }

    // EndpointSliceList tests
    #[test]
    fn test_endpoint_slice_list_default() {
        let list = EndpointSliceList::default();
        assert!(list.metadata.is_none());
        assert!(list.items.is_empty());
    }

    #[test]
    fn test_endpoint_slice_list_with_items() {
        let list = EndpointSliceList {
            type_meta: TypeMeta::default(),
            metadata: None,
            items: vec![EndpointSlice::default()],
        };
        let json = serde_json::to_string(&list).unwrap();
        assert!(json.contains("items"));
    }

    // Endpoint tests
    #[test]
    fn test_endpoint_default() {
        let endpoint = Endpoint::default();
        assert!(endpoint.addresses.is_empty());
        assert!(endpoint.hostname.is_none());
        assert!(endpoint.target_ref.is_none());
    }

    #[test]
    fn test_endpoint_with_all_fields() {
        let endpoint = Endpoint {
            addresses: vec!["10.0.0.1".to_string()],
            conditions: EndpointConditions {
                ready: Some(true),
                serving: Some(true),
                terminating: Some(false),
            },
            hostname: Some("pod-1".to_string()),
            node_name: Some("node-1".to_string()),
            zone: Some("zone-1".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&endpoint).unwrap();
        assert!(json.contains("10.0.0.1"));
        assert!(json.contains("pod-1"));
    }

    #[test]
    fn test_endpoint_serialization_omits_empty() {
        let endpoint = Endpoint {
            addresses: vec!["10.0.0.1".to_string()],
            ..Default::default()
        };
        let json = serde_json::to_string(&endpoint).unwrap();
        // Empty optional fields should be omitted
        assert!(!json.contains("hostname"));
        assert!(!json.contains("nodeName"));
        assert!(!json.contains("zone"));
    }

    // EndpointConditions tests
    #[test]
    fn test_endpoint_conditions_default() {
        let conditions = EndpointConditions::default();
        assert!(conditions.ready.is_none());
        assert!(conditions.serving.is_none());
        assert!(conditions.terminating.is_none());
    }

    #[test]
    fn test_endpoint_conditions_with_values() {
        let conditions = EndpointConditions {
            ready: Some(true),
            serving: Some(true),
            terminating: Some(false),
        };

        let json = serde_json::to_string(&conditions).unwrap();
        assert!(json.contains("ready"));
        assert!(json.contains("serving"));
        assert!(json.contains("terminating"));
    }

    // EndpointHints tests
    #[test]
    fn test_endpoint_hints_default() {
        let hints = EndpointHints::default();
        assert!(hints.for_zones.is_empty());
        assert!(hints.for_nodes.is_empty());
    }

    #[test]
    fn test_endpoint_hints_with_zones() {
        let hints = EndpointHints {
            for_zones: vec![ForZone {
                name: "zone-a".to_string(),
            }],
            ..Default::default()
        };

        let json = serde_json::to_string(&hints).unwrap();
        assert!(json.contains("zone-a"));
    }

    #[test]
    fn test_endpoint_hints_with_nodes() {
        let hints = EndpointHints {
            for_nodes: vec![ForNode {
                name: "node-1".to_string(),
            }],
            ..Default::default()
        };

        let json = serde_json::to_string(&hints).unwrap();
        assert!(json.contains("node-1"));
    }

    // ForZone and ForNode tests
    #[test]
    fn test_for_zone() {
        let zone = ForZone {
            name: "zone-1".to_string(),
        };
        let json = serde_json::to_string(&zone).unwrap();
        assert!(json.contains("zone-1"));
    }

    #[test]
    fn test_for_node() {
        let node = ForNode {
            name: "node-1".to_string(),
        };
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("node-1"));
    }

    // EndpointPort tests
    #[test]
    fn test_endpoint_port_default() {
        let port = EndpointPort::default();
        assert!(port.name.is_none());
        assert!(port.protocol.is_none());
        assert!(port.port.is_none());
    }

    #[test]
    fn test_endpoint_port_with_fields() {
        let port = EndpointPort {
            name: Some("http".to_string()),
            protocol: Some("TCP".to_string()),
            port: Some(8080),
            app_protocol: Some("HTTP".to_string()),
        };

        let json = serde_json::to_string(&port).unwrap();
        assert!(json.contains("http"));
        assert!(json.contains("8080"));
        assert!(json.contains("TCP"));
    }

    #[test]
    fn test_endpoint_port_serialization_omits_empty() {
        let port = EndpointPort {
            name: Some("http".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_string(&port).unwrap();
        assert!(json.contains("name"));
        assert!(!json.contains("protocol"));
        assert!(!json.contains("port"));
    }

    // Protocol constants tests
    #[test]
    fn test_protocol_constants() {
        assert_eq!(protocol::TCP, "TCP");
        assert_eq!(protocol::UDP, "UDP");
        assert_eq!(protocol::SCTP, "SCTP");
    }

    // Integration tests
    #[test]
    fn test_full_endpoint_slice() {
        let slice = EndpointSlice {
            type_meta: TypeMeta {
                api_version: "discovery.k8s.io/v1".to_string(),
                kind: "EndpointSlice".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("my-service".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            address_type: AddressType::IPv4,
            endpoints: vec![Endpoint {
                addresses: vec!["10.0.0.1".to_string(), "10.0.0.2".to_string()],
                conditions: EndpointConditions {
                    ready: Some(true),
                    ..Default::default()
                },
                hostname: Some("pod-1".to_string()),
                zone: Some("us-west-1a".to_string()),
                ..Default::default()
            }],
            ports: vec![EndpointPort {
                name: Some("https".to_string()),
                protocol: Some("TCP".to_string()),
                port: Some(443),
                app_protocol: Some("HTTPS".to_string()),
            }],
        };

        let json = serde_json::to_string(&slice).unwrap();
        assert!(json.contains("my-service"));
        assert!(json.contains("10.0.0.1"));
        assert!(json.contains("443"));
    }

    #[test]
    fn test_endpoint_slice_list_round_trip() {
        let original = EndpointSliceList {
            type_meta: TypeMeta {
                api_version: "discovery.k8s.io/v1".to_string(),
                kind: "EndpointSliceList".to_string(),
            },
            metadata: None,
            items: vec![EndpointSlice {
                type_meta: TypeMeta {
                    api_version: "discovery.k8s.io/v1".to_string(),
                    kind: "EndpointSlice".to_string(),
                },
                ..Default::default()
            }],
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: EndpointSliceList = serde_json::from_str(&json).unwrap();
        assert_eq!(original.items.len(), deserialized.items.len());
    }
}
