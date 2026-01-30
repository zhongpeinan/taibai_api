//! Trait implementation tests for testapigroup/v1

use super::*;
use crate::common::{
    ApplyDefault, FromInternal, HasObjectMeta, HasTypeMeta, ResourceSchema, ToInternal,
    VersionedObject,
};
use crate::testapigroup::internal;

/// Compile-time check: verify all top-level resources implement required traits
#[test]
fn top_level_resources_implement_required_traits() {
    fn check_versioned<T: VersionedObject + ApplyDefault>() {}
    fn check_default<T: Default>() {}
    fn check_schema<T: ResourceSchema>() {}
    fn check_has_type_meta<T: HasTypeMeta>() {}

    check_versioned::<Carp>();
    check_default::<Carp>();
    check_schema::<Carp>();
    check_has_type_meta::<Carp>();

    check_default::<CarpList>();
    check_schema::<CarpList>();
    check_has_type_meta::<CarpList>();
}

/// Compile-time check: verify conversion traits are implemented
#[test]
fn top_level_resources_have_conversion_traits() {
    fn check_conversion<T, I>()
    where
        T: ToInternal<I> + FromInternal<I>,
    {
    }

    check_conversion::<Carp, internal::Carp>();
    check_conversion::<CarpList, internal::CarpList>();
}

/// Compile-time check: verify prost::Message trait is implemented
#[test]
fn top_level_resources_implement_prost_message() {
    fn check_prost<T: prost::Message>() {}

    check_prost::<Carp>();
    check_prost::<CarpList>();
}

/// Runtime check: VersionedObject trait provides correct metadata access
#[test]
fn versioned_object_metadata_access() {
    let resource = Carp::default();
    let meta = resource.metadata();
    assert!(meta.name.is_none());

    let mut resource = Carp::default();
    resource.metadata_mut().name = Some("test".to_string());
    assert_eq!(resource.metadata().name.as_deref(), Some("test"));
}

/// Runtime check: ApplyDefault correctly sets TypeMeta
#[test]
fn apply_default_sets_type_meta() {
    let mut resource = Carp::default();
    resource.apply_default();
    assert_eq!(
        resource.type_meta.api_version,
        "testapigroup.apimachinery.k8s.io/v1"
    );
    assert_eq!(resource.type_meta.kind, "Carp");

    let mut list = CarpList::default();
    list.apply_default();
    assert_eq!(
        list.type_meta.api_version,
        "testapigroup.apimachinery.k8s.io/v1"
    );
    assert_eq!(list.type_meta.kind, "CarpList");
}

/// Compile-time check: verify internal resources implement HasObjectMeta
#[test]
fn internal_resources_implement_required_traits() {
    fn check<T: HasObjectMeta>() {}

    check::<internal::Carp>();
}
