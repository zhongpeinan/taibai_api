//! Trait implementation tests for scheduling/v1
//!
//! This module verifies that all top-level resources implement required traits.

use super::*;
use crate::common::{ApplyDefault, FromInternal, ResourceSchema, ToInternal, VersionedObject};
use crate::scheduling::v1::conversion::{PriorityClassInternal, PriorityClassListInternal};

/// Compile-time check: verify all top-level resources implement required traits
#[test]
fn top_level_resources_implement_required_traits() {
    // Helper functions that enforce trait bounds (compile-time only)
    fn check_versioned<T: VersionedObject + ApplyDefault>() {}
    fn check_default<T: Default>() {}
    fn check_schema<T: ResourceSchema>() {}

    // Top-level resources in scheduling/v1
    check_versioned::<PriorityClass>();
    check_default::<PriorityClass>();
    check_schema::<PriorityClass>();
    check_default::<PriorityClassList>();
    check_schema::<PriorityClassList>();
}

/// Compile-time check: verify prost::Message trait is implemented
#[test]
fn top_level_resources_implement_prost_message() {
    fn check_prost<T: prost::Message>() {}

    // All top-level resources implement prost::Message
    check_prost::<PriorityClass>();
    check_prost::<PriorityClassList>();
}

/// Runtime check: VersionedObject trait provides correct metadata access
#[test]
fn versioned_object_metadata_access() {
    // Test with PriorityClass
    let resource = PriorityClass::default();
    let meta = resource.metadata();
    assert!(meta.name.is_none());

    // Test metadata_mut auto-inserts default
    let mut resource = PriorityClass::default();
    resource.metadata_mut().name = Some("test".to_string());
    assert_eq!(resource.metadata().name.as_deref(), Some("test"));
}

/// Runtime check: ApplyDefault correctly sets TypeMeta
#[test]
fn apply_default_sets_type_meta() {
    // Test PriorityClass
    let mut priority_class = PriorityClass::default();
    priority_class.apply_default();
    assert_eq!(priority_class.type_meta.api_version, "scheduling.k8s.io/v1");
    assert_eq!(priority_class.type_meta.kind, "PriorityClass");
}

/// Compile-time check: verify conversion traits are implemented
#[test]
fn top_level_resources_have_conversion_traits() {
    fn check_conversion<T, I>()
    where
        T: ToInternal<I> + FromInternal<I>,
    {
    }

    check_conversion::<PriorityClass, PriorityClassInternal>();
    check_conversion::<PriorityClassList, PriorityClassListInternal>();
}

// Note: scheduling/v1 uses custom internal types defined in conversion.rs
// These don't implement HasObjectMeta, so we skip that test
