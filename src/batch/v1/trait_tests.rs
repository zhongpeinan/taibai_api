use super::*;
use crate::batch::internal;
use crate::common::{
    ApplyDefault, FromInternal, HasObjectMeta, ResourceSchema, ToInternal, VersionedObject,
};

#[test]
fn top_level_resources_implement_required_traits() {
    fn check_versioned<T: VersionedObject + ApplyDefault>() {}
    fn check_default<T: Default>() {}
    fn check_schema<T: ResourceSchema>() {}

    check_versioned::<Job>();
    check_versioned::<CronJob>();

    check_default::<Job>();
    check_default::<JobList>();
    check_default::<CronJob>();
    check_default::<CronJobList>();

    check_schema::<Job>();
    check_schema::<JobList>();
    check_schema::<CronJob>();
    check_schema::<CronJobList>();
}

#[test]
fn top_level_resources_have_conversion_traits() {
    fn check_conversion<T, I>()
    where
        T: ToInternal<I> + FromInternal<I>,
    {
    }

    check_conversion::<Job, internal::Job>();
    check_conversion::<JobList, internal::JobList>();
    check_conversion::<JobTemplateSpec, internal::JobTemplateSpec>();
    check_conversion::<CronJob, internal::CronJob>();
    check_conversion::<CronJobList, internal::CronJobList>();
}

#[test]
fn top_level_resources_implement_prost_message() {
    fn check_prost<T: prost::Message>() {}

    check_prost::<Job>();
    check_prost::<JobList>();
    check_prost::<CronJob>();
    check_prost::<CronJobList>();
}

#[test]
fn versioned_object_metadata_access() {
    let resource = Job::default();
    let meta = resource.metadata();
    assert!(meta.name.is_none());

    let mut resource = Job::default();
    resource.metadata_mut().name = Some("job".to_string());
    assert_eq!(resource.metadata().name.as_deref(), Some("job"));
}

#[test]
fn apply_default_sets_type_meta() {
    let mut resource = Job::default();
    resource.apply_default();
    assert_eq!(resource.type_meta.api_version, "batch/v1");
    assert_eq!(resource.type_meta.kind, "Job");
}

#[test]
fn internal_resources_implement_required_traits() {
    fn check<T: HasObjectMeta>() {}

    check::<internal::Job>();
    check::<internal::JobTemplateSpec>();
    check::<internal::CronJob>();
}
