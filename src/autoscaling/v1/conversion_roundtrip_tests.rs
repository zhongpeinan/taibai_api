use super::{
    CrossVersionObjectReference, HorizontalPodAutoscaler, HorizontalPodAutoscalerList,
    HorizontalPodAutoscalerSpec, HorizontalPodAutoscalerStatus, Scale, ScaleSpec, ScaleStatus,
};
use crate::autoscaling::internal;
use crate::common::test_utils::assert_conversion_roundtrip;
use crate::common::{
    ApplyDefault, FromInternal, ListMeta, ObjectMeta, Timestamp, ToInternal, TypeMeta,
};

fn hpa_basic() -> HorizontalPodAutoscaler {
    HorizontalPodAutoscaler {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("hpa-b".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        spec: Some(HorizontalPodAutoscalerSpec {
            scale_target_ref: CrossVersionObjectReference {
                kind: "Deployment".to_string(),
                name: "api".to_string(),
                api_version: Some("apps/v1".to_string()),
            },
            min_replicas: Some(1),
            max_replicas: 5,
            target_cpu_utilization_percentage: Some(80),
        }),
        status: Some(HorizontalPodAutoscalerStatus {
            observed_generation: Some(2),
            last_scale_time: Some(Timestamp::from_str("2024-01-02T00:00:00Z").unwrap()),
            current_replicas: 2,
            desired_replicas: 4,
            current_cpu_utilization_percentage: None,
        }),
    }
}

fn hpa_list_basic() -> HorizontalPodAutoscalerList {
    let mut item = hpa_basic();
    item.apply_default();

    HorizontalPodAutoscalerList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("2".to_string()),
            ..Default::default()
        }),
        items: vec![item],
    }
}

fn scale_basic() -> Scale {
    Scale {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("scale-b".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        spec: Some(ScaleSpec { replicas: Some(2) }),
        status: Some(ScaleStatus {
            replicas: 2,
            selector: Some("app=api".to_string()),
        }),
    }
}

#[test]
fn conversion_roundtrip_hpa() {
    assert_conversion_roundtrip::<HorizontalPodAutoscaler, internal::HorizontalPodAutoscaler>(
        hpa_basic(),
    );
}

#[test]
fn conversion_roundtrip_hpa_list() {
    assert_conversion_roundtrip::<HorizontalPodAutoscalerList, internal::HorizontalPodAutoscalerList>(
        hpa_list_basic(),
    );
}

#[test]
fn conversion_roundtrip_scale() {
    let original = scale_basic();
    let internal = original.clone().to_internal();
    let back = Scale::from_internal(internal);
    assert_eq!(original, back, "scale conversion roundtrip mismatch");
}
