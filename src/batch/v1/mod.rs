//! Kubernetes Batch v1 API types
//!
//! This module contains types from the Kubernetes batch/v1 API group.
//!
//! Source: https://github.com/kubernetes/api/blob/master/batch/v1/types.go

use crate::batch::internal::{
    CompletionMode, JobCondition, PodFailurePolicyAction, PodFailurePolicyOnExitCodesOperator,
    PodFailurePolicyOnPodConditionsPattern, PodReplacementPolicy,
};
use crate::common::{LabelSelector, ListMeta, ObjectMeta, TypeMeta};
use crate::core::v1::{ObjectReference, PodTemplateSpec};
use crate::impl_versioned_object;
use serde::{Deserialize, Serialize};

// ============================================================================
// Job Types
// ============================================================================

/// Job represents the configuration of a single job.
///
/// Source: https://github.com/kubernetes/api/blob/master/batch/v1/types.go#L68
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Job {
    /// Standard type metadata.
    #[serde(default)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// Specification of the desired behavior of a job.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<JobSpec>,
    /// Current status of a job.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<JobStatus>,
}

/// JobList is a collection of jobs.
///
/// Source: https://github.com/kubernetes/api/blob/master/batch/v1/types.go#L90
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct JobList {
    /// Standard type metadata.
    #[serde(default)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,
    /// items is the list of Jobs.
    #[serde(default)]
    pub items: Vec<Job>,
}

/// JobSpec describes how the job execution will look like.
///
/// Source: https://github.com/kubernetes/api/blob/master/batch/v1/types.go#L302
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct JobSpec {
    /// Specifies the maximum desired number of pods the job should run at any given time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parallelism: Option<i32>,
    /// Specifies the desired number of successfully finished pods the job should be run with.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completions: Option<i32>,
    /// Specifies the duration in seconds relative to the startTime that the job may be active.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_deadline_seconds: Option<i64>,
    /// Specifies the policy of handling failed pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pod_failure_policy: Option<PodFailurePolicy>,
    /// successPolicy specifies the policy when the Job can be declared as succeeded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub success_policy: Option<SuccessPolicy>,
    /// Specifies the number of retries before marking this job failed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backoff_limit: Option<i32>,
    /// Specifies the limit for the number of retries within an index.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backoff_limit_per_index: Option<i32>,
    /// Specifies the maximal number of failed indexes before marking the Job as failed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_failed_indexes: Option<i32>,
    /// A label query over pods that should match the pod count.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<LabelSelector>,
    /// manualSelector controls generation of pod labels and pod selectors.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manual_selector: Option<bool>,
    /// Describes the pod that will be created when executing a job.
    #[serde(default)]
    pub template: PodTemplateSpec,
    /// ttlSecondsAfterFinished limits the lifetime of a Job that has finished.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl_seconds_after_finished: Option<i32>,
    /// completionMode specifies how Pod completions are tracked.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completion_mode: Option<CompletionMode>,
    /// suspend specifies whether the Job controller should create Pods or not.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspend: Option<bool>,
    /// podReplacementPolicy specifies when to create replacement Pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pod_replacement_policy: Option<PodReplacementPolicy>,
    /// ManagedBy field indicates the controller that manages a Job.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
}

/// JobStatus represents the current state of a Job.
///
/// Source: https://github.com/kubernetes/api/blob/master/batch/v1/types.go#L476
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct JobStatus {
    /// The latest available observations of an object's current state.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<JobCondition>,
    /// Represents time when the job controller started processing a job.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<crate::common::Timestamp>,
    /// Represents time when the job was completed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completion_time: Option<crate::common::Timestamp>,
    /// The number of pending and running pods.
    #[serde(default)]
    pub active: i32,
    /// The number of pods which are terminating.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminating: Option<i32>,
    /// The number of active pods which have a Ready condition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ready: Option<i32>,
    /// The number of pods which reached phase Succeeded.
    #[serde(default)]
    pub succeeded: i32,
    /// The number of pods which reached phase Failed.
    #[serde(default)]
    pub failed: i32,
    /// completedIndexes holds the completed indexes when completionMode is "Indexed".
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub completed_indexes: String,
    /// FailedIndexes holds the failed indexes when backoffLimitPerIndex is set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed_indexes: Option<String>,
    /// uncountedTerminatedPods holds UIDs of Pods that have terminated but haven't been accounted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uncounted_terminated_pods: Option<UncountedTerminatedPods>,
}

/// UncountedTerminatedPods holds UIDs of Pods that have terminated but haven't been accounted.
///
/// Source: https://github.com/kubernetes/api/blob/master/batch/v1/types.go#L588
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct UncountedTerminatedPods {
    /// succeeded holds UIDs of succeeded Pods.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub succeeded: Vec<String>,
    /// failed holds UIDs of failed Pods.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub failed: Vec<String>,
}

/// JobTemplateSpec describes the data a Job should have when created from a template.
///
/// Source: https://github.com/kubernetes/api/blob/master/batch/v1/types.go#L662
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct JobTemplateSpec {
    /// Standard object's metadata of the jobs created from this template.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// Specification of the desired behavior of the job.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<JobSpec>,
}
impl_versioned_object!(JobTemplateSpec);

// ============================================================================
// Pod Failure Policy Types
// ============================================================================

/// PodFailurePolicyOnExitCodesRequirement describes the requirement for handling a failed pod.
///
/// Source: https://github.com/kubernetes/api/blob/master/batch/v1/types.go#L165
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodFailurePolicyOnExitCodesRequirement {
    /// Restricts the check for exit codes to the container with the specified name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    /// Represents the relationship between the container exit code(s) and the specified values.
    pub operator: PodFailurePolicyOnExitCodesOperator,
    /// Specifies the set of values.
    #[serde(default)]
    pub values: Vec<i32>,
}

/// PodFailurePolicyRule describes how a pod failure is handled when the requirements are met.
///
/// Source: https://github.com/kubernetes/api/blob/master/batch/v1/types.go#L216
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodFailurePolicyRule {
    /// Specifies the action taken on a pod failure when the requirements are satisfied.
    pub action: PodFailurePolicyAction,
    /// Represents the requirement on the container exit codes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_exit_codes: Option<PodFailurePolicyOnExitCodesRequirement>,
    /// Represents the requirement on the pod conditions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub on_pod_conditions: Vec<PodFailurePolicyOnPodConditionsPattern>,
}

/// PodFailurePolicy describes how failed pods influence the backoffLimit.
///
/// Source: https://github.com/kubernetes/api/blob/master/batch/v1/types.go#L246
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodFailurePolicy {
    /// A list of pod failure policy rules.
    #[serde(default)]
    pub rules: Vec<PodFailurePolicyRule>,
}

// ============================================================================
// Success Policy Types
// ============================================================================

/// SuccessPolicy describes when a Job can be declared as succeeded.
///
/// Source: https://github.com/kubernetes/api/blob/master/batch/v1/types.go#L257
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SuccessPolicy {
    /// rules represents the list of alternative rules for declaring the Jobs as successful.
    #[serde(default)]
    pub rules: Vec<SuccessPolicyRule>,
}

/// SuccessPolicyRule describes rule for declaring a Job as succeeded.
///
/// Source: https://github.com/kubernetes/api/blob/master/batch/v1/types.go#L269
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SuccessPolicyRule {
    /// succeededIndexes specifies the set of indexes which need to be contained.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub succeeded_indexes: Option<String>,
    /// succeededCount specifies the minimal required size of the actual set of succeeded indexes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub succeeded_count: Option<i32>,
}

// ============================================================================
// CronJob Types
// ============================================================================

/// CronJob represents the configuration of a single cron job.
///
/// Source: https://github.com/kubernetes/api/blob/master/batch/v1/types.go#L679
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CronJob {
    /// Standard type metadata.
    #[serde(default)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// Specification of the desired behavior of a cron job.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<CronJobSpec>,
    /// Current status of a cron job.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<CronJobStatus>,
}

/// CronJobList is a collection of cron jobs.
///
/// Source: https://github.com/kubernetes/api/blob/master/batch/v1/types.go#L701
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CronJobList {
    /// Standard type metadata.
    #[serde(default)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,
    /// items is the list of CronJobs.
    #[serde(default)]
    pub items: Vec<CronJob>,
}

/// CronJobSpec describes how the job execution will look like and when it will run.
///
/// Source: https://github.com/kubernetes/api/blob/master/batch/v1/types.go#L714
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CronJobSpec {
    /// The schedule in Cron format.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub schedule: String,
    /// The time zone name for the given schedule.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    /// Optional deadline in seconds for starting the job if it misses scheduled time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub starting_deadline_seconds: Option<i64>,
    /// Specifies how to treat concurrent executions of a Job.
    #[serde(default)]
    pub concurrency_policy: ConcurrencyPolicy,
    /// This flag tells the controller to suspend subsequent executions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspend: Option<bool>,
    /// Specifies the job that will be created when executing a CronJob.
    #[serde(default)]
    pub job_template: JobTemplateSpec,
    /// The number of successful finished jobs to retain.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub successful_jobs_history_limit: Option<i32>,
    /// The number of failed finished jobs to retain.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed_jobs_history_limit: Option<i32>,
}

/// CronJobStatus represents the current state of a cron job.
///
/// Source: https://github.com/kubernetes/api/blob/master/batch/v1/types.go#L784
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CronJobStatus {
    /// A list of pointers to currently running jobs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub active: Vec<ObjectReference>,
    /// Information when was the last time the job was successfully scheduled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_schedule_time: Option<crate::common::Timestamp>,
    /// Information when was the last time the job successfully completed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_successful_time: Option<crate::common::Timestamp>,
}

/// ConcurrencyPolicy describes how the job will be handled.
///
/// Source: https://github.com/kubernetes/api/blob/master/batch/v1/types.go#L769
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

// ============================================================================
// Constants
// ============================================================================

pub mod label_prefix {
    pub const LABEL_PREFIX: &str = "batch.kubernetes.io/";
    pub const CRONJOB_SCHEDULED_TIMESTAMP_ANNOTATION: &str =
        "batch.kubernetes.io/cronjob-scheduled-timestamp";
    pub const JOB_COMPLETION_INDEX_ANNOTATION: &str = "batch.kubernetes.io/job-completion-index";
    pub const JOB_TRACKING_FINALIZER: &str = "batch.kubernetes.io/job-tracking";
    pub const JOB_NAME_LABEL: &str = "batch.kubernetes.io/job-name";
    pub const CONTROLLER_UID_LABEL: &str = "batch.kubernetes.io/controller-uid";
    pub const JOB_INDEX_FAILURE_COUNT_ANNOTATION: &str =
        "batch.kubernetes.io/job-index-failure-count";
    pub const JOB_INDEX_IGNORED_FAILURE_COUNT_ANNOTATION: &str =
        "batch.kubernetes.io/job-index-ignored-failure-count";
}

pub mod controller_name {
    pub const JOB_CONTROLLER_NAME: &str = "kubernetes.io/job-controller";
}

pub mod job_reason {
    pub const POD_FAILURE_POLICY: &str = "PodFailurePolicy";
    pub const BACKOFF_LIMIT_EXCEEDED: &str = "BackoffLimitExceeded";
    pub const DEADLINE_EXCEEDED: &str = "DeadlineExceeded";
    pub const MAX_FAILED_INDEXES_EXCEEDED: &str = "MaxFailedIndexesExceeded";
    pub const FAILED_INDEXES: &str = "FailedIndexes";
    pub const SUCCESS_POLICY: &str = "SuccessPolicy";
    pub const COMPLETIONS_REACHED: &str = "CompletionsReached";
}

// ============================================================================
// Trait Implementations
// ============================================================================

use crate::common::{
    ApplyDefault, HasTypeMeta, ResourceSchema, UnimplementedConversion, VersionedObject,
};
use crate::impl_unimplemented_prost_message;

// ----------------------------------------------------------------------------
// Job
// ----------------------------------------------------------------------------

impl ResourceSchema for Job {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "batch"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "Job"
    }
    fn resource(_: &Self::Meta) -> &str {
        "jobs"
    }

    fn group_static() -> &'static str {
        "batch"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "Job"
    }
    fn resource_static() -> &'static str {
        "jobs"
    }
}

impl ResourceSchema for JobList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "batch"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "JobList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "jobs"
    }

    fn group_static() -> &'static str {
        "batch"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "JobList"
    }
    fn resource_static() -> &'static str {
        "jobs"
    }
}

impl HasTypeMeta for Job {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for JobList {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl VersionedObject for Job {
    fn metadata(&self) -> &ObjectMeta {
        self.metadata
            .as_ref()
            .unwrap_or_else(|| static_default_object_meta())
    }

    fn metadata_mut(&mut self) -> &mut ObjectMeta {
        self.metadata.get_or_insert_with(Default::default)
    }
}

impl ApplyDefault for Job {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "batch/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "Job".to_string();
        }
    }
}

impl ApplyDefault for JobList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "batch/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "JobList".to_string();
        }
    }
}

impl UnimplementedConversion for Job {}
impl_unimplemented_prost_message!(Job);
impl_unimplemented_prost_message!(JobList);

// ----------------------------------------------------------------------------
// CronJob
// ----------------------------------------------------------------------------

impl ResourceSchema for CronJob {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "batch"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "CronJob"
    }
    fn resource(_: &Self::Meta) -> &str {
        "cronjobs"
    }

    fn group_static() -> &'static str {
        "batch"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "CronJob"
    }
    fn resource_static() -> &'static str {
        "cronjobs"
    }
}

impl ResourceSchema for CronJobList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "batch"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "CronJobList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "cronjobs"
    }

    fn group_static() -> &'static str {
        "batch"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "CronJobList"
    }
    fn resource_static() -> &'static str {
        "cronjobs"
    }
}

impl HasTypeMeta for CronJob {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for CronJobList {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl VersionedObject for CronJob {
    fn metadata(&self) -> &ObjectMeta {
        self.metadata
            .as_ref()
            .unwrap_or_else(|| static_default_object_meta())
    }

    fn metadata_mut(&mut self) -> &mut ObjectMeta {
        self.metadata.get_or_insert_with(Default::default)
    }
}

impl ApplyDefault for CronJob {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "batch/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "CronJob".to_string();
        }
    }
}

impl ApplyDefault for CronJobList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "batch/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "CronJobList".to_string();
        }
    }
}

impl UnimplementedConversion for CronJob {}
impl_unimplemented_prost_message!(CronJob);
impl_unimplemented_prost_message!(CronJobList);

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
}
