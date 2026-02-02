//! AdmissionRegistration v1 type registrations for the test harness.

use crate::harness::helpers::register_type;
use crate::harness::registry::Registry;

pub fn register(registry: &mut Registry) {
    // ---- admissionregistration.k8s.io/v1/MutatingWebhookConfiguration ----
    register_type::<
        crate::admissionregistration::v1::MutatingWebhookConfiguration,
        crate::admissionregistration::internal::MutatingWebhookConfiguration,
        _,
    >(
        registry,
        "admissionregistration.k8s.io/v1/MutatingWebhookConfiguration",
        crate::admissionregistration::v1::validation::validate_mutating_webhook_configuration,
    );

    // ---- admissionregistration.k8s.io/v1/ValidatingWebhookConfiguration ----
    register_type::<
        crate::admissionregistration::v1::ValidatingWebhookConfiguration,
        crate::admissionregistration::internal::ValidatingWebhookConfiguration,
        _,
    >(
        registry,
        "admissionregistration.k8s.io/v1/ValidatingWebhookConfiguration",
        crate::admissionregistration::v1::validation::validate_validating_webhook_configuration,
    );
}
