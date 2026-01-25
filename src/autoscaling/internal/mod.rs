//! Kubernetes Autoscaling API Internal Types
//!
//! This module contains type definitions from k8s.io/kubernetes/pkg/apis/autoscaling/types.go
//! that are used internally by the Kubernetes API.
//!
//! Source: https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/autoscaling/types.go

use crate::common::{HasObjectMeta, LabelSelector, ObjectMeta, Quantity};
use crate::core::v1::resource::ResourceName;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

// ============================================================================
// Condition Related Enums
// ============================================================================

/// ConditionStatus indicates the status of a condition (true, false, or unknown).
///
/// These are valid condition statuses. "ConditionTrue" means a resource is in the condition;
/// "ConditionFalse" means a resource is not in the condition; "ConditionUnknown" means kubernetes
/// can't decide if a resource is in the condition or not. In the future, we could add other
/// intermediate conditions, e.g. ConditionDegraded.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum ConditionStatus {
    /// The condition is true.
    #[serde(rename = "True")]
    True,
    /// The condition is false.
    #[serde(rename = "False")]
    False,
    /// The condition status is unknown.
    #[serde(rename = "Unknown")]
    #[default]
    Unknown,
}

pub mod condition_status {
    pub const TRUE: &str = "True";
    pub const FALSE: &str = "False";
    pub const UNKNOWN: &str = "Unknown";
}

// ============================================================================
// Scaling Policy Enums
// ============================================================================

/// ScalingPolicySelect is used to specify which policy should be used while scaling in a certain direction
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum ScalingPolicySelect {
    /// MaxPolicySelect selects the policy with the highest possible change.
    #[serde(rename = "Max")]
    Max,
    /// MinPolicySelect selects the policy with the lowest possible change.
    #[serde(rename = "Min")]
    Min,
    /// DisabledPolicySelect disables the scaling in this direction.
    #[serde(rename = "Disabled")]
    Disabled,
}

pub mod scaling_policy_select {
    pub const MAX: &str = "Max";
    pub const MIN: &str = "Min";
    pub const DISABLED: &str = "Disabled";
}

/// HPAScalingPolicyType is the type of the policy which could be used while making scaling decisions.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum HPAScalingPolicyType {
    /// PodsScalingPolicy is a policy used to specify a change in absolute number of pods.
    #[serde(rename = "Pods")]
    #[default]
    Pods,
    /// PercentScalingPolicy is a policy used to specify a relative amount of change with respect to
    /// the current number of pods.
    #[serde(rename = "Percent")]
    Percent,
}

pub mod hpa_scaling_policy_type {
    pub const PODS: &str = "Pods";
    pub const PERCENT: &str = "Percent";
}

// ============================================================================
// Metric Source Type Enums
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
    /// ExternalMetricSourceType is a global metric that is not associated
    /// with any Kubernetes object. It allows autoscaling based on information
    /// coming from components running outside of cluster
    /// (for example length of queue in cloud messaging service, or
    /// QPS from loadbalancer running outside of cluster).
    #[serde(rename = "External")]
    External,
    /// ContainerResourceMetricSourceType is a resource metric known to Kubernetes, as
    /// specified in requests and limits, describing a single container in each pod in the current
    /// scale target (e.g. CPU or memory).  Such metrics are built in to
    /// Kubernetes, and have special scaling options on top of those available
    /// to normal per-pod metrics (the "pods" source).
    #[serde(rename = "ContainerResource")]
    ContainerResource,
}

pub mod metric_source_type {
    pub const OBJECT: &str = "Object";
    pub const PODS: &str = "Pods";
    pub const RESOURCE: &str = "Resource";
    pub const EXTERNAL: &str = "External";
    pub const CONTAINER_RESOURCE: &str = "ContainerResource";
}

/// MetricTargetType specifies the type of metric being targeted, and should be either
/// "Value", "AverageValue", or "Utilization"
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum MetricTargetType {
    /// UtilizationMetricType is a possible value for MetricTarget.Type.
    #[serde(rename = "Utilization")]
    #[default]
    Utilization,
    /// ValueMetricType is a possible value for MetricTarget.Type.
    #[serde(rename = "Value")]
    Value,
    /// AverageValueMetricType is a possible value for MetricTarget.Type.
    #[serde(rename = "AverageValue")]
    AverageValue,
}

pub mod metric_target_type {
    pub const UTILIZATION: &str = "Utilization";
    pub const VALUE: &str = "Value";
    pub const AVERAGE_VALUE: &str = "AverageValue";
}

// ============================================================================
// HPA Condition Type Enums
// ============================================================================

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

// ============================================================================
// Core Types
// ============================================================================

/// Scale represents a scaling request for a resource.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct Scale {
    /// Standard object metadata; More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::common::ObjectMeta>,
    /// spec defines the behavior of the scale. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<ScaleSpec>,
    /// status represents the current status of the scale. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status. Read-only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ScaleStatus>,
}

/// ScaleSpec describes the attributes of a scale subresource.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct ScaleSpec {
    /// replicas is the desired number of instances for the scaled object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
}

/// ScaleStatus represents the current status of a scale subresource.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct ScaleStatus {
    /// replicas is the actual number of observed instances of the scaled object.
    #[serde(default)]
    pub replicas: i32,
    /// label query over pods that should match the replicas count. This is same
    /// as the label selector but in the string format to avoid introspection
    /// by clients. The string will be in the same format as the query-param syntax.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
}

/// CrossVersionObjectReference contains enough information to let you identify the referred resource.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct CrossVersionObjectReference {
    /// kind is the kind of the referent; More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds"
    #[serde(default)]
    pub kind: String,
    /// name is the name of the referent; More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default)]
    pub name: String,
    /// apiVersion is the API version of the referent
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
}

/// HorizontalPodAutoscalerSpec describes the desired functionality of the HorizontalPodAutoscaler.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct HorizontalPodAutoscalerSpec {
    /// scaleTargetRef points to the target resource to scale, and is used to the pods for which metrics
    /// should be collected, as well as to actually change the replica count.
    #[serde(default)]
    pub scale_target_ref: CrossVersionObjectReference,
    /// minReplicas is the lower limit for the number of replicas to which the autoscaler
    /// can scale down.  It defaults to 1 pod.  minReplicas is allowed to be 0 if the
    /// alpha feature gate HPAScaleToZero is enabled and at least one Object or External
    /// metric is configured.  Scaling is active as long as at least one metric value is
    /// available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_replicas: Option<i32>,
    /// maxReplicas is the upper limit for the number of replicas to which the autoscaler can scale up.
    /// It cannot be less that minReplicas.
    #[serde(default)]
    pub max_replicas: i32,
    /// metrics contains the specifications for which to use to calculate the
    /// desired replica count (the maximum replica count across all metrics will
    /// be used).  The desired replica count is calculated multiplying the
    /// ratio between the target value and the current value by the current
    /// number of pods.  Ergo, metrics used must decrease as the pod count is
    /// increased, and vice-versa.  See the individual metric source types for
    /// more information about how each type of metric must respond.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metrics: Vec<MetricSpec>,
    /// behavior configures the scaling behavior of the target
    /// in both Up and Down directions (scaleUp and scaleDown fields respectively).
    /// If not set, the default HPAScalingRules for scale up and scale down are used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub behavior: Option<HorizontalPodAutoscalerBehavior>,
}

/// HorizontalPodAutoscalerBehavior configures a scaling behavior for Up and Down direction
/// (scaleUp and scaleDown fields respectively).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct HorizontalPodAutoscalerBehavior {
    /// scaleUp is scaling policy for scaling Up.
    /// If not set, the default value is the higher of:
    ///
    ///   * increase no more than 4 pods per 60 seconds
    ///   * double the number of pods per 60 seconds
    ///     No stabilization is used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale_up: Option<HPAScalingRules>,
    /// scaleDown is scaling policy for scaling Down.
    /// If not set, the default value is to allow to scale down to minReplicas pods, with a
    /// 300 second stabilization window (i.e., the highest recommendation for
    /// the last 300sec is used).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale_down: Option<HPAScalingRules>,
}

/// HPAScalingRules configures the scaling behavior for one direction via
/// scaling Policy Rules and a configurable metric tolerance.
///
/// Scaling Policy Rules are applied after calculating DesiredReplicas from metrics for the HPA.
/// They can limit the scaling velocity by specifying scaling policies.
/// They can prevent flapping by specifying the stabilization window, so that the
/// number of replicas is not set instantly, instead, the safest value from the stabilization
/// window is chosen.
///
/// The tolerance is applied to the metric values and prevents scaling too
/// eagerly for small metric variations. (Note that setting a tolerance requires
/// enabling the alpha HPAConfigurableTolerance feature gate.)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct HPAScalingRules {
    /// StabilizationWindowSeconds is the number of seconds for which past recommendations should be
    /// considered while scaling up or scaling down.
    /// StabilizationWindowSeconds must be greater than or equal to zero and less than or equal to 3600 (one hour).
    /// If not set, use the default values:
    /// - For scale up: 0 (i.e. no stabilization is done).
    /// - For scale down: 300 (i.e. the stabilization window is 300 seconds long).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stabilization_window_seconds: Option<i32>,
    /// selectPolicy is used to specify which policy should be used.
    /// If not set, the default value MaxPolicySelect is used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub select_policy: Option<ScalingPolicySelect>,
    /// policies is a list of potential scaling polices which can be used during scaling.
    /// If not set, use the default values:
    /// - For scale up: allow doubling the number of pods, or an absolute change of 4 pods in a 15s window.
    /// - For scale down: allow all pods to be removed in a 15s window.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policies: Vec<HPAScalingPolicy>,
    /// tolerance is the tolerance on the ratio between the current and desired
    /// metric value under which no updates are made to the desired number of
    /// replicas (e.g. 0.01 for 1%). Must be greater than or equal to zero. If not
    /// set, the default cluster-wide tolerance is applied (by default 10%).
    ///
    /// For example, if autoscaling is configured with a memory consumption target of 100Mi,
    /// and scale-down and scale-up tolerances of 5% and 1% respectively, scaling will be
    /// triggered when the actual consumption falls below 95Mi or exceeds 101Mi.
    ///
    /// This is an alpha field and requires enabling the HPAConfigurableTolerance
    /// feature gate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tolerance: Option<Quantity>,
}

/// HPAScalingPolicy is a single policy which must hold true for a specified past interval.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct HPAScalingPolicy {
    /// Type is used to specify the scaling policy.
    #[serde(default)]
    pub type_: HPAScalingPolicyType,
    /// Value contains the amount of change which is permitted by the policy.
    /// It must be greater than zero
    #[serde(default)]
    pub value: i32,
    /// PeriodSeconds specifies the window of time for which the policy should hold true.
    /// PeriodSeconds must be greater than zero and less than or equal to 1800 (30 min).
    #[serde(default)]
    pub period_seconds: i32,
}

/// MetricSpec specifies how to scale based on a single metric
/// (only `type` and one other matching field should be set at once).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct MetricSpec {
    /// Type is the type of metric source.  It should be one of "Object",
    /// "Pods" or "Resource", each mapping to a matching field in the object.
    #[serde(default)]
    pub type_: MetricSourceType,
    /// Object refers to a metric describing a single kubernetes object
    /// (for example, hits-per-second on an Ingress object).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<ObjectMetricSource>,
    /// Pods refers to a metric describing each pod in the current scale target
    /// (for example, transactions-processed-per-second).  The values will be
    /// averaged together before being compared to the target value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pods: Option<PodsMetricSource>,
    /// Resource refers to a resource metric (such as those specified in
    /// requests and limits) known to Kubernetes describing each pod in the
    /// current scale target (e.g. CPU or memory). Such metrics are built in to
    /// Kubernetes, and have special scaling options on top of those available
    /// to normal per-pod metrics using the "pods" source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<ResourceMetricSource>,
    /// ContainerResource refers to a resource metric (such as those specified in
    /// requests and limits) known to Kubernetes describing a single container in each pod of the
    /// current scale target (e.g. CPU or memory). Such metrics are built in to
    /// Kubernetes, and have special scaling options on top of those available
    /// to normal per-pod metrics using the "pods" source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container_resource: Option<ContainerResourceMetricSource>,
    /// External refers to a global metric that is not associated
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
pub struct ObjectMetricSource {
    #[serde(default)]
    pub described_object: CrossVersionObjectReference,
    #[serde(default)]
    pub target: MetricTarget,
    #[serde(default)]
    pub metric: MetricIdentifier,
}

/// PodsMetricSource indicates how to scale on a metric describing each pod in
/// the current scale target (for example, transactions-processed-per-second).
/// The values will be averaged together before being compared to the target
/// value.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct PodsMetricSource {
    /// metric identifies the target metric by name and selector
    #[serde(default)]
    pub metric: MetricIdentifier,
    /// target specifies the target value for the given metric
    #[serde(default)]
    pub target: MetricTarget,
}

/// ResourceMetricSource indicates how to scale on a resource metric known to
/// Kubernetes, as specified in requests and limits, describing each pod in the
/// current scale target (e.g. CPU or memory).  The values will be averaged
/// together before being compared to the target.  Such metrics are built in to
/// Kubernetes, and have special scaling options on top of those available to
/// normal per-pod metrics using the "pods" source.  Only one "target" type
/// should be set.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct ResourceMetricSource {
    /// Name is the name of the resource in question.
    #[serde(default)]
    pub name: ResourceName,
    /// Target specifies the target value for the given metric
    #[serde(default)]
    pub target: MetricTarget,
}

/// ContainerResourceMetricSource indicates how to scale on a resource metric known to
/// Kubernetes, as specified in the requests and limits, describing a single container in
/// each of the pods of the current scale target(e.g. CPU or memory). The values will be
/// averaged together before being compared to the target. Such metrics are built into
/// Kubernetes, and have special scaling options on top of those available to
/// normal per-pod metrics using the "pods" source. Only one "target" type
/// should be set.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct ContainerResourceMetricSource {
    /// name is the name of the of the resource
    #[serde(default)]
    pub name: ResourceName,
    /// container is the name of the container in the pods of the scaling target.
    #[serde(default)]
    pub container: String,
    /// target specifies the target value for the given metric
    #[serde(default)]
    pub target: MetricTarget,
}

/// ExternalMetricSource indicates how to scale on a metric not associated with
/// any Kubernetes object (for example length of queue in cloud
/// messaging service, or QPS from loadbalancer running outside of cluster).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct ExternalMetricSource {
    /// Metric identifies the target metric by name and selector
    #[serde(default)]
    pub metric: MetricIdentifier,
    /// Target specifies the target value for the given metric
    #[serde(default)]
    pub target: MetricTarget,
}

/// MetricIdentifier defines the name and optionally selector for a metric
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct MetricIdentifier {
    /// Name is the name of the given metric
    #[serde(default)]
    pub name: String,
    /// Selector is the selector for the given metric
    /// it is the string-encoded form of a standard kubernetes label selector
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<LabelSelector>,
}

/// MetricTarget defines the target value, average value, or average utilization of a specific metric
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct MetricTarget {
    /// Type represents whether the metric type is Utilization, Value, or AverageValue
    #[serde(default)]
    pub type_: MetricTargetType,
    /// Value is the target value of the metric (as a quantity).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Quantity>,
    /// TargetAverageValue is the target value of the average of the
    /// metric across all relevant pods (as a quantity)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub average_value: Option<Quantity>,
    /// AverageUtilization is the target value of the average of the
    /// resource metric across all relevant pods, represented as a percentage of
    /// the requested value of the resource for the pods.
    /// Currently only valid for Resource metric source type
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub average_utilization: Option<i32>,
}

/// HorizontalPodAutoscalerStatus describes the current status of a horizontal pod autoscaler.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct HorizontalPodAutoscalerStatus {
    /// ObservedGeneration is the most recent generation observed by this autoscaler.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
    /// LastScaleTime is the last time the HorizontalPodAutoscaler scaled the number of pods,
    /// used by the autoscaler to control how often the number of pods is changed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_scale_time: Option<crate::common::Timestamp>,
    /// CurrentReplicas is current number of replicas of pods managed by this autoscaler,
    /// as last seen by the autoscaler.
    #[serde(default)]
    pub current_replicas: i32,
    /// DesiredReplicas is the desired number of replicas of pods managed by this autoscaler,
    /// as last calculated by the autoscaler.
    #[serde(default)]
    pub desired_replicas: i32,
    /// CurrentMetrics is the last read state of the metrics used by this autoscaler.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub current_metrics: Vec<MetricStatus>,
    /// Conditions is the set of conditions required for this autoscaler to scale its target,
    /// and indicates whether or not those conditions are met.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<HorizontalPodAutoscalerCondition>,
}

/// HorizontalPodAutoscalerCondition describes the state of
/// a HorizontalPodAutoscaler at a certain point.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct HorizontalPodAutoscalerCondition {
    /// Type describes the current condition
    #[serde(default)]
    pub type_: HorizontalPodAutoscalerConditionType,
    /// Status is the status of the condition (True, False, Unknown)
    #[serde(default)]
    pub status: ConditionStatus,
    /// LastTransitionTime is the last time the condition transitioned from
    /// one status to another
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<crate::common::Timestamp>,
    /// Reason is the reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Message is a human-readable explanation containing details about
    /// the transition
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// MetricStatus describes the last-read state of a single metric.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct MetricStatus {
    /// Type is the type of metric source.  It will be one of "Object",
    /// "Pods" or "Resource", each corresponds to a matching field in the object.
    #[serde(default)]
    pub type_: MetricSourceType,
    /// Object refers to a metric describing a single kubernetes object
    /// (for example, hits-per-second on an Ingress object).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<ObjectMetricStatus>,
    /// Pods refers to a metric describing each pod in the current scale target
    /// (for example, transactions-processed-per-second).  The values will be
    /// averaged together before being compared to the target value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pods: Option<PodsMetricStatus>,
    /// Resource refers to a resource metric (such as those specified in
    /// requests and limits) known to Kubernetes describing each pod in the
    /// current scale target (e.g. CPU or memory). Such metrics are built in to
    /// Kubernetes, and have special scaling options on top of those available
    /// to normal per-pod metrics using the "pods" source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<ResourceMetricStatus>,
    /// ContainerResource refers to a resource metric (such as those specified in
    /// requests and limits) known to Kubernetes describing a single container in each pod in the
    /// current scale target (e.g. CPU or memory). Such metrics are built in to
    /// Kubernetes, and have special scaling options on top of those available
    /// to normal per-pod metrics using the "pods" source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container_resource: Option<ContainerResourceMetricStatus>,
    /// External refers to a global metric that is not associated
    /// with any Kubernetes object. It allows autoscaling based on information
    /// coming from components running outside of cluster
    /// (for example length of queue in cloud messaging service, or
    /// QPS from loadbalancer running outside of cluster).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external: Option<ExternalMetricStatus>,
}

/// ObjectMetricStatus indicates the current value of a metric describing a
/// kubernetes object (for example, hits-per-second on an Ingress object).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct ObjectMetricStatus {
    #[serde(default)]
    pub metric: MetricIdentifier,
    #[serde(default)]
    pub current: MetricValueStatus,
    #[serde(default)]
    pub described_object: CrossVersionObjectReference,
}

/// PodsMetricStatus indicates the current value of a metric describing each pod in
/// the current scale target (for example, transactions-processed-per-second).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct PodsMetricStatus {
    #[serde(default)]
    pub metric: MetricIdentifier,
    #[serde(default)]
    pub current: MetricValueStatus,
}

/// ResourceMetricStatus indicates the current value of a resource metric known to
/// Kubernetes, as specified in requests and limits, describing each pod in the
/// current scale target (e.g. CPU or memory).  Such metrics are built in to
/// Kubernetes, and have special scaling options on top of those available to
/// normal per-pod metrics using the "pods" source.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct ResourceMetricStatus {
    /// name is the name of the resource in question.
    #[serde(default)]
    pub name: ResourceName,
    #[serde(default)]
    pub current: MetricValueStatus,
}

/// ContainerResourceMetricStatus indicates the current value of a resource metric known to
/// Kubernetes, as specified in requests and limits, describing each pod in the
/// current scale target (e.g. CPU or memory).  Such metrics are built in to
/// Kubernetes, and have special scaling options on top of those available to
/// normal per-pod metrics using the "pods" source.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct ContainerResourceMetricStatus {
    /// name is the name of the resource in question.
    #[serde(default)]
    pub name: ResourceName,
    #[serde(default)]
    pub container: String,
    #[serde(default)]
    pub current: MetricValueStatus,
}

/// ExternalMetricStatus indicates the current value of a global metric
/// not associated with any Kubernetes object.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct ExternalMetricStatus {
    #[serde(default)]
    pub metric: MetricIdentifier,
    #[serde(default)]
    pub current: MetricValueStatus,
}

/// MetricValueStatus indicates the current value of a metric.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct MetricValueStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Quantity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub average_value: Option<Quantity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub average_utilization: Option<i32>,
}

/// HorizontalPodAutoscaler is the configuration for a horizontal pod
/// autoscaler, which automatically manages the replica count of any resource
/// implementing the scale subresource based on the metrics specified.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct HorizontalPodAutoscaler {
    /// Metadata is the standard object metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::common::ObjectMeta>,
    /// spec is the specification for the behaviour of the autoscaler.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<HorizontalPodAutoscalerSpec>,
    /// status is the current information about the autoscaler.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<HorizontalPodAutoscalerStatus>,
}

/// HorizontalPodAutoscalerList is a list of horizontal pod autoscaler objects.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct HorizontalPodAutoscalerList {
    /// Metadata is the standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::common::ListMeta>,
    /// items is the list of horizontal pod autoscaler objects.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<HorizontalPodAutoscaler>,
}

// ============================================================================
// Trait Implementations
// ============================================================================

impl HasObjectMeta for HorizontalPodAutoscaler {
    fn meta(&self) -> &ObjectMeta {
        self.metadata
            .as_ref()
            .unwrap_or(static_default_object_meta())
    }

    fn meta_mut(&mut self) -> &mut ObjectMeta {
        self.metadata.get_or_insert_with(ObjectMeta::default)
    }
}

impl HasObjectMeta for Scale {
    fn meta(&self) -> &ObjectMeta {
        self.metadata
            .as_ref()
            .unwrap_or(static_default_object_meta())
    }

    fn meta_mut(&mut self) -> &mut ObjectMeta {
        self.metadata.get_or_insert_with(ObjectMeta::default)
    }
}

fn static_default_object_meta() -> &'static ObjectMeta {
    static DEFAULT: OnceLock<ObjectMeta> = OnceLock::new();
    DEFAULT.get_or_init(ObjectMeta::default)
}
