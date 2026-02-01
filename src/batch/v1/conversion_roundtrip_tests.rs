use super::{
    CronJob, CronJobList, CronJobSpec, CronJobStatus, Job, JobList, JobSpec, JobStatus,
    JobTemplateSpec,
};
use crate::batch::internal;
use crate::batch::internal::{CompletionMode, ConcurrencyPolicy, PodReplacementPolicy};
use crate::common::test_utils::assert_conversion_roundtrip;
use crate::common::{ApplyDefault, ListMeta, ObjectMeta, TypeMeta};
use crate::core::v1::PodTemplateSpec;

fn job_basic() -> Job {
    Job {
        type_meta: TypeMeta::default(),
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
        status: Some(JobStatus::default()),
    }
}

fn job_list_basic() -> JobList {
    let mut item = job_basic();
    item.apply_default();
    JobList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("1".to_string()),
            ..Default::default()
        }),
        items: vec![item],
    }
}

fn cron_job_basic() -> CronJob {
    CronJob {
        type_meta: TypeMeta::default(),
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
        status: Some(CronJobStatus::default()),
    }
}

fn cron_job_list_basic() -> CronJobList {
    let mut item = cron_job_basic();
    item.apply_default();
    CronJobList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("2".to_string()),
            ..Default::default()
        }),
        items: vec![item],
    }
}

#[test]
fn conversion_roundtrip_job() {
    assert_conversion_roundtrip::<Job, internal::Job>(job_basic());
}

#[test]
fn conversion_roundtrip_job_list() {
    assert_conversion_roundtrip::<JobList, internal::JobList>(job_list_basic());
}

#[test]
fn conversion_roundtrip_cron_job() {
    assert_conversion_roundtrip::<CronJob, internal::CronJob>(cron_job_basic());
}

#[test]
fn conversion_roundtrip_cron_job_list() {
    assert_conversion_roundtrip::<CronJobList, internal::CronJobList>(cron_job_list_basic());
}
