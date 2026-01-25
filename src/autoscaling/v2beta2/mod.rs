//! Kubernetes Autoscaling v2beta2 API types
//!
//! This module contains types from the Kubernetes autoscaling/v2beta2 API group.
//!
//! Source: https://github.com/kubernetes/api/blob/master/autoscaling/v2beta2/types.go

pub mod conversion;
pub mod defaults;

use crate::common::{
    ApplyDefault, HasTypeMeta, LabelSelector, ListMeta, ObjectMeta, Quantity, ResourceSchema,
    Timestamp, TypeMeta,
};
use crate::impl_unimplemented_prost_message;
use crate::impl_versioned_object;
use serde::{Deserialize, Serialize};

// ============================================================================
// HorizontalPodAutoscaler
// ============================================================================

/// HorizontalPodAutoscaler is the configuration for a horizontal pod autoscaler.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct HorizontalPodAutoscaler {
    /// TypeMeta for this resource
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// metadata is the standard object metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// spec is the specification for the behaviour of the autoscaler.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<HorizontalPodAutoscalerSpec>,

    /// status is the current information about the autoscaler.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<HorizontalPodAutoscalerStatus>,
}
impl_versioned_object!(HorizontalPodAutoscaler);

/// HorizontalPodAutoscalerList is a list of horizontal pod autoscaler objects.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct HorizontalPodAutoscalerList {
    /// TypeMeta for this resource
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// metadata is the standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// items is the list of horizontal pod autoscaler objects.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<HorizontalPodAutoscaler>,
}

/// HorizontalPodAutoscalerSpec describes the desired functionality of the HorizontalPodAutoscaler.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct HorizontalPodAutoscalerSpec {
    /// scaleTargetRef points to the target resource to scale.
    #[serde(default)]
    pub scale_target_ref: CrossVersionObjectReference,
    /// minReplicas is the lower limit for the number of replicas to which the autoscaler can scale down.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_replicas: Option<i32>,
    /// maxReplicas is the upper limit for the number of replicas to which the autoscaler can scale up.
    #[serde(default)]
    pub max_replicas: i32,
    /// metrics contains the specifications for which to use to calculate the desired replica count.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metrics: Vec<MetricSpec>,
    /// behavior configures the scaling behavior of the target.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub behavior: Option<HorizontalPodAutoscalerBehavior>,
}

/// CrossVersionObjectReference contains enough information to let you identify the referred resource.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CrossVersionObjectReference {
    /// kind is the kind of the referent.
    #[serde(default)]
    pub kind: String,
    /// name is the name of the referent.
    #[serde(default)]
    pub name: String,
    /// apiVersion is the API version of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
}

// ============================================================================
// Scaling Behavior
// ============================================================================

/// HorizontalPodAutoscalerBehavior configures the scaling behavior of the target.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct HorizontalPodAutoscalerBehavior {
    /// scaleUp is scaling policy for scaling Up.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale_up: Option<HPAScalingRules>,
    /// scaleDown is scaling policy for scaling Down.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale_down: Option<HPAScalingRules>,
}

/// ScalingPolicySelect is used to specify which policy should be used while scaling in a certain direction.
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

/// HPAScalingRules configures the scaling behavior for one direction.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct HPAScalingRules {
    /// stabilizationWindowSeconds is the number of seconds for which past recommendations should be considered.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stabilization_window_seconds: Option<i32>,
    /// selectPolicy is used to specify which policy should be used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub select_policy: Option<ScalingPolicySelect>,
    /// policies is a list of potential scaling polices which can be used during scaling.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policies: Vec<HPAScalingPolicy>,
}

/// HPAScalingPolicyType is the type of the policy which could be used while making scaling decisions.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum HPAScalingPolicyType {
    /// PodsScalingPolicy is a policy used to specify a change in absolute number of pods.
    #[serde(rename = "Pods")]
    #[default]
    Pods,
    /// PercentScalingPolicy is a policy used to specify a relative amount of change.
    #[serde(rename = "Percent")]
    Percent,
}

pub mod hpa_scaling_policy_type {
    pub const PODS: &str = "Pods";
    pub const PERCENT: &str = "Percent";
}

/// HPAScalingPolicy is a single policy which must hold true for a specified past interval.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct HPAScalingPolicy {
    /// type is used to specify the scaling policy.
    #[serde(default)]
    pub type_: HPAScalingPolicyType,
    /// value contains the amount of change which is permitted by the policy.
    #[serde(default)]
    pub value: i32,
    /// periodSeconds specifies the window of time for which the policy should hold true.
    #[serde(default)]
    pub period_seconds: i32,
}

// ============================================================================
// Metrics
// ============================================================================

/// MetricSourceType indicates the type of metric.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum MetricSourceType {
    /// ObjectMetricSourceType is a metric describing a kubernetes object.
    #[serde(rename = "Object")]
    #[default]
    Object,
    /// PodsMetricSourceType is a metric describing each pod in the current scale target.
    #[serde(rename = "Pods")]
    Pods,
    /// ResourceMetricSourceType is a resource metric known to Kubernetes.
    #[serde(rename = "Resource")]
    Resource,
    /// ContainerResourceMetricSourceType is a resource metric describing a single container.
    #[serde(rename = "ContainerResource")]
    ContainerResource,
    /// ExternalMetricSourceType is a global metric that is not associated with any Kubernetes object.
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

/// MetricSpec specifies how to scale based on a single metric.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MetricSpec {
    /// type is the type of metric source.
    #[serde(default)]
    pub type_: MetricSourceType,
    /// object refers to a metric describing a single kubernetes object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<ObjectMetricSource>,
    /// pods refers to a metric describing each pod in the current scale target.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pods: Option<PodsMetricSource>,
    /// resource refers to a resource metric known to Kubernetes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<ResourceMetricSource>,
    /// containerResource refers to a resource metric describing a single container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container_resource: Option<ContainerResourceMetricSource>,
    /// external refers to a global metric that is not associated with any Kubernetes object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external: Option<ExternalMetricSource>,
}

/// ObjectMetricSource indicates how to scale on a metric describing a kubernetes object.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ObjectMetricSource {
    /// describedObject specifies the descriptions of an object.
    #[serde(default)]
    pub described_object: CrossVersionObjectReference,
    /// target specifies the target value for the given metric.
    #[serde(default)]
    pub target: MetricTarget,
    /// metric identifies the target metric by name and selector.
    #[serde(default)]
    pub metric: MetricIdentifier,
}

/// PodsMetricSource indicates how to scale on a metric describing each pod in the current scale target.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodsMetricSource {
    /// metric identifies the target metric by name and selector.
    #[serde(default)]
    pub metric: MetricIdentifier,
    /// target specifies the target value for the given metric.
    #[serde(default)]
    pub target: MetricTarget,
}

/// ResourceMetricSource indicates how to scale on a resource metric known to Kubernetes.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMetricSource {
    /// name is the name of the resource in question.
    #[serde(default)]
    pub name: crate::core::v1::resource::ResourceName,
    /// target specifies the target value for the given metric.
    #[serde(default)]
    pub target: MetricTarget,
}

/// ContainerResourceMetricSource indicates how to scale on a resource metric describing a single container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContainerResourceMetricSource {
    /// name is the name of the resource in question.
    #[serde(default)]
    pub name: crate::core::v1::resource::ResourceName,
    /// target specifies the target value for the given metric.
    #[serde(default)]
    pub target: MetricTarget,
    /// container is the name of the container in the pods of the scaling target.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub container: String,
}

/// ExternalMetricSource indicates how to scale on a metric not associated with any Kubernetes object.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExternalMetricSource {
    /// metric identifies the target metric by name and selector.
    #[serde(default)]
    pub metric: MetricIdentifier,
    /// target specifies the target value for the given metric.
    #[serde(default)]
    pub target: MetricTarget,
}

/// MetricIdentifier defines the name and optionally selector for a metric.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MetricIdentifier {
    /// name is the name of the given metric.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// selector is the selector for the given metric.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<LabelSelector>,
}

/// MetricTarget defines the target value, average value, or average utilization of a specific metric.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MetricTarget {
    /// type represents whether the metric type is Utilization, Value, or AverageValue.
    #[serde(default)]
    pub type_: MetricTargetType,
    /// value is the target value of the metric.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Quantity>,
    /// averageValue is the target value of the average of the metric across all relevant pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub average_value: Option<Quantity>,
    /// averageUtilization is the target value of the average of the resource metric.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub average_utilization: Option<i32>,
}

/// MetricTargetType specifies the type of metric being targeted.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum MetricTargetType {
    /// UtilizationMetricType declares a MetricTarget is an AverageUtilization value.
    #[serde(rename = "Utilization")]
    #[default]
    Utilization,
    /// ValueMetricType declares a MetricTarget is a raw value.
    #[serde(rename = "Value")]
    Value,
    /// AverageValueMetricType declares a MetricTarget is an AverageValue value.
    #[serde(rename = "AverageValue")]
    AverageValue,
}

pub mod metric_target_type {
    pub const UTILIZATION: &str = "Utilization";
    pub const VALUE: &str = "Value";
    pub const AVERAGE_VALUE: &str = "AverageValue";
}

// ============================================================================
// Status Types
// ============================================================================

/// HorizontalPodAutoscalerStatus describes the current status of a horizontal pod autoscaler.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct HorizontalPodAutoscalerStatus {
    /// observedGeneration is the most recent generation observed by this autoscaler.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
    /// lastScaleTime is the last time the autoscaler scaled the number of pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_scale_time: Option<Timestamp>,
    /// currentReplicas is current number of replicas of pods managed by this autoscaler.
    #[serde(default)]
    pub current_replicas: i32,
    /// desiredReplicas is the desired number of replicas of pods managed by this autoscaler.
    #[serde(default)]
    pub desired_replicas: i32,
    /// currentMetrics is the last read state of the metrics used by this autoscaler.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub current_metrics: Vec<MetricStatus>,
    /// conditions is the set of conditions required for this autoscaler to scale its target.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<HorizontalPodAutoscalerCondition>,
}

/// HorizontalPodAutoscalerConditionType are the valid conditions of a HorizontalPodAutoscaler.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum HorizontalPodAutoscalerConditionType {
    /// ScalingActive indicates that the HPA controller is able to scale if necessary.
    #[serde(rename = "ScalingActive")]
    #[default]
    ScalingActive,
    /// AbleToScale indicates a lack of transient issues which prevent scaling from occurring.
    #[serde(rename = "AbleToScale")]
    AbleToScale,
    /// ScalingLimited indicates that the calculated scale based on metrics would be above or below the range for the HPA.
    #[serde(rename = "ScalingLimited")]
    ScalingLimited,
}

pub mod horizontal_pod_autoscaler_condition_type {
    pub const SCALING_ACTIVE: &str = "ScalingActive";
    pub const ABLE_TO_SCALE: &str = "AbleToScale";
    pub const SCALING_LIMITED: &str = "ScalingLimited";
}

/// HorizontalPodAutoscalerCondition describes the state of a HorizontalPodAutoscaler at a certain point.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct HorizontalPodAutoscalerCondition {
    /// type describes the current condition.
    #[serde(default)]
    pub type_: HorizontalPodAutoscalerConditionType,
    /// status is the status of the condition (True, False, Unknown).
    #[serde(default)]
    pub status: String,
    /// lastTransitionTime is the last time the condition transitioned from one status to another.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<Timestamp>,
    /// reason is the reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// message is a human-readable explanation containing details about the transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// MetricStatus describes the last-read state of a single metric.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MetricStatus {
    /// type is the type of metric source.
    #[serde(default)]
    pub type_: MetricSourceType,
    /// object refers to a metric describing a single kubernetes object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<ObjectMetricStatus>,
    /// pods refers to a metric describing each pod in the current scale target.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pods: Option<PodsMetricStatus>,
    /// resource refers to a resource metric known to Kubernetes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<ResourceMetricStatus>,
    /// containerResource refers to a resource metric describing a single container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container_resource: Option<ContainerResourceMetricStatus>,
    /// external refers to a global metric that is not associated with any Kubernetes object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external: Option<ExternalMetricStatus>,
}

/// ObjectMetricStatus indicates the current value of a metric describing a kubernetes object.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ObjectMetricStatus {
    /// metric identifies the target metric by name and selector.
    #[serde(default)]
    pub metric: MetricIdentifier,
    /// current contains the current value for the given metric.
    #[serde(default)]
    pub current: MetricValueStatus,
    /// describedObject specifies the descriptions of an object.
    #[serde(default)]
    pub described_object: CrossVersionObjectReference,
}

/// PodsMetricStatus indicates the current value of a metric describing each pod.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodsMetricStatus {
    /// metric identifies the target metric by name and selector.
    #[serde(default)]
    pub metric: MetricIdentifier,
    /// current contains the current value for the given metric.
    #[serde(default)]
    pub current: MetricValueStatus,
}

/// ResourceMetricStatus indicates the current value of a resource metric known to Kubernetes.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMetricStatus {
    /// name is the name of the resource in question.
    #[serde(default)]
    pub name: crate::core::v1::resource::ResourceName,
    /// current contains the current value for the given metric.
    #[serde(default)]
    pub current: MetricValueStatus,
}

/// ContainerResourceMetricStatus indicates the current value of a resource metric for a single container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContainerResourceMetricStatus {
    /// name is the name of the resource in question.
    #[serde(default)]
    pub name: crate::core::v1::resource::ResourceName,
    /// current contains the current value for the given metric.
    #[serde(default)]
    pub current: MetricValueStatus,
    /// container is the name of the container in the pods of the scaling target.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub container: String,
}

/// ExternalMetricStatus indicates the current value of a global metric not associated with any Kubernetes object.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExternalMetricStatus {
    /// metric identifies the target metric by name and selector.
    #[serde(default)]
    pub metric: MetricIdentifier,
    /// current contains the current value for the given metric.
    #[serde(default)]
    pub current: MetricValueStatus,
}

/// MetricValueStatus holds the current value for a metric.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MetricValueStatus {
    /// value is the current value of the metric.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Quantity>,
    /// averageValue is the current value of the average of the metric across all relevant pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub average_value: Option<Quantity>,
    /// averageUtilization is the current value of the average of the resource metric.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub average_utilization: Option<i32>,
}

// ============================================================================
// Trait Implementations
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
        "v2beta2"
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
        "v2beta2"
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
        "v2beta2"
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
        "v2beta2"
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
// ApplyDefaults Implementation
// ----------------------------------------------------------------------------

impl ApplyDefault for HorizontalPodAutoscaler {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "autoscaling/v2beta2".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "HorizontalPodAutoscaler".to_string();
        }
        crate::autoscaling::v2beta2::defaults::set_defaults_horizontal_pod_autoscaler(self);
    }
}

impl ApplyDefault for HorizontalPodAutoscalerList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "autoscaling/v2beta2".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "HorizontalPodAutoscalerList".to_string();
        }
    }
}

// ----------------------------------------------------------------------------
// Protobuf Placeholder (using macro)
// ----------------------------------------------------------------------------

impl_unimplemented_prost_message!(HorizontalPodAutoscaler);
impl_unimplemented_prost_message!(HorizontalPodAutoscalerList);

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod trait_tests;
#[cfg(test)]
mod tests {}
