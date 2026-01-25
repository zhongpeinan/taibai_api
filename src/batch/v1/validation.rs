//! Validation for Kubernetes Batch API v1 types
//!
//! Ported from k8s.io/kubernetes/pkg/apis/batch/validation/validation.go

use crate::common::validation::{
    BadValue, ErrorList, Path, invalid, name_is_dns_subdomain, required, validate_object_meta,
};

use super::{CronJob, CronJobList, Job, JobList};

// ============================================================================
// Constants
// ============================================================================

const MAX_PARALLELISM_FOR_INDEXED_JOB: i32 = 100_000;
const MAX_FAILED_INDEXES_FOR_INDEXED_JOB: i32 = 100_000;
const COMPLETIONS_SOFT_LIMIT: i32 = 100_000;
const PARALLELISM_LIMIT_FOR_HIGH_COMPLETIONS: i32 = 10_000;
const MAX_FAILED_INDEXES_LIMIT_FOR_HIGH_COMPLETIONS: i32 = 10_000;
const MAX_POD_FAILURE_POLICY_RULES: usize = 20;
const MAX_POD_FAILURE_POLICY_ON_EXIT_CODES_VALUES: usize = 255;
const MAX_POD_FAILURE_POLICY_ON_POD_CONDITIONS_PATTERNS: usize = 20;
const MAX_MANAGED_BY_LENGTH: usize = 63;
const MAX_JOB_SUCCESS_POLICY_SUCCEEDED_INDEXES_LIMIT: usize = 64 * 1024;
const MAX_SUCCESS_POLICY_RULES: usize = 20;

// ============================================================================
// Job Validation
// ============================================================================

/// Validates a Job object for creation.
pub fn validate_job(job: &Job) -> ErrorList {
    validate_job_with_path(job, &Path::nil())
}

fn validate_job_with_path(job: &Job, base_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let default_meta = crate::common::ObjectMeta::default();
    let meta = job.metadata.as_ref().unwrap_or(&default_meta);

    // Validate metadata
    all_errs.extend(validate_object_meta(
        meta,
        true,
        name_is_dns_subdomain,
        &base_path.child("metadata"),
    ));

    // Validate spec if present
    if let Some(ref spec) = job.spec {
        all_errs.extend(validate_job_spec(spec, &base_path.child("spec")));
    }

    all_errs
}

fn validate_job_spec(spec: &super::JobSpec, base_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate non-negative fields
    if let Some(parallelism) = spec.parallelism {
        if parallelism < 0 {
            all_errs.push(invalid(
                &base_path.child("parallelism"),
                BadValue::Int(parallelism as i64),
                "must be non-negative",
            ));
        }
    }

    if let Some(completions) = spec.completions {
        if completions < 0 {
            all_errs.push(invalid(
                &base_path.child("completions"),
                BadValue::Int(completions as i64),
                "must be non-negative",
            ));
        }
    }

    if let Some(active_deadline_seconds) = spec.active_deadline_seconds {
        if active_deadline_seconds < 0 {
            all_errs.push(invalid(
                &base_path.child("activeDeadlineSeconds"),
                BadValue::Int(active_deadline_seconds),
                "must be non-negative",
            ));
        }
    }

    if let Some(backoff_limit) = spec.backoff_limit {
        if backoff_limit < 0 {
            all_errs.push(invalid(
                &base_path.child("backoffLimit"),
                BadValue::Int(backoff_limit as i64),
                "must be non-negative",
            ));
        }
    }

    if let Some(backoff_limit_per_index) = spec.backoff_limit_per_index {
        if backoff_limit_per_index < 0 {
            all_errs.push(invalid(
                &base_path.child("backoffLimitPerIndex"),
                BadValue::Int(backoff_limit_per_index as i64),
                "must be non-negative",
            ));
        }
    }

    if let Some(max_failed_indexes) = spec.max_failed_indexes {
        if max_failed_indexes < 0 {
            all_errs.push(invalid(
                &base_path.child("maxFailedIndexes"),
                BadValue::Int(max_failed_indexes as i64),
                "must be non-negative",
            ));
        }
        // maxFailedIndexes requires backoffLimitPerIndex
        if spec.backoff_limit_per_index.is_none() {
            all_errs.push(required(
                &base_path.child("backoffLimitPerIndex"),
                "required when maxFailedIndexes is specified",
            ));
        }
    }

    if let Some(ttl) = spec.ttl_seconds_after_finished {
        if ttl < 0 {
            all_errs.push(invalid(
                &base_path.child("ttlSecondsAfterFinished"),
                BadValue::Int(ttl as i64),
                "must be non-negative",
            ));
        }
    }

    // Validate managed_by length
    if let Some(ref managed_by) = spec.managed_by {
        if managed_by.len() > MAX_MANAGED_BY_LENGTH {
            all_errs.push(invalid(
                &base_path.child("managedBy"),
                BadValue::String(managed_by.clone()),
                &format!("must have at most {} characters", MAX_MANAGED_BY_LENGTH),
            ));
        }
    }

    // Validate completion mode specific rules
    if let Some(ref completion_mode) = spec.completion_mode {
        let completion_mode_str = format!("{:?}", completion_mode);
        if completion_mode_str == "Indexed" {
            // Indexed mode requires completions
            if spec.completions.is_none() {
                all_errs.push(required(
                    &base_path.child("completions"),
                    "required when completionMode is Indexed",
                ));
            }

            // Check parallelism limit for indexed jobs
            if let Some(parallelism) = spec.parallelism {
                if parallelism > MAX_PARALLELISM_FOR_INDEXED_JOB {
                    all_errs.push(invalid(
                        &base_path.child("parallelism"),
                        BadValue::Int(parallelism as i64),
                        &format!(
                            "must be less than or equal to {} when completionMode is Indexed",
                            MAX_PARALLELISM_FOR_INDEXED_JOB
                        ),
                    ));
                }
            }

            // Validate maxFailedIndexes
            if let Some(max_failed) = spec.max_failed_indexes {
                if let Some(completions) = spec.completions {
                    if max_failed > completions {
                        all_errs.push(invalid(
                            &base_path.child("maxFailedIndexes"),
                            BadValue::Int(max_failed as i64),
                            "must be less than or equal to completions",
                        ));
                    }
                }

                if max_failed > MAX_FAILED_INDEXES_FOR_INDEXED_JOB {
                    all_errs.push(invalid(
                        &base_path.child("maxFailedIndexes"),
                        BadValue::Int(max_failed as i64),
                        &format!(
                            "must be less than or equal to {}",
                            MAX_FAILED_INDEXES_FOR_INDEXED_JOB
                        ),
                    ));
                }
            }
        } else {
            // NonIndexed mode restrictions
            if spec.backoff_limit_per_index.is_some() {
                all_errs.push(invalid(
                    &base_path.child("backoffLimitPerIndex"),
                    BadValue::String("set".to_string()),
                    "requires indexed completion mode",
                ));
            }
            if spec.max_failed_indexes.is_some() {
                all_errs.push(invalid(
                    &base_path.child("maxFailedIndexes"),
                    BadValue::String("set".to_string()),
                    "requires indexed completion mode",
                ));
            }
        }
    }

    // Validate successPolicy requires indexed mode
    if spec.success_policy.is_some() {
        if spec.completion_mode.is_none()
            || format!("{:?}", spec.completion_mode.as_ref().unwrap()) != "Indexed"
        {
            all_errs.push(invalid(
                &base_path.child("successPolicy"),
                BadValue::String("set".to_string()),
                "requires indexed completion mode",
            ));
        }
    }

    // Selector is required
    if spec.selector.is_none() {
        all_errs.push(required(
            &base_path.child("selector"),
            "selector is required",
        ));
    }

    all_errs
}

/// Validates a JobList object.
pub fn validate_job_list(list: &JobList) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (i, item) in list.items.iter().enumerate() {
        let item_path = Path::new("items").index(i);
        all_errs.extend(validate_job_with_path(item, &item_path));
    }

    all_errs
}

// ============================================================================
// CronJob Validation
// ============================================================================

/// Validates a CronJob object for creation.
pub fn validate_cron_job(cron_job: &CronJob) -> ErrorList {
    validate_cron_job_with_path(cron_job, &Path::nil())
}

fn validate_cron_job_with_path(cron_job: &CronJob, base_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let default_meta = crate::common::ObjectMeta::default();
    let meta = cron_job.metadata.as_ref().unwrap_or(&default_meta);

    // Validate metadata
    all_errs.extend(validate_object_meta(
        meta,
        true,
        name_is_dns_subdomain,
        &base_path.child("metadata"),
    ));

    // Validate spec if present
    if let Some(ref spec) = cron_job.spec {
        all_errs.extend(validate_cron_job_spec(spec, &base_path.child("spec")));
    }

    all_errs
}

fn validate_cron_job_spec(spec: &super::CronJobSpec, base_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Schedule is required and must not be empty
    if spec.schedule.is_empty() {
        all_errs.push(required(
            &base_path.child("schedule"),
            "schedule is required",
        ));
    }

    // Validate starting_deadline_seconds
    if let Some(deadline) = spec.starting_deadline_seconds {
        if deadline < 0 {
            all_errs.push(invalid(
                &base_path.child("startingDeadlineSeconds"),
                BadValue::Int(deadline),
                "must be non-negative",
            ));
        }
    }

    // Validate history limits
    if let Some(limit) = spec.successful_jobs_history_limit {
        if limit < 0 {
            all_errs.push(invalid(
                &base_path.child("successfulJobsHistoryLimit"),
                BadValue::Int(limit as i64),
                "must be non-negative",
            ));
        }
    }

    if let Some(limit) = spec.failed_jobs_history_limit {
        if limit < 0 {
            all_errs.push(invalid(
                &base_path.child("failedJobsHistoryLimit"),
                BadValue::Int(limit as i64),
                "must be non-negative",
            ));
        }
    }

    // Validate job_template.spec
    if let Some(ref job_spec) = spec.job_template.spec {
        all_errs.extend(validate_job_spec(
            job_spec,
            &base_path.child("jobTemplate").child("spec"),
        ));
    }

    all_errs
}

/// Validates a CronJobList object.
pub fn validate_cron_job_list(list: &CronJobList) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (i, item) in list.items.iter().enumerate() {
        let item_path = Path::new("items").index(i);
        all_errs.extend(validate_cron_job_with_path(item, &item_path));
    }

    all_errs
}

// ============================================================================
// Tests
// ============================================================================

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
