//! Conversions between v1 and internal event types
//!
//! Based on k8s.io/kubernetes/pkg/apis/events/v1/conversion.go

#[allow(unused_imports)]
use crate::common::{
    ApplyDefault, FromInternal, ListMeta, MicroTime, ObjectMeta, Timestamp, ToInternal, TypeMeta,
};
use crate::core::v1::reference::ObjectReference;
use crate::events::internal;

use super::{Event, EventList, EventSeries, EventSource};

// ============================================================================
// Conversion Helper Functions
// ============================================================================

fn is_empty_object_meta(meta: &ObjectMeta) -> bool {
    meta.name.is_none()
        && meta.generate_name.is_none()
        && meta.namespace.is_none()
        && meta.uid.is_none()
        && meta.resource_version.is_none()
        && meta.generation.is_none()
        && meta.self_link.is_none()
        && meta.labels.is_empty()
        && meta.annotations.is_empty()
        && meta.owner_references.is_empty()
        && meta.finalizers.is_empty()
        && meta.managed_fields.is_empty()
        && meta.creation_timestamp.is_none()
        && meta.deletion_timestamp.is_none()
        && meta.deletion_grace_period_seconds.is_none()
}

fn is_empty_list_meta(meta: &ListMeta) -> bool {
    meta.continue_.is_none()
        && meta.remaining_item_count.is_none()
        && meta.resource_version.is_none()
        && meta.self_link.is_none()
}

fn option_object_meta_to_meta(meta: Option<ObjectMeta>) -> ObjectMeta {
    meta.unwrap_or_default()
}

fn meta_to_option_object_meta(meta: ObjectMeta) -> Option<ObjectMeta> {
    if is_empty_object_meta(&meta) {
        None
    } else {
        Some(meta)
    }
}

fn option_list_meta_to_meta(meta: Option<ListMeta>) -> ListMeta {
    meta.unwrap_or_default()
}

fn meta_to_option_list_meta(meta: ListMeta) -> Option<ListMeta> {
    if is_empty_list_meta(&meta) {
        None
    } else {
        Some(meta)
    }
}

// MicroTime (in v1) <-> Option<MicroTime> (in internal)
fn microtime_to_option_microtime(mt: MicroTime) -> Option<MicroTime> {
    if mt.0.timestamp() == 0 && mt.0.timestamp_subsec_nanos() == 0 {
        None
    } else {
        Some(mt)
    }
}

fn option_microtime_to_microtime(mt: Option<MicroTime>) -> MicroTime {
    mt.unwrap_or_default()
}

// Timestamp (in v1) <-> Option<Timestamp> (in internal)
fn timestamp_to_option_timestamp(ts: Timestamp) -> Option<Timestamp> {
    if ts.0.timestamp() == 0 && ts.0.timestamp_subsec_nanos() == 0 {
        None
    } else {
        Some(ts)
    }
}

// EventSource conversion helpers
fn event_source_to_internal(src: EventSource) -> internal::EventSource {
    internal::EventSource {
        component: src.component.unwrap_or_default(),
        host: src.host.unwrap_or_default(),
    }
}

fn internal_event_source_to_v1(src: internal::EventSource) -> EventSource {
    EventSource {
        component: if src.component.is_empty() {
            None
        } else {
            Some(src.component)
        },
        host: if src.host.is_empty() {
            None
        } else {
            Some(src.host)
        },
    }
}

fn internal_event_source_to_option_v1(src: internal::EventSource) -> Option<EventSource> {
    if src.component.is_empty() && src.host.is_empty() {
        None
    } else {
        Some(internal_event_source_to_v1(src))
    }
}

// EventSeries conversion helpers
fn option_event_series_to_internal(series: Option<EventSeries>) -> Option<internal::EventSeries> {
    series.map(|s| internal::EventSeries {
        count: s.count,
        last_observed_time: microtime_to_option_microtime(s.last_observed_time),
    })
}

fn internal_event_series_to_v1(series: Option<internal::EventSeries>) -> Option<EventSeries> {
    series.map(|s| EventSeries {
        count: s.count,
        last_observed_time: option_microtime_to_microtime(s.last_observed_time),
    })
}

// ObjectReference conversion helpers - internal Event uses the same ObjectReference as v1
fn object_reference_to_internal(obj: ObjectReference) -> ObjectReference {
    // No conversion needed - both use the same type
    obj
}

fn internal_object_reference_to_v1(obj: ObjectReference) -> ObjectReference {
    // No conversion needed - both use the same type
    obj
}

// ============================================================================
// Event Conversions
// ============================================================================

impl ToInternal<internal::Event> for Event {
    fn to_internal(self) -> internal::Event {
        // Based on Convert_v1_Event_To_core_Event from
        // k8s.io/kubernetes/pkg/apis/events/v1/conversion.go
        internal::Event {
            metadata: option_object_meta_to_meta(self.metadata),
            involved_object: object_reference_to_internal(self.regarding),
            reason: self.reason,
            message: self.note,
            source: self
                .deprecated_source
                .map(event_source_to_internal)
                .unwrap_or_default(),
            first_timestamp: self
                .deprecated_first_timestamp
                .and_then(timestamp_to_option_timestamp),
            last_timestamp: self
                .deprecated_last_timestamp
                .and_then(timestamp_to_option_timestamp),
            count: self.deprecated_count,
            r#type: self.type_,
            event_time: microtime_to_option_microtime(self.event_time),
            series: option_event_series_to_internal(self.series),
            action: self.action,
            related: self.related.map(object_reference_to_internal),
            reporting_controller: self.reporting_controller,
            reporting_instance: self.reporting_instance,
        }
    }
}

impl FromInternal<internal::Event> for Event {
    fn from_internal(value: internal::Event) -> Self {
        // Based on Convert_core_Event_To_v1_Event from
        // k8s.io/kubernetes/pkg/apis/events/v1/conversion.go
        let result = Self {
            type_meta: TypeMeta::default(),
            metadata: meta_to_option_object_meta(value.metadata),
            event_time: option_microtime_to_microtime(value.event_time),
            series: internal_event_series_to_v1(value.series),
            reporting_controller: value.reporting_controller,
            reporting_instance: value.reporting_instance,
            action: value.action,
            reason: value.reason,
            regarding: internal_object_reference_to_v1(value.involved_object),
            related: value.related.map(internal_object_reference_to_v1),
            note: value.message,
            type_: value.r#type,
            deprecated_source: internal_event_source_to_option_v1(value.source),
            deprecated_first_timestamp: value.first_timestamp.map(|t| Timestamp(t.0)),
            deprecated_last_timestamp: value.last_timestamp.map(|t| Timestamp(t.0)),
            deprecated_count: value.count,
        };

        result
    }
}

// ============================================================================
// EventList Conversions
// ============================================================================

impl ToInternal<internal::EventList> for EventList {
    fn to_internal(self) -> internal::EventList {
        internal::EventList {
            type_meta: TypeMeta::default(),
            metadata: option_list_meta_to_meta(self.metadata),
            items: self.items.into_iter().map(|e| e.to_internal()).collect(),
        }
    }
}

impl FromInternal<internal::EventList> for EventList {
    fn from_internal(value: internal::EventList) -> Self {
        let result = Self {
            type_meta: TypeMeta::default(),
            metadata: meta_to_option_list_meta(value.metadata),
            items: value.items.into_iter().map(Event::from_internal).collect(),
        };

        result
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_event_round_trip_v1_to_internal_to_v1() {
        // Create a v1 Event with various fields populated
        let original = Event {
            type_meta: TypeMeta {
                api_version: "events.k8s.io/v1".to_string(),
                kind: "Event".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("test-event".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            event_time: MicroTime(Utc::now()),
            series: Some(EventSeries {
                count: 3,
                last_observed_time: MicroTime(Utc::now()),
            }),
            reporting_controller: "test-controller".to_string(),
            reporting_instance: "test-instance".to_string(),
            action: "TestAction".to_string(),
            reason: "TestReason".to_string(),
            regarding: ObjectReference {
                kind: Some("Pod".to_string()),
                namespace: Some("default".to_string()),
                name: Some("test-pod".to_string()),
                ..Default::default()
            },
            related: None,
            note: "Test note".to_string(),
            type_: "Normal".to_string(),
            deprecated_source: Some(EventSource {
                component: Some("test-component".to_string()),
                host: Some("test-host".to_string()),
            }),
            deprecated_first_timestamp: Some(Timestamp(Utc::now())),
            deprecated_last_timestamp: Some(Timestamp(Utc::now())),
            deprecated_count: 5,
        };

        // Convert to internal
        let internal = original.clone().to_internal();

        // Verify some internal conversions
        assert_eq!(internal.message, "Test note");
        assert_eq!(internal.involved_object.name, Some("test-pod".to_string()));
        assert_eq!(internal.source.component, "test-component");

        // Convert back to v1
        let mut round_trip = Event::from_internal(internal);
        round_trip.apply_default();

        // Verify key fields survived the round trip
        assert_eq!(round_trip.metadata, original.metadata);
        assert_eq!(
            round_trip.reporting_controller,
            original.reporting_controller
        );
        assert_eq!(round_trip.reporting_instance, original.reporting_instance);
        assert_eq!(round_trip.action, original.action);
        assert_eq!(round_trip.reason, original.reason);
        assert_eq!(round_trip.note, original.note);
        assert_eq!(round_trip.type_, original.type_);
        assert_eq!(round_trip.regarding.name, original.regarding.name);

        // TypeMeta should be defaulted
        assert_eq!(round_trip.type_meta.api_version, "events.k8s.io/v1");
        assert_eq!(round_trip.type_meta.kind, "Event");
    }

    #[test]
    fn test_event_list_round_trip() {
        let original = EventList {
            type_meta: TypeMeta {
                api_version: "events.k8s.io/v1".to_string(),
                kind: "EventList".to_string(),
            },
            metadata: Some(ListMeta {
                resource_version: Some("12345".to_string()),
                ..Default::default()
            }),
            items: vec![Event {
                type_meta: TypeMeta::default(),
                metadata: Some(ObjectMeta {
                    name: Some("event-1".to_string()),
                    namespace: Some("default".to_string()),
                    ..Default::default()
                }),
                event_time: MicroTime(Utc::now()),
                reporting_controller: "controller-1".to_string(),
                reporting_instance: "instance-1".to_string(),
                action: "Create".to_string(),
                reason: "Created".to_string(),
                regarding: ObjectReference::default(),
                note: "Event 1".to_string(),
                type_: "Normal".to_string(),
                ..Default::default()
            }],
        };

        // Convert to internal and back
        let internal = original.clone().to_internal();
        let mut round_trip = EventList::from_internal(internal);
        round_trip.apply_default();

        // Verify
        assert_eq!(round_trip.items.len(), original.items.len());
        assert_eq!(round_trip.items[0].metadata, original.items[0].metadata);
        assert_eq!(
            round_trip.items[0].reporting_controller,
            original.items[0].reporting_controller
        );
        assert_eq!(round_trip.type_meta.api_version, "events.k8s.io/v1");
        assert_eq!(round_trip.type_meta.kind, "EventList");
    }
}
