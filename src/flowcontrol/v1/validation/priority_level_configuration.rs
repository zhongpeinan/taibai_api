use crate::common::validation::ErrorList;
use crate::flowcontrol::internal::validation as internal_validation;
use crate::flowcontrol::v1 as flowcontrol;

pub fn validate_priority_level_configuration(
    plc: &flowcontrol::PriorityLevelConfiguration,
) -> ErrorList {
    internal_validation::validate_priority_level_configuration(plc)
}

pub fn validate_priority_level_configuration_status_update(
    old: &flowcontrol::PriorityLevelConfiguration,
    new: &flowcontrol::PriorityLevelConfiguration,
) -> ErrorList {
    internal_validation::validate_priority_level_configuration_status_update(old, new)
}

pub fn validate_priority_level_configuration_spec(
    spec: &flowcontrol::PriorityLevelConfigurationSpec,
    name: &str,
    path: &crate::common::validation::Path,
) -> ErrorList {
    internal_validation::validate_priority_level_configuration_spec(spec, name, path)
}

pub fn validate_priority_level_configuration_condition(
    condition: &flowcontrol::PriorityLevelConfigurationCondition,
    path: &crate::common::validation::Path,
) -> ErrorList {
    internal_validation::validate_priority_level_configuration_condition(condition, path)
}

pub fn validate_priority_level_configuration_status(
    status: &flowcontrol::PriorityLevelConfigurationStatus,
    path: &crate::common::validation::Path,
) -> ErrorList {
    internal_validation::validate_priority_level_configuration_status(status, path)
}
