use super::{CronJob, CronJobList, CronJobSpec, Job, JobList, JobSpec, JobStatus, JobTemplateSpec};
use crate::batch::internal::{CompletionMode, ConcurrencyPolicy, PodReplacementPolicy};
use crate::common::test_utils::assert_serde_roundtrip;
use crate::common::{ListMeta, ObjectMeta, TypeMeta};
use crate::core::v1::PodTemplateSpec;

fn job_basic() -> Job {
    Job {
        type_meta: TypeMeta {
            api_version: "batch/v1".to_string(),
            kind: "Job".to_string(),
        },
        metadata: Some(ObjectMeta {
            name: Some("job-a".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        spec: Some(JobSpec {
            parallelism: Some(2),
            completions: Some(3),
            backoff_limit: Some(5),
            completion_mode: Some(CompletionMode::Indexed),
            suspend: Some(true),
            pod_replacement_policy: Some(PodReplacementPolicy::Failed),
            manual_selector: Some(true),
            template: PodTemplateSpec {
                metadata: Some(ObjectMeta {
                    labels: std::collections::BTreeMap::from([(
                        "app".to_string(),
                        "job-a".to_string(),
                    )]),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        }),
        status: Some(JobStatus {
            active: 1,
            succeeded: 0,
            failed: 0,
            ..Default::default()
        }),
    }
}

fn job_list_basic() -> JobList {
    JobList {
        type_meta: TypeMeta {
            api_version: "batch/v1".to_string(),
            kind: "JobList".to_string(),
        },
        metadata: Some(ListMeta {
            resource_version: Some("1".to_string()),
            ..Default::default()
        }),
        items: vec![job_basic()],
    }
}

fn cron_job_basic() -> CronJob {
    CronJob {
        type_meta: TypeMeta {
            api_version: "batch/v1".to_string(),
            kind: "CronJob".to_string(),
        },
        metadata: Some(ObjectMeta {
            name: Some("cronjob-a".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        spec: Some(CronJobSpec {
            schedule: "*/5 * * * *".to_string(),
            time_zone: Some("UTC".to_string()),
            concurrency_policy: ConcurrencyPolicy::Forbid,
            suspend: Some(false),
            job_template: JobTemplateSpec {
                metadata: Some(ObjectMeta {
                    labels: std::collections::BTreeMap::from([(
                        "job".to_string(),
                        "cronjob-a".to_string(),
                    )]),
                    ..Default::default()
                }),
                spec: Some(JobSpec {
                    template: PodTemplateSpec::default(),
                    ..Default::default()
                }),
            },
            successful_jobs_history_limit: Some(2),
            failed_jobs_history_limit: Some(1),
            ..Default::default()
        }),
        status: None,
    }
}

fn cron_job_list_basic() -> CronJobList {
    CronJobList {
        type_meta: TypeMeta {
            api_version: "batch/v1".to_string(),
            kind: "CronJobList".to_string(),
        },
        metadata: Some(ListMeta {
            resource_version: Some("2".to_string()),
            ..Default::default()
        }),
        items: vec![cron_job_basic()],
    }
}

#[test]
fn serde_roundtrip_job() {
    assert_serde_roundtrip(&job_basic());
}

#[test]
fn serde_roundtrip_job_list() {
    assert_serde_roundtrip(&job_list_basic());
}

#[test]
fn serde_roundtrip_cron_job() {
    assert_serde_roundtrip(&cron_job_basic());
}

#[test]
fn serde_roundtrip_cron_job_list() {
    assert_serde_roundtrip(&cron_job_list_basic());
}
