//! Trait implementation tests for imagepolicy/v1alpha1
//!
//! This module verifies that all top-level resources implement required traits.

use super::*;
use crate::common::{
    ApplyDefault, FromInternal, HasObjectMeta, ResourceSchema, ToInternal, VersionedObject,
};
use crate::imagepolicy::internal;

/// Compile-time check: verify all top-level resources implement required traits
#[test]
fn top_level_resources_implement_required_traits() {
    // Helper functions that enforce trait bounds (compile-time only)
    fn check_versioned<T: VersionedObject + ApplyDefault>() {}
    fn check_default<T: Default>() {}
    fn check_schema<T: ResourceSchema>() {}

    // Top-level resources in imagepolicy/v1alpha1
    check_versioned::<ImageReview>();
    check_default::<ImageReview>();
    check_schema::<ImageReview>();
    check_default::<ImageReviewList>();
    check_schema::<ImageReviewList>();
}

/// Compile-time check: verify prost::Message trait is implemented
#[test]
fn top_level_resources_implement_prost_message() {
    fn check_prost<T: prost::Message>() {}

    // All top-level resources implement prost::Message
    check_prost::<ImageReview>();
    check_prost::<ImageReviewList>();
}

/// Runtime check: VersionedObject trait provides correct metadata access
#[test]
fn versioned_object_metadata_access() {
    // Test with ImageReview
    let resource = ImageReview::default();
    let meta = resource.metadata();
    assert!(meta.name.is_none());

    // Test metadata_mut auto-inserts default
    let mut resource = ImageReview::default();
    resource.metadata_mut().name = Some("test".to_string());
    assert_eq!(resource.metadata().name.as_deref(), Some("test"));
}

/// Runtime check: ApplyDefault correctly sets TypeMeta
#[test]
fn apply_default_sets_type_meta() {
    // Test ImageReview
    let mut review = ImageReview::default();
    review.apply_default();
    assert_eq!(review.type_meta.api_version, "imagepolicy.k8s.io/v1alpha1");
    assert_eq!(review.type_meta.kind, "ImageReview");
}

/// Compile-time check: verify conversion traits are implemented
#[test]
fn top_level_resources_have_conversion_traits() {
    fn check_conversion<T, I>()
    where
        T: ToInternal<I> + FromInternal<I>,
    {
    }

    // Note: Internal types are re-exported from v1alpha1, so they're identical
    // but we still verify the conversion traits are implemented
    check_conversion::<ImageReview, internal::ImageReview>();
    check_conversion::<ImageReviewList, internal::ImageReviewList>();
}

/// Compile-time check: verify internal resources implement HasObjectMeta
#[test]
fn internal_resources_implement_required_traits() {
    fn check<T: HasObjectMeta>() {}

    // Note: Internal types are re-exported from v1alpha1, so they're identical
    // but we still verify HasObjectMeta is implemented
    check::<internal::ImageReview>();
    // Note: List types don't implement HasObjectMeta (they have ListMeta)
}
