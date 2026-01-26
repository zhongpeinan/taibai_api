use super::*;
use crate::common::{ApplyDefault, FromInternal, ResourceSchema, ToInternal, VersionedObject};
use crate::core::internal;

#[test]
fn top_level_resources_implement_required_traits() {
    fn check_versioned<T: VersionedObject + ApplyDefault>() {}
    fn check_default<T: Default>() {}

    check_versioned::<Binding>();
    check_versioned::<Event>();

    check_default::<Binding>();
    check_default::<Event>();
}

#[test]
fn top_level_resources_have_conversion_traits() {
    fn check_conversion<T, I>()
    where
        T: ToInternal<I> + FromInternal<I>,
    {
    }

    check_conversion::<Binding, internal::Binding>();
    check_conversion::<Event, internal::Event>();
}

#[test]
fn top_level_resources_implement_prost_message() {
    fn check_prost<T: prost::Message>() {}

    check_prost::<Binding>();
    check_prost::<Event>();
    check_prost::<EventList>();
}

#[test]
fn versioned_object_metadata_access() {
    let resource = Binding::default();
    let meta = resource.metadata();
    assert!(meta.name.is_none());

    let mut resource = Binding::default();
    resource.metadata_mut().name = Some("test".to_string());
    assert_eq!(resource.metadata().name.as_deref(), Some("test"));
}

#[test]
fn apply_default_sets_type_meta() {
    let mut resource = Binding::default();
    resource.apply_default();
    let group = <Binding as ResourceSchema>::group_static();
    let version = <Binding as ResourceSchema>::version_static();
    let expected_api_version = if group.is_empty() {
        version.to_string()
    } else {
        format!("{}/{}", group, version)
    };
    assert_eq!(resource.type_meta.api_version, expected_api_version);
    assert_eq!(
        resource.type_meta.kind,
        <Binding as ResourceSchema>::kind_static()
    );
}
