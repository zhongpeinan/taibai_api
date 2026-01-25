//! Kubernetes Batch API types
//!
//! This module contains types from the Kubernetes batch API group.

pub mod internal;
pub mod v1;

pub use internal::{
    CompletionMode, ConcurrencyPolicy, CronJob as InternalCronJob,
    CronJobList as InternalCronJobList, CronJobSpec as InternalCronJobSpec,
    CronJobStatus as InternalCronJobStatus, Job as InternalJob, JobCondition, JobConditionType,
    JobList as InternalJobList, JobSpec as InternalJobSpec, JobStatus as InternalJobStatus,
    JobTemplateSpec as InternalJobTemplateSpec, PodFailurePolicy as InternalPodFailurePolicy,
    PodFailurePolicyAction, PodFailurePolicyOnExitCodesOperator,
    PodFailurePolicyOnExitCodesRequirement as InternalPodFailurePolicyOnExitCodesRequirement,
    PodFailurePolicyOnPodConditionsPattern, PodFailurePolicyRule as InternalPodFailurePolicyRule,
    PodReplacementPolicy, SuccessPolicy as InternalSuccessPolicy,
    SuccessPolicyRule as InternalSuccessPolicyRule,
    UncountedTerminatedPods as InternalUncountedTerminatedPods,
};
pub use v1::{
    CronJob, CronJobList, CronJobSpec, CronJobStatus, Job, JobList, JobSpec, JobStatus,
    JobTemplateSpec, PodFailurePolicy, PodFailurePolicyOnExitCodesRequirement,
    PodFailurePolicyRule, SuccessPolicy, SuccessPolicyRule, UncountedTerminatedPods,
};
