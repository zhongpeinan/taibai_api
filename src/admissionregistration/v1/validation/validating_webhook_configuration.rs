use crate::admissionregistration::internal::validation as internal_validation;
use crate::admissionregistration::v1::ValidatingWebhookConfiguration;
use crate::common::validation::ErrorList;
use crate::common::ToInternal;

pub fn validate_validating_webhook_configuration(
    obj: &ValidatingWebhookConfiguration,
) -> ErrorList {
    internal_validation::validate_validating_webhook_configuration(&obj.clone().to_internal())
}
