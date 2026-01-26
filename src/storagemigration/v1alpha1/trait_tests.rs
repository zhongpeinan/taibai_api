//! Trait implementation tests for storagemigration/v1alpha1
//!
//! This module verifies that all top-level resources implement required traits.

use super::*;
use crate::common::{
    ApplyDefault, FromInternal, HasObjectMeta, ResourceSchema, ToInternal, VersionedObject,
};
use crate::storagemigration::internal;

/// Compile-time check: verify all top-level resources implement required traits
#[test]
fn top_level_resources_implement_required_traits() {
    // Helper functions that enforce trait bounds (compile-time only)
    fn check_versioned<T: VersionedObject + ApplyDefault>() {}
    fn check_default<T: Default>() {}
    fn check_schema<T: ResourceSchema>() {}

    // Top-level resources in storagemigration/v1alpha1
    check_versioned::<StorageVersionMigration>();
    check_default::<StorageVersionMigration>();
    check_schema::<StorageVersionMigration>();
    check_default::<StorageVersionMigrationList>();
    check_schema::<StorageVersionMigrationList>();
}

/// Compile-time check: verify prost::Message trait is implemented
#[test]
fn top_level_resources_implement_prost_message() {
    fn check_prost<T: prost::Message>() {}

    // All top-level resources implement prost::Message
    check_prost::<StorageVersionMigration>();
    check_prost::<StorageVersionMigrationList>();
}

/// Runtime check: VersionedObject trait provides correct metadata access
#[test]
fn versioned_object_metadata_access() {
    // Test with StorageVersionMigration
    let resource = StorageVersionMigration::default();
    let meta = resource.metadata();
    assert!(meta.name.is_none());

    // Test metadata_mut auto-inserts default
    let mut resource = StorageVersionMigration::default();
    resource.metadata_mut().name = Some("test".to_string());
    assert_eq!(resource.metadata().name.as_deref(), Some("test"));
}

/// Runtime check: ApplyDefault correctly sets TypeMeta
#[test]
fn apply_default_sets_type_meta() {
    // Test StorageVersionMigration
    let mut migration = StorageVersionMigration::default();
    migration.apply_default();
    assert_eq!(
        migration.type_meta.api_version,
        "storagemigration.k8s.io/v1alpha1"
    );
    assert_eq!(migration.type_meta.kind, "StorageVersionMigration");
}

/// Compile-time check: verify conversion traits are implemented
#[test]
fn top_level_resources_have_conversion_traits() {
    fn check_conversion<T, I>()
    where
        T: ToInternal<I> + FromInternal<I>,
    {
    }

    check_conversion::<StorageVersionMigration, internal::StorageVersionMigration>();
    check_conversion::<StorageVersionMigrationList, internal::StorageVersionMigrationList>();
}

/// Compile-time check: verify internal resources implement HasObjectMeta
#[test]
fn internal_resources_implement_required_traits() {
    fn check<T: HasObjectMeta>() {}

    check::<internal::StorageVersionMigration>();
    // Note: List types don't implement HasObjectMeta (they have ListMeta)
}
