//! Kubernetes Batch API Internal Types
//!
//! This module contains type definitions from k8s.io/kubernetes/pkg/apis/batch/types.go
//! that are used internally by the Kubernetes API.
//!
//! Source: https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/batch/types.go

use crate::common::{LabelSelector, ListMeta, ObjectMeta, Timestamp, TypeMeta};
use crate::core::internal::{ConditionStatus, PodConditionType};
use crate::core::v1::{ObjectReference, PodTemplateSpec};
use crate::impl_has_object_meta;
use serde::{Deserialize, Serialize};

// ============================================================================
// Job Related Constants
// ============================================================================

pub mod label_prefix {
    pub const LABEL_PREFIX: &str = "batch.kubernetes.io/";

    pub const JOB_TRACKING_FINALIZER: &str = "batch.kubernetes.io/job-tracking";
    pub const JOB_NAME_LABEL: &str = "batch.kubernetes.io/job-name";
    pub const CONTROLLER_UID_LABEL: &str = "batch.kubernetes.io/controller-uid";
    pub const JOB_INDEX_FAILURE_COUNT_ANNOTATION: &str =
        "batch.kubernetes.io/job-index-failure-count";
    pub const JOB_INDEX_IGNORED_FAILURE_COUNT_ANNOTATION: &str =
        "batch.kubernetes.io/job-index-ignored-failure-count";
}

pub mod legacy_labels {
    pub const LEGACY_JOB_NAME_LABEL: &str = "job-name";
    pub const LEGACY_CONTROLLER_UID_LABEL: &str = "controller-uid";
}

pub mod controller_name {
    pub const JOB_CONTROLLER_NAME: &str = "kubernetes.io/job-controller";
}

// ============================================================================
// CompletionMode Enums
// ============================================================================

/// CompletionMode specifies how Pod completions of a Job are tracked.
///
/// Source: https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/batch/types.go#L107
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum CompletionMode {
    /// NonIndexed completion mode - Job completes when there have been .spec.completions
    /// successfully completed Pods.
    #[serde(rename = "NonIndexed")]
    #[default]
    NonIndexed,
    /// Indexed completion mode - Each Pod gets a completion index from 0 to (.spec.completions - 1).
    #[serde(rename = "Indexed")]
    Indexed,
}

pub mod completion_mode {
    pub const NON_INDEXED: &str = "NonIndexed";
    pub const INDEXED: &str = "Indexed";
}

// ============================================================================
// PodFailurePolicy Enums
// ============================================================================

/// PodFailurePolicyAction specifies how a Pod failure is handled.
///
/// Source: https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/batch/types.go#L123
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum PodFailurePolicyAction {
    /// Mark the pod's job as Failed and terminate all running pods.
    #[serde(rename = "FailJob")]
    #[default]
    FailJob,
    /// Mark the Job's index as failed to avoid restarts within this index.
    #[serde(rename = "FailIndex")]
    FailIndex,
    /// Don't increment the backoffLimit counter and create a replacement pod.
    #[serde(rename = "Ignore")]
    Ignore,
    /// Handle the pod failure in the default way - increment the backoffLimit counter.
    #[serde(rename = "Count")]
    Count,
}

pub mod pod_failure_policy_action {
    pub const FAIL_JOB: &str = "FailJob";
    pub const FAIL_INDEX: &str = "FailIndex";
    pub const IGNORE: &str = "Ignore";
    pub const COUNT: &str = "Count";
}

/// PodFailurePolicyOnExitCodesOperator specifies the operator for exit code checking.
///
/// Source: https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/batch/types.go#L148
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum PodFailurePolicyOnExitCodesOperator {
    /// Exit code is in the set of specified values.
    #[serde(rename = "In")]
    #[default]
    In,
    /// Exit code is not in the set of specified values.
    #[serde(rename = "NotIn")]
    NotIn,
}

pub mod pod_failure_policy_on_exit_codes_operator {
    pub const IN: &str = "In";
    pub const NOT_IN: &str = "NotIn";
}

// ============================================================================
// PodReplacementPolicy Enums
// ============================================================================

/// PodReplacementPolicy specifies the policy for creating pod replacements.
///
/// Source: https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/batch/types.go#L156
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum PodReplacementPolicy {
    /// Recreate pods when they are terminating or failed.
    #[serde(rename = "TerminatingOrFailed")]
    TerminatingOrFailed,
    /// Wait until a previously created Pod is fully terminated before creating a replacement.
    #[serde(rename = "Failed")]
    Failed,
}

pub mod pod_replacement_policy {
    pub const TERMINATING_OR_FAILED: &str = "TerminatingOrFailed";
    pub const FAILED: &str = "Failed";
}

// ============================================================================
// JobConditionType Enums
// ============================================================================

/// JobConditionType is a valid value for JobCondition.Type.
///
/// Source: https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/batch/types.go#L600
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum JobConditionType {
    /// JobSuspended means the job has been suspended.
    #[serde(rename = "Suspended")]
    Suspended,
    /// JobComplete means the job has completed its execution.
    #[serde(rename = "Complete")]
    Complete,
    /// JobFailed means the job has failed its execution.
    #[serde(rename = "Failed")]
    Failed,
    /// FailureTarget means the job is about to fail its execution.
    #[serde(rename = "FailureTarget")]
    FailureTarget,
    /// JobSuccessCriteriaMet means the Job has reached a success state.
    #[serde(rename = "SuccessCriteriaMet")]
    SuccessCriteriaMet,
}

pub mod job_condition_type {
    pub const SUSPENDED: &str = "Suspended";
    pub const COMPLETE: &str = "Complete";
    pub const FAILED: &str = "Failed";
    pub const FAILURE_TARGET: &str = "FailureTarget";
    pub const SUCCESS_CRITERIA_MET: &str = "SuccessCriteriaMet";
}

// ============================================================================
// JobCondition
// ============================================================================

/// JobCondition describes current state of a job.
///
/// Source: https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/batch/types.go#L617
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct JobCondition {
    /// Type of job condition.
    #[serde(rename = "type")]
    pub type_: JobConditionType,
    /// Status of the condition, one of True, False, Unknown.
    pub status: ConditionStatus,
    /// Last time the condition was checked.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_probe_time: Option<crate::common::Timestamp>,
    /// Last time the condition transit from one status to another.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<crate::common::Timestamp>,
    /// (brief) reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    /// Human readable message indicating details about last transition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
}

// ============================================================================
// PodFailurePolicyOnPodConditionsPattern
// ============================================================================

/// PodFailurePolicyOnPodConditionsPattern describes a pattern for matching a pod condition.
///
/// Source: https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/batch/types.go#L206
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PodFailurePolicyOnPodConditionsPattern {
    /// Specifies the required Pod condition type.
    pub type_: PodConditionType,
    /// Specifies the required Pod condition status. Defaults to True.
    #[serde(default)]
    pub status: ConditionStatus,
}

// ============================================================================
// Job Types
// ============================================================================

/// Job represents the configuration of a single job (internal version).
///
/// Source: https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/batch/types.go#L62
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Job {
    /// Standard type metadata (not serialized in internal version).
    #[serde(skip)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    pub metadata: ObjectMeta,
    /// Specification of the desired behavior of a job.
    pub spec: JobSpec,
    /// Current status of a job.
    pub status: JobStatus,
}
impl_has_object_meta!(Job);

/// JobList is a collection of jobs (internal version).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct JobList {
    /// Standard type metadata (not serialized in internal version).
    #[serde(skip)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    pub metadata: ListMeta,
    /// items is the list of Jobs.
    pub items: Vec<Job>,
}

/// JobSpec describes how the job execution will look like (internal version).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct JobSpec {
    /// Specifies the maximum desired number of pods the job should run at any given time.
    pub parallelism: Option<i32>,
    /// Specifies the desired number of successfully finished pods the job should be run with.
    pub completions: Option<i32>,
    /// Specifies the duration in seconds relative to the startTime that the job may be active.
    pub active_deadline_seconds: Option<i64>,
    /// Specifies the policy of handling failed pods.
    pub pod_failure_policy: Option<PodFailurePolicy>,
    /// successPolicy specifies the policy when the Job can be declared as succeeded.
    pub success_policy: Option<SuccessPolicy>,
    /// Specifies the number of retries before marking this job failed.
    pub backoff_limit: Option<i32>,
    /// Specifies the limit for the number of retries within an index.
    pub backoff_limit_per_index: Option<i32>,
    /// Specifies the maximal number of failed indexes before marking the Job as failed.
    pub max_failed_indexes: Option<i32>,
    /// A label query over pods that should match the pod count.
    pub selector: Option<LabelSelector>,
    /// manualSelector controls generation of pod labels and pod selectors.
    pub manual_selector: Option<bool>,
    /// Describes the pod that will be created when executing a job.
    pub template: PodTemplateSpec,
    /// ttlSecondsAfterFinished limits the lifetime of a Job that has finished.
    pub ttl_seconds_after_finished: Option<i32>,
    /// completionMode specifies how Pod completions are tracked.
    pub completion_mode: Option<CompletionMode>,
    /// suspend specifies whether the Job controller should create Pods or not.
    pub suspend: Option<bool>,
    /// podReplacementPolicy specifies when to create replacement Pods.
    pub pod_replacement_policy: Option<PodReplacementPolicy>,
    /// ManagedBy field indicates the controller that manages a Job.
    pub managed_by: Option<String>,
}

/// JobStatus represents the current state of a Job (internal version).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct JobStatus {
    /// The latest available observations of an object's current state.
    pub conditions: Vec<JobCondition>,
    /// Represents time when the job controller started processing a job.
    pub start_time: Option<Timestamp>,
    /// Represents time when the job was completed.
    pub completion_time: Option<Timestamp>,
    /// The number of pending and running pods.
    pub active: Option<i32>,
    /// The number of pods which are terminating.
    pub terminating: Option<i32>,
    /// The number of active pods which have a Ready condition.
    pub ready: Option<i32>,
    /// The number of pods which reached phase Succeeded.
    pub succeeded: Option<i32>,
    /// The number of pods which reached phase Failed.
    pub failed: Option<i32>,
    /// completedIndexes holds the completed indexes when completionMode is "Indexed".
    pub completed_indexes: String,
    /// FailedIndexes holds the failed indexes when backoffLimitPerIndex is set.
    pub failed_indexes: Option<String>,
    /// uncountedTerminatedPods holds UIDs of Pods that have terminated but haven't been accounted.
    pub uncounted_terminated_pods: Option<UncountedTerminatedPods>,
}

/// UncountedTerminatedPods holds UIDs of Pods that have terminated but haven't been accounted.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct UncountedTerminatedPods {
    /// succeeded holds UIDs of succeeded Pods.
    pub succeeded: Vec<String>,
    /// failed holds UIDs of failed Pods.
    pub failed: Vec<String>,
}

/// JobTemplateSpec describes the data a Job should have when created from a template (internal).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct JobTemplateSpec {
    /// Standard object's metadata of the jobs created from this template.
    pub metadata: ObjectMeta,
    /// Specification of the desired behavior of the job.
    pub spec: JobSpec,
}
impl_has_object_meta!(JobTemplateSpec);

/// PodFailurePolicyOnExitCodesRequirement describes the requirement for handling a failed pod.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodFailurePolicyOnExitCodesRequirement {
    /// Restricts the check for exit codes to the container with the specified name.
    pub container_name: Option<String>,
    /// Represents the relationship between the container exit code(s) and the specified values.
    pub operator: PodFailurePolicyOnExitCodesOperator,
    /// Specifies the set of values.
    pub values: Vec<i32>,
}

/// PodFailurePolicyRule describes how a pod failure is handled when the requirements are met.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodFailurePolicyRule {
    /// Specifies the action taken on a pod failure when the requirements are satisfied.
    pub action: PodFailurePolicyAction,
    /// Represents the requirement on the container exit codes.
    pub on_exit_codes: Option<PodFailurePolicyOnExitCodesRequirement>,
    /// Represents the requirement on the pod conditions.
    pub on_pod_conditions: Vec<PodFailurePolicyOnPodConditionsPattern>,
}

/// PodFailurePolicy describes how failed pods influence the backoffLimit.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodFailurePolicy {
    /// A list of pod failure policy rules.
    pub rules: Vec<PodFailurePolicyRule>,
}

/// SuccessPolicy describes when a Job can be declared as succeeded.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SuccessPolicy {
    /// rules represents the list of alternative rules for declaring the Jobs as successful.
    pub rules: Vec<SuccessPolicyRule>,
}

/// SuccessPolicyRule describes rule for declaring a Job as succeeded.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SuccessPolicyRule {
    /// succeededIndexes specifies the set of indexes which need to be contained.
    pub succeeded_indexes: Option<String>,
    /// succeededCount specifies the minimal required size of the actual set of succeeded indexes.
    pub succeeded_count: Option<i32>,
}

// ============================================================================
// CronJob Types
// ============================================================================

/// CronJob represents the configuration of a single cron job (internal version).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CronJob {
    /// Standard type metadata (not serialized in internal version).
    #[serde(skip)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    pub metadata: ObjectMeta,
    /// Specification of the desired behavior of a cron job.
    pub spec: CronJobSpec,
    /// Current status of a cron job.
    pub status: CronJobStatus,
}
impl_has_object_meta!(CronJob);

/// CronJobList is a collection of cron jobs (internal version).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CronJobList {
    /// Standard type metadata (not serialized in internal version).
    #[serde(skip)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    pub metadata: ListMeta,
    /// items is the list of CronJobs.
    pub items: Vec<CronJob>,
}

/// CronJobSpec describes how the job execution will look like and when it will run (internal).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CronJobSpec {
    /// The schedule in Cron format.
    pub schedule: String,
    /// The time zone name for the given schedule.
    pub time_zone: Option<String>,
    /// Optional deadline in seconds for starting the job if it misses scheduled time.
    pub starting_deadline_seconds: Option<i64>,
    /// Specifies how to treat concurrent executions of a Job.
    pub concurrency_policy: ConcurrencyPolicy,
    /// This flag tells the controller to suspend subsequent executions.
    pub suspend: Option<bool>,
    /// Specifies the job that will be created when executing a CronJob.
    pub job_template: JobTemplateSpec,
    /// The number of successful finished jobs to retain.
    pub successful_jobs_history_limit: Option<i32>,
    /// The number of failed finished jobs to retain.
    pub failed_jobs_history_limit: Option<i32>,
}

/// CronJobStatus represents the current state of a cron job (internal).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CronJobStatus {
    /// A list of pointers to currently running jobs.
    pub active: Vec<ObjectReference>,
    /// Information when was the last time the job was successfully scheduled.
    pub last_schedule_time: Option<Timestamp>,
    /// Information when was the last time the job successfully completed.
    pub last_successful_time: Option<Timestamp>,
}

/// ConcurrencyPolicy describes how the job will be handled.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum ConcurrencyPolicy {
    /// Allows CronJobs to run concurrently.
    #[serde(rename = "Allow")]
    #[default]
    Allow,
    /// Forbids concurrent runs, skipping next run if previous hasn't finished.
    #[serde(rename = "Forbid")]
    Forbid,
    /// Cancels currently running job and replaces it with a new one.
    #[serde(rename = "Replace")]
    Replace,
}

pub mod concurrency_policy {
    pub const ALLOW: &str = "Allow";
    pub const FORBID: &str = "Forbid";
    pub const REPLACE: &str = "Replace";
}

// Validation helpers for internal batch types
pub mod validation;

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {}

// AsRefStr / AsRef<str> implementations for enums
crate::impl_as_str_ref!(CompletionMode, {
    NonIndexed => "NonIndexed",
    Indexed => "Indexed",
});

crate::impl_as_str_ref!(PodFailurePolicyAction, {
    FailJob => "FailJob",
    FailIndex => "FailIndex",
    Ignore => "Ignore",
    Count => "Count",
});

crate::impl_as_str_ref!(PodFailurePolicyOnExitCodesOperator, {
    In => "In",
    NotIn => "NotIn",
});

crate::impl_as_str_ref!(PodReplacementPolicy, {
    TerminatingOrFailed => "TerminatingOrFailed",
    Failed => "Failed",
});

crate::impl_as_str_ref!(JobConditionType, {
    Suspended => "Suspended",
    Complete => "Complete",
    Failed => "Failed",
    FailureTarget => "FailureTarget",
    SuccessCriteriaMet => "SuccessCriteriaMet",
});

crate::impl_as_str_ref!(ConcurrencyPolicy, {
    Allow => "Allow",
    Forbid => "Forbid",
    Replace => "Replace",
});
