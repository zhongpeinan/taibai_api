use super::*;
use crate::admissionregistration::internal;
use crate::common::{
    ApplyDefault, FromInternal, HasObjectMeta, ResourceSchema, ToInternal, VersionedObject,
};

#[test]
fn top_level_resources_implement_required_traits() {
    fn check_versioned<T: VersionedObject + ApplyDefault>() {}
    fn check_default<T: Default>() {}
    fn check_schema<T: ResourceSchema>() {}

    check_versioned::<MutatingAdmissionPolicy>();
    check_versioned::<MutatingAdmissionPolicyBinding>();

    check_default::<MutatingAdmissionPolicy>();
    check_default::<MutatingAdmissionPolicyList>();
    check_default::<MutatingAdmissionPolicyBinding>();
    check_default::<MutatingAdmissionPolicyBindingList>();

    check_schema::<MutatingAdmissionPolicy>();
    check_schema::<MutatingAdmissionPolicyList>();
    check_schema::<MutatingAdmissionPolicyBinding>();
    check_schema::<MutatingAdmissionPolicyBindingList>();
}

#[test]
fn top_level_resources_have_conversion_traits() {
    fn check_conversion<T, I>()
    where
        T: ToInternal<I> + FromInternal<I>,
    {
    }

    check_conversion::<MutatingAdmissionPolicy, internal::MutatingAdmissionPolicy>();
    check_conversion::<MutatingAdmissionPolicyList, internal::MutatingAdmissionPolicyList>();
    check_conversion::<MutatingAdmissionPolicyBinding, internal::MutatingAdmissionPolicyBinding>();
    check_conversion::<
        MutatingAdmissionPolicyBindingList,
        internal::MutatingAdmissionPolicyBindingList,
    >();
}

#[test]
fn top_level_resources_implement_prost_message() {
    fn check_prost<T: prost::Message>() {}

    check_prost::<MutatingAdmissionPolicy>();
    check_prost::<MutatingAdmissionPolicyList>();
    check_prost::<MutatingAdmissionPolicyBinding>();
    check_prost::<MutatingAdmissionPolicyBindingList>();
}

#[test]
fn versioned_object_metadata_access() {
    let resource = MutatingAdmissionPolicy::default();
    let meta = resource.metadata();
    assert!(meta.name.is_none());

    let mut resource = MutatingAdmissionPolicy::default();
    resource.metadata_mut().name = Some("policy".to_string());
    assert_eq!(resource.metadata().name.as_deref(), Some("policy"));
}

#[test]
fn apply_default_sets_type_meta() {
    let mut resource = MutatingAdmissionPolicy::default();
    resource.apply_default();
    assert_eq!(
        resource.type_meta.api_version,
        "admissionregistration.k8s.io/v1beta1"
    );
    assert_eq!(resource.type_meta.kind, "MutatingAdmissionPolicy");
}

#[test]
fn internal_resources_implement_required_traits() {
    fn check<T: HasObjectMeta>() {}

    check::<internal::MutatingAdmissionPolicy>();
    check::<internal::MutatingAdmissionPolicyBinding>();
}
