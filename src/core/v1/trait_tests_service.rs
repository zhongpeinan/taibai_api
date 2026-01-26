use super::*;
use crate::common::{ApplyDefault, FromInternal, ResourceSchema, ToInternal, VersionedObject};
use crate::core::internal;

#[test]
fn top_level_resources_implement_required_traits() {
    fn check_versioned<T: VersionedObject + ApplyDefault>() {}
    fn check_default<T: Default>() {}

    check_versioned::<Service>();
    check_versioned::<Endpoints>();

    check_default::<Service>();
    check_default::<Endpoints>();
}

#[test]
fn top_level_resources_have_conversion_traits() {
    fn check_conversion<T, I>()
    where
        T: ToInternal<I> + FromInternal<I>,
    {
    }

    check_conversion::<Service, internal::Service>();
    check_conversion::<Endpoints, internal::Endpoints>();
}

#[test]
fn top_level_resources_implement_prost_message() {
    fn check_prost<T: prost::Message>() {}

    check_prost::<Service>();
    check_prost::<ServiceList>();
    check_prost::<Endpoints>();
    check_prost::<EndpointsList>();
}

#[test]
fn versioned_object_metadata_access() {
    let resource = Service::default();
    let meta = resource.metadata();
    assert!(meta.name.is_none());

    let mut resource = Service::default();
    resource.metadata_mut().name = Some("test".to_string());
    assert_eq!(resource.metadata().name.as_deref(), Some("test"));
}

#[test]
fn apply_default_sets_type_meta() {
    let mut resource = Service::default();
    resource.apply_default();
    let group = <Service as ResourceSchema>::group_static();
    let version = <Service as ResourceSchema>::version_static();
    let expected_api_version = if group.is_empty() {
        version.to_string()
    } else {
        format!("{}/{}", group, version)
    };
    assert_eq!(resource.type_meta.api_version, expected_api_version);
    assert_eq!(
        resource.type_meta.kind,
        <Service as ResourceSchema>::kind_static()
    );
}
