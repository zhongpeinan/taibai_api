//! ImagePolicy v1alpha1 API types
//!
//! This module contains the v1alpha1 version of the Kubernetes ImagePolicy API types.
//!
//! The imagepolicy API is used for image review webhooks that can validate
//! whether images in a pod are allowed to run.
//!
//! Source: api-master/imagepolicy/v1alpha1/types.go

use crate::common::{
    ApplyDefaults, HasTypeMeta, ListMeta, ObjectMeta, ResourceSchema, TypeMeta,
    UnimplementedConversion, VersionedObject,
};
use crate::impl_unimplemented_prost_message;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::OnceLock;

// ============================================================================
// ImageReview Types
// ============================================================================

/// ImageReview checks if the set of images in a pod are allowed.
///
/// This resource is used by image review webhooks to validate container images
/// before they are allowed to run in a pod.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ImageReview {
    /// TypeMeta for this resource
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard object metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// Spec holds information about the pod being evaluated.
    #[serde(default)]
    pub spec: ImageReviewSpec,

    /// Status is filled in by the backend and indicates whether the pod should be allowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ImageReviewStatus>,
}

/// ImageReviewSpec is a description of the pod creation request.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ImageReviewSpec {
    /// Containers is a list of a subset of the information in each container of the Pod being created.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub containers: Vec<ImageReviewContainerSpec>,

    /// Annotations is a list of key-value pairs extracted from the Pod's annotations.
    /// It only includes keys which match the pattern `*.image-policy.k8s.io/*`.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub annotations: BTreeMap<String, String>,

    /// Namespace is the namespace the pod is being created in.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub namespace: String,
}

/// ImageReviewContainerSpec is a description of a container within the pod creation request.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ImageReviewContainerSpec {
    /// This can be in the form image:tag or image@SHA:012345679abcdef.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub image: String,
}

/// ImageReviewStatus is the result of the review for the pod creation request.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ImageReviewStatus {
    /// Allowed indicates that all images were allowed to be run.
    #[serde(default)]
    pub allowed: bool,

    /// Reason should be empty unless Allowed is false in which case it
    /// may contain a short description of what is wrong.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,

    /// AuditAnnotations will be added to the attributes object of the
    /// admission controller request using 'AddAnnotation'.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub audit_annotations: BTreeMap<String, String>,
}

// ============================================================================
// ImageReviewList
// ============================================================================

/// ImageReviewList is a list of ImageReview resources.
///
/// Note: This is not part of the original Go API but is provided for completeness.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ImageReviewList {
    /// TypeMeta for this resource
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// List of ImageReview resources.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ImageReview>,
}

// ============================================================================
// Constants
// ============================================================================

/// Annotation prefix for image policy annotations.
pub const IMAGE_POLICY_ANNOTATION_PREFIX: &str = "image-policy.k8s.io/";

// ============================================================================
// Trait Implementations for ImageReview and ImageReviewList
// ============================================================================

// ----------------------------------------------------------------------------
// ResourceSchema Implementation
// ----------------------------------------------------------------------------

impl ResourceSchema for ImageReview {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "imagepolicy.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1alpha1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "ImageReview"
    }
    fn resource(_: &Self::Meta) -> &str {
        "imagereviews"
    }

    fn group_static() -> &'static str {
        "imagepolicy.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1alpha1"
    }
    fn kind_static() -> &'static str {
        "ImageReview"
    }
    fn resource_static() -> &'static str {
        "imagereviews"
    }
}

impl ResourceSchema for ImageReviewList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "imagepolicy.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1alpha1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "ImageReviewList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "imagereviews"
    }

    fn group_static() -> &'static str {
        "imagepolicy.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1alpha1"
    }
    fn kind_static() -> &'static str {
        "ImageReviewList"
    }
    fn resource_static() -> &'static str {
        "imagereviews"
    }
}

// ----------------------------------------------------------------------------
// HasTypeMeta Implementation
// ----------------------------------------------------------------------------

impl HasTypeMeta for ImageReview {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for ImageReviewList {
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

impl VersionedObject for ImageReview {
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

// Note: ImageReviewList does not implement VersionedObject because its metadata is ListMeta

// ----------------------------------------------------------------------------
// ApplyDefaults Implementation
// ----------------------------------------------------------------------------

impl ApplyDefaults for ImageReview {
    fn apply_defaults(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "imagepolicy.k8s.io/v1alpha1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "ImageReview".to_string();
        }
    }
}

impl ApplyDefaults for ImageReviewList {
    fn apply_defaults(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "imagepolicy.k8s.io/v1alpha1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "ImageReviewList".to_string();
        }
    }
}

// ----------------------------------------------------------------------------
// Version Conversion Placeholder (using UnimplementedConversion)
// ----------------------------------------------------------------------------

impl UnimplementedConversion for ImageReview {}

// ----------------------------------------------------------------------------
// Protobuf Placeholder (using macro)
// ----------------------------------------------------------------------------

impl_unimplemented_prost_message!(ImageReview);
impl_unimplemented_prost_message!(ImageReviewList);

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    // ImageReview tests
    #[test]
    fn test_image_review_default() {
        let review = ImageReview::default();
        assert!(review.metadata.is_none());
        assert!(review.status.is_none());
    }

    #[test]
    fn test_image_review_with_spec() {
        let review = ImageReview {
            spec: ImageReviewSpec {
                namespace: "default".to_string(),
                containers: vec![ImageReviewContainerSpec {
                    image: "nginx:latest".to_string(),
                    ..Default::default()
                }],
                ..Default::default()
            },
            ..Default::default()
        };
        assert_eq!(review.spec.namespace, "default");
        assert_eq!(review.spec.containers.len(), 1);
    }

    #[test]
    fn test_image_review_serialize() {
        let review = ImageReview {
            spec: ImageReviewSpec {
                namespace: "default".to_string(),
                containers: vec![ImageReviewContainerSpec {
                    image: "nginx:latest".to_string(),
                    ..Default::default()
                }],
                ..Default::default()
            },
            status: Some(ImageReviewStatus {
                allowed: true,
                ..Default::default()
            }),
            ..Default::default()
        };
        let json = serde_json::to_string(&review).unwrap();
        assert!(json.contains(r#""namespace":"default""#));
        assert!(json.contains(r#""image":"nginx:latest""#));
        assert!(json.contains(r#""allowed":true"#));
    }

    #[test]
    fn test_image_review_deserialize() {
        let json = r#"{
            "spec": {
                "namespace": "default",
                "containers": [{"image": "nginx:latest"}]
            },
            "status": {
                "allowed": true
            }
        }"#;
        let review: ImageReview = serde_json::from_str(json).unwrap();
        assert_eq!(review.spec.namespace, "default");
        assert_eq!(review.spec.containers.len(), 1);
        assert_eq!(review.spec.containers[0].image, "nginx:latest");
        assert_eq!(review.status.unwrap().allowed, true);
    }

    #[test]
    fn test_image_review_round_trip() {
        let original = ImageReview {
            spec: ImageReviewSpec {
                namespace: "test-ns".to_string(),
                containers: vec![
                    ImageReviewContainerSpec {
                        image: "nginx:1.19".to_string(),
                        ..Default::default()
                    },
                    ImageReviewContainerSpec {
                        image: "redis@sha256:abcdef".to_string(),
                        ..Default::default()
                    },
                ],
                annotations: {
                    let mut map = BTreeMap::new();
                    map.insert("key".to_string(), "value".to_string());
                    map
                },
                ..Default::default()
            },
            status: Some(ImageReviewStatus {
                allowed: false,
                reason: "Image not allowed".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: ImageReview = serde_json::from_str(&json).unwrap();
        assert_eq!(original.spec.namespace, deserialized.spec.namespace);
        assert_eq!(
            original.spec.containers.len(),
            deserialized.spec.containers.len()
        );
        assert_eq!(
            original.status.as_ref().unwrap().allowed,
            deserialized.status.as_ref().unwrap().allowed
        );
    }

    // ImageReviewSpec tests
    #[test]
    fn test_image_review_spec_default() {
        let spec = ImageReviewSpec::default();
        assert!(spec.containers.is_empty());
        assert!(spec.annotations.is_empty());
        assert!(spec.namespace.is_empty());
    }

    #[test]
    fn test_image_review_spec_with_containers() {
        let spec = ImageReviewSpec {
            containers: vec![
                ImageReviewContainerSpec {
                    image: "nginx:latest".to_string(),
                    ..Default::default()
                },
                ImageReviewContainerSpec {
                    image: "redis:alpine".to_string(),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        assert_eq!(spec.containers.len(), 2);
    }

    #[test]
    fn test_image_review_spec_with_annotations() {
        let mut annotations = BTreeMap::new();
        annotations.insert("image-policy.k8s.io/a".to_string(), "value1".to_string());
        annotations.insert("image-policy.k8s.io/b".to_string(), "value2".to_string());

        let spec = ImageReviewSpec {
            annotations,
            ..Default::default()
        };
        assert_eq!(spec.annotations.len(), 2);
    }

    // ImageReviewContainerSpec tests
    #[test]
    fn test_image_review_container_spec_default() {
        let container = ImageReviewContainerSpec::default();
        assert!(container.image.is_empty());
    }

    #[test]
    fn test_image_review_container_spec_with_image() {
        let container = ImageReviewContainerSpec {
            image: "myregistry.io/image:tag".to_string(),
        };
        assert_eq!(container.image, "myregistry.io/image:tag");
    }

    // ImageReviewStatus tests
    #[test]
    fn test_image_review_status_default() {
        let status = ImageReviewStatus::default();
        assert!(!status.allowed);
        assert!(status.reason.is_empty());
    }

    #[test]
    fn test_image_review_status_allowed() {
        let status = ImageReviewStatus {
            allowed: true,
            ..Default::default()
        };
        assert!(status.allowed);
    }

    #[test]
    fn test_image_review_status_denied() {
        let status = ImageReviewStatus {
            allowed: false,
            reason: "Image not whitelisted".to_string(),
            ..Default::default()
        };
        assert!(!status.allowed);
        assert_eq!(status.reason, "Image not whitelisted");
    }

    #[test]
    fn test_image_review_status_with_audit_annotations() {
        let mut audit_annotations = BTreeMap::new();
        audit_annotations.insert("key1".to_string(), "value1".to_string());
        audit_annotations.insert("key2".to_string(), "value2".to_string());

        let status = ImageReviewStatus {
            allowed: true,
            audit_annotations,
            ..Default::default()
        };
        assert_eq!(status.audit_annotations.len(), 2);
    }

    // ImageReviewList tests
    #[test]
    fn test_image_review_list_default() {
        let list = ImageReviewList::default();
        assert!(list.items.is_empty());
    }

    #[test]
    fn test_image_review_list_with_items() {
        let list = ImageReviewList {
            items: vec![ImageReview::default(), ImageReview::default()],
            ..Default::default()
        };
        assert_eq!(list.items.len(), 2);
    }

    // Integration test
    #[test]
    fn test_full_image_review_workflow() {
        let review = ImageReview {
            spec: ImageReviewSpec {
                namespace: "production".to_string(),
                containers: vec![ImageReviewContainerSpec {
                    image: "nginx:1.19".to_string(),
                    ..Default::default()
                }],
                ..Default::default()
            },
            status: Some(ImageReviewStatus {
                allowed: true,
                ..Default::default()
            }),
            ..Default::default()
        };

        // Serialize and deserialize
        let json = serde_json::to_string_pretty(&review).unwrap();
        let deserialized: ImageReview = serde_json::from_str(&json).unwrap();

        assert_eq!(review.spec.namespace, deserialized.spec.namespace);
        assert_eq!(
            review.status.as_ref().unwrap().allowed,
            deserialized.status.as_ref().unwrap().allowed
        );
        assert_eq!(
            review.status.as_ref().unwrap().audit_annotations.len(),
            deserialized
                .status
                .as_ref()
                .unwrap()
                .audit_annotations
                .len()
        );
    }

    // Constants tests
    #[test]
    fn test_constants() {
        assert_eq!(IMAGE_POLICY_ANNOTATION_PREFIX, "image-policy.k8s.io/");
    }

    // Edge case tests
    #[test]
    fn test_empty_image_review() {
        let json = r#"{"spec":{},"status":{"allowed":false}}"#;
        let review: ImageReview = serde_json::from_str(json).unwrap();
        assert!(!review.status.unwrap().allowed);
    }

    #[test]
    fn test_image_review_with_sha() {
        let review = ImageReview {
            spec: ImageReviewSpec {
                containers: vec![ImageReviewContainerSpec {
                    image: "nginx@sha256:abcdef1234567890".to_string(),
                    ..Default::default()
                }],
                ..Default::default()
            },
            ..Default::default()
        };
        assert_eq!(
            review.spec.containers[0].image,
            "nginx@sha256:abcdef1234567890"
        );
    }

    #[test]
    fn test_image_review_denied_with_reason() {
        let json = r#"{
            "spec": {
                "containers": [{"image": "untrusted:image"}]
            },
            "status": {
                "allowed": false,
                "reason": "Image not in allowed registry"
            }
        }"#;
        let review: ImageReview = serde_json::from_str(json).unwrap();
        let status = review.status.unwrap();
        assert!(!status.allowed);
        assert_eq!(status.reason, "Image not in allowed registry");
    }
}
