use crate::admissionregistration::internal::ValidatingWebhookConfiguration;
use crate::common::validation::{ErrorList, Path};

use super::validate_webhook_configuration;

pub fn validate_validating_webhook_configuration(
    obj: &ValidatingWebhookConfiguration,
) -> ErrorList {
    validate_webhook_configuration(&obj.metadata, &obj.webhooks, &Path::new("webhooks"), true)
}
