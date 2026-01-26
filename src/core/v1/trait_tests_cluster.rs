use super::*;
use crate::common::{ApplyDefault, FromInternal, ResourceSchema, ToInternal, VersionedObject};
use crate::core::internal;

#[test]
fn top_level_resources_implement_required_traits() {
    fn check_versioned<T: VersionedObject + ApplyDefault>() {}
    fn check_default<T: Default>() {}

    check_versioned::<Namespace>();
    check_versioned::<Node>();
    check_versioned::<ComponentStatus>();

    check_default::<Namespace>();
    check_default::<Node>();
    check_default::<ComponentStatus>();
}

#[test]
fn top_level_resources_have_conversion_traits() {
    fn check_conversion<T, I>()
    where
        T: ToInternal<I> + FromInternal<I>,
    {
    }

    check_conversion::<Namespace, internal::Namespace>();
    check_conversion::<Node, internal::Node>();
    check_conversion::<ComponentStatus, internal::ComponentStatus>();
}

#[test]
fn top_level_resources_implement_prost_message() {
    fn check_prost<T: prost::Message>() {}

    check_prost::<Namespace>();
    check_prost::<NamespaceList>();
    check_prost::<Node>();
    check_prost::<NodeList>();
    check_prost::<ComponentStatus>();
    check_prost::<ComponentStatusList>();
}

#[test]
fn versioned_object_metadata_access() {
    let resource = Namespace::default();
    let meta = resource.metadata();
    assert!(meta.name.is_none());

    let mut resource = Namespace::default();
    resource.metadata_mut().name = Some("test".to_string());
    assert_eq!(resource.metadata().name.as_deref(), Some("test"));
}

#[test]
fn apply_default_sets_type_meta() {
    let mut resource = Namespace::default();
    resource.apply_default();
    let group = <Namespace as ResourceSchema>::group_static();
    let version = <Namespace as ResourceSchema>::version_static();
    let expected_api_version = if group.is_empty() {
        version.to_string()
    } else {
        format!("{}/{}", group, version)
    };
    assert_eq!(resource.type_meta.api_version, expected_api_version);
    assert_eq!(
        resource.type_meta.kind,
        <Namespace as ResourceSchema>::kind_static()
    );
}
