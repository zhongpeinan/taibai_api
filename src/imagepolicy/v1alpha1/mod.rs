//! ImagePolicy v1alpha1 API types
//!
//! This module contains the v1alpha1 version of the Kubernetes ImagePolicy API types.
//!
//! The imagepolicy API is used for image review webhooks that can validate
//! whether images in a pod are allowed to run.
//!
//! Source: api-master/imagepolicy/v1alpha1/types.go

use crate::common::{
    ApplyDefault, HasTypeMeta, ListMeta, ObjectMeta, ResourceSchema, TypeMeta,
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

impl ApplyDefault for ImageReview {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "imagepolicy.k8s.io/v1alpha1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "ImageReview".to_string();
        }
    }
}

impl ApplyDefault for ImageReviewList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "imagepolicy.k8s.io/v1alpha1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "ImageReviewList".to_string();
        }
    }
}

// ----------------------------------------------------------------------------
// Protobuf Placeholder (using macro)
// ----------------------------------------------------------------------------

impl_unimplemented_prost_message!(ImageReview);
impl_unimplemented_prost_message!(ImageReviewList);

// ----------------------------------------------------------------------------
// Conversion Placeholder (internal types are same as v1alpha1)
// ----------------------------------------------------------------------------

impl UnimplementedConversion for ImageReview {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{FromInternal, ToInternal};
    use crate::imagepolicy::internal;

    // ========================================================================
    // Compile-time Trait Checks
    // ========================================================================

    /// 编译时检查：确保顶级资源实现了必需的 traits
    #[test]
    fn top_level_resources_implement_required_traits() {
        fn check<T: VersionedObject + ApplyDefault>() {}

        check::<ImageReview>();
    }

    /// 编译时检查：确保资源实现了版本转换 traits
    #[test]
    fn conversion_traits() {
        fn check<T, I>()
        where
            T: ToInternal<I> + FromInternal<I>,
        {
        }

        check::<ImageReview, internal::ImageReview>();
    }

    /// 编译时检查：确保资源实现了 prost::Message
    #[test]
    fn prost_message() {
        fn check<T: prost::Message>() {}

        check::<ImageReview>();
        check::<ImageReviewList>();
    }

    // ========================================================================
    // Runtime Behavior Tests
    // ========================================================================

    #[test]
    fn test_image_review_apply_default() {
        let mut obj = ImageReview::default();
        obj.apply_default();
        assert_eq!(obj.type_meta.api_version, "imagepolicy.k8s.io/v1alpha1");
        assert_eq!(obj.type_meta.kind, "ImageReview");
    }

    #[test]
    fn test_image_review_list_apply_default() {
        let mut obj = ImageReviewList::default();
        obj.apply_default();
        assert_eq!(obj.type_meta.api_version, "imagepolicy.k8s.io/v1alpha1");
        assert_eq!(obj.type_meta.kind, "ImageReviewList");
    }

    #[test]
    fn test_image_review_with_metadata() {
        let obj = ImageReview {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-review".to_string()),
                ..Default::default()
            }),
            spec: ImageReviewSpec::default(),
            status: None,
        };
        assert_eq!(
            obj.metadata.as_ref().unwrap().name,
            Some("test-review".to_string())
        );
    }
}
