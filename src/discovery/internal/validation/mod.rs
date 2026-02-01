//! Validation for Kubernetes Discovery API internal types
//!
//! Ported from k8s.io/kubernetes/pkg/apis/discovery/validation/validation.go

use crate::common::validation::{
    BadValue, ErrorList, Path, invalid, is_dns1123_subdomain, name_is_dns_subdomain,
    required, validate_object_meta, validate_object_meta_update,
};
use crate::common::ObjectMeta;
use crate::core::internal::{protocol, Protocol};
use crate::discovery::internal::{
    address_type, AddressType, Endpoint, EndpointHints, EndpointPort, EndpointSlice,
    EndpointSliceList, ForNode, ForZone,
};

// ============================================================================
// EndpointSlice Validation
// ============================================================================

/// Validates an EndpointSlice for creation.
pub fn validate_endpoint_slice(obj: &EndpointSlice) -> ErrorList {
    validate_endpoint_slice_with_path(obj, &Path::nil())
}

fn validate_endpoint_slice_with_path(obj: &EndpointSlice, base_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let meta = &obj.metadata;

    all_errs.extend(validate_object_meta(
        meta,
        true,
        name_is_dns_subdomain,
        &base_path.child("metadata"),
    ));

    if meta.namespace.as_deref().unwrap_or("").is_empty() {
        all_errs.push(required(
            &base_path.child("metadata").child("namespace"),
            "",
        ));
    }

    all_errs.extend(validate_address_type(&obj.address_type, &base_path.child("addressType")));

    if obj.endpoints.is_empty() {
        all_errs.push(required(&base_path.child("endpoints"), ""));
    } else {
        for (i, endpoint) in obj.endpoints.iter().enumerate() {
            all_errs.extend(validate_endpoint(
                endpoint,
                &base_path.child("endpoints").index(i),
            ));
        }
    }

    for (i, port) in obj.ports.iter().enumerate() {
        all_errs.extend(validate_endpoint_port(
            port,
            &base_path.child("ports").index(i),
        ));
    }

    all_errs
}

/// Validates an EndpointSliceList.
pub fn validate_endpoint_slice_list(obj: &EndpointSliceList) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (i, item) in obj.items.iter().enumerate() {
        all_errs.extend(validate_endpoint_slice_with_path(
            item,
            &Path::new("items").index(i),
        ));
    }

    all_errs
}

/// Validates EndpointSlice updates.
pub fn validate_endpoint_slice_update(obj: &EndpointSlice, old: &EndpointSlice) -> ErrorList {
    let mut all_errs = ErrorList::new();

    all_errs.extend(validate_object_meta_update(
        &obj.metadata,
        &old.metadata,
        &Path::new("metadata"),
    ));

    if obj.address_type != old.address_type {
        all_errs.push(invalid(
            &Path::new("addressType"),
            BadValue::String(format!("{:?}", obj.address_type)),
            "field is immutable",
        ));
    }

    all_errs
}

// ============================================================================
// AddressType Validation
// ============================================================================

fn validate_address_type(value: &AddressType, fld_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let value_str = match value {
        AddressType::IPv4 => address_type::IPV4,
        AddressType::IPv6 => address_type::IPV6,
        AddressType::FQDN => address_type::FQDN,
    };

    if value_str != address_type::IPV4
        && value_str != address_type::IPV6
        && value_str != address_type::FQDN
    {
        all_errs.push(invalid(
            fld_path,
            BadValue::String(value_str.to_string()),
            "must be one of IPv4, IPv6, FQDN",
        ));
    }

    all_errs
}

// ============================================================================
// Endpoint Validation
// ============================================================================

fn validate_endpoint(endpoint: &Endpoint, fld_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if endpoint.addresses.is_empty() {
        all_errs.push(required(&fld_path.child("addresses"), ""));
    } else {
        for (i, address) in endpoint.addresses.iter().enumerate() {
            if address.is_empty() {
                all_errs.push(required(&fld_path.child("addresses").index(i), ""));
            }
        }
    }

    if let Some(ref hostname) = endpoint.hostname {
        for msg in is_dns1123_subdomain(hostname) {
            all_errs.push(invalid(
                &fld_path.child("hostname"),
                BadValue::String(hostname.clone()),
                &msg,
            ));
        }
    }

    if let Some(ref node_name) = endpoint.node_name {
        for msg in is_dns1123_subdomain(node_name) {
            all_errs.push(invalid(
                &fld_path.child("nodeName"),
                BadValue::String(node_name.clone()),
                &msg,
            ));
        }
    }

    if let Some(ref zone) = endpoint.zone {
        for msg in is_dns1123_subdomain(zone) {
            all_errs.push(invalid(
                &fld_path.child("zone"),
                BadValue::String(zone.clone()),
                &msg,
            ));
        }
    }

    if let Some(ref hints) = endpoint.hints {
        all_errs.extend(validate_endpoint_hints(hints, &fld_path.child("hints")));
    }

    all_errs
}

fn validate_endpoint_hints(hints: &EndpointHints, fld_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (i, zone) in hints.for_zones.iter().enumerate() {
        all_errs.extend(validate_for_zone(zone, &fld_path.child("forZones").index(i)));
    }

    for (i, node) in hints.for_nodes.iter().enumerate() {
        all_errs.extend(validate_for_node(node, &fld_path.child("forNodes").index(i)));
    }

    all_errs
}

fn validate_for_zone(zone: &ForZone, fld_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if zone.name.is_empty() {
        all_errs.push(required(&fld_path.child("name"), ""));
    }

    all_errs
}

fn validate_for_node(node: &ForNode, fld_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if node.name.is_empty() {
        all_errs.push(required(&fld_path.child("name"), ""));
    }

    all_errs
}

// ============================================================================
// EndpointPort Validation
// ============================================================================

fn validate_endpoint_port(port: &EndpointPort, fld_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if let Some(ref name) = port.name {
        if name.len() > 15 {
            all_errs.push(invalid(
                &fld_path.child("name"),
                BadValue::String(name.clone()),
                "name must be no more than 15 characters",
            ));
        }
        for msg in crate::common::validation::is_dns1123_label(name) {
            all_errs.push(invalid(
                &fld_path.child("name"),
                BadValue::String(name.clone()),
                &msg,
            ));
        }
    }

    if let Some(port_value) = port.port {
        if port_value < 1 || port_value > 65535 {
            all_errs.push(invalid(
                &fld_path.child("port"),
                BadValue::Int(port_value as i64),
                "must be between 1 and 65535",
            ));
        }
    }

    if let Some(ref protocol_value) = port.protocol {
        let protocol_string = match protocol_value {
            Protocol::Tcp => protocol::TCP,
            Protocol::Udp => protocol::UDP,
            Protocol::Sctp => protocol::SCTP,
        };
        if protocol_string != protocol::TCP
            && protocol_string != protocol::UDP
            && protocol_string != protocol::SCTP
        {
            all_errs.push(invalid(
                &fld_path.child("protocol"),
                BadValue::String(protocol_string.to_string()),
                "must be TCP, UDP, or SCTP",
            ));
        }
    }

    if let Some(ref app_protocol) = port.app_protocol {
        if app_protocol.is_empty() {
            all_errs.push(required(&fld_path.child("appProtocol"), ""));
        }
    }

    all_errs
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::TypeMeta;

    fn base_endpoint_slice() -> EndpointSlice {
        EndpointSlice {
            type_meta: TypeMeta::default(),
            metadata: ObjectMeta {
                name: Some("slice-a".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            },
            address_type: AddressType::IPv4,
            endpoints: vec![Endpoint {
                addresses: vec!["10.0.0.1".to_string()],
                ..Default::default()
            }],
            ports: vec![],
        }
    }

    #[test]
    fn test_validate_endpoint_slice_valid() {
        let obj = base_endpoint_slice();
        let errs = validate_endpoint_slice(&obj);
        assert!(errs.is_empty(), "expected no errors, got {errs:?}");
    }

    #[test]
    fn test_validate_endpoint_slice_missing_namespace() {
        let mut obj = base_endpoint_slice();
        obj.metadata.namespace = None;
        let errs = validate_endpoint_slice(&obj);
        assert!(errs.errors.iter().any(|e| e.field == "metadata.namespace"));
    }

    #[test]
    fn test_validate_endpoint_slice_empty_endpoints() {
        let mut obj = base_endpoint_slice();
        obj.endpoints.clear();
        let errs = validate_endpoint_slice(&obj);
        assert!(errs.errors.iter().any(|e| e.field == "endpoints"));
    }

    #[test]
    fn test_validate_endpoint_port_invalid_port() {
        let mut obj = base_endpoint_slice();
        obj.ports.push(EndpointPort {
            port: Some(70000),
            ..Default::default()
        });
        let errs = validate_endpoint_slice(&obj);
        assert!(errs.errors.iter().any(|e| e.field.contains("ports[0].port")));
    }

    #[test]
    fn test_validate_endpoint_slice_update_immutable_address_type() {
        let mut old = base_endpoint_slice();
        let mut new = base_endpoint_slice();
        new.address_type = AddressType::FQDN;

        let errs = validate_endpoint_slice_update(&new, &old);
        assert!(errs.errors.iter().any(|e| e.field == "addressType"));
    }

    #[test]
    fn test_validate_endpoint_slice_list_item_index() {
        let mut list = EndpointSliceList {
            type_meta: TypeMeta::default(),
            metadata: None,
            items: vec![base_endpoint_slice()],
        };
        list.items[0].endpoints.clear();

        let errs = validate_endpoint_slice_list(&list);
        assert!(errs.errors.iter().any(|e| e.field.contains("items[0].endpoints")));
    }
}
