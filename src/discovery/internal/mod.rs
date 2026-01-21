//! Kubernetes Discovery API Internal Types
//!
//! This module contains type definitions from k8s-pkg/apis/discovery/types.go
//! that are used internally by the Kubernetes API.
//!
//! Source: k8s.io/kubernetes/pkg/apis/discovery

use crate::common::{ListMeta, ObjectMeta, TypeMeta};
use crate::core::internal::{ObjectReference, Protocol};
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
            metadata: ObjectMeta::default(),
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
            metadata: ObjectMeta::default(),
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

    // EndpointSliceList tests
    #[test]
    fn test_endpoint_slice_list_default() {
        let list = EndpointSliceList::default();
        assert!(list.metadata.is_none());
        assert!(list.items.is_empty());
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

    // EndpointConditions tests
    #[test]
    fn test_endpoint_conditions_default() {
        let conditions = EndpointConditions::default();
        assert!(conditions.ready.is_none());
        assert!(conditions.serving.is_none());
        assert!(conditions.terminating.is_none());
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
            protocol: Some(Protocol::Tcp),
            port: Some(8080),
            app_protocol: Some("HTTP".to_string()),
        };

        let json = serde_json::to_string(&port).unwrap();
        assert!(json.contains("http"));
        assert!(json.contains("8080"));
    }
}
