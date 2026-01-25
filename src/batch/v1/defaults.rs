//! Default values for batch v1 API types
//!
//! Ported from k8s/pkg/apis/batch/v1/defaults.go and zz_generated.defaults.go

use crate::batch::internal::{CompletionMode, PodReplacementPolicy};
use crate::common::ApplyDefault;
use crate::core::internal::ConditionStatus;

use super::{CronJob, CronJobList, Job, JobList, JobSpec, PodFailurePolicyOnPodConditionsPattern};

fn set_defaults_pod_failure_policy_on_pod_conditions_pattern(
    pattern: &mut PodFailurePolicyOnPodConditionsPattern,
) {
    if pattern.status == ConditionStatus::Unknown {
        pattern.status = ConditionStatus::True;
    }
}

fn set_defaults_job_spec(spec: &mut JobSpec) {
    if spec.completions.is_none() && spec.parallelism.is_none() {
        spec.completions = Some(1);
        spec.parallelism = Some(1);
    }
    if spec.parallelism.is_none() {
        spec.parallelism = Some(1);
    }
    if spec.backoff_limit.is_none() {
        spec.backoff_limit = Some(if spec.backoff_limit_per_index.is_some() {
            i32::MAX
        } else {
            6
        });
    }
    if spec.completion_mode.is_none() {
        spec.completion_mode = Some(CompletionMode::NonIndexed);
    }
    if spec.suspend.is_none() {
        spec.suspend = Some(false);
    }
    if spec.pod_replacement_policy.is_none() {
        spec.pod_replacement_policy = Some(if spec.pod_failure_policy.is_some() {
            PodReplacementPolicy::Failed
        } else {
            PodReplacementPolicy::TerminatingOrFailed
        });
    }
    if spec.manual_selector.is_none() {
        spec.manual_selector = Some(false);
    }

    if let Some(policy) = spec.pod_failure_policy.as_mut() {
        for rule in &mut policy.rules {
            for pattern in &mut rule.on_pod_conditions {
                set_defaults_pod_failure_policy_on_pod_conditions_pattern(pattern);
            }
        }
    }

    if let Some(pod_spec) = spec.template.spec.as_mut() {
        pod_spec.apply_default();
    }
}

fn set_defaults_job(obj: &mut Job) {
    if let Some(spec) = obj.spec.as_mut() {
        set_defaults_job_spec(spec);

        if let Some(template_meta) = spec.template.metadata.as_ref() {
            if !template_meta.labels.is_empty() {
                let metadata = obj.metadata.get_or_insert_with(Default::default);
                if metadata.labels.is_empty() {
                    metadata.labels = template_meta.labels.clone();
                }
            }
        }
    }
}

fn set_defaults_cron_job(obj: &mut CronJob) {
    if let Some(spec) = obj.spec.as_mut() {
        if spec.suspend.is_none() {
            spec.suspend = Some(false);
        }
        if spec.successful_jobs_history_limit.is_none() {
            spec.successful_jobs_history_limit = Some(3);
        }
        if spec.failed_jobs_history_limit.is_none() {
            spec.failed_jobs_history_limit = Some(1);
        }

        if let Some(job_spec) = spec.job_template.spec.as_mut() {
            set_defaults_job_spec(job_spec);
        }
    }
}

impl ApplyDefault for Job {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "batch/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "Job".to_string();
        }

        set_defaults_job(self);
    }
}

impl ApplyDefault for JobList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "batch/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "JobList".to_string();
        }

        for item in &mut self.items {
            item.apply_default();
        }
    }
}

impl ApplyDefault for CronJob {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "batch/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "CronJob".to_string();
        }

        set_defaults_cron_job(self);
    }
}

impl ApplyDefault for CronJobList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "batch/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "CronJobList".to_string();
        }

        for item in &mut self.items {
            item.apply_default();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::TypeMeta;

    #[test]
    fn test_job_default_parallelism_and_completions() {
        let mut job = Job {
            type_meta: TypeMeta::default(),
            metadata: None,
            spec: Some(JobSpec::default()),
            status: None,
        };

        job.apply_default();

        let spec = job.spec.as_ref().unwrap();
        assert_eq!(spec.parallelism, Some(1));
        assert_eq!(spec.completions, Some(1));
    }

    #[test]
    fn test_cronjob_defaults_history_limits() {
        let mut cronjob = CronJob {
            type_meta: TypeMeta::default(),
            metadata: None,
            spec: Some(Default::default()),
            status: None,
        };

        cronjob.apply_default();

        let spec = cronjob.spec.as_ref().unwrap();
        assert_eq!(spec.successful_jobs_history_limit, Some(3));
        assert_eq!(spec.failed_jobs_history_limit, Some(1));
        assert_eq!(spec.suspend, Some(false));
    }
}
