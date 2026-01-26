//! Defaulting functions for autoscaling/v2 API types
//!
//! Ported from k8s.io/kubernetes/pkg/apis/autoscaling/v2/defaults.go

use super::{
    HPAScalingPolicy, HPAScalingPolicyType, HPAScalingRules, HorizontalPodAutoscaler,
    MetricSourceType, MetricTarget, MetricTargetType, ResourceMetricSource, ScalingPolicySelect,
};
use crate::autoscaling::DEFAULT_CPU_UTILIZATION;
use crate::core::v1::resource::resource_name;

const SCALE_UP_LIMIT_PERCENT: i32 = 100;
const SCALE_UP_LIMIT_MINIMUM_PODS: i32 = 4;
const SCALE_UP_PERIOD_SECONDS: i32 = 15;
const SCALE_UP_STABILIZATION_SECONDS: i32 = 0;
const SCALE_DOWN_PERIOD_SECONDS: i32 = 15;
const SCALE_DOWN_LIMIT_PERCENT: i32 = 100;

fn default_scale_up_rules() -> HPAScalingRules {
    HPAScalingRules {
        stabilization_window_seconds: Some(SCALE_UP_STABILIZATION_SECONDS),
        select_policy: Some(ScalingPolicySelect::Max),
        policies: vec![
            HPAScalingPolicy {
                type_: HPAScalingPolicyType::Pods,
                value: SCALE_UP_LIMIT_MINIMUM_PODS,
                period_seconds: SCALE_UP_PERIOD_SECONDS,
            },
            HPAScalingPolicy {
                type_: HPAScalingPolicyType::Percent,
                value: SCALE_UP_LIMIT_PERCENT,
                period_seconds: SCALE_UP_PERIOD_SECONDS,
            },
        ],
        tolerance: None,
    }
}

fn default_scale_down_rules() -> HPAScalingRules {
    HPAScalingRules {
        stabilization_window_seconds: None,
        select_policy: Some(ScalingPolicySelect::Max),
        policies: vec![HPAScalingPolicy {
            type_: HPAScalingPolicyType::Percent,
            value: SCALE_DOWN_LIMIT_PERCENT,
            period_seconds: SCALE_DOWN_PERIOD_SECONDS,
        }],
        tolerance: None,
    }
}

fn merge_scaling_rules(
    from: Option<HPAScalingRules>,
    mut rules: HPAScalingRules,
) -> HPAScalingRules {
    let Some(from) = from else {
        return rules;
    };

    if from.select_policy.is_some() {
        rules.select_policy = from.select_policy;
    }
    if from.stabilization_window_seconds.is_some() {
        rules.stabilization_window_seconds = from.stabilization_window_seconds;
    }
    if !from.policies.is_empty() {
        rules.policies = from.policies;
    }
    if from.tolerance.is_some() {
        rules.tolerance = from.tolerance;
    }
    rules
}

/// Apply defaults to HorizontalPodAutoscaler.
pub fn set_defaults_horizontal_pod_autoscaler(obj: &mut HorizontalPodAutoscaler) {
    let Some(ref mut spec) = obj.spec else {
        return;
    };

    if spec.min_replicas.is_none() {
        spec.min_replicas = Some(1);
    }

    if spec.metrics.is_empty() {
        spec.metrics = vec![super::MetricSpec {
            type_: MetricSourceType::Resource,
            resource: Some(ResourceMetricSource {
                name: resource_name::CPU.to_string(),
                target: MetricTarget {
                    type_: MetricTargetType::Utilization,
                    average_utilization: Some(DEFAULT_CPU_UTILIZATION),
                    ..Default::default()
                },
            }),
            ..Default::default()
        }];
    }

    set_defaults_horizontal_pod_autoscaler_behavior(obj);
}

/// Apply defaults to HorizontalPodAutoscalerBehavior when behavior is present.
pub fn set_defaults_horizontal_pod_autoscaler_behavior(obj: &mut HorizontalPodAutoscaler) {
    let Some(ref mut spec) = obj.spec else {
        return;
    };
    let Some(ref mut behavior) = spec.behavior else {
        return;
    };

    let scale_up = merge_scaling_rules(behavior.scale_up.take(), default_scale_up_rules());
    let scale_down = merge_scaling_rules(behavior.scale_down.take(), default_scale_down_rules());
    behavior.scale_up = Some(scale_up);
    behavior.scale_down = Some(scale_down);
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{ObjectMeta, TypeMeta};

    #[test]
    fn test_default_min_replicas_and_metrics() {
        let mut hpa = HorizontalPodAutoscaler {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("hpa".to_string()),
                ..Default::default()
            }),
            spec: Some(Default::default()),
            status: None,
        };

        set_defaults_horizontal_pod_autoscaler(&mut hpa);

        let spec = hpa.spec.as_ref().unwrap();
        assert_eq!(spec.min_replicas, Some(1));
        assert_eq!(spec.metrics.len(), 1);
        let metric = &spec.metrics[0];
        assert_eq!(metric.type_, MetricSourceType::Resource);
        assert_eq!(
            metric.resource.as_ref().unwrap().target.average_utilization,
            Some(DEFAULT_CPU_UTILIZATION)
        );
    }

    #[test]
    fn test_default_behavior_rules() {
        let mut hpa = HorizontalPodAutoscaler {
            type_meta: TypeMeta::default(),
            metadata: None,
            spec: Some(HorizontalPodAutoscalerSpec {
                behavior: Some(HorizontalPodAutoscalerBehavior::default()),
                ..Default::default()
            }),
            status: None,
        };

        set_defaults_horizontal_pod_autoscaler(&mut hpa);
        let behavior = hpa.spec.as_ref().unwrap().behavior.as_ref().unwrap();
        assert!(behavior.scale_up.is_some());
        assert!(behavior.scale_down.is_some());
        assert!(!behavior.scale_up.as_ref().unwrap().policies.is_empty());
        assert!(!behavior.scale_down.as_ref().unwrap().policies.is_empty());
    }
}
