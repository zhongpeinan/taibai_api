//! Test harness for GVK-based dispatch of defaults, conversion, and validation.
//!
//! Feature-gated behind `harness`. Provides a CLI binary and programmatic API
//! for Go fixture tools to verify Rust implementations via process calls.

mod api;
mod error;
mod registry;
mod types;

pub use api::{apply_defaults, convert_roundtrip, full_pipeline, list_registered_gvks, validate};
pub use error::{FieldError, HarnessError};

use serde::{Deserialize, Serialize};

/// Result of applying defaults to a resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultResult {
    pub gvk: String,
    pub result: serde_json::Value,
    pub defaults_applied: bool,
}

/// Result of a v1 -> internal -> v1 conversion roundtrip.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionResult {
    pub gvk: String,
    pub original: serde_json::Value,
    pub roundtrip: serde_json::Value,
    pub success: bool,
}

/// Result of validating a resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub gvk: String,
    pub valid: bool,
    pub errors: Vec<FieldError>,
}

/// Result of running the full pipeline (default -> convert -> validate).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineResult {
    pub gvk: String,
    pub defaults: DefaultResult,
    pub conversion: Option<ConversionResult>,
    pub validation: Option<ValidationResult>,
    pub success: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pod_defaults() {
        let result = apply_defaults("core/v1/Pod", "{}").unwrap();
        assert!(result.defaults_applied);
        assert_eq!(result.gvk, "core/v1/Pod");
        let obj = &result.result;
        assert_eq!(obj["apiVersion"], "v1");
        assert_eq!(obj["kind"], "Pod");
    }

    #[test]
    fn test_pod_conversion_roundtrip() {
        let json =
            r#"{"metadata":{"name":"test"},"spec":{"containers":[{"name":"c1","image":"nginx"}]}}"#;
        let result = convert_roundtrip("core/v1/Pod", json).unwrap();
        assert!(result.success);
    }

    #[test]
    fn test_pod_validation() {
        let json = r#"{"metadata":{"name":"test"}}"#;
        let result = validate("core/v1/Pod", json).unwrap();
        assert_eq!(result.gvk, "core/v1/Pod");
        // An empty pod (no containers) should have validation errors
        assert!(!result.valid);
    }

    #[test]
    fn test_deployment_full_pipeline() {
        let json = r#"{"metadata":{"name":"test","namespace":"default"},"spec":{"selector":{"matchLabels":{"app":"test"}},"template":{"metadata":{"labels":{"app":"test"}},"spec":{"containers":[{"name":"c1","image":"nginx"}]}}}}"#;
        let result = full_pipeline("apps/v1/Deployment", json).unwrap();
        assert_eq!(result.gvk, "apps/v1/Deployment");
        assert!(result.defaults.defaults_applied);
    }

    #[test]
    fn test_unknown_gvk() {
        let result = apply_defaults("unknown/v1/Foo", "{}");
        assert!(result.is_err());
        match result.unwrap_err() {
            HarnessError::UnknownGvk { gvk } => assert_eq!(gvk, "unknown/v1/Foo"),
            other => panic!("Expected UnknownGvk, got {:?}", other),
        }
    }

    #[test]
    fn test_list_registered_gvks() {
        let gvks = list_registered_gvks();
        assert!(gvks.contains(&"core/v1/Pod".to_string()));
        assert!(gvks.contains(&"apps/v1/Deployment".to_string()));
        assert!(gvks.contains(&"batch/v1/Job".to_string()));
        assert!(gvks.len() >= 10);
    }
}
