use super::*;
use crate::common::{
    ApplyDefault, FromInternal, HasObjectMeta, ToInternal, TypeMeta, VersionedObject,
};
use crate::node::internal;

#[test]
fn top_level_resources_implement_required_traits() {
    fn check_versioned<T: VersionedObject + ApplyDefault>() {}

    check_versioned::<RuntimeClass>();
}

#[test]
fn top_level_resources_have_conversion_traits() {
    fn check_conversion<T, I>()
    where
        T: ToInternal<I> + FromInternal<I>,
    {
    }

    check_conversion::<RuntimeClass, internal::RuntimeClass>();
    check_conversion::<RuntimeClassList, internal::RuntimeClassList>();
}

#[test]
fn top_level_resources_implement_prost_message() {
    fn check_prost<T: prost::Message>() {}

    check_prost::<RuntimeClass>();
    check_prost::<RuntimeClassList>();
}

#[test]
fn versioned_object_metadata_access() {
    let resource = RuntimeClass {
        type_meta: TypeMeta::default(),
        metadata: None,
        handler: String::new(),
        overhead: None,
        scheduling: None,
    };
    let meta = resource.metadata();
    assert!(meta.name.is_none());

    let mut resource = RuntimeClass {
        type_meta: TypeMeta::default(),
        metadata: None,
        handler: String::new(),
        overhead: None,
        scheduling: None,
    };
    resource.metadata_mut().name = Some("runtime-class".to_string());
    assert_eq!(resource.metadata().name.as_deref(), Some("runtime-class"));
}

#[test]
fn apply_default_sets_type_meta() {
    let mut resource = RuntimeClass {
        type_meta: TypeMeta::default(),
        metadata: None,
        handler: String::new(),
        overhead: None,
        scheduling: None,
    };
    resource.apply_default();
    assert_eq!(resource.type_meta.api_version, "node.k8s.io/v1");
    assert_eq!(resource.type_meta.kind, "RuntimeClass");
}

#[test]
fn internal_resources_implement_required_traits() {
    fn check<T: HasObjectMeta>() {}

    check::<internal::RuntimeClass>();
}
