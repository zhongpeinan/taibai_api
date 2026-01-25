//! Defaulting functions for autoscaling/v2beta1 API types
//!
//! Ported from k8s.io/kubernetes/pkg/apis/autoscaling/v2beta1/defaults.go

use super::{HorizontalPodAutoscaler, MetricSourceType, ResourceMetricSource};
use crate::autoscaling::DEFAULT_CPU_UTILIZATION;
use crate::core::v1::resource::resource_name;

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
                target_average_utilization: Some(DEFAULT_CPU_UTILIZATION),
                ..Default::default()
            }),
            ..Default::default()
        }];
    }
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
            metric.resource.as_ref().unwrap().target_average_utilization,
            Some(DEFAULT_CPU_UTILIZATION)
        );
    }
}
