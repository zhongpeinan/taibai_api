//! Helper and subresource types from Kubernetes Core v1 API
//!
//! This module contains helper types and subresource options used across
//! the Kubernetes API, such as pod options, preconditions, and serialized references.

use serde::{Deserialize, Serialize};

use crate::common::{ApplyDefault, ObjectMeta, Timestamp, TypeMeta};

// ============================================================================
// ByteString
// ============================================================================

/// ByteString represents a string that may contain non-UTF8 data.
///
/// Corresponds to [Kubernetes ByteString](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L52)
pub type ByteString = Vec<u8>;

// ============================================================================
// Pod Subresource Options
// ============================================================================

/// PodLogOptions is the query options to a Pod's log call.
///
/// Corresponds to [Kubernetes PodLogOptions](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7498)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodLogOptions {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// The container for which to stream logs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,

    /// Follow the log stream of the pod.
    #[serde(default)]
    pub follow: bool,

    /// If set, the number of bytes to read from the server before terminating the log output.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit_bytes: Option<i64>,

    /// If set, the number of lines from the end of the logs to show.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tail_lines: Option<i64>,

    /// If true, print the logs for the previous instance of the container in this pod.
    #[serde(default)]
    pub previous: bool,

    /// A relative time in seconds before the current time from which to show logs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub since_seconds: Option<i64>,

    /// An RFC3339 timestamp from which to show logs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub since_time: Option<Timestamp>,

    /// If true, then the output is followed by timestamp information.
    #[serde(default)]
    pub timestamps: bool,

    /// If true, the apiserver will skip verifying the backend TLS certificate.
    #[serde(default)]
    pub insecure_skip_tls_verify_backend: bool,

    /// Specify which container log stream to return. Defaults to "All".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream: Option<String>,
}

impl ApplyDefault for PodLogOptions {
    fn apply_default(&mut self) {
        if self.stream.is_none() {
            self.stream = Some("All".to_string());
        }
    }
}

/// PodAttachOptions is the query options to a Pod's remote attach call.
///
/// Corresponds to [Kubernetes PodAttachOptions](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7536)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodAttachOptions {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Stdin if true indicates that stdin is to be redirected to the container.
    #[serde(default)]
    pub stdin: bool,

    /// Stdout if true indicates that stdout is to be redirected from the container.
    #[serde(default)]
    pub stdout: bool,

    /// Stderr if true indicates that stderr is to be redirected from the container.
    #[serde(default)]
    pub stderr: bool,

    /// TTY if true indicates that a TTY will be allocated for the attach call.
    #[serde(default)]
    pub tty: bool,

    /// The container in which to execute the command.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
}

/// PodExecOptions is the query options to a Pod's remote exec call.
///
/// Corresponds to [Kubernetes PodExecOptions](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7562)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodExecOptions {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Stdin if true indicates that stdin is to be redirected to the container.
    #[serde(default)]
    pub stdin: bool,

    /// Stdout if true indicates that stdout is to be redirected from the container.
    #[serde(default)]
    pub stdout: bool,

    /// Stderr if true indicates that stderr is to be redirected from the container.
    #[serde(default)]
    pub stderr: bool,

    /// TTY if true indicates that a TTY will be allocated for the exec call.
    #[serde(default)]
    pub tty: bool,

    /// The container in which to execute the command.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,

    /// Command to execute.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub command: Vec<String>,
}

/// PodPortForwardOptions is the query options to a Pod's port forward call.
///
/// Corresponds to [Kubernetes PodPortForwardOptions](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7589)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodPortForwardOptions {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// List of ports to forward
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<i32>,
}

/// PodProxyOptions is the query options to a Pod's proxy call.
///
/// Corresponds to [Kubernetes PodProxyOptions](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7597)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodProxyOptions {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Path is the URL path to use for the proxy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

/// ServiceProxyOptions is the query options to a Service's proxy call.
///
/// Corresponds to [Kubernetes ServiceProxyOptions](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7605)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceProxyOptions {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Path is the part of URLs that include service endpoints.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

/// NodeProxyOptions is the query options to a Node's proxy call.
///
/// Corresponds to [Kubernetes NodeProxyOptions](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7613)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NodeProxyOptions {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Path is the URL path to use for the proxy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

// ============================================================================
// Preconditions
// ============================================================================

/// Preconditions must be fulfilled before an operation is updated.
///
/// Corresponds to [Kubernetes Preconditions](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7621)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Preconditions {
    /// Specifies the target UID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

// ============================================================================
// RangeAllocation
// ============================================================================

/// RangeAllocation is a range of allocatable resources.
///
/// Corresponds to [Kubernetes RangeAllocation](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7631)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RangeAllocation {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// A string representing a range.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub range: String,

    /// The bitmap data for the range.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<u8>,
}

// ============================================================================
// SerializedReference
// ============================================================================

/// SerializedReference is a reference to an object that has been serialized.
///
/// Corresponds to [Kubernetes SerializedReference](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7641)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SerializedReference {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// The reference to the object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<super::reference::ObjectReference>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pod_log_options_defaults_stream() {
        let mut opts = PodLogOptions::default();
        opts.apply_default();
        assert_eq!(opts.stream.as_deref(), Some("All"));
    }
}
