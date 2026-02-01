//! Batch v1 type registrations for the test harness.

use crate::common::{ApplyDefault, ToInternal};
use crate::harness::error::{FieldError, HarnessError};
use crate::harness::registry::{Registry, TypeHandler};
use crate::harness::{ConversionResult, DefaultResult, ValidationResult};

fn parse_json<T: serde::de::DeserializeOwned>(json: &str) -> Result<T, HarnessError> {
    serde_json::from_str(json).map_err(|e| HarnessError::JsonParse {
        message: e.to_string(),
    })
}

fn to_value<T: serde::Serialize>(obj: &T) -> Result<serde_json::Value, HarnessError> {
    serde_json::to_value(obj).map_err(|e| HarnessError::Internal {
        message: e.to_string(),
    })
}

fn errors_to_field_errors(errors: &crate::common::validation::ErrorList) -> Vec<FieldError> {
    errors.errors.iter().map(FieldError::from).collect()
}

pub fn register(registry: &mut Registry) {
    // ---- batch/v1/Job ----
    registry.register(
        "batch/v1/Job",
        TypeHandler {
            apply_defaults: Box::new(|json| {
                let mut obj: crate::batch::v1::Job = parse_json(json)?;
                obj.apply_default();
                Ok(DefaultResult {
                    gvk: "batch/v1/Job".to_string(),
                    result: to_value(&obj)?,
                    defaults_applied: true,
                })
            }),
            convert_roundtrip: Some(Box::new(|json| {
                let obj: crate::batch::v1::Job = parse_json(json)?;
                let original = to_value(&obj)?;
                let internal: crate::batch::internal::Job = obj.to_internal();
                let roundtrip = to_value(&internal)?;
                Ok(ConversionResult {
                    gvk: "batch/v1/Job".to_string(),
                    original,
                    roundtrip,
                    success: true,
                })
            })),
            validate: Some(Box::new(|json| {
                let obj: crate::batch::v1::Job = parse_json(json)?;
                let errors = crate::batch::v1::validation::validate_job(&obj);
                Ok(ValidationResult {
                    gvk: "batch/v1/Job".to_string(),
                    valid: errors.is_empty(),
                    errors: errors_to_field_errors(&errors),
                })
            })),
        },
    );

    // ---- batch/v1/CronJob ----
    registry.register(
        "batch/v1/CronJob",
        TypeHandler {
            apply_defaults: Box::new(|json| {
                let mut obj: crate::batch::v1::CronJob = parse_json(json)?;
                obj.apply_default();
                Ok(DefaultResult {
                    gvk: "batch/v1/CronJob".to_string(),
                    result: to_value(&obj)?,
                    defaults_applied: true,
                })
            }),
            convert_roundtrip: Some(Box::new(|json| {
                let obj: crate::batch::v1::CronJob = parse_json(json)?;
                let original = to_value(&obj)?;
                let internal: crate::batch::internal::CronJob = obj.to_internal();
                let roundtrip = to_value(&internal)?;
                Ok(ConversionResult {
                    gvk: "batch/v1/CronJob".to_string(),
                    original,
                    roundtrip,
                    success: true,
                })
            })),
            validate: Some(Box::new(|json| {
                let obj: crate::batch::v1::CronJob = parse_json(json)?;
                let errors = crate::batch::v1::validation::validate_cron_job(&obj);
                Ok(ValidationResult {
                    gvk: "batch/v1/CronJob".to_string(),
                    valid: errors.is_empty(),
                    errors: errors_to_field_errors(&errors),
                })
            })),
        },
    );
}
