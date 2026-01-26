//! Trait implementation tests for resource/v1
//!
//! This module verifies that all top-level resources implement required traits.

use super::*;
use crate::common::{
    ApplyDefault, FromInternal, HasObjectMeta, ResourceSchema, ToInternal, VersionedObject,
};
use crate::resource::internal;

/// Compile-time check: verify all top-level resources implement required traits
#[test]
fn top_level_resources_implement_required_traits() {
    // Helper functions that enforce trait bounds (compile-time only)
    fn check_versioned<T: VersionedObject + ApplyDefault>() {}
    fn check_default<T: Default>() {}
    fn check_schema<T: ResourceSchema>() {}

    // DeviceClass
    check_versioned::<DeviceClass>();
    check_default::<DeviceClass>();
    check_schema::<DeviceClass>();
    check_default::<DeviceClassList>();
    check_schema::<DeviceClassList>();

    // ResourceClaim
    check_versioned::<ResourceClaim>();
    check_default::<ResourceClaim>();
    check_schema::<ResourceClaim>();
    check_default::<ResourceClaimList>();
    check_schema::<ResourceClaimList>();

    // ResourceClaimTemplate
    check_versioned::<ResourceClaimTemplate>();
    check_default::<ResourceClaimTemplate>();
    check_schema::<ResourceClaimTemplate>();
    check_default::<ResourceClaimTemplateList>();
    check_schema::<ResourceClaimTemplateList>();

    // ResourceSlice
    check_versioned::<ResourceSlice>();
    check_default::<ResourceSlice>();
    check_schema::<ResourceSlice>();
    check_default::<ResourceSliceList>();
    check_schema::<ResourceSliceList>();
}

/// Runtime check: VersionedObject trait provides correct metadata access
#[test]
fn versioned_object_metadata_access() {
    // Test with DeviceClass
    let resource = DeviceClass::default();
    let meta = resource.metadata();
    assert!(meta.name.is_none());

    // Test metadata_mut auto-inserts default
    let mut resource = DeviceClass::default();
    resource.metadata_mut().name = Some("test".to_string());
    assert_eq!(resource.metadata().name.as_deref(), Some("test"));
}

/// Runtime check: ApplyDefault correctly sets TypeMeta
#[test]
fn apply_default_sets_type_meta() {
    // Test DeviceClass
    let mut device_class = DeviceClass::default();
    device_class.apply_default();
    assert_eq!(device_class.type_meta.api_version, "resource.k8s.io/v1");
    assert_eq!(device_class.type_meta.kind, "DeviceClass");

    // Test ResourceClaim
    let mut claim = ResourceClaim::default();
    claim.apply_default();
    assert_eq!(claim.type_meta.api_version, "resource.k8s.io/v1");
    assert_eq!(claim.type_meta.kind, "ResourceClaim");

    // Test ResourceClaimTemplate
    let mut template = ResourceClaimTemplate::default();
    template.apply_default();
    assert_eq!(template.type_meta.api_version, "resource.k8s.io/v1");
    assert_eq!(template.type_meta.kind, "ResourceClaimTemplate");

    // Test ResourceSlice
    let mut slice = ResourceSlice::default();
    slice.apply_default();
    assert_eq!(slice.type_meta.api_version, "resource.k8s.io/v1");
    assert_eq!(slice.type_meta.kind, "ResourceSlice");
}

/// Compile-time check: verify prost::Message trait is implemented
#[test]
fn top_level_resources_implement_prost_message() {
    fn check_prost<T: prost::Message>() {}

    check_prost::<DeviceClass>();
    check_prost::<DeviceClassList>();
    check_prost::<ResourceClaim>();
    check_prost::<ResourceClaimList>();
    check_prost::<ResourceClaimTemplate>();
    check_prost::<ResourceClaimTemplateList>();
    check_prost::<ResourceSlice>();
    check_prost::<ResourceSliceList>();
}

/// Compile-time check: verify conversion traits are implemented
#[test]
fn top_level_resources_have_conversion_traits() {
    fn check_conversion<T, I>()
    where
        T: ToInternal<I> + FromInternal<I>,
    {
    }

    // Note: Only resource types have internal counterparts, not List types
    check_conversion::<DeviceClass, internal::DeviceClass>();
    check_conversion::<ResourceClaim, internal::ResourceClaim>();
    check_conversion::<ResourceClaimTemplate, internal::ResourceClaimTemplate>();
    check_conversion::<ResourceSlice, internal::ResourceSlice>();
}

/// Compile-time check: verify internal resources implement HasObjectMeta
#[test]
fn internal_resources_implement_required_traits() {
    fn check<T: HasObjectMeta>() {}

    check::<internal::DeviceClass>();
    check::<internal::ResourceClaim>();
    check::<internal::ResourceClaimTemplate>();
    check::<internal::ResourceSlice>();
    // Note: List types don't implement HasObjectMeta (they have ListMeta)
}
