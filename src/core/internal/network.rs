//! Network-related types for Kubernetes health checks and probes
//!
//! This module contains types for network-based health checks used in liveness and readiness probes.

use crate::common::util::IntOrString;
use crate::core::internal::URIScheme;
use serde::{Deserialize, Serialize};

/// TCPSocketAction describes an action based on opening a TCP socket.
///
/// Corresponds to [Kubernetes TCPSocketAction](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2424)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TCPSocketAction {
    /// Port number or name to connect to.
    pub port: IntOrString,
    /// Optional: Host name to connect to, defaults to the pod IP.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub host: String,
}

/// HTTPGetAction describes an action based on HTTP Get requests.
///
/// Corresponds to [Kubernetes HTTPGetAction](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2394)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct HTTPGetAction {
    /// Optional: Path to access on the HTTP server.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub path: String,
    /// Port number or name to access on the container.
    pub port: IntOrString,
    /// Optional: Host name to connect to, defaults to the pod IP.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub host: String,
    /// Optional: Scheme to use for connecting to the host.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<URIScheme>,
    /// Optional: Custom headers to set in the request.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub http_headers: Vec<HTTPHeader>,
}

/// HTTPHeader describes a custom header to use in HTTP probes.
///
/// Corresponds to [Kubernetes HTTPHeader](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2385)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct HTTPHeader {
    /// The header field name.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// The header field value.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub value: String,
}

#[cfg(test)]
mod tests {
}
