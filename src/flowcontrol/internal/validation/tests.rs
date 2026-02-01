use super::*;
use crate::common::{ObjectMeta, TypeMeta};
use crate::flowcontrol::v1 as flowcontrol;

#[test]
fn test_validate_flow_schema_requires_subjects_and_rules() {
    let schema = flowcontrol::FlowSchema {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("test".to_string()),
            ..Default::default()
        }),
        spec: Some(flowcontrol::FlowSchemaSpec {
            priority_level_configuration: flowcontrol::PriorityLevelConfigurationReference {
                name: "pl".to_string(),
            },
            matching_precedence: Some(10),
            distinguisher_method: None,
            rules: vec![flowcontrol::PolicyRulesWithSubjects::default()],
        }),
        status: None,
    };

    let errors = validate_flow_schema(&schema);
    assert!(!errors.is_empty(), "expected validation errors");
}

#[test]
fn test_validate_priority_level_configuration_requires_type() {
    let plc = flowcontrol::PriorityLevelConfiguration {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("pl".to_string()),
            ..Default::default()
        }),
        spec: Some(flowcontrol::PriorityLevelConfigurationSpec::default()),
        status: None,
    };

    let errors = validate_priority_level_configuration(&plc);
    assert!(!errors.is_empty(), "expected validation errors");
}
