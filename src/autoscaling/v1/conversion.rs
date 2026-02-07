//! Conversions between autoscaling v1 and internal types
//!
//! Based on k8s.io/kubernetes/pkg/apis/autoscaling/v1/conversion.go

use crate::autoscaling::internal;
use crate::autoscaling::{
    BEHAVIOR_SPECS_ANNOTATION, DEFAULT_CPU_UTILIZATION, HPA_CONDITIONS_ANNOTATION,
    METRIC_SPECS_ANNOTATION, METRIC_STATUSES_ANNOTATION,
    drop_round_trip_horizontal_pod_autoscaler_annotations,
};
#[allow(unused_imports)]
use crate::common::{ApplyDefault, FromInternal, ObjectMeta, Quantity, ToInternal, TypeMeta};
use crate::core::v1::resource::resource_name;
use serde_json;
use std::collections::BTreeMap;

use super::*;

// ============================================================================
// Helper Conversions
// ============================================================================

fn ensure_annotations(metadata: &mut Option<ObjectMeta>) -> &mut BTreeMap<String, String> {
    &mut metadata.get_or_insert_with(ObjectMeta::default).annotations
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

fn convert_object_metric_source_to_internal(
    source: ObjectMetricSource,
) -> internal::ObjectMetricSource {
    let metric_type = if source.average_value.is_none() {
        internal::MetricTargetType::Value
    } else {
        internal::MetricTargetType::AverageValue
    };

    internal::ObjectMetricSource {
        described_object: internal::CrossVersionObjectReference {
            kind: source.target.kind,
            name: source.target.name,
            api_version: source.target.api_version,
        },
        target: internal::MetricTarget {
            type_: metric_type,
            value: Some(source.target_value),
            average_value: source.average_value,
            average_utilization: None,
        },
        metric: internal::MetricIdentifier {
            name: source.metric_name,
            selector: source.selector,
        },
    }
}

fn convert_object_metric_source_from_internal(
    source: internal::ObjectMetricSource,
) -> ObjectMetricSource {
    let target_value = source.target.value.unwrap_or_default();

    ObjectMetricSource {
        target: CrossVersionObjectReference {
            kind: source.described_object.kind,
            name: source.described_object.name,
            api_version: source.described_object.api_version,
        },
        metric_name: source.metric.name,
        target_value,
        selector: source.metric.selector,
        average_value: source.target.average_value,
    }
}

fn convert_pods_metric_source_to_internal(source: PodsMetricSource) -> internal::PodsMetricSource {
    internal::PodsMetricSource {
        metric: internal::MetricIdentifier {
            name: source.metric_name,
            selector: source.selector,
        },
        target: internal::MetricTarget {
            type_: internal::MetricTargetType::AverageValue,
            value: None,
            average_value: Some(source.target_average_value),
            average_utilization: None,
        },
    }
}

fn convert_pods_metric_source_from_internal(
    source: internal::PodsMetricSource,
) -> PodsMetricSource {
    let target_average_value = source
        .target
        .average_value
        .unwrap_or_else(Quantity::default);
    PodsMetricSource {
        metric_name: source.metric.name,
        target_average_value,
        selector: source.metric.selector,
    }
}

fn convert_resource_metric_source_to_internal(
    source: ResourceMetricSource,
) -> internal::ResourceMetricSource {
    let metric_type = if source.target_average_utilization.is_none() {
        internal::MetricTargetType::AverageValue
    } else {
        internal::MetricTargetType::Utilization
    };

    internal::ResourceMetricSource {
        name: source.name,
        target: internal::MetricTarget {
            type_: metric_type,
            value: None,
            average_value: source.target_average_value,
            average_utilization: source.target_average_utilization,
        },
    }
}

fn convert_resource_metric_source_from_internal(
    source: internal::ResourceMetricSource,
) -> ResourceMetricSource {
    ResourceMetricSource {
        name: source.name,
        target_average_utilization: source.target.average_utilization,
        target_average_value: source.target.average_value,
    }
}

fn convert_container_resource_metric_source_to_internal(
    source: ContainerResourceMetricSource,
) -> internal::ContainerResourceMetricSource {
    let metric_type = if source.target_average_utilization.is_none() {
        internal::MetricTargetType::AverageValue
    } else {
        internal::MetricTargetType::Utilization
    };

    internal::ContainerResourceMetricSource {
        name: source.name,
        container: source.container,
        target: internal::MetricTarget {
            type_: metric_type,
            value: None,
            average_value: source.target_average_value,
            average_utilization: source.target_average_utilization,
        },
    }
}

fn convert_container_resource_metric_source_from_internal(
    source: internal::ContainerResourceMetricSource,
) -> ContainerResourceMetricSource {
    ContainerResourceMetricSource {
        name: source.name,
        target_average_utilization: source.target.average_utilization,
        target_average_value: source.target.average_value,
        container: source.container,
    }
}

fn convert_external_metric_source_to_internal(
    source: ExternalMetricSource,
) -> internal::ExternalMetricSource {
    let metric_type = if source.target_value.is_none() {
        internal::MetricTargetType::AverageValue
    } else {
        internal::MetricTargetType::Value
    };

    internal::ExternalMetricSource {
        target: internal::MetricTarget {
            type_: metric_type,
            value: source.target_value,
            average_value: source.target_average_value,
            average_utilization: None,
        },
        metric: internal::MetricIdentifier {
            name: source.metric_name,
            selector: source.metric_selector,
        },
    }
}

fn convert_external_metric_source_from_internal(
    source: internal::ExternalMetricSource,
) -> ExternalMetricSource {
    ExternalMetricSource {
        metric_name: source.metric.name,
        metric_selector: source.metric.selector,
        target_value: source.target.value,
        target_average_value: source.target.average_value,
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

fn convert_object_metric_status_to_internal(
    status: ObjectMetricStatus,
) -> internal::ObjectMetricStatus {
    internal::ObjectMetricStatus {
        metric: internal::MetricIdentifier {
            name: status.metric_name,
            selector: status.selector,
        },
        current: internal::MetricValueStatus {
            value: Some(status.current_value),
            average_value: status.average_value,
            average_utilization: None,
        },
        described_object: internal::CrossVersionObjectReference {
            kind: status.target.kind,
            name: status.target.name,
            api_version: status.target.api_version,
        },
    }
}

fn convert_object_metric_status_from_internal(
    status: internal::ObjectMetricStatus,
) -> ObjectMetricStatus {
    let current_value = status.current.value.unwrap_or_default();

    ObjectMetricStatus {
        target: CrossVersionObjectReference {
            kind: status.described_object.kind,
            name: status.described_object.name,
            api_version: status.described_object.api_version,
        },
        metric_name: status.metric.name,
        current_value,
        selector: status.metric.selector,
        average_value: status.current.average_value,
    }
}

fn convert_pods_metric_status_to_internal(status: PodsMetricStatus) -> internal::PodsMetricStatus {
    internal::PodsMetricStatus {
        metric: internal::MetricIdentifier {
            name: status.metric_name,
            selector: status.selector,
        },
        current: internal::MetricValueStatus {
            value: None,
            average_value: Some(status.current_average_value),
            average_utilization: None,
        },
    }
}

fn convert_pods_metric_status_from_internal(
    status: internal::PodsMetricStatus,
) -> PodsMetricStatus {
    let current_average_value = status
        .current
        .average_value
        .unwrap_or_else(Quantity::default);
    PodsMetricStatus {
        metric_name: status.metric.name,
        current_average_value,
        selector: status.metric.selector,
    }
}

fn convert_resource_metric_status_to_internal(
    status: ResourceMetricStatus,
) -> internal::ResourceMetricStatus {
    internal::ResourceMetricStatus {
        name: status.name,
        current: internal::MetricValueStatus {
            value: None,
            average_value: Some(status.current_average_value),
            average_utilization: status.current_average_utilization,
        },
    }
}

fn convert_resource_metric_status_from_internal(
    status: internal::ResourceMetricStatus,
) -> ResourceMetricStatus {
    let current_average_value = status
        .current
        .average_value
        .unwrap_or_else(Quantity::default);
    ResourceMetricStatus {
        name: status.name,
        current_average_utilization: status.current.average_utilization,
        current_average_value,
    }
}

fn convert_container_resource_metric_status_to_internal(
    status: ContainerResourceMetricStatus,
) -> internal::ContainerResourceMetricStatus {
    internal::ContainerResourceMetricStatus {
        name: status.name,
        container: status.container,
        current: internal::MetricValueStatus {
            value: None,
            average_value: Some(status.current_average_value),
            average_utilization: status.current_average_utilization,
        },
    }
}

fn convert_container_resource_metric_status_from_internal(
    status: internal::ContainerResourceMetricStatus,
) -> ContainerResourceMetricStatus {
    let current_average_value = status
        .current
        .average_value
        .unwrap_or_else(Quantity::default);
    ContainerResourceMetricStatus {
        name: status.name,
        current_average_utilization: status.current.average_utilization,
        current_average_value,
        container: status.container,
    }
}

fn convert_external_metric_status_to_internal(
    status: ExternalMetricStatus,
) -> internal::ExternalMetricStatus {
    internal::ExternalMetricStatus {
        metric: internal::MetricIdentifier {
            name: status.metric_name,
            selector: status.metric_selector,
        },
        current: internal::MetricValueStatus {
            value: Some(status.current_value),
            average_value: status.current_average_value,
            average_utilization: None,
        },
    }
}

fn convert_external_metric_status_from_internal(
    status: internal::ExternalMetricStatus,
) -> ExternalMetricStatus {
    let current_value = status.current.value.unwrap_or_default();
    ExternalMetricStatus {
        metric_name: status.metric.name,
        metric_selector: status.metric.selector,
        current_value,
        current_average_value: status.current.average_value,
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

fn convert_scale_spec_to_internal(spec: ScaleSpec) -> internal::ScaleSpec {
    internal::ScaleSpec {
        replicas: spec.replicas,
    }
}

fn convert_scale_spec_from_internal(spec: internal::ScaleSpec) -> ScaleSpec {
    ScaleSpec {
        replicas: spec.replicas,
    }
}

fn convert_scale_status_to_internal(status: ScaleStatus) -> internal::ScaleStatus {
    internal::ScaleStatus {
        replicas: status.replicas,
        selector: status.selector,
    }
}

fn convert_scale_status_from_internal(status: internal::ScaleStatus) -> ScaleStatus {
    ScaleStatus {
        replicas: status.replicas,
        selector: status.selector,
    }
}

fn is_cpu_utilization_metric(metric: &internal::MetricSpec) -> bool {
    let Some(resource) = metric.resource.as_ref() else {
        return false;
    };
    metric.type_ == internal::MetricSourceType::Resource
        && resource.name == resource_name::CPU
        && resource.target.average_utilization.is_some()
}

fn add_behavior_annotation(
    metadata: &mut Option<ObjectMeta>,
    behavior: &internal::HorizontalPodAutoscalerBehavior,
) {
    if let Ok(serialized) = serde_json::to_string(behavior) {
        let annotations = ensure_annotations(metadata);
        annotations.insert(BEHAVIOR_SPECS_ANNOTATION.to_string(), serialized);
    }
}

fn restore_behavior_from_annotations(
    metadata: &Option<ObjectMeta>,
) -> Option<internal::HorizontalPodAutoscalerBehavior> {
    let Some(meta) = metadata.as_ref() else {
        return None;
    };
    let value = meta.annotations.get(BEHAVIOR_SPECS_ANNOTATION)?;
    serde_json::from_str(value).ok()
}

// ============================================================================
// Spec / Status Conversions
// ============================================================================

fn convert_spec_to_internal(
    spec: HorizontalPodAutoscalerSpec,
) -> internal::HorizontalPodAutoscalerSpec {
    let mut metrics = Vec::new();
    if let Some(target) = spec.target_cpu_utilization_percentage {
        metrics.push(internal::MetricSpec {
            type_: internal::MetricSourceType::Resource,
            object: None,
            pods: None,
            resource: Some(internal::ResourceMetricSource {
                name: resource_name::CPU.to_string(),
                target: internal::MetricTarget {
                    type_: internal::MetricTargetType::Utilization,
                    value: None,
                    average_value: None,
                    average_utilization: Some(target),
                },
            }),
            container_resource: None,
            external: None,
        });
    }

    internal::HorizontalPodAutoscalerSpec {
        scale_target_ref: convert_cross_version_object_reference_to_internal(spec.scale_target_ref),
        min_replicas: spec.min_replicas,
        max_replicas: spec.max_replicas,
        metrics,
        behavior: None,
    }
}

fn convert_spec_from_internal(
    spec: internal::HorizontalPodAutoscalerSpec,
) -> HorizontalPodAutoscalerSpec {
    let mut target_cpu_utilization_percentage = None;
    for metric in &spec.metrics {
        if is_cpu_utilization_metric(metric) {
            target_cpu_utilization_percentage = metric
                .resource
                .as_ref()
                .and_then(|resource| resource.target.average_utilization);
            break;
        }
    }

    HorizontalPodAutoscalerSpec {
        scale_target_ref: convert_cross_version_object_reference_from_internal(
            spec.scale_target_ref,
        ),
        min_replicas: spec.min_replicas,
        max_replicas: spec.max_replicas,
        target_cpu_utilization_percentage,
    }
}

fn convert_status_to_internal(
    status: HorizontalPodAutoscalerStatus,
) -> internal::HorizontalPodAutoscalerStatus {
    let mut current_metrics = Vec::new();
    if let Some(cpu) = status.current_cpu_utilization_percentage {
        current_metrics.push(internal::MetricStatus {
            type_: internal::MetricSourceType::Resource,
            object: None,
            pods: None,
            resource: Some(internal::ResourceMetricStatus {
                name: resource_name::CPU.to_string(),
                current: internal::MetricValueStatus {
                    value: None,
                    average_value: None,
                    average_utilization: Some(cpu),
                },
            }),
            container_resource: None,
            external: None,
        });
    }

    internal::HorizontalPodAutoscalerStatus {
        observed_generation: status.observed_generation,
        last_scale_time: status.last_scale_time,
        current_replicas: status.current_replicas,
        desired_replicas: status.desired_replicas,
        current_metrics,
        conditions: Vec::new(),
    }
}

fn convert_status_from_internal(
    status: internal::HorizontalPodAutoscalerStatus,
) -> HorizontalPodAutoscalerStatus {
    let mut current_cpu_utilization_percentage = None;
    for metric in &status.current_metrics {
        let Some(resource) = metric.resource.as_ref() else {
            continue;
        };
        if metric.type_ == internal::MetricSourceType::Resource
            && resource.name == resource_name::CPU
        {
            current_cpu_utilization_percentage = resource.current.average_utilization;
            break;
        }
    }

    HorizontalPodAutoscalerStatus {
        observed_generation: status.observed_generation,
        last_scale_time: status.last_scale_time,
        current_replicas: status.current_replicas,
        desired_replicas: status.desired_replicas,
        current_cpu_utilization_percentage,
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

        if let Some(other_metrics_enc) = result
            .metadata
            .as_ref()
            .and_then(|meta| meta.annotations.get(METRIC_SPECS_ANNOTATION))
        {
            if let Ok(other_metrics) = serde_json::from_str::<Vec<MetricSpec>>(other_metrics_enc) {
                let spec = result
                    .spec
                    .get_or_insert_with(internal::HorizontalPodAutoscalerSpec::default);
                let mut out_metrics = Vec::with_capacity(other_metrics.len() + spec.metrics.len());
                for metric in other_metrics {
                    out_metrics.push(convert_metric_spec_to_internal(metric));
                }
                out_metrics.extend(spec.metrics.drain(..));
                spec.metrics = out_metrics;
            }
        }

        if let Some(behavior) = restore_behavior_from_annotations(&result.metadata) {
            let spec = result
                .spec
                .get_or_insert_with(internal::HorizontalPodAutoscalerSpec::default);
            spec.behavior = Some(behavior);
        }

        if let Some(current_metrics_enc) = result
            .metadata
            .as_ref()
            .and_then(|meta| meta.annotations.get(METRIC_STATUSES_ANNOTATION))
        {
            if let Ok(current_metrics) =
                serde_json::from_str::<Vec<MetricStatus>>(current_metrics_enc)
            {
                let status = result
                    .status
                    .get_or_insert_with(internal::HorizontalPodAutoscalerStatus::default);
                status.current_metrics = current_metrics
                    .into_iter()
                    .map(convert_metric_status_to_internal)
                    .collect();
            }
        }

        if let Some(conditions_enc) = result
            .metadata
            .as_ref()
            .and_then(|meta| meta.annotations.get(HPA_CONDITIONS_ANNOTATION))
        {
            if let Ok(conditions) =
                serde_json::from_str::<Vec<HorizontalPodAutoscalerCondition>>(conditions_enc)
            {
                let status = result
                    .status
                    .get_or_insert_with(internal::HorizontalPodAutoscalerStatus::default);
                status.conditions = conditions
                    .into_iter()
                    .map(|condition| internal::HorizontalPodAutoscalerCondition {
                        type_: convert_condition_type_to_internal(condition.type_),
                        status: condition_status_from_string(&condition.status),
                        last_transition_time: condition.last_transition_time,
                        reason: condition.reason,
                        message: condition.message,
                    })
                    .collect();
            }
        }

        if result
            .spec
            .as_ref()
            .map(|spec| spec.metrics.is_empty())
            .unwrap_or(true)
        {
            let spec = result
                .spec
                .get_or_insert_with(internal::HorizontalPodAutoscalerSpec::default);
            spec.metrics.push(internal::MetricSpec {
                type_: internal::MetricSourceType::Resource,
                object: None,
                pods: None,
                resource: Some(internal::ResourceMetricSource {
                    name: resource_name::CPU.to_string(),
                    target: internal::MetricTarget {
                        type_: internal::MetricTargetType::Utilization,
                        value: None,
                        average_value: None,
                        average_utilization: Some(DEFAULT_CPU_UTILIZATION),
                    },
                }),
                container_resource: None,
                external: None,
            });
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

        let other_metrics = spec.as_ref().map(|spec| {
            spec.metrics
                .iter()
                .filter(|metric| !is_cpu_utilization_metric(metric))
                .cloned()
                .map(convert_metric_spec_from_internal)
                .collect::<Vec<_>>()
        });

        let behavior = spec
            .as_ref()
            .and_then(|spec| spec.behavior.as_ref().cloned());

        let current_metrics = status.as_ref().and_then(|status| {
            if status.current_metrics.is_empty() {
                None
            } else {
                Some(
                    status
                        .current_metrics
                        .iter()
                        .cloned()
                        .map(convert_metric_status_from_internal)
                        .collect::<Vec<_>>(),
                )
            }
        });

        let current_conditions = status.as_ref().and_then(|status| {
            if status.conditions.is_empty() {
                None
            } else {
                Some(
                    status
                        .conditions
                        .iter()
                        .cloned()
                        .map(|condition| HorizontalPodAutoscalerCondition {
                            type_: convert_condition_type_from_internal(condition.type_),
                            status: condition_status_to_string(condition.status),
                            last_transition_time: condition.last_transition_time,
                            reason: condition.reason,
                            message: condition.message,
                        })
                        .collect::<Vec<_>>(),
                )
            }
        });

        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata,
            spec: spec.map(convert_spec_from_internal),
            status: status.map(convert_status_from_internal),
        };

        if let Some(ref mut meta) = result.metadata {
            drop_round_trip_horizontal_pod_autoscaler_annotations(&mut meta.annotations);
        }

        if let Some(other_metrics) = other_metrics {
            if !other_metrics.is_empty() {
                if let Ok(serialized) = serde_json::to_string(&other_metrics) {
                    let annotations = ensure_annotations(&mut result.metadata);
                    annotations.insert(METRIC_SPECS_ANNOTATION.to_string(), serialized);
                }
            }
        }

        if let Some(behavior) = behavior {
            add_behavior_annotation(&mut result.metadata, &behavior);
        }

        if let Some(current_metrics) = current_metrics {
            if let Ok(serialized) = serde_json::to_string(&current_metrics) {
                let annotations = ensure_annotations(&mut result.metadata);
                annotations.insert(METRIC_STATUSES_ANNOTATION.to_string(), serialized);
            }
        }

        if let Some(current_conditions) = current_conditions {
            if let Ok(serialized) = serde_json::to_string(&current_conditions) {
                let annotations = ensure_annotations(&mut result.metadata);
                annotations.insert(HPA_CONDITIONS_ANNOTATION.to_string(), serialized);
            }
        }

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
        let result = Self {
            type_meta: TypeMeta::default(),
            metadata: value.metadata,
            items: value
                .items
                .into_iter()
                .map(HorizontalPodAutoscaler::from_internal)
                .collect(),
        };

        result
    }
}

// ============================================================================
// Scale Conversions
// ============================================================================

impl ToInternal<internal::Scale> for Scale {
    fn to_internal(self) -> internal::Scale {
        internal::Scale {
            metadata: self.metadata,
            spec: self.spec.map(convert_scale_spec_to_internal),
            status: self.status.map(convert_scale_status_to_internal),
        }
    }
}

impl FromInternal<internal::Scale> for Scale {
    fn from_internal(value: internal::Scale) -> Self {
        Self {
            type_meta: Default::default(),
            metadata: value.metadata,
            spec: value.spec.map(convert_scale_spec_from_internal),
            status: value.status.map(convert_scale_status_from_internal),
        }
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
                api_version: "autoscaling/v1".to_string(),
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
        let mut round_trip = HorizontalPodAutoscaler::from_internal(internal);
        round_trip.apply_default();

        assert_eq!(round_trip.metadata, original.metadata);
        assert_eq!(round_trip.type_meta.api_version, "autoscaling/v1");
        assert_eq!(round_trip.type_meta.kind, "HorizontalPodAutoscaler");
    }
}
