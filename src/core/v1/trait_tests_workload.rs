use super::*;
use crate::common::{ApplyDefault, FromInternal, ResourceSchema, ToInternal, VersionedObject};
use crate::core::internal;

#[test]
fn top_level_resources_implement_required_traits() {
    fn check_versioned<T: VersionedObject + ApplyDefault>() {}
    fn check_default<T: Default>() {}

    check_versioned::<Pod>();
    check_versioned::<ReplicationController>();
    check_versioned::<PodTemplate>();
    check_versioned::<PodStatusResult>();

    check_default::<Pod>();
    check_default::<ReplicationController>();
    check_default::<PodTemplate>();
    check_default::<PodStatusResult>();
}

#[test]
fn top_level_resources_have_conversion_traits() {
    fn check_conversion<T, I>()
    where
        T: ToInternal<I> + FromInternal<I>,
    {
    }

    check_conversion::<Pod, internal::Pod>();
    check_conversion::<ReplicationController, internal::ReplicationController>();
    check_conversion::<PodTemplate, internal::PodTemplate>();
    check_conversion::<PodStatusResult, internal::PodStatusResult>();
}

#[test]
fn top_level_resources_implement_prost_message() {
    fn check_prost<T: prost::Message>() {}

    check_prost::<Pod>();
    check_prost::<PodList>();
    check_prost::<ReplicationController>();
    check_prost::<ReplicationControllerList>();
    check_prost::<PodTemplate>();
    check_prost::<PodTemplateList>();
    check_prost::<PodStatusResult>();
}

#[test]
fn versioned_object_metadata_access() {
    let resource = Pod::default();
    let meta = resource.metadata();
    assert!(meta.name.is_none());

    let mut resource = Pod::default();
    resource.metadata_mut().name = Some("test".to_string());
    assert_eq!(resource.metadata().name.as_deref(), Some("test"));
}

#[test]
fn apply_default_sets_type_meta() {
    let mut resource = Pod::default();
    resource.apply_default();
    let group = <Pod as ResourceSchema>::group_static();
    let version = <Pod as ResourceSchema>::version_static();
    let expected_api_version = if group.is_empty() {
        version.to_string()
    } else {
        format!("{}/{}", group, version)
    };
    assert_eq!(resource.type_meta.api_version, expected_api_version);
    assert_eq!(
        resource.type_meta.kind,
        <Pod as ResourceSchema>::kind_static()
    );
}
