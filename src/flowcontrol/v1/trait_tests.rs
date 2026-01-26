//! Trait implementation tests for flowcontrol/v1
//!
//! This module verifies that all top-level resources implement required traits.

use super::*;
use crate::common::{
    ApplyDefault, FromInternal, HasObjectMeta, ResourceSchema, ToInternal, VersionedObject,
};
use crate::flowcontrol::internal;

/// Compile-time check: verify all top-level resources implement required traits
#[test]
fn top_level_resources_implement_required_traits() {
    // Helper functions that enforce trait bounds (compile-time only)
    fn check_versioned<T: VersionedObject + ApplyDefault>() {}
    fn check_default<T: Default>() {}
    fn check_schema<T: ResourceSchema>() {}

    // Top-level resources in flowcontrol/v1
    check_versioned::<FlowSchema>();
    check_default::<FlowSchema>();
    check_schema::<FlowSchema>();
    check_default::<FlowSchemaList>();
    check_schema::<FlowSchemaList>();

    check_versioned::<PriorityLevelConfiguration>();
    check_default::<PriorityLevelConfiguration>();
    check_schema::<PriorityLevelConfiguration>();
    check_default::<PriorityLevelConfigurationList>();
    check_schema::<PriorityLevelConfigurationList>();
}

/// Compile-time check: verify prost::Message trait is implemented
#[test]
fn top_level_resources_implement_prost_message() {
    fn check_prost<T: prost::Message>() {}

    // All top-level resources implement prost::Message
    check_prost::<FlowSchema>();
    check_prost::<FlowSchemaList>();
    check_prost::<PriorityLevelConfiguration>();
    check_prost::<PriorityLevelConfigurationList>();
}

/// Runtime check: VersionedObject trait provides correct metadata access
#[test]
fn versioned_object_metadata_access() {
    // Test with FlowSchema
    let resource = FlowSchema::default();
    let meta = resource.metadata();
    assert!(meta.name.is_none());

    // Test metadata_mut auto-inserts default
    let mut resource = FlowSchema::default();
    resource.metadata_mut().name = Some("test".to_string());
    assert_eq!(resource.metadata().name.as_deref(), Some("test"));
}

/// Runtime check: ApplyDefault correctly sets TypeMeta
#[test]
fn apply_default_sets_type_meta() {
    // Test FlowSchema
    let mut flow_schema = FlowSchema::default();
    flow_schema.apply_default();
    assert_eq!(
        flow_schema.type_meta.api_version,
        "flowcontrol.apiserver.k8s.io/v1"
    );
    assert_eq!(flow_schema.type_meta.kind, "FlowSchema");

    // Test PriorityLevelConfiguration
    let mut priority_level = PriorityLevelConfiguration::default();
    priority_level.apply_default();
    assert_eq!(
        priority_level.type_meta.api_version,
        "flowcontrol.apiserver.k8s.io/v1"
    );
    assert_eq!(priority_level.type_meta.kind, "PriorityLevelConfiguration");
}

/// Compile-time check: verify conversion traits are implemented
#[test]
fn top_level_resources_have_conversion_traits() {
    fn check_conversion<T, I>()
    where
        T: ToInternal<I> + FromInternal<I>,
    {
    }

    check_conversion::<FlowSchema, internal::FlowSchema>();
    check_conversion::<FlowSchemaList, internal::FlowSchemaList>();
    check_conversion::<PriorityLevelConfiguration, internal::PriorityLevelConfiguration>();
    check_conversion::<PriorityLevelConfigurationList, internal::PriorityLevelConfigurationList>();
}

/// Compile-time check: verify internal resources implement HasObjectMeta
#[test]
fn internal_resources_implement_required_traits() {
    fn check<T: HasObjectMeta>() {}

    check::<internal::FlowSchema>();
    check::<internal::PriorityLevelConfiguration>();
    // Note: List types don't implement HasObjectMeta (they have ListMeta)
}
