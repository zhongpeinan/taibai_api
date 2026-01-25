//! Defaulting functions for autoscaling/v1 API types
//!
//! Ported from k8s.io/kubernetes/pkg/apis/autoscaling/v1/defaults.go

use super::HorizontalPodAutoscaler;

/// Apply defaults to HorizontalPodAutoscaler.
pub fn set_defaults_horizontal_pod_autoscaler(obj: &mut HorizontalPodAutoscaler) {
    if let Some(ref mut spec) = obj.spec {
        if spec.min_replicas.is_none() {
            spec.min_replicas = Some(1);
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
    fn test_default_min_replicas() {
        let mut hpa = HorizontalPodAutoscaler {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("hpa".to_string()),
                ..Default::default()
            }),
            spec: Some(Default::default()),
            status: None,
        };

        set_defaults_horizontal_pod_autoscaler(&mut hpa);
        assert_eq!(hpa.spec.as_ref().unwrap().min_replicas, Some(1));
    }
}
