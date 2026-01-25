//! Conversions between autoscaling v2beta2 and internal types
//!
//! Based on k8s.io/kubernetes/pkg/apis/autoscaling/v2beta2/conversion.go

use crate::autoscaling::internal;
use crate::autoscaling::{
    TOLERANCE_SCALE_DOWN_ANNOTATION, TOLERANCE_SCALE_UP_ANNOTATION,
    drop_round_trip_horizontal_pod_autoscaler_annotations,
};
use crate::common::{ApplyDefault, FromInternal, ObjectMeta, Quantity, ToInternal, TypeMeta};
use std::collections::BTreeMap;

use super::*;

// ============================================================================
// Helper Conversions
// ============================================================================

fn ensure_annotations(metadata: &mut Option<ObjectMeta>) -> &mut BTreeMap<String, String> {
    &mut metadata.get_or_insert_with(ObjectMeta::default).annotations
}

fn convert_scaling_policy_select_to_internal(
    select: ScalingPolicySelect,
) -> internal::ScalingPolicySelect {
    match select {
        ScalingPolicySelect::Max => internal::ScalingPolicySelect::Max,
        ScalingPolicySelect::Min => internal::ScalingPolicySelect::Min,
        ScalingPolicySelect::Disabled => internal::ScalingPolicySelect::Disabled,
    }
}

fn convert_scaling_policy_select_from_internal(
    select: internal::ScalingPolicySelect,
) -> ScalingPolicySelect {
    match select {
        internal::ScalingPolicySelect::Max => ScalingPolicySelect::Max,
        internal::ScalingPolicySelect::Min => ScalingPolicySelect::Min,
        internal::ScalingPolicySelect::Disabled => ScalingPolicySelect::Disabled,
    }
}

fn convert_scaling_policy_type_to_internal(
    policy: HPAScalingPolicyType,
) -> internal::HPAScalingPolicyType {
    match policy {
        HPAScalingPolicyType::Pods => internal::HPAScalingPolicyType::Pods,
        HPAScalingPolicyType::Percent => internal::HPAScalingPolicyType::Percent,
    }
}

fn convert_scaling_policy_type_from_internal(
    policy: internal::HPAScalingPolicyType,
) -> HPAScalingPolicyType {
    match policy {
        internal::HPAScalingPolicyType::Pods => HPAScalingPolicyType::Pods,
        internal::HPAScalingPolicyType::Percent => HPAScalingPolicyType::Percent,
    }
}

fn convert_metric_source_type_to_internal(
    metric_type: MetricSourceType,
) -> internal::MetricSourceType {
    match metric_type {
        MetricSourceType::Object => internal::MetricSourceType::Object,
        MetricSourceType::Pods => internal::MetricSourceType::Pods,
        MetricSourceType::Resource => internal::MetricSourceType::Resource,
        MetricSourceType::ContainerResource => internal::MetricSourceType::ContainerResource,
        MetricSourceType::External => internal::MetricSourceType::External,
    }
}

fn convert_metric_source_type_from_internal(
    metric_type: internal::MetricSourceType,
) -> MetricSourceType {
    match metric_type {
        internal::MetricSourceType::Object => MetricSourceType::Object,
        internal::MetricSourceType::Pods => MetricSourceType::Pods,
        internal::MetricSourceType::Resource => MetricSourceType::Resource,
        internal::MetricSourceType::ContainerResource => MetricSourceType::ContainerResource,
        internal::MetricSourceType::External => MetricSourceType::External,
    }
}

fn convert_metric_target_type_to_internal(
    target_type: MetricTargetType,
) -> internal::MetricTargetType {
    match target_type {
        MetricTargetType::Utilization => internal::MetricTargetType::Utilization,
        MetricTargetType::Value => internal::MetricTargetType::Value,
        MetricTargetType::AverageValue => internal::MetricTargetType::AverageValue,
    }
}

fn convert_metric_target_type_from_internal(
    target_type: internal::MetricTargetType,
) -> MetricTargetType {
    match target_type {
        internal::MetricTargetType::Utilization => MetricTargetType::Utilization,
        internal::MetricTargetType::Value => MetricTargetType::Value,
        internal::MetricTargetType::AverageValue => MetricTargetType::AverageValue,
    }
}

fn convert_condition_type_to_internal(
    condition_type: HorizontalPodAutoscalerConditionType,
) -> internal::HorizontalPodAutoscalerConditionType {
    match condition_type {
        HorizontalPodAutoscalerConditionType::ScalingActive => {
            internal::HorizontalPodAutoscalerConditionType::ScalingActive
        }
        HorizontalPodAutoscalerConditionType::AbleToScale => {
            internal::HorizontalPodAutoscalerConditionType::AbleToScale
        }
        HorizontalPodAutoscalerConditionType::ScalingLimited => {
            internal::HorizontalPodAutoscalerConditionType::ScalingLimited
        }
    }
}

fn convert_condition_type_from_internal(
    condition_type: internal::HorizontalPodAutoscalerConditionType,
) -> HorizontalPodAutoscalerConditionType {
    match condition_type {
        internal::HorizontalPodAutoscalerConditionType::ScalingActive => {
            HorizontalPodAutoscalerConditionType::ScalingActive
        }
        internal::HorizontalPodAutoscalerConditionType::AbleToScale => {
            HorizontalPodAutoscalerConditionType::AbleToScale
        }
        internal::HorizontalPodAutoscalerConditionType::ScalingLimited => {
            HorizontalPodAutoscalerConditionType::ScalingLimited
        }
    }
}

fn condition_status_from_string(value: &str) -> internal::ConditionStatus {
    match value {
        "True" => internal::ConditionStatus::True,
        "False" => internal::ConditionStatus::False,
        _ => internal::ConditionStatus::Unknown,
    }
}

fn condition_status_to_string(value: internal::ConditionStatus) -> String {
    match value {
        internal::ConditionStatus::True => "True".to_string(),
        internal::ConditionStatus::False => "False".to_string(),
        internal::ConditionStatus::Unknown => "Unknown".to_string(),
    }
}

fn convert_cross_version_object_reference_to_internal(
    reference: CrossVersionObjectReference,
) -> internal::CrossVersionObjectReference {
    internal::CrossVersionObjectReference {
        kind: reference.kind,
        name: reference.name,
        api_version: reference.api_version,
    }
}

fn convert_cross_version_object_reference_from_internal(
    reference: internal::CrossVersionObjectReference,
) -> CrossVersionObjectReference {
    CrossVersionObjectReference {
        kind: reference.kind,
        name: reference.name,
        api_version: reference.api_version,
    }
}

fn convert_metric_identifier_to_internal(
    identifier: MetricIdentifier,
) -> internal::MetricIdentifier {
    internal::MetricIdentifier {
        name: identifier.name,
        selector: identifier.selector,
    }
}

fn convert_metric_identifier_from_internal(
    identifier: internal::MetricIdentifier,
) -> MetricIdentifier {
    MetricIdentifier {
        name: identifier.name,
        selector: identifier.selector,
    }
}

fn convert_metric_target_to_internal(target: MetricTarget) -> internal::MetricTarget {
    internal::MetricTarget {
        type_: convert_metric_target_type_to_internal(target.type_),
        value: target.value,
        average_value: target.average_value,
        average_utilization: target.average_utilization,
    }
}

fn convert_metric_target_from_internal(target: internal::MetricTarget) -> MetricTarget {
    MetricTarget {
        type_: convert_metric_target_type_from_internal(target.type_),
        value: target.value,
        average_value: target.average_value,
        average_utilization: target.average_utilization,
    }
}

fn convert_object_metric_source_to_internal(
    source: ObjectMetricSource,
) -> internal::ObjectMetricSource {
    internal::ObjectMetricSource {
        described_object: convert_cross_version_object_reference_to_internal(
            source.described_object,
        ),
        target: convert_metric_target_to_internal(source.target),
        metric: convert_metric_identifier_to_internal(source.metric),
    }
}

fn convert_object_metric_source_from_internal(
    source: internal::ObjectMetricSource,
) -> ObjectMetricSource {
    ObjectMetricSource {
        described_object: convert_cross_version_object_reference_from_internal(
            source.described_object,
        ),
        target: convert_metric_target_from_internal(source.target),
        metric: convert_metric_identifier_from_internal(source.metric),
    }
}

fn convert_pods_metric_source_to_internal(source: PodsMetricSource) -> internal::PodsMetricSource {
    internal::PodsMetricSource {
        metric: convert_metric_identifier_to_internal(source.metric),
        target: convert_metric_target_to_internal(source.target),
    }
}

fn convert_pods_metric_source_from_internal(
    source: internal::PodsMetricSource,
) -> PodsMetricSource {
    PodsMetricSource {
        metric: convert_metric_identifier_from_internal(source.metric),
        target: convert_metric_target_from_internal(source.target),
    }
}

fn convert_resource_metric_source_to_internal(
    source: ResourceMetricSource,
) -> internal::ResourceMetricSource {
    internal::ResourceMetricSource {
        name: source.name,
        target: convert_metric_target_to_internal(source.target),
    }
}

fn convert_resource_metric_source_from_internal(
    source: internal::ResourceMetricSource,
) -> ResourceMetricSource {
    ResourceMetricSource {
        name: source.name,
        target: convert_metric_target_from_internal(source.target),
    }
}

fn convert_container_resource_metric_source_to_internal(
    source: ContainerResourceMetricSource,
) -> internal::ContainerResourceMetricSource {
    internal::ContainerResourceMetricSource {
        name: source.name,
        container: source.container,
        target: convert_metric_target_to_internal(source.target),
    }
}

fn convert_container_resource_metric_source_from_internal(
    source: internal::ContainerResourceMetricSource,
) -> ContainerResourceMetricSource {
    ContainerResourceMetricSource {
        name: source.name,
        container: source.container,
        target: convert_metric_target_from_internal(source.target),
    }
}

fn convert_external_metric_source_to_internal(
    source: ExternalMetricSource,
) -> internal::ExternalMetricSource {
    internal::ExternalMetricSource {
        metric: convert_metric_identifier_to_internal(source.metric),
        target: convert_metric_target_to_internal(source.target),
    }
}

fn convert_external_metric_source_from_internal(
    source: internal::ExternalMetricSource,
) -> ExternalMetricSource {
    ExternalMetricSource {
        metric: convert_metric_identifier_from_internal(source.metric),
        target: convert_metric_target_from_internal(source.target),
    }
}

fn convert_metric_spec_to_internal(spec: MetricSpec) -> internal::MetricSpec {
    internal::MetricSpec {
        type_: convert_metric_source_type_to_internal(spec.type_),
        object: spec.object.map(convert_object_metric_source_to_internal),
        pods: spec.pods.map(convert_pods_metric_source_to_internal),
        resource: spec
            .resource
            .map(convert_resource_metric_source_to_internal),
        container_resource: spec
            .container_resource
            .map(convert_container_resource_metric_source_to_internal),
        external: spec
            .external
            .map(convert_external_metric_source_to_internal),
    }
}

fn convert_metric_spec_from_internal(spec: internal::MetricSpec) -> MetricSpec {
    MetricSpec {
        type_: convert_metric_source_type_from_internal(spec.type_),
        object: spec.object.map(convert_object_metric_source_from_internal),
        pods: spec.pods.map(convert_pods_metric_source_from_internal),
        resource: spec
            .resource
            .map(convert_resource_metric_source_from_internal),
        container_resource: spec
            .container_resource
            .map(convert_container_resource_metric_source_from_internal),
        external: spec
            .external
            .map(convert_external_metric_source_from_internal),
    }
}

fn convert_metric_value_status_to_internal(
    status: MetricValueStatus,
) -> internal::MetricValueStatus {
    internal::MetricValueStatus {
        value: status.value,
        average_value: status.average_value,
        average_utilization: status.average_utilization,
    }
}

fn convert_metric_value_status_from_internal(
    status: internal::MetricValueStatus,
) -> MetricValueStatus {
    MetricValueStatus {
        value: status.value,
        average_value: status.average_value,
        average_utilization: status.average_utilization,
    }
}

fn convert_object_metric_status_to_internal(
    status: ObjectMetricStatus,
) -> internal::ObjectMetricStatus {
    internal::ObjectMetricStatus {
        metric: convert_metric_identifier_to_internal(status.metric),
        current: convert_metric_value_status_to_internal(status.current),
        described_object: convert_cross_version_object_reference_to_internal(
            status.described_object,
        ),
    }
}

fn convert_object_metric_status_from_internal(
    status: internal::ObjectMetricStatus,
) -> ObjectMetricStatus {
    ObjectMetricStatus {
        metric: convert_metric_identifier_from_internal(status.metric),
        current: convert_metric_value_status_from_internal(status.current),
        described_object: convert_cross_version_object_reference_from_internal(
            status.described_object,
        ),
    }
}

fn convert_pods_metric_status_to_internal(status: PodsMetricStatus) -> internal::PodsMetricStatus {
    internal::PodsMetricStatus {
        metric: convert_metric_identifier_to_internal(status.metric),
        current: convert_metric_value_status_to_internal(status.current),
    }
}

fn convert_pods_metric_status_from_internal(
    status: internal::PodsMetricStatus,
) -> PodsMetricStatus {
    PodsMetricStatus {
        metric: convert_metric_identifier_from_internal(status.metric),
        current: convert_metric_value_status_from_internal(status.current),
    }
}

fn convert_resource_metric_status_to_internal(
    status: ResourceMetricStatus,
) -> internal::ResourceMetricStatus {
    internal::ResourceMetricStatus {
        name: status.name,
        current: convert_metric_value_status_to_internal(status.current),
    }
}

fn convert_resource_metric_status_from_internal(
    status: internal::ResourceMetricStatus,
) -> ResourceMetricStatus {
    ResourceMetricStatus {
        name: status.name,
        current: convert_metric_value_status_from_internal(status.current),
    }
}

fn convert_container_resource_metric_status_to_internal(
    status: ContainerResourceMetricStatus,
) -> internal::ContainerResourceMetricStatus {
    internal::ContainerResourceMetricStatus {
        name: status.name,
        container: status.container,
        current: convert_metric_value_status_to_internal(status.current),
    }
}

fn convert_container_resource_metric_status_from_internal(
    status: internal::ContainerResourceMetricStatus,
) -> ContainerResourceMetricStatus {
    ContainerResourceMetricStatus {
        name: status.name,
        container: status.container,
        current: convert_metric_value_status_from_internal(status.current),
    }
}

fn convert_external_metric_status_to_internal(
    status: ExternalMetricStatus,
) -> internal::ExternalMetricStatus {
    internal::ExternalMetricStatus {
        metric: convert_metric_identifier_to_internal(status.metric),
        current: convert_metric_value_status_to_internal(status.current),
    }
}

fn convert_external_metric_status_from_internal(
    status: internal::ExternalMetricStatus,
) -> ExternalMetricStatus {
    ExternalMetricStatus {
        metric: convert_metric_identifier_from_internal(status.metric),
        current: convert_metric_value_status_from_internal(status.current),
    }
}

fn convert_metric_status_to_internal(status: MetricStatus) -> internal::MetricStatus {
    internal::MetricStatus {
        type_: convert_metric_source_type_to_internal(status.type_),
        object: status.object.map(convert_object_metric_status_to_internal),
        pods: status.pods.map(convert_pods_metric_status_to_internal),
        resource: status
            .resource
            .map(convert_resource_metric_status_to_internal),
        container_resource: status
            .container_resource
            .map(convert_container_resource_metric_status_to_internal),
        external: status
            .external
            .map(convert_external_metric_status_to_internal),
    }
}

fn convert_metric_status_from_internal(status: internal::MetricStatus) -> MetricStatus {
    MetricStatus {
        type_: convert_metric_source_type_from_internal(status.type_),
        object: status
            .object
            .map(convert_object_metric_status_from_internal),
        pods: status.pods.map(convert_pods_metric_status_from_internal),
        resource: status
            .resource
            .map(convert_resource_metric_status_from_internal),
        container_resource: status
            .container_resource
            .map(convert_container_resource_metric_status_from_internal),
        external: status
            .external
            .map(convert_external_metric_status_from_internal),
    }
}

fn convert_scaling_policy_to_internal(policy: HPAScalingPolicy) -> internal::HPAScalingPolicy {
    internal::HPAScalingPolicy {
        type_: convert_scaling_policy_type_to_internal(policy.type_),
        value: policy.value,
        period_seconds: policy.period_seconds,
    }
}

fn convert_scaling_policy_from_internal(policy: internal::HPAScalingPolicy) -> HPAScalingPolicy {
    HPAScalingPolicy {
        type_: convert_scaling_policy_type_from_internal(policy.type_),
        value: policy.value,
        period_seconds: policy.period_seconds,
    }
}

fn convert_scaling_rules_to_internal(rules: HPAScalingRules) -> internal::HPAScalingRules {
    internal::HPAScalingRules {
        stabilization_window_seconds: rules.stabilization_window_seconds,
        select_policy: rules
            .select_policy
            .map(convert_scaling_policy_select_to_internal),
        policies: rules
            .policies
            .into_iter()
            .map(convert_scaling_policy_to_internal)
            .collect(),
        tolerance: None,
    }
}

fn convert_scaling_rules_from_internal(rules: internal::HPAScalingRules) -> HPAScalingRules {
    HPAScalingRules {
        stabilization_window_seconds: rules.stabilization_window_seconds,
        select_policy: rules
            .select_policy
            .map(convert_scaling_policy_select_from_internal),
        policies: rules
            .policies
            .into_iter()
            .map(convert_scaling_policy_from_internal)
            .collect(),
    }
}

fn convert_behavior_to_internal(
    behavior: HorizontalPodAutoscalerBehavior,
) -> internal::HorizontalPodAutoscalerBehavior {
    internal::HorizontalPodAutoscalerBehavior {
        scale_up: behavior.scale_up.map(convert_scaling_rules_to_internal),
        scale_down: behavior.scale_down.map(convert_scaling_rules_to_internal),
    }
}

fn convert_behavior_from_internal(
    behavior: internal::HorizontalPodAutoscalerBehavior,
) -> HorizontalPodAutoscalerBehavior {
    HorizontalPodAutoscalerBehavior {
        scale_up: behavior.scale_up.map(convert_scaling_rules_from_internal),
        scale_down: behavior.scale_down.map(convert_scaling_rules_from_internal),
    }
}

fn convert_spec_to_internal(
    spec: HorizontalPodAutoscalerSpec,
) -> internal::HorizontalPodAutoscalerSpec {
    internal::HorizontalPodAutoscalerSpec {
        scale_target_ref: convert_cross_version_object_reference_to_internal(spec.scale_target_ref),
        min_replicas: spec.min_replicas,
        max_replicas: spec.max_replicas,
        metrics: spec
            .metrics
            .into_iter()
            .map(convert_metric_spec_to_internal)
            .collect(),
        behavior: spec.behavior.map(convert_behavior_to_internal),
    }
}

fn convert_spec_from_internal(
    spec: internal::HorizontalPodAutoscalerSpec,
) -> HorizontalPodAutoscalerSpec {
    HorizontalPodAutoscalerSpec {
        scale_target_ref: convert_cross_version_object_reference_from_internal(
            spec.scale_target_ref,
        ),
        min_replicas: spec.min_replicas,
        max_replicas: spec.max_replicas,
        metrics: spec
            .metrics
            .into_iter()
            .map(convert_metric_spec_from_internal)
            .collect(),
        behavior: spec.behavior.map(convert_behavior_from_internal),
    }
}

fn convert_status_to_internal(
    status: HorizontalPodAutoscalerStatus,
) -> internal::HorizontalPodAutoscalerStatus {
    internal::HorizontalPodAutoscalerStatus {
        observed_generation: status.observed_generation,
        last_scale_time: status.last_scale_time,
        current_replicas: status.current_replicas,
        desired_replicas: status.desired_replicas,
        current_metrics: status
            .current_metrics
            .into_iter()
            .map(convert_metric_status_to_internal)
            .collect(),
        conditions: status
            .conditions
            .into_iter()
            .map(|condition| internal::HorizontalPodAutoscalerCondition {
                type_: convert_condition_type_to_internal(condition.type_),
                status: condition_status_from_string(&condition.status),
                last_transition_time: condition.last_transition_time,
                reason: condition.reason,
                message: condition.message,
            })
            .collect(),
    }
}

fn convert_status_from_internal(
    status: internal::HorizontalPodAutoscalerStatus,
) -> HorizontalPodAutoscalerStatus {
    HorizontalPodAutoscalerStatus {
        observed_generation: status.observed_generation,
        last_scale_time: status.last_scale_time,
        current_replicas: status.current_replicas,
        desired_replicas: status.desired_replicas,
        current_metrics: status
            .current_metrics
            .into_iter()
            .map(convert_metric_status_from_internal)
            .collect(),
        conditions: status
            .conditions
            .into_iter()
            .map(|condition| HorizontalPodAutoscalerCondition {
                type_: convert_condition_type_from_internal(condition.type_),
                status: condition_status_to_string(condition.status),
                last_transition_time: condition.last_transition_time,
                reason: condition.reason,
                message: condition.message,
            })
            .collect(),
    }
}

fn apply_tolerance_annotations(
    metadata: &mut Option<ObjectMeta>,
    behavior: &internal::HorizontalPodAutoscalerBehavior,
) {
    if let Some(scale_down) = behavior.scale_down.as_ref() {
        if let Some(tolerance) = scale_down.tolerance.as_ref() {
            let annotations = ensure_annotations(metadata);
            annotations.insert(
                TOLERANCE_SCALE_DOWN_ANNOTATION.to_string(),
                tolerance.as_str().to_string(),
            );
        }
    }
    if let Some(scale_up) = behavior.scale_up.as_ref() {
        if let Some(tolerance) = scale_up.tolerance.as_ref() {
            let annotations = ensure_annotations(metadata);
            annotations.insert(
                TOLERANCE_SCALE_UP_ANNOTATION.to_string(),
                tolerance.as_str().to_string(),
            );
        }
    }
}

fn restore_tolerance_annotations(
    metadata: &Option<ObjectMeta>,
    behavior: &mut internal::HorizontalPodAutoscalerBehavior,
) {
    let Some(meta) = metadata.as_ref() else {
        return;
    };
    let annotations = &meta.annotations;

    if let Some(value) = annotations.get(TOLERANCE_SCALE_DOWN_ANNOTATION) {
        if let Ok(tolerance) = Quantity::from_str_validated(value) {
            behavior
                .scale_down
                .get_or_insert_with(internal::HPAScalingRules::default)
                .tolerance = Some(tolerance);
        }
    }
    if let Some(value) = annotations.get(TOLERANCE_SCALE_UP_ANNOTATION) {
        if let Ok(tolerance) = Quantity::from_str_validated(value) {
            behavior
                .scale_up
                .get_or_insert_with(internal::HPAScalingRules::default)
                .tolerance = Some(tolerance);
        }
    }
}

// ============================================================================
// HorizontalPodAutoscaler Conversions
// ============================================================================

impl ToInternal<internal::HorizontalPodAutoscaler> for HorizontalPodAutoscaler {
    fn to_internal(self) -> internal::HorizontalPodAutoscaler {
        let mut metadata = self.metadata;
        if let Some(ref mut meta) = metadata {
            drop_round_trip_horizontal_pod_autoscaler_annotations(&mut meta.annotations);
        }
        let mut result = internal::HorizontalPodAutoscaler {
            metadata,
            spec: self.spec.map(convert_spec_to_internal),
            status: self.status.map(convert_status_to_internal),
        };

        if let Some(ref mut spec) = result.spec {
            if spec.behavior.is_none() {
                spec.behavior = Some(internal::HorizontalPodAutoscalerBehavior::default());
            }
            if let Some(ref mut behavior) = spec.behavior {
                restore_tolerance_annotations(&result.metadata, behavior);
            }
        }

        if let Some(ref mut meta) = result.metadata {
            drop_round_trip_horizontal_pod_autoscaler_annotations(&mut meta.annotations);
        }

        result
    }
}

impl FromInternal<internal::HorizontalPodAutoscaler> for HorizontalPodAutoscaler {
    fn from_internal(value: internal::HorizontalPodAutoscaler) -> Self {
        let internal::HorizontalPodAutoscaler {
            metadata,
            spec,
            status,
        } = value;

        let behavior = spec
            .as_ref()
            .and_then(|spec| spec.behavior.as_ref().cloned());

        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata,
            spec: spec.map(convert_spec_from_internal),
            status: status.map(convert_status_from_internal),
        };

        if let Some(ref mut meta) = result.metadata {
            drop_round_trip_horizontal_pod_autoscaler_annotations(&mut meta.annotations);
        }

        if let Some(behavior) = behavior {
            apply_tolerance_annotations(&mut result.metadata, &behavior);
        }

        result.apply_default();
        result
    }
}

// ============================================================================
// List Conversions
// ============================================================================

impl ToInternal<internal::HorizontalPodAutoscalerList> for HorizontalPodAutoscalerList {
    fn to_internal(mut self) -> internal::HorizontalPodAutoscalerList {
        self.type_meta = TypeMeta::default();
        internal::HorizontalPodAutoscalerList {
            metadata: self.metadata,
            items: self
                .items
                .into_iter()
                .map(HorizontalPodAutoscaler::to_internal)
                .collect(),
        }
    }
}

impl FromInternal<internal::HorizontalPodAutoscalerList> for HorizontalPodAutoscalerList {
    fn from_internal(value: internal::HorizontalPodAutoscalerList) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: value.metadata,
            items: value
                .items
                .into_iter()
                .map(HorizontalPodAutoscaler::from_internal)
                .collect(),
        };
        result.apply_default();
        result
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
    fn test_hpa_round_trip() {
        let original = HorizontalPodAutoscaler {
            type_meta: TypeMeta {
                api_version: "autoscaling/v2beta2".to_string(),
                kind: "HorizontalPodAutoscaler".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("hpa".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: None,
            status: None,
        };

        let internal = original.clone().to_internal();
        let round_trip = HorizontalPodAutoscaler::from_internal(internal);

        assert_eq!(round_trip.metadata, original.metadata);
        assert_eq!(round_trip.type_meta.api_version, "autoscaling/v2beta2");
        assert_eq!(round_trip.type_meta.kind, "HorizontalPodAutoscaler");
    }
}
