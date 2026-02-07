//! Kubernetes Autoscaling v1 API types
//!
//! This module contains types from the Kubernetes autoscaling/v1 API group.
//!
//! Source: https://github.com/kubernetes/api/blob/master/autoscaling/v1/types.go

pub mod conversion;
pub mod defaults;
pub mod validation;

use crate::common::{
    ApplyDefault, HasTypeMeta, LabelSelector, ListMeta, ObjectMeta, Quantity, ResourceSchema,
    Timestamp, TypeMeta, VersionedObject,
};
use crate::impl_unimplemented_prost_message;
use crate::impl_versioned_object;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

// ============================================================================
// CrossVersionObjectReference
// ============================================================================

/// CrossVersionObjectReference contains enough information to let you identify the referred resource.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CrossVersionObjectReference {
    /// kind is the kind of the referent; More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(default)]
    pub kind: String,
    /// name is the name of the referent; More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default)]
    pub name: String,
    /// apiVersion is the API version of the referent
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
}

// ============================================================================
// HorizontalPodAutoscaler
// ============================================================================

/// HorizontalPodAutoscalerSpec is the specification of a horizontal pod autoscaler.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct HorizontalPodAutoscalerSpec {
    /// reference to scaled resource; horizontal pod autoscaler will learn the current resource consumption
    /// and will set the desired number of pods by using its Scale subresource.
    #[serde(default)]
    pub scale_target_ref: CrossVersionObjectReference,
    /// minReplicas is the lower limit for the number of replicas to which the autoscaler
    /// can scale down.  It defaults to 1 pod.  minReplicas is allowed to be 0 if the
    /// alpha feature gate HPAScaleToZero is enabled and at least one Object or External
    /// metric is configured.  Scaling is active as long as at least one metric value is
    /// available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_replicas: Option<i32>,
    /// maxReplicas is the upper limit for the number of pods that can be set by the autoscaler; cannot be smaller than MinReplicas.
    #[serde(default)]
    pub max_replicas: i32,
    /// targetCPUUtilizationPercentage is the target average CPU utilization (represented as a percentage of requested CPU) over all the pods;
    /// if not specified the default autoscaling policy will be used.
    #[serde(
        rename = "targetCPUUtilizationPercentage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_cpu_utilization_percentage: Option<i32>,
}

/// HorizontalPodAutoscalerStatus is the current status of a horizontal pod autoscaler.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct HorizontalPodAutoscalerStatus {
    /// observedGeneration is the most recent generation observed by this autoscaler.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
    /// lastScaleTime is the last time the HorizontalPodAutoscaler scaled the number of pods;
    /// used by the autoscaler to control how often the number of pods is changed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_scale_time: Option<Timestamp>,
    /// currentReplicas is the current number of replicas of pods managed by this autoscaler.
    #[serde(default)]
    pub current_replicas: i32,
    /// desiredReplicas is the  desired number of replicas of pods managed by this autoscaler.
    #[serde(default)]
    pub desired_replicas: i32,
    /// currentCPUUtilizationPercentage is the current average CPU utilization over all pods, represented as a percentage of requested CPU,
    /// e.g. 70 means that an average pod is using now 70% of its requested CPU.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_cpu_utilization_percentage: Option<i32>,
}

/// HorizontalPodAutoscaler is the configuration for a horizontal pod autoscaler, which automatically manages the replica count of any resource
/// implementing the scale subresource.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct HorizontalPodAutoscaler {
    /// TypeMeta for this resource
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard object metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// spec defines the behaviour of autoscaler. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<HorizontalPodAutoscalerSpec>,
    /// status is the current information about the autoscaler.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<HorizontalPodAutoscalerStatus>,
}

/// HorizontalPodAutoscalerList is a list of horizontal pod autoscaler objects.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct HorizontalPodAutoscalerList {
    /// TypeMeta for this resource
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,
    /// items is the list of horizontal pod autoscaler objects.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<HorizontalPodAutoscaler>,
}

// ============================================================================
// Scale
// ============================================================================

/// Scale represents a scaling request for a resource.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Scale {
    /// Standard object metadata; More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// spec defines the behavior of the scale. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<ScaleSpec>,
    /// status is the current status of the scale. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status. Read-only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ScaleStatus>,
}
impl_versioned_object!(Scale);

/// ScaleSpec describes the attributes of a scale subresource.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScaleSpec {
    /// replicas is the desired number of instances for the scaled object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
}

/// ScaleStatus represents the current status of a scale subresource.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScaleStatus {
    /// replicas is the actual number of observed instances of the scaled object.
    #[serde(default)]
    pub replicas: i32,
    /// selector is the label query over pods that should match the replicas count. This is same
    /// as the label selector but in the string format to avoid introspection
    /// by clients. The string will be in the same format as the query-param syntax.
    /// More info about label selectors: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
}

// ============================================================================
// Metrics (alpha types used in annotations)
// ============================================================================

/// MetricSourceType indicates the type of metric.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum MetricSourceType {
    /// ObjectMetricSourceType is a metric describing a kubernetes object
    /// (for example, hits-per-second on an Ingress object).
    #[serde(rename = "Object")]
    #[default]
    Object,
    /// PodsMetricSourceType is a metric describing each pod in the current scale
    /// target (for example, transactions-processed-per-second).  The values
    /// will be averaged together before being compared to the target value.
    #[serde(rename = "Pods")]
    Pods,
    /// ResourceMetricSourceType is a resource metric known to Kubernetes, as
    /// specified in requests and limits, describing each pod in the current
    /// scale target (e.g. CPU or memory).  Such metrics are built in to
    /// Kubernetes, and have special scaling options on top of those available
    /// to normal per-pod metrics (the "pods" source).
    #[serde(rename = "Resource")]
    Resource,
    /// ContainerResourceMetricSourceType is a resource metric known to Kubernetes, as
    /// specified in requests and limits, describing a single container in each pod in the current
    /// scale target (e.g. CPU or memory).  Such metrics are built in to
    /// Kubernetes, and have special scaling options on top of those available
    /// to normal per-pod metrics (the "pods" source).
    #[serde(rename = "ContainerResource")]
    ContainerResource,
    /// ExternalMetricSourceType is a global metric that is not associated
    /// with any Kubernetes object. It allows autoscaling based on information
    /// coming from components running outside of cluster
    /// (for example length of queue in cloud messaging service, or
    /// QPS from loadbalancer running outside of cluster).
    #[serde(rename = "External")]
    External,
}

pub mod metric_source_type {
    pub const OBJECT: &str = "Object";
    pub const PODS: &str = "Pods";
    pub const RESOURCE: &str = "Resource";
    pub const CONTAINER_RESOURCE: &str = "ContainerResource";
    pub const EXTERNAL: &str = "External";
}

/// MetricSpec specifies how to scale based on a single metric
/// (only `type` and one other matching field should be set at once).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MetricSpec {
    /// type is the type of metric source.  It should be one of "ContainerResource",
    /// "External", "Object", "Pods" or "Resource", each mapping to a matching field in the object.
    #[serde(default)]
    pub type_: MetricSourceType,
    /// object refers to a metric describing a single kubernetes object
    /// (for example, hits-per-second on an Ingress object).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<ObjectMetricSource>,
    /// pods refers to a metric describing each pod in the current scale target
    /// (for example, transactions-processed-per-second).  The values will be
    /// averaged together before being compared to the target value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pods: Option<PodsMetricSource>,
    /// resource refers to a resource metric (such as those specified in
    /// requests and limits) known to Kubernetes describing each pod in the
    /// current scale target (e.g. CPU or memory). Such metrics are built in to
    /// Kubernetes, and have special scaling options on top of those available
    /// to normal per-pod metrics using the "pods" source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<ResourceMetricSource>,
    /// containerResource refers to a resource metric (such as those specified in
    /// requests and limits) known to Kubernetes describing a single container in each pod of the
    /// current scale target (e.g. CPU or memory). Such metrics are built in to
    /// Kubernetes, and have special scaling options on top of those available
    /// to normal per-pod metrics using the "pods" source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container_resource: Option<ContainerResourceMetricSource>,
    /// external refers to a global metric that is not associated
    /// with any Kubernetes object. It allows autoscaling based on information
    /// coming from components running outside of cluster
    /// (for example length of queue in cloud messaging service, or
    /// QPS from loadbalancer running outside of cluster).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external: Option<ExternalMetricSource>,
}

/// ObjectMetricSource indicates how to scale on a metric describing a
/// kubernetes object (for example, hits-per-second on an Ingress object).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ObjectMetricSource {
    /// target is the described Kubernetes object.
    #[serde(default)]
    pub target: CrossVersionObjectReference,
    /// metricName is the name of the metric in question.
    #[serde(default)]
    pub metric_name: String,
    /// targetValue is the target value of the metric (as a quantity).
    #[serde(default)]
    pub target_value: Quantity,
    /// selector is the string-encoded form of a standard kubernetes label selector for the given metric.
    /// When set, it is passed as an additional parameter to the metrics server for more specific metrics scoping
    /// When unset, just the metricName will be used to gather metrics.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<LabelSelector>,
    /// averageValue is the target value of the average of the
    /// metric across all relevant pods (as a quantity)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub average_value: Option<Quantity>,
}

/// PodsMetricSource indicates how to scale on a metric describing each pod in
/// the current scale target (for example, transactions-processed-per-second).
/// The values will be averaged together before being compared to the target
/// value.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodsMetricSource {
    /// metricName is the name of the metric in question
    #[serde(default)]
    pub metric_name: String,
    /// targetAverageValue is the target value of the average of the
    /// metric across all relevant pods (as a quantity)
    #[serde(default)]
    pub target_average_value: Quantity,
    /// selector is the string-encoded form of a standard kubernetes label selector for the given metric
    /// When set, it is passed as an additional parameter to the metrics server for more specific metrics scoping
    /// When unset, just the metricName will be used to gather metrics.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<LabelSelector>,
}

/// ResourceMetricSource indicates how to scale on a resource metric known to
/// Kubernetes, as specified in requests and limits, describing each pod in the
/// current scale target (e.g. CPU or memory).  The values will be averaged
/// together before being compared to the target.  Such metrics are built in to
/// Kubernetes, and have special scaling options on top of those available to
/// normal per-pod metrics using the "pods" source.  Only one "target" type
/// should be set.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMetricSource {
    /// name is the name of the resource in question.
    #[serde(default)]
    pub name: String,
    /// targetAverageUtilization is the target value of the average of the
    /// resource metric across all relevant pods, represented as a percentage of
    /// the requested value of the resource for the pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_average_utilization: Option<i32>,
    /// targetAverageValue is the target value of the average of the
    /// resource metric across all relevant pods, as a raw value (instead of as
    /// a percentage of the request), similar to the "pods" metric source type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_average_value: Option<Quantity>,
}

/// ContainerResourceMetricSource indicates how to scale on a resource metric known to
/// Kubernetes, as specified in the requests and limits, describing a single container in
/// each of the pods of the current scale target(e.g. CPU or memory). The values will be
/// averaged together before being compared to the target. Such metrics are built into
/// Kubernetes, and have special scaling options on top of those available to
/// normal per-pod metrics using the "pods" source. Only one "target" type
/// should be set.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContainerResourceMetricSource {
    /// name is the name of the resource in question.
    #[serde(default)]
    pub name: String,
    /// targetAverageUtilization is the target value of the average of the
    /// resource metric across all relevant pods, represented as a percentage of
    /// the requested value of the resource for the pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_average_utilization: Option<i32>,
    /// targetAverageValue is the target value of the average of the
    /// resource metric across all relevant pods, as a raw value (instead of as
    /// a percentage of the request), similar to the "pods" metric source type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_average_value: Option<Quantity>,
    /// container is the name of the container in the pods of the scaling target.
    #[serde(default)]
    pub container: String,
}

/// ExternalMetricSource indicates how to scale on a metric not associated with
/// any Kubernetes object (for example length of queue in cloud
/// messaging service, or QPS from loadbalancer running outside of cluster).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExternalMetricSource {
    /// metricName is the name of the metric in question.
    #[serde(default)]
    pub metric_name: String,
    /// metricSelector is used to identify a specific time series
    /// within a given metric.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric_selector: Option<LabelSelector>,
    /// targetValue is the target value of the metric (as a quantity).
    /// Mutually exclusive with TargetAverageValue.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_value: Option<Quantity>,
    /// targetAverageValue is the target per-pod value of global metric (as a quantity).
    /// Mutually exclusive with TargetValue.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_average_value: Option<Quantity>,
}

/// MetricStatus describes the last-read state of a single metric.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MetricStatus {
    /// type is the type of metric source.  It will be one of "ContainerResource",
    /// "External", "Object", "Pods" or "Resource", each corresponds to a matching field in the object.
    #[serde(default)]
    pub type_: MetricSourceType,
    /// object refers to a metric describing a single kubernetes object
    /// (for example, hits-per-second on an Ingress object).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<ObjectMetricStatus>,
    /// pods refers to a metric describing each pod in the current scale target
    /// (for example, transactions-processed-per-second).  The values will be
    /// averaged together before being compared to the target value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pods: Option<PodsMetricStatus>,
    /// resource refers to a resource metric (such as those specified in
    /// requests and limits) known to Kubernetes describing each pod in the
    /// current scale target (e.g. CPU or memory). Such metrics are built in to
    /// Kubernetes, and have special scaling options on top of those available
    /// to normal per-pod metrics using the "pods" source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<ResourceMetricStatus>,
    /// containerResource refers to a resource metric (such as those specified in
    /// requests and limits) known to Kubernetes describing a single container in each pod in the
    /// current scale target (e.g. CPU or memory). Such metrics are built in to
    /// Kubernetes, and have special scaling options on top of those available
    /// to normal per-pod metrics using the "pods" source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container_resource: Option<ContainerResourceMetricStatus>,
    /// external refers to a global metric that is not associated
    /// with any Kubernetes object. It allows autoscaling based on information
    /// coming from components running outside of cluster
    /// (for example length of queue in cloud messaging service, or
    /// QPS from loadbalancer running outside of cluster).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external: Option<ExternalMetricStatus>,
}

/// HorizontalPodAutoscalerConditionType are the valid conditions of
/// a HorizontalPodAutoscaler.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum HorizontalPodAutoscalerConditionType {
    /// ScalingActive indicates that the HPA controller is able to scale if necessary:
    /// it's correctly configured, can fetch the desired metrics, and isn't disabled.
    #[serde(rename = "ScalingActive")]
    #[default]
    ScalingActive,
    /// AbleToScale indicates a lack of transient issues which prevent scaling from occurring,
    /// such as being in a backoff window, or being unable to access/update the target scale.
    #[serde(rename = "AbleToScale")]
    AbleToScale,
    /// ScalingLimited indicates that the calculated scale based on metrics would be above or
    /// below the range for the HPA, and has thus been capped.
    #[serde(rename = "ScalingLimited")]
    ScalingLimited,
}

pub mod horizontal_pod_autoscaler_condition_type {
    pub const SCALING_ACTIVE: &str = "ScalingActive";
    pub const ABLE_TO_SCALE: &str = "AbleToScale";
    pub const SCALING_LIMITED: &str = "ScalingLimited";
}

/// HorizontalPodAutoscalerCondition describes the state of
/// a HorizontalPodAutoscaler at a certain point.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct HorizontalPodAutoscalerCondition {
    /// type describes the current condition
    #[serde(default)]
    pub type_: HorizontalPodAutoscalerConditionType,
    /// status is the status of the condition (True, False, Unknown)
    #[serde(default)]
    pub status: String,
    /// lastTransitionTime is the last time the condition transitioned from
    /// one status to another
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<Timestamp>,
    /// reason is the reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// message is a human-readable explanation containing details about
    /// the transition
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// ObjectMetricStatus indicates the current value of a metric describing a
/// kubernetes object (for example, hits-per-second on an Ingress object).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ObjectMetricStatus {
    /// target is the described Kubernetes object.
    #[serde(default)]
    pub target: CrossVersionObjectReference,
    /// metricName is the name of the metric in question.
    #[serde(default)]
    pub metric_name: String,
    /// currentValue is the current value of the metric (as a quantity).
    #[serde(default)]
    pub current_value: Quantity,
    /// selector is the string-encoded form of a standard kubernetes label selector for the given metric
    /// When set in the ObjectMetricSource, it is passed as an additional parameter to the metrics server for more specific metrics scoping.
    /// When unset, just the metricName will be used to gather metrics.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<LabelSelector>,
    /// averageValue is the current value of the average of the
    /// metric across all relevant pods (as a quantity)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub average_value: Option<Quantity>,
}

/// PodsMetricStatus indicates the current value of a metric describing each pod in
/// the current scale target (for example, transactions-processed-per-second).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodsMetricStatus {
    /// metricName is the name of the metric in question
    #[serde(default)]
    pub metric_name: String,
    /// currentAverageValue is the current value of the average of the
    /// metric across all relevant pods (as a quantity)
    #[serde(default)]
    pub current_average_value: Quantity,
    /// selector is the string-encoded form of a standard kubernetes label selector for the given metric
    /// When set in the PodsMetricSource, it is passed as an additional parameter to the metrics server for more specific metrics scoping.
    /// When unset, just the metricName will be used to gather metrics.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<LabelSelector>,
}

/// ResourceMetricStatus indicates the current value of a resource metric known to
/// Kubernetes, as specified in requests and limits, describing each pod in the
/// current scale target (e.g. CPU or memory).  Such metrics are built in to
/// Kubernetes, and have special scaling options on top of those available to
/// normal per-pod metrics using the "pods" source.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMetricStatus {
    /// name is the name of the resource in question.
    #[serde(default)]
    pub name: String,
    /// currentAverageUtilization is the current value of the average of the
    /// resource metric across all relevant pods, represented as a percentage of
    /// the requested value of the resource for the pods.  It will only be
    /// present if `targetAverageValue` was set in the corresponding metric
    /// specification.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_average_utilization: Option<i32>,
    /// currentAverageValue is the current value of the average of the
    /// resource metric across all relevant pods, as a raw value (instead of as
    /// a percentage of the request), similar to the "pods" metric source type.
    /// It will always be set, regardless of the corresponding metric specification.
    #[serde(default)]
    pub current_average_value: Quantity,
}

/// ContainerResourceMetricStatus indicates the current value of a resource metric known to
/// Kubernetes, as specified in requests and limits, describing a single container in each pod in the
/// current scale target (e.g. CPU or memory).  Such metrics are built in to
/// Kubernetes, and have special scaling options on top of those available to
/// normal per-pod metrics using the "pods" source.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContainerResourceMetricStatus {
    /// name is the name of the resource in question.
    #[serde(default)]
    pub name: String,
    /// currentAverageUtilization is the current value of the average of the
    /// resource metric across all relevant pods, represented as a percentage of
    /// the requested value of the resource for the pods.  It will only be
    /// present if `targetAverageValue` was set in the corresponding metric
    /// specification.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_average_utilization: Option<i32>,
    /// currentAverageValue is the current value of the average of the
    /// resource metric across all relevant pods, as a raw value (instead of as
    /// a percentage of the request), similar to the "pods" metric source type.
    /// It will always be set, regardless of the corresponding metric specification.
    #[serde(default)]
    pub current_average_value: Quantity,
    /// container is the name of the container in the pods of the scaling taget
    #[serde(default)]
    pub container: String,
}

/// ExternalMetricStatus indicates the current value of a global metric
/// not associated with any Kubernetes object.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExternalMetricStatus {
    /// metricName is the name of a metric used for autoscaling in
    /// metric system.
    #[serde(default)]
    pub metric_name: String,
    /// metricSelector is used to identify a specific time series
    /// within a given metric.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric_selector: Option<LabelSelector>,
    /// currentValue is the current value of the metric (as a quantity)
    #[serde(default)]
    pub current_value: Quantity,
    /// currentAverageValue is the current value of metric averaged over autoscaled pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_average_value: Option<Quantity>,
}

// ============================================================================
// Trait Implementations for HorizontalPodAutoscaler and HorizontalPodAutoscalerList
// ============================================================================

// ----------------------------------------------------------------------------
// ResourceSchema Implementation
// ----------------------------------------------------------------------------

impl ResourceSchema for HorizontalPodAutoscaler {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "autoscaling"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "HorizontalPodAutoscaler"
    }
    fn resource(_: &Self::Meta) -> &str {
        "horizontalpodautoscalers"
    }

    fn group_static() -> &'static str {
        "autoscaling"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "HorizontalPodAutoscaler"
    }
    fn resource_static() -> &'static str {
        "horizontalpodautoscalers"
    }
}

impl ResourceSchema for HorizontalPodAutoscalerList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "autoscaling"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "HorizontalPodAutoscalerList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "horizontalpodautoscalers"
    }

    fn group_static() -> &'static str {
        "autoscaling"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "HorizontalPodAutoscalerList"
    }
    fn resource_static() -> &'static str {
        "horizontalpodautoscalers"
    }
}

// ----------------------------------------------------------------------------
// HasTypeMeta Implementation
// ----------------------------------------------------------------------------

impl HasTypeMeta for HorizontalPodAutoscaler {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for HorizontalPodAutoscalerList {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

// ----------------------------------------------------------------------------
// VersionedObject Implementation
// ----------------------------------------------------------------------------

impl VersionedObject for HorizontalPodAutoscaler {
    fn metadata(&self) -> &ObjectMeta {
        self.metadata
            .as_ref()
            .unwrap_or_else(|| static_default_object_meta())
    }

    fn metadata_mut(&mut self) -> &mut ObjectMeta {
        self.metadata.get_or_insert_with(ObjectMeta::default)
    }
}

// Helper function for static default ObjectMeta
fn static_default_object_meta() -> &'static ObjectMeta {
    static DEFAULT: OnceLock<ObjectMeta> = OnceLock::new();
    DEFAULT.get_or_init(ObjectMeta::default)
}

// Note: HorizontalPodAutoscalerList does not implement VersionedObject because its metadata is ListMeta

// ----------------------------------------------------------------------------
// ApplyDefaults Implementation
// ----------------------------------------------------------------------------

impl ApplyDefault for HorizontalPodAutoscaler {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "autoscaling/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "HorizontalPodAutoscaler".to_string();
        }
        crate::autoscaling::v1::defaults::set_defaults_horizontal_pod_autoscaler(self);
    }
}

impl ApplyDefault for HorizontalPodAutoscalerList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "autoscaling/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "HorizontalPodAutoscalerList".to_string();
        }
        for item in &mut self.items {
            item.apply_default();
        }
    }
}

// ----------------------------------------------------------------------------
// Protobuf Placeholder (using macro)
// ----------------------------------------------------------------------------

impl_unimplemented_prost_message!(HorizontalPodAutoscaler);
impl_unimplemented_prost_message!(HorizontalPodAutoscalerList);
impl_unimplemented_prost_message!(Scale);

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod trait_tests;

#[cfg(test)]
mod tests {}

#[cfg(test)]
mod serde_roundtrip_tests;

#[cfg(test)]
mod conversion_roundtrip_tests;

// AsRefStr / AsRef<str> implementations for enums
crate::impl_as_str_ref!(MetricSourceType, {
    Object => metric_source_type::OBJECT,
    Pods => metric_source_type::PODS,
    Resource => metric_source_type::RESOURCE,
    ContainerResource => metric_source_type::CONTAINER_RESOURCE,
    External => metric_source_type::EXTERNAL,
});

crate::impl_as_str_ref!(HorizontalPodAutoscalerConditionType, {
    ScalingActive => horizontal_pod_autoscaler_condition_type::SCALING_ACTIVE,
    AbleToScale => horizontal_pod_autoscaler_condition_type::ABLE_TO_SCALE,
    ScalingLimited => horizontal_pod_autoscaler_condition_type::SCALING_LIMITED,
});
