use super::*;
use crate::common::{ApplyDefault, FromInternal, ResourceSchema, ToInternal, VersionedObject};
use crate::core::internal;

#[test]
fn top_level_resources_implement_required_traits() {
    fn check_versioned<T: VersionedObject + ApplyDefault>() {}
    fn check_default<T: Default>() {}

    check_versioned::<ConfigMap>();
    check_versioned::<Secret>();
    check_versioned::<ServiceAccount>();

    check_default::<ConfigMap>();
    check_default::<Secret>();
    check_default::<ServiceAccount>();
}

#[test]
fn top_level_resources_have_conversion_traits() {
    fn check_conversion<T, I>()
    where
        T: ToInternal<I> + FromInternal<I>,
    {
    }

    check_conversion::<ConfigMap, internal::ConfigMap>();
    check_conversion::<Secret, internal::Secret>();
    check_conversion::<ServiceAccount, internal::ServiceAccount>();
}

#[test]
fn top_level_resources_implement_prost_message() {
    fn check_prost<T: prost::Message>() {}

    check_prost::<ConfigMap>();
    check_prost::<ConfigMapList>();
    check_prost::<Secret>();
    check_prost::<SecretList>();
    check_prost::<ServiceAccount>();
    check_prost::<ServiceAccountList>();
}

#[test]
fn versioned_object_metadata_access() {
    let resource = ConfigMap::default();
    let meta = resource.metadata();
    assert!(meta.name.is_none());

    let mut resource = ConfigMap::default();
    resource.metadata_mut().name = Some("test".to_string());
    assert_eq!(resource.metadata().name.as_deref(), Some("test"));
}

#[test]
fn apply_default_sets_type_meta() {
    let mut resource = ConfigMap::default();
    resource.apply_default();
    let group = <ConfigMap as ResourceSchema>::group_static();
    let version = <ConfigMap as ResourceSchema>::version_static();
    let expected_api_version = if group.is_empty() {
        version.to_string()
    } else {
        format!("{}/{}", group, version)
    };
    assert_eq!(resource.type_meta.api_version, expected_api_version);
    assert_eq!(
        resource.type_meta.kind,
        <ConfigMap as ResourceSchema>::kind_static()
    );
}
