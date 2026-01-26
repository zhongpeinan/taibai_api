use super::*;
use crate::authentication::internal;
use crate::common::{
    ApplyDefault, FromInternal, HasObjectMeta, ResourceSchema, ToInternal, VersionedObject,
};

#[test]
fn top_level_resources_implement_required_traits() {
    fn check_versioned<T: VersionedObject + ApplyDefault>() {}
    fn check_default<T: Default>() {}
    fn check_schema<T: ResourceSchema>() {}

    check_versioned::<TokenReview>();
    check_versioned::<TokenRequest>();
    check_versioned::<SelfSubjectReview>();

    check_default::<TokenReview>();
    check_default::<TokenRequest>();
    check_default::<SelfSubjectReview>();

    check_schema::<TokenReview>();
    check_schema::<TokenRequest>();
    check_schema::<SelfSubjectReview>();
}

#[test]
fn top_level_resources_have_conversion_traits() {
    fn check_conversion<T, I>()
    where
        T: ToInternal<I> + FromInternal<I>,
    {
    }

    check_conversion::<TokenReview, internal::TokenReview>();
    check_conversion::<TokenRequest, internal::TokenRequest>();
    check_conversion::<SelfSubjectReview, internal::SelfSubjectReview>();
}

#[test]
fn top_level_resources_implement_prost_message() {
    fn check_prost<T: prost::Message>() {}

    check_prost::<TokenReview>();
    check_prost::<TokenRequest>();
    check_prost::<SelfSubjectReview>();
}

#[test]
fn versioned_object_metadata_access() {
    let resource = TokenReview::default();
    let meta = resource.metadata();
    assert!(meta.name.is_none());

    let mut resource = TokenReview::default();
    resource.metadata_mut().name = Some("token-review".to_string());
    assert_eq!(resource.metadata().name.as_deref(), Some("token-review"));
}

#[test]
fn apply_default_sets_type_meta() {
    let mut resource = TokenReview::default();
    resource.apply_default();
    assert_eq!(resource.type_meta.api_version, "authentication.k8s.io/v1");
    assert_eq!(resource.type_meta.kind, "TokenReview");
}

#[test]
fn internal_resources_implement_required_traits() {
    fn check<T: HasObjectMeta>() {}

    check::<internal::TokenReview>();
    check::<internal::TokenRequest>();
    check::<internal::SelfSubjectReview>();
}
