//! Kubernetes Apiextensions API types
//!
//! This module contains types from the apiextensions.k8s.io API group.

pub mod internal;
pub mod v1;
pub mod v1beta1;
pub mod validation;

/// Placeholder for JSON schema types (types_jsonschema.go is intentionally not ported).
pub type JSONSchemaProps = serde_json::Value;

pub use v1::{CustomResourceDefinition, CustomResourceDefinitionList};
pub use v1beta1::{
    CustomResourceDefinition as CustomResourceDefinitionV1Beta1,
    CustomResourceDefinitionList as CustomResourceDefinitionListV1Beta1,
};
