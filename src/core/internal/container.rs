//! Container-related types from the Kubernetes Core API
//!
//! This module contains types for container ports, states, and execution actions.

use crate::common::time::Timestamp;
use crate::core::internal::{HTTPGetAction, Protocol, Signal, TCPSocketAction};
use serde::{Deserialize, Serialize};

/// ContainerPort represents a network port in a single container.
///
/// Corresponds to [Kubernetes ContainerPort](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2114)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContainerPort {
    /// Optional: If specified, this must be an IANA_SVC_NAME and unique within a pod.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// Optional: Host port number that should be exposed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_port: Option<i32>,
    /// Required: Container port number.
    pub container_port: i32,
    /// Required: Protocol for the port.
    #[serde(default)]
    pub protocol: Protocol,
    /// Optional: Host IP to bind the external port to.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub host_ip: String,
}

/// ContainerState holds the current state of a single container.
///
/// Corresponds to [Kubernetes ContainerState](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2929)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContainerState {
    /// Details about a waiting container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub waiting: Option<ContainerStateWaiting>,
    /// Details about a running container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub running: Option<ContainerStateRunning>,
    /// Details about a terminated container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminated: Option<ContainerStateTerminated>,
}

/// ContainerStateWaiting holds details about a container in waiting state.
///
/// Corresponds to [Kubernetes ContainerStateWaiting](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2894)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContainerStateWaiting {
    /// Optional: Brief reason for the waiting state.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    /// Optional: Human-readable message indicating details about waiting state.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
}

/// ContainerStateRunning holds details about a running container.
///
/// Corresponds to [Kubernetes ContainerStateRunning](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2907)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContainerStateRunning {
    /// Optional: Time at which the container was started.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<Timestamp>,
}

/// ContainerStateTerminated holds details about a terminated container.
///
/// Corresponds to [Kubernetes ContainerStateRunning](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2914)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContainerStateTerminated {
    /// Optional: Exit code from the last termination.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    /// Optional: Signal that caused the termination.
    #[serde(default)]
    pub signal: i32,
    /// Optional: Brief reason for the termination.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    /// Optional: Message regarding the termination.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
    /// Optional: Time at which previous execution started.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<Timestamp>,
    /// Optional: Time at which the container terminated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<Timestamp>,
}

/// ExecAction describes a command to be executed in the container.
///
/// Corresponds to [Kubernetes ExecAction](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2434)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExecAction {
    /// Command is the command line to execute inside the container.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub command: Vec<String>,
}

/// LifecycleHandler defines a specific action that should be taken in a lifecycle hook.
///
/// Corresponds to [Kubernetes LifecycleHandler](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2748)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct LifecycleHandler {
    /// Exec specifies the action to take.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exec: Option<ExecAction>,
    /// HTTPGet specifies the http request to perform.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http_get: Option<HTTPGetAction>,
    /// TCPSocket is NOT supported as a LifecycleHandler and kept
    /// for backward compatibility.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcp_socket: Option<TCPSocketAction>,
    /// Sleep represents the duration that the container should sleep before being terminated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sleep: Option<i64>,
}

/// Lifecycle describes actions that the management system should take in response
/// to container lifecycle events.
///
/// Corresponds to [Kubernetes Lifecycle](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2851)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Lifecycle {
    /// PostStart is called immediately after a container is created.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_start: Option<LifecycleHandler>,
    /// PreStop is called immediately before a container is terminated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pre_stop: Option<LifecycleHandler>,
    /// StopSignal defines which signal will be sent to a container when it is being stopped.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop_signal: Option<Signal>,
}

/// ContainerStatus contains details for the current status of this container.
///
/// Corresponds to [Kubernetes ContainerStatus](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2939)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContainerStatus {
    /// Name is a DNS_LABEL representing the unique name of the container.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// State holds details about the container's current condition.
    #[serde(default)]
    pub state: ContainerState,
    /// LastTerminationState holds the last termination state of the container.
    #[serde(default)]
    pub last_termination_state: ContainerState,
    /// Ready specifies whether the container is currently passing its readiness check.
    #[serde(default)]
    pub ready: bool,
    /// RestartCount holds the number of times the container has been restarted.
    #[serde(default)]
    pub restart_count: i32,
    /// Image is the name of container image that the container is running.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub image: String,
    /// ImageID is the image ID of the container's image.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub image_id: String,
    /// ContainerID is the ID of the container in the format '<type>://<container_id>'.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub container_id: String,
    /// Started indicates whether the container has finished its postStart lifecycle hook.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_port_default() {
        let port = ContainerPort::default();
        assert!(port.name.is_empty());
        assert!(port.host_port.is_none());
        assert_eq!(port.container_port, 0);
        assert_eq!(port.protocol, Protocol::Tcp);
        assert!(port.host_ip.is_empty());
    }

    #[test]
    fn test_container_port_with_fields() {
        let port = ContainerPort {
            name: "http".to_string(),
            host_port: Some(8080),
            container_port: 80,
            protocol: Protocol::Tcp,
            host_ip: "0.0.0.0".to_string(),
        };

        assert_eq!(port.name, "http");
        assert_eq!(port.host_port, Some(8080));
        assert_eq!(port.container_port, 80);
    }

    #[test]
    fn test_container_port_serialize() {
        let port = ContainerPort {
            name: "https".to_string(),
            host_port: None,
            container_port: 443,
            protocol: Protocol::Tcp,
            host_ip: String::new(),
        };

        let json = serde_json::to_string(&port).unwrap();
        assert!(json.contains("\"name\":\"https\""));
        assert!(json.contains("\"containerPort\":443"));
        assert!(json.contains("\"protocol\":\"TCP\""));
        // None/empty fields should be omitted
        assert!(!json.contains("\"hostPort\""));
        assert!(!json.contains("\"hostIP\""));
    }

    #[test]
    fn test_container_port_deserialize() {
        let json = r#"{"name":"web","hostPort":8080,"containerPort":80,"protocol":"UDP"}"#;
        let port: ContainerPort = serde_json::from_str(json).unwrap();

        assert_eq!(port.name, "web");
        assert_eq!(port.host_port, Some(8080));
        assert_eq!(port.container_port, 80);
        assert_eq!(port.protocol, Protocol::Udp);
    }

    #[test]
    fn test_container_port_round_trip() {
        let original = ContainerPort {
            name: "metrics".to_string(),
            host_port: Some(9090),
            container_port: 9090,
            protocol: Protocol::Tcp,
            host_ip: "127.0.0.1".to_string(),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: ContainerPort = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_container_state_default() {
        let state = ContainerState::default();
        assert!(state.waiting.is_none());
        assert!(state.running.is_none());
        assert!(state.terminated.is_none());
    }

    #[test]
    fn test_container_state_waiting() {
        let waiting = ContainerStateWaiting {
            reason: "ImagePullBackOff".to_string(),
            message: "Failed to pull image".to_string(),
        };

        let state = ContainerState {
            waiting: Some(waiting),
            running: None,
            terminated: None,
        };

        assert!(state.waiting.is_some());
        assert_eq!(state.waiting.as_ref().unwrap().reason, "ImagePullBackOff");
    }

    #[test]
    fn test_container_state_serialize() {
        let state = ContainerState {
            waiting: None,
            running: Some(ContainerStateRunning {
                started_at: Some(Timestamp::from_str("2024-01-15T10:00:00Z").unwrap()),
            }),
            terminated: None,
        };

        let json = serde_json::to_string(&state).unwrap();
        assert!(json.contains("\"running\""));
        assert!(json.contains("\"startedAt\""));
        // None states should be omitted
        assert!(!json.contains("\"waiting\""));
        assert!(!json.contains("\"terminated\""));
    }

    #[test]
    fn test_container_state_waiting_serialize() {
        let waiting = ContainerStateWaiting {
            reason: "CrashLoopBackOff".to_string(),
            message: "Container crashed repeatedly".to_string(),
        };

        let json = serde_json::to_string(&waiting).unwrap();
        assert!(json.contains("\"reason\":\"CrashLoopBackOff\""));
        assert!(json.contains("\"message\":\"Container crashed repeatedly\""));
    }

    #[test]
    fn test_container_state_running_serialize() {
        let running = ContainerStateRunning {
            started_at: Some(Timestamp::from_str("2024-01-15T10:00:00Z").unwrap()),
        };

        let json = serde_json::to_string(&running).unwrap();
        assert!(json.contains("\"startedAt\":\"2024-01-15T10:00:00"));
    }

    #[test]
    fn test_container_state_terminated_serialize() {
        let terminated = ContainerStateTerminated {
            exit_code: Some(137),
            signal: 9,
            reason: "OOMKilled".to_string(),
            message: "Container was killed due to OOM".to_string(),
            started_at: Some(Timestamp::from_str("2024-01-15T09:00:00Z").unwrap()),
            finished_at: Some(Timestamp::from_str("2024-01-15T10:00:00Z").unwrap()),
        };

        let json = serde_json::to_string(&terminated).unwrap();
        assert!(json.contains("\"exitCode\":137"));
        assert!(json.contains("\"signal\":9"));
        assert!(json.contains("\"reason\":\"OOMKilled\""));
    }

    #[test]
    fn test_container_state_round_trip() {
        let original = ContainerState {
            waiting: Some(ContainerStateWaiting {
                reason: "ContainerCreating".to_string(),
                message: "Creating container".to_string(),
            }),
            running: None,
            terminated: None,
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: ContainerState = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_exec_action_default() {
        let action = ExecAction::default();
        assert!(action.command.is_empty());
    }

    #[test]
    fn test_exec_action_with_command() {
        let action = ExecAction {
            command: vec![
                "/bin/sh".to_string(),
                "-c".to_string(),
                "echo hello".to_string(),
            ],
        };

        assert_eq!(action.command.len(), 3);
        assert_eq!(action.command[0], "/bin/sh");
    }

    #[test]
    fn test_exec_action_serialize() {
        let action = ExecAction {
            command: vec!["ls".to_string(), "-la".to_string()],
        };

        let json = serde_json::to_string(&action).unwrap();
        assert!(json.contains("\"command\":[\"ls\",\"-la\"]"));
    }

    #[test]
    fn test_exec_action_deserialize() {
        let json = r#"{"command":["/bin/echo","hello"]}"#;
        let action: ExecAction = serde_json::from_str(json).unwrap();

        assert_eq!(action.command.len(), 2);
        assert_eq!(action.command[0], "/bin/echo");
        assert_eq!(action.command[1], "hello");
    }

    #[test]
    fn test_exec_action_round_trip() {
        let original = ExecAction {
            command: vec![
                "python".to_string(),
                "-m".to_string(),
                "http.server".to_string(),
                "8080".to_string(),
            ],
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: ExecAction = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_protocol_serialize() {
        let json_tcp = serde_json::to_string(&Protocol::Tcp).unwrap();
        assert_eq!(json_tcp, r#""TCP""#);

        let json_udp = serde_json::to_string(&Protocol::Udp).unwrap();
        assert_eq!(json_udp, r#""UDP""#);

        let json_sctp = serde_json::to_string(&Protocol::Sctp).unwrap();
        assert_eq!(json_sctp, r#""SCTP""#);
    }

    #[test]
    fn test_protocol_deserialize() {
        let tcp: Protocol = serde_json::from_str(r#""TCP""#).unwrap();
        assert_eq!(tcp, Protocol::Tcp);

        let udp: Protocol = serde_json::from_str(r#""UDP""#).unwrap();
        assert_eq!(udp, Protocol::Udp);
    }
}
