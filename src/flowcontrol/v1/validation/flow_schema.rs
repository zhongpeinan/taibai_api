use crate::common::validation::ErrorList;
use crate::flowcontrol::internal::validation as internal_validation;
use crate::flowcontrol::v1 as flowcontrol;

pub fn validate_flow_schema(flow_schema: &flowcontrol::FlowSchema) -> ErrorList {
    internal_validation::validate_flow_schema(flow_schema)
}

pub fn validate_flow_schema_update(
    old: &flowcontrol::FlowSchema,
    new: &flowcontrol::FlowSchema,
) -> ErrorList {
    internal_validation::validate_flow_schema_update(old, new)
}

pub fn validate_flow_schema_status_update(
    old: &flowcontrol::FlowSchema,
    new: &flowcontrol::FlowSchema,
) -> ErrorList {
    internal_validation::validate_flow_schema_status_update(old, new)
}

pub fn validate_flow_schema_spec(
    flow_schema_name: &str,
    spec: &flowcontrol::FlowSchemaSpec,
    path: &crate::common::validation::Path,
) -> ErrorList {
    internal_validation::validate_flow_schema_spec(flow_schema_name, spec, path)
}

pub fn validate_flow_schema_policy_rules_with_subjects(
    rule: &flowcontrol::PolicyRulesWithSubjects,
    path: &crate::common::validation::Path,
) -> ErrorList {
    internal_validation::validate_flow_schema_policy_rules_with_subjects(rule, path)
}

pub fn validate_flow_schema_condition(
    condition: &flowcontrol::FlowSchemaCondition,
    path: &crate::common::validation::Path,
) -> ErrorList {
    internal_validation::validate_flow_schema_condition(condition, path)
}

pub fn validate_flow_schema_status(
    status: &flowcontrol::FlowSchemaStatus,
    path: &crate::common::validation::Path,
) -> ErrorList {
    internal_validation::validate_flow_schema_status(status, path)
}
