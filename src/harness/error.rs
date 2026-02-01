//! Error types for the test harness.

use serde::{Deserialize, Serialize};

/// Errors that can occur during harness operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum HarnessError {
    JsonParse { message: String },
    UnknownGvk { gvk: String },
    ConversionNotImplemented { gvk: String },
    ValidationNotImplemented { gvk: String },
    Internal { message: String },
}

/// A single field-level validation error for JSON output.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FieldError {
    pub error_type: String,
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bad_value: Option<String>,
    pub detail: String,
}

impl From<&crate::common::validation::Error> for FieldError {
    fn from(err: &crate::common::validation::Error) -> Self {
        FieldError {
            error_type: err.error_type.to_string(),
            field: err.field.clone(),
            bad_value: err.bad_value.as_ref().map(|v| v.to_string()),
            detail: err.detail.clone(),
        }
    }
}
