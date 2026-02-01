//! Apps v1 type registrations for the test harness.

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
    // ---- apps/v1/Deployment ----
    registry.register(
        "apps/v1/Deployment",
        TypeHandler {
            apply_defaults: Box::new(|json| {
                let mut obj: crate::apps::v1::Deployment = parse_json(json)?;
                obj.apply_default();
                Ok(DefaultResult {
                    gvk: "apps/v1/Deployment".to_string(),
                    result: to_value(&obj)?,
                    defaults_applied: true,
                })
            }),
            convert_roundtrip: Some(Box::new(|json| {
                let obj: crate::apps::v1::Deployment = parse_json(json)?;
                let original = to_value(&obj)?;
                let internal: crate::apps::internal::Deployment = obj.to_internal();
                let roundtrip = to_value(&internal)?;
                Ok(ConversionResult {
                    gvk: "apps/v1/Deployment".to_string(),
                    original,
                    roundtrip,
                    success: true,
                })
            })),
            validate: Some(Box::new(|json| {
                let obj: crate::apps::v1::Deployment = parse_json(json)?;
                let errors = crate::apps::v1::validation::validate_deployment(&obj);
                Ok(ValidationResult {
                    gvk: "apps/v1/Deployment".to_string(),
                    valid: errors.is_empty(),
                    errors: errors_to_field_errors(&errors),
                })
            })),
        },
    );

    // ---- apps/v1/DaemonSet ----
    registry.register(
        "apps/v1/DaemonSet",
        TypeHandler {
            apply_defaults: Box::new(|json| {
                let mut obj: crate::apps::v1::DaemonSet = parse_json(json)?;
                obj.apply_default();
                Ok(DefaultResult {
                    gvk: "apps/v1/DaemonSet".to_string(),
                    result: to_value(&obj)?,
                    defaults_applied: true,
                })
            }),
            convert_roundtrip: Some(Box::new(|json| {
                let obj: crate::apps::v1::DaemonSet = parse_json(json)?;
                let original = to_value(&obj)?;
                let internal: crate::apps::internal::DaemonSet = obj.to_internal();
                let roundtrip = to_value(&internal)?;
                Ok(ConversionResult {
                    gvk: "apps/v1/DaemonSet".to_string(),
                    original,
                    roundtrip,
                    success: true,
                })
            })),
            validate: Some(Box::new(|json| {
                let obj: crate::apps::v1::DaemonSet = parse_json(json)?;
                let errors = crate::apps::v1::validation::validate_daemon_set(&obj);
                Ok(ValidationResult {
                    gvk: "apps/v1/DaemonSet".to_string(),
                    valid: errors.is_empty(),
                    errors: errors_to_field_errors(&errors),
                })
            })),
        },
    );

    // ---- apps/v1/StatefulSet ----
    registry.register(
        "apps/v1/StatefulSet",
        TypeHandler {
            apply_defaults: Box::new(|json| {
                let mut obj: crate::apps::v1::StatefulSet = parse_json(json)?;
                obj.apply_default();
                Ok(DefaultResult {
                    gvk: "apps/v1/StatefulSet".to_string(),
                    result: to_value(&obj)?,
                    defaults_applied: true,
                })
            }),
            convert_roundtrip: Some(Box::new(|json| {
                let obj: crate::apps::v1::StatefulSet = parse_json(json)?;
                let original = to_value(&obj)?;
                let internal: crate::apps::internal::StatefulSet = obj.to_internal();
                let roundtrip = to_value(&internal)?;
                Ok(ConversionResult {
                    gvk: "apps/v1/StatefulSet".to_string(),
                    original,
                    roundtrip,
                    success: true,
                })
            })),
            validate: Some(Box::new(|json| {
                let obj: crate::apps::v1::StatefulSet = parse_json(json)?;
                let errors = crate::apps::v1::validation::validate_stateful_set(&obj);
                Ok(ValidationResult {
                    gvk: "apps/v1/StatefulSet".to_string(),
                    valid: errors.is_empty(),
                    errors: errors_to_field_errors(&errors),
                })
            })),
        },
    );
}
