//! ReplicationController types from the Kubernetes Core API
//!
//! This module contains types for ReplicationController resources,
//! which ensure that a specified number of pod replicas are running at any time.
//!
//! Source: k8s-pkg/apis/core/types.go

use crate::common::{ListMeta, ObjectMeta, TypeMeta};
use serde::{Deserialize, Serialize};

// ============================================================================
// ReplicationController
// ============================================================================

/// ReplicationController ensures that a specified number of pod replicas are running at any time.
///
/// Corresponds to [Kubernetes ReplicationController](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3332)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReplicationController {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// Spec defines the desired behavior of this replication controller.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<ReplicationControllerSpec>,
    /// Status represents the current status of this replication controller.
    #[serde(default)]
    pub status: ReplicationControllerStatus,
}

/// ReplicationControllerSpec defines the desired behavior of a replication controller.
///
/// Corresponds to [Kubernetes ReplicationControllerSpec](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3347)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReplicationControllerSpec {
    /// Replicas is the number of desired replicas.
    #[serde(default)]
    pub replicas: i32,
    /// Minimum number of seconds for which a newly created pod should be ready.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_ready_seconds: Option<i32>,
    /// Selector is a label query over pods that should match the Replicas count.
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub selector: std::collections::HashMap<String, String>,
    /// Template is the object that describes the pod that will be created.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<PodTemplateSpec>,
}

/// ReplicationControllerStatus represents the current status of a replication controller.
///
/// Corresponds to [Kubernetes ReplicationControllerStatus](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3366)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReplicationControllerStatus {
    /// Replicas is the most recently observed number of replicas.
    #[serde(default)]
    pub replicas: i32,
    /// FullyLabeledReplicas is the number of pods that have labels.
    #[serde(default)]
    pub fully_labeled_replicas: i32,
    /// ReadyReplicas is the number of pods that are ready.
    #[serde(default)]
    pub ready_replicas: i32,
    /// AvailableReplicas is the number of pods that are available.
    #[serde(default)]
    pub available_replicas: i32,
    /// ObservedGeneration is the most recent generation observed by the controller.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
    /// Conditions represent the latest available observations of a replication controller's current state.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<ReplicationControllerCondition>,
}

/// ReplicationControllerList is a collection of replication controllers.
///
/// Corresponds to [Kubernetes ReplicationControllerList](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3341)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReplicationControllerList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    /// List of replication controllers.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ReplicationController>,
}

/// ReplicationControllerCondition describes the state of a replication controller at a certain point.
///
/// Corresponds to [Kubernetes ReplicationControllerCondition](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3382)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReplicationControllerCondition {
    /// Type of replication controller condition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub r#type: String,
    /// Status of the condition, one of True, False, Unknown.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub status: String,
    /// The last time the condition transitioned from one status to another.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<crate::common::Timestamp>,
    /// The reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    /// A human readable message indicating details about the transition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
}

/// PodTemplateSpec describes the data a pod should have when created from a template.
///
/// Corresponds to [Kubernetes PodTemplateSpec](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3392)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodTemplateSpec {
    /// Metadata of the pods created from this template.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// Spec defines the behavior of a pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<crate::core::internal::PodSpec>,
}

#[cfg(test)]
mod tests {
    use super::*;

    // ReplicationController tests
    #[test]
    fn test_replication_controller_default() {
        let rc = ReplicationController {
            type_meta: TypeMeta::default(),
            metadata: None,
            spec: None,
            status: ReplicationControllerStatus::default(),
        };
        assert!(rc.spec.is_none());
        assert_eq!(rc.status.replicas, 0);
    }

    #[test]
    fn test_replication_controller_with_fields() {
        let spec = ReplicationControllerSpec {
            replicas: 3,
            ..Default::default()
        };
        let rc = ReplicationController {
            type_meta: TypeMeta::default(),
            metadata: None,
            spec: Some(spec),
            ..Default::default()
        };
        assert!(rc.spec.is_some());
        assert_eq!(rc.spec.unwrap().replicas, 3);
    }

    #[test]
    fn test_replication_controller_serialize() {
        let rc = ReplicationController {
            type_meta: TypeMeta {
                kind: Some("ReplicationController".to_string()),
                api_version: Some("v1".to_string()),
            },
            metadata: Some(ObjectMeta {
                name: Some("my-rc".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let json = serde_json::to_string(&rc).unwrap();
        assert!(json.contains(r#""kind":"ReplicationController""#));
        assert!(json.contains(r#""name":"my-rc""#));
    }

    #[test]
    fn test_replication_controller_list_default() {
        let list = ReplicationControllerList::default();
        assert!(list.items.is_empty());
    }

    #[test]
    fn test_replication_controller_list_with_items() {
        let list = ReplicationControllerList {
            items: vec![ReplicationController {
                type_meta: TypeMeta {
                    kind: Some("ReplicationController".to_string()),
                    api_version: Some("v1".to_string()),
                },
                ..Default::default()
            }],
            ..Default::default()
        };
        assert_eq!(list.items.len(), 1);
    }

    // ReplicationControllerSpec tests
    #[test]
    fn test_replication_controller_spec_default() {
        let spec = ReplicationControllerSpec::default();
        assert_eq!(spec.replicas, 0);
        assert!(spec.selector.is_empty());
        assert!(spec.template.is_none());
    }

    #[test]
    fn test_replication_controller_spec_with_replicas() {
        let spec = ReplicationControllerSpec {
            replicas: 5,
            min_ready_seconds: Some(30),
            ..Default::default()
        };
        assert_eq!(spec.replicas, 5);
        assert_eq!(spec.min_ready_seconds, Some(30));
    }

    #[test]
    fn test_replication_controller_spec_with_selector() {
        let mut selector = std::collections::HashMap::new();
        selector.insert("app".to_string(), "nginx".to_string());

        let spec = ReplicationControllerSpec {
            selector,
            ..Default::default()
        };
        assert_eq!(spec.selector.len(), 1);
    }

    #[test]
    fn test_replication_controller_spec_serialize() {
        let spec = ReplicationControllerSpec {
            replicas: 3,
            ..Default::default()
        };
        let json = serde_json::to_string(&spec).unwrap();
        assert!(json.contains(r#""replicas":3"#));
    }

    // ReplicationControllerStatus tests
    #[test]
    fn test_replication_controller_status_default() {
        let status = ReplicationControllerStatus::default();
        assert_eq!(status.replicas, 0);
        assert_eq!(status.ready_replicas, 0);
    }

    #[test]
    fn test_replication_controller_status_with_replicas() {
        let status = ReplicationControllerStatus {
            replicas: 3,
            fully_labeled_replicas: 3,
            ready_replicas: 2,
            available_replicas: 2,
            ..Default::default()
        };
        assert_eq!(status.replicas, 3);
        assert_eq!(status.ready_replicas, 2);
    }

    #[test]
    fn test_replication_controller_status_with_conditions() {
        let status = ReplicationControllerStatus {
            replicas: 3,
            conditions: vec![ReplicationControllerCondition {
                r#type: "ReplicaFailure".to_string(),
                status: "True".to_string(),
                ..Default::default()
            }],
            ..Default::default()
        };
        assert_eq!(status.conditions.len(), 1);
        assert_eq!(status.conditions[0].r#type, "ReplicaFailure");
    }

    #[test]
    fn test_replication_controller_status_serialize() {
        let status = ReplicationControllerStatus {
            replicas: 5,
            ready_replicas: 3,
            ..Default::default()
        };
        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains(r#""replicas":5"#));
        assert!(json.contains(r#""readyReplicas":3"#));
    }

    // ReplicationControllerCondition tests
    #[test]
    fn test_replication_controller_condition_default() {
        let condition = ReplicationControllerCondition::default();
        assert!(condition.r#type.is_empty());
        assert!(condition.status.is_empty());
    }

    #[test]
    fn test_replication_controller_condition_with_fields() {
        let condition = ReplicationControllerCondition {
            r#type: "ReplicaFailure".to_string(),
            status: "True".to_string(),
            reason: "FailedCreate".to_string(),
            message: "Failed to create pod".to_string(),
            ..Default::default()
        };
        assert_eq!(condition.r#type, "ReplicaFailure");
        assert_eq!(condition.status, "True");
        assert_eq!(condition.reason, "FailedCreate");
    }

    #[test]
    fn test_replication_controller_condition_serialize() {
        let condition = ReplicationControllerCondition {
            r#type: "ReplicaFailure".to_string(),
            status: "False".to_string(),
            ..Default::default()
        };
        let json = serde_json::to_string(&condition).unwrap();
        assert!(json.contains(r#""type":"ReplicaFailure""#));
        assert!(json.contains(r#""status":"False""#));
    }

    #[test]
    fn test_replication_controller_condition_round_trip() {
        let original = ReplicationControllerCondition {
            r#type: "Available".to_string(),
            status: "True".to_string(),
            reason: "MinimumReplicasAvailable".to_string(),
            ..Default::default()
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: ReplicationControllerCondition = serde_json::from_str(&json).unwrap();
        assert_eq!(original.r#type, deserialized.r#type);
        assert_eq!(original.status, deserialized.status);
    }

    // PodTemplateSpec tests
    #[test]
    fn test_pod_template_spec_default() {
        let template = PodTemplateSpec::default();
        assert!(template.metadata.is_none());
        assert!(template.spec.is_none());
    }

    #[test]
    fn test_pod_template_spec_with_metadata() {
        let template = PodTemplateSpec {
            metadata: Some(ObjectMeta {
                name: Some("my-pod".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert!(template.metadata.is_some());
        assert_eq!(template.metadata.unwrap().name.unwrap(), "my-pod");
    }

    #[test]
    fn test_pod_template_spec_serialize() {
        let template = PodTemplateSpec {
            metadata: Some(ObjectMeta {
                name: Some("test-pod".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let json = serde_json::to_string(&template).unwrap();
        assert!(json.contains(r#""metadata""#));
    }

    // Integration tests
    #[test]
    fn test_replication_controller_with_status() {
        let rc = ReplicationController {
            type_meta: TypeMeta {
                kind: Some("ReplicationController".to_string()),
                api_version: Some("v1".to_string()),
            },
            metadata: Some(ObjectMeta {
                name: Some("my-rc".to_string()),
                ..Default::default()
            }),
            spec: Some(ReplicationControllerSpec {
                replicas: 3,
                ..Default::default()
            }),
            status: ReplicationControllerStatus {
                replicas: 3,
                ready_replicas: 3,
                ..Default::default()
            },
        };
        assert_eq!(rc.spec.unwrap().replicas, 3);
        assert_eq!(rc.status.ready_replicas, 3);
    }

    #[test]
    fn test_replication_controller_round_trip() {
        let original = ReplicationController {
            type_meta: TypeMeta {
                kind: Some("ReplicationController".to_string()),
                api_version: Some("v1".to_string()),
            },
            metadata: Some(ObjectMeta {
                name: Some("test-rc".to_string()),
                ..Default::default()
            }),
            spec: Some(ReplicationControllerSpec {
                replicas: 2,
                ..Default::default()
            }),
            ..Default::default()
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: ReplicationController = serde_json::from_str(&json).unwrap();
        assert_eq!(
            original.metadata.unwrap().name,
            deserialized.metadata.unwrap().name
        );
    }

    #[test]
    fn test_replication_controller_list_serialize() {
        let list = ReplicationControllerList {
            type_meta: TypeMeta {
                kind: Some("ReplicationControllerList".to_string()),
                api_version: Some("v1".to_string()),
            },
            ..Default::default()
        };
        let json = serde_json::to_string(&list).unwrap();
        assert!(json.contains(r#""kind":"ReplicationControllerList""#));
    }

    #[test]
    fn test_replication_controller_spec_min_ready_seconds() {
        let spec = ReplicationControllerSpec {
            replicas: 1,
            min_ready_seconds: Some(60),
            ..Default::default()
        };
        assert_eq!(spec.min_ready_seconds, Some(60));
    }

    #[test]
    fn test_replication_controller_status_observed_generation() {
        let status = ReplicationControllerStatus {
            replicas: 3,
            observed_generation: Some(5),
            ..Default::default()
        };
        assert_eq!(status.observed_generation, Some(5));
    }

    #[test]
    fn test_replication_controller_status_with_multiple_conditions() {
        let status = ReplicationControllerStatus {
            replicas: 3,
            conditions: vec![
                ReplicationControllerCondition {
                    r#type: "Available".to_string(),
                    status: "True".to_string(),
                    ..Default::default()
                },
                ReplicationControllerCondition {
                    r#type: "Progressing".to_string(),
                    status: "True".to_string(),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        assert_eq!(status.conditions.len(), 2);
    }

    #[test]
    fn test_pod_template_spec_round_trip() {
        let original = PodTemplateSpec {
            metadata: Some(ObjectMeta {
                name: Some("test-pod".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: PodTemplateSpec = serde_json::from_str(&json).unwrap();
        assert_eq!(
            original.metadata.unwrap().name,
            deserialized.metadata.unwrap().name
        );
    }
}
