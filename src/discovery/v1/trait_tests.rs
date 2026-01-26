use super::*;
use crate::common::{
    ApplyDefault, FromInternal, HasObjectMeta, ResourceSchema, ToInternal, VersionedObject,
};
use crate::discovery::internal;

#[test]
fn top_level_resources_implement_required_traits() {
    fn check_versioned<T: VersionedObject + ApplyDefault>() {}
    fn check_default<T: Default>() {}
    fn check_schema<T: ResourceSchema>() {}

    check_versioned::<EndpointSlice>();

    check_default::<EndpointSlice>();
    check_default::<EndpointSliceList>();

    check_schema::<EndpointSlice>();
    check_schema::<EndpointSliceList>();
}

#[test]
fn top_level_resources_have_conversion_traits() {
    fn check_conversion<T, I>()
    where
        T: ToInternal<I> + FromInternal<I>,
    {
    }

    check_conversion::<EndpointSlice, internal::EndpointSlice>();
    check_conversion::<EndpointSliceList, internal::EndpointSliceList>();
}

#[test]
fn top_level_resources_implement_prost_message() {
    fn check_prost<T: prost::Message>() {}

    check_prost::<EndpointSlice>();
    check_prost::<EndpointSliceList>();
}

#[test]
fn versioned_object_metadata_access() {
    let resource = EndpointSlice::default();
    let meta = resource.metadata();
    assert!(meta.name.is_none());

    let mut resource = EndpointSlice::default();
    resource.metadata_mut().name = Some("endpoint-slice".to_string());
    assert_eq!(resource.metadata().name.as_deref(), Some("endpoint-slice"));
}

#[test]
fn apply_default_sets_type_meta() {
    let mut resource = EndpointSlice::default();
    resource.apply_default();
    assert_eq!(resource.type_meta.api_version, "discovery.k8s.io/v1");
    assert_eq!(resource.type_meta.kind, "EndpointSlice");
}

#[test]
fn internal_resources_implement_required_traits() {
    fn check<T: HasObjectMeta>() {}

    check::<internal::EndpointSlice>();
}
