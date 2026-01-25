use super::*;
use crate::autoscaling::internal;
use crate::common::{ApplyDefault, FromInternal, HasObjectMeta, ToInternal, VersionedObject};

#[test]
fn top_level_resources_implement_required_traits() {
    fn check_versioned<T: VersionedObject + ApplyDefault>() {}

    check_versioned::<HorizontalPodAutoscaler>();
}

#[test]
fn top_level_resources_have_conversion_traits() {
    fn check_conversion<T, I>()
    where
        T: ToInternal<I> + FromInternal<I>,
    {
    }

    check_conversion::<HorizontalPodAutoscaler, internal::HorizontalPodAutoscaler>();
    check_conversion::<HorizontalPodAutoscalerList, internal::HorizontalPodAutoscalerList>();
}

#[test]
fn top_level_resources_implement_prost_message() {
    fn check_prost<T: prost::Message>() {}

    check_prost::<HorizontalPodAutoscaler>();
    check_prost::<HorizontalPodAutoscalerList>();
}

#[test]
fn versioned_object_metadata_access() {
    let resource = HorizontalPodAutoscaler::default();
    let meta = resource.metadata();
    assert!(meta.name.is_none());

    let mut resource = HorizontalPodAutoscaler::default();
    resource.metadata_mut().name = Some("autoscaler".to_string());
    assert_eq!(resource.metadata().name.as_deref(), Some("autoscaler"));
}

#[test]
fn apply_default_sets_type_meta() {
    let mut resource = HorizontalPodAutoscaler::default();
    resource.apply_default();
    assert_eq!(resource.type_meta.api_version, "autoscaling/v2beta1");
    assert_eq!(resource.type_meta.kind, "HorizontalPodAutoscaler");
}

#[test]
fn internal_resources_implement_required_traits() {
    fn check<T: HasObjectMeta>() {}

    check::<internal::HorizontalPodAutoscaler>();
}
