//! Trait implementation tests for policy/v1
//!
//! This module verifies that all top-level resources implement required traits.

use super::*;
use crate::common::{
    ApplyDefault, FromInternal, HasObjectMeta, ResourceSchema, ToInternal, VersionedObject,
};
use crate::policy::internal;

/// Compile-time check: verify all top-level resources implement required traits
#[test]
fn top_level_resources_implement_required_traits() {
    // Helper functions that enforce trait bounds (compile-time only)
    fn check_versioned<T: VersionedObject + ApplyDefault>() {}
    fn check_default<T: Default>() {}
    fn check_schema<T: ResourceSchema>() {}

    // Top-level resources in policy/v1
    check_versioned::<PodDisruptionBudget>();
    check_default::<PodDisruptionBudget>();
    check_schema::<PodDisruptionBudget>();
    check_default::<PodDisruptionBudgetList>();
    check_schema::<PodDisruptionBudgetList>();

    check_versioned::<Eviction>();
    check_default::<Eviction>();
    check_schema::<Eviction>();
}

/// Compile-time check: verify prost::Message trait is implemented
#[test]
fn top_level_resources_implement_prost_message() {
    fn check_prost<T: prost::Message>() {}

    // All top-level resources implement prost::Message
    check_prost::<PodDisruptionBudget>();
    check_prost::<PodDisruptionBudgetList>();
    check_prost::<Eviction>();
}

/// Runtime check: VersionedObject trait provides correct metadata access
#[test]
fn versioned_object_metadata_access() {
    // Test with PodDisruptionBudget
    let resource = PodDisruptionBudget::default();
    let meta = resource.metadata();
    assert!(meta.name.is_none());

    // Test metadata_mut auto-inserts default
    let mut resource = PodDisruptionBudget::default();
    resource.metadata_mut().name = Some("test".to_string());
    assert_eq!(resource.metadata().name.as_deref(), Some("test"));
}

/// Runtime check: ApplyDefault correctly sets TypeMeta
#[test]
fn apply_default_sets_type_meta() {
    // Test PodDisruptionBudget
    let mut pdb = PodDisruptionBudget::default();
    pdb.apply_default();
    assert_eq!(pdb.type_meta.api_version, "policy/v1");
    assert_eq!(pdb.type_meta.kind, "PodDisruptionBudget");

    // Test Eviction
    let mut eviction = Eviction::default();
    eviction.apply_default();
    assert_eq!(eviction.type_meta.api_version, "policy/v1");
    assert_eq!(eviction.type_meta.kind, "Eviction");
}

/// Compile-time check: verify conversion traits are implemented
#[test]
fn top_level_resources_have_conversion_traits() {
    fn check_conversion<T, I>()
    where
        T: ToInternal<I> + FromInternal<I>,
    {
    }

    check_conversion::<PodDisruptionBudget, internal::PodDisruptionBudget>();
    check_conversion::<PodDisruptionBudgetList, internal::PodDisruptionBudgetList>();
    check_conversion::<Eviction, internal::Eviction>();
}

/// Compile-time check: verify internal resources implement HasObjectMeta
#[test]
fn internal_resources_implement_required_traits() {
    fn check<T: HasObjectMeta>() {}

    check::<internal::PodDisruptionBudget>();
    check::<internal::Eviction>();
    // Note: List types don't implement HasObjectMeta (they have ListMeta)
}
