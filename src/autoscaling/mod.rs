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

pub use v1::{
    ContainerResourceMetricSource, ContainerResourceMetricStatus, CrossVersionObjectReference,
    ExternalMetricSource, ExternalMetricStatus, HorizontalPodAutoscaler,
    HorizontalPodAutoscalerCondition, HorizontalPodAutoscalerConditionType,
    HorizontalPodAutoscalerList, HorizontalPodAutoscalerSpec, HorizontalPodAutoscalerStatus,
    MetricSourceType, MetricSpec, MetricStatus, ObjectMetricSource, ObjectMetricStatus,
    PodsMetricSource, PodsMetricStatus, ResourceMetricSource, ResourceMetricStatus, Scale,
    ScaleSpec, ScaleStatus, horizontal_pod_autoscaler_condition_type, metric_source_type,
};
