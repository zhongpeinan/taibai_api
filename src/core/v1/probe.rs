//! Kubernetes Probe types
//!
//! This module contains probe-related types from the Kubernetes core/v1 API.

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
pub struct HTTPHeader {
    /// The header field name.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// The header field value
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub value: String,
}

impl Default for HTTPHeader {
    fn default() -> Self {
        Self {
            name: String::new(),
            value: String::new(),
        }
    }
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
pub struct GRPCAction {
    /// Port number of the gRPC service.
    #[serde(default)]
    pub port: i32,
    /// Service is the name of the service to place in the gRPC HealthCheckRequest.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
}

impl Default for GRPCAction {
    fn default() -> Self {
        Self {
            port: 0,
            service: None,
        }
    }
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
pub struct SleepAction {
    /// Seconds is the number of seconds to sleep.
    #[serde(default)]
    pub seconds: i64,
}

impl Default for SleepAction {
    fn default() -> Self {
        Self { seconds: 0 }
    }
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

impl Default for Probe {
    fn default() -> Self {
        Self {
            probe_handler: ProbeHandler::default(),
            initial_delay_seconds: None,
            timeout_seconds: None,
            period_seconds: None,
            success_threshold: None,
            failure_threshold: None,
            termination_grace_period_seconds: None,
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_header() {
        let header = HTTPHeader {
            name: "Content-Type".to_string(),
            value: "application/json".to_string(),
        };

        let json = serde_json::to_string(&header).unwrap();
        let deserialized: HTTPHeader = serde_json::from_str(&json).unwrap();

        assert_eq!(header.name, deserialized.name);
        assert_eq!(header.value, deserialized.value);
    }

    #[test]
    fn test_http_get_action() {
        let action = HTTPGetAction {
            path: "/healthz".to_string(),
            port: IntOrString::String("http".to_string()),
            host: String::new(),
            scheme: Some(uri_scheme::HTTP.to_string()),
            http_headers: vec![],
        };

        let json = serde_json::to_string(&action).unwrap();
        let deserialized: HTTPGetAction = serde_json::from_str(&json).unwrap();

        assert_eq!(action.path, deserialized.path);
        assert_eq!(action.scheme, deserialized.scheme);
    }

    #[test]
    fn test_tcp_socket_action() {
        let action = TCPSocketAction {
            port: IntOrString::Int(8080),
            host: String::new(),
        };

        let json = serde_json::to_string(&action).unwrap();
        let deserialized: TCPSocketAction = serde_json::from_str(&json).unwrap();

        assert_eq!(action.port, deserialized.port);
    }

    #[test]
    fn test_grpc_action() {
        let action = GRPCAction {
            port: 8080,
            service: Some("health-check".to_string()),
        };

        let json = serde_json::to_string(&action).unwrap();
        let deserialized: GRPCAction = serde_json::from_str(&json).unwrap();

        assert_eq!(action.port, deserialized.port);
        assert_eq!(action.service, deserialized.service);
    }

    #[test]
    fn test_exec_action() {
        let action = ExecAction {
            command: vec!["cat".to_string(), "/tmp/healthy".to_string()],
        };

        let json = serde_json::to_string(&action).unwrap();
        let deserialized: ExecAction = serde_json::from_str(&json).unwrap();

        assert_eq!(action.command.len(), deserialized.command.len());
    }

    #[test]
    fn test_sleep_action() {
        let action = SleepAction { seconds: 10 };

        let json = serde_json::to_string(&action).unwrap();
        let deserialized: SleepAction = serde_json::from_str(&json).unwrap();

        assert_eq!(action.seconds, deserialized.seconds);
    }

    #[test]
    fn test_probe_handler() {
        let handler = ProbeHandler {
            exec: Some(ExecAction {
                command: vec!["echo".to_string(), "ok".to_string()],
            }),
            ..Default::default()
        };

        let json = serde_json::to_string(&handler).unwrap();
        let deserialized: ProbeHandler = serde_json::from_str(&json).unwrap();

        assert!(deserialized.exec.is_some());
    }

    #[test]
    fn test_probe_with_http_get() {
        let probe = Probe {
            probe_handler: ProbeHandler {
                http_get: Some(HTTPGetAction {
                    path: "/healthz".to_string(),
                    port: IntOrString::Int(8080),
                    host: String::new(),
                    scheme: Some(uri_scheme::HTTP.to_string()),
                    http_headers: vec![],
                }),
                ..Default::default()
            },
            initial_delay_seconds: Some(15),
            period_seconds: Some(10),
            ..Default::default()
        };

        let json = serde_json::to_string(&probe).unwrap();
        let deserialized: Probe = serde_json::from_str(&json).unwrap();

        assert_eq!(
            probe.initial_delay_seconds,
            deserialized.initial_delay_seconds
        );
        assert!(deserialized.probe_handler.http_get.is_some());
    }

    #[test]
    fn test_probe_with_grpc() {
        let probe = Probe {
            probe_handler: ProbeHandler {
                grpc: Some(GRPCAction {
                    port: 8080,
                    service: None,
                }),
                ..Default::default()
            },
            timeout_seconds: Some(5),
            ..Default::default()
        };

        let json = serde_json::to_string(&probe).unwrap();
        let deserialized: Probe = serde_json::from_str(&json).unwrap();

        assert_eq!(probe.timeout_seconds, deserialized.timeout_seconds);
        assert!(deserialized.probe_handler.grpc.is_some());
    }

    #[test]
    fn test_lifecycle_handler() {
        let handler = LifecycleHandler {
            exec: Some(ExecAction {
                command: vec!["sh".to_string(), "-c".to_string(), "echo hello".to_string()],
            }),
            ..Default::default()
        };

        let json = serde_json::to_string(&handler).unwrap();
        let deserialized: LifecycleHandler = serde_json::from_str(&json).unwrap();

        assert!(deserialized.exec.is_some());
    }

    #[test]
    fn test_lifecycle() {
        let lifecycle = Lifecycle {
            post_start: Some(LifecycleHandler {
                http_get: Some(HTTPGetAction {
                    path: "/startup".to_string(),
                    port: IntOrString::Int(8080),
                    host: String::new(),
                    scheme: None,
                    http_headers: vec![],
                }),
                ..Default::default()
            }),
            pre_stop: Some(LifecycleHandler {
                exec: Some(ExecAction {
                    command: vec!["echo".to_string(), "bye".to_string()],
                }),
                ..Default::default()
            }),
        };

        let json = serde_json::to_string(&lifecycle).unwrap();
        let _deserialized: Lifecycle = serde_json::from_str(&json).unwrap();

        assert!(lifecycle.post_start.is_some());
        assert!(lifecycle.pre_stop.is_some());
    }

    #[test]
    fn test_uri_scheme_constants() {
        assert_eq!(uri_scheme::HTTP, "HTTP");
        assert_eq!(uri_scheme::HTTPS, "HTTPS");
    }

    #[test]
    fn test_probe_with_tcp_socket() {
        let probe = Probe {
            probe_handler: ProbeHandler {
                tcp_socket: Some(TCPSocketAction {
                    port: IntOrString::String("health".to_string()),
                    host: String::new(),
                }),
                ..Default::default()
            },
            failure_threshold: Some(3),
            ..Default::default()
        };

        let json = serde_json::to_string(&probe).unwrap();
        let deserialized: Probe = serde_json::from_str(&json).unwrap();

        assert_eq!(probe.failure_threshold, deserialized.failure_threshold);
        assert!(deserialized.probe_handler.tcp_socket.is_some());
    }

    #[test]
    fn test_lifecycle_handler_with_sleep() {
        let handler = LifecycleHandler {
            sleep: Some(SleepAction { seconds: 5 }),
            ..Default::default()
        };

        let json = serde_json::to_string(&handler).unwrap();
        let deserialized: LifecycleHandler = serde_json::from_str(&json).unwrap();

        assert!(deserialized.sleep.is_some());
        assert_eq!(
            handler.sleep.unwrap().seconds,
            deserialized.sleep.unwrap().seconds
        );
    }

    #[test]
    fn test_probe_round_trip() {
        let probe = Probe {
            probe_handler: ProbeHandler {
                http_get: Some(HTTPGetAction {
                    path: "/health".to_string(),
                    port: IntOrString::Int(8080),
                    host: String::new(),
                    scheme: Some(uri_scheme::HTTPS.to_string()),
                    http_headers: vec![HTTPHeader {
                        name: "Authorization".to_string(),
                        value: "Bearer token".to_string(),
                    }],
                }),
                ..Default::default()
            },
            initial_delay_seconds: Some(10),
            timeout_seconds: Some(3),
            period_seconds: Some(30),
            success_threshold: Some(1),
            failure_threshold: Some(5),
            termination_grace_period_seconds: Some(60),
        };

        let json = serde_json::to_string(&probe).unwrap();
        let deserialized: Probe = serde_json::from_str(&json).unwrap();

        assert_eq!(probe, deserialized);
    }
}
