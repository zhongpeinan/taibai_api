use crate::admissionregistration::internal::MutatingWebhookConfiguration;
use crate::common::validation::{ErrorList, Path};

use super::validate_webhook_configuration;

pub fn validate_mutating_webhook_configuration(obj: &MutatingWebhookConfiguration) -> ErrorList {
    validate_webhook_configuration(&obj.metadata, &obj.webhooks, &Path::new("webhooks"), false)
}
