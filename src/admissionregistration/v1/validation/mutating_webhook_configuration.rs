use crate::admissionregistration::internal::validation as internal_validation;
use crate::admissionregistration::v1::MutatingWebhookConfiguration;
use crate::common::validation::ErrorList;
use crate::common::ToInternal;

pub fn validate_mutating_webhook_configuration(
    obj: &MutatingWebhookConfiguration,
) -> ErrorList {
    internal_validation::validate_mutating_webhook_configuration(&obj.clone().to_internal())
}
