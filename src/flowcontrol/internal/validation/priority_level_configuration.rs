//! PriorityLevelConfiguration validation.

use std::collections::HashSet;

use crate::common::validation::{
    BadValue, ErrorList, Path, duplicate, forbidden, invalid, name_is_dns_subdomain, not_supported,
    required, validate_object_meta,
};
use crate::flowcontrol::v1 as flowcontrol;

use super::helpers::{
    MAX_HASH_BITS, PRIORITY_LEVEL_CONFIGURATION_DEFAULT_HAND_SIZE,
    PRIORITY_LEVEL_CONFIGURATION_DEFAULT_NOMINAL_CONCURRENCY_SHARES,
    PRIORITY_LEVEL_CONFIGURATION_DEFAULT_QUEUE_LENGTH_LIMIT,
    PRIORITY_LEVEL_CONFIGURATION_DEFAULT_QUEUES, PRIORITY_LEVEL_CONFIGURATION_QUEUING_MAX_QUEUES,
    SUPPORTED_LIMIT_RESPONSE_TYPE, SUPPORTED_PRIORITY_LEVEL_ENABLEMENT, required_entropy_bits,
    required_field,
};

pub fn validate_priority_level_configuration(
    plc: &flowcontrol::PriorityLevelConfiguration,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let base_path = Path::nil();
    let default_meta = crate::common::ObjectMeta::default();
    let metadata = plc.metadata.as_ref().unwrap_or(&default_meta);

    all_errs.extend(validate_object_meta(
        metadata,
        false,
        name_is_dns_subdomain,
        &base_path.child("metadata"),
    ));

    if let Some(spec) = &plc.spec {
        let name = metadata.name.as_deref().unwrap_or("");
        all_errs.extend(validate_priority_level_configuration_spec(
            spec,
            name,
            &base_path.child("spec"),
        ));
    } else {
        all_errs.push(required(&base_path.child("spec"), "spec is required"));
    }

    if let Some(status) = &plc.status {
        all_errs.extend(validate_priority_level_configuration_status(
            status,
            &base_path.child("status"),
        ));
    }

    all_errs
}

pub fn validate_priority_level_configuration_status_update(
    _old: &flowcontrol::PriorityLevelConfiguration,
    new_obj: &flowcontrol::PriorityLevelConfiguration,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if let Some(status) = &new_obj.status {
        all_errs.extend(validate_priority_level_configuration_status(
            status,
            &Path::new("status"),
        ));
    }
    all_errs
}

pub fn validate_priority_level_configuration_spec(
    spec: &flowcontrol::PriorityLevelConfigurationSpec,
    name: &str,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    let spec_type = match spec.r#type {
        Some(ref t) => t,
        None => {
            all_errs.push(required(&path.child("type"), ""));
            return all_errs;
        }
    };

    let is_exempt_name = name == flowcontrol::priority_level_names::EXEMPT;
    let is_exempt_type = matches!(spec_type, flowcontrol::PriorityLevelEnablement::Exempt);
    if is_exempt_name != is_exempt_type {
        all_errs.push(invalid(
            &path.child("type"),
            BadValue::String(format!("{:?}", spec_type)),
            "must be 'Exempt' if and only if `name` is 'exempt'",
        ));
    }

    match spec_type {
        flowcontrol::PriorityLevelEnablement::Exempt => {
            if spec.limited.is_some() {
                all_errs.push(forbidden(
                    &path.child("limited"),
                    "must be nil if the type is not Limited",
                ));
            }
            if let Some(exempt) = &spec.exempt {
                all_errs.extend(validate_exempt_priority_level_configuration(
                    exempt,
                    &path.child("exempt"),
                ));
            }
        }
        flowcontrol::PriorityLevelEnablement::Limited => {
            if spec.exempt.is_some() {
                all_errs.push(forbidden(
                    &path.child("exempt"),
                    "must be nil if the type is Limited",
                ));
            }

            if let Some(limited) = &spec.limited {
                all_errs.extend(validate_limited_priority_level_configuration(
                    limited,
                    &path.child("limited"),
                ));
            } else {
                all_errs.push(required(
                    &path.child("limited"),
                    "must not be empty when type is Limited",
                ));
            }
        }
    }

    let type_str = match spec_type {
        flowcontrol::PriorityLevelEnablement::Exempt => {
            flowcontrol::priority_level_enablement::EXEMPT
        }
        flowcontrol::PriorityLevelEnablement::Limited => {
            flowcontrol::priority_level_enablement::LIMITED
        }
    };
    if !SUPPORTED_PRIORITY_LEVEL_ENABLEMENT.contains(&type_str) {
        all_errs.push(not_supported(
            &path.child("type"),
            BadValue::String(type_str.to_string()),
            &SUPPORTED_PRIORITY_LEVEL_ENABLEMENT,
        ));
    }

    all_errs
}

fn validate_limited_priority_level_configuration(
    limited: &flowcontrol::LimitedPriorityLevelConfiguration,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let nominal = limited
        .nominal_concurrency_shares
        .unwrap_or(PRIORITY_LEVEL_CONFIGURATION_DEFAULT_NOMINAL_CONCURRENCY_SHARES);
    if nominal < 0 {
        all_errs.push(invalid(
            &path.child("nominalConcurrencyShares"),
            BadValue::Int(nominal as i64),
            "must be a non-negative integer",
        ));
    }

    if let Some(limit_response) = &limited.limit_response {
        all_errs.extend(validate_limit_response(
            limit_response,
            &path.child("limitResponse"),
        ));
    } else {
        all_errs.push(required(
            &path.child("limitResponse"),
            "must not be empty when type is Limited",
        ));
    }

    if let Some(lendable) = limited.lendable_percent {
        if !(0..=100).contains(&lendable) {
            all_errs.push(invalid(
                &path.child("lendablePercent"),
                BadValue::Int(lendable as i64),
                "must be between 0 and 100, inclusive",
            ));
        }
    }

    if let Some(borrowing) = limited.borrowing_limit_percent {
        if borrowing < 0 {
            all_errs.push(invalid(
                &path.child("borrowingLimitPercent"),
                BadValue::Int(borrowing as i64),
                "if specified, must be a non-negative integer",
            ));
        }
    }

    all_errs
}

fn validate_exempt_priority_level_configuration(
    exempt: &flowcontrol::ExemptPriorityLevelConfiguration,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if let Some(shares) = exempt.nominal_concurrency_shares {
        if shares < 0 {
            all_errs.push(invalid(
                &path.child("nominalConcurrencyShares"),
                BadValue::Int(shares as i64),
                "must be a non-negative integer",
            ));
        }
    }

    if let Some(lendable) = exempt.lendable_percent {
        if !(0..=100).contains(&lendable) {
            all_errs.push(invalid(
                &path.child("lendablePercent"),
                BadValue::Int(lendable as i64),
                "must be between 0 and 100, inclusive",
            ));
        }
    }

    all_errs
}

fn validate_limit_response(lr: &flowcontrol::LimitResponse, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    match lr.r#type {
        flowcontrol::LimitResponseType::Reject => {
            if lr.queuing.is_some() {
                all_errs.push(forbidden(
                    &path.child("queuing"),
                    "must be nil if limited.limitResponse.type is not Limited",
                ));
            }
        }
        flowcontrol::LimitResponseType::Queue => {
            if let Some(queuing) = &lr.queuing {
                all_errs.extend(validate_priority_level_queuing_configuration(
                    queuing,
                    &path.child("queuing"),
                ));
            } else {
                all_errs.push(required(
                    &path.child("queuing"),
                    "must not be empty if limited.limitResponse.type is Limited",
                ));
            }
        }
    }

    let type_str = match lr.r#type {
        flowcontrol::LimitResponseType::Queue => flowcontrol::limit_response_type::QUEUE,
        flowcontrol::LimitResponseType::Reject => flowcontrol::limit_response_type::REJECT,
    };
    if !SUPPORTED_LIMIT_RESPONSE_TYPE.contains(&type_str) {
        all_errs.push(not_supported(
            &path.child("type"),
            BadValue::String(type_str.to_string()),
            &SUPPORTED_LIMIT_RESPONSE_TYPE,
        ));
    }

    all_errs
}

fn validate_priority_level_queuing_configuration(
    queuing: &flowcontrol::QueuingConfiguration,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    let queue_length_limit = queuing
        .queue_length_limit
        .unwrap_or(PRIORITY_LEVEL_CONFIGURATION_DEFAULT_QUEUE_LENGTH_LIMIT);
    if queue_length_limit <= 0 {
        all_errs.push(invalid(
            &path.child("queueLengthLimit"),
            BadValue::Int(queue_length_limit as i64),
            "must be positive",
        ));
    }

    let queues = queuing
        .queues
        .unwrap_or(PRIORITY_LEVEL_CONFIGURATION_DEFAULT_QUEUES);
    if queues <= 0 {
        all_errs.push(invalid(
            &path.child("queues"),
            BadValue::Int(queues as i64),
            "must be positive",
        ));
    } else if queues > PRIORITY_LEVEL_CONFIGURATION_QUEUING_MAX_QUEUES {
        all_errs.push(invalid(
            &path.child("queues"),
            BadValue::Int(queues as i64),
            &format!("must not be greater than {PRIORITY_LEVEL_CONFIGURATION_QUEUING_MAX_QUEUES}"),
        ));
    }

    let hand_size = queuing
        .hand_size
        .unwrap_or(PRIORITY_LEVEL_CONFIGURATION_DEFAULT_HAND_SIZE);
    if hand_size <= 0 {
        all_errs.push(invalid(
            &path.child("handSize"),
            BadValue::Int(hand_size as i64),
            "must be positive",
        ));
    } else if hand_size > queues {
        all_errs.push(invalid(
            &path.child("handSize"),
            BadValue::Int(hand_size as i64),
            &format!("should not be greater than queues ({queues})"),
        ));
    } else if required_entropy_bits(queues, hand_size) > MAX_HASH_BITS {
        all_errs.push(invalid(
            &path.child("handSize"),
            BadValue::Int(hand_size as i64),
            &format!(
                "required entropy bits of deckSize {queues} and handSize {hand_size} should not be greater than {MAX_HASH_BITS}"
            ),
        ));
    }

    all_errs
}

pub fn validate_priority_level_configuration_status(
    status: &flowcontrol::PriorityLevelConfigurationStatus,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let mut keys = HashSet::new();

    for (i, condition) in status.conditions.iter().enumerate() {
        let condition_path = path.child("conditions").index(i);
        let key = condition
            .r#type
            .as_ref()
            .map(priority_level_condition_type_str)
            .unwrap_or("");
        if !key.is_empty() && !keys.insert(key.to_string()) {
            all_errs.push(duplicate(
                &condition_path.child("type"),
                BadValue::String(key.to_string()),
            ));
        }
        all_errs.extend(validate_priority_level_configuration_condition(
            condition,
            &condition_path,
        ));
    }

    all_errs
}

pub fn validate_priority_level_configuration_condition(
    condition: &flowcontrol::PriorityLevelConfigurationCondition,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    all_errs.extend(required_field(&condition.r#type, &path.child("type"), ""));
    all_errs
}

fn priority_level_condition_type_str(
    ty: &flowcontrol::PriorityLevelConfigurationConditionType,
) -> &'static str {
    match ty {
        flowcontrol::PriorityLevelConfigurationConditionType::ConcurrencyShared => {
            flowcontrol::priority_level_condition_type::CONCURRENCY_SHARED
        }
    }
}
