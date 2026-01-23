//! ReplicationController types from the Kubernetes Core API
//!
//! This module contains types for ReplicationController resources,
//! which ensure that a specified number of pod replicas are running at any time.
//!
//! Source: k8s-pkg/apis/core/types.go

use crate::common::{ListMeta, ObjectMeta, TypeMeta};
use crate::impl_has_object_meta;
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
    pub metadata: ObjectMeta,
    /// Spec defines the desired behavior of this replication controller.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<ReplicationControllerSpec>,
    /// Status represents the current status of this replication controller.
    #[serde(default)]
    pub status: ReplicationControllerStatus,
}
impl_has_object_meta!(ReplicationController);

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
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub selector: std::collections::BTreeMap<String, String>,
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
    pub metadata: ObjectMeta,
    /// Spec defines the behavior of a pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<crate::core::internal::PodSpec>,
}

#[cfg(test)]
mod tests {}
