//! Validation for Kubernetes Events API v1 types
//!
//! Ported from k8s.io/kubernetes/pkg/apis/core/validation/events.go

use crate::common::validation::{
    BadValue, ErrorList, Path, invalid, is_dns1123_subdomain, is_qualified_name,
    name_is_dns_subdomain, required, validate_object_meta,
};
use crate::events::v1::{Event, EventList};

// ============================================================================
// Constants
// ============================================================================

const REPORTING_INSTANCE_LENGTH_LIMIT: usize = 128;
const ACTION_LENGTH_LIMIT: usize = 128;
const REASON_LENGTH_LIMIT: usize = 128;
const NOTE_LENGTH_LIMIT: usize = 1024;

// ============================================================================
// Event Validation
// ============================================================================

/// Validates an Event object for creation.
///
/// Based on ValidateEventCreate from k8s.io/kubernetes/pkg/apis/core/validation/events.go
pub fn validate_event(event: &Event) -> ErrorList {
    validate_event_with_path(event, &Path::nil())
}

fn validate_event_with_path(event: &Event, base_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate metadata
    let default_meta = crate::common::ObjectMeta::default();
    let meta = event.metadata.as_ref().unwrap_or(&default_meta);
    all_errs.extend(validate_object_meta(
        meta,
        true,
        name_is_dns_subdomain,
        &base_path.child("metadata"),
    ));

    let event_namespace = meta.namespace.as_deref().unwrap_or("");
    let regarding_namespace = event.regarding.namespace.as_deref().unwrap_or("");
    let event_time_zero =
        event.event_time.0.timestamp() == 0 && event.event_time.0.timestamp_subsec_nanos() == 0;

    if event_time_zero {
        if regarding_namespace.is_empty() {
            if event_namespace != "" && event_namespace != "default" {
                all_errs.push(invalid(
                    &base_path.child("regarding").child("namespace"),
                    BadValue::String(regarding_namespace.to_string()),
                    "does not match event.namespace",
                ));
            }
        } else if event_namespace != regarding_namespace {
            all_errs.push(invalid(
                &base_path.child("regarding").child("namespace"),
                BadValue::String(regarding_namespace.to_string()),
                "does not match event.namespace",
            ));
        }
    } else if regarding_namespace.is_empty()
        && event_namespace != "default"
        && event_namespace != "kube-system"
    {
        all_errs.push(invalid(
            &base_path.child("regarding").child("namespace"),
            BadValue::String(regarding_namespace.to_string()),
            "does not match event.namespace",
        ));
    }

    // Validate namespace is a DNS subdomain
    for msg in is_dns1123_subdomain(event_namespace) {
        all_errs.push(invalid(
            &base_path.child("metadata").child("namespace"),
            BadValue::String(event_namespace.to_string()),
            &msg,
        ));
    }

    // EventTime is required (check if it's zero/default)
    if event_time_zero {
        all_errs.push(required(
            &base_path.child("eventTime"),
            "eventTime is required for new Events",
        ));
    }

    // Type must be "Normal" or "Warning"
    if event.type_ != "Normal" && event.type_ != "Warning" {
        all_errs.push(invalid(
            &base_path.child("type"),
            BadValue::String(event.type_.clone()),
            "must be 'Normal' or 'Warning'",
        ));
    }

    if !event_time_zero {
        // ReportingController is required
        if event.reporting_controller.is_empty() {
            all_errs.push(required(
                &base_path.child("reportingController"),
                "reportingController is required for new Events",
            ));
        } else {
            // ReportingController must be a qualified name
            for msg in is_qualified_name(&event.reporting_controller) {
                all_errs.push(invalid(
                    &base_path.child("reportingController"),
                    BadValue::String(event.reporting_controller.clone()),
                    &msg,
                ));
            }
        }

        // ReportingInstance is required
        if event.reporting_instance.is_empty() {
            all_errs.push(required(
                &base_path.child("reportingInstance"),
                "reportingInstance is required for new Events",
            ));
        } else if event.reporting_instance.len() > REPORTING_INSTANCE_LENGTH_LIMIT {
            all_errs.push(invalid(
                &base_path.child("reportingInstance"),
                BadValue::String(event.reporting_instance.clone()),
                &format!(
                    "must have at most {} characters",
                    REPORTING_INSTANCE_LENGTH_LIMIT
                ),
            ));
        }

        // Action is required
        if event.action.is_empty() {
            all_errs.push(required(
                &base_path.child("action"),
                "action is required for new Events",
            ));
        } else if event.action.len() > ACTION_LENGTH_LIMIT {
            all_errs.push(invalid(
                &base_path.child("action"),
                BadValue::String(event.action.clone()),
                &format!("must have at most {} characters", ACTION_LENGTH_LIMIT),
            ));
        }

        // Reason is required
        if event.reason.is_empty() {
            all_errs.push(required(
                &base_path.child("reason"),
                "reason is required for new Events",
            ));
        } else if event.reason.len() > REASON_LENGTH_LIMIT {
            all_errs.push(invalid(
                &base_path.child("reason"),
                BadValue::String(event.reason.clone()),
                &format!("must have at most {} characters", REASON_LENGTH_LIMIT),
            ));
        }

        // Note length limit
        if event.note.len() > NOTE_LENGTH_LIMIT {
            all_errs.push(invalid(
                &base_path.child("note"),
                BadValue::String(if event.note.len() > 50 {
                    format!("{}...", &event.note[..50])
                } else {
                    event.note.clone()
                }),
                &format!("must have at most {} characters", NOTE_LENGTH_LIMIT),
            ));
        }
    }

    // Validate EventSeries if present
    if let Some(ref series) = event.series {
        all_errs.extend(validate_event_series(series, &base_path.child("series")));
    }

    // For new events (events.k8s.io/v1), deprecated fields should be unset
    // We check if any of them are explicitly set to non-default values

    // FirstTimestamp should be unset
    if let Some(ref ts) = event.deprecated_first_timestamp {
        if ts.0.timestamp() != 0 || ts.0.timestamp_subsec_nanos() != 0 {
            all_errs.push(invalid(
                &base_path.child("deprecatedFirstTimestamp"),
                BadValue::String("set".to_string()),
                "deprecatedFirstTimestamp should be unset for events.k8s.io/v1 Events",
            ));
        }
    }

    // LastTimestamp should be unset
    if let Some(ref ts) = event.deprecated_last_timestamp {
        if ts.0.timestamp() != 0 || ts.0.timestamp_subsec_nanos() != 0 {
            all_errs.push(invalid(
                &base_path.child("deprecatedLastTimestamp"),
                BadValue::String("set".to_string()),
                "deprecatedLastTimestamp should be unset for events.k8s.io/v1 Events",
            ));
        }
    }

    // Count should be unset (0)
    if event.deprecated_count != 0 {
        all_errs.push(invalid(
            &base_path.child("deprecatedCount"),
            BadValue::Int(event.deprecated_count as i64),
            "deprecatedCount should be unset for events.k8s.io/v1 Events",
        ));
    }

    // Source should be unset
    if let Some(ref source) = event.deprecated_source {
        if source.component.is_some() || source.host.is_some() {
            all_errs.push(invalid(
                &base_path.child("deprecatedSource"),
                BadValue::String("set".to_string()),
                "deprecatedSource should be unset for events.k8s.io/v1 Events",
            ));
        }
    }

    all_errs
}

/// Validates EventSeries
fn validate_event_series(series: &crate::events::v1::EventSeries, base_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Count should be at least 2 for a series
    if series.count < 2 {
        all_errs.push(invalid(
            &base_path.child("count"),
            BadValue::Int(series.count as i64),
            "series count must be at least 2",
        ));
    }

    // LastObservedTime is required
    if series.last_observed_time.0.timestamp() == 0
        && series.last_observed_time.0.timestamp_subsec_nanos() == 0
    {
        all_errs.push(required(
            &base_path.child("lastObservedTime"),
            "lastObservedTime is required when series is set",
        ));
    }

    all_errs
}

/// Validates an EventList object.
pub fn validate_event_list(list: &EventList) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (i, item) in list.items.iter().enumerate() {
        let item_path = Path::new("items").index(i);
        all_errs.extend(validate_event_with_path(item, &item_path));
    }

    all_errs
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{MicroTime, ObjectMeta};
    use crate::core::v1::reference::ObjectReference;
    use crate::events::v1::{Event, EventSeries, EventSource};
    use chrono::Utc;

    #[test]
    fn test_validate_event_valid() {
        let event = Event {
            type_meta: crate::common::TypeMeta {
                api_version: "events.k8s.io/v1".to_string(),
                kind: "Event".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("test-event".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            event_time: MicroTime(Utc::now()),
            reporting_controller: "k8s.io/my-controller".to_string(),
            reporting_instance: "my-instance-1".to_string(),
            action: "Started".to_string(),
            reason: "Started".to_string(),
            regarding: ObjectReference {
                kind: Some("Pod".to_string()),
                name: Some("my-pod".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            },
            type_: "Normal".to_string(),
            note: "Container started".to_string(),
            ..Default::default()
        };

        let errors = validate_event(&event);
        assert!(
            errors.is_empty(),
            "Expected no validation errors, got: {:?}",
            errors
        );
    }

    #[test]
    fn test_validate_event_missing_required_fields() {
        let event = Event {
            metadata: Some(ObjectMeta {
                name: Some("test-event".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            event_time: MicroTime(Utc::now()),
            // reporting_controller is empty (invalid)
            reporting_controller: String::new(),
            // reporting_instance is empty (invalid)
            reporting_instance: String::new(),
            // action is empty (invalid)
            action: String::new(),
            // reason is empty (invalid)
            reason: String::new(),
            type_: "Normal".to_string(),
            ..Default::default()
        };

        let errors = validate_event(&event);
        assert!(
            !errors.is_empty(),
            "Expected validation errors for missing required fields"
        );

        // Check that we have errors for each required field
        let error_fields: Vec<String> = errors.errors.iter().map(|e| e.field.to_string()).collect();

        assert!(
            error_fields
                .iter()
                .any(|f| f.contains("reportingController")),
            "Expected error for reportingController"
        );
        assert!(
            error_fields.iter().any(|f| f.contains("reportingInstance")),
            "Expected error for reportingInstance"
        );
        assert!(
            error_fields.iter().any(|f| f.contains("action")),
            "Expected error for action"
        );
        assert!(
            error_fields.iter().any(|f| f.contains("reason")),
            "Expected error for reason"
        );
    }

    #[test]
    fn test_validate_event_missing_event_time() {
        let event = Event {
            metadata: Some(ObjectMeta {
                name: Some("test-event".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            event_time: MicroTime::default(),
            reporting_controller: "k8s.io/controller".to_string(),
            reporting_instance: "instance-1".to_string(),
            action: "Action".to_string(),
            reason: "Reason".to_string(),
            type_: "Normal".to_string(),
            ..Default::default()
        };

        let errors = validate_event(&event);
        assert!(
            errors.errors.iter().any(|e| e.field.contains("eventTime")),
            "Expected error for eventTime"
        );
    }

    #[test]
    fn test_validate_event_invalid_type() {
        let event = Event {
            metadata: Some(ObjectMeta {
                name: Some("test-event".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            event_time: MicroTime(Utc::now()),
            reporting_controller: "k8s.io/controller".to_string(),
            reporting_instance: "instance-1".to_string(),
            action: "Action".to_string(),
            reason: "Reason".to_string(),
            type_: "Invalid".to_string(), // Invalid type
            ..Default::default()
        };

        let errors = validate_event(&event);
        assert!(
            !errors.is_empty(),
            "Expected validation error for invalid type"
        );

        let error_msg = errors
            .errors
            .iter()
            .find(|e| e.field.to_string().contains("type"));
        assert!(error_msg.is_some(), "Expected error for type field");
    }

    #[test]
    fn test_validate_event_length_limits() {
        let event = Event {
            metadata: Some(ObjectMeta {
                name: Some("test-event".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            event_time: MicroTime(Utc::now()),
            reporting_controller: "k8s.io/controller".to_string(),
            reporting_instance: "x".repeat(200), // Exceeds 128 char limit
            action: "y".repeat(200),             // Exceeds 128 char limit
            reason: "z".repeat(200),             // Exceeds 128 char limit
            note: "n".repeat(2000),              // Exceeds 1024 char limit
            type_: "Normal".to_string(),
            ..Default::default()
        };

        let errors = validate_event(&event);
        assert!(
            !errors.is_empty(),
            "Expected validation errors for length limits"
        );

        let error_fields: Vec<String> = errors.errors.iter().map(|e| e.field.to_string()).collect();

        assert!(
            error_fields.iter().any(|f| f.contains("reportingInstance")),
            "Expected error for reportingInstance length"
        );
        assert!(
            error_fields.iter().any(|f| f.contains("action")),
            "Expected error for action length"
        );
        assert!(
            error_fields.iter().any(|f| f.contains("reason")),
            "Expected error for reason length"
        );
        assert!(
            error_fields.iter().any(|f| f.contains("note")),
            "Expected error for note length"
        );
    }

    #[test]
    fn test_validate_event_series_invalid() {
        let event = Event {
            metadata: Some(ObjectMeta {
                name: Some("test-event".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            event_time: MicroTime(Utc::now()),
            reporting_controller: "k8s.io/controller".to_string(),
            reporting_instance: "instance-1".to_string(),
            action: "Action".to_string(),
            reason: "Reason".to_string(),
            type_: "Normal".to_string(),
            series: Some(EventSeries {
                count: 1,                                 // Invalid: should be at least 2
                last_observed_time: MicroTime::default(), // Invalid: should be set
            }),
            ..Default::default()
        };

        let errors = validate_event(&event);
        assert!(
            !errors.is_empty(),
            "Expected validation errors for invalid series"
        );

        let error_fields: Vec<String> = errors.errors.iter().map(|e| e.field.to_string()).collect();

        assert!(
            error_fields.iter().any(|f| f.contains("series.count")),
            "Expected error for series count"
        );
        assert!(
            error_fields
                .iter()
                .any(|f| f.contains("series.lastObservedTime")),
            "Expected error for series lastObservedTime"
        );
    }

    #[test]
    fn test_validate_event_deprecated_fields_set() {
        let event = Event {
            metadata: Some(ObjectMeta {
                name: Some("test-event".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            event_time: MicroTime(Utc::now()),
            reporting_controller: "k8s.io/controller".to_string(),
            reporting_instance: "instance-1".to_string(),
            action: "Action".to_string(),
            reason: "Reason".to_string(),
            type_: "Normal".to_string(),
            // These deprecated fields should not be set for v1 events
            deprecated_source: Some(EventSource {
                component: Some("component".to_string()),
                host: Some("host".to_string()),
            }),
            deprecated_first_timestamp: Some(crate::common::Timestamp(Utc::now())),
            deprecated_last_timestamp: Some(crate::common::Timestamp(Utc::now())),
            deprecated_count: 5,
            ..Default::default()
        };

        let errors = validate_event(&event);
        assert!(
            !errors.is_empty(),
            "Expected validation errors for deprecated fields being set"
        );

        let error_fields: Vec<String> = errors.errors.iter().map(|e| e.field.to_string()).collect();

        assert!(
            error_fields.iter().any(|f| f.contains("deprecatedSource")),
            "Expected error for deprecatedSource"
        );
        assert!(
            error_fields
                .iter()
                .any(|f| f.contains("deprecatedFirstTimestamp")),
            "Expected error for deprecatedFirstTimestamp"
        );
        assert!(
            error_fields
                .iter()
                .any(|f| f.contains("deprecatedLastTimestamp")),
            "Expected error for deprecatedLastTimestamp"
        );
        assert!(
            error_fields.iter().any(|f| f.contains("deprecatedCount")),
            "Expected error for deprecatedCount"
        );
    }
}
