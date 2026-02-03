//! Validation for Event resources (core internal).
//!
//! Ported from k8s.io/kubernetes/pkg/apis/core/validation/events.go

use crate::common::time::MicroTime;
use crate::common::validation::{
    BadValue, ErrorList, Path, invalid, required, validate_object_meta,
    validate_object_meta_update, validate_qualified_name,
};
use crate::core::internal::event::Event;

const EVENT_TYPE_NORMAL: &str = "Normal";
const EVENT_TYPE_WARNING: &str = "Warning";
const FIELD_IMMUTABLE_ERROR_MSG: &str = "field is immutable";

const REPORTING_INSTANCE_LENGTH_LIMIT: usize = 128;
const ACTION_LENGTH_LIMIT: usize = 128;
const REASON_LENGTH_LIMIT: usize = 128;
const NOTE_LENGTH_LIMIT: usize = 1024;

const NAMESPACE_DEFAULT: &str = "default";
const NAMESPACE_SYSTEM: &str = "kube-system";
const NAMESPACE_NONE: &str = "";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EventRequestVersion {
    pub group: &'static str,
    pub version: &'static str,
}

impl EventRequestVersion {
    pub const fn new(group: &'static str, version: &'static str) -> Self {
        Self { group, version }
    }

    pub fn is_legacy(self) -> bool {
        self == EVENT_REQUEST_VERSION_CORE_V1 || self == EVENT_REQUEST_VERSION_EVENTS_V1BETA1
    }

    pub fn is_core_v1(self) -> bool {
        self == EVENT_REQUEST_VERSION_CORE_V1
    }
}

pub const EVENT_REQUEST_VERSION_CORE_V1: EventRequestVersion = EventRequestVersion::new("", "v1");
pub const EVENT_REQUEST_VERSION_EVENTS_V1BETA1: EventRequestVersion =
    EventRequestVersion::new("events.k8s.io", "v1beta1");
pub const EVENT_REQUEST_VERSION_EVENTS_V1: EventRequestVersion =
    EventRequestVersion::new("events.k8s.io", "v1");

pub fn validate_event_create(event: &Event, request_version: EventRequestVersion) -> ErrorList {
    let mut all_errs = legacy_validate_event(event, request_version);
    if request_version.is_legacy() {
        return all_errs;
    }

    all_errs.extend(validate_object_meta(
        &event.metadata,
        true,
        crate::common::validation::name_is_dns_subdomain,
        &Path::new("metadata"),
    ));

    all_errs.extend(validate_v1_event_series(event));

    if event.event_time.is_none() {
        all_errs.push(required(&Path::new("eventTime"), ""));
    }

    let event_type_value = event.r#type.as_str();
    if event_type_value != EVENT_TYPE_NORMAL && event_type_value != EVENT_TYPE_WARNING {
        all_errs.push(invalid(
            &Path::new("type"),
            BadValue::String(event_type_value.to_string()),
            &format!("has invalid value: {}", event_type_value),
        ));
    }

    if event.first_timestamp.is_some() {
        all_errs.push(invalid(
            &Path::new("firstTimestamp"),
            BadValue::String(String::new()),
            "needs to be unset",
        ));
    }
    if event.last_timestamp.is_some() {
        all_errs.push(invalid(
            &Path::new("lastTimestamp"),
            BadValue::String(String::new()),
            "needs to be unset",
        ));
    }
    if event.count != 0 {
        all_errs.push(invalid(
            &Path::new("count"),
            BadValue::Int(event.count as i64),
            "needs to be unset",
        ));
    }
    if !event.source.component.is_empty() || !event.source.host.is_empty() {
        all_errs.push(invalid(
            &Path::new("source"),
            BadValue::String(String::new()),
            "needs to be unset",
        ));
    }

    all_errs
}

pub fn validate_event_update(
    new_event: &Event,
    old_event: &Event,
    request_version: EventRequestVersion,
) -> ErrorList {
    let mut all_errs = legacy_validate_event(new_event, request_version);
    if request_version.is_legacy() {
        return all_errs;
    }

    all_errs.extend(validate_object_meta_update(
        &new_event.metadata,
        &old_event.metadata,
        &Path::new("metadata"),
    ));

    if new_event.series != old_event.series {
        all_errs.extend(validate_v1_event_series(new_event));
    }

    all_errs.extend(validate_immutable_field(
        &new_event.involved_object,
        &old_event.involved_object,
        &Path::new("involvedObject"),
    ));
    all_errs.extend(validate_immutable_field(
        &new_event.reason,
        &old_event.reason,
        &Path::new("reason"),
    ));
    all_errs.extend(validate_immutable_field(
        &new_event.message,
        &old_event.message,
        &Path::new("message"),
    ));
    all_errs.extend(validate_immutable_field(
        &new_event.source,
        &old_event.source,
        &Path::new("source"),
    ));
    all_errs.extend(validate_immutable_field_option(
        &new_event.first_timestamp,
        &old_event.first_timestamp,
        &Path::new("firstTimestamp"),
    ));
    all_errs.extend(validate_immutable_field_option(
        &new_event.last_timestamp,
        &old_event.last_timestamp,
        &Path::new("lastTimestamp"),
    ));
    all_errs.extend(validate_immutable_field(
        &new_event.count,
        &old_event.count,
        &Path::new("count"),
    ));
    all_errs.extend(validate_immutable_field(
        &new_event.r#type,
        &old_event.r#type,
        &Path::new("type"),
    ));

    let new_truncated = new_event.event_time.as_ref().map(truncate_micro_time);
    let old_truncated = old_event.event_time.as_ref().map(truncate_micro_time);
    if new_truncated != old_truncated {
        all_errs.extend(validate_immutable_field_option(
            &new_event.event_time,
            &old_event.event_time,
            &Path::new("eventTime"),
        ));
    }

    all_errs.extend(validate_immutable_field(
        &new_event.action,
        &old_event.action,
        &Path::new("action"),
    ));
    all_errs.extend(validate_immutable_field_option(
        &new_event.related,
        &old_event.related,
        &Path::new("related"),
    ));
    all_errs.extend(validate_immutable_field(
        &new_event.reporting_controller,
        &old_event.reporting_controller,
        &Path::new("reportingController"),
    ));
    all_errs.extend(validate_immutable_field(
        &new_event.reporting_instance,
        &old_event.reporting_instance,
        &Path::new("reportingInstance"),
    ));

    all_errs
}

fn truncate_micro_time(time: &MicroTime) -> i64 {
    time.as_ref().timestamp_micros()
}

fn validate_v1_event_series(event: &Event) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if let Some(series) = &event.series {
        if series.count < 2 {
            all_errs.push(invalid(
                &Path::new("series").child("count"),
                BadValue::Int(series.count as i64),
                "should be at least 2",
            ));
        }
        if series.last_observed_time.is_none() {
            all_errs.push(required(&Path::new("series").child("lastObservedTime"), ""));
        }
    }
    all_errs
}

fn legacy_validate_event(event: &Event, request_version: EventRequestVersion) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let event_time_is_zero = event.event_time.is_none();

    let reporting_controller_field_name = if request_version.is_core_v1() {
        "reportingComponent"
    } else {
        "reportingController"
    };

    let event_namespace = event.metadata.namespace.as_deref().unwrap_or("");
    let involved_namespace = event.involved_object.namespace.as_deref().unwrap_or("");

    if event_time_is_zero {
        if involved_namespace.is_empty() {
            if event_namespace != NAMESPACE_NONE && event_namespace != NAMESPACE_DEFAULT {
                all_errs.push(invalid(
                    &Path::new("involvedObject").child("namespace"),
                    BadValue::String(involved_namespace.to_string()),
                    "does not match event.namespace",
                ));
            }
        } else if event_namespace != involved_namespace {
            all_errs.push(invalid(
                &Path::new("involvedObject").child("namespace"),
                BadValue::String(involved_namespace.to_string()),
                "does not match event.namespace",
            ));
        }
    } else {
        if involved_namespace.is_empty()
            && event_namespace != NAMESPACE_DEFAULT
            && event_namespace != NAMESPACE_SYSTEM
        {
            all_errs.push(invalid(
                &Path::new("involvedObject").child("namespace"),
                BadValue::String(involved_namespace.to_string()),
                "does not match event.namespace",
            ));
        }

        let reporting_controller = event.reporting_controller.as_str();
        if reporting_controller.is_empty() {
            all_errs.push(required(&Path::new(reporting_controller_field_name), ""));
        }
        all_errs.extend(validate_qualified_name(
            reporting_controller,
            &Path::new(reporting_controller_field_name),
        ));

        let reporting_instance = event.reporting_instance.as_str();
        if reporting_instance.is_empty() {
            all_errs.push(required(&Path::new("reportingInstance"), ""));
        }
        if reporting_instance.len() > REPORTING_INSTANCE_LENGTH_LIMIT {
            all_errs.push(invalid(
                &Path::new("reportingInstance"),
                BadValue::String(reporting_instance.to_string()),
                &format!(
                    "can have at most {} characters",
                    REPORTING_INSTANCE_LENGTH_LIMIT
                ),
            ));
        }

        let action = event.action.as_str();
        if action.is_empty() {
            all_errs.push(required(&Path::new("action"), ""));
        }
        if action.len() > ACTION_LENGTH_LIMIT {
            all_errs.push(invalid(
                &Path::new("action"),
                BadValue::String(action.to_string()),
                &format!("can have at most {} characters", ACTION_LENGTH_LIMIT),
            ));
        }

        let reason = event.reason.as_str();
        if reason.is_empty() {
            all_errs.push(required(&Path::new("reason"), ""));
        }
        if reason.len() > REASON_LENGTH_LIMIT {
            all_errs.push(invalid(
                &Path::new("reason"),
                BadValue::String(reason.to_string()),
                &format!("can have at most {} characters", REASON_LENGTH_LIMIT),
            ));
        }

        let message = event.message.as_str();
        if message.len() > NOTE_LENGTH_LIMIT {
            all_errs.push(invalid(
                &Path::new("message"),
                BadValue::String(message.to_string()),
                &format!("can have at most {} characters", NOTE_LENGTH_LIMIT),
            ));
        }
    }

    all_errs.extend(validate_dns1123_subdomain(
        event_namespace,
        &Path::new("namespace"),
    ));
    all_errs
}

fn validate_dns1123_subdomain(value: &str, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    for msg in crate::common::validation::is_dns1123_subdomain(value) {
        all_errs.push(invalid(path, BadValue::String(value.to_string()), &msg));
    }
    all_errs
}

fn validate_immutable_field<T: PartialEq>(new: &T, old: &T, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if new != old {
        all_errs.push(crate::common::validation::forbidden(
            path,
            FIELD_IMMUTABLE_ERROR_MSG,
        ));
    }
    all_errs
}

fn validate_immutable_field_option<T: PartialEq>(
    new: &Option<T>,
    old: &Option<T>,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    match (new, old) {
        (Some(n), Some(o)) if n != o => {
            all_errs.push(crate::common::validation::forbidden(
                path,
                FIELD_IMMUTABLE_ERROR_MSG,
            ));
        }
        (Some(_), None) | (None, Some(_)) => {
            all_errs.push(crate::common::validation::forbidden(
                path,
                FIELD_IMMUTABLE_ERROR_MSG,
            ));
        }
        _ => {}
    }
    all_errs
}
