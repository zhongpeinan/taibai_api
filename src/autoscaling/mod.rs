//! Kubernetes Autoscaling API
//!
//! This module contains types from the Kubernetes autoscaling API group.
//!
//! The autoscaling API contains types for:
//! - Horizontal Pod Autoscaler (HPA)
//! - Scale subresource
//! - Metric specifications for autoscaling

pub mod internal;
pub mod v1;
pub mod v2;
pub mod v2beta1;
pub mod v2beta2;
pub mod validation;

use std::collections::BTreeMap;

// Round-trip annotation keys (ported from k8s/pkg/apis/autoscaling/annotations.go)
pub const METRIC_SPECS_ANNOTATION: &str = "autoscaling.alpha.kubernetes.io/metrics";
pub const METRIC_STATUSES_ANNOTATION: &str = "autoscaling.alpha.kubernetes.io/current-metrics";
pub const HPA_CONDITIONS_ANNOTATION: &str = "autoscaling.alpha.kubernetes.io/conditions";
pub const BEHAVIOR_SPECS_ANNOTATION: &str = "autoscaling.alpha.kubernetes.io/behavior";
pub const TOLERANCE_SCALE_DOWN_ANNOTATION: &str =
    "autoscaling.alpha.kubernetes.io/scale-down-tolerance";
pub const TOLERANCE_SCALE_UP_ANNOTATION: &str =
    "autoscaling.alpha.kubernetes.io/scale-up-tolerance";
pub const DEFAULT_CPU_UTILIZATION: i32 = 80;

pub fn drop_round_trip_horizontal_pod_autoscaler_annotations(
    annotations: &mut BTreeMap<String, String>,
) {
    annotations.remove(METRIC_SPECS_ANNOTATION);
    annotations.remove(METRIC_STATUSES_ANNOTATION);
    annotations.remove(HPA_CONDITIONS_ANNOTATION);
    annotations.remove(BEHAVIOR_SPECS_ANNOTATION);
    annotations.remove(TOLERANCE_SCALE_DOWN_ANNOTATION);
    annotations.remove(TOLERANCE_SCALE_UP_ANNOTATION);
}

pub use v1::{
    ContainerResourceMetricSource, ContainerResourceMetricStatus, CrossVersionObjectReference,
    ExternalMetricSource, ExternalMetricStatus, HorizontalPodAutoscaler,
    HorizontalPodAutoscalerCondition, HorizontalPodAutoscalerConditionType,
    HorizontalPodAutoscalerList, HorizontalPodAutoscalerSpec, HorizontalPodAutoscalerStatus,
    MetricSourceType, MetricSpec, MetricStatus, ObjectMetricSource, ObjectMetricStatus,
    PodsMetricSource, PodsMetricStatus, ResourceMetricSource, ResourceMetricStatus, Scale,
    ScaleSpec, ScaleStatus, horizontal_pod_autoscaler_condition_type, metric_source_type,
};
