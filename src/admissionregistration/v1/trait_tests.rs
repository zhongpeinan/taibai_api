use super::*;
use crate::admissionregistration::internal;
use crate::common::{
    ApplyDefault, FromInternal, HasObjectMeta, ResourceSchema, ToInternal, VersionedObject,
};

#[test]
fn top_level_resources_implement_resource_schema() {
    fn check_resource_schema<T: ResourceSchema>() {}

    check_resource_schema::<ValidatingAdmissionPolicy>();
    check_resource_schema::<ValidatingAdmissionPolicyList>();
    check_resource_schema::<ValidatingAdmissionPolicyBinding>();
    check_resource_schema::<ValidatingAdmissionPolicyBindingList>();
    check_resource_schema::<ValidatingWebhookConfiguration>();
    check_resource_schema::<ValidatingWebhookConfigurationList>();
    check_resource_schema::<MutatingWebhookConfiguration>();
    check_resource_schema::<MutatingWebhookConfigurationList>();
}

#[test]
fn resource_schema_returns_correct_values() {
    // Test ValidatingAdmissionPolicy
    assert_eq!(
        ValidatingAdmissionPolicy::group_static(),
        "admissionregistration.k8s.io"
    );
    assert_eq!(ValidatingAdmissionPolicy::version_static(), "v1");
    assert_eq!(
        ValidatingAdmissionPolicy::kind_static(),
        "ValidatingAdmissionPolicy"
    );
    assert_eq!(
        ValidatingAdmissionPolicy::resource_static(),
        "validatingadmissionpolicies"
    );

    // Test ValidatingAdmissionPolicyBinding
    assert_eq!(
        ValidatingAdmissionPolicyBinding::group_static(),
        "admissionregistration.k8s.io"
    );
    assert_eq!(ValidatingAdmissionPolicyBinding::version_static(), "v1");
    assert_eq!(
        ValidatingAdmissionPolicyBinding::kind_static(),
        "ValidatingAdmissionPolicyBinding"
    );
    assert_eq!(
        ValidatingAdmissionPolicyBinding::resource_static(),
        "validatingadmissionpolicybindings"
    );
}

#[test]
fn top_level_resources_implement_required_traits() {
    fn check_versioned<T: VersionedObject + ApplyDefault>() {}
    fn check_default<T: Default>() {}

    check_versioned::<ValidatingAdmissionPolicy>();
    check_versioned::<ValidatingAdmissionPolicyBinding>();
    check_versioned::<ValidatingWebhookConfiguration>();
    check_versioned::<MutatingWebhookConfiguration>();

    check_default::<ValidatingAdmissionPolicy>();
    check_default::<ValidatingAdmissionPolicyList>();
    check_default::<ValidatingAdmissionPolicyBinding>();
    check_default::<ValidatingAdmissionPolicyBindingList>();
    check_default::<ValidatingWebhookConfiguration>();
    check_default::<ValidatingWebhookConfigurationList>();
    check_default::<MutatingWebhookConfiguration>();
    check_default::<MutatingWebhookConfigurationList>();
}

#[test]
fn top_level_resources_have_conversion_traits() {
    fn check_conversion<T, I>()
    where
        T: ToInternal<I> + FromInternal<I>,
    {
    }

    check_conversion::<ValidatingAdmissionPolicy, internal::ValidatingAdmissionPolicy>();
    check_conversion::<ValidatingAdmissionPolicyList, internal::ValidatingAdmissionPolicyList>();
    check_conversion::<ValidatingAdmissionPolicyBinding, internal::ValidatingAdmissionPolicyBinding>(
    );
    check_conversion::<
        ValidatingAdmissionPolicyBindingList,
        internal::ValidatingAdmissionPolicyBindingList,
    >();
    check_conversion::<ValidatingWebhookConfiguration, internal::ValidatingWebhookConfiguration>();
    check_conversion::<
        ValidatingWebhookConfigurationList,
        internal::ValidatingWebhookConfigurationList,
    >();
    check_conversion::<MutatingWebhookConfiguration, internal::MutatingWebhookConfiguration>();
    check_conversion::<MutatingWebhookConfigurationList, internal::MutatingWebhookConfigurationList>(
    );
}

#[test]
fn top_level_resources_implement_prost_message() {
    fn check_prost<T: prost::Message>() {}

    check_prost::<ValidatingAdmissionPolicy>();
    check_prost::<ValidatingAdmissionPolicyList>();
    check_prost::<ValidatingAdmissionPolicyBinding>();
    check_prost::<ValidatingAdmissionPolicyBindingList>();
    check_prost::<ValidatingWebhookConfiguration>();
    check_prost::<ValidatingWebhookConfigurationList>();
    check_prost::<MutatingWebhookConfiguration>();
    check_prost::<MutatingWebhookConfigurationList>();
}

#[test]
fn versioned_object_metadata_access() {
    let resource = ValidatingAdmissionPolicy::default();
    let meta = resource.metadata();
    assert!(meta.name.is_none());

    let mut resource = ValidatingAdmissionPolicy::default();
    resource.metadata_mut().name = Some("policy".to_string());
    assert_eq!(resource.metadata().name.as_deref(), Some("policy"));
}

#[test]
fn apply_default_sets_type_meta() {
    let mut resource = ValidatingAdmissionPolicy::default();
    resource.apply_default();
    assert_eq!(
        resource.type_meta.api_version,
        "admissionregistration.k8s.io/v1"
    );
    assert_eq!(resource.type_meta.kind, "ValidatingAdmissionPolicy");
}

#[test]
fn internal_resources_implement_required_traits() {
    fn check<T: HasObjectMeta>() {}

    check::<internal::MutatingAdmissionPolicy>();
    check::<internal::MutatingAdmissionPolicyBinding>();
    check::<internal::ValidatingAdmissionPolicy>();
    check::<internal::ValidatingAdmissionPolicyBinding>();
    check::<internal::ValidatingWebhookConfiguration>();
    check::<internal::MutatingWebhookConfiguration>();
}
