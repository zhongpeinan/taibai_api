//! Core v1 type registrations for the test harness.

use crate::common::validation::Path;
use crate::common::{ApplyDefault, FromInternal, ToInternal};
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
    // ---- core/v1/Pod ----
    registry.register(
        "core/v1/Pod",
        TypeHandler {
            apply_defaults: Box::new(|json| {
                let mut pod: crate::core::v1::Pod = parse_json(json)?;
                pod.apply_default();
                Ok(DefaultResult {
                    gvk: "core/v1/Pod".to_string(),
                    result: to_value(&pod)?,
                    defaults_applied: true,
                })
            }),
            convert_roundtrip: Some(Box::new(|json| {
                let pod: crate::core::v1::Pod = parse_json(json)?;
                let original = to_value(&pod)?;
                let internal: crate::core::internal::Pod = pod.to_internal();
                let roundtrip_pod = crate::core::v1::Pod::from_internal(internal);
                let roundtrip = to_value(&roundtrip_pod)?;
                Ok(ConversionResult {
                    gvk: "core/v1/Pod".to_string(),
                    original,
                    roundtrip,
                    success: true,
                })
            })),
            validate: Some(Box::new(|json| {
                let pod: crate::core::v1::Pod = parse_json(json)?;
                let errors = crate::core::v1::validation::pod::validate_pod(&pod);
                Ok(ValidationResult {
                    gvk: "core/v1/Pod".to_string(),
                    valid: errors.is_empty(),
                    errors: errors_to_field_errors(&errors),
                })
            })),
        },
    );

    // ---- core/v1/Service ----
    registry.register(
        "core/v1/Service",
        TypeHandler {
            apply_defaults: Box::new(|json| {
                let mut svc: crate::core::v1::service::Service = parse_json(json)?;
                svc.apply_default();
                Ok(DefaultResult {
                    gvk: "core/v1/Service".to_string(),
                    result: to_value(&svc)?,
                    defaults_applied: true,
                })
            }),
            convert_roundtrip: Some(Box::new(|json| {
                let svc: crate::core::v1::service::Service = parse_json(json)?;
                let original = to_value(&svc)?;
                let internal: crate::core::internal::service::Service = svc.to_internal();
                let roundtrip_svc = crate::core::v1::service::Service::from_internal(internal);
                let roundtrip = to_value(&roundtrip_svc)?;
                Ok(ConversionResult {
                    gvk: "core/v1/Service".to_string(),
                    original,
                    roundtrip,
                    success: true,
                })
            })),
            validate: Some(Box::new(|json| {
                let svc: crate::core::v1::service::Service = parse_json(json)?;
                let root = Path::new("");
                let errors = crate::core::v1::validation::service::validate_service(&svc, &root);
                Ok(ValidationResult {
                    gvk: "core/v1/Service".to_string(),
                    valid: errors.is_empty(),
                    errors: errors_to_field_errors(&errors),
                })
            })),
        },
    );

    // ---- core/v1/ConfigMap ----
    registry.register(
        "core/v1/ConfigMap",
        TypeHandler {
            apply_defaults: Box::new(|json| {
                let mut cm: crate::core::v1::config::ConfigMap = parse_json(json)?;
                cm.apply_default();
                Ok(DefaultResult {
                    gvk: "core/v1/ConfigMap".to_string(),
                    result: to_value(&cm)?,
                    defaults_applied: true,
                })
            }),
            convert_roundtrip: Some(Box::new(|json| {
                let cm: crate::core::v1::config::ConfigMap = parse_json(json)?;
                let original = to_value(&cm)?;
                let internal: crate::core::internal::config::ConfigMap = cm.to_internal();
                // ConfigMap doesn't have FromInternal yet, so roundtrip returns the
                // serialized internal form
                let roundtrip =
                    serde_json::to_value(&internal).map_err(|e| HarnessError::Internal {
                        message: e.to_string(),
                    })?;
                Ok(ConversionResult {
                    gvk: "core/v1/ConfigMap".to_string(),
                    original,
                    roundtrip,
                    success: true,
                })
            })),
            validate: None,
        },
    );

    // ---- core/v1/Secret ----
    registry.register(
        "core/v1/Secret",
        TypeHandler {
            apply_defaults: Box::new(|json| {
                let mut secret: crate::core::v1::config::Secret = parse_json(json)?;
                secret.apply_default();
                Ok(DefaultResult {
                    gvk: "core/v1/Secret".to_string(),
                    result: to_value(&secret)?,
                    defaults_applied: true,
                })
            }),
            convert_roundtrip: Some(Box::new(|json| {
                let secret: crate::core::v1::config::Secret = parse_json(json)?;
                let original = to_value(&secret)?;
                let internal: crate::core::internal::config::Secret = secret.to_internal();
                let roundtrip =
                    serde_json::to_value(&internal).map_err(|e| HarnessError::Internal {
                        message: e.to_string(),
                    })?;
                Ok(ConversionResult {
                    gvk: "core/v1/Secret".to_string(),
                    original,
                    roundtrip,
                    success: true,
                })
            })),
            validate: None,
        },
    );

    // ---- core/v1/Namespace ----
    registry.register(
        "core/v1/Namespace",
        TypeHandler {
            apply_defaults: Box::new(|json| {
                let mut ns: crate::core::v1::namespace::Namespace = parse_json(json)?;
                ns.apply_default();
                Ok(DefaultResult {
                    gvk: "core/v1/Namespace".to_string(),
                    result: to_value(&ns)?,
                    defaults_applied: true,
                })
            }),
            convert_roundtrip: Some(Box::new(|json| {
                let ns: crate::core::v1::namespace::Namespace = parse_json(json)?;
                let original = to_value(&ns)?;
                let internal: crate::core::internal::Namespace = ns.to_internal();
                let roundtrip_ns = crate::core::v1::namespace::Namespace::from_internal(internal);
                let roundtrip = to_value(&roundtrip_ns)?;
                Ok(ConversionResult {
                    gvk: "core/v1/Namespace".to_string(),
                    original,
                    roundtrip,
                    success: true,
                })
            })),
            validate: Some(Box::new(|json| {
                let ns: crate::core::v1::namespace::Namespace = parse_json(json)?;
                let errors = crate::core::v1::validation::namespace::validate_namespace(&ns);
                Ok(ValidationResult {
                    gvk: "core/v1/Namespace".to_string(),
                    valid: errors.is_empty(),
                    errors: errors_to_field_errors(&errors),
                })
            })),
        },
    );
}
