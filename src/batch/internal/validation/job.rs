//! Job validation for Kubernetes batch internal API

use crate::batch::internal::{CompletionMode, Job, JobList, JobSpec};
use crate::common::validation::{
    BadValue, ErrorList, Path, invalid, name_is_dns_subdomain, required, validate_object_meta,
};

use super::{
    MAX_FAILED_INDEXES_FOR_INDEXED_JOB, MAX_MANAGED_BY_LENGTH, MAX_PARALLELISM_FOR_INDEXED_JOB,
};

// =============================================================================
// Job Validation
// =============================================================================

/// Validates a Job object for creation.
pub fn validate_job(job: &Job) -> ErrorList {
    validate_job_with_path(job, &Path::nil())
}

fn validate_job_with_path(job: &Job, base_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate metadata
    all_errs.extend(validate_object_meta(
        &job.metadata,
        true,
        name_is_dns_subdomain,
        &base_path.child("metadata"),
    ));

    // Validate spec
    all_errs.extend(validate_job_spec(&job.spec, &base_path.child("spec")));

    all_errs
}

pub(crate) fn validate_job_spec(spec: &JobSpec, base_path: &Path) -> ErrorList {
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
        if matches!(completion_mode, CompletionMode::Indexed) {
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
        if !matches!(spec.completion_mode, Some(CompletionMode::Indexed)) {
            all_errs.push(invalid(
                &base_path.child("successPolicy"),
                BadValue::String("set".to_string()),
                "requires indexed completion mode",
            ));
        }
    }

    // Selector validation:
    // - If manualSelector is true, selector MUST be specified by user
    // - If manualSelector is false/nil (default), selector will be auto-generated by API server
    //   so it's OK if it's not specified at this point
    if spec.manual_selector == Some(true) && spec.selector.is_none() {
        all_errs.push(required(
            &base_path.child("selector"),
            "selector is required when manualSelector is true",
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
