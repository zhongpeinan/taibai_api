//! Validation for Kubernetes Batch API v1 types
//!
//! Wrapper around internal validation (v1 -> internal -> validate)

use crate::batch::internal::validation as internal_validation;
use crate::common::ToInternal;
use crate::common::validation::ErrorList;

use super::{CronJob, CronJobList, Job, JobList};

// =============================================================================
// Job Validation
// =============================================================================

/// Validates a Job object for creation.
pub fn validate_job(job: &Job) -> ErrorList {
    internal_validation::validate_job(&job.clone().to_internal())
}

/// Validates a JobList object.
pub fn validate_job_list(list: &JobList) -> ErrorList {
    internal_validation::validate_job_list(&list.clone().to_internal())
}

// =============================================================================
// CronJob Validation
// =============================================================================

/// Validates a CronJob object for creation.
pub fn validate_cron_job(cron_job: &CronJob) -> ErrorList {
    internal_validation::validate_cron_job(&cron_job.clone().to_internal())
}

/// Validates a CronJobList object.
pub fn validate_cron_job_list(list: &CronJobList) -> ErrorList {
    internal_validation::validate_cron_job_list(&list.clone().to_internal())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::batch::v1::{CronJobSpec, JobSpec, JobTemplateSpec};
    use crate::common::{LabelSelector, ObjectMeta, TypeMeta};
    use crate::core::v1::PodTemplateSpec;

    #[test]
    fn test_validate_job_valid() {
        let job = Job {
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
                selector: Some(LabelSelector::default()),
                template: PodTemplateSpec::default(),
                ..Default::default()
            }),
            status: None,
        };

        let errors = validate_job(&job);
        assert!(
            errors.is_empty(),
            "Expected no validation errors, got: {:?}",
            errors.errors
        );
    }

    #[test]
    fn test_validate_job_negative_parallelism() {
        let job = Job {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-job".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(JobSpec {
                parallelism: Some(-1), // Invalid: negative
                selector: Some(LabelSelector::default()),
                template: PodTemplateSpec::default(),
                ..Default::default()
            }),
            status: None,
        };

        let errors = validate_job(&job);
        assert!(!errors.is_empty(), "Expected validation errors");
        assert!(
            errors
                .errors
                .iter()
                .any(|e| e.field.to_string().contains("parallelism")),
            "Expected error for parallelism"
        );
    }

    #[test]
    fn test_validate_job_missing_selector() {
        let job = Job {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-job".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(JobSpec {
                selector: None, // Invalid: required
                template: PodTemplateSpec::default(),
                ..Default::default()
            }),
            status: None,
        };

        let errors = validate_job(&job);
        assert!(!errors.is_empty(), "Expected validation errors");
        assert!(
            errors
                .errors
                .iter()
                .any(|e| e.field.to_string().contains("selector")),
            "Expected error for missing selector"
        );
    }

    #[test]
    fn test_validate_cronjob_valid() {
        let cronjob = CronJob {
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
                        selector: Some(LabelSelector::default()),
                        template: PodTemplateSpec::default(),
                        ..Default::default()
                    }),
                },
                ..Default::default()
            }),
            status: None,
        };

        let errors = validate_cron_job(&cronjob);
        assert!(
            errors.is_empty(),
            "Expected no validation errors, got: {:?}",
            errors.errors
        );
    }

    #[test]
    fn test_validate_cronjob_missing_schedule() {
        let cronjob = CronJob {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-cronjob".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(CronJobSpec {
                schedule: String::new(), // Invalid: empty
                job_template: JobTemplateSpec {
                    metadata: None,
                    spec: Some(JobSpec {
                        selector: Some(LabelSelector::default()),
                        template: PodTemplateSpec::default(),
                        ..Default::default()
                    }),
                },
                ..Default::default()
            }),
            status: None,
        };

        let errors = validate_cron_job(&cronjob);
        assert!(!errors.is_empty(), "Expected validation errors");
        assert!(
            errors
                .errors
                .iter()
                .any(|e| e.field.to_string().contains("schedule")),
            "Expected error for missing schedule"
        );
    }
}
