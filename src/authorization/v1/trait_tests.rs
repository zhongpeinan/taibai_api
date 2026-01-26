use super::*;
use crate::authorization::internal;
use crate::common::{
    ApplyDefault, FromInternal, HasObjectMeta, ResourceSchema, ToInternal, VersionedObject,
};

#[test]
fn top_level_resources_implement_required_traits() {
    fn check_versioned<T: VersionedObject + ApplyDefault>() {}
    fn check_default<T: Default>() {}
    fn check_schema<T: ResourceSchema>() {}

    check_versioned::<SubjectAccessReview>();
    check_versioned::<SelfSubjectAccessReview>();
    check_versioned::<LocalSubjectAccessReview>();
    check_versioned::<SelfSubjectRulesReview>();

    check_default::<SubjectAccessReview>();
    check_default::<SelfSubjectAccessReview>();
    check_default::<LocalSubjectAccessReview>();
    check_default::<SelfSubjectRulesReview>();

    check_schema::<SubjectAccessReview>();
    check_schema::<SelfSubjectAccessReview>();
    check_schema::<LocalSubjectAccessReview>();
    check_schema::<SelfSubjectRulesReview>();
}

#[test]
fn top_level_resources_have_conversion_traits() {
    fn check_conversion<T, I>()
    where
        T: ToInternal<I> + FromInternal<I>,
    {
    }

    check_conversion::<SubjectAccessReview, internal::SubjectAccessReview>();
    check_conversion::<SelfSubjectAccessReview, internal::SelfSubjectAccessReview>();
    check_conversion::<LocalSubjectAccessReview, internal::LocalSubjectAccessReview>();
    check_conversion::<SelfSubjectRulesReview, internal::SelfSubjectRulesReview>();
}

#[test]
fn top_level_resources_implement_prost_message() {
    fn check_prost<T: prost::Message>() {}

    check_prost::<SubjectAccessReview>();
    check_prost::<SelfSubjectAccessReview>();
    check_prost::<LocalSubjectAccessReview>();
    check_prost::<SelfSubjectRulesReview>();
}

#[test]
fn versioned_object_metadata_access() {
    let resource = SubjectAccessReview::default();
    let meta = resource.metadata();
    assert!(meta.name.is_none());

    let mut resource = SubjectAccessReview::default();
    resource.metadata_mut().name = Some("subject-access-review".to_string());
    assert_eq!(
        resource.metadata().name.as_deref(),
        Some("subject-access-review")
    );
}

#[test]
fn apply_default_sets_type_meta() {
    let mut resource = SubjectAccessReview::default();
    resource.apply_default();
    assert_eq!(resource.type_meta.api_version, "authorization.k8s.io/v1");
    assert_eq!(resource.type_meta.kind, "SubjectAccessReview");
}

#[test]
fn internal_resources_implement_required_traits() {
    fn check<T: HasObjectMeta>() {}

    check::<internal::SubjectAccessReview>();
    check::<internal::SelfSubjectAccessReview>();
    check::<internal::LocalSubjectAccessReview>();
    check::<internal::SelfSubjectRulesReview>();
}
