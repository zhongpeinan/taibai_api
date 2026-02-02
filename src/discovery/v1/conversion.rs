//! Conversions between v1 and internal discovery types
//!
//! Based on k8s.io/kubernetes/pkg/apis/discovery/v1/conversion.go

use crate::common::{ApplyDefault, FromInternal, ToInternal, TypeMeta};
use crate::core::internal::Protocol;
use crate::core::v1::ObjectReference;
use crate::core::v1::conversion::{
    meta_to_option_object_meta, option_object_meta_to_meta, protocol_to_option_string,
};
use crate::discovery::internal;

use super::{
    AddressType, Endpoint, EndpointConditions, EndpointHints, EndpointPort, EndpointSlice,
    EndpointSliceList, ForNode, ForZone,
};

// ============================================================================
// Conversion Helper Functions
// ============================================================================

fn option_string_to_option_protocol(s: Option<String>) -> Option<Protocol> {
    match s.as_deref() {
        Some("TCP") => Some(Protocol::Tcp),
        Some("UDP") => Some(Protocol::Udp),
        Some("SCTP") => Some(Protocol::Sctp),
        Some(_) => Some(Protocol::default()),
        None => None,
    }
}

fn option_protocol_to_option_string(p: Option<Protocol>) -> Option<String> {
    protocol_to_option_string(p)
}

fn address_type_to_internal(value: AddressType) -> internal::AddressType {
    match value {
        AddressType::IPv4 => internal::AddressType::IPv4,
        AddressType::IPv6 => internal::AddressType::IPv6,
        AddressType::FQDN => internal::AddressType::FQDN,
    }
}

fn address_type_from_internal(value: internal::AddressType) -> AddressType {
    match value {
        internal::AddressType::IPv4 => AddressType::IPv4,
        internal::AddressType::IPv6 => AddressType::IPv6,
        internal::AddressType::FQDN => AddressType::FQDN,
    }
}

fn endpoint_conditions_to_internal(value: EndpointConditions) -> internal::EndpointConditions {
    internal::EndpointConditions {
        ready: value.ready,
        serving: value.serving,
        terminating: value.terminating,
    }
}

fn endpoint_conditions_from_internal(value: internal::EndpointConditions) -> EndpointConditions {
    EndpointConditions {
        ready: value.ready,
        serving: value.serving,
        terminating: value.terminating,
    }
}

fn endpoint_hints_to_internal(value: EndpointHints) -> internal::EndpointHints {
    internal::EndpointHints {
        for_zones: value
            .for_zones
            .into_iter()
            .map(for_zone_to_internal)
            .collect(),
        for_nodes: value
            .for_nodes
            .into_iter()
            .map(for_node_to_internal)
            .collect(),
    }
}

fn endpoint_hints_from_internal(value: internal::EndpointHints) -> EndpointHints {
    EndpointHints {
        for_zones: value
            .for_zones
            .into_iter()
            .map(for_zone_from_internal)
            .collect(),
        for_nodes: value
            .for_nodes
            .into_iter()
            .map(for_node_from_internal)
            .collect(),
    }
}

fn for_zone_to_internal(value: ForZone) -> internal::ForZone {
    internal::ForZone { name: value.name }
}

fn for_zone_from_internal(value: internal::ForZone) -> ForZone {
    ForZone { name: value.name }
}

fn for_node_to_internal(value: ForNode) -> internal::ForNode {
    internal::ForNode { name: value.name }
}

fn for_node_from_internal(value: internal::ForNode) -> ForNode {
    ForNode { name: value.name }
}

fn endpoint_port_to_internal(value: EndpointPort) -> internal::EndpointPort {
    internal::EndpointPort {
        name: value.name,
        protocol: option_string_to_option_protocol(value.protocol),
        port: value.port,
        app_protocol: value.app_protocol,
    }
}

fn endpoint_port_from_internal(value: internal::EndpointPort) -> EndpointPort {
    EndpointPort {
        name: value.name,
        protocol: option_protocol_to_option_string(value.protocol),
        port: value.port,
        app_protocol: value.app_protocol,
    }
}

fn endpoint_to_internal(value: Endpoint) -> internal::Endpoint {
    internal::Endpoint {
        addresses: value.addresses,
        conditions: endpoint_conditions_to_internal(value.conditions),
        hostname: value.hostname,
        target_ref: value.target_ref.map(ObjectReference::to_internal),
        deprecated_topology: value.deprecated_topology,
        node_name: value.node_name,
        zone: value.zone,
        hints: value.hints.map(endpoint_hints_to_internal),
    }
}

fn endpoint_from_internal(value: internal::Endpoint) -> Endpoint {
    Endpoint {
        addresses: value.addresses,
        conditions: endpoint_conditions_from_internal(value.conditions),
        hostname: value.hostname,
        target_ref: value.target_ref.map(ObjectReference::from_internal),
        deprecated_topology: value.deprecated_topology,
        node_name: value.node_name,
        zone: value.zone,
        hints: value.hints.map(endpoint_hints_from_internal),
    }
}

// ============================================================================
// EndpointSlice Conversions
// ============================================================================

impl ToInternal<internal::EndpointSlice> for EndpointSlice {
    fn to_internal(self) -> internal::EndpointSlice {
        internal::EndpointSlice {
            type_meta: TypeMeta::default(),
            metadata: option_object_meta_to_meta(self.metadata),
            address_type: address_type_to_internal(self.address_type),
            endpoints: self
                .endpoints
                .into_iter()
                .map(endpoint_to_internal)
                .collect(),
            ports: self
                .ports
                .into_iter()
                .map(endpoint_port_to_internal)
                .collect(),
        }
    }
}

impl FromInternal<internal::EndpointSlice> for EndpointSlice {
    fn from_internal(value: internal::EndpointSlice) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: meta_to_option_object_meta(value.metadata),
            address_type: address_type_from_internal(value.address_type),
            endpoints: value
                .endpoints
                .into_iter()
                .map(endpoint_from_internal)
                .collect(),
            ports: value
                .ports
                .into_iter()
                .map(endpoint_port_from_internal)
                .collect(),
        };

        result
    }
}

// ============================================================================
// EndpointSliceList Conversions
// ============================================================================

impl ToInternal<internal::EndpointSliceList> for EndpointSliceList {
    fn to_internal(self) -> internal::EndpointSliceList {
        internal::EndpointSliceList {
            type_meta: TypeMeta::default(),
            metadata: self.metadata,
            items: self
                .items
                .into_iter()
                .map(EndpointSlice::to_internal)
                .collect(),
        }
    }
}

impl FromInternal<internal::EndpointSliceList> for EndpointSliceList {
    fn from_internal(value: internal::EndpointSliceList) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: value.metadata,
            items: value
                .items
                .into_iter()
                .map(EndpointSlice::from_internal)
                .collect(),
        };

        result
    }
}
