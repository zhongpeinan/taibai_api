//! Validation for Kubernetes Autoscaling API types
//!
//! Ported from k8s.io/kubernetes/pkg/apis/autoscaling/validation/validation.go

use crate::autoscaling::internal;
use crate::common::ObjectMeta;
use crate::common::validation::{
    BadValue, ErrorList, Path, forbidden, invalid, is_dns1123_label, name_is_dns_subdomain,
    required, validate_object_meta, validate_object_meta_update, validate_qualified_name,
};

const MAX_PERIOD_SECONDS: i32 = 1800;
const MAX_STABILIZATION_WINDOW_SECONDS: i32 = 3600;

#[derive(Clone, Debug, Default)]
pub struct CrossVersionObjectReferenceValidationOptions {
    pub allow_empty_api_group: bool,
    pub allow_invalid_api_version: bool,
}

#[derive(Clone, Debug)]
pub struct HorizontalPodAutoscalerSpecValidationOptions {
    pub min_replicas_lower_bound: i32,
    pub scale_target_ref_validation_options: CrossVersionObjectReferenceValidationOptions,
    pub object_metrics_validation_options: CrossVersionObjectReferenceValidationOptions,
}

impl Default for HorizontalPodAutoscalerSpecValidationOptions {
    fn default() -> Self {
        Self {
            min_replicas_lower_bound: 1,
            scale_target_ref_validation_options:
                CrossVersionObjectReferenceValidationOptions::default(),
            object_metrics_validation_options:
                CrossVersionObjectReferenceValidationOptions::default(),
        }
    }
}

// ============================================================================
// Scale Validation
// ============================================================================

/// Validates a Scale.
pub fn validate_scale(scale: &internal::Scale) -> ErrorList {
    let default_meta = ObjectMeta::default();
    let meta = scale.metadata.as_ref().unwrap_or(&default_meta);

    let mut all_errs =
        validate_object_meta(meta, true, name_is_dns_subdomain, &Path::new("metadata"));

    if let Some(ref spec) = scale.spec {
        if let Some(replicas) = spec.replicas {
            if replicas < 0 {
                all_errs.push(invalid(
                    &Path::new("spec").child("replicas"),
                    BadValue::Int(replicas as i64),
                    "must be greater than or equal to 0",
                ));
            }
        }
    }

    all_errs
}

// ============================================================================
// HorizontalPodAutoscaler Validation
// ============================================================================

/// Validates a HorizontalPodAutoscaler.
pub fn validate_horizontal_pod_autoscaler(
    autoscaler: &internal::HorizontalPodAutoscaler,
    opts: &HorizontalPodAutoscalerSpecValidationOptions,
) -> ErrorList {
    let default_meta = ObjectMeta::default();
    let meta = autoscaler.metadata.as_ref().unwrap_or(&default_meta);

    let mut all_errs =
        validate_object_meta(meta, true, name_is_dns_subdomain, &Path::new("metadata"));

    if let Some(ref spec) = autoscaler.spec {
        all_errs.extend(validate_horizontal_pod_autoscaler_spec(
            spec,
            &Path::new("spec"),
            opts,
        ));
    }

    all_errs
}

/// Validates a HorizontalPodAutoscaler update.
pub fn validate_horizontal_pod_autoscaler_update(
    new_autoscaler: &internal::HorizontalPodAutoscaler,
    old_autoscaler: &internal::HorizontalPodAutoscaler,
    opts: &HorizontalPodAutoscalerSpecValidationOptions,
) -> ErrorList {
    let default_meta = ObjectMeta::default();
    let new_meta = new_autoscaler.metadata.as_ref().unwrap_or(&default_meta);
    let old_meta = old_autoscaler.metadata.as_ref().unwrap_or(&default_meta);

    let mut all_errs = validate_object_meta_update(new_meta, old_meta, &Path::new("metadata"));

    if let Some(ref spec) = new_autoscaler.spec {
        all_errs.extend(validate_horizontal_pod_autoscaler_spec(
            spec,
            &Path::new("spec"),
            opts,
        ));
    }

    all_errs
}

/// Validates a HorizontalPodAutoscaler status update.
pub fn validate_horizontal_pod_autoscaler_status_update(
    new_autoscaler: &internal::HorizontalPodAutoscaler,
    old_autoscaler: &internal::HorizontalPodAutoscaler,
) -> ErrorList {
    let default_meta = ObjectMeta::default();
    let new_meta = new_autoscaler.metadata.as_ref().unwrap_or(&default_meta);
    let old_meta = old_autoscaler.metadata.as_ref().unwrap_or(&default_meta);

    let mut all_errs = validate_object_meta_update(new_meta, old_meta, &Path::new("metadata"));

    if let Some(ref status) = new_autoscaler.status {
        if status.current_replicas < 0 {
            all_errs.push(invalid(
                &Path::new("status").child("currentReplicas"),
                BadValue::Int(status.current_replicas as i64),
                "must be greater than or equal to 0",
            ));
        }
        if status.desired_replicas < 0 {
            all_errs.push(invalid(
                &Path::new("status").child("desiredReplicas"),
                BadValue::Int(status.desired_replicas as i64),
                "must be greater than or equal to 0",
            ));
        }
    }

    all_errs
}

fn validate_horizontal_pod_autoscaler_spec(
    autoscaler: &internal::HorizontalPodAutoscalerSpec,
    fld_path: &Path,
    opts: &HorizontalPodAutoscalerSpecValidationOptions,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if let Some(min_replicas) = autoscaler.min_replicas {
        if min_replicas < opts.min_replicas_lower_bound {
            all_errs.push(invalid(
                &fld_path.child("minReplicas"),
                BadValue::Int(min_replicas as i64),
                &format!(
                    "must be greater than or equal to {}",
                    opts.min_replicas_lower_bound
                ),
            ));
        }
    }

    if autoscaler.max_replicas < 1 {
        all_errs.push(invalid(
            &fld_path.child("maxReplicas"),
            BadValue::Int(autoscaler.max_replicas as i64),
            "must be greater than 0",
        ));
    }

    if let Some(min_replicas) = autoscaler.min_replicas {
        if autoscaler.max_replicas < min_replicas {
            all_errs.push(invalid(
                &fld_path.child("maxReplicas"),
                BadValue::Int(autoscaler.max_replicas as i64),
                "must be greater than or equal to `minReplicas`",
            ));
        }
    }

    all_errs.extend(validate_cross_version_object_reference(
        &autoscaler.scale_target_ref,
        &fld_path.child("scaleTargetRef"),
        &opts.scale_target_ref_validation_options,
    ));

    all_errs.extend(validate_metrics(
        &autoscaler.metrics,
        &fld_path.child("metrics"),
        autoscaler.min_replicas,
        &opts.object_metrics_validation_options,
    ));

    all_errs.extend(validate_behavior(
        autoscaler.behavior.as_ref(),
        &fld_path.child("behavior"),
        opts,
    ));

    all_errs
}

fn validate_cross_version_object_reference(
    reference: &internal::CrossVersionObjectReference,
    fld_path: &Path,
    opts: &CrossVersionObjectReferenceValidationOptions,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if reference.kind.is_empty() {
        all_errs.push(required(&fld_path.child("kind"), ""));
    } else {
        all_errs.extend(validate_qualified_name(
            &reference.kind,
            &fld_path.child("kind"),
        ));
    }

    if reference.name.is_empty() {
        all_errs.push(required(&fld_path.child("name"), ""));
    } else {
        all_errs.extend(validate_qualified_name(
            &reference.name,
            &fld_path.child("name"),
        ));
    }

    if !opts.allow_invalid_api_version {
        let api_version = reference.api_version.as_deref().unwrap_or("");
        if api_version.is_empty() {
            all_errs.push(invalid(
                &fld_path.child("apiVersion"),
                BadValue::String(api_version.to_string()),
                "apiVersion must be specified",
            ));
        } else if !opts.allow_empty_api_group {
            let mut parts = api_version.splitn(2, '/');
            let group = parts.next().unwrap_or("");
            let version = parts.next().unwrap_or("");
            if group.is_empty() || version.is_empty() {
                all_errs.push(invalid(
                    &fld_path.child("apiVersion"),
                    BadValue::String(api_version.to_string()),
                    "apiVersion must specify API group",
                ));
            }
        }
    }

    all_errs
}

fn validate_metrics(
    metrics: &[internal::MetricSpec],
    fld_path: &Path,
    min_replicas: Option<i32>,
    opts: &CrossVersionObjectReferenceValidationOptions,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let mut has_object_metrics = false;
    let mut has_external_metrics = false;

    for (idx, metric_spec) in metrics.iter().enumerate() {
        let idx_path = fld_path.index(idx);
        all_errs.extend(validate_metric_spec(metric_spec, &idx_path, opts));

        if metric_spec.type_ == internal::MetricSourceType::Object {
            has_object_metrics = true;
        }
        if metric_spec.type_ == internal::MetricSourceType::External {
            has_external_metrics = true;
        }
    }

    if min_replicas == Some(0) && !has_object_metrics && !has_external_metrics {
        all_errs.push(forbidden(
            fld_path,
            "must specify at least one Object or External metric to support scaling to zero replicas",
        ));
    }

    all_errs
}

fn validate_behavior(
    behavior: Option<&internal::HorizontalPodAutoscalerBehavior>,
    fld_path: &Path,
    opts: &HorizontalPodAutoscalerSpecValidationOptions,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if let Some(behavior) = behavior {
        if let Some(ref rules) = behavior.scale_up {
            all_errs.extend(validate_scaling_rules(
                rules,
                &fld_path.child("scaleUp"),
                opts,
            ));
        }
        if let Some(ref rules) = behavior.scale_down {
            all_errs.extend(validate_scaling_rules(
                rules,
                &fld_path.child("scaleDown"),
                opts,
            ));
        }
    }
    all_errs
}

fn validate_scaling_rules(
    rules: &internal::HPAScalingRules,
    fld_path: &Path,
    _opts: &HorizontalPodAutoscalerSpecValidationOptions,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if let Some(value) = rules.stabilization_window_seconds {
        if value < 0 {
            all_errs.push(invalid(
                &fld_path.child("stabilizationWindowSeconds"),
                BadValue::Int(value as i64),
                "must be greater than or equal to zero",
            ));
        }
        if value > MAX_STABILIZATION_WINDOW_SECONDS {
            all_errs.push(invalid(
                &fld_path.child("stabilizationWindowSeconds"),
                BadValue::Int(value as i64),
                &format!(
                    "must be less than or equal to {}",
                    MAX_STABILIZATION_WINDOW_SECONDS
                ),
            ));
        }
    }

    if rules.policies.is_empty() {
        all_errs.push(required(
            &fld_path.child("policies"),
            "must specify at least one Policy",
        ));
    }

    for (idx, policy) in rules.policies.iter().enumerate() {
        all_errs.extend(validate_scaling_policy(
            policy,
            &fld_path.child("policies").index(idx),
        ));
    }

    if let Some(ref tolerance) = rules.tolerance {
        if let Some(err) = validate_quantity_nonnegative(tolerance, &fld_path.child("tolerance")) {
            all_errs.push(err);
        }
    }

    all_errs
}

fn validate_scaling_policy(policy: &internal::HPAScalingPolicy, fld_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if policy.value <= 0 {
        all_errs.push(invalid(
            &fld_path.child("value"),
            BadValue::Int(policy.value as i64),
            "must be greater than zero",
        ));
    }

    if policy.period_seconds <= 0 {
        all_errs.push(invalid(
            &fld_path.child("periodSeconds"),
            BadValue::Int(policy.period_seconds as i64),
            "must be greater than zero",
        ));
    }

    if policy.period_seconds > MAX_PERIOD_SECONDS {
        all_errs.push(invalid(
            &fld_path.child("periodSeconds"),
            BadValue::Int(policy.period_seconds as i64),
            &format!("must be less than or equal to {}", MAX_PERIOD_SECONDS),
        ));
    }

    all_errs
}

fn validate_metric_spec(
    spec: &internal::MetricSpec,
    fld_path: &Path,
    opts: &CrossVersionObjectReferenceValidationOptions,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    let mut types_present = Vec::new();
    if spec.object.is_some() {
        types_present.push("object");
    }
    if spec.external.is_some() {
        types_present.push("external");
    }
    if spec.pods.is_some() {
        types_present.push("pods");
    }
    if spec.resource.is_some() {
        types_present.push("resource");
    }
    if spec.container_resource.is_some() {
        types_present.push("containerResource");
    }

    let expected_field = match spec.type_ {
        internal::MetricSourceType::Object => "object",
        internal::MetricSourceType::Pods => "pods",
        internal::MetricSourceType::Resource => "resource",
        internal::MetricSourceType::External => "external",
        internal::MetricSourceType::ContainerResource => "containerResource",
    };

    if types_present.len() != 1 {
        for name in types_present.iter() {
            if *name != expected_field {
                all_errs.push(forbidden(
                    &fld_path.child(name),
                    "must populate the given metric source only",
                ));
            }
        }
    }

    match spec.type_ {
        internal::MetricSourceType::Object => {
            if let Some(ref src) = spec.object {
                all_errs.extend(validate_object_source(src, &fld_path.child("object"), opts));
            } else {
                all_errs.push(required(
                    &fld_path.child("object"),
                    "must populate information for the given metric source",
                ));
            }
        }
        internal::MetricSourceType::Pods => {
            if let Some(ref src) = spec.pods {
                all_errs.extend(validate_pods_source(src, &fld_path.child("pods")));
            } else {
                all_errs.push(required(
                    &fld_path.child("pods"),
                    "must populate information for the given metric source",
                ));
            }
        }
        internal::MetricSourceType::Resource => {
            if let Some(ref src) = spec.resource {
                all_errs.extend(validate_resource_source(src, &fld_path.child("resource")));
            } else {
                all_errs.push(required(
                    &fld_path.child("resource"),
                    "must populate information for the given metric source",
                ));
            }
        }
        internal::MetricSourceType::External => {
            if let Some(ref src) = spec.external {
                all_errs.extend(validate_external_source(src, &fld_path.child("external")));
            } else {
                all_errs.push(required(
                    &fld_path.child("external"),
                    "must populate information for the given metric source",
                ));
            }
        }
        internal::MetricSourceType::ContainerResource => {
            if let Some(ref src) = spec.container_resource {
                all_errs.extend(validate_container_resource_source(
                    src,
                    &fld_path.child("containerResource"),
                ));
            } else {
                all_errs.push(required(
                    &fld_path.child("containerResource"),
                    "must populate information for the given metric source",
                ));
            }
        }
    }

    all_errs
}

fn validate_object_source(
    src: &internal::ObjectMetricSource,
    fld_path: &Path,
    opts: &CrossVersionObjectReferenceValidationOptions,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    all_errs.extend(validate_cross_version_object_reference(
        &src.described_object,
        &fld_path.child("describedObject"),
        opts,
    ));
    all_errs.extend(validate_metric_identifier(
        &src.metric,
        &fld_path.child("metric"),
    ));
    all_errs.extend(validate_metric_target(
        &src.target,
        &fld_path.child("target"),
    ));

    if src.target.value.is_none() && src.target.average_value.is_none() {
        all_errs.push(required(
            &fld_path.child("target").child("averageValue"),
            "must set either a target value or averageValue",
        ));
    }

    all_errs
}

fn validate_external_source(src: &internal::ExternalMetricSource, fld_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    all_errs.extend(validate_metric_identifier(
        &src.metric,
        &fld_path.child("metric"),
    ));
    all_errs.extend(validate_metric_target(
        &src.target,
        &fld_path.child("target"),
    ));

    if src.target.value.is_none() && src.target.average_value.is_none() {
        all_errs.push(required(
            &fld_path.child("target").child("averageValue"),
            "must set either a target value for metric or a per-pod target",
        ));
    }

    if src.target.value.is_some() && src.target.average_value.is_some() {
        all_errs.push(forbidden(
            &fld_path.child("target").child("value"),
            "may not set both a target value for metric and a per-pod target",
        ));
    }

    all_errs
}

fn validate_pods_source(src: &internal::PodsMetricSource, fld_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    all_errs.extend(validate_metric_identifier(
        &src.metric,
        &fld_path.child("metric"),
    ));
    all_errs.extend(validate_metric_target(
        &src.target,
        &fld_path.child("target"),
    ));

    if src.target.average_value.is_none() {
        all_errs.push(required(
            &fld_path.child("target").child("averageValue"),
            "must specify a positive target averageValue",
        ));
    }

    all_errs
}

fn validate_container_resource_source(
    src: &internal::ContainerResourceMetricSource,
    fld_path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if src.name.is_empty() {
        all_errs.push(required(
            &fld_path.child("name"),
            "must specify a resource name",
        ));
    }

    if src.container.is_empty() {
        all_errs.push(required(
            &fld_path.child("container"),
            "must specify a container",
        ));
    } else {
        for msg in is_dns1123_label(&src.container) {
            all_errs.push(invalid(
                &fld_path.child("container"),
                BadValue::String(src.container.clone()),
                &msg,
            ));
        }
    }

    all_errs.extend(validate_metric_target(
        &src.target,
        &fld_path.child("target"),
    ));

    if src.target.average_utilization.is_none() && src.target.average_value.is_none() {
        all_errs.push(required(
            &fld_path.child("target").child("averageUtilization"),
            "must set either a target raw value or a target utilization",
        ));
    }

    if src.target.average_utilization.is_some() && src.target.average_value.is_some() {
        all_errs.push(forbidden(
            &fld_path.child("target").child("averageValue"),
            "may not set both a target raw value and a target utilization",
        ));
    }

    all_errs
}

fn validate_resource_source(src: &internal::ResourceMetricSource, fld_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if src.name.is_empty() {
        all_errs.push(required(
            &fld_path.child("name"),
            "must specify a resource name",
        ));
    }

    all_errs.extend(validate_metric_target(
        &src.target,
        &fld_path.child("target"),
    ));

    if src.target.average_utilization.is_none() && src.target.average_value.is_none() {
        all_errs.push(required(
            &fld_path.child("target").child("averageUtilization"),
            "must set either a target raw value or a target utilization",
        ));
    }

    if src.target.average_utilization.is_some() && src.target.average_value.is_some() {
        all_errs.push(forbidden(
            &fld_path.child("target").child("averageValue"),
            "may not set both a target raw value and a target utilization",
        ));
    }

    all_errs
}

fn validate_metric_target(mt: &internal::MetricTarget, fld_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if let Some(ref value) = mt.value {
        if let Some(err) = validate_quantity_positive(value, &fld_path.child("value")) {
            all_errs.push(err);
        }
    }

    if let Some(ref value) = mt.average_value {
        if let Some(err) = validate_quantity_positive(value, &fld_path.child("averageValue")) {
            all_errs.push(err);
        }
    }

    if let Some(value) = mt.average_utilization {
        if value < 1 {
            all_errs.push(invalid(
                &fld_path.child("averageUtilization"),
                BadValue::Int(value as i64),
                "must be greater than 0",
            ));
        }
    }

    all_errs
}

fn validate_metric_identifier(id: &internal::MetricIdentifier, fld_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if id.name.is_empty() {
        all_errs.push(required(
            &fld_path.child("name"),
            "must specify a metric name",
        ));
    } else {
        all_errs.extend(validate_qualified_name(&id.name, &fld_path.child("name")));
    }

    all_errs
}

fn validate_quantity_positive(
    value: &crate::common::Quantity,
    fld_path: &Path,
) -> Option<crate::common::validation::Error> {
    match value.to_f64() {
        Ok(parsed) => {
            if parsed > 0.0 {
                None
            } else {
                Some(invalid(
                    fld_path,
                    BadValue::String(value.as_str().to_string()),
                    "must be positive",
                ))
            }
        }
        Err(_) => Some(invalid(
            fld_path,
            BadValue::String(value.as_str().to_string()),
            "must be a valid quantity",
        )),
    }
}

fn validate_quantity_nonnegative(
    value: &crate::common::Quantity,
    fld_path: &Path,
) -> Option<crate::common::validation::Error> {
    match value.to_f64() {
        Ok(parsed) => {
            if parsed >= 0.0 {
                None
            } else {
                Some(invalid(
                    fld_path,
                    BadValue::String(value.as_str().to_string()),
                    "must be greater than or equal to 0",
                ))
            }
        }
        Err(_) => Some(invalid(
            fld_path,
            BadValue::String(value.as_str().to_string()),
            "must be a valid quantity",
        )),
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn hpa_meta(name: &str) -> ObjectMeta {
        ObjectMeta {
            name: Some(name.to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }
    }

    fn valid_hpa_spec() -> internal::HorizontalPodAutoscalerSpec {
        internal::HorizontalPodAutoscalerSpec {
            scale_target_ref: internal::CrossVersionObjectReference {
                kind: "Deployment".to_string(),
                name: "example".to_string(),
                api_version: Some("apps/v1".to_string()),
            },
            min_replicas: Some(1),
            max_replicas: 3,
            metrics: vec![internal::MetricSpec {
                type_: internal::MetricSourceType::Resource,
                resource: Some(internal::ResourceMetricSource {
                    name: "cpu".to_string(),
                    target: internal::MetricTarget {
                        type_: internal::MetricTargetType::Utilization,
                        average_utilization: Some(80),
                        ..Default::default()
                    },
                }),
                ..Default::default()
            }],
            behavior: None,
        }
    }

    #[test]
    fn test_validate_hpa_valid() {
        let autoscaler = internal::HorizontalPodAutoscaler {
            metadata: Some(hpa_meta("hpa")),
            spec: Some(valid_hpa_spec()),
            status: None,
        };

        let errs = validate_horizontal_pod_autoscaler(
            &autoscaler,
            &HorizontalPodAutoscalerSpecValidationOptions::default(),
        );
        assert!(errs.is_empty(), "expected no errors, got: {errs:?}");
    }

    #[test]
    fn test_validate_hpa_scale_to_zero_requires_object_or_external() {
        let mut spec = valid_hpa_spec();
        spec.min_replicas = Some(0);

        let autoscaler = internal::HorizontalPodAutoscaler {
            metadata: Some(hpa_meta("hpa")),
            spec: Some(spec),
            status: None,
        };

        let errs = validate_horizontal_pod_autoscaler(
            &autoscaler,
            &HorizontalPodAutoscalerSpecValidationOptions::default(),
        );
        assert!(
            errs.errors.iter().any(|err| err.field == "spec.metrics"),
            "expected metrics forbidden error, got: {errs:?}"
        );
    }

    #[test]
    fn test_validate_scale_negative_replicas() {
        let scale = internal::Scale {
            metadata: Some(hpa_meta("scale")),
            spec: Some(internal::ScaleSpec { replicas: Some(-1) }),
            status: None,
        };

        let errs = validate_scale(&scale);
        assert!(
            errs.errors.iter().any(|err| err.field == "spec.replicas"),
            "expected replicas error, got: {errs:?}"
        );
    }
}
