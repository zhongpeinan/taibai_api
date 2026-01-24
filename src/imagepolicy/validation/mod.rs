//! Validation for Kubernetes ImagePolicy API types
//!
//! Note: Kubernetes upstream does not validate imagepolicy types.
//! This module provides no-op validation functions that always return empty errors,
//! matching upstream behavior.

use crate::common::validation::{ErrorList, Path};
use crate::imagepolicy::v1alpha1::{ImageReview, ImageReviewList};

// ============================================================================
// ImageReview Validation
// ============================================================================

pub fn validate_image_review(obj: &ImageReview) -> ErrorList {
    validate_image_review_with_path(obj, &Path::nil())
}

fn validate_image_review_with_path(_obj: &ImageReview, _base_path: &Path) -> ErrorList {
    

    ErrorList::new()
}

pub fn validate_image_review_list(obj: &ImageReviewList) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (i, item) in obj.items.iter().enumerate() {
        let item_path = Path::new("items").index(i);
        all_errs.extend(validate_image_review_with_path(item, &item_path));
    }

    all_errs
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::TypeMeta;
    use std::collections::BTreeMap;

    #[test]
    fn test_validate_image_review_valid() {
        let obj = ImageReview {
            type_meta: TypeMeta {
                api_version: "imagepolicy.k8s.io/v1alpha1".to_string(),
                kind: "ImageReview".to_string(),
            },
            metadata: None,
            spec: crate::imagepolicy::v1alpha1::ImageReviewSpec {
                containers: vec![crate::imagepolicy::v1alpha1::ImageReviewContainerSpec {
                    image: "nginx:latest".to_string(),
                }],
                annotations: BTreeMap::new(),
                namespace: "default".to_string(),
            },
            status: Some(crate::imagepolicy::v1alpha1::ImageReviewStatus {
                allowed: true,
                reason: String::new(),
                audit_annotations: BTreeMap::new(),
            }),
        };

        let errs = validate_image_review(&obj);
        assert!(
            errs.is_empty(),
            "expected no errors, got: {:?}",
            errs.errors
        );
    }

    #[test]
    fn test_validate_image_review_empty_namespace() {
        let obj = ImageReview {
            type_meta: TypeMeta {
                api_version: "imagepolicy.k8s.io/v1alpha1".to_string(),
                kind: "ImageReview".to_string(),
            },
            metadata: None,
            spec: crate::imagepolicy::v1alpha1::ImageReviewSpec {
                containers: vec![],
                annotations: BTreeMap::new(),
                namespace: String::new(),
            },
            status: None,
        };

        let errs = validate_image_review(&obj);
        assert!(errs.is_empty());
    }

    #[test]
    fn test_validate_image_review_list_item_index() {
        // List validation should allow empty namespace in items
        let obj = ImageReviewList {
            type_meta: TypeMeta {
                api_version: "imagepolicy.k8s.io/v1alpha1".to_string(),
                kind: "ImageReviewList".to_string(),
            },
            metadata: None,
            items: vec![ImageReview {
                type_meta: TypeMeta {
                    api_version: "imagepolicy.k8s.io/v1alpha1".to_string(),
                    kind: "ImageReview".to_string(),
                },
                metadata: None,
                spec: crate::imagepolicy::v1alpha1::ImageReviewSpec {
                    containers: vec![],
                    annotations: BTreeMap::new(),
                    namespace: String::new(),
                },
                status: None,
            }],
        };

        let errs = validate_image_review_list(&obj);
        assert!(errs.is_empty());
    }
}
