//! Kubernetes Batch API types
//!
//! This module contains types from the Kubernetes batch API group.

pub mod internal;
pub mod v1;

pub use internal::{
    CompletionMode, JobCondition, JobConditionType, PodFailurePolicyAction,
    PodFailurePolicyOnExitCodesOperator, PodFailurePolicyOnPodConditionsPattern,
    PodReplacementPolicy,
};
pub use v1::{
    ConcurrencyPolicy, CronJob, CronJobList, CronJobSpec, CronJobStatus, Job, JobList, JobSpec,
    JobStatus, JobTemplateSpec, PodFailurePolicy, PodFailurePolicyOnExitCodesRequirement,
    PodFailurePolicyRule, SuccessPolicy, SuccessPolicyRule, UncountedTerminatedPods,
};
// Re-export JobConditionType as V1JobConditionType for consistency
pub use internal::JobConditionType as V1JobConditionType;
