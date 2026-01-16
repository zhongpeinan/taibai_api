//! Helper types from the Kubernetes Core API
//!
//! This module contains various helper types used across Kubernetes resources,
//! including actions, references, and options.
//!
//! Source: k8s-pkg/apis/core/types.go

use crate::common::TypeMeta;
use serde::{Deserialize, Serialize};

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

/// Preconditions must be fulfilled before an operation is updated.
///
/// Corresponds to [Kubernetes Preconditions](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3989)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Preconditions {
    /// Specifies the target ResourceVersion.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_version: Option<String>,
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
/// Corresponds to [Kubernetes RangeAllocation](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4660)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RangeAllocation {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::common::ObjectMeta>,
    /// Range is string that identifies the range represented by this allocation.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub range: String,
    /// Data is a byte array representing the serialized state of this range.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<u8>,
}

// ============================================================================
// Pod Options
// ============================================================================

/// PodLogOptions is the query options for a Pod's log.
///
/// Corresponds to [Kubernetes PodLogOptions](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4403)
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
    /// If set, the number of lines from the end of the logs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tail_lines: Option<i64>,
    /// If set, the number of lines from the beginning of the logs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit_bytes: Option<i64>,
    /// If true, success is indicated by HTTP status 200-204.
    #[serde(default)]
    pub insecure_skip_tls_verify_backend: bool,
    /// Restricts the logs to return only those lines that match the regular expression.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub filter: String,
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
mod tests {
    use super::*;

    // GRPCAction tests
    #[test]
    fn test_grpc_action_default() {
        let action = GRPCAction::default();
        assert_eq!(action.port, 0);
        assert!(action.service.is_empty());
    }

    #[test]
    fn test_grpc_action_with_fields() {
        let action = GRPCAction {
            port: 8080,
            service: "health-check".to_string(),
        };
        assert_eq!(action.port, 8080);
        assert_eq!(action.service, "health-check");
    }

    #[test]
    fn test_grpc_action_serialize() {
        let action = GRPCAction {
            port: 9090,
            ..Default::default()
        };
        let json = serde_json::to_string(&action).unwrap();
        assert!(json.contains(r#""port":9090"#));
    }

    // TypedLocalObjectReference tests
    #[test]
    fn test_typed_local_object_reference_default() {
        let reference = TypedLocalObjectReference::default();
        assert!(reference.kind.is_empty());
        assert!(reference.name.is_empty());
    }

    #[test]
    fn test_typed_local_object_reference_with_fields() {
        let reference = TypedLocalObjectReference {
            api_group: Some("apps".to_string()),
            kind: "Deployment".to_string(),
            name: "my-deployment".to_string(),
            namespace: Some("default".to_string()),
        };
        assert_eq!(reference.api_group, Some("apps".to_string()));
        assert_eq!(reference.kind, "Deployment");
        assert_eq!(reference.name, "my-deployment");
        assert_eq!(reference.namespace, Some("default".to_string()));
    }

    #[test]
    fn test_typed_local_object_reference_serialize() {
        let reference = TypedLocalObjectReference {
            kind: "ConfigMap".to_string(),
            name: "my-config".to_string(),
            ..Default::default()
        };
        let json = serde_json::to_string(&reference).unwrap();
        assert!(json.contains(r#""kind":"ConfigMap""#));
        assert!(json.contains(r#""name":"my-config""#));
    }

    #[test]
    fn test_typed_local_object_reference_round_trip() {
        let original = TypedLocalObjectReference {
            api_group: Some("".to_string()),
            kind: "Secret".to_string(),
            name: "my-secret".to_string(),
            ..Default::default()
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: TypedLocalObjectReference = serde_json::from_str(&json).unwrap();
        assert_eq!(original.kind, deserialized.kind);
        assert_eq!(original.name, deserialized.name);
    }

    // Preconditions tests
    #[test]
    fn test_preconditions_default() {
        let preconditions = Preconditions::default();
        assert!(preconditions.resource_version.is_none());
        assert!(preconditions.uid.is_none());
    }

    #[test]
    fn test_preconditions_with_fields() {
        let preconditions = Preconditions {
            resource_version: Some("12345".to_string()),
            uid: Some("abc-123".to_string()),
        };
        assert_eq!(preconditions.resource_version, Some("12345".to_string()));
        assert_eq!(preconditions.uid, Some("abc-123".to_string()));
    }

    #[test]
    fn test_preconditions_serialize() {
        let preconditions = Preconditions {
            resource_version: Some("100".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_string(&preconditions).unwrap();
        assert!(json.contains(r#""resourceVersion":"100""#));
    }

    // SerializedReference tests
    #[test]
    fn test_serialized_reference_default() {
        let reference = SerializedReference::default();
        assert!(reference.reference.is_none());
    }

    #[test]
    fn test_serialized_reference_with_reference() {
        let reference = SerializedReference {
            reference: Some(crate::core::internal::ObjectReference {
                kind: "Pod".to_string(),
                name: "my-pod".to_string(),
                ..Default::default()
            }),
        };
        assert!(reference.reference.is_some());
        assert_eq!(reference.reference.as_ref().unwrap().kind, "Pod");
    }

    #[test]
    fn test_serialized_reference_serialize() {
        let reference = SerializedReference {
            reference: Some(crate::core::internal::ObjectReference {
                kind: "Service".to_string(),
                name: "my-service".to_string(),
                ..Default::default()
            }),
        };
        let json = serde_json::to_string(&reference).unwrap();
        assert!(json.contains(r#""reference""#));
    }

    // RangeAllocation tests
    #[test]
    fn test_range_allocation_default() {
        let allocation = RangeAllocation::default();
        assert!(allocation.range.is_empty());
        assert!(allocation.data.is_empty());
    }

    #[test]
    fn test_range_allocation_with_fields() {
        let allocation = RangeAllocation {
            range: "10.0.0.1-10.0.0.100".to_string(),
            data: vec![0x01, 0x02, 0x03],
            ..Default::default()
        };
        assert_eq!(allocation.range, "10.0.0.1-10.0.0.100");
        assert_eq!(allocation.data.len(), 3);
    }

    #[test]
    fn test_range_allocation_serialize() {
        let allocation = RangeAllocation {
            range: "192.168.1.1-192.168.1.255".to_string(),
            ..Default::default()
        };
        let json = serde_json::to_string(&allocation).unwrap();
        assert!(json.contains(r#""range":"192.168.1.1-192.168.1.255""#));
    }

    // PodLogOptions tests
    #[test]
    fn test_pod_log_options_default() {
        let options = PodLogOptions::default();
        assert!(!options.follow);
        assert!(!options.previous);
    }

    #[test]
    fn test_pod_log_options_with_fields() {
        let options = PodLogOptions {
            container: "my-container".to_string(),
            follow: true,
            tail_lines: Some(100),
            ..Default::default()
        };
        assert_eq!(options.container, "my-container");
        assert!(options.follow);
        assert_eq!(options.tail_lines, Some(100));
    }

    #[test]
    fn test_pod_log_options_serialize() {
        let options = PodLogOptions {
            follow: true,
            previous: true,
            ..Default::default()
        };
        let json = serde_json::to_string(&options).unwrap();
        assert!(json.contains(r#""follow":true"#));
        assert!(json.contains(r#""previous":true"#));
    }

    // PodAttachOptions tests
    #[test]
    fn test_pod_attach_options_default() {
        let options = PodAttachOptions::default();
        assert!(!options.stdin);
        assert!(!options.stdout);
        assert!(!options.stderr);
        assert!(!options.tty);
    }

    #[test]
    fn test_pod_attach_options_with_tty() {
        let options = PodAttachOptions {
            stdin: true,
            stdout: true,
            stderr: true,
            tty: true,
            container: "my-container".to_string(),
        };
        assert!(options.tty);
        assert!(options.stdin);
    }

    #[test]
    fn test_pod_attach_options_serialize() {
        let options = PodAttachOptions {
            tty: true,
            ..Default::default()
        };
        let json = serde_json::to_string(&options).unwrap();
        assert!(json.contains(r#""tty":true"#));
    }

    // PodExecOptions tests
    #[test]
    fn test_pod_exec_options_default() {
        let options = PodExecOptions::default();
        assert!(!options.stdin);
        assert!(options.command.is_empty());
    }

    #[test]
    fn test_pod_exec_options_with_command() {
        let options = PodExecOptions {
            command: vec!["/bin/sh".to_string(), "-c".to_string(), "ls".to_string()],
            ..Default::default()
        };
        assert_eq!(options.command.len(), 3);
        assert_eq!(options.command[0], "/bin/sh");
    }

    #[test]
    fn test_pod_exec_options_serialize() {
        let options = PodExecOptions {
            container: "my-container".to_string(),
            ..Default::default()
        };
        let json = serde_json::to_string(&options).unwrap();
        assert!(json.contains(r#""container":"my-container""#));
    }

    // PodPortForwardOptions tests
    #[test]
    fn test_pod_port_forward_options_default() {
        let options = PodPortForwardOptions::default();
        assert!(options.ports.is_empty());
    }

    #[test]
    fn test_pod_port_forward_options_with_ports() {
        let options = PodPortForwardOptions {
            ports: vec![8080, 9090],
        };
        assert_eq!(options.ports.len(), 2);
        assert_eq!(options.ports[0], 8080);
    }

    #[test]
    fn test_pod_port_forward_options_serialize() {
        let options = PodPortForwardOptions {
            ports: vec![80],
            ..Default::default()
        };
        let json = serde_json::to_string(&options).unwrap();
        assert!(json.contains(r#""ports":[80]"#));
    }

    // PodProxyOptions tests
    #[test]
    fn test_pod_proxy_options_default() {
        let options = PodProxyOptions::default();
        assert!(options.path.is_empty());
    }

    #[test]
    fn test_pod_proxy_options_with_path() {
        let options = PodProxyOptions {
            path: "/api/v1/namespaces/default/pods/my-pod/".to_string(),
        };
        assert!(options.path.contains("/api/v1"));
    }

    #[test]
    fn test_pod_proxy_options_serialize() {
        let options = PodProxyOptions {
            path: "/metrics".to_string(),
        };
        let json = serde_json::to_string(&options).unwrap();
        assert!(json.contains(r#""path":"/metrics""#));
    }

    // NodeProxyOptions tests
    #[test]
    fn test_node_proxy_options_default() {
        let options = NodeProxyOptions::default();
        assert!(options.path.is_empty());
    }

    #[test]
    fn test_node_proxy_options_serialize() {
        let options = NodeProxyOptions {
            path: "/healthz".to_string(),
        };
        let json = serde_json::to_string(&options).unwrap();
        assert!(json.contains(r#""path":"/healthz""#));
    }

    // ServiceProxyOptions tests
    #[test]
    fn test_service_proxy_options_default() {
        let options = ServiceProxyOptions::default();
        assert!(options.path.is_empty());
    }

    #[test]
    fn test_service_proxy_options_serialize() {
        let options = ServiceProxyOptions {
            path: "/api/v1/services".to_string(),
        };
        let json = serde_json::to_string(&options).unwrap();
        assert!(json.contains(r#""path":"/api/v1/services""#));
    }
}
