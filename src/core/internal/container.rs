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
}
