//! Kubernetes Probe types
//!
//! This module contains probe-related types from the Kubernetes core/v1 API.

use crate::common::ApplyDefault;
use crate::common::util::IntOrString;
use serde::{Deserialize, Serialize};

/// URIScheme identifies the scheme used for connection to a host for Get actions
pub type URIScheme = String;

/// Constants for URIScheme
pub mod uri_scheme {
    pub const HTTP: &str = "HTTP";
    pub const HTTPS: &str = "HTTPS";
}

/// HTTPHeader describes a custom header to be used in HTTP probes
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct HTTPHeader {
    /// The header field name.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// The header field value
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub value: String,
}

/// HTTPGetAction describes an action based on HTTP Get requests.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct HTTPGetAction {
    /// Path to access on the HTTP server.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub path: String,
    /// Name or number of the port to access on the container.
    pub port: IntOrString,
    /// Host name to connect to, defaults to the pod IP.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub host: String,
    /// Scheme to use for connecting to the host.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
    /// Custom headers to set in the request.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub http_headers: Vec<HTTPHeader>,
}

impl Default for HTTPGetAction {
    fn default() -> Self {
        Self {
            path: String::new(),
            port: IntOrString::Int(0),
            host: String::new(),
            scheme: None,
            http_headers: Vec::new(),
        }
    }
}

/// TCPSocketAction describes an action based on opening a socket
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TCPSocketAction {
    /// Number or name of the port to access on the container.
    pub port: IntOrString,
    /// Optional: Host name to connect to, defaults to the pod IP.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub host: String,
}

impl Default for TCPSocketAction {
    fn default() -> Self {
        Self {
            port: IntOrString::Int(0),
            host: String::new(),
        }
    }
}

/// GRPCAction specifies an action involving a GRPC service.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct GRPCAction {
    /// Port number of the gRPC service.
    #[serde(default)]
    pub port: i32,
    /// Service is the name of the service to place in the gRPC HealthCheckRequest.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
}

/// ExecAction describes a "run in container" action.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExecAction {
    /// Command is the command line to execute inside the container.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub command: Vec<String>,
}

/// SleepAction describes a "sleep" action.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct SleepAction {
    /// Seconds is the number of seconds to sleep.
    #[serde(default)]
    pub seconds: i64,
}

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
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct Probe {
    /// The action taken to determine the health of a container
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

/// LifecycleHandler defines a specific action that should be taken in a lifecycle hook.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct LifecycleHandler {
    /// Exec specifies a command to execute in the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exec: Option<ExecAction>,
    /// HTTPGet specifies an HTTP GET request to perform.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http_get: Option<HTTPGetAction>,
    /// Deprecated. TCPSocket is NOT supported as a LifecycleHandler.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcp_socket: Option<TCPSocketAction>,
    /// Sleep represents a duration that the container should sleep.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sleep: Option<SleepAction>,
}

/// Lifecycle describes actions that the management system should take in response to container lifecycle events.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Lifecycle {
    /// PostStart is called immediately after a container is created.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_start: Option<LifecycleHandler>,
    /// PreStop is called immediately before a container is terminated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pre_stop: Option<LifecycleHandler>,
}

// ----------------------------------------------------------------------------
// ApplyDefaults Implementation
// ----------------------------------------------------------------------------

impl ApplyDefault for Probe {
    fn apply_default(&mut self) {
        set_default_if_zero(&mut self.timeout_seconds, 1);
        set_default_if_zero(&mut self.period_seconds, 10);
        set_default_if_zero(&mut self.success_threshold, 1);
        set_default_if_zero(&mut self.failure_threshold, 3);

        // Apply defaults to HTTPGetAction if present
        if let Some(ref mut http_get) = self.probe_handler.http_get {
            http_get.apply_default();
        }
    }
}

impl ApplyDefault for HTTPGetAction {
    fn apply_default(&mut self) {
        // Set default path to "/" if empty
        if self.path.is_empty() {
            self.path = "/".to_string();
        }

        // Set default scheme to "HTTP" when empty or unspecified
        if self.scheme.as_deref().unwrap_or_default().is_empty() {
            self.scheme = Some(uri_scheme::HTTP.to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{HTTPGetAction, Probe};
    use crate::common::ApplyDefault;
    use crate::common::util::IntOrString;

    #[test]
    fn probe_defaults_when_none() {
        let mut probe = Probe::default();
        probe.apply_default();
        assert_eq!(probe.timeout_seconds, Some(1));
        assert_eq!(probe.period_seconds, Some(10));
        assert_eq!(probe.success_threshold, Some(1));
        assert_eq!(probe.failure_threshold, Some(3));
    }

    #[test]
    fn probe_defaults_when_zero() {
        let mut probe = Probe::default();
        probe.timeout_seconds = Some(0);
        probe.period_seconds = Some(0);
        probe.success_threshold = Some(0);
        probe.failure_threshold = Some(0);
        probe.apply_default();
        assert_eq!(probe.timeout_seconds, Some(1));
        assert_eq!(probe.period_seconds, Some(10));
        assert_eq!(probe.success_threshold, Some(1));
        assert_eq!(probe.failure_threshold, Some(3));
    }

    #[test]
    fn probe_preserves_non_zero_values() {
        let mut probe = Probe::default();
        probe.timeout_seconds = Some(5);
        probe.period_seconds = Some(7);
        probe.success_threshold = Some(2);
        probe.failure_threshold = Some(4);
        probe.apply_default();
        assert_eq!(probe.timeout_seconds, Some(5));
        assert_eq!(probe.period_seconds, Some(7));
        assert_eq!(probe.success_threshold, Some(2));
        assert_eq!(probe.failure_threshold, Some(4));
    }

    #[test]
    fn http_get_defaults_scheme() {
        let mut http_get = HTTPGetAction {
            port: IntOrString::Int(80),
            ..HTTPGetAction::default()
        };
        http_get.apply_default();
        assert_eq!(http_get.scheme.as_deref(), Some("HTTP"));

        let mut http_get = HTTPGetAction {
            port: IntOrString::Int(80),
            scheme: Some(String::new()),
            ..HTTPGetAction::default()
        };
        http_get.apply_default();
        assert_eq!(http_get.scheme.as_deref(), Some("HTTP"));

        let mut http_get = HTTPGetAction {
            port: IntOrString::Int(80),
            scheme: Some("HTTPS".to_string()),
            ..HTTPGetAction::default()
        };
        http_get.apply_default();
        assert_eq!(http_get.scheme.as_deref(), Some("HTTPS"));
    }
}

fn set_default_if_zero(target: &mut Option<i32>, default_value: i32) {
    if target.is_none() || target == &Some(0) {
        *target = Some(default_value);
    }
}
