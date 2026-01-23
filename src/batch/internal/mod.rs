//! Kubernetes Batch API Internal Types
//!
//! This module contains type definitions from k8s.io/kubernetes/pkg/apis/batch/types.go
//! that are used internally by the Kubernetes API.
//!
//! Source: https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/batch/types.go

use crate::core::internal::{ConditionStatus, PodConditionType};
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
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
}
