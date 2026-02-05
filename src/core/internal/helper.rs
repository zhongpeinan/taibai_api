//! Helper types from the Kubernetes Core API
//!
//! This module contains various helper types used across Kubernetes resources,
//! including actions, references, and options.
//!
//! Source: k8s-pkg/apis/core/types.go

use crate::common::TypeMeta;
use crate::{impl_has_object_meta, impl_has_type_meta};
use base64::Engine;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// ============================================================================
// ByteString (base64 ↔ bytes)
// ============================================================================

/// Semantic type for base64 string ↔ bytes conversion.
///
/// This type represents binary data that should be serialized as base64 strings
/// in JSON/YAML format, matching Kubernetes `[]byte` and OpenAPI `format: byte`.
///
/// - **Serialization**: `Vec<u8>` → base64 string (e.g., `vec![1,2,3]` → `"AQID"`)
/// - **Deserialization**: base64 string → `Vec<u8>` (e.g., `"AQID"` → `vec![1,2,3]`)
///
/// Corresponds to Kubernetes `[]byte` and [OpenAPI byte format](https://spec.openapis.org/oas/v3.1.0#data-types)
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ByteString(pub Vec<u8>);

impl From<Vec<u8>> for ByteString {
    fn from(v: Vec<u8>) -> Self {
        Self(v)
    }
}

impl From<ByteString> for Vec<u8> {
    fn from(b: ByteString) -> Self {
        b.0
    }
}

impl AsRef<[u8]> for ByteString {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Serialize for ByteString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let encoded = base64::engine::general_purpose::STANDARD.encode(&self.0);
        serializer.serialize_str(&encoded)
    }
}

impl<'de> Deserialize<'de> for ByteString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(s.as_bytes())
            .map_err(serde::de::Error::custom)?;
        Ok(ByteString(bytes))
    }
}

// ============================================================================
// Actions
// ============================================================================

/// GRPCAction describes an action based on an HTTP request.
///
/// Corresponds to [Kubernetes GRPCAction](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3875)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GRPCAction {
    /// Port number of the gRPC service.
    #[serde(default)]
    pub port: i32,
    /// Service is the name of the service to place in the gRPC HealthCheckRequest.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub service: String,
}

// ============================================================================
// Typed References
// ============================================================================

/// TypedLocalObjectReference contains enough information to let you locate the typed referenced object.
///
/// Corresponds to [Kubernetes TypedLocalObjectReference](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5528)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TypedLocalObjectReference {
    /// APIGroup is the group for the resource being referenced.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_group: Option<String>,
    /// Kind is the type of resource being referenced.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub kind: String,
    /// Name is the name of resource being referenced.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// Namespace is the namespace of resource being referenced.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

// ============================================================================
// Preconditions
// ============================================================================

/// Preconditions must be fulfilled before an operation is carried out.
///
/// Corresponds to [Kubernetes Preconditions](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/core/types.go)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Preconditions {
    /// Specifies the target UID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

// ============================================================================
// Serialized Reference
// ============================================================================

/// SerializedReference is a reference to serialized resource.
///
/// Corresponds to [Kubernetes SerializedReference](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5575)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SerializedReference {
    /// The reference to an object in the system.
    #[serde(default)]
    pub reference: Option<super::ObjectReference>,
}

// ============================================================================
// Range Allocation
// ============================================================================

/// RangeAllocation is a range of allocated resources.
///
/// Corresponds to [Kubernetes RangeAllocation](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/core/types.go)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RangeAllocation {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object metadata.
    pub metadata: crate::common::ObjectMeta,
    /// Range is string that identifies the range represented by this allocation.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub range: String,
    /// Data is a byte array representing the serialized state of this range.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<ByteString>,
}

// ----------------------------------------------------------------------------
// Trait Implementations for RangeAllocation
// ----------------------------------------------------------------------------

impl_has_type_meta!(RangeAllocation);
impl_has_object_meta!(RangeAllocation);

// ============================================================================
// Pod Options
// ============================================================================

/// PodLogOptions is the query options for a Pod's log.
///
/// Corresponds to [Kubernetes PodLogOptions](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/core/types.go)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodLogOptions {
    /// The container for which to stream logs.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub container: String,
    /// Follow the log stream of the pod.
    #[serde(default)]
    pub follow: bool,
    /// Return previous terminated container logs.
    #[serde(default)]
    pub previous: bool,
    /// A relative time in seconds before the current time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub since_seconds: Option<i64>,
    /// An RFC3339 timestamp from which to show logs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub since_time: Option<String>,
    /// If true, add an RFC3339 timestamp at the beginning of every line of log output.
    #[serde(default)]
    pub timestamps: bool,
    /// If set, the number of lines from the end of the logs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tail_lines: Option<i64>,
    /// If set, the number of bytes to read from the server before terminating the log output.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit_bytes: Option<i64>,
    /// If true, the apiserver will skip verifying the backend TLS certificate.
    #[serde(default)]
    pub insecure_skip_tls_verify_backend: bool,
    /// Specify which container log stream to return. Acceptable values are "All", "Stdout" and "Stderr".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream: Option<String>,
}

/// PodAttachOptions is the query options to a Pod's remote attach call.
///
/// Corresponds to [Kubernetes PodAttachOptions](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4428)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodAttachOptions {
    /// Stdin if true indicates that stdin is to be redirected to the container.
    #[serde(default)]
    pub stdin: bool,
    /// Stdout if true indicates that stdout is to be redirected from the container.
    #[serde(default)]
    pub stdout: bool,
    /// Stderr if true indicates that stderr is to be redirected from the container.
    #[serde(default)]
    pub stderr: bool,
    /// TTY if true indicates that a TTY will be allocated to the container.
    #[serde(default)]
    pub tty: bool,
    /// The container in which to execute the command.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub container: String,
}

/// PodExecOptions is the query options to a Pod's remote exec call.
///
/// Corresponds to [Kubernetes PodExecOptions](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4455)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodExecOptions {
    /// Stdin if true indicates that stdin is to be redirected to the container.
    #[serde(default)]
    pub stdin: bool,
    /// Stdout if true indicates that stdout is to be redirected from the container.
    #[serde(default)]
    pub stdout: bool,
    /// Stderr if true indicates that stderr is to be redirected from the container.
    #[serde(default)]
    pub stderr: bool,
    /// TTY if true indicates that a TTY will be allocated to the container.
    #[serde(default)]
    pub tty: bool,
    /// The container in which to execute the command.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub container: String,
    /// Command is the remote command to execute.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub command: Vec<String>,
}

/// PodPortForwardOptions is the query options to a Pod's port forward call.
///
/// Corresponds to [Kubernetes PodPortForwardOptions](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4482)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodPortForwardOptions {
    /// List of ports to forward
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<i32>,
}

/// PodProxyOptions is the query options to a Pod's proxy call.
///
/// Corresponds to [Kubernetes PodProxyOptions](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4491)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodProxyOptions {
    /// Path is the URL path to use for the proxy.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub path: String,
}

// ============================================================================
// Node and Service Proxy Options
// ============================================================================

/// NodeProxyOptions is the query options to a Node's proxy call.
///
/// Corresponds to [Kubernetes NodeProxyOptions](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4500)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NodeProxyOptions {
    /// Path is the URL path to use for the proxy.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub path: String,
}

/// ServiceProxyOptions is the query options to a Service's proxy call.
///
/// Corresponds to [Kubernetes ServiceProxyOptions](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4509)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceProxyOptions {
    /// Path is the URL path to use for the proxy.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub path: String,
}

#[cfg(test)]
mod tests {}
