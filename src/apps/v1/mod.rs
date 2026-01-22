//! Kubernetes Apps v1 API types
//!
//! This module contains types from the Kubernetes apps/v1 API group.

use crate::common::{IntOrString, LabelSelector, ListMeta, ObjectMeta, TypeMeta};
use crate::core::v1::{PersistentVolumeClaim, PodTemplateSpec};
use crate::impl_versioned_object;
use serde::{Deserialize, Serialize};

// ============================================================================
// StatefulSet Related Types
// ============================================================================

/// PodManagementPolicyType defines the policy for creating pods under a stateful set.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum PodManagementPolicyType {
    /// OrderedReadyPodManagement will create pods in strictly increasing order on
    /// scale up and strictly decreasing order on scale down.
    #[serde(rename = "OrderedReady")]
    #[default]
    OrderedReady,
    /// ParallelPodManagement will create and delete pods as soon as the stateful set
    /// replica count is changed.
    #[serde(rename = "Parallel")]
    Parallel,
}

pub mod pod_management_policy_type {
    pub const ORDERED_READY: &str = "OrderedReady";
    pub const PARALLEL: &str = "Parallel";
}

/// StatefulSetUpdateStrategyType is a string enumeration type that enumerates
/// all possible update strategies for the StatefulSet controller.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum StatefulSetUpdateStrategyType {
    /// RollingUpdateStatefulSetStrategyType indicates that update will be applied
    /// to all Pods in the StatefulSet with respect to the StatefulSet ordering constraints.
    #[serde(rename = "RollingUpdate")]
    #[default]
    RollingUpdate,
    /// OnDeleteStatefulSetStrategyType triggers the legacy behavior.
    #[serde(rename = "OnDelete")]
    OnDelete,
}

pub mod stateful_set_update_strategy_type {
    pub const ROLLING_UPDATE: &str = "RollingUpdate";
    pub const ON_DELETE: &str = "OnDelete";
}

/// StatefulSetUpdateStrategy indicates the strategy that the StatefulSet
/// controller will use to perform updates.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StatefulSetUpdateStrategy {
    /// Type indicates the type of the StatefulSetUpdateStrategy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<StatefulSetUpdateStrategyType>,
    /// RollingUpdate is used to communicate parameters when Type is RollingUpdateStatefulSetStrategyType.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rolling_update: Option<RollingUpdateStatefulSetStrategy>,
}

/// RollingUpdateStatefulSetStrategy is used to communicate parameter for RollingUpdateStatefulSetStrategyType.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RollingUpdateStatefulSetStrategy {
    /// Partition indicates the ordinal at which the StatefulSet should be partitioned for updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition: Option<i32>,
    /// The maximum number of pods that can be unavailable during the update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_unavailable: Option<IntOrString>,
}

impl Default for RollingUpdateStatefulSetStrategy {
    fn default() -> Self {
        Self {
            partition: Some(0),
            max_unavailable: None,
        }
    }
}

/// PersistentVolumeClaimRetentionPolicyType is a string enumeration of the policies that will determine
/// when volumes from the VolumeClaimTemplates will be deleted.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum PersistentVolumeClaimRetentionPolicyType {
    /// RetainPersistentVolumeClaimRetentionPolicyType causes PVCs to not be deleted.
    #[serde(rename = "Retain")]
    #[default]
    Retain,
    /// DeletePersistentVolumeClaimRetentionPolicyType causes PVCs to be deleted.
    #[serde(rename = "Delete")]
    Delete,
}

pub mod persistent_volume_claim_retention_policy_type {
    pub const RETAIN: &str = "Retain";
    pub const DELETE: &str = "Delete";
}

/// StatefulSetPersistentVolumeClaimRetentionPolicy describes the policy used for PVCs
/// created from the StatefulSet VolumeClaimTemplates.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StatefulSetPersistentVolumeClaimRetentionPolicy {
    /// WhenDeleted specifies what happens to PVCs created from StatefulSet VolumeClaimTemplates
    /// when the StatefulSet is deleted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub when_deleted: Option<PersistentVolumeClaimRetentionPolicyType>,
    /// WhenScaled specifies what happens to PVCs created from StatefulSet VolumeClaimTemplates
    /// when the StatefulSet is scaled down.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub when_scaled: Option<PersistentVolumeClaimRetentionPolicyType>,
}

/// StatefulSetOrdinals describes the policy used for replica ordinal assignment in this StatefulSet.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StatefulSetOrdinals {
    /// start is the number representing the first replica's index.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,
}

/// StatefulSet represents a set of pods with consistent identities.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StatefulSet {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// Spec defines the desired identities of pods in this set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<StatefulSetSpec>,
    /// Status is the current status of Pods in this StatefulSet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<StatefulSetStatus>,
}
impl_versioned_object!(StatefulSet);

/// A StatefulSetSpec is the specification of a StatefulSet.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StatefulSetSpec {
    /// replicas is the desired number of replicas of the given Template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    /// selector is a label query over pods that should match the replica count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<LabelSelector>,
    /// template is the object that describes the pod that will be created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<PodTemplateSpec>,
    /// volumeClaimTemplates is a list of claims that pods are allowed to reference.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub volume_claim_templates: Vec<PersistentVolumeClaim>,
    /// serviceName is the name of the service that governs this StatefulSet.
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub service_name: String,
    /// podManagementPolicy controls how pods are created during initial scale up.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pod_management_policy: Option<PodManagementPolicyType>,
    /// updateStrategy indicates the StatefulSetUpdateStrategy that will be employed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_strategy: Option<StatefulSetUpdateStrategy>,
    /// revisionHistoryLimit is the maximum number of revisions that will be maintained.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_history_limit: Option<i32>,
    /// Minimum number of seconds for which a newly created pod should be ready.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_ready_seconds: Option<i32>,
    /// persistentVolumeClaimRetentionPolicy describes the lifecycle of persistent volume claims.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_volume_claim_retention_policy:
        Option<StatefulSetPersistentVolumeClaimRetentionPolicy>,
    /// ordinals controls the numbering of replica indices in a StatefulSet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinals: Option<StatefulSetOrdinals>,
}

/// StatefulSetStatus represents the current state of a StatefulSet.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StatefulSetStatus {
    /// observedGeneration is the most recent generation observed for this StatefulSet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
    /// replicas is the number of Pods created by the StatefulSet controller.
    #[serde(default)]
    pub replicas: i32,
    /// readyReplicas is the number of pods created for this StatefulSet with a Ready Condition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ready_replicas: Option<i32>,
    /// currentReplicas is the number of Pods created from the StatefulSet version indicated by currentRevision.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_replicas: Option<i32>,
    /// updatedReplicas is the number of Pods created from the StatefulSet version indicated by updateRevision.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_replicas: Option<i32>,
    /// currentRevision, if not empty, indicates the version of the StatefulSet used to generate Pods.
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub current_revision: String,
    /// updateRevision, if not empty, indicates the version of the StatefulSet used to generate Pods.
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub update_revision: String,
    /// collisionCount is the count of hash collisions for the StatefulSet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collision_count: Option<i32>,
    /// Represents the latest available observations of a statefulset's current state.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub conditions: Vec<StatefulSetCondition>,
    /// Total number of available pods targeted by this statefulset.
    #[serde(default)]
    pub available_replicas: i32,
}

/// StatefulSetConditionType describes the condition of a stateful set.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum StatefulSetConditionType {
    // StatefulSetCondition types are not predefined in the v1 API
    #[serde(rename = "")]
    #[default]
    Unknown,
}

/// StatefulSetCondition describes the state of a statefulset at a certain point.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StatefulSetCondition {
    /// Type of statefulset condition.
    pub r#type: StatefulSetConditionType,
    /// Status of the condition, one of True, False, Unknown.
    pub status: String,
    /// Last time the condition transitioned from one status to another.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<String>,
    /// The reason for the condition's last transition.
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub reason: String,
    /// A human readable message indicating details about the transition.
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub message: String,
}

/// StatefulSetList is a collection of StatefulSets.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StatefulSetList {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list's metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,
    /// Items is the list of stateful sets.
    #[serde(default)]
    pub items: Vec<StatefulSet>,
}

// ============================================================================
// Deployment Related Types
// ============================================================================

/// DeploymentStrategyType defines the strategy for a deployment.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum DeploymentStrategyType {
    /// Recreate - kill all existing pods before creating new ones.
    #[serde(rename = "Recreate")]
    Recreate,
    /// RollingUpdate - replace the old ReplicaSets by new one using rolling update.
    #[serde(rename = "RollingUpdate")]
    #[default]
    RollingUpdate,
}

pub mod deployment_strategy_type {
    pub const RECREATE: &str = "Recreate";
    pub const ROLLING_UPDATE: &str = "RollingUpdate";
}

/// DeploymentStrategy describes how to replace existing pods with new ones.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentStrategy {
    /// Type of deployment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<DeploymentStrategyType>,
    /// Rolling update config params. Present only if DeploymentStrategyType = RollingUpdate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rolling_update: Option<RollingUpdateDeployment>,
}

/// RollingUpdateDeployment specifies the parameters for a rolling update.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RollingUpdateDeployment {
    /// The maximum number of pods that can be unavailable during the update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_unavailable: Option<IntOrString>,
    /// The maximum number of pods that can be scheduled above the desired number of pods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_surge: Option<IntOrString>,
}

impl Default for RollingUpdateDeployment {
    fn default() -> Self {
        Self {
            max_unavailable: Some(IntOrString::String("25%".to_string())),
            max_surge: Some(IntOrString::String("25%".to_string())),
        }
    }
}

/// Deployment enables declarative updates for Pods and ReplicaSets.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Deployment {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// Specification of the desired behavior of the Deployment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<DeploymentSpec>,
    /// Most recently observed status of the Deployment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DeploymentStatus>,
}
impl_versioned_object!(Deployment);

/// DeploymentSpec is the specification of the desired behavior of the Deployment.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentSpec {
    /// Number of desired pods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    /// Label selector for pods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<LabelSelector>,
    /// Template describes the pods that will be created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<PodTemplateSpec>,
    /// The deployment strategy to use to replace existing pods with new ones.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strategy: Option<DeploymentStrategy>,
    /// Minimum number of seconds for which a newly created pod should be ready.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_ready_seconds: Option<i32>,
    /// The number of old ReplicaSets to retain to allow rollback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_history_limit: Option<i32>,
    /// Indicates that the deployment is paused.
    #[serde(default)]
    pub paused: bool,
    /// The maximum time in seconds for a deployment to make progress before it is considered to be failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_deadline_seconds: Option<i32>,
}

/// DeploymentStatus is the most recently observed status of the Deployment.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentStatus {
    /// The generation observed by the deployment controller.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
    /// Total number of non-terminating pods targeted by this deployment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    /// Total number of non-terminating pods targeted by this deployment that have the desired template spec.
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminating_replicas: Option<i32>,
    /// Represents the latest available observations of a deployment's current state.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub conditions: Vec<DeploymentCondition>,
    /// Count of hash collisions for the Deployment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collision_count: Option<i32>,
}

/// DeploymentConditionType defines valid conditions of a deployment.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum DeploymentConditionType {
    /// Available means the deployment is available.
    #[serde(rename = "Available")]
    #[default]
    Available,
    /// Progressing means the deployment is progressing.
    #[serde(rename = "Progressing")]
    Progressing,
    /// ReplicaFailure is added in a deployment when one of its pods fails to be created or deleted.
    #[serde(rename = "ReplicaFailure")]
    ReplicaFailure,
}

pub mod deployment_condition_type {
    pub const AVAILABLE: &str = "Available";
    pub const PROGRESSING: &str = "Progressing";
    pub const REPLICA_FAILURE: &str = "ReplicaFailure";
}

/// DeploymentCondition describes the state of a deployment at a certain point.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentCondition {
    /// Type of deployment condition.
    pub r#type: DeploymentConditionType,
    /// Status of the condition, one of True, False, Unknown.
    pub status: String,
    /// The last time this condition was updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<String>,
    /// Last time the condition transitioned from one status to another.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<String>,
    /// The reason for the condition's last transition.
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub reason: String,
    /// A human readable message indicating details about the transition.
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub message: String,
}

/// DeploymentList is a list of Deployments.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentList {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,
    /// Items is the list of Deployments.
    #[serde(default)]
    pub items: Vec<Deployment>,
}

// ============================================================================
// DaemonSet Related Types
// ============================================================================

/// DaemonSetUpdateStrategyType defines the strategy type for a daemon set update.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum DaemonSetUpdateStrategyType {
    /// RollingUpdate - replace the old daemons by new ones using rolling update.
    #[serde(rename = "RollingUpdate")]
    #[default]
    RollingUpdate,
    /// OnDelete - replace the old daemons only when it's killed.
    #[serde(rename = "OnDelete")]
    OnDelete,
}

pub mod daemon_set_update_strategy_type {
    pub const ROLLING_UPDATE: &str = "RollingUpdate";
    pub const ON_DELETE: &str = "OnDelete";
}

/// DaemonSetUpdateStrategy is a struct used to control the update strategy for a DaemonSet.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DaemonSetUpdateStrategy {
    /// Type of daemon set update.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<DaemonSetUpdateStrategyType>,
    /// Rolling update config params. Present only if type = "RollingUpdate".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rolling_update: Option<RollingUpdateDaemonSet>,
}

/// RollingUpdateDaemonSet is the spec to control the desired behavior of daemon set rolling update.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RollingUpdateDaemonSet {
    /// The maximum number of DaemonSet pods that can be unavailable during the update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_unavailable: Option<IntOrString>,
    /// The maximum number of nodes with an existing available DaemonSet pod that can have an updated pod.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_surge: Option<IntOrString>,
}

impl Default for RollingUpdateDaemonSet {
    fn default() -> Self {
        Self {
            max_unavailable: Some(IntOrString::Int(1)),
            max_surge: None,
        }
    }
}

/// DaemonSetSpec is the specification of a daemon set.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DaemonSetSpec {
    /// A label query over pods that are managed by the daemon set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<LabelSelector>,
    /// An object that describes the pod that will be created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<PodTemplateSpec>,
    /// An update strategy to replace existing DaemonSet pods with new pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_strategy: Option<DaemonSetUpdateStrategy>,
    /// The minimum number of seconds for which a newly created DaemonSet pod should be ready.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_ready_seconds: Option<i32>,
    /// The number of old history to retain to allow rollback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_history_limit: Option<i32>,
}

/// DaemonSetStatus represents the current status of a daemon set.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DaemonSetStatus {
    /// The number of nodes that are running at least 1 daemon pod and are supposed to run the daemon pod.
    #[serde(default)]
    pub current_number_scheduled: i32,
    /// The number of nodes that are running the daemon pod, but are not supposed to run the daemon pod.
    #[serde(default)]
    pub number_misscheduled: i32,
    /// The total number of nodes that should be running the daemon pod.
    #[serde(default)]
    pub desired_number_scheduled: i32,
    /// numberReady is the number of nodes that should be running the daemon pod and have one or more of the daemon pod running.
    #[serde(default)]
    pub number_ready: i32,
    /// The most recent generation observed by the daemon set controller.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
    /// The total number of nodes that are running updated daemon pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_number_scheduled: Option<i32>,
    /// The number of nodes that should be running the daemon pod and have one or more of the daemon pod running and available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_available: Option<i32>,
    /// The number of nodes that should be running the daemon pod and have none of the daemon pod running and available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_unavailable: Option<i32>,
    /// Count of hash collisions for the DaemonSet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collision_count: Option<i32>,
    /// Represents the latest available observations of a DaemonSet's current state.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub conditions: Vec<DaemonSetCondition>,
}

/// DaemonSetConditionType defines the condition type of a DaemonSet.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum DaemonSetConditionType {
    // DaemonSetCondition types are not predefined in the v1 API
    #[serde(rename = "")]
    #[default]
    Unknown,
}

/// DaemonSetCondition describes the state of a DaemonSet at a certain point.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DaemonSetCondition {
    /// Type of DaemonSet condition.
    pub r#type: DaemonSetConditionType,
    /// Status of the condition, one of True, False, Unknown.
    pub status: String,
    /// Last time the condition transitioned from one status to another.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<String>,
    /// The reason for the condition's last transition.
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub reason: String,
    /// A human readable message indicating details about the transition.
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub message: String,
}

/// DaemonSet represents the configuration of a daemon set.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DaemonSet {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// The desired behavior of this daemon set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<DaemonSetSpec>,
    /// The current status of this daemon set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DaemonSetStatus>,
}
impl_versioned_object!(DaemonSet);

/// DaemonSetList is a collection of daemon sets.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DaemonSetList {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,
    /// A list of daemon sets.
    #[serde(default)]
    pub items: Vec<DaemonSet>,
}

// ============================================================================
// ReplicaSet Related Types
// ============================================================================

/// ReplicaSet ensures that a specified number of pod replicas are running at any given time.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReplicaSet {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// Spec defines the specification of the desired behavior of the ReplicaSet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<ReplicaSetSpec>,
    /// Status is the most recently observed status of the ReplicaSet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ReplicaSetStatus>,
}
impl_versioned_object!(ReplicaSet);

/// ReplicaSetSpec is the specification of a ReplicaSet.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReplicaSetSpec {
    /// Replicas is the number of desired pods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    /// Minimum number of seconds for which a newly created pod should be ready.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_ready_seconds: Option<i32>,
    /// Selector is a label query over pods that should match the replica count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<LabelSelector>,
    /// Template is the object that describes the pod that will be created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<PodTemplateSpec>,
}

/// ReplicaSetStatus represents the current status of a ReplicaSet.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReplicaSetStatus {
    /// Replicas is the most recently observed number of non-terminating pods.
    #[serde(default)]
    pub replicas: i32,
    /// The number of non-terminating pods that have labels matching the labels of the pod template.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fully_labeled_replicas: Option<i32>,
    /// The number of non-terminating pods targeted by this ReplicaSet with a Ready Condition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ready_replicas: Option<i32>,
    /// The number of available non-terminating pods for this replica set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub available_replicas: Option<i32>,
    /// The number of terminating pods for this replica set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminating_replicas: Option<i32>,
    /// ObservedGeneration reflects the generation of the most recently observed ReplicaSet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
    /// Represents the latest available observations of a replica set's current state.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub conditions: Vec<ReplicaSetCondition>,
}

/// ReplicaSetConditionType defines valid conditions of a replica set.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum ReplicaSetConditionType {
    /// ReplicaSetReplicaFailure is added in a replica set when one of its pods fails to be created or deleted.
    #[serde(rename = "ReplicaFailure")]
    #[default]
    ReplicaFailure,
}

pub mod replica_set_condition_type {
    pub const REPLICA_FAILURE: &str = "ReplicaFailure";
}

/// ReplicaSetCondition describes the state of a replica set at a certain point.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReplicaSetCondition {
    /// Type of replica set condition.
    pub r#type: ReplicaSetConditionType,
    /// Status of the condition, one of True, False, Unknown.
    pub status: String,
    /// The last time the condition transitioned from one status to another.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<String>,
    /// The reason for the condition's last transition.
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub reason: String,
    /// A human readable message indicating details about the transition.
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub message: String,
}

/// ReplicaSetList is a collection of ReplicaSets.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReplicaSetList {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,
    /// List of ReplicaSets.
    #[serde(default)]
    pub items: Vec<ReplicaSet>,
}

// ============================================================================
// ControllerRevision Related Types
// ============================================================================

/// ControllerRevision implements an immutable snapshot of state data.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ControllerRevision {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// Data is the serialized representation of the state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    /// Revision indicates the revision of the state represented by Data.
    #[serde(default)]
    pub revision: i64,
}
impl_versioned_object!(ControllerRevision);

/// ControllerRevisionList is a resource containing a list of ControllerRevision objects.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ControllerRevisionList {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,
    /// Items is the list of ControllerRevisions.
    #[serde(default)]
    pub items: Vec<ControllerRevision>,
}

// ============================================================================
// Constants
// ============================================================================

/// DefaultDeploymentUniqueLabelKey is the default key of the selector that is added
/// to existing ReplicaSets to prevent the existing ReplicaSets to select new pods.
pub const DEFAULT_DEPLOYMENT_UNIQUE_LABEL_KEY: &str = "pod-template-hash";

/// DefaultDaemonSetUniqueLabelKey is the default label key that is added
/// to existing DaemonSet pods to distinguish between old and new DaemonSet pods.
pub const DEFAULT_DAEMON_SET_UNIQUE_LABEL_KEY: &str = "controller-revision-hash";

// ============================================================================
// Trait Implementations
// ============================================================================

use crate::common::{ApplyDefault, HasTypeMeta, ResourceSchema, UnimplementedConversion};
use crate::impl_unimplemented_prost_message;

// ----------------------------------------------------------------------------
// StatefulSet
// ----------------------------------------------------------------------------

impl ResourceSchema for StatefulSet {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "apps"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "StatefulSet"
    }
    fn resource(_: &Self::Meta) -> &str {
        "statefulsets"
    }

    fn group_static() -> &'static str {
        "apps"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "StatefulSet"
    }
    fn resource_static() -> &'static str {
        "statefulsets"
    }
}

impl ResourceSchema for StatefulSetList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "apps"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "StatefulSetList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "statefulsets"
    }

    fn group_static() -> &'static str {
        "apps"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "StatefulSetList"
    }
    fn resource_static() -> &'static str {
        "statefulsets"
    }
}

impl HasTypeMeta for StatefulSet {
    fn type_meta(&self) -> &crate::common::TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut crate::common::TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for StatefulSetList {
    fn type_meta(&self) -> &crate::common::TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut crate::common::TypeMeta {
        &mut self.type_meta
    }
}

impl ApplyDefault for StatefulSet {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "apps/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "StatefulSet".to_string();
        }
    }
}

impl ApplyDefault for StatefulSetList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "apps/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "StatefulSetList".to_string();
        }
    }
}

impl UnimplementedConversion for StatefulSet {}
impl_unimplemented_prost_message!(StatefulSet);
impl_unimplemented_prost_message!(StatefulSetList);

// ----------------------------------------------------------------------------
// Deployment
// ----------------------------------------------------------------------------

impl ResourceSchema for Deployment {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "apps"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "Deployment"
    }
    fn resource(_: &Self::Meta) -> &str {
        "deployments"
    }

    fn group_static() -> &'static str {
        "apps"
    }
    fn version_static() -> &'static str {
        "v1"
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
        "apps"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "DeploymentList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "deployments"
    }

    fn group_static() -> &'static str {
        "apps"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "DeploymentList"
    }
    fn resource_static() -> &'static str {
        "deployments"
    }
}

impl HasTypeMeta for Deployment {
    fn type_meta(&self) -> &crate::common::TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut crate::common::TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for DeploymentList {
    fn type_meta(&self) -> &crate::common::TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut crate::common::TypeMeta {
        &mut self.type_meta
    }
}

impl ApplyDefault for Deployment {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "apps/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "Deployment".to_string();
        }
    }
}

impl ApplyDefault for DeploymentList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "apps/v1".to_string();
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
        "apps"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "DaemonSet"
    }
    fn resource(_: &Self::Meta) -> &str {
        "daemonsets"
    }

    fn group_static() -> &'static str {
        "apps"
    }
    fn version_static() -> &'static str {
        "v1"
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
        "apps"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "DaemonSetList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "daemonsets"
    }

    fn group_static() -> &'static str {
        "apps"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "DaemonSetList"
    }
    fn resource_static() -> &'static str {
        "daemonsets"
    }
}

impl HasTypeMeta for DaemonSet {
    fn type_meta(&self) -> &crate::common::TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut crate::common::TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for DaemonSetList {
    fn type_meta(&self) -> &crate::common::TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut crate::common::TypeMeta {
        &mut self.type_meta
    }
}

impl ApplyDefault for DaemonSet {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "apps/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "DaemonSet".to_string();
        }
    }
}

impl ApplyDefault for DaemonSetList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "apps/v1".to_string();
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
        "apps"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "ReplicaSet"
    }
    fn resource(_: &Self::Meta) -> &str {
        "replicasets"
    }

    fn group_static() -> &'static str {
        "apps"
    }
    fn version_static() -> &'static str {
        "v1"
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
        "apps"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "ReplicaSetList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "replicasets"
    }

    fn group_static() -> &'static str {
        "apps"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "ReplicaSetList"
    }
    fn resource_static() -> &'static str {
        "replicasets"
    }
}

impl HasTypeMeta for ReplicaSet {
    fn type_meta(&self) -> &crate::common::TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut crate::common::TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for ReplicaSetList {
    fn type_meta(&self) -> &crate::common::TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut crate::common::TypeMeta {
        &mut self.type_meta
    }
}

impl ApplyDefault for ReplicaSet {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "apps/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "ReplicaSet".to_string();
        }
    }
}

impl ApplyDefault for ReplicaSetList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "apps/v1".to_string();
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
// ControllerRevision
// ----------------------------------------------------------------------------

impl ResourceSchema for ControllerRevision {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "apps"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "ControllerRevision"
    }
    fn resource(_: &Self::Meta) -> &str {
        "controllerrevisions"
    }

    fn group_static() -> &'static str {
        "apps"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "ControllerRevision"
    }
    fn resource_static() -> &'static str {
        "controllerrevisions"
    }
}

impl ResourceSchema for ControllerRevisionList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "apps"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "ControllerRevisionList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "controllerrevisions"
    }

    fn group_static() -> &'static str {
        "apps"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "ControllerRevisionList"
    }
    fn resource_static() -> &'static str {
        "controllerrevisions"
    }
}

impl HasTypeMeta for ControllerRevision {
    fn type_meta(&self) -> &crate::common::TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut crate::common::TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for ControllerRevisionList {
    fn type_meta(&self) -> &crate::common::TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut crate::common::TypeMeta {
        &mut self.type_meta
    }
}

impl ApplyDefault for ControllerRevision {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "apps/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "ControllerRevision".to_string();
        }
    }
}

impl ApplyDefault for ControllerRevisionList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "apps/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "ControllerRevisionList".to_string();
        }
    }
}

impl UnimplementedConversion for ControllerRevision {}
impl_unimplemented_prost_message!(ControllerRevision);
impl_unimplemented_prost_message!(ControllerRevisionList);

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // PodManagementPolicyType tests
    #[test]
    fn test_pod_management_policy_serialize() {
        assert_eq!(
            serde_json::to_string(&PodManagementPolicyType::OrderedReady).unwrap(),
            r#""OrderedReady""#
        );
    }

    #[test]
    fn test_pod_management_policy_deserialize() {
        assert_eq!(
            serde_json::from_str::<PodManagementPolicyType>(r#""OrderedReady""#).unwrap(),
            PodManagementPolicyType::OrderedReady
        );
    }

    #[test]
    fn test_pod_management_policy_constants() {
        assert_eq!(pod_management_policy_type::ORDERED_READY, "OrderedReady");
        assert_eq!(pod_management_policy_type::PARALLEL, "Parallel");
    }

    // StatefulSetUpdateStrategyType tests
    #[test]
    fn test_stateful_set_update_strategy_serialize() {
        assert_eq!(
            serde_json::to_string(&StatefulSetUpdateStrategyType::RollingUpdate).unwrap(),
            r#""RollingUpdate""#
        );
    }

    #[test]
    fn test_stateful_set_update_strategy_deserialize() {
        assert_eq!(
            serde_json::from_str::<StatefulSetUpdateStrategyType>(r#""RollingUpdate""#).unwrap(),
            StatefulSetUpdateStrategyType::RollingUpdate
        );
    }

    #[test]
    fn test_stateful_set_update_strategy_constants() {
        assert_eq!(
            stateful_set_update_strategy_type::ROLLING_UPDATE,
            "RollingUpdate"
        );
        assert_eq!(stateful_set_update_strategy_type::ON_DELETE, "OnDelete");
    }

    // DeploymentStrategyType tests
    #[test]
    fn test_deployment_strategy_serialize() {
        assert_eq!(
            serde_json::to_string(&DeploymentStrategyType::RollingUpdate).unwrap(),
            r#""RollingUpdate""#
        );
    }

    #[test]
    fn test_deployment_strategy_deserialize() {
        assert_eq!(
            serde_json::from_str::<DeploymentStrategyType>(r#""RollingUpdate""#).unwrap(),
            DeploymentStrategyType::RollingUpdate
        );
    }

    #[test]
    fn test_deployment_strategy_constants() {
        assert_eq!(deployment_strategy_type::RECREATE, "Recreate");
        assert_eq!(deployment_strategy_type::ROLLING_UPDATE, "RollingUpdate");
    }

    // DaemonSetUpdateStrategyType tests
    #[test]
    fn test_daemon_set_update_strategy_serialize() {
        assert_eq!(
            serde_json::to_string(&DaemonSetUpdateStrategyType::RollingUpdate).unwrap(),
            r#""RollingUpdate""#
        );
    }

    #[test]
    fn test_daemon_set_update_strategy_deserialize() {
        assert_eq!(
            serde_json::from_str::<DaemonSetUpdateStrategyType>(r#""RollingUpdate""#).unwrap(),
            DaemonSetUpdateStrategyType::RollingUpdate
        );
    }

    #[test]
    fn test_daemon_set_update_strategy_constants() {
        assert_eq!(
            daemon_set_update_strategy_type::ROLLING_UPDATE,
            "RollingUpdate"
        );
        assert_eq!(daemon_set_update_strategy_type::ON_DELETE, "OnDelete");
    }

    // ReplicaSetConditionType tests
    #[test]
    fn test_replica_set_condition_serialize() {
        assert_eq!(
            serde_json::to_string(&ReplicaSetConditionType::ReplicaFailure).unwrap(),
            r#""ReplicaFailure""#
        );
    }

    #[test]
    fn test_replica_set_condition_deserialize() {
        assert_eq!(
            serde_json::from_str::<ReplicaSetConditionType>(r#""ReplicaFailure""#).unwrap(),
            ReplicaSetConditionType::ReplicaFailure
        );
    }

    #[test]
    fn test_replica_set_condition_constants() {
        assert_eq!(
            replica_set_condition_type::REPLICA_FAILURE,
            "ReplicaFailure"
        );
    }

    // Constants tests
    #[test]
    fn test_deployment_constants() {
        assert_eq!(DEFAULT_DEPLOYMENT_UNIQUE_LABEL_KEY, "pod-template-hash");
    }

    #[test]
    fn test_daemon_set_constants() {
        assert_eq!(
            DEFAULT_DAEMON_SET_UNIQUE_LABEL_KEY,
            "controller-revision-hash"
        );
    }

    // StatefulSet default and round-trip tests
    #[test]
    fn test_stateful_set_default() {
        let ss = StatefulSet::default();
        assert!(ss.metadata.is_none());
        assert!(ss.spec.is_none());
        assert!(ss.status.is_none());
    }

    #[test]
    fn test_stateful_set_round_trip() {
        let original = StatefulSet {
            type_meta: TypeMeta {
                api_version: "apps/v1".to_string(),
                kind: "StatefulSet".to_string(),
            },
            metadata: None,
            spec: None,
            status: None,
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: StatefulSet = serde_json::from_str(&json).unwrap();
        assert_eq!(
            original.type_meta.api_version,
            deserialized.type_meta.api_version
        );
        assert_eq!(original.type_meta.kind, deserialized.type_meta.kind);
    }

    // Deployment default and round-trip tests
    #[test]
    fn test_deployment_default() {
        let d = Deployment::default();
        assert!(d.metadata.is_none());
        assert!(d.spec.is_none());
        assert!(d.status.is_none());
    }

    #[test]
    fn test_deployment_round_trip() {
        let original = Deployment {
            type_meta: TypeMeta {
                api_version: "apps/v1".to_string(),
                kind: "Deployment".to_string(),
            },
            metadata: None,
            spec: None,
            status: None,
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Deployment = serde_json::from_str(&json).unwrap();
        assert_eq!(
            original.type_meta.api_version,
            deserialized.type_meta.api_version
        );
        assert_eq!(original.type_meta.kind, deserialized.type_meta.kind);
    }

    // DaemonSet default and round-trip tests
    #[test]
    fn test_daemon_set_default() {
        let ds = DaemonSet::default();
        assert!(ds.metadata.is_none());
        assert!(ds.spec.is_none());
        assert!(ds.status.is_none());
    }

    #[test]
    fn test_daemon_set_round_trip() {
        let original = DaemonSet {
            type_meta: TypeMeta {
                api_version: "apps/v1".to_string(),
                kind: "DaemonSet".to_string(),
            },
            metadata: None,
            spec: None,
            status: None,
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: DaemonSet = serde_json::from_str(&json).unwrap();
        assert_eq!(
            original.type_meta.api_version,
            deserialized.type_meta.api_version
        );
        assert_eq!(original.type_meta.kind, deserialized.type_meta.kind);
    }

    // ReplicaSet default and round-trip tests
    #[test]
    fn test_replica_set_default() {
        let rs = ReplicaSet::default();
        assert!(rs.metadata.is_none());
        assert!(rs.spec.is_none());
        assert!(rs.status.is_none());
    }

    #[test]
    fn test_replica_set_round_trip() {
        let original = ReplicaSet {
            type_meta: TypeMeta {
                api_version: "apps/v1".to_string(),
                kind: "ReplicaSet".to_string(),
            },
            metadata: None,
            spec: None,
            status: None,
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: ReplicaSet = serde_json::from_str(&json).unwrap();
        assert_eq!(
            original.type_meta.api_version,
            deserialized.type_meta.api_version
        );
        assert_eq!(original.type_meta.kind, deserialized.type_meta.kind);
    }

    // ControllerRevision default and round-trip tests
    #[test]
    fn test_controller_revision_default() {
        let cr = ControllerRevision::default();
        assert!(cr.metadata.is_none());
        assert!(cr.data.is_none());
    }

    #[test]
    fn test_controller_revision_round_trip() {
        let original = ControllerRevision {
            type_meta: TypeMeta {
                api_version: "apps/v1".to_string(),
                kind: "ControllerRevision".to_string(),
            },
            metadata: None,
            data: None,
            revision: 1,
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: ControllerRevision = serde_json::from_str(&json).unwrap();
        assert_eq!(
            original.type_meta.api_version,
            deserialized.type_meta.api_version
        );
        assert_eq!(original.type_meta.kind, deserialized.type_meta.kind);
        assert_eq!(original.revision, deserialized.revision);
    }

    // List type tests
    #[test]
    fn test_stateful_set_list_default() {
        let list = StatefulSetList::default();
        assert!(list.metadata.is_none());
        assert!(list.items.is_empty());
    }

    #[test]
    fn test_deployment_list_default() {
        let list = DeploymentList::default();
        assert!(list.metadata.is_none());
        assert!(list.items.is_empty());
    }

    #[test]
    fn test_daemon_set_list_default() {
        let list = DaemonSetList::default();
        assert!(list.metadata.is_none());
        assert!(list.items.is_empty());
    }

    #[test]
    fn test_replica_set_list_default() {
        let list = ReplicaSetList::default();
        assert!(list.metadata.is_none());
        assert!(list.items.is_empty());
    }

    #[test]
    fn test_controller_revision_list_default() {
        let list = ControllerRevisionList::default();
        assert!(list.metadata.is_none());
        assert!(list.items.is_empty());
    }

    // Condition type constants tests
    #[test]
    fn test_deployment_condition_type_constants() {
        assert_eq!(deployment_condition_type::AVAILABLE, "Available");
        assert_eq!(deployment_condition_type::PROGRESSING, "Progressing");
        assert_eq!(deployment_condition_type::REPLICA_FAILURE, "ReplicaFailure");
    }
}
