use super::*;
use crate::common::{ApplyDefault, FromInternal, HasObjectMeta, ToInternal, VersionedObject};
use crate::events::internal;

#[test]
fn top_level_resources_implement_required_traits() {
    fn check_versioned<T: VersionedObject + ApplyDefault>() {}

    check_versioned::<Event>();
}

#[test]
fn top_level_resources_have_conversion_traits() {
    fn check_conversion<T, I>()
    where
        T: ToInternal<I> + FromInternal<I>,
    {
    }

    check_conversion::<Event, internal::Event>();
    check_conversion::<EventList, internal::EventList>();
}

#[test]
fn top_level_resources_implement_prost_message() {
    fn check_prost<T: prost::Message>() {}

    check_prost::<Event>();
    check_prost::<EventList>();
}

#[test]
fn versioned_object_metadata_access() {
    let resource = Event::default();
    let meta = resource.metadata();
    assert!(meta.name.is_none());

    let mut resource = Event::default();
    resource.metadata_mut().name = Some("event".to_string());
    assert_eq!(resource.metadata().name.as_deref(), Some("event"));
}

#[test]
fn apply_default_sets_type_meta() {
    let mut resource = Event::default();
    resource.apply_default();
    assert_eq!(resource.type_meta.api_version, "events.k8s.io/v1");
    assert_eq!(resource.type_meta.kind, "Event");
}

#[test]
fn internal_resources_implement_required_traits() {
    fn check<T: HasObjectMeta>() {}

    check::<internal::Event>();
}
