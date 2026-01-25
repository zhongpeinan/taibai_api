//! Conversions between batch v1 and internal types
//!
//! Based on k8s.io/kubernetes/pkg/apis/batch/v1/conversion.go

use crate::batch::internal;
use crate::common::{ApplyDefault, FromInternal, ListMeta, ObjectMeta, ToInternal, TypeMeta};

use super::{
    CronJob, CronJobList, CronJobSpec, CronJobStatus, Job, JobList, JobSpec, JobStatus,
    JobTemplateSpec, PodFailurePolicy, PodFailurePolicyOnExitCodesRequirement,
    PodFailurePolicyRule, SuccessPolicy, SuccessPolicyRule, UncountedTerminatedPods,
};

// ============================================================================
// Helper Functions
// ============================================================================

// Metadata conversion helpers
fn is_empty_object_meta(meta: &ObjectMeta) -> bool {
    meta.name.is_none()
        && meta.generate_name.is_none()
        && meta.namespace.is_none()
        && meta.uid.is_none()
        && meta.resource_version.is_none()
        && meta.generation.is_none()
        && meta.self_link.is_none()
        && meta.labels.is_empty()
        && meta.annotations.is_empty()
        && meta.owner_references.is_empty()
        && meta.finalizers.is_empty()
        && meta.managed_fields.is_empty()
        && meta.creation_timestamp.is_none()
        && meta.deletion_timestamp.is_none()
        && meta.deletion_grace_period_seconds.is_none()
}

fn is_empty_list_meta(meta: &ListMeta) -> bool {
    meta.continue_.is_none()
        && meta.remaining_item_count.is_none()
        && meta.resource_version.is_none()
        && meta.self_link.is_none()
}

fn option_object_meta_to_meta(meta: Option<ObjectMeta>) -> ObjectMeta {
    meta.unwrap_or_default()
}

fn meta_to_option_object_meta(meta: ObjectMeta) -> Option<ObjectMeta> {
    if is_empty_object_meta(&meta) {
        None
    } else {
        Some(meta)
    }
}

fn option_list_meta_to_meta(meta: Option<ListMeta>) -> ListMeta {
    meta.unwrap_or_default()
}

fn meta_to_option_list_meta(meta: ListMeta) -> Option<ListMeta> {
    if is_empty_list_meta(&meta) {
        None
    } else {
        Some(meta)
    }
}

// ============================================================================
// Job Conversions
// ============================================================================

impl ToInternal<internal::Job> for Job {
    fn to_internal(self) -> internal::Job {
        internal::Job {
            type_meta: TypeMeta::default(),
            metadata: option_object_meta_to_meta(self.metadata),
            spec: self.spec.map(|s| s.to_internal()).unwrap_or_default(),
            status: self.status.map(|s| s.to_internal()).unwrap_or_default(),
        }
    }
}

impl FromInternal<internal::Job> for Job {
    fn from_internal(value: internal::Job) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: meta_to_option_object_meta(value.metadata),
            spec: Some(JobSpec::from_internal(value.spec)),
            status: Some(JobStatus::from_internal(value.status)),
        };
        result.apply_default();
        result
    }
}

impl ToInternal<internal::JobList> for JobList {
    fn to_internal(self) -> internal::JobList {
        internal::JobList {
            type_meta: TypeMeta::default(),
            metadata: option_list_meta_to_meta(self.metadata),
            items: self.items.into_iter().map(|e| e.to_internal()).collect(),
        }
    }
}

impl FromInternal<internal::JobList> for JobList {
    fn from_internal(value: internal::JobList) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: meta_to_option_list_meta(value.metadata),
            items: value.items.into_iter().map(Job::from_internal).collect(),
        };
        result.apply_default();
        result
    }
}

// ============================================================================
// JobSpec Conversions
// ============================================================================

impl ToInternal<internal::JobSpec> for JobSpec {
    fn to_internal(self) -> internal::JobSpec {
        internal::JobSpec {
            parallelism: self.parallelism,
            completions: self.completions,
            active_deadline_seconds: self.active_deadline_seconds,
            pod_failure_policy: self.pod_failure_policy.map(|p| p.to_internal()),
            success_policy: self.success_policy.map(|p| p.to_internal()),
            backoff_limit: self.backoff_limit,
            backoff_limit_per_index: self.backoff_limit_per_index,
            max_failed_indexes: self.max_failed_indexes,
            selector: self.selector,
            manual_selector: self.manual_selector,
            template: self.template,
            ttl_seconds_after_finished: self.ttl_seconds_after_finished,
            completion_mode: self.completion_mode,
            suspend: self.suspend,
            pod_replacement_policy: self.pod_replacement_policy,
            managed_by: self.managed_by,
        }
    }
}

impl FromInternal<internal::JobSpec> for JobSpec {
    fn from_internal(value: internal::JobSpec) -> Self {
        Self {
            parallelism: value.parallelism,
            completions: value.completions,
            active_deadline_seconds: value.active_deadline_seconds,
            pod_failure_policy: value
                .pod_failure_policy
                .map(PodFailurePolicy::from_internal),
            success_policy: value.success_policy.map(SuccessPolicy::from_internal),
            backoff_limit: value.backoff_limit,
            backoff_limit_per_index: value.backoff_limit_per_index,
            max_failed_indexes: value.max_failed_indexes,
            selector: value.selector,
            manual_selector: value.manual_selector,
            template: value.template,
            ttl_seconds_after_finished: value.ttl_seconds_after_finished,
            completion_mode: value.completion_mode,
            suspend: value.suspend,
            pod_replacement_policy: value.pod_replacement_policy,
            managed_by: value.managed_by,
        }
    }
}

// ============================================================================
// JobStatus Conversions
// ============================================================================

impl ToInternal<internal::JobStatus> for JobStatus {
    fn to_internal(self) -> internal::JobStatus {
        internal::JobStatus {
            conditions: self.conditions,
            start_time: self.start_time,
            completion_time: self.completion_time,
            active: self.active,
            terminating: self.terminating,
            ready: self.ready,
            succeeded: self.succeeded,
            failed: self.failed,
            completed_indexes: self.completed_indexes,
            failed_indexes: self.failed_indexes,
            uncounted_terminated_pods: self.uncounted_terminated_pods.map(|u| u.to_internal()),
        }
    }
}

impl FromInternal<internal::JobStatus> for JobStatus {
    fn from_internal(value: internal::JobStatus) -> Self {
        Self {
            conditions: value.conditions,
            start_time: value.start_time,
            completion_time: value.completion_time,
            active: value.active,
            terminating: value.terminating,
            ready: value.ready,
            succeeded: value.succeeded,
            failed: value.failed,
            completed_indexes: value.completed_indexes,
            failed_indexes: value.failed_indexes,
            uncounted_terminated_pods: value
                .uncounted_terminated_pods
                .map(UncountedTerminatedPods::from_internal),
        }
    }
}

// ============================================================================
// Supporting Type Conversions
// ============================================================================

impl ToInternal<internal::UncountedTerminatedPods> for UncountedTerminatedPods {
    fn to_internal(self) -> internal::UncountedTerminatedPods {
        internal::UncountedTerminatedPods {
            succeeded: self.succeeded,
            failed: self.failed,
        }
    }
}

impl FromInternal<internal::UncountedTerminatedPods> for UncountedTerminatedPods {
    fn from_internal(value: internal::UncountedTerminatedPods) -> Self {
        Self {
            succeeded: value.succeeded,
            failed: value.failed,
        }
    }
}

impl ToInternal<internal::PodFailurePolicy> for PodFailurePolicy {
    fn to_internal(self) -> internal::PodFailurePolicy {
        internal::PodFailurePolicy {
            rules: self.rules.into_iter().map(|r| r.to_internal()).collect(),
        }
    }
}

impl FromInternal<internal::PodFailurePolicy> for PodFailurePolicy {
    fn from_internal(value: internal::PodFailurePolicy) -> Self {
        Self {
            rules: value
                .rules
                .into_iter()
                .map(PodFailurePolicyRule::from_internal)
                .collect(),
        }
    }
}

impl ToInternal<internal::PodFailurePolicyRule> for PodFailurePolicyRule {
    fn to_internal(self) -> internal::PodFailurePolicyRule {
        internal::PodFailurePolicyRule {
            action: self.action,
            on_exit_codes: self.on_exit_codes.map(|o| o.to_internal()),
            on_pod_conditions: self.on_pod_conditions,
        }
    }
}

impl FromInternal<internal::PodFailurePolicyRule> for PodFailurePolicyRule {
    fn from_internal(value: internal::PodFailurePolicyRule) -> Self {
        Self {
            action: value.action,
            on_exit_codes: value
                .on_exit_codes
                .map(PodFailurePolicyOnExitCodesRequirement::from_internal),
            on_pod_conditions: value.on_pod_conditions,
        }
    }
}

impl ToInternal<internal::PodFailurePolicyOnExitCodesRequirement>
    for PodFailurePolicyOnExitCodesRequirement
{
    fn to_internal(self) -> internal::PodFailurePolicyOnExitCodesRequirement {
        internal::PodFailurePolicyOnExitCodesRequirement {
            container_name: self.container_name,
            operator: self.operator,
            values: self.values,
        }
    }
}

impl FromInternal<internal::PodFailurePolicyOnExitCodesRequirement>
    for PodFailurePolicyOnExitCodesRequirement
{
    fn from_internal(value: internal::PodFailurePolicyOnExitCodesRequirement) -> Self {
        Self {
            container_name: value.container_name,
            operator: value.operator,
            values: value.values,
        }
    }
}

impl ToInternal<internal::SuccessPolicy> for SuccessPolicy {
    fn to_internal(self) -> internal::SuccessPolicy {
        internal::SuccessPolicy {
            rules: self.rules.into_iter().map(|r| r.to_internal()).collect(),
        }
    }
}

impl FromInternal<internal::SuccessPolicy> for SuccessPolicy {
    fn from_internal(value: internal::SuccessPolicy) -> Self {
        Self {
            rules: value
                .rules
                .into_iter()
                .map(SuccessPolicyRule::from_internal)
                .collect(),
        }
    }
}

impl ToInternal<internal::SuccessPolicyRule> for SuccessPolicyRule {
    fn to_internal(self) -> internal::SuccessPolicyRule {
        internal::SuccessPolicyRule {
            succeeded_indexes: self.succeeded_indexes,
            succeeded_count: self.succeeded_count,
        }
    }
}

impl FromInternal<internal::SuccessPolicyRule> for SuccessPolicyRule {
    fn from_internal(value: internal::SuccessPolicyRule) -> Self {
        Self {
            succeeded_indexes: value.succeeded_indexes,
            succeeded_count: value.succeeded_count,
        }
    }
}

// ============================================================================
// CronJob Conversions
// ============================================================================

impl ToInternal<internal::CronJob> for CronJob {
    fn to_internal(self) -> internal::CronJob {
        internal::CronJob {
            type_meta: TypeMeta::default(),
            metadata: option_object_meta_to_meta(self.metadata),
            spec: self.spec.map(|s| s.to_internal()).unwrap_or_default(),
            status: self.status.map(|s| s.to_internal()).unwrap_or_default(),
        }
    }
}

impl FromInternal<internal::CronJob> for CronJob {
    fn from_internal(value: internal::CronJob) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: meta_to_option_object_meta(value.metadata),
            spec: Some(CronJobSpec::from_internal(value.spec)),
            status: Some(CronJobStatus::from_internal(value.status)),
        };
        result.apply_default();
        result
    }
}

impl ToInternal<internal::CronJobList> for CronJobList {
    fn to_internal(self) -> internal::CronJobList {
        internal::CronJobList {
            type_meta: TypeMeta::default(),
            metadata: option_list_meta_to_meta(self.metadata),
            items: self.items.into_iter().map(|e| e.to_internal()).collect(),
        }
    }
}

impl FromInternal<internal::CronJobList> for CronJobList {
    fn from_internal(value: internal::CronJobList) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: meta_to_option_list_meta(value.metadata),
            items: value
                .items
                .into_iter()
                .map(CronJob::from_internal)
                .collect(),
        };
        result.apply_default();
        result
    }
}

// ============================================================================
// CronJobSpec Conversions
// ============================================================================

impl ToInternal<internal::CronJobSpec> for CronJobSpec {
    fn to_internal(self) -> internal::CronJobSpec {
        internal::CronJobSpec {
            schedule: self.schedule,
            time_zone: self.time_zone,
            starting_deadline_seconds: self.starting_deadline_seconds,
            concurrency_policy: self.concurrency_policy,
            suspend: self.suspend,
            job_template: self.job_template.to_internal(),
            successful_jobs_history_limit: self.successful_jobs_history_limit,
            failed_jobs_history_limit: self.failed_jobs_history_limit,
        }
    }
}

impl FromInternal<internal::CronJobSpec> for CronJobSpec {
    fn from_internal(value: internal::CronJobSpec) -> Self {
        Self {
            schedule: value.schedule,
            time_zone: value.time_zone,
            starting_deadline_seconds: value.starting_deadline_seconds,
            concurrency_policy: value.concurrency_policy,
            suspend: value.suspend,
            job_template: JobTemplateSpec::from_internal(value.job_template),
            successful_jobs_history_limit: value.successful_jobs_history_limit,
            failed_jobs_history_limit: value.failed_jobs_history_limit,
        }
    }
}

// ============================================================================
// CronJobStatus Conversions
// ============================================================================

impl ToInternal<internal::CronJobStatus> for CronJobStatus {
    fn to_internal(self) -> internal::CronJobStatus {
        internal::CronJobStatus {
            active: self.active,
            last_schedule_time: self.last_schedule_time,
            last_successful_time: self.last_successful_time,
        }
    }
}

impl FromInternal<internal::CronJobStatus> for CronJobStatus {
    fn from_internal(value: internal::CronJobStatus) -> Self {
        Self {
            active: value.active,
            last_schedule_time: value.last_schedule_time,
            last_successful_time: value.last_successful_time,
        }
    }
}

// ============================================================================
// JobTemplateSpec Conversions
// ============================================================================

impl ToInternal<internal::JobTemplateSpec> for JobTemplateSpec {
    fn to_internal(self) -> internal::JobTemplateSpec {
        internal::JobTemplateSpec {
            metadata: self.metadata.unwrap_or_default(),
            spec: self.spec.map(|s| s.to_internal()).unwrap_or_default(),
        }
    }
}

impl FromInternal<internal::JobTemplateSpec> for JobTemplateSpec {
    fn from_internal(value: internal::JobTemplateSpec) -> Self {
        Self {
            metadata: if is_empty_object_meta(&value.metadata) {
                None
            } else {
                Some(value.metadata)
            },
            spec: Some(JobSpec::from_internal(value.spec)),
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{ObjectMeta, TypeMeta};
    use crate::core::v1::PodTemplateSpec;

    #[test]
    fn test_job_round_trip_v1_to_internal_to_v1() {
        // Create a v1 Job
        let original = Job {
            type_meta: TypeMeta {
                api_version: "batch/v1".to_string(),
                kind: "Job".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("test-job".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(JobSpec {
                parallelism: Some(2),
                completions: Some(5),
                template: PodTemplateSpec::default(),
                ..Default::default()
            }),
            status: Some(JobStatus {
                active: 2,
                succeeded: 1,
                failed: 0,
                ..Default::default()
            }),
        };

        // Convert to internal
        let internal = original.clone().to_internal();

        // Verify internal conversion
        assert_eq!(internal.metadata.name, Some("test-job".to_string()));
        assert_eq!(internal.spec.parallelism, Some(2));
        assert_eq!(internal.spec.completions, Some(5));

        // Convert back to v1
        let round_trip = Job::from_internal(internal);

        // Verify key fields survived the round trip
        assert_eq!(round_trip.metadata, original.metadata);
        assert_eq!(
            round_trip.spec.as_ref().unwrap().parallelism,
            original.spec.as_ref().unwrap().parallelism
        );
        assert_eq!(
            round_trip.spec.as_ref().unwrap().completions,
            original.spec.as_ref().unwrap().completions
        );
        assert_eq!(round_trip.status.as_ref().unwrap().active, 2);

        // TypeMeta should be defaulted
        assert_eq!(round_trip.type_meta.api_version, "batch/v1");
        assert_eq!(round_trip.type_meta.kind, "Job");
    }

    #[test]
    fn test_cronjob_round_trip() {
        let original = CronJob {
            type_meta: TypeMeta {
                api_version: "batch/v1".to_string(),
                kind: "CronJob".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("test-cronjob".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(CronJobSpec {
                schedule: "*/5 * * * *".to_string(),
                job_template: JobTemplateSpec {
                    metadata: None,
                    spec: Some(JobSpec {
                        template: PodTemplateSpec::default(),
                        ..Default::default()
                    }),
                },
                ..Default::default()
            }),
            status: None,
        };

        // Convert to internal and back
        let internal = original.clone().to_internal();
        let round_trip = CronJob::from_internal(internal);

        // Verify
        assert_eq!(round_trip.metadata, original.metadata);
        assert_eq!(
            round_trip.spec.as_ref().unwrap().schedule,
            original.spec.as_ref().unwrap().schedule
        );
        assert_eq!(round_trip.type_meta.api_version, "batch/v1");
        assert_eq!(round_trip.type_meta.kind, "CronJob");
    }
}
