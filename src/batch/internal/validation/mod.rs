//! Validation for Kubernetes Batch internal API types
//!
//! Ported from k8s.io/kubernetes/pkg/apis/batch/validation/validation.go

pub mod cronjob;
pub mod job;

pub use cronjob::{validate_cron_job, validate_cron_job_list};
pub use job::{validate_job, validate_job_list};

// =============================================================================
// Constants
// =============================================================================

pub const MAX_PARALLELISM_FOR_INDEXED_JOB: i32 = 100_000;
pub const MAX_FAILED_INDEXES_FOR_INDEXED_JOB: i32 = 100_000;
pub const MAX_MANAGED_BY_LENGTH: usize = 63;

// TODO: These constants will be used when validation is fully implemented
#[allow(dead_code)]
pub const COMPLETIONS_SOFT_LIMIT: i32 = 100_000;
#[allow(dead_code)]
pub const PARALLELISM_LIMIT_FOR_HIGH_COMPLETIONS: i32 = 10_000;
#[allow(dead_code)]
pub const MAX_FAILED_INDEXES_LIMIT_FOR_HIGH_COMPLETIONS: i32 = 10_000;
#[allow(dead_code)]
pub const MAX_POD_FAILURE_POLICY_RULES: usize = 20;
#[allow(dead_code)]
pub const MAX_POD_FAILURE_POLICY_ON_EXIT_CODES_VALUES: usize = 255;
#[allow(dead_code)]
pub const MAX_POD_FAILURE_POLICY_ON_POD_CONDITIONS_PATTERNS: usize = 20;
#[allow(dead_code)]
pub const MAX_JOB_SUCCESS_POLICY_SUCCEEDED_INDEXES_LIMIT: usize = 64 * 1024;
#[allow(dead_code)]
pub const MAX_SUCCESS_POLICY_RULES: usize = 20;
