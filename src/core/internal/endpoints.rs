//! Endpoints types from the Kubernetes Core API
//!
//! This module contains types for Kubernetes Endpoints resources,
//! which represent the endpoints that implement a Service.
//!
//! Source: k8s-pkg/apis/core/types.go

use crate::common::{ListMeta, ObjectMeta, TypeMeta};
use crate::impl_has_object_meta;
use crate::core::internal::Protocol;
use serde::{Deserialize, Serialize};

// ============================================================================
// Endpoints
// ============================================================================

/// Endpoints is a collection of endpoints that implement the actually served services.
///
/// Corresponds to [Kubernetes Endpoints](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3284)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Endpoints {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ObjectMeta,
    /// The set of all endpoints is the union of all subsets.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subsets: Vec<EndpointSubset>,
}
    impl_has_object_meta!(Endpoints);

/// EndpointsList is a list of Endpoints.
///
/// Corresponds to [Kubernetes EndpointsList](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3294)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct EndpointsList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    /// List of endpoints.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Endpoints>,
}

// ============================================================================
// Endpoint Subset
// ============================================================================

/// EndpointSubset is a group of addresses with a common set of ports.
///
/// Corresponds to [Kubernetes EndpointSubset](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3304)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct EndpointSubset {
    /// IP addresses which offer the related ports.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub addresses: Vec<EndpointAddress>,
    /// IP addresses which offer the related ports but are not currently marked as ready.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub not_ready_addresses: Vec<EndpointAddress>,
    /// Port numbers available on the related IP addresses.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<EndpointPort>,
}

// ============================================================================
// Endpoint Address
// ============================================================================

/// EndpointAddress is a tuple that describes single IP address.
///
/// Corresponds to [Kubernetes EndpointAddress](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3319)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct EndpointAddress {
    /// The IP of this endpoint.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub ip: String,
    /// The Hostname of this endpoint.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub hostname: String,
    /// Optional: Node hosting this endpoint.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub node_name: String,
    /// Optional: Reference to an object representing the target of this endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_ref: Option<ObjectReference>,
}

/// ObjectReference contains enough information to let you inspect or modify the referred object.
///
/// Corresponds to [Kubernetes ObjectReference](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4196)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ObjectReference {
    /// Kind of the referent.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub kind: String,
    /// Namespace of the referent.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub namespace: String,
    /// Name of the referent.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// UID of the referent.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub uid: String,
    /// API version of the referent.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub api_version: String,
    /// Specific resourceVersion to which this reference is made.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub resource_version: String,
    /// Specific field within this resource.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub field_path: String,
}

// ============================================================================
// Endpoint Port
// ============================================================================

/// EndpointPort is a tuple that describes a single port.
///
/// Corresponds to [Kubernetes EndpointPort](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3347)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct EndpointPort {
    /// The name of this port.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// The port number of the endpoint.
    #[serde(default)]
    pub port: i32,
    /// The IP protocol for this port.
    #[serde(default)]
    pub protocol: Protocol,
    /// The application protocol for this port.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub app_protocol: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    // Endpoints tests
    #[test]
    fn test_endpoints_default() {
        let endpoints = Endpoints {
            type_meta: TypeMeta::default(),
            metadata: None,
            subsets: vec![],
        };
        assert!(endpoints.subsets.is_empty());
    }

    #[test]
    fn test_endpoints_list_default() {
        let list = EndpointsList::default();
        assert!(list.items.is_empty());
    }

    #[test]
    fn test_endpoints_with_subsets() {
        let endpoints = Endpoints {
            type_meta: TypeMeta::default(),
            metadata: None,
            subsets: vec![EndpointSubset {
                addresses: vec![EndpointAddress {
                    ip: "10.0.0.1".to_string(),
                    ..Default::default()
                }],
                ..Default::default()
            }],
        };
        assert_eq!(endpoints.subsets.len(), 1);
        assert_eq!(endpoints.subsets[0].addresses.len(), 1);
    }

    #[test]
    fn test_endpoints_serialize() {
        let endpoints = Endpoints {
            type_meta: TypeMeta {
                kind: "Endpoints".to_string(),
                api_version: "v1".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("my-endpoints".to_string()),
                ..Default::default()
            }),
            subsets: vec![],
        };
        let json = serde_json::to_string(&endpoints).unwrap();
        assert!(json.contains("\"kind\":\"Endpoints\""));
        assert!(json.contains("\"name\":\"my-endpoints\""));
    }

    // EndpointSubset tests
    #[test]
    fn test_endpoint_subset_default() {
        let subset = EndpointSubset::default();
        assert!(subset.addresses.is_empty());
        assert!(subset.not_ready_addresses.is_empty());
        assert!(subset.ports.is_empty());
    }

    #[test]
    fn test_endpoint_subset_with_addresses() {
        let subset = EndpointSubset {
            addresses: vec![
                EndpointAddress {
                    ip: "192.168.1.1".to_string(),
                    ..Default::default()
                },
                EndpointAddress {
                    ip: "192.168.1.2".to_string(),
                    ..Default::default()
                },
            ],
            not_ready_addresses: vec![],
            ports: vec![EndpointPort {
                name: "http".to_string(),
                port: 80,
                ..Default::default()
            }],
        };
        assert_eq!(subset.addresses.len(), 2);
        assert_eq!(subset.ports.len(), 1);
    }

    #[test]
    fn test_endpoint_subset_serialize() {
        let subset = EndpointSubset {
            addresses: vec![EndpointAddress {
                ip: "10.0.0.1".to_string(),
                ..Default::default()
            }],
            ..Default::default()
        };
        let json = serde_json::to_string(&subset).unwrap();
        assert!(json.contains("\"addresses\""));
        assert!(json.contains("\"10.0.0.1\""));
    }

    #[test]
    fn test_endpoint_subset_round_trip() {
        let original = EndpointSubset {
            addresses: vec![EndpointAddress {
                ip: "1.2.3.4".to_string(),
                ..Default::default()
            }],
            not_ready_addresses: vec![],
            ports: vec![EndpointPort {
                port: 8080,
                ..Default::default()
            }],
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: EndpointSubset = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    // EndpointAddress tests
    #[test]
    fn test_endpoint_address_default() {
        let address = EndpointAddress::default();
        assert!(address.ip.is_empty());
        assert!(address.hostname.is_empty());
    }

    #[test]
    fn test_endpoint_address_with_fields() {
        let address = EndpointAddress {
            ip: "10.244.1.5".to_string(),
            hostname: "pod-1".to_string(),
            node_name: "node-1".to_string(),
            target_ref: Some(ObjectReference {
                kind: "Pod".to_string(),
                name: "pod-1".to_string(),
                namespace: "default".to_string(),
                ..Default::default()
            }),
        };
        assert_eq!(address.ip, "10.244.1.5");
        assert_eq!(address.hostname, "pod-1");
        assert_eq!(address.node_name, "node-1");
        assert!(address.target_ref.is_some());
    }

    #[test]
    fn test_endpoint_address_serialize() {
        let address = EndpointAddress {
            ip: "172.16.0.1".to_string(),
            ..Default::default()
        };
        let json = serde_json::to_string(&address).unwrap();
        assert!(json.contains("\"ip\":\"172.16.0.1\""));
    }

    #[test]
    fn test_endpoint_address_round_trip() {
        let original = EndpointAddress {
            ip: "192.168.0.10".to_string(),
            hostname: "web-server".to_string(),
            ..Default::default()
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: EndpointAddress = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    // ObjectReference tests
    #[test]
    fn test_object_reference_default() {
        let reference = ObjectReference::default();
        assert!(reference.kind.is_empty());
        assert!(reference.name.is_empty());
    }

    #[test]
    fn test_object_reference_with_fields() {
        let reference = ObjectReference {
            kind: "Pod".to_string(),
            namespace: "default".to_string(),
            name: "my-pod".to_string(),
            uid: "abc123".to_string(),
            api_version: "v1".to_string(),
            ..Default::default()
        };
        assert_eq!(reference.kind, "Pod");
        assert_eq!(reference.name, "my-pod");
        assert_eq!(reference.namespace, "default");
    }

    #[test]
    fn test_object_reference_serialize() {
        let reference = ObjectReference {
            kind: "Service".to_string(),
            name: "my-service".to_string(),
            ..Default::default()
        };
        let json = serde_json::to_string(&reference).unwrap();
        assert!(json.contains("\"kind\":\"Service\""));
        assert!(json.contains("\"name\":\"my-service\""));
    }

    #[test]
    fn test_object_reference_round_trip() {
        let original = ObjectReference {
            kind: "Node".to_string(),
            name: "node-1".to_string(),
            namespace: "kube-system".to_string(),
            ..Default::default()
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: ObjectReference = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    // EndpointPort tests
    #[test]
    fn test_endpoint_port_default() {
        let port = EndpointPort::default();
        assert!(port.name.is_empty());
        assert_eq!(port.port, 0);
        assert_eq!(port.protocol, Protocol::Tcp);
    }

    #[test]
    fn test_endpoint_port_with_fields() {
        let port = EndpointPort {
            name: "https".to_string(),
            port: 443,
            protocol: Protocol::Tcp,
            app_protocol: "https".to_string(),
        };
        assert_eq!(port.name, "https");
        assert_eq!(port.port, 443);
    }

    #[test]
    fn test_endpoint_port_serialize() {
        let port = EndpointPort {
            name: "metrics".to_string(),
            port: 9090,
            protocol: Protocol::Tcp,
            ..Default::default()
        };
        let json = serde_json::to_string(&port).unwrap();
        assert!(json.contains("\"name\":\"metrics\""));
        assert!(json.contains("\"port\":9090"));
        assert!(json.contains("\"protocol\":\"TCP\""));
    }

    #[test]
    fn test_endpoint_port_round_trip() {
        let original = EndpointPort {
            name: "web".to_string(),
            port: 8080,
            protocol: Protocol::Tcp,
            app_protocol: "http".to_string(),
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: EndpointPort = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    // Integration tests
    #[test]
    fn test_endpoints_with_multiple_subsets() {
        let endpoints = Endpoints {
            type_meta: TypeMeta::default(),
            metadata: None,
            subsets: vec![
                EndpointSubset {
                    addresses: vec![EndpointAddress {
                        ip: "10.0.0.1".to_string(),
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                EndpointSubset {
                    addresses: vec![EndpointAddress {
                        ip: "10.0.0.2".to_string(),
                        ..Default::default()
                    }],
                    ..Default::default()
                },
            ],
        };
        assert_eq!(endpoints.subsets.len(), 2);
    }

    #[test]
    fn test_endpoint_subset_with_not_ready_addresses() {
        let subset = EndpointSubset {
            addresses: vec![EndpointAddress {
                ip: "10.0.0.1".to_string(),
                ..Default::default()
            }],
            not_ready_addresses: vec![EndpointAddress {
                ip: "10.0.0.2".to_string(),
                ..Default::default()
            }],
            ..Default::default()
        };
        assert_eq!(subset.addresses.len(), 1);
        assert_eq!(subset.not_ready_addresses.len(), 1);
    }

    #[test]
    fn test_endpoint_subset_with_multiple_ports() {
        let subset = EndpointSubset {
            ports: vec![
                EndpointPort {
                    name: "http".to_string(),
                    port: 80,
                    ..Default::default()
                },
                EndpointPort {
                    name: "https".to_string(),
                    port: 443,
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        assert_eq!(subset.ports.len(), 2);
    }

    #[test]
    fn test_endpoints_list_with_items() {
        let list = EndpointsList {
            items: vec![
                Endpoints {
                    type_meta: TypeMeta {
                        kind: "Endpoints".to_string(),
                        api_version: "v1".to_string(),
                    },
                    ..Default::default()
                },
                Endpoints {
                    type_meta: TypeMeta {
                        kind: "Endpoints".to_string(),
                        api_version: "v1".to_string(),
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        assert_eq!(list.items.len(), 2);
    }

    #[test]
    fn test_endpoint_address_with_pod_reference() {
        let address = EndpointAddress {
            ip: "10.244.0.5".to_string(),
            target_ref: Some(ObjectReference {
                kind: "Pod".to_string(),
                namespace: "production".to_string(),
                name: "web-pod-abc123".to_string(),
                uid: "uid-123".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert!(address.target_ref.is_some());
        let target_ref = address.target_ref.unwrap();
        assert_eq!(target_ref.kind, "Pod");
        assert_eq!(target_ref.name, "web-pod-abc123");
    }
}
