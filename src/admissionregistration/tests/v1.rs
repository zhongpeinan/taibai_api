//! AdmissionRegistration v1 trait tests

use crate::admissionregistration::v1::{
    MutatingWebhookConfiguration, MutatingWebhookConfigurationList, ValidatingAdmissionPolicy,
    ValidatingAdmissionPolicyBinding, ValidatingAdmissionPolicyBindingList,
    ValidatingAdmissionPolicyList, ValidatingWebhookConfiguration,
    ValidatingWebhookConfigurationList,
};
use crate::{generate_internal_object_meta_tests, generate_trait_tests};
use crate::common::{ApplyDefault, ResourceSchema, VersionedObject};

mod internal {
    pub use crate::admissionregistration::internal::{
        MutatingAdmissionPolicy, MutatingAdmissionPolicyBinding, MutatingWebhookConfiguration,
        MutatingWebhookConfigurationList, ValidatingAdmissionPolicy,
        ValidatingAdmissionPolicyBinding, ValidatingAdmissionPolicyBindingList,
        ValidatingAdmissionPolicyList, ValidatingWebhookConfiguration,
        ValidatingWebhookConfigurationList,
    };
}

generate_trait_tests!(
    api_version: "admissionregistration.k8s.io/v1",
    resources: [
        ValidatingAdmissionPolicy,
        ValidatingAdmissionPolicyBinding,
        ValidatingWebhookConfiguration,
        MutatingWebhookConfiguration
    ],
    list_resources: [
        ValidatingAdmissionPolicyList,
        ValidatingAdmissionPolicyBindingList,
        ValidatingWebhookConfigurationList,
        MutatingWebhookConfigurationList
    ],
    internal_resources: [
        internal::ValidatingAdmissionPolicy,
        internal::ValidatingAdmissionPolicyBinding,
        internal::ValidatingWebhookConfiguration,
        internal::MutatingWebhookConfiguration
    ],
    internal_list_resources: [
        internal::ValidatingAdmissionPolicyList,
        internal::ValidatingAdmissionPolicyBindingList,
        internal::ValidatingWebhookConfigurationList,
        internal::MutatingWebhookConfigurationList
    ]
);

generate_internal_object_meta_tests!(
    resources: [
        internal::MutatingAdmissionPolicy,
        internal::MutatingAdmissionPolicyBinding,
        internal::ValidatingAdmissionPolicy,
        internal::ValidatingAdmissionPolicyBinding,
        internal::ValidatingWebhookConfiguration,
        internal::MutatingWebhookConfiguration
    ]
);

#[test]
fn resource_schema_returns_correct_values() {
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

