//! CronJob validation for Kubernetes batch internal API

use crate::batch::internal::{CronJob, CronJobList, CronJobSpec};
use crate::common::validation::{
    BadValue, ErrorList, Path, invalid, name_is_dns_subdomain, required, validate_object_meta,
};

use super::job::validate_job_spec;

// =============================================================================
// CronJob Validation
// =============================================================================

/// Validates a CronJob object for creation.
pub fn validate_cron_job(cron_job: &CronJob) -> ErrorList {
    validate_cron_job_with_path(cron_job, &Path::nil())
}

fn validate_cron_job_with_path(cron_job: &CronJob, base_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate metadata
    all_errs.extend(validate_object_meta(
        &cron_job.metadata,
        true,
        name_is_dns_subdomain,
        &base_path.child("metadata"),
    ));

    // Validate spec
    all_errs.extend(validate_cron_job_spec(
        &cron_job.spec,
        &base_path.child("spec"),
    ));

    all_errs
}

fn validate_cron_job_spec(spec: &CronJobSpec, base_path: &Path) -> ErrorList {
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
    all_errs.extend(validate_job_spec(
        &spec.job_template.spec,
        &base_path.child("jobTemplate").child("spec"),
    ));

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
