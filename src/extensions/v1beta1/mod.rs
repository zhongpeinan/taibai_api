//! Extensions v1beta1 API types
//!
//! This module contains the v1beta1 version of the Kubernetes Extensions API types.
//!
//! **NOTE**: This API group is deprecated. Most types have been moved to:
//! - `apps/v1beta2` and `apps/v1` for Deployment, DaemonSet, ReplicaSet
//! - `networking.k8s.io/v1beta1` and `networking.k8s.io/v1` for Ingress, NetworkPolicy
//!
//! Source: api-master/extensions/v1beta1/types.go

use crate::common::{IntOrString, LabelSelector, ListMeta, ObjectMeta, TypeMeta};
use crate::core::internal::{ConditionStatus, Protocol};
use crate::core::v1::{PodTemplateSpec, TypedLocalObjectReference};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// ============================================================================
// Scale Types
// ============================================================================

/// Scale represents a scaling request for a resource.
///
/// This is a subresource used for scaling resources like Deployments, ReplicaSets, etc.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Scale {
    /// Standard object metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// Defines the behavior of the scale.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<ScaleSpec>,

    /// Current status of the scale.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ScaleStatus>,
}

/// ScaleSpec describes the attributes of a scale subresource.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScaleSpec {
    /// Desired number of instances for the scaled object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
}

/// ScaleStatus represents the current status of a scale subresource.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScaleStatus {
    /// Actual number of observed instances of the scaled object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,

    /// Label selector for pods that should match the replicas count.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub selector: BTreeMap<String, String>,

    /// Label selector for pods that should match the replicas count (string format).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_selector: Option<String>,
}

// ============================================================================
// Deployment Types (DEPRECATED)
// ============================================================================

/// Deployment enables declarative updates for Pods and ReplicaSets.
///
/// **DEPRECATED** - This group version of Deployment is deprecated by apps/v1beta2/Deployment.
/// Use apps/v1 instead.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Deployment {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// Specification of the desired behavior of the Deployment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<DeploymentSpec>,

    /// Most recently observed status of the Deployment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<DeploymentStatus>,
}

/// DeploymentSpec is the specification of the desired behavior of the Deployment.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentSpec {
    /// Number of desired pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,

    /// Label selector for pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<LabelSelector>,

    /// Template describes the pods that will be created.
    #[serde(default)]
    pub template: PodTemplateSpec,

    /// The deployment strategy to use to replace existing pods with new ones.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strategy: Option<DeploymentStrategy>,

    /// Minimum number of seconds for which a newly created pod should be ready.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_ready_seconds: Option<i32>,

    /// The number of old ReplicaSets to retain to allow rollback.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision_history_limit: Option<i32>,

    /// Indicates that the deployment is paused.
    #[serde(default)]
    pub paused: bool,

    /// **DEPRECATED**. The config this deployment is rolling back to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rollback_to: Option<RollbackConfig>,

    /// The maximum time in seconds for a deployment to make progress before it is considered failed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_deadline_seconds: Option<i32>,
}

/// DeploymentStrategy describes how to replace existing pods with new ones.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentStrategy {
    /// Type of deployment. Can be "Recreate" or "RollingUpdate".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<DeploymentStrategyType>,

    /// Rolling update config params.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rolling_update: Option<RollingUpdateDeployment>,
}

/// DeploymentStrategyType specifies the type of deployment.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "PascalCase")]
pub enum DeploymentStrategyType {
    /// Kill all existing pods before creating new ones.
    #[default]
    Recreate,
    /// Replace the old RCs by new one using rolling update.
    RollingUpdate,
}

/// RollingUpdateDeployment specifies the rolling update behavior.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RollingUpdateDeployment {
    /// The maximum number of pods that can be unavailable during the update.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_unavailable: Option<IntOrString>,

    /// The maximum number of pods that can be scheduled above the desired number of pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_surge: Option<IntOrString>,
}

/// RollbackConfig specifies the rollback configuration.
///
/// **DEPRECATED**.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RollbackConfig {
    /// The revision to rollback to. If set to 0, rollback to the last revision.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
}

/// DeploymentStatus is the most recently observed status of the Deployment.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentStatus {
    /// The generation observed by the deployment controller.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,

    /// Total number of non-terminating pods targeted by this deployment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,

    /// Total number of non-terminating pods targeted by this deployment with the desired template spec.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_replicas: Option<i32>,

    /// Total number of non-terminating pods targeted by this Deployment with a Ready Condition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ready_replicas: Option<i32>,

    /// Total number of available non-terminating pods targeted by this deployment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub available_replicas: Option<i32>,

    /// Total number of unavailable pods targeted by this deployment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unavailable_replicas: Option<i32>,

    /// Total number of terminating pods targeted by this deployment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminating_replicas: Option<i32>,

    /// Represents the latest available observations of a deployment's current state.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<DeploymentCondition>,

    /// Count of hash collisions for the Deployment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collision_count: Option<i32>,
}

/// DeploymentCondition describes the state of a deployment at a certain point.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentCondition {
    /// Type of deployment condition.
    #[serde(rename = "type")]
    pub type_: DeploymentConditionType,

    /// Status of the condition, one of True, False, Unknown.
    #[serde(default)]
    pub status: ConditionStatus,

    /// The last time this condition was updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<crate::common::Timestamp>,

    /// Last time the condition transitioned from one status to another.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<crate::common::Timestamp>,

    /// The reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,

    /// A human readable message indicating details about the transition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
}

/// DeploymentConditionType specifies the type of deployment condition.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "PascalCase")]
pub enum DeploymentConditionType {
    /// Available means the deployment is available.
    #[default]
    Available,
    /// Progressing means the deployment is progressing.
    Progressing,
    /// ReplicaFailure is added when one of its pods fails to be created or deleted.
    ReplicaFailure,
}

/// DeploymentList is a list of Deployments.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentList {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// Items is the list of Deployments.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Deployment>,
}

/// DeploymentRollback stores the information required to rollback a deployment.
///
/// **DEPRECATED**.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentRollback {
    /// Required: This must match the Name of a deployment.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    /// The annotations to be updated to a deployment.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub updated_annotations: BTreeMap<String, String>,

    /// The config of this deployment rollback.
    #[serde(default)]
    pub rollback_to: RollbackConfig,
}

// ============================================================================
// DaemonSet Types (DEPRECATED)
// ============================================================================

/// DaemonSet represents the configuration of a daemon set.
///
/// **DEPRECATED** - This group version of DaemonSet is deprecated by apps/v1beta2/DaemonSet.
/// Use apps/v1 instead.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DaemonSet {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// The desired behavior of this daemon set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<DaemonSetSpec>,

    /// The current status of this daemon set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<DaemonSetStatus>,
}

/// DaemonSetSpec is the specification of a daemon set.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DaemonSetSpec {
    /// A label query over pods that are managed by the daemon set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<LabelSelector>,

    /// An object that describes the pod that will be created.
    #[serde(default)]
    pub template: PodTemplateSpec,

    /// An update strategy to replace existing DaemonSet pods with new pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_strategy: Option<DaemonSetUpdateStrategy>,

    /// The minimum number of seconds for which a newly created DaemonSet pod should be ready.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_ready_seconds: Option<i32>,

    /// **DEPRECATED**. A sequence number representing a specific generation of the template.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template_generation: Option<i64>,

    /// The number of old history to retain to allow rollback.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision_history_limit: Option<i32>,
}

/// DaemonSetUpdateStrategy indicates the strategy that the DaemonSet controller will use.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DaemonSetUpdateStrategy {
    /// Type of daemon set update. Can be "RollingUpdate" or "OnDelete".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<DaemonSetUpdateStrategyType>,

    /// Rolling update config params.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rolling_update: Option<RollingUpdateDaemonSet>,
}

/// DaemonSetUpdateStrategyType specifies the strategy for DaemonSet updates.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "PascalCase")]
pub enum DaemonSetUpdateStrategyType {
    /// Replace the old daemons by new ones using rolling update.
    #[default]
    RollingUpdate,
    /// Replace the old daemons only when it's killed.
    OnDelete,
}

/// RollingUpdateDaemonSet specifies the rolling update behavior for DaemonSets.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RollingUpdateDaemonSet {
    /// The maximum number of DaemonSet pods that can be unavailable during the update.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_unavailable: Option<IntOrString>,

    /// The maximum number of nodes with an existing available DaemonSet pod that can have an updated pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_surge: Option<IntOrString>,
}

/// DaemonSetStatus represents the current status of a daemon set.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DaemonSetStatus {
    /// The number of nodes that are running at least 1 daemon pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_number_scheduled: Option<i32>,

    /// The number of nodes that are running the daemon pod, but are not supposed to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_misscheduled: Option<i32>,

    /// The total number of nodes that should be running the daemon pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub desired_number_scheduled: Option<i32>,

    /// The number of nodes that should be running the daemon pod and have one ready.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_ready: Option<i32>,

    /// The most recent generation observed by the daemon set controller.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,

    /// The total number of nodes that are running updated daemon pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_number_scheduled: Option<i32>,

    /// The number of nodes that should be running the daemon pod and have available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_available: Option<i32>,

    /// The number of nodes that should be running the daemon pod and have none available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_unavailable: Option<i32>,

    /// Count of hash collisions for the DaemonSet.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collision_count: Option<i32>,

    /// Represents the latest available observations of a DaemonSet's current state.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<DaemonSetCondition>,
}

/// DaemonSetCondition describes the state of a DaemonSet at a certain point.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DaemonSetCondition {
    /// Type of DaemonSet condition.
    #[serde(rename = "type")]
    pub type_: String,

    /// Status of the condition, one of True, False, Unknown.
    #[serde(default)]
    pub status: ConditionStatus,

    /// Last time the condition transitioned from one status to another.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<crate::common::Timestamp>,

    /// The reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,

    /// A human readable message indicating details about the transition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
}

/// DaemonSetList is a collection of daemon sets.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DaemonSetList {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// A list of daemon sets.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<DaemonSet>,
}

// ============================================================================
// ReplicaSet Types (DEPRECATED)
// ============================================================================

/// ReplicaSet ensures that a specified number of pod replicas are running.
///
/// **DEPRECATED** - This group version of ReplicaSet is deprecated by apps/v1beta2/ReplicaSet.
/// Use apps/v1 instead.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReplicaSet {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// Spec defines the specification of the desired behavior.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<ReplicaSetSpec>,

    /// Status is the most recently observed status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ReplicaSetStatus>,
}

/// ReplicaSetSpec is the specification of a ReplicaSet.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReplicaSetSpec {
    /// Replicas is the number of desired pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,

    /// Minimum number of seconds for which a newly created pod should be ready.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_ready_seconds: Option<i32>,

    /// Selector is a label query over pods that should match the replica count.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<LabelSelector>,

    /// Template is the object that describes the pod that will be created.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<PodTemplateSpec>,
}

/// ReplicaSetStatus represents the current status of a ReplicaSet.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReplicaSetStatus {
    /// Replicas is the most recently observed number of pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,

    /// The number of pods with labels matching the labels of the pod template.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fully_labeled_replicas: Option<i32>,

    /// The number of pods with a Ready Condition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ready_replicas: Option<i32>,

    /// The number of available pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub available_replicas: Option<i32>,

    /// The number of terminating pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminating_replicas: Option<i32>,

    /// ObservedGeneration reflects the generation of the most recently observed ReplicaSet.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,

    /// Represents the latest available observations.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<ReplicaSetCondition>,
}

/// ReplicaSetCondition describes the state of a replica set at a certain point.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReplicaSetCondition {
    /// Type of replica set condition.
    #[serde(rename = "type")]
    pub type_: ReplicaSetConditionType,

    /// Status of the condition, one of True, False, Unknown.
    #[serde(default)]
    pub status: ConditionStatus,

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

/// ReplicaSetConditionType specifies the type of replica set condition.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "PascalCase")]
pub enum ReplicaSetConditionType {
    /// ReplicaFailure is added when one of its pods fails to be created or deleted.
    #[default]
    ReplicaFailure,
}

/// ReplicaSetList is a collection of ReplicaSets.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReplicaSetList {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// List of ReplicaSets.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ReplicaSet>,
}

// ============================================================================
// Ingress Types (DEPRECATED)
// ============================================================================

/// Ingress is a collection of rules that allow inbound connections to reach endpoints.
///
/// **DEPRECATED** - This group version of Ingress is deprecated by networking.k8s.io/v1beta1.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Ingress {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// Spec is the desired state of the Ingress.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<IngressSpec>,

    /// Status is the current state of the Ingress.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<IngressStatus>,
}

/// IngressSpec describes the Ingress the user wishes to exist.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct IngressSpec {
    /// IngressClassName is the name of the IngressClass cluster resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ingress_class_name: Option<String>,

    /// A default backend capable of servicing requests that don't match any rule.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backend: Option<IngressBackend>,

    /// TLS configuration.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tls: Vec<IngressTLS>,

    /// A list of host rules used to configure the Ingress.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<IngressRule>,
}

/// IngressTLS describes the transport layer security associated with an Ingress.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct IngressTLS {
    /// Hosts are a list of hosts included in the TLS certificate.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hosts: Vec<String>,

    /// SecretName is the name of the secret used to terminate SSL traffic on 443.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub secret_name: String,
}

/// IngressStatus describe the current state of the Ingress.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct IngressStatus {
    /// LoadBalancer contains the current status of the load-balancer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load_balancer: Option<IngressLoadBalancerStatus>,
}

/// IngressLoadBalancerStatus represents the status of a load-balancer.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct IngressLoadBalancerStatus {
    /// Ingress is a list containing ingress points for the load-balancer.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ingress: Vec<IngressLoadBalancerIngress>,
}

/// IngressLoadBalancerIngress represents the status of a load-balancer ingress point.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct IngressLoadBalancerIngress {
    /// IP is set for load-balancer ingress points that are IP based.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub ip: String,

    /// Hostname is set for load-balancer ingress points that are DNS based.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub hostname: String,

    /// Ports provides information about the ports exposed by this LoadBalancer.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<IngressPortStatus>,
}

/// IngressPortStatus represents the error condition of a service port.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct IngressPortStatus {
    /// Port is the port number of the ingress port.
    #[serde(default)]
    pub port: i32,

    /// Protocol is the protocol of the ingress port.
    #[serde(default)]
    pub protocol: Protocol,

    /// Error is to record the problem with the service port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// IngressRule represents the rules mapping the paths under a specified host to related backends.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct IngressRule {
    /// Host is the fully qualified domain name of a network host.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub host: String,

    /// IngressRuleValue represents a rule to route requests for this IngressRule.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http: Option<HTTPIngressRuleValue>,
}

/// HTTPIngressRuleValue is a list of http selectors pointing to backends.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct HTTPIngressRuleValue {
    /// A collection of paths that map requests to backends.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub paths: Vec<HTTPIngressPath>,
}

/// HTTPIngressPath associates a path with a backend.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct HTTPIngressPath {
    /// Path is matched against the path of an incoming request.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub path: String,

    /// PathType determines the interpretation of the Path matching.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path_type: Option<PathType>,

    /// Backend defines the referenced service endpoint.
    #[serde(default)]
    pub backend: IngressBackend,
}

/// PathType represents the type of path referred to by a HTTPIngressPath.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "PascalCase")]
pub enum PathType {
    /// Matches the URL path exactly.
    Exact,
    /// Matches based on a URL path prefix split by '/'.
    Prefix,
    /// Matching is up to the IngressClass.
    ImplementationSpecific,
}

/// IngressBackend describes all endpoints for a given service and port.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct IngressBackend {
    /// Specifies the name of the referenced service.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub service_name: String,

    /// Specifies the port of the referenced service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_port: Option<IntOrString>,

    /// Resource is an ObjectRef to another Kubernetes resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<TypedLocalObjectReference>,
}

/// IngressList is a collection of Ingress.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct IngressList {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// Items is the list of Ingress.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Ingress>,
}

// ============================================================================
// NetworkPolicy Types (DEPRECATED)
// ============================================================================

/// NetworkPolicy describes what network traffic is allowed for a set of Pods.
///
/// **DEPRECATED** - This group version of NetworkPolicy is deprecated by networking/v1.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPolicy {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// Specification of the desired behavior.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<NetworkPolicySpec>,
}

/// NetworkPolicySpec describes the desired behavior for a NetworkPolicy.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPolicySpec {
    /// Selects the pods to which this NetworkPolicy object applies.
    #[serde(default)]
    pub pod_selector: LabelSelector,

    /// List of ingress rules to be applied to the selected pods.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ingress: Vec<NetworkPolicyIngressRule>,

    /// List of egress rules to be applied to the selected pods.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub egress: Vec<NetworkPolicyEgressRule>,

    /// List of rule types that the NetworkPolicy relates to.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policy_types: Vec<PolicyType>,
}

/// PolicyType describes the NetworkPolicy type.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "PascalCase")]
pub enum PolicyType {
    /// Affects ingress traffic on selected pods.
    Ingress,
    /// Affects egress traffic on selected pods.
    Egress,
}

/// NetworkPolicyIngressRule matches traffic if and only if the traffic matches both ports AND from.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPolicyIngressRule {
    /// List of ports which should be made accessible.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<NetworkPolicyPort>,

    /// List of sources which should be able to access the pods.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub from_: Vec<NetworkPolicyPeer>,
}

/// NetworkPolicyEgressRule describes a particular set of traffic that is allowed out.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPolicyEgressRule {
    /// List of destination ports for outgoing traffic.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<NetworkPolicyPort>,

    /// List of destinations for outgoing traffic.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub to: Vec<NetworkPolicyPeer>,
}

/// NetworkPolicyPort describes a port on which the policy applies.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPolicyPort {
    /// The protocol (TCP, UDP, or SCTP) which traffic must match.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<Protocol>,

    /// The port on the given protocol.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<IntOrString>,

    /// The end port in a range.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_port: Option<i32>,
}

/// NetworkPolicyPeer describes a peer to allow traffic from/to.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPolicyPeer {
    /// This is a label selector which selects Pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pod_selector: Option<LabelSelector>,

    /// Selects Namespaces using cluster-scoped labels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace_selector: Option<LabelSelector>,

    /// IPBlock defines policy on a particular IPBlock.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_block: Option<IPBlock>,
}

/// IPBlock describes a particular CIDR that is allowed to the pods.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct IPBlock {
    /// CIDR is a string representing the IP Block.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub cidr: String,

    /// Except is a slice of CIDRs that should not be included.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub except: Vec<String>,
}

/// NetworkPolicyList is a list of NetworkPolicy objects.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPolicyList {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// Items is a list of schema objects.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<NetworkPolicy>,
}

// ============================================================================
// Constants
// ============================================================================

/// DefaultDeploymentUniqueLabelKey is the default key of the selector that is added
/// to existing RCs (and label key that is added to its pods) to prevent the existing RCs
/// to select new pods (and old pods being select by new RC).
pub const DEFAULT_DEPLOYMENT_UNIQUE_LABEL_KEY: &str = "pod-template-hash";

/// DefaultDaemonSetUniqueLabelKey is the default label key that is added
/// to existing DaemonSet pods to distinguish between old and new DaemonSet pods.
pub const DEFAULT_DAEMON_SET_UNIQUE_LABEL_KEY: &str = "pod-template-hash";

/// DaemonSetTemplateGenerationKey is the key of the labels that is added
/// to daemon set pods to distinguish between old and new pod templates.
///
/// **DEPRECATED**: DefaultDaemonSetUniqueLabelKey is used instead.
pub const DAEMON_SET_TEMPLATE_GENERATION_KEY: &str = "pod-template-generation";

// ============================================================================
// Trait Implementations
// ============================================================================

use crate::common::{
    ApplyDefault, HasTypeMeta, ResourceSchema, UnimplementedConversion, VersionedObject,
};
use crate::impl_unimplemented_prost_message;

// ----------------------------------------------------------------------------
// Deployment
// ----------------------------------------------------------------------------

impl ResourceSchema for Deployment {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "extensions"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1beta1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "Deployment"
    }
    fn resource(_: &Self::Meta) -> &str {
        "deployments"
    }

    fn group_static() -> &'static str {
        "extensions"
    }
    fn version_static() -> &'static str {
        "v1beta1"
    }
    fn kind_static() -> &'static str {
        "Deployment"
    }
    fn resource_static() -> &'static str {
        "deployments"
    }
}

impl ResourceSchema for DeploymentList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "extensions"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1beta1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "DeploymentList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "deployments"
    }

    fn group_static() -> &'static str {
        "extensions"
    }
    fn version_static() -> &'static str {
        "v1beta1"
    }
    fn kind_static() -> &'static str {
        "DeploymentList"
    }
    fn resource_static() -> &'static str {
        "deployments"
    }
}

impl HasTypeMeta for Deployment {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for DeploymentList {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl VersionedObject for Deployment {
    fn metadata(&self) -> &ObjectMeta {
        self.metadata
            .as_ref()
            .unwrap_or_else(|| static_default_object_meta())
    }

    fn metadata_mut(&mut self) -> &mut ObjectMeta {
        self.metadata.get_or_insert_with(Default::default)
    }
}

impl ApplyDefault for Deployment {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "extensions/v1beta1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "Deployment".to_string();
        }
    }
}

impl ApplyDefault for DeploymentList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "extensions/v1beta1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "DeploymentList".to_string();
        }
    }
}

impl UnimplementedConversion for Deployment {}
impl_unimplemented_prost_message!(Deployment);
impl_unimplemented_prost_message!(DeploymentList);

// ----------------------------------------------------------------------------
// DaemonSet
// ----------------------------------------------------------------------------

impl ResourceSchema for DaemonSet {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "extensions"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1beta1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "DaemonSet"
    }
    fn resource(_: &Self::Meta) -> &str {
        "daemonsets"
    }

    fn group_static() -> &'static str {
        "extensions"
    }
    fn version_static() -> &'static str {
        "v1beta1"
    }
    fn kind_static() -> &'static str {
        "DaemonSet"
    }
    fn resource_static() -> &'static str {
        "daemonsets"
    }
}

impl ResourceSchema for DaemonSetList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "extensions"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1beta1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "DaemonSetList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "daemonsets"
    }

    fn group_static() -> &'static str {
        "extensions"
    }
    fn version_static() -> &'static str {
        "v1beta1"
    }
    fn kind_static() -> &'static str {
        "DaemonSetList"
    }
    fn resource_static() -> &'static str {
        "daemonsets"
    }
}

impl HasTypeMeta for DaemonSet {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for DaemonSetList {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl VersionedObject for DaemonSet {
    fn metadata(&self) -> &ObjectMeta {
        self.metadata
            .as_ref()
            .unwrap_or_else(|| static_default_object_meta())
    }

    fn metadata_mut(&mut self) -> &mut ObjectMeta {
        self.metadata.get_or_insert_with(Default::default)
    }
}

impl ApplyDefault for DaemonSet {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "extensions/v1beta1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "DaemonSet".to_string();
        }
    }
}

impl ApplyDefault for DaemonSetList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "extensions/v1beta1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "DaemonSetList".to_string();
        }
    }
}

impl UnimplementedConversion for DaemonSet {}
impl_unimplemented_prost_message!(DaemonSet);
impl_unimplemented_prost_message!(DaemonSetList);

// ----------------------------------------------------------------------------
// ReplicaSet
// ----------------------------------------------------------------------------

impl ResourceSchema for ReplicaSet {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "extensions"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1beta1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "ReplicaSet"
    }
    fn resource(_: &Self::Meta) -> &str {
        "replicasets"
    }

    fn group_static() -> &'static str {
        "extensions"
    }
    fn version_static() -> &'static str {
        "v1beta1"
    }
    fn kind_static() -> &'static str {
        "ReplicaSet"
    }
    fn resource_static() -> &'static str {
        "replicasets"
    }
}

impl ResourceSchema for ReplicaSetList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "extensions"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1beta1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "ReplicaSetList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "replicasets"
    }

    fn group_static() -> &'static str {
        "extensions"
    }
    fn version_static() -> &'static str {
        "v1beta1"
    }
    fn kind_static() -> &'static str {
        "ReplicaSetList"
    }
    fn resource_static() -> &'static str {
        "replicasets"
    }
}

impl HasTypeMeta for ReplicaSet {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for ReplicaSetList {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl VersionedObject for ReplicaSet {
    fn metadata(&self) -> &ObjectMeta {
        self.metadata
            .as_ref()
            .unwrap_or_else(|| static_default_object_meta())
    }

    fn metadata_mut(&mut self) -> &mut ObjectMeta {
        self.metadata.get_or_insert_with(Default::default)
    }
}

impl ApplyDefault for ReplicaSet {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "extensions/v1beta1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "ReplicaSet".to_string();
        }
    }
}

impl ApplyDefault for ReplicaSetList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "extensions/v1beta1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "ReplicaSetList".to_string();
        }
    }
}

impl UnimplementedConversion for ReplicaSet {}
impl_unimplemented_prost_message!(ReplicaSet);
impl_unimplemented_prost_message!(ReplicaSetList);

// ----------------------------------------------------------------------------
// NetworkPolicy
// ----------------------------------------------------------------------------

impl ResourceSchema for NetworkPolicy {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "extensions"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1beta1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "NetworkPolicy"
    }
    fn resource(_: &Self::Meta) -> &str {
        "networkpolicies"
    }

    fn group_static() -> &'static str {
        "extensions"
    }
    fn version_static() -> &'static str {
        "v1beta1"
    }
    fn kind_static() -> &'static str {
        "NetworkPolicy"
    }
    fn resource_static() -> &'static str {
        "networkpolicies"
    }
}

impl ResourceSchema for NetworkPolicyList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "extensions"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1beta1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "NetworkPolicyList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "networkpolicies"
    }

    fn group_static() -> &'static str {
        "extensions"
    }
    fn version_static() -> &'static str {
        "v1beta1"
    }
    fn kind_static() -> &'static str {
        "NetworkPolicyList"
    }
    fn resource_static() -> &'static str {
        "networkpolicies"
    }
}

impl HasTypeMeta for NetworkPolicy {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for NetworkPolicyList {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl VersionedObject for NetworkPolicy {
    fn metadata(&self) -> &ObjectMeta {
        self.metadata
            .as_ref()
            .unwrap_or_else(|| static_default_object_meta())
    }

    fn metadata_mut(&mut self) -> &mut ObjectMeta {
        self.metadata.get_or_insert_with(Default::default)
    }
}

impl ApplyDefault for NetworkPolicy {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "extensions/v1beta1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "NetworkPolicy".to_string();
        }
    }
}

impl ApplyDefault for NetworkPolicyList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "extensions/v1beta1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "NetworkPolicyList".to_string();
        }
    }
}

impl UnimplementedConversion for NetworkPolicy {}
impl_unimplemented_prost_message!(NetworkPolicy);
impl_unimplemented_prost_message!(NetworkPolicyList);

// ----------------------------------------------------------------------------
// Ingress
// ----------------------------------------------------------------------------

impl ResourceSchema for Ingress {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "extensions"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1beta1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "Ingress"
    }
    fn resource(_: &Self::Meta) -> &str {
        "ingresses"
    }

    fn group_static() -> &'static str {
        "extensions"
    }
    fn version_static() -> &'static str {
        "v1beta1"
    }
    fn kind_static() -> &'static str {
        "Ingress"
    }
    fn resource_static() -> &'static str {
        "ingresses"
    }
}

impl ResourceSchema for IngressList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "extensions"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1beta1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "IngressList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "ingresses"
    }

    fn group_static() -> &'static str {
        "extensions"
    }
    fn version_static() -> &'static str {
        "v1beta1"
    }
    fn kind_static() -> &'static str {
        "IngressList"
    }
    fn resource_static() -> &'static str {
        "ingresses"
    }
}

impl HasTypeMeta for Ingress {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for IngressList {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl VersionedObject for Ingress {
    fn metadata(&self) -> &ObjectMeta {
        self.metadata
            .as_ref()
            .unwrap_or_else(|| static_default_object_meta())
    }

    fn metadata_mut(&mut self) -> &mut ObjectMeta {
        self.metadata.get_or_insert_with(Default::default)
    }
}

impl ApplyDefault for Ingress {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "extensions/v1beta1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "Ingress".to_string();
        }
    }
}

impl ApplyDefault for IngressList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "extensions/v1beta1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "IngressList".to_string();
        }
    }
}

impl UnimplementedConversion for Ingress {}
impl_unimplemented_prost_message!(Ingress);
impl_unimplemented_prost_message!(IngressList);

// Helper function for static default ObjectMeta
fn static_default_object_meta() -> &'static ObjectMeta {
    use std::sync::OnceLock;
    static DEFAULT: OnceLock<ObjectMeta> = OnceLock::new();
    DEFAULT.get_or_init(ObjectMeta::default)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    // Scale tests
    #[test]
    fn test_scale_default() {
        let scale = Scale::default();
        assert!(scale.metadata.is_none());
        assert!(scale.spec.is_none());
    }

    #[test]
    fn test_scale_with_spec() {
        let scale = Scale {
            spec: Some(ScaleSpec {
                replicas: Some(3),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(scale.spec.as_ref().unwrap().replicas, Some(3));
    }

    #[test]
    fn test_scale_serialize() {
        let scale = Scale {
            spec: Some(ScaleSpec {
                replicas: Some(5),
                ..Default::default()
            }),
            ..Default::default()
        };
        let json = serde_json::to_string(&scale).unwrap();
        assert!(json.contains(r#""replicas":5"#));
    }

    #[test]
    fn test_scale_deserialize() {
        let json = r#"{"spec":{"replicas":3}}"#;
        let scale: Scale = serde_json::from_str(json).unwrap();
        assert_eq!(scale.spec.unwrap().replicas, Some(3));
    }

    // Deployment tests
    #[test]
    fn test_deployment_default() {
        let deployment = Deployment::default();
        assert!(deployment.metadata.is_none());
    }

    #[test]
    fn test_deployment_strategy_type_serialize() {
        assert_eq!(
            serde_json::to_string(&DeploymentStrategyType::Recreate).unwrap(),
            r#""Recreate""#
        );
        assert_eq!(
            serde_json::to_string(&DeploymentStrategyType::RollingUpdate).unwrap(),
            r#""RollingUpdate""#
        );
    }

    // DaemonSet tests
    #[test]
    fn test_daemon_set_default() {
        let daemon_set = DaemonSet::default();
        assert!(daemon_set.metadata.is_none());
    }

    // ReplicaSet tests
    #[test]
    fn test_replica_set_default() {
        let replica_set = ReplicaSet::default();
        assert!(replica_set.metadata.is_none());
    }

    // Ingress tests
    #[test]
    fn test_ingress_default() {
        let ingress = Ingress::default();
        assert!(ingress.metadata.is_none());
    }

    #[test]
    fn test_path_type_serialize() {
        assert_eq!(
            serde_json::to_string(&PathType::Exact).unwrap(),
            r#""Exact""#
        );
        assert_eq!(
            serde_json::to_string(&PathType::Prefix).unwrap(),
            r#""Prefix""#
        );
    }

    // NetworkPolicy tests
    #[test]
    fn test_network_policy_default() {
        let policy = NetworkPolicy::default();
        assert!(policy.metadata.is_none());
    }

    #[test]
    fn test_policy_type_serialize() {
        assert_eq!(
            serde_json::to_string(&PolicyType::Ingress).unwrap(),
            r#""Ingress""#
        );
        assert_eq!(
            serde_json::to_string(&PolicyType::Egress).unwrap(),
            r#""Egress""#
        );
    }

    // Constants tests
    #[test]
    fn test_constants() {
        assert_eq!(DEFAULT_DEPLOYMENT_UNIQUE_LABEL_KEY, "pod-template-hash");
        assert_eq!(DEFAULT_DAEMON_SET_UNIQUE_LABEL_KEY, "pod-template-hash");
        assert_eq!(
            DAEMON_SET_TEMPLATE_GENERATION_KEY,
            "pod-template-generation"
        );
    }
}
