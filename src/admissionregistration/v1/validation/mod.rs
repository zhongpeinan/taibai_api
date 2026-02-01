//! Validation for Kubernetes AdmissionRegistration v1 API types

mod mutating_webhook_configuration;
mod validating_webhook_configuration;

pub use mutating_webhook_configuration::validate_mutating_webhook_configuration;
pub use validating_webhook_configuration::validate_validating_webhook_configuration;
