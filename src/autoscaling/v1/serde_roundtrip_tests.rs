use super::{
    CrossVersionObjectReference, HorizontalPodAutoscaler, HorizontalPodAutoscalerList,
    HorizontalPodAutoscalerSpec, HorizontalPodAutoscalerStatus, Scale, ScaleSpec, ScaleStatus,
};
use crate::common::test_utils::assert_serde_roundtrip;
use crate::common::{ListMeta, ObjectMeta, Timestamp, TypeMeta};

fn hpa_basic() -> HorizontalPodAutoscaler {
    HorizontalPodAutoscaler {
        type_meta: TypeMeta {
            api_version: "autoscaling/v1".to_string(),
            kind: "HorizontalPodAutoscaler".to_string(),
        },
        metadata: Some(ObjectMeta {
            name: Some("hpa-a".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        spec: Some(HorizontalPodAutoscalerSpec {
            scale_target_ref: CrossVersionObjectReference {
                kind: "Deployment".to_string(),
                name: "web".to_string(),
                api_version: Some("apps/v1".to_string()),
            },
            min_replicas: Some(1),
            max_replicas: 10,
            target_cpu_utilization_percentage: Some(80),
        }),
        status: Some(HorizontalPodAutoscalerStatus {
            observed_generation: Some(1),
            last_scale_time: Some(Timestamp::from_str("2024-01-01T00:00:00Z").unwrap()),
            current_replicas: 3,
            desired_replicas: 5,
            current_cpu_utilization_percentage: Some(70),
        }),
    }
}

fn hpa_list_basic() -> HorizontalPodAutoscalerList {
    HorizontalPodAutoscalerList {
        type_meta: TypeMeta {
            api_version: "autoscaling/v1".to_string(),
            kind: "HorizontalPodAutoscalerList".to_string(),
        },
        metadata: Some(ListMeta {
            resource_version: Some("1".to_string()),
            ..Default::default()
        }),
        items: vec![hpa_basic()],
    }
}

fn scale_basic() -> Scale {
    Scale {
        metadata: Some(ObjectMeta {
            name: Some("scale-a".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        spec: Some(ScaleSpec { replicas: Some(3) }),
        status: Some(ScaleStatus {
            replicas: 3,
            selector: Some("app=web".to_string()),
        }),
    }
}

#[test]
fn serde_roundtrip_hpa() {
    assert_serde_roundtrip(&hpa_basic());
}

#[test]
fn serde_roundtrip_hpa_list() {
    assert_serde_roundtrip(&hpa_list_basic());
}

#[test]
fn serde_roundtrip_scale() {
    assert_serde_roundtrip(&scale_basic());
}
