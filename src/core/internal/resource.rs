//! Resource-related types from the Kubernetes Core API
//!
//! This module contains types for managing compute resources in Kubernetes.

use crate::common::util::Quantity;
use crate::core::internal::Protocol;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// ResourceList maps a ResourceName to a Quantity.
///
/// Corresponds to [Kubernetes ResourceList](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5779)
pub type ResourceList = BTreeMap<String, Quantity>;

/// ResourceRequirements describes the compute resource requirements.
///
/// Corresponds to [Kubernetes ResourceRequirements](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2559)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceRequirements {
    /// Limits describes the maximum amount of compute resources allowed.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub limits: ResourceList,
    /// Requests describes the minimum amount of compute resources required.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub requests: ResourceList,
    /// Claims lists the names of resources, defined in spec.resourceClaims,
    /// that are used by this container.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub claims: Vec<ResourceClaim>,
}

/// ResourceClaim describes a resource claim reference by name.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceClaim {
    /// Name must match the name of one entry in pod.spec.resourceClaims of
    /// the Pod where this field is used.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// Request is the name chosen for a request in the referenced claim.
    /// If empty, everything from the claim is made available.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub request: String,
}

/// PortStatus represents the status of a service port.
///
/// Corresponds to [Kubernetes PortStatus](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7052)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PortStatus {
    /// Port is the port number of the service port.
    pub port: i32,
    /// Protocol is the protocol of the service port.
    pub protocol: Protocol,
    /// Error is to record the problem with the service port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[cfg(test)]
mod tests {}
