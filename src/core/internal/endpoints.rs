//! Endpoints types from the Kubernetes Core API
//!
//! This module contains types for Kubernetes Endpoints resources,
//! which represent the endpoints that implement a Service.
//!
//! Source: k8s-pkg/apis/core/types.go

use crate::common::{ListMeta, ObjectMeta, TypeMeta};
use crate::core::internal::Protocol;
use crate::impl_has_object_meta;
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_protocol: Option<String>,
}

#[cfg(test)]
mod tests {}
