//! Pod types from the Kubernetes Core v1 API
//!
//! This module contains the Pod type and its associated spec and status types.

use crate::common::ObjectMeta;
use serde::{Deserialize, Serialize};

/// Pod is a collection of containers that can run on a host.
///
/// This is a minimal implementation focusing on the basic structure.
/// The full implementation would include many more fields.
///
/// Corresponds to [Kubernetes Pod](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4300)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Pod {
    /// Standard object's metadata.
    #[serde(rename = "metadata", default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// Specification of the desired behavior of the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<PodSpec>,

    /// Most recently observed status of the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PodStatus>,
}

/// PodSpec is a description of a pod.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct PodSpec {
    /// List of containers belonging to the pod.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub containers: Vec<Container>,

    /// Restart policy for all containers within the pod.
    #[serde(
        rename = "restartPolicy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub restart_policy: Option<String>,

    /// DNS policy for containers within the pod.
    #[serde(rename = "dnsPolicy", default, skip_serializing_if = "Option::is_none")]
    pub dns_policy: Option<String>,
}

/// PodStatus represents information about the status of a pod.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct PodStatus {
    /// Current phase of the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,

    /// IP address of the host to which the pod is assigned.
    #[serde(rename = "hostIP", default, skip_serializing_if = "Option::is_none")]
    pub host_ip: Option<String>,

    /// IP address allocated to the pod.
    #[serde(rename = "podIP", default, skip_serializing_if = "Option::is_none")]
    pub pod_ip: Option<String>,
}

/// A single application container that you want to run within a pod.
///
/// This is a minimal implementation of the Container type.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Container {
    /// Name of the container specified as a DNS_LABEL.
    pub name: String,

    /// Docker image name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,

    /// List of ports to expose from the container.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<ContainerPort>,
}

/// ContainerPort represents a network port in a single container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ContainerPort {
    /// If specified, this must be an IANA_SVC_NAME and unique within the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Number of port to expose on the pod's IP address.
    #[serde(rename = "containerPort")]
    pub container_port: i32,

    /// Protocol for port. Must be UDP, TCP, or SCTP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_pod_default() {
        let pod = Pod {
            metadata: None,
            spec: None,
            status: None,
        };
        assert!(pod.metadata.is_none());
        assert!(pod.spec.is_none());
        assert!(pod.status.is_none());
    }

    #[test]
    fn test_pod_with_metadata() {
        let metadata = ObjectMeta {
            name: Some("my-pod".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        };
        let pod = Pod {
            metadata: Some(metadata),
            spec: None,
            status: None,
        };
        assert_eq!(
            pod.metadata.as_ref().unwrap().name,
            Some("my-pod".to_string())
        );
    }

    #[test]
    fn test_pod_serialize() {
        let mut labels = HashMap::new();
        labels.insert("app".to_string(), "nginx".to_string());

        let metadata = ObjectMeta {
            name: Some("my-pod".to_string()),
            labels,
            ..Default::default()
        };

        let container = Container {
            name: "nginx".to_string(),
            image: Some("nginx:latest".to_string()),
            ports: vec![],
        };

        let spec = PodSpec {
            containers: vec![container],
            restart_policy: Some("Always".to_string()),
            dns_policy: Some("ClusterFirst".to_string()),
        };

        let pod = Pod {
            metadata: Some(metadata),
            spec: Some(spec),
            status: None,
        };

        let json = serde_json::to_string(&pod).unwrap();
        assert!(json.contains("\"name\":\"my-pod\""));
        assert!(json.contains("\"app\":\"nginx\""));
        assert!(json.contains("\"containers\""));
        assert!(json.contains("\"restartPolicy\":\"Always\""));
    }

    #[test]
    fn test_pod_deserialize() {
        let json = r#"{
            "metadata": {"name": "my-pod"},
            "spec": {
                "containers": [{"name": "nginx", "image": "nginx:latest"}],
                "restartPolicy": "Always"
            }
        }"#;
        let pod: Pod = serde_json::from_str(json).unwrap();
        assert_eq!(
            pod.metadata.as_ref().unwrap().name,
            Some("my-pod".to_string())
        );
        assert_eq!(pod.spec.as_ref().unwrap().containers.len(), 1);
        assert_eq!(
            pod.spec.as_ref().unwrap().containers[0].name,
            "nginx".to_string()
        );
    }

    #[test]
    fn test_pod_round_trip() {
        let container = Container {
            name: "nginx".to_string(),
            image: Some("nginx:latest".to_string()),
            ports: vec![ContainerPort {
                name: Some("http".to_string()),
                container_port: 80,
                protocol: Some("TCP".to_string()),
            }],
        };

        let spec = PodSpec {
            containers: vec![container],
            restart_policy: Some("Always".to_string()),
            dns_policy: Some("ClusterFirst".to_string()),
        };

        let original = Pod {
            metadata: Some(ObjectMeta {
                name: Some("my-pod".to_string()),
                ..Default::default()
            }),
            spec: Some(spec),
            status: None,
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Pod = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_container_port() {
        let port = ContainerPort {
            name: Some("http".to_string()),
            container_port: 8080,
            protocol: Some("TCP".to_string()),
        };
        assert_eq!(port.name, Some("http".to_string()));
        assert_eq!(port.container_port, 8080);
    }

    #[test]
    fn test_container_port_serialize() {
        let port = ContainerPort {
            name: Some("http".to_string()),
            container_port: 8080,
            protocol: Some("TCP".to_string()),
        };
        let json = serde_json::to_string(&port).unwrap();
        // Check that containerPort field is present with value 8080
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["containerPort"], 8080);
    }
}
