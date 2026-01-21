//! Pod types from the Kubernetes Core API
//!
//! This module contains the core Pod types including Pod, PodSpec, PodStatus,
//! PodTemplate, and related condition and configuration types.
//!
//! Source: k8s-pkg/apis/core/types.go

use crate::common::{ListMeta, ObjectMeta, TypeMeta};
use crate::core::internal::{
    Affinity, DNSPolicy, LocalObjectReference, PodDNSConfig, PodOS, PodPhase, PodResourceClaim,
    PodSchedulingGate, PodSecurityContext, PreemptionPolicy, ResourceList, RestartPolicy,
    Toleration,
};
use crate::core::v1;
use serde::{Deserialize, Serialize};

/// Type alias for Container from v1 API
pub type Container = v1::Container;
/// Type alias for PodReadinessGate from v1 API
pub type PodReadinessGate = v1::PodReadinessGate;
/// Type alias for TopologySpreadConstraint from v1 API
pub type TopologySpreadConstraint = v1::TopologySpreadConstraint;

// ============================================================================
// Pod
// ============================================================================

/// Pod is a collection of containers that can run on a host.
///
/// Corresponds to [Kubernetes Pod](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4583)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Pod {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// Spec defines the behavior of a pod.
    #[serde(default)]
    pub spec: PodSpec,
    /// Status represents the current information about a pod.
    #[serde(default)]
    pub status: PodStatus,
}

/// PodSpec is a description of a pod.
///
/// Corresponds to [Kubernetes PodSpec](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3656)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodSpec {
    /// List of volumes that can be mounted by containers belonging to the pod.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volumes: Vec<crate::core::internal::Volume>,
    /// List of initialization containers belonging to the pod.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub init_containers: Vec<Container>,
    /// List of containers belonging to the pod.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub containers: Vec<Container>,
    /// List of ephemeral containers run in this pod.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ephemeral_containers: Vec<Container>,
    /// Restart policy for all containers within the pod.
    #[serde(default)]
    pub restart_policy: RestartPolicy,
    /// Optional duration in seconds the pod needs to terminate gracefully.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub termination_grace_period_seconds: Option<i64>,
    /// Optional duration in seconds the pod may be active on a node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_deadline_seconds: Option<i64>,
    /// Set DNS policy for the pod.
    #[serde(default)]
    pub dns_policy: DNSPolicy,
    /// NodeSelector is a selector which must be true for the pod to fit on a node.
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub node_selector: std::collections::BTreeMap<String, String>,
    /// ServiceAccountName is the name of the ServiceAccount to use to run this pod.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub service_account_name: String,
    /// AutomountServiceAccountToken indicates whether a service account token should be automatically mounted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub automount_service_account_token: Option<bool>,
    /// NodeName indicates in which node this pod is scheduled.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub node_name: String,
    /// SecurityContext holds pod-level security attributes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_context: Option<PodSecurityContext>,
    /// ImagePullSecrets is an optional list of references to secrets.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub image_pull_secrets: Vec<LocalObjectReference>,
    /// Specifies the hostname of the Pod.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub hostname: String,
    /// If specified, the fully qualified Pod hostname will be "<hostname>.<subdomain>...".
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub subdomain: String,
    /// If true the pod's hostname will be configured as the pod's FQDN.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set_hostname_as_fqdn: Option<bool>,
    /// If specified, the pod's scheduling constraints.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub affinity: Option<Affinity>,
    /// If specified, the pod will be dispatched by specified scheduler.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub scheduler_name: String,
    /// If specified, the pod's tolerations.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tolerations: Vec<Toleration>,
    /// HostAliases is an optional list of hosts and IPs that will be injected into the pod's hosts file.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub host_aliases: Vec<HostAlias>,
    /// If specified, indicates the pod's priority.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub priority_class_name: String,
    /// The priority value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// PreemptionPolicy is the Policy for preempting pods with lower priority.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preemption_policy: Option<PreemptionPolicy>,
    /// Specifies the DNS parameters of a pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dns_config: Option<PodDNSConfig>,
    /// If specified, all readiness gates will be evaluated for pod readiness.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub readiness_gates: Vec<PodReadinessGate>,
    /// RuntimeClassName refers to a RuntimeClass object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runtime_class_name: Option<String>,
    /// Overhead represents the resource overhead associated with running a pod.
    #[serde(default, skip_serializing_if = "ResourceList::is_empty")]
    pub overhead: ResourceList,
    /// EnableServiceLinks indicates whether information about services should be injected.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable_service_links: Option<bool>,
    /// TopologySpreadConstraints describes how a group of pods ought to spread across topology domains.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topology_spread_constraints: Vec<TopologySpreadConstraint>,
    /// Specifies the OS of the containers in the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub os: Option<PodOS>,
    /// SchedulingGates is an opaque list of values that if specified will block scheduling the pod.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scheduling_gates: Vec<PodSchedulingGate>,
    /// ResourceClaims defines which ResourceClaims must be allocated.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resource_claims: Vec<PodResourceClaim>,
}

/// PodStatus represents the current state of a Pod.
///
/// Corresponds to [Kubernetes PodStatus](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4462)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodStatus {
    /// ObservedGeneration is the .metadata.generation that the pod status was set based upon.
    #[serde(default)]
    pub observed_generation: i64,
    /// Phase is the current phase of the pod.
    #[serde(default)]
    pub phase: PodPhase,
    /// Conditions is an array of current conditions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<PodCondition>,
    /// A human readable message indicating details about why the pod is in this state.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
    /// A brief CamelCase message indicating details about why the pod is in this state.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    /// nominatedNodeName is set when this pod preempts other pods on the node.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub nominated_node_name: String,
    /// HostIP holds the IP address of the host to which the pod is assigned.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub host_ip: String,
    /// HostIPs holds the IP addresses allocated to the host.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub host_ips: Vec<HostIP>,
    /// PodIPs holds all of the known IP addresses allocated to the pod.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pod_ips: Vec<PodIP>,
    /// StartTime is the time when the pod was acknowledged by the Kubelet.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<crate::common::Timestamp>,
    /// QOSClass is the quality of service class.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub qos_class: String,
    /// Statuses of init containers in this pod.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub init_container_statuses: Vec<crate::core::internal::ContainerStatus>,
    /// Statuses of containers in this pod.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub container_statuses: Vec<crate::core::internal::ContainerStatus>,
    /// Statuses for any ephemeral containers that have run in this pod.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ephemeral_container_statuses: Vec<crate::core::internal::ContainerStatus>,
}

/// PodList is a list of Pods.
///
/// Corresponds to [Kubernetes PodList](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3275)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    /// List of pods.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Pod>,
}

// ============================================================================
// Pod Template
// ============================================================================

/// PodTemplateSpec describes the data a pod should have when created from a template.
///
/// Corresponds to [Kubernetes PodTemplateSpec](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4599)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodTemplateSpec {
    /// Metadata of the pods created from this template.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// Spec defines the behavior of a pod.
    #[serde(default)]
    pub spec: PodSpec,
}

/// PodTemplate describes a template for creating copies of a predefined pod.
///
/// Corresponds to [Kubernetes PodTemplate](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4612)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodTemplate {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// Template defines the pods that will be created from this pod template.
    #[serde(default)]
    pub template: PodTemplateSpec,
}

/// PodTemplateList is a list of PodTemplates.
///
/// Corresponds to [Kubernetes PodTemplateList](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4625)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodTemplateList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    /// List of PodTemplates.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<PodTemplate>,
}

// ============================================================================
// Pod Condition
// ============================================================================

/// PodCondition contains details for the current condition of this pod.
///
/// Corresponds to [Kubernetes PodCondition](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3154)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodCondition {
    /// Type is the type of the condition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub r#type: String,
    /// ObservedGeneration is the .metadata.generation that the condition was set based upon.
    #[serde(default)]
    pub observed_generation: i64,
    /// Status is the status of the condition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub status: String,
    /// Last time we probed the condition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_probe_time: Option<crate::common::Timestamp>,
    /// Last time the condition transitioned from one status to another.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<crate::common::Timestamp>,
    /// Unique, one-word, CamelCase reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    /// Human-readable message indicating details about last transition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
}

// ============================================================================
// Pod IP and Host IP
// ============================================================================

/// PodIP represents a single IP address allocated to the pod.
///
/// Corresponds to [Kubernetes PodIP](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4311)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodIP {
    /// IP is the IP address assigned to the pod.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub ip: String,
}

/// HostIP represents a single IP address allocated to the host.
///
/// Corresponds to [Kubernetes HostIP](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4317)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct HostIP {
    /// IP is the IP address assigned to the host.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub ip: String,
}

// ============================================================================
// Host Alias
// ============================================================================

/// HostAlias holds the mapping between IP and hostnames that will be injected
/// as an entry in the pod's hosts file.
///
/// Corresponds to [Kubernetes HostAlias](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3986)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct HostAlias {
    /// IP address of the host file entry.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub ip: String,
    /// Hostnames for the above IP address.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hostnames: Vec<String>,
}

// ============================================================================
// Pod Resource Claim (already defined in pod_resources.rs)
// ============================================================================

// Preemption Policy (already defined in mod.rs)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Timestamp;
    use crate::core::internal::Volume;

    // ============================================================================
    // Pod Tests
    // ============================================================================

    #[test]
    fn test_pod_with_metadata() {
        let pod = Pod {
            type_meta: TypeMeta {
                kind: "Pod".to_string(),
                api_version: "v1".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("test-pod".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: PodSpec::default(),
            status: PodStatus::default(),
        };

        assert!(pod.metadata.is_some());
        assert_eq!(pod.metadata.unwrap().name.unwrap(), "test-pod");
    }

    #[test]
    fn test_pod_serialize() {
        let pod = Pod {
            type_meta: TypeMeta {
                kind: "Pod".to_string(),
                api_version: "v1".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("my-pod".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };

        let json = serde_json::to_string(&pod).unwrap();
        assert!(json.contains(r#""kind":"Pod""#));
        assert!(json.contains(r#""name":"my-pod""#));
        assert!(json.contains(r#""spec""#));
        assert!(json.contains(r#""status""#));
    }

    #[test]
    fn test_pod_deserialize() {
        let json = r#"{
            "kind": "Pod",
            "apiVersion": "v1",
            "metadata": {"name": "test-pod"},
            "spec": {},
            "status": {}
        }"#;

        let pod: Pod = serde_json::from_str(json).unwrap();
        assert_eq!(pod.type_meta.kind.unwrap(), "Pod");
        assert_eq!(pod.metadata.unwrap().name.unwrap(), "test-pod");
    }

    #[test]
    fn test_pod_round_trip() {
        let original = Pod {
            type_meta: TypeMeta {
                kind: "Pod".to_string(),
                api_version: "v1".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("round-trip-pod".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Pod = serde_json::from_str(&json).unwrap();

        assert_eq!(
            original.metadata.unwrap().name,
            deserialized.metadata.unwrap().name
        );
    }

    // ============================================================================
    // PodSpec Tests
    // ============================================================================

    #[test]
    fn test_pod_spec_default() {
        let spec = PodSpec::default();
        assert!(spec.containers.is_empty());
        assert!(spec.volumes.is_empty());
        assert_eq!(spec.restart_policy, RestartPolicy::Always);
        assert_eq!(spec.dns_policy, DNSPolicy::ClusterFirst);
        assert!(spec.node_selector.is_empty());
    }

    #[test]
    fn test_pod_spec_with_containers() {
        let spec = PodSpec {
            containers: vec![Container {
                name: "nginx".to_string(),
                image: Some("nginx:latest".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        };

        assert_eq!(spec.containers.len(), 1);
        assert_eq!(spec.containers[0].name, "nginx");
    }

    #[test]
    fn test_pod_spec_with_volumes() {
        let spec = PodSpec {
            volumes: vec![Volume {
                name: "config-volume".to_string(),
                ..Default::default()
            }],
            ..Default::default()
        };

        assert_eq!(spec.volumes.len(), 1);
        assert_eq!(spec.volumes[0].name, "config-volume");
    }

    #[test]
    fn test_pod_spec_with_node_selector() {
        let mut node_selector = std::collections::BTreeMap::new();
        node_selector.insert("disktype".to_string(), "ssd".to_string());

        let spec = PodSpec {
            node_selector,
            ..Default::default()
        };

        assert_eq!(spec.node_selector.len(), 1);
        assert_eq!(spec.node_selector.get("disktype").unwrap(), "ssd");
    }

    #[test]
    fn test_pod_spec_with_restart_policy() {
        let spec = PodSpec {
            restart_policy: RestartPolicy::OnFailure,
            ..Default::default()
        };

        assert_eq!(spec.restart_policy, RestartPolicy::OnFailure);
    }

    #[test]
    fn test_pod_spec_with_termination_grace_period() {
        let spec = PodSpec {
            termination_grace_period_seconds: Some(30),
            ..Default::default()
        };

        assert_eq!(spec.termination_grace_period_seconds, Some(30));
    }

    #[test]
    fn test_pod_spec_with_priority() {
        let spec = PodSpec {
            priority_class_name: "high-priority".to_string(),
            priority: Some(1000),
            ..Default::default()
        };

        assert_eq!(spec.priority_class_name, "high-priority");
        assert_eq!(spec.priority, Some(1000));
    }

    #[test]
    fn test_pod_spec_serialize() {
        let spec = PodSpec {
            restart_policy: RestartPolicy::Never,
            service_account_name: "my-service-account".to_string(),
            hostname: "my-host".to_string(),
            ..Default::default()
        };

        let json = serde_json::to_string(&spec).unwrap();
        assert!(json.contains(r#""restartPolicy":"Never""#));
        assert!(json.contains(r#""serviceAccountName":"my-service-account""#));
        assert!(json.contains(r#""hostname":"my-host""#));
    }

    #[test]
    fn test_pod_spec_with_host_aliases() {
        let spec = PodSpec {
            host_aliases: vec![HostAlias {
                ip: "127.0.0.1".to_string(),
                hostnames: vec!["localhost".to_string()],
            }],
            ..Default::default()
        };

        assert_eq!(spec.host_aliases.len(), 1);
        assert_eq!(spec.host_aliases[0].ip, "127.0.0.1");
    }

    #[test]
    fn test_pod_spec_with_dns_config() {
        let spec = PodSpec {
            dns_config: Some(PodDNSConfig {
                nameservers: vec!["8.8.8.8".to_string()],
                ..Default::default()
            }),
            ..Default::default()
        };

        assert!(spec.dns_config.is_some());
        assert_eq!(spec.dns_config.as_ref().unwrap().nameservers.len(), 1);
    }

    // ============================================================================
    // PodStatus Tests
    // ============================================================================

    #[test]
    fn test_pod_status_default() {
        let status = PodStatus::default();
        assert_eq!(status.observed_generation, 0);
        assert!(status.conditions.is_empty());
        assert!(status.container_statuses.is_empty());
        assert!(status.host_ip.is_empty());
        assert!(status.pod_ips.is_empty());
        assert_eq!(status.phase, PodPhase::Pending);
    }

    #[test]
    fn test_pod_status_with_phase() {
        let status = PodStatus {
            phase: PodPhase::Running,
            ..Default::default()
        };

        assert_eq!(status.phase, PodPhase::Running);
    }

    #[test]
    fn test_pod_status_with_pod_ips() {
        let status = PodStatus {
            pod_ips: vec![PodIP {
                ip: "10.0.0.1".to_string(),
            }],
            ..Default::default()
        };

        assert_eq!(status.pod_ips.len(), 1);
        assert_eq!(status.pod_ips[0].ip, "10.0.0.1");
    }

    #[test]
    fn test_pod_status_with_host_ips() {
        let status = PodStatus {
            host_ips: vec![HostIP {
                ip: "192.168.1.1".to_string(),
            }],
            ..Default::default()
        };

        assert_eq!(status.host_ips.len(), 1);
        assert_eq!(status.host_ips[0].ip, "192.168.1.1");
    }

    #[test]
    fn test_pod_status_with_conditions() {
        let status = PodStatus {
            conditions: vec![PodCondition {
                r#type: "Ready".to_string(),
                status: "True".to_string(),
                ..Default::default()
            }],
            ..Default::default()
        };

        assert_eq!(status.conditions.len(), 1);
        assert_eq!(status.conditions[0].r#type, "Ready");
    }

    #[test]
    fn test_pod_status_with_message_and_reason() {
        let status = PodStatus {
            message: "Pod was terminated".to_string(),
            reason: "OOMKilled".to_string(),
            ..Default::default()
        };

        assert_eq!(status.message, "Pod was terminated");
        assert_eq!(status.reason, "OOMKilled");
    }

    #[test]
    fn test_pod_status_serialize() {
        let status = PodStatus {
            phase: PodPhase::Running,
            host_ip: "192.168.1.1".to_string(),
            pod_ips: vec![PodIP {
                ip: "10.0.0.1".to_string(),
            }],
            message: "Test message".to_string(),
            qos_class: "Guaranteed".to_string(),
            ..Default::default()
        };

        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains(r#""phase":"Running""#));
        assert!(json.contains(r#""hostIp":"192.168.1.1""#));
        assert!(json.contains(r#""podIps""#));
    }

    // ============================================================================
    // PodCondition Tests
    // ============================================================================

    #[test]
    fn test_pod_condition_default() {
        let condition = PodCondition::default();
        assert!(condition.r#type.is_empty());
        assert!(condition.status.is_empty());
        assert_eq!(condition.observed_generation, 0);
    }

    #[test]
    fn test_pod_condition_with_type_and_status() {
        let condition = PodCondition {
            r#type: "Ready".to_string(),
            status: "True".to_string(),
            ..Default::default()
        };

        assert_eq!(condition.r#type, "Ready");
        assert_eq!(condition.status, "True");
    }

    #[test]
    fn test_pod_condition_with_timestamps() {
        let condition = PodCondition {
            r#type: "Ready".to_string(),
            status: "True".to_string(),
            last_probe_time: Some(Timestamp::from_str("2024-01-01T00:00:00Z").unwrap()),
            last_transition_time: Some(Timestamp::from_str("2024-01-01T00:01:00Z").unwrap()),
            ..Default::default()
        };

        assert!(condition.last_probe_time.is_some());
        assert!(condition.last_transition_time.is_some());
    }

    #[test]
    fn test_pod_condition_with_reason_and_message() {
        let condition = PodCondition {
            r#type: "ContainersReady".to_string(),
            status: "False".to_string(),
            reason: "ContainersNotReady".to_string(),
            message: "containers with unready status: [nginx]".to_string(),
            ..Default::default()
        };

        assert_eq!(condition.reason, "ContainersNotReady");
        assert!(condition.message.contains("nginx"));
    }

    #[test]
    fn test_pod_condition_serialize() {
        let condition = PodCondition {
            r#type: "Ready".to_string(),
            status: "True".to_string(),
            ..Default::default()
        };

        let json = serde_json::to_string(&condition).unwrap();
        assert!(json.contains(r#""type":"Ready""#));
        assert!(json.contains(r#""status":"True""#));
    }

    #[test]
    fn test_pod_condition_round_trip() {
        let original = PodCondition {
            r#type: "Initialized".to_string(),
            status: "True".to_string(),
            reason: "PodCompleted".to_string(),
            ..Default::default()
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: PodCondition = serde_json::from_str(&json).unwrap();

        assert_eq!(original.r#type, deserialized.r#type);
        assert_eq!(original.status, deserialized.status);
        assert_eq!(original.reason, deserialized.reason);
    }

    // ============================================================================
    // PodList Tests
    // ============================================================================

    #[test]
    fn test_pod_list_default() {
        let list = PodList::default();
        assert!(list.items.is_empty());
    }

    #[test]
    fn test_pod_list_with_items() {
        let list = PodList {
            items: vec![Pod {
                type_meta: TypeMeta {
                    kind: "Pod".to_string(),
                    api_version: "v1".to_string(),
                },
                ..Default::default()
            }],
            ..Default::default()
        };

        assert_eq!(list.items.len(), 1);
    }

    #[test]
    fn test_pod_list_serialize() {
        let list = PodList {
            type_meta: TypeMeta {
                kind: "PodList".to_string(),
                api_version: "v1".to_string(),
            },
            ..Default::default()
        };

        let json = serde_json::to_string(&list).unwrap();
        assert!(json.contains(r#""kind":"PodList""#));
        // Empty items are skipped due to skip_serializing_if, so we just check it doesn't contain items field
        assert!(!json.contains(r#""items""#));
    }

    // ============================================================================
    // PodTemplate Tests
    // ============================================================================

    #[test]
    fn test_pod_template_spec_default() {
        let template = PodTemplateSpec::default();
        assert!(template.spec.containers.is_empty());
        assert!(template.metadata.is_none());
    }

    #[test]
    fn test_pod_template_spec_with_metadata() {
        let template = PodTemplateSpec {
            metadata: Some(ObjectMeta {
                name: Some("pod-template".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };

        assert!(template.metadata.is_some());
        assert_eq!(template.metadata.unwrap().name.unwrap(), "pod-template");
    }

    #[test]
    fn test_pod_template_spec_with_containers() {
        let template = PodTemplateSpec {
            spec: PodSpec {
                containers: vec![Container {
                    name: "nginx".to_string(),
                    image: Some("nginx:latest".to_string()),
                    ..Default::default()
                }],
                ..Default::default()
            },
            ..Default::default()
        };

        assert_eq!(template.spec.containers.len(), 1);
    }

    #[test]
    fn test_pod_template_default() {
        let template = PodTemplate::default();
        assert!(template.metadata.is_none());
        assert!(template.template.spec.containers.is_empty());
    }

    #[test]
    fn test_pod_template_with_full_spec() {
        let template = PodTemplate {
            type_meta: TypeMeta {
                kind: "PodTemplate".to_string(),
                api_version: "v1".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("my-template".to_string()),
                ..Default::default()
            }),
            template: PodTemplateSpec {
                spec: PodSpec {
                    restart_policy: RestartPolicy::OnFailure,
                    ..Default::default()
                },
                ..Default::default()
            },
        };

        assert_eq!(template.metadata.unwrap().name.unwrap(), "my-template");
        assert_eq!(
            template.template.spec.restart_policy,
            RestartPolicy::OnFailure
        );
    }

    #[test]
    fn test_pod_template_list_default() {
        let list = PodTemplateList::default();
        assert!(list.items.is_empty());
    }

    #[test]
    fn test_pod_template_list_with_items() {
        let list = PodTemplateList {
            items: vec![PodTemplate {
                type_meta: TypeMeta {
                    kind: "PodTemplate".to_string(),
                    api_version: "v1".to_string(),
                },
                ..Default::default()
            }],
            ..Default::default()
        };

        assert_eq!(list.items.len(), 1);
    }

    // ============================================================================
    // PodIP and HostIP Tests
    // ============================================================================

    #[test]
    fn test_pod_ip_default() {
        let pod_ip = PodIP::default();
        assert!(pod_ip.ip.is_empty());
    }

    #[test]
    fn test_pod_ip_with_value() {
        let pod_ip = PodIP {
            ip: "10.244.1.5".to_string(),
        };

        assert_eq!(pod_ip.ip, "10.244.1.5");
    }

    #[test]
    fn test_pod_ip_serialize() {
        let pod_ip = PodIP {
            ip: "10.244.1.5".to_string(),
        };

        let json = serde_json::to_string(&pod_ip).unwrap();
        assert!(json.contains(r#""ip":"10.244.1.5""#));
    }

    #[test]
    fn test_host_ip_default() {
        let host_ip = HostIP::default();
        assert!(host_ip.ip.is_empty());
    }

    #[test]
    fn test_host_ip_with_value() {
        let host_ip = HostIP {
            ip: "192.168.1.100".to_string(),
        };

        assert_eq!(host_ip.ip, "192.168.1.100");
    }

    #[test]
    fn test_host_ip_serialize() {
        let host_ip = HostIP {
            ip: "192.168.1.100".to_string(),
        };

        let json = serde_json::to_string(&host_ip).unwrap();
        assert!(json.contains(r#""ip":"192.168.1.100""#));
    }

    // ============================================================================
    // HostAlias Tests
    // ============================================================================

    #[test]
    fn test_host_alias_default() {
        let alias = HostAlias::default();
        assert!(alias.ip.is_empty());
        assert!(alias.hostnames.is_empty());
    }

    #[test]
    fn test_host_alias_with_hostnames() {
        let alias = HostAlias {
            ip: "127.0.0.1".to_string(),
            hostnames: vec!["localhost".to_string(), "local".to_string()],
        };

        assert_eq!(alias.ip, "127.0.0.1");
        assert_eq!(alias.hostnames.len(), 2);
    }

    #[test]
    fn test_host_alias_serialize() {
        let alias = HostAlias {
            ip: "127.0.0.1".to_string(),
            hostnames: vec!["localhost".to_string()],
        };

        let json = serde_json::to_string(&alias).unwrap();
        assert!(json.contains(r#""ip":"127.0.0.1""#));
        assert!(json.contains(r#""hostnames""#));
    }

    #[test]
    fn test_host_alias_round_trip() {
        let original = HostAlias {
            ip: "10.0.0.1".to_string(),
            hostnames: vec!["host1".to_string(), "host2".to_string()],
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: HostAlias = serde_json::from_str(&json).unwrap();

        assert_eq!(original.ip, deserialized.ip);
        assert_eq!(original.hostnames.len(), deserialized.hostnames.len());
    }

    // ============================================================================
    // Integration Tests
    // ============================================================================

    #[test]
    fn test_pod_full_spec() {
        let mut node_selector = std::collections::BTreeMap::new();
        node_selector.insert("kubernetes.io/os".to_string(), "linux".to_string());

        let spec = PodSpec {
            containers: vec![Container {
                name: "nginx".to_string(),
                image: Some("nginx:1.21".to_string()),
                ..Default::default()
            }],
            init_containers: vec![Container {
                name: "init".to_string(),
                image: Some("busybox:latest".to_string()),
                ..Default::default()
            }],
            volumes: vec![Volume {
                name: "config".to_string(),
                ..Default::default()
            }],
            restart_policy: RestartPolicy::Always,
            node_selector,
            dns_policy: DNSPolicy::ClusterFirst,
            hostname: "my-pod".to_string(),
            ..Default::default()
        };

        assert_eq!(spec.containers.len(), 1);
        assert_eq!(spec.init_containers.len(), 1);
        assert_eq!(spec.volumes.len(), 1);
        assert_eq!(spec.hostname, "my-pod");
    }

    #[test]
    fn test_pod_full_status() {
        let status = PodStatus {
            phase: PodPhase::Running,
            conditions: vec![
                PodCondition {
                    r#type: "Initialized".to_string(),
                    status: "True".to_string(),
                    ..Default::default()
                },
                PodCondition {
                    r#type: "Ready".to_string(),
                    status: "True".to_string(),
                    ..Default::default()
                },
            ],
            host_ip: "192.168.1.10".to_string(),
            pod_ips: vec![PodIP {
                ip: "10.244.0.5".to_string(),
            }],
            qos_class: "Guaranteed".to_string(),
            ..Default::default()
        };

        assert_eq!(status.phase, PodPhase::Running);
        assert_eq!(status.conditions.len(), 2);
        assert_eq!(status.qos_class, "Guaranteed");
    }

    #[test]
    fn test_pod_complete_round_trip() {
        let original = Pod {
            type_meta: TypeMeta {
                kind: "Pod".to_string(),
                api_version: "v1".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("complete-pod".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: PodSpec {
                containers: vec![Container {
                    name: "main".to_string(),
                    image: Some("image:latest".to_string()),
                    ..Default::default()
                }],
                restart_policy: RestartPolicy::OnFailure,
                ..Default::default()
            },
            status: PodStatus {
                phase: PodPhase::Running,
                ..Default::default()
            },
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Pod = serde_json::from_str(&json).unwrap();

        assert_eq!(
            original.metadata.unwrap().name,
            deserialized.metadata.unwrap().name
        );
        assert_eq!(
            original.spec.restart_policy,
            deserialized.spec.restart_policy
        );
        assert_eq!(original.status.phase, deserialized.status.phase);
    }
}
