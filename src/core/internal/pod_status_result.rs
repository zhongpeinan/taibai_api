//! PodStatusResult types from the Kubernetes Core API (Internal)
//!
//! This module contains internal types for pod status result resources.
//!
//! Source: k8s.io/kubernetes/pkg/apis/core/types.go

use crate::common::{ObjectMeta, TypeMeta};
use crate::core::internal::PodStatus;
use crate::impl_has_object_meta;
use serde::{Deserialize, Serialize};

// ============================================================================
// PodStatusResult
// ============================================================================

/// PodStatusResult represents the status of a pod.
///
/// Corresponds to [Kubernetes PodStatusResult](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/core/types.go)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PodStatusResult {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    pub metadata: ObjectMeta,
    /// Most recently observed status of the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PodStatus>,
}
impl_has_object_meta!(PodStatusResult);

impl Default for PodStatusResult {
    fn default() -> Self {
        Self {
            type_meta: TypeMeta::default(),
            metadata: ObjectMeta::default(),
            status: None,
        }
    }
}

// ============================================================================
// Trait Implementations
// ============================================================================

impl crate::common::traits::ResourceSchema for PodStatusResult {
    type Meta = ();

    fn group(_meta: &Self::Meta) -> &str {
        ""
    }

    fn version(_meta: &Self::Meta) -> &str {
        "v1"
    }

    fn kind(_meta: &Self::Meta) -> &str {
        "PodStatusResult"
    }

    fn resource(_meta: &Self::Meta) -> &str {
        "pods/status"
    }

    fn group_static() -> &'static str
    where
        Self::Meta: Default,
    {
        ""
    }

    fn version_static() -> &'static str
    where
        Self::Meta: Default,
    {
        "v1"
    }

    fn kind_static() -> &'static str
    where
        Self::Meta: Default,
    {
        "PodStatusResult"
    }

    fn resource_static() -> &'static str
    where
        Self::Meta: Default,
    {
        "pods/status"
    }
}

impl crate::common::traits::HasTypeMeta for PodStatusResult {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }

    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pod_status_result_default() {
        let psr = PodStatusResult::default();
        assert!(psr.metadata.name.is_none());
        assert!(psr.status.is_none());
    }

    #[test]
    fn test_pod_status_result_with_status() {
        let psr = PodStatusResult {
            type_meta: TypeMeta::default(),
            metadata: ObjectMeta {
                name: Some("my-pod".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            },
            status: Some(PodStatus {
                phase: Some("Running".to_string()),
                ..Default::default()
            }),
        };
        assert!(psr.metadata.name.is_some());
        assert!(psr.status.is_some());
        assert_eq!(psr.status.as_ref().unwrap().phase, Some("Running".to_string()));
    }

    #[test]
    fn test_pod_status_result_serialize() {
        let psr = PodStatusResult {
            type_meta: TypeMeta {
                kind: "PodStatusResult".to_string(),
                api_version: "v1".to_string(),
            },
            metadata: ObjectMeta {
                name: Some("test-pod".to_string()),
                ..Default::default()
            },
            status: Some(PodStatus::default()),
        };
        let json = serde_json::to_string(&psr).unwrap();
        assert!(json.contains(r#""kind":"PodStatusResult""#));
        assert!(json.contains(r#""name":"test-pod""#));
    }

    #[test]
    fn test_pod_status_result_round_trip() {
        let original = PodStatusResult {
            type_meta: TypeMeta {
                kind: "PodStatusResult".to_string(),
                api_version: "v1".to_string(),
            },
            metadata: ObjectMeta {
                name: Some("round-trip-pod".to_string()),
                namespace: Some("kube-system".to_string()),
                ..Default::default()
            },
            status: Some(PodStatus {
                phase: Some("Running".to_string()),
                conditions: vec![],
                ..Default::default()
            }),
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: PodStatusResult = serde_json::from_str(&json).unwrap();
        assert_eq!(original.metadata.name, deserialized.metadata.name);
    }

    #[test]
    fn test_pod_status_result_with_full_status() {
        let psr = PodStatusResult {
            type_meta: TypeMeta::default(),
            metadata: ObjectMeta {
                name: Some("full-status-pod".to_string()),
                ..Default::default()
            },
            status: Some(PodStatus {
                phase: Some("Running".to_string()),
                conditions: vec![],
                message: "Pod is running".to_string(),
                reason: "Running".to_string(),
                host_ip: "10.0.0.1".to_string(),
                pod_ip: "10.244.1.5".to_string(),
                ..Default::default()
            }),
        };
        assert!(psr.status.is_some());
        let status = psr.status.unwrap();
        assert_eq!(status.phase, Some("Running".to_string()));
        assert_eq!(status.message, "Pod is running");
    }

    #[test]
    fn test_has_object_meta_impl() {
        let mut psr = PodStatusResult::default();
        psr.meta_mut().name = Some("test-pod".to_string());
        assert_eq!(psr.meta().name, Some("test-pod".to_string()));
    }

    #[test]
    fn test_pod_status_result_type_meta_accessor() {
        let psr = PodStatusResult {
            type_meta: TypeMeta {
                api_version: "v1".to_string(),
                kind: "PodStatusResult".to_string(),
            },
            ..Default::default()
        };
        assert_eq!(psr.type_meta().api_version, "v1");
        assert_eq!(psr.type_meta().kind, "PodStatusResult");
    }

    // Integration tests
    #[test]
    fn test_pod_status_result_with_container_statuses() {
        let psr = PodStatusResult {
            type_meta: TypeMeta::default(),
            metadata: ObjectMeta {
                name: Some("multi-container-pod".to_string()),
                ..Default::default()
            },
            status: Some(PodStatus {
                phase: Some("Running".to_string()),
                container_statuses: Some(vec![]),
                ..Default::default()
            }),
        };
        assert!(psr.status.is_some());
        assert!(psr.status.unwrap().container_statuses.is_some());
    }
}
