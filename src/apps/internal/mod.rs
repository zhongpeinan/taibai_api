//! Kubernetes Apps API Internal Types
//!
//! This module contains type definitions from k8s-pkg/apis/apps/types.go
//! that are used internally by the Kubernetes API.
//!
//! Source: k8s.io/kubernetes/pkg/apis/apps

use crate::common::{IntOrString, LabelSelector, ObjectMeta, Timestamp, TypeMeta};
use crate::core::internal::{ConditionStatus, PersistentVolumeClaim, PodTemplateSpec};
use crate::impl_has_object_meta;
use serde::{Deserialize, Serialize};

// ============================================================================
// StatefulSet Related Types
// ============================================================================

/// PodManagementPolicyType defines the policy for creating pods under a stateful set.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum PodManagementPolicyType {
    /// OrderedReadyPodManagement will create pods in strictly increasing order on
    /// scale up and strictly decreasing order on scale down, progressing only when
    /// the previous pod is ready or terminated. At most one pod will be changed
    /// at any time.
    #[serde(rename = "OrderedReady")]
    #[default]
    OrderedReady,
    /// ParallelPodManagement will create and delete pods as soon as the stateful set
    /// replica count is changed, and will not wait for pods to be ready or complete
    /// termination.
    #[serde(rename = "Parallel")]
    Parallel,
}

mod as_str_ref_impls;
pub mod pod_management_policy_type {
    pub const ORDERED_READY: &str = "OrderedReady";
    pub const PARALLEL: &str = "Parallel";
}

/// StatefulSetUpdateStrategyType is a string enumeration type that enumerates
/// all possible update strategies for the StatefulSet controller.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum StatefulSetUpdateStrategyType {
    /// RollingUpdateStatefulSetStrategyType indicates that update will be
    /// applied to all Pods in the StatefulSet with respect to the StatefulSet
    /// ordering constraints.
    #[serde(rename = "RollingUpdate")]
    #[default]
    RollingUpdate,
    /// OnDeleteStatefulSetStrategyType triggers the legacy behavior. Version
    /// tracking and ordered rolling restarts are disabled.
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
    #[serde(default)]
    pub r#type: StatefulSetUpdateStrategyType,
    /// RollingUpdate is used to communicate parameters when Type is RollingUpdateStatefulSetStrategyType.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rolling_update: Option<RollingUpdateStatefulSetStrategy>,
}

/// RollingUpdateStatefulSetStrategy is used to communicate parameter for RollingUpdateStatefulSetStrategyType.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct RollingUpdateStatefulSetStrategy {
    /// Partition indicates the ordinal at which the StatefulSet should be partitioned
    /// for updates.
    pub partition: i32,
    /// The maximum number of pods that can be unavailable during the update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_unavailable: Option<IntOrString>,
}

/// PersistentVolumeClaimRetentionPolicyType is a string enumeration of the policies that will determine
/// when volumes from the VolumeClaimTemplates will be deleted.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum PersistentVolumeClaimRetentionPolicyType {
    /// RetainPersistentVolumeClaimRetentionPolicyType is the default
    /// PersistentVolumeClaimRetentionPolicy and specifies that
    /// PersistentVolumeClaims associated with StatefulSet VolumeClaimTemplates
    /// will not be deleted.
    #[serde(rename = "Retain")]
    #[default]
    Retain,
    /// DeletePersistentVolumeClaimRetentionPolicyType specifies that
    /// PersistentVolumeClaims associated with StatefulSet VolumeClaimTemplates
    /// will be deleted in the scenario specified.
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
    /// WhenDeleted specifies what happens to PVCs created from StatefulSet
    /// VolumeClaimTemplates when the StatefulSet is deleted.
    #[serde(default)]
    pub when_deleted: PersistentVolumeClaimRetentionPolicyType,
    /// WhenScaled specifies what happens to PVCs created from StatefulSet
    /// VolumeClaimTemplates when the StatefulSet is scaled down.
    #[serde(default)]
    pub when_scaled: PersistentVolumeClaimRetentionPolicyType,
}

/// StatefulSetOrdinals describes the policy used for replica ordinal assignment
/// in this StatefulSet.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StatefulSetOrdinals {
    /// start is the number representing the first replica's index.
    #[serde(default)]
    pub start: i32,
}

/// StatefulSet represents a set of pods with consistent identities.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct StatefulSet {
    /// Standard type metadata.
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    pub metadata: ObjectMeta,
    /// Spec defines the desired identities of pods in this set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<StatefulSetSpec>,
    /// Status is the current status of Pods in this StatefulSet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<StatefulSetStatus>,
}
impl_has_object_meta!(StatefulSet);

/// A StatefulSetSpec is the specification of a StatefulSet.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StatefulSetSpec {
    /// Replicas is the desired number of replicas of the given Template.
    #[serde(default)]
    pub replicas: i32,
    /// Selector is a label query over pods that should match the replica count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<LabelSelector>,
    /// Template is the object that describes the pod that will be created.
    pub template: PodTemplateSpec,
    /// VolumeClaimTemplates is a list of claims that pods are allowed to reference.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub volume_claim_templates: Vec<PersistentVolumeClaim>,
    /// ServiceName is the name of the service that governs this StatefulSet.
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub service_name: String,
    /// PodManagementPolicy controls how pods are created during initial scale up.
    #[serde(default)]
    pub pod_management_policy: PodManagementPolicyType,
    /// updateStrategy indicates the StatefulSetUpdateStrategy that will be employed.
    #[serde(default)]
    pub update_strategy: StatefulSetUpdateStrategy,
    /// revisionHistoryLimit is the maximum number of revisions that will be maintained.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_history_limit: Option<i32>,
    /// Minimum number of seconds for which a newly created pod should be ready.
    #[serde(default)]
    pub min_ready_seconds: i32,
    /// PersistentVolumeClaimRetentionPolicy describes the policy used for PVCs.
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
    /// readyReplicas is the number of Pods created with a Ready Condition.
    #[serde(default)]
    pub ready_replicas: i32,
    /// currentReplicas is the number of Pods created from the currentRevision.
    #[serde(default)]
    pub current_replicas: i32,
    /// updatedReplicas is the number of Pods created from the updateRevision.
    #[serde(default)]
    pub updated_replicas: i32,
    /// currentRevision indicates the version used to generate Pods in [0,currentReplicas).
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub current_revision: String,
    /// updateRevision indicates the version used to generate Pods in [replicas-updatedReplicas,replicas).
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

/// StatefulSetConditionType describes the condition types of StatefulSets.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum StatefulSetConditionType {
    // TODO: Add valid condition types for StatefulSets when they are defined
    #[serde(rename = "")]
    Unknown,
}

/// StatefulSetCondition describes the state of a statefulset at a certain point.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct StatefulSetCondition {
    /// Type of statefulset condition.
    pub r#type: StatefulSetConditionType,
    /// Status of the condition, one of True, False, Unknown.
    pub status: ConditionStatus,
    /// The last time this condition was updated.
    pub last_transition_time: Timestamp,
    /// The reason for the condition's last transition.
    pub reason: String,
    /// A human readable message indicating details about the transition.
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
    pub metadata: ObjectMeta,
    /// Items is the list of stateful sets.
    #[serde(default)]
    pub items: Vec<StatefulSet>,
}

// ============================================================================
// ControllerRevision Related Types
// ============================================================================

/// ControllerRevision implements an immutable snapshot of state data.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ControllerRevision {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    pub metadata: ObjectMeta,
    /// Data is the serialized representation of the state.
    pub data: serde_json::Value,
    /// Revision indicates the revision of the state represented by Data.
    pub revision: i64,
}
impl_has_object_meta!(ControllerRevision);

impl Default for ControllerRevision {
    fn default() -> Self {
        Self {
            type_meta: TypeMeta::default(),
            metadata: ObjectMeta::default(),
            data: serde_json::Value::Null,
            revision: 0,
        }
    }
}

/// ControllerRevisionList is a resource containing a list of ControllerRevision objects.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ControllerRevisionList {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    pub metadata: ObjectMeta,
    /// Items is the list of ControllerRevision objects.
    #[serde(default)]
    pub items: Vec<ControllerRevision>,
}

// ============================================================================
// Deployment Related Types
// ============================================================================

/// DeploymentStrategyType defines strategies with a deployment.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum DeploymentStrategyType {
    /// RecreateDeploymentStrategyType - kill all existing pods before creating new ones.
    #[serde(rename = "Recreate")]
    Recreate,
    /// RollingUpdateDeploymentStrategyType - Replace the old RCs by new one using rolling update.
    #[serde(rename = "RollingUpdate")]
    #[default]
    RollingUpdate,
}

pub mod deployment_strategy_type {
    pub const RECREATE: &str = "Recreate";
    pub const ROLLING_UPDATE: &str = "RollingUpdate";
}

/// DeploymentStrategy stores information about the strategy and rolling-update behavior.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentStrategy {
    /// Type of deployment.
    #[serde(default)]
    pub r#type: DeploymentStrategyType,
    /// Rolling update config params.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rolling_update: Option<RollingUpdateDeployment>,
}

/// RollingUpdateDeployment is the spec to control the desired behavior of rolling update.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RollingUpdateDeployment {
    /// The maximum number of pods that can be unavailable during the update.
    pub max_unavailable: IntOrString,
    /// The maximum number of pods that can be scheduled above the original number of pods.
    pub max_surge: IntOrString,
}

impl Default for RollingUpdateDeployment {
    fn default() -> Self {
        Self {
            max_unavailable: IntOrString::Int(1),
            max_surge: IntOrString::Int(1),
        }
    }
}

/// RollbackConfig specifies the state of a revision to roll back to.
/// DEPRECATED.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RollbackConfig {
    /// The revision to rollback to. If set to 0, rollback to the last revision.
    #[serde(default)]
    pub revision: i64,
}

/// Deployment provides declarative updates for Pods and ReplicaSets.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct Deployment {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    pub metadata: ObjectMeta,
    /// Specification of the desired behavior of the Deployment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<DeploymentSpec>,
    /// Most recently observed status of the Deployment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DeploymentStatus>,
}
impl_has_object_meta!(Deployment);

/// DeploymentSpec specifies the state of a Deployment.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentSpec {
    /// Number of desired pods.
    #[serde(default)]
    pub replicas: i32,
    /// Label selector for pods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<LabelSelector>,
    /// Template describes the pods that will be created.
    pub template: PodTemplateSpec,
    /// The deployment strategy to use to replace existing pods with new ones.
    #[serde(default)]
    pub strategy: DeploymentStrategy,
    /// Minimum number of seconds for which a newly created pod should be ready.
    #[serde(default)]
    pub min_ready_seconds: i32,
    /// The number of old ReplicaSets to retain to allow rollback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_history_limit: Option<i32>,
    /// Indicates that the deployment is paused.
    #[serde(default)]
    pub paused: bool,
    /// DEPRECATED. The config this deployment is rolling back to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback_to: Option<RollbackConfig>,
    /// The maximum time in seconds for a deployment to make progress before it is considered failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_deadline_seconds: Option<i32>,
}

/// DeploymentStatus holds information about the observed status of a deployment.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentStatus {
    /// The generation observed by the deployment controller.
    #[serde(default)]
    pub observed_generation: i64,
    /// Total number of non-terminating pods targeted by this deployment.
    #[serde(default)]
    pub replicas: i32,
    /// Total number of non-terminating pods that have the desired template spec.
    #[serde(default)]
    pub updated_replicas: i32,
    /// Total number of non-terminating pods with a Ready Condition.
    #[serde(default)]
    pub ready_replicas: i32,
    /// Total number of available non-terminating pods targeted by this deployment.
    #[serde(default)]
    pub available_replicas: i32,
    /// Total number of unavailable pods targeted by this deployment.
    #[serde(default)]
    pub unavailable_replicas: i32,
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

/// DeploymentConditionType defines conditions of a deployment.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum DeploymentConditionType {
    /// Available means the deployment is available.
    #[serde(rename = "Available")]
    Available,
    /// Progressing means the deployment is progressing.
    #[serde(rename = "Progressing")]
    Progressing,
    /// ReplicaFailure is added when one of its pods fails to be created or deleted.
    #[serde(rename = "ReplicaFailure")]
    ReplicaFailure,
}

pub mod deployment_condition_type {
    pub const AVAILABLE: &str = "Available";
    pub const PROGRESSING: &str = "Progressing";
    pub const REPLICA_FAILURE: &str = "ReplicaFailure";
}

/// DeploymentCondition describes the state of a deployment at a certain point.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentCondition {
    /// Type of deployment condition.
    pub r#type: DeploymentConditionType,
    /// Status of the condition, one of True, False, Unknown.
    pub status: ConditionStatus,
    /// The last time this condition was updated.
    pub last_update_time: Timestamp,
    /// Last time the condition transitioned from one status to another.
    pub last_transition_time: Timestamp,
    /// The reason for the condition's last transition.
    pub reason: String,
    /// A human readable message indicating details about the transition.
    pub message: String,
}

/// DeploymentRollback stores the information required to rollback a deployment.
/// DEPRECATED.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct DeploymentRollback {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Required: This must match the Name of a deployment.
    pub name: String,
    /// The annotations to be updated to a deployment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_annotations: Option<std::collections::BTreeMap<String, String>>,
    /// The config of this deployment rollback.
    pub rollback_to: RollbackConfig,
}

/// DeploymentList defines multiple deployments.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentList {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    pub metadata: ObjectMeta,
    /// Items is the list of deployments.
    #[serde(default)]
    pub items: Vec<Deployment>,
}

// ============================================================================
// DaemonSet Related Types
// ============================================================================

/// DaemonSetUpdateStrategyType is a strategy according to which a daemon set gets updated.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum DaemonSetUpdateStrategyType {
    /// RollingUpdateDaemonSetStrategyType - Replace the old daemons by new ones using rolling update.
    #[serde(rename = "RollingUpdate")]
    #[default]
    RollingUpdate,
    /// OnDeleteDaemonSetStrategyType - Replace the old daemons only when it's killed.
    #[serde(rename = "OnDelete")]
    OnDelete,
}

pub mod daemon_set_update_strategy_type {
    pub const ROLLING_UPDATE: &str = "RollingUpdate";
    pub const ON_DELETE: &str = "OnDelete";
}

/// DaemonSetUpdateStrategy defines a strategy to update a daemon set.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DaemonSetUpdateStrategy {
    /// Type of daemon set update.
    #[serde(default)]
    pub r#type: DaemonSetUpdateStrategyType,
    /// Rolling update config params.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rolling_update: Option<RollingUpdateDaemonSet>,
}

/// RollingUpdateDaemonSet is the spec to control the desired behavior of daemon set rolling update.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RollingUpdateDaemonSet {
    /// The maximum number of DaemonSet pods that can be unavailable during the update.
    pub max_unavailable: IntOrString,
    /// The maximum number of nodes with an existing available DaemonSet pod that can have an updated pod.
    pub max_surge: IntOrString,
}

impl Default for RollingUpdateDaemonSet {
    fn default() -> Self {
        Self {
            max_unavailable: IntOrString::Int(1),
            max_surge: IntOrString::Int(0),
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
    pub template: PodTemplateSpec,
    /// An update strategy to replace existing DaemonSet pods with new pods.
    #[serde(default)]
    pub update_strategy: DaemonSetUpdateStrategy,
    /// The minimum number of seconds for which a newly created DaemonSet pod should be ready.
    #[serde(default)]
    pub min_ready_seconds: i32,
    /// DEPRECATED. A sequence number representing a specific generation of the template.
    #[serde(default)]
    pub template_generation: i64,
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
    /// The number of nodes that should be running the daemon pod and have one or more running and ready.
    #[serde(default)]
    pub number_ready: i32,
    /// The most recent generation observed by the daemon set controller.
    #[serde(default)]
    pub observed_generation: i64,
    /// The total number of nodes that are running updated daemon pod.
    #[serde(default)]
    pub updated_number_scheduled: i32,
    /// The number of nodes that should be running the daemon pod and have one or more running and available.
    #[serde(default)]
    pub number_available: i32,
    /// The number of nodes that should be running the daemon pod and have none running and available.
    #[serde(default)]
    pub number_unavailable: i32,
    /// Count of hash collisions for the DaemonSet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collision_count: Option<i32>,
    /// Represents the latest available observations of a DaemonSet's current state.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub conditions: Vec<DaemonSetCondition>,
}

/// DaemonSetConditionType defines a daemon set condition.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum DaemonSetConditionType {
    // TODO: Add valid condition types of a DaemonSet when defined
    #[serde(rename = "")]
    Unknown,
}

/// DaemonSetCondition describes the state of a DaemonSet at a certain point.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DaemonSetCondition {
    /// Type of DaemonSet condition.
    pub r#type: DaemonSetConditionType,
    /// Status of the condition, one of True, False, Unknown.
    pub status: ConditionStatus,
    /// Last time the condition transitioned from one status to another.
    pub last_transition_time: Timestamp,
    /// The reason for the condition's last transition.
    pub reason: String,
    /// A human readable message indicating details about the transition.
    pub message: String,
}

/// DaemonSet represents the configuration of a daemon set.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct DaemonSet {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    pub metadata: ObjectMeta,
    /// The desired behavior of this daemon set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<DaemonSetSpec>,
    /// The current status of this daemon set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DaemonSetStatus>,
}
impl_has_object_meta!(DaemonSet);

/// DaemonSetList is a collection of daemon sets.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DaemonSetList {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    pub metadata: ObjectMeta,
    /// A list of daemon sets.
    #[serde(default)]
    pub items: Vec<DaemonSet>,
}

// ============================================================================
// ReplicaSet Related Types
// ============================================================================

/// ReplicaSet ensures that a specified number of pod replicas are running at any given time.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct ReplicaSet {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    pub metadata: ObjectMeta,
    /// Spec defines the desired behavior of this ReplicaSet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<ReplicaSetSpec>,
    /// Status is the current status of this ReplicaSet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ReplicaSetStatus>,
}
impl_has_object_meta!(ReplicaSet);

/// ReplicaSetSpec is the specification of a ReplicaSet.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReplicaSetSpec {
    /// Replicas is the number of desired replicas.
    #[serde(default)]
    pub replicas: i32,
    /// Minimum number of seconds for which a newly created pod should be ready.
    #[serde(default)]
    pub min_ready_seconds: i32,
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
    #[serde(default)]
    pub fully_labeled_replicas: i32,
    /// The number of non-terminating pods with a Ready Condition.
    #[serde(default)]
    pub ready_replicas: i32,
    /// The number of available non-terminating pods for this replica set.
    #[serde(default)]
    pub available_replicas: i32,
    /// The number of terminating pods for this replica set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminating_replicas: Option<i32>,
    /// ObservedGeneration reflects the generation of the most recently observed ReplicaSet.
    #[serde(default)]
    pub observed_generation: i64,
    /// Represents the latest available observations of a replica set's current state.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub conditions: Vec<ReplicaSetCondition>,
}

/// ReplicaSetConditionType is a condition of a replica set.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum ReplicaSetConditionType {
    /// ReplicaSetReplicaFailure is added when one of its pods fails to be created or deleted.
    #[serde(rename = "ReplicaFailure")]
    ReplicaFailure,
}

pub mod replica_set_condition_type {
    pub const REPLICA_FAILURE: &str = "ReplicaFailure";
}

/// ReplicaSetCondition describes the state of a replica set at a certain point.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ReplicaSetCondition {
    /// Type of replica set condition.
    pub r#type: ReplicaSetConditionType,
    /// Status of the condition, one of True, False, Unknown.
    pub status: ConditionStatus,
    /// The last time the condition transitioned from one status to another.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<Timestamp>,
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
    pub metadata: ObjectMeta,
    /// Items is the list of ReplicaSets.
    #[serde(default)]
    pub items: Vec<ReplicaSet>,
}

// ============================================================================
// Constants
// ============================================================================

/// DefaultDeploymentUniqueLabelKey is the default key of the selector that is added
/// to existing RCs (and label key that is added to its pods) to prevent the existing RCs
/// to select new pods (and old pods being select by new RC).
pub const DEFAULT_DEPLOYMENT_UNIQUE_LABEL_KEY: &str = "pod-template-hash";

/// DaemonSetTemplateGenerationKey is the key of the labels that is added
/// to daemon set pods to distinguish between old and new pod templates.
/// DEPRECATED: DefaultDaemonSetUniqueLabelKey is used instead.
pub const DAEMON_SET_TEMPLATE_GENERATION_KEY: &str = "pod-template-generation";

// Validation helpers for internal apps types
pub mod validation;

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {}
