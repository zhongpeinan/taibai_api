//! Public API functions for the test harness.

use super::error::HarnessError;
use super::registry::global_registry;
use super::{ConversionResult, DefaultResult, PipelineResult, ValidationResult};

/// Apply defaults to a resource identified by GVK.
pub fn apply_defaults(gvk: &str, json: &str) -> Result<DefaultResult, HarnessError> {
    let registry = global_registry();
    let handler = registry.get(gvk).ok_or_else(|| HarnessError::UnknownGvk {
        gvk: gvk.to_string(),
    })?;
    (handler.apply_defaults)(json)
}

/// Perform a v1 -> internal -> v1 conversion roundtrip.
pub fn convert_roundtrip(gvk: &str, json: &str) -> Result<ConversionResult, HarnessError> {
    let registry = global_registry();
    let handler = registry.get(gvk).ok_or_else(|| HarnessError::UnknownGvk {
        gvk: gvk.to_string(),
    })?;
    match &handler.convert_roundtrip {
        Some(f) => f(json),
        None => Err(HarnessError::ConversionNotImplemented {
            gvk: gvk.to_string(),
        }),
    }
}

/// Validate a resource identified by GVK.
pub fn validate(gvk: &str, json: &str) -> Result<ValidationResult, HarnessError> {
    let registry = global_registry();
    let handler = registry.get(gvk).ok_or_else(|| HarnessError::UnknownGvk {
        gvk: gvk.to_string(),
    })?;
    match &handler.validate {
        Some(f) => f(json),
        None => Err(HarnessError::ValidationNotImplemented {
            gvk: gvk.to_string(),
        }),
    }
}

/// Run the full pipeline: defaults -> conversion -> validation.
pub fn full_pipeline(gvk: &str, json: &str) -> Result<PipelineResult, HarnessError> {
    let defaults = apply_defaults(gvk, json)?;
    let defaulted_json =
        serde_json::to_string(&defaults.result).map_err(|e| HarnessError::Internal {
            message: e.to_string(),
        })?;

    let conversion = convert_roundtrip(gvk, &defaulted_json).ok();
    let validate_input = if let Some(ref conv) = conversion {
        serde_json::to_string(&conv.roundtrip).map_err(|e| HarnessError::Internal {
            message: e.to_string(),
        })?
    } else {
        defaulted_json
    };
    let validation = validate(gvk, &validate_input).ok();

    let success = conversion.as_ref().map_or(true, |c| c.success)
        && validation.as_ref().map_or(true, |v| v.valid);

    Ok(PipelineResult {
        gvk: gvk.to_string(),
        defaults,
        conversion,
        validation,
        success,
    })
}

/// List all registered GVKs.
pub fn list_registered_gvks() -> Vec<String> {
    global_registry().gvks()
}
