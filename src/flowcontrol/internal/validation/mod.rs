//! Validation for Kubernetes FlowControl API types (internal implementation)
//!
//! Ported from k8s.io/kubernetes/pkg/apis/flowcontrol/validation/validation.go

mod flow_schema;
mod helpers;
mod priority_level_configuration;

pub use flow_schema::{
    validate_flow_schema, validate_flow_schema_condition,
    validate_flow_schema_policy_rules_with_subjects, validate_flow_schema_spec,
    validate_flow_schema_status, validate_flow_schema_status_update, validate_flow_schema_update,
};
pub use priority_level_configuration::{
    validate_priority_level_configuration, validate_priority_level_configuration_condition,
    validate_priority_level_configuration_spec, validate_priority_level_configuration_status,
    validate_priority_level_configuration_status_update,
};

#[cfg(test)]
mod tests;
