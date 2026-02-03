//!
//! This module contains probe-related types from the Kubernetes core internal API.

use crate::core::internal::{ExecAction, GRPCAction, HTTPGetAction, TCPSocketAction};
use serde::{Deserialize, Serialize};

/// ProbeHandler defines a specific action that should be taken in a probe.
/// One and only one of the fields must be specified.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProbeHandler {
    /// Exec specifies a command to execute in the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exec: Option<ExecAction>,
    /// HTTPGet specifies an HTTP GET request to perform.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http_get: Option<HTTPGetAction>,
    /// TCPSocket specifies a connection to a TCP port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcp_socket: Option<TCPSocketAction>,
    /// GRPC specifies a GRPC HealthCheckRequest.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grpc: Option<GRPCAction>,
}

/// Probe describes a health check to be performed against a container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Probe {
    /// The action taken to determine the health of a container.
    #[serde(flatten)]
    pub probe_handler: ProbeHandler,
    /// Number of seconds after the container has started before liveness probes are initiated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initial_delay_seconds: Option<i32>,
    /// Number of seconds after which the probe times out.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i32>,
    /// How often (in seconds) to perform the probe.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period_seconds: Option<i32>,
    /// Minimum consecutive successes for the probe to be considered successful.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub success_threshold: Option<i32>,
    /// Minimum consecutive failures for the probe to be considered failed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_threshold: Option<i32>,
    /// Optional duration in seconds the pod needs to terminate gracefully upon probe failure.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub termination_grace_period_seconds: Option<i64>,
}
