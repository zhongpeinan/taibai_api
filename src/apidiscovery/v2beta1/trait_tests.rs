use super::*;
use crate::apidiscovery::internal;
use crate::common::{
    ApplyDefault, FromInternal, HasObjectMeta, ResourceSchema, ToInternal, VersionedObject,
};

#[test]
fn top_level_resources_implement_required_traits() {
    fn check_versioned<T: VersionedObject + ApplyDefault>() {}
    fn check_default<T: Default>() {}
    fn check_schema<T: ResourceSchema>() {}

    check_versioned::<APIGroupDiscovery>();

    check_default::<APIGroupDiscovery>();
    check_default::<APIGroupDiscoveryList>();

    check_schema::<APIGroupDiscovery>();
    check_schema::<APIGroupDiscoveryList>();
}

#[test]
fn top_level_resources_have_conversion_traits() {
    fn check_conversion<T, I>()
    where
        T: ToInternal<I> + FromInternal<I>,
    {
    }

    check_conversion::<APIGroupDiscovery, internal::APIGroupDiscovery>();
    check_conversion::<APIGroupDiscoveryList, internal::APIGroupDiscoveryList>();
}

#[test]
fn top_level_resources_implement_prost_message() {
    fn check_prost<T: prost::Message>() {}

    check_prost::<APIGroupDiscovery>();
    check_prost::<APIGroupDiscoveryList>();
}

#[test]
fn versioned_object_metadata_access() {
    let resource = APIGroupDiscovery::default();
    let meta = resource.metadata();
    assert!(meta.name.is_none());

    let mut resource = APIGroupDiscovery::default();
    resource.metadata_mut().name = Some("api-group".to_string());
    assert_eq!(resource.metadata().name.as_deref(), Some("api-group"));
}

#[test]
fn apply_default_sets_type_meta() {
    let mut resource = APIGroupDiscovery::default();
    resource.apply_default();
    assert_eq!(
        resource.type_meta.api_version,
        "apidiscovery.k8s.io/v2beta1"
    );
    assert_eq!(resource.type_meta.kind, "APIGroupDiscovery");
}

#[test]
fn internal_resources_implement_required_traits() {
    fn check<T: HasObjectMeta>() {}

    check::<internal::APIGroupDiscovery>();
}
