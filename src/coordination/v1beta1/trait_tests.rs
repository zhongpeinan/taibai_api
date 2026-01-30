use super::*;
use crate::common::{
    ApplyDefault, FromInternal, HasObjectMeta, ResourceSchema, ToInternal, VersionedObject,
};
use crate::coordination::internal;

#[test]
fn top_level_resources_implement_required_traits() {
    fn check_versioned<T: VersionedObject + ApplyDefault>() {}
    fn check_default<T: Default>() {}
    fn check_schema<T: ResourceSchema>() {}

    check_versioned::<Lease>();
    check_versioned::<LeaseCandidate>();

    check_default::<Lease>();
    check_default::<LeaseList>();
    check_default::<LeaseCandidate>();
    check_default::<LeaseCandidateList>();

    check_schema::<Lease>();
    check_schema::<LeaseList>();
    check_schema::<LeaseCandidate>();
    check_schema::<LeaseCandidateList>();
}

#[test]
fn top_level_resources_have_conversion_traits() {
    fn check_conversion<T, I>()
    where
        T: ToInternal<I> + FromInternal<I>,
    {
    }

    check_conversion::<Lease, internal::Lease>();
    check_conversion::<LeaseList, internal::LeaseList>();
    check_conversion::<LeaseCandidate, internal::LeaseCandidate>();
    check_conversion::<LeaseCandidateList, internal::LeaseCandidateList>();
}

#[test]
fn top_level_resources_implement_prost_message() {
    fn check_prost<T: prost::Message>() {}

    check_prost::<Lease>();
    check_prost::<LeaseList>();
    check_prost::<LeaseCandidate>();
    check_prost::<LeaseCandidateList>();
}

#[test]
fn versioned_object_metadata_access() {
    let resource = Lease::default();
    let meta = resource.metadata();
    assert!(meta.name.is_none());

    let mut resource = Lease::default();
    resource.metadata_mut().name = Some("lease".to_string());
    assert_eq!(resource.metadata().name.as_deref(), Some("lease"));

    let resource = LeaseCandidate::default();
    let meta = resource.metadata();
    assert!(meta.name.is_none());

    let mut resource = LeaseCandidate::default();
    resource.metadata_mut().name = Some("candidate".to_string());
    assert_eq!(resource.metadata().name.as_deref(), Some("candidate"));
}

#[test]
fn apply_default_sets_type_meta() {
    let mut resource = Lease::default();
    resource.apply_default();
    assert_eq!(
        resource.type_meta.api_version,
        "coordination.k8s.io/v1beta1"
    );
    assert_eq!(resource.type_meta.kind, "Lease");

    let mut resource = LeaseCandidate::default();
    resource.apply_default();
    assert_eq!(
        resource.type_meta.api_version,
        "coordination.k8s.io/v1beta1"
    );
    assert_eq!(resource.type_meta.kind, "LeaseCandidate");
}

#[test]
fn internal_resources_implement_required_traits() {
    fn check<T: HasObjectMeta>() {}

    check::<internal::Lease>();
    check::<internal::LeaseCandidate>();
}
