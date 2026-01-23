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
mod tests {
}
