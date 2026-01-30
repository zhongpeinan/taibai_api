use super::*;
use crate::apiregistration::internal;
use crate::common::{
    ApplyDefault, FromInternal, HasObjectMeta, HasTypeMeta, ResourceSchema, ToInternal,
    VersionedObject,
};

#[test]
fn top_level_resources_implement_required_traits() {
    fn check_versioned<T: VersionedObject + ApplyDefault + Default>() {}
    fn check_list<T: ApplyDefault + Default + HasTypeMeta + ResourceSchema>() {}
    fn check_has_type_meta<T: HasTypeMeta>() {}
    fn check_schema<T: ResourceSchema>() {}

    check_versioned::<APIService>();
    check_list::<APIServiceList>();
    check_has_type_meta::<APIService>();
    check_has_type_meta::<APIServiceList>();
    check_schema::<APIService>();
    check_schema::<APIServiceList>();
}

#[test]
fn top_level_resources_have_conversion_traits() {
    fn check_conversion<T, I>()
    where
        T: ToInternal<I> + FromInternal<I>,
    {
    }

    check_conversion::<APIService, internal::APIService>();
    check_conversion::<APIServiceList, internal::APIServiceList>();
}

#[test]
fn top_level_resources_implement_prost_message() {
    fn check_prost<T: prost::Message>() {}

    check_prost::<APIService>();
    check_prost::<APIServiceList>();
}

#[test]
fn versioned_object_metadata_access() {
    let resource = APIService::default();
    let meta = resource.metadata();
    assert!(meta.name.is_none());

    let mut resource = APIService::default();
    resource.metadata_mut().name = Some("v1.apps".to_string());
    assert_eq!(resource.metadata().name.as_deref(), Some("v1.apps"));
}

#[test]
fn apply_default_sets_type_meta() {
    let mut resource = APIService::default();
    resource.apply_default();
    assert_eq!(resource.type_meta.api_version, "apiregistration.k8s.io/v1");
    assert_eq!(resource.type_meta.kind, "APIService");
}

#[test]
fn internal_resources_implement_required_traits() {
    fn check<T: HasObjectMeta>() {}

    check::<internal::APIService>();
}
