//! Kubernetes Events API v1 types
//!
//! This module contains the events v1 API types.
//!
//! Source: https://github.com/kubernetes/api/blob/master/events/v1/types.go

use crate::common::{
    ApplyDefaults, HasTypeMeta, ListMeta, MicroTime, ObjectMeta, ResourceSchema, Timestamp,
    TypeMeta, UnimplementedConversion, VersionedObject,
};
use crate::core::v1::reference::ObjectReference;
use crate::impl_unimplemented_prost_message;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

// ============================================================================
// Event
// ============================================================================

/// Event is a report of an event somewhere in the cluster.
///
/// It generally denotes some state change in the system. Events have a limited
/// retention time and triggers and messages may evolve with time.
///
/// Corresponds to [Kubernetes Event](https://github.com/kubernetes/api/blob/master/events/v1/types.go#L28)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    /// TypeMeta for this resource
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// eventTime is the time when this Event was first observed. It is required.
    #[serde(default)]
    pub event_time: MicroTime,

    /// series is data about the Event series this event represents or nil if it's a singleton Event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub series: Option<EventSeries>,

    /// reportingController is the name of the controller that emitted this Event,
    /// e.g. `kubernetes.io/kubelet`. This field cannot be empty for new Events.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reporting_controller: String,

    /// reportingInstance is the ID of the controller instance, e.g. `kubelet-xyzf`.
    /// This field cannot be empty for new Events and it can have at most 128 characters.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reporting_instance: String,

    /// action is what action was taken/failed regarding to the regarding object.
    /// It is machine-readable. This field cannot be empty for new Events and it can have at most 128 characters.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub action: String,

    /// reason is why the action was taken. It is human-readable.
    /// This field cannot be empty for new Events and it can have at most 128 characters.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,

    /// regarding contains the object this Event is about. In most cases it's an Object
    /// reporting controller implements, e.g. ReplicaSetController implements ReplicaSets
    /// and this event is emitted because it acts on some changes in a ReplicaSet object.
    #[serde(default)]
    pub regarding: ObjectReference,

    /// related is the optional secondary object for more complex actions.
    /// E.g. when regarding object triggers a creation or deletion of related object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related: Option<ObjectReference>,

    /// note is a human-readable description of the status of this operation.
    /// Maximal length of the note is 1kB, but libraries should be prepared to handle values up to 64kB.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub note: String,

    /// type is the type of this event (Normal, Warning), new types could be added in the future.
    /// It is machine-readable. This field cannot be empty for new Events.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub type_: String,

    /// deprecatedSource is the deprecated field assuring backward compatibility with core.v1 Event type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deprecated_source: Option<EventSource>,

    /// deprecatedFirstTimestamp is the deprecated field assuring backward compatibility with core.v1 Event type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deprecated_first_timestamp: Option<Timestamp>,

    /// deprecatedLastTimestamp is the deprecated field assuring backward compatibility with core.v1 Event type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deprecated_last_timestamp: Option<Timestamp>,

    /// deprecatedCount is the deprecated field assuring backward compatibility with core.v1 Event type.
    #[serde(default)]
    pub deprecated_count: i32,
}

// ============================================================================
// EventSeries
// ============================================================================

/// EventSeries contain information on series of events, i.e. thing that was/is happening
/// continuously for some time.
///
/// Corresponds to [Kubernetes EventSeries](https://github.com/kubernetes/api/blob/master/events/v1/types.go#L101)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventSeries {
    /// count is the number of occurrences in this series up to the last heartbeat time.
    pub count: i32,

    /// lastObservedTime is the time when last Event from the series was seen before last heartbeat.
    pub last_observed_time: MicroTime,
}

// ============================================================================
// EventList
// ============================================================================

/// EventList is a list of Event objects.
///
/// Corresponds to [Kubernetes EventList](https://github.com/kubernetes/api/blob/master/events/v1/types.go#L115)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventList {
    /// TypeMeta for this resource
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// items is a list of schema objects.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Event>,
}

// ============================================================================
// EventSource (for backward compatibility)
// ============================================================================

/// EventSource contains information for an event.
///
/// This is kept for backward compatibility with core.v1 Event type.
///
/// Corresponds to [Kubernetes EventSource](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7512)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventSource {
    /// Component from which the event is generated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,

    /// Node name on which the event is generated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
}

// ============================================================================
// Constants
// ============================================================================

/// Event type constants
pub mod event_type {
    /// Information only and will not cause any problems
    pub const NORMAL: &str = "Normal";
    /// These events are to warn that something might go wrong
    pub const WARNING: &str = "Warning";
}

// ============================================================================
// Trait Implementations for Event and EventList
// ============================================================================

// ----------------------------------------------------------------------------
// ResourceSchema Implementation
// ----------------------------------------------------------------------------

impl ResourceSchema for Event {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "events.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "Event"
    }
    fn resource(_: &Self::Meta) -> &str {
        "events"
    }

    fn group_static() -> &'static str {
        "events.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "Event"
    }
    fn resource_static() -> &'static str {
        "events"
    }
}

impl ResourceSchema for EventList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "events.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "EventList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "events"
    }

    fn group_static() -> &'static str {
        "events.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "EventList"
    }
    fn resource_static() -> &'static str {
        "events"
    }
}

// ----------------------------------------------------------------------------
// HasTypeMeta Implementation
// ----------------------------------------------------------------------------

impl HasTypeMeta for Event {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for EventList {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

// ----------------------------------------------------------------------------
// VersionedObject Implementation
// ----------------------------------------------------------------------------

impl VersionedObject for Event {
    fn metadata(&self) -> &ObjectMeta {
        self.metadata
            .as_ref()
            .unwrap_or_else(|| static_default_object_meta())
    }

    fn metadata_mut(&mut self) -> &mut ObjectMeta {
        self.metadata.get_or_insert_with(ObjectMeta::default)
    }
}

// Helper function for static default ObjectMeta
fn static_default_object_meta() -> &'static ObjectMeta {
    static DEFAULT: OnceLock<ObjectMeta> = OnceLock::new();
    DEFAULT.get_or_init(ObjectMeta::default)
}

// Note: EventList does not implement VersionedObject because its metadata is ListMeta

// ----------------------------------------------------------------------------
// ApplyDefaults Implementation
// ----------------------------------------------------------------------------

impl ApplyDefaults for Event {
    fn apply_defaults(&mut self) {
        if self.type_meta.api_version.is_none() {
            self.type_meta.api_version = Some("events.k8s.io/v1".to_string());
        }
        if self.type_meta.kind.is_none() {
            self.type_meta.kind = Some("Event".to_string());
        }
    }
}

impl ApplyDefaults for EventList {
    fn apply_defaults(&mut self) {
        if self.type_meta.api_version.is_none() {
            self.type_meta.api_version = Some("events.k8s.io/v1".to_string());
        }
        if self.type_meta.kind.is_none() {
            self.type_meta.kind = Some("EventList".to_string());
        }
    }
}

// ----------------------------------------------------------------------------
// Version Conversion Placeholder (using UnimplementedConversion)
// ----------------------------------------------------------------------------

impl UnimplementedConversion for Event {}

// ----------------------------------------------------------------------------
// Protobuf Placeholder (using macro)
// ----------------------------------------------------------------------------

impl_unimplemented_prost_message!(Event);
impl_unimplemented_prost_message!(EventList);

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================================
    // Event Tests
    // ============================================================================

    #[test]
    fn test_event_with_minimal_fields() {
        let event = Event {
            event_time: MicroTime::from_str("2024-01-15T10:00:00.123456Z").unwrap(),
            reporting_controller: "kubelet".to_string(),
            reporting_instance: "kubelet-xyzf".to_string(),
            action: "Started".to_string(),
            reason: "Started".to_string(),
            regarding: ObjectReference {
                kind: Some("Pod".to_string()),
                name: Some("my-pod".to_string()),
                ..Default::default()
            },
            type_: event_type::NORMAL.to_string(),
            ..Default::default()
        };
        assert_eq!(event.reporting_controller, "kubelet");
        assert_eq!(event.regarding.kind, Some("Pod".to_string()));
    }

    #[test]
    fn test_event_serialize() {
        let event = Event {
            metadata: None,
            event_time: MicroTime::from_str("2024-01-15T10:00:00.123456Z").unwrap(),
            series: None,
            reporting_controller: "kubelet".to_string(),
            reporting_instance: "kubelet-node-1".to_string(),
            action: "Started".to_string(),
            reason: "Started".to_string(),
            regarding: ObjectReference {
                kind: Some("Pod".to_string()),
                name: Some("my-pod".to_string()),
                ..Default::default()
            },
            related: None,
            note: "Container started successfully".to_string(),
            type_: event_type::NORMAL.to_string(),
            ..Default::default()
        };
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("\"eventTime\":\"2024-01-15T10:00:00.123456Z\""));
        assert!(json.contains("\"reportingController\":\"kubelet\""));
        assert!(json.contains("\"regarding\""));
        assert!(json.contains("\"note\":\"Container started successfully\""));
    }

    #[test]
    fn test_event_deserialize() {
        let json = r#"{
            "eventTime": "2024-01-15T10:00:00.123456Z",
            "reportingController": "kubelet",
            "reportingInstance": "kubelet-node-1",
            "action": "Started",
            "reason": "Started",
            "regarding": {"kind": "Pod", "name": "my-pod"},
            "note": "Container started",
            "type": "Normal"
        }"#;
        let event: Event = serde_json::from_str(json).unwrap();
        assert_eq!(event.reporting_controller, "kubelet");
        assert_eq!(event.regarding.kind, Some("Pod".to_string()));
        assert_eq!(event.note, "Container started");
        assert_eq!(event.type_, "Normal");
    }

    #[test]
    fn test_event_round_trip() {
        let original = Event {
            metadata: Some(ObjectMeta {
                name: Some("my-event".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            event_time: MicroTime::from_str("2024-01-15T10:00:00.123456Z").unwrap(),
            series: Some(EventSeries {
                count: 5,
                last_observed_time: MicroTime::from_str("2024-01-15T10:05:00.123456Z").unwrap(),
            }),
            reporting_controller: "kubelet".to_string(),
            reporting_instance: "kubelet-node-1".to_string(),
            action: "Started".to_string(),
            reason: "Started".to_string(),
            regarding: ObjectReference {
                kind: Some("Pod".to_string()),
                name: Some("my-pod".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            },
            related: Some(ObjectReference {
                kind: Some("Node".to_string()),
                name: Some("node-1".to_string()),
                ..Default::default()
            }),
            note: "Container started successfully".to_string(),
            type_: event_type::NORMAL.to_string(),
            ..Default::default()
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Event = serde_json::from_str(&json).unwrap();
        assert_eq!(original.metadata, deserialized.metadata);
        assert_eq!(original.series, deserialized.series);
        assert_eq!(
            original.reporting_controller,
            deserialized.reporting_controller
        );
    }

    #[test]
    fn test_event_with_series() {
        let event = Event {
            event_time: MicroTime::from_str("2024-01-15T10:00:00.123456Z").unwrap(),
            series: Some(EventSeries {
                count: 10,
                last_observed_time: MicroTime::from_str("2024-01-15T10:05:00.123456Z").unwrap(),
            }),
            reporting_controller: "kubelet".to_string(),
            reporting_instance: "kubelet-node-1".to_string(),
            action: "Started".to_string(),
            reason: "Started".to_string(),
            regarding: ObjectReference {
                kind: Some("Pod".to_string()),
                ..Default::default()
            },
            type_: event_type::NORMAL.to_string(),
            ..Default::default()
        };
        assert!(event.series.is_some());
        assert_eq!(event.series.as_ref().unwrap().count, 10);
    }

    #[test]
    fn test_event_with_related_object() {
        let event = Event {
            event_time: MicroTime::from_str("2024-01-15T10:00:00.123456Z").unwrap(),
            reporting_controller: "kubelet".to_string(),
            reporting_instance: "kubelet-node-1".to_string(),
            action: "Scheduled".to_string(),
            reason: "Scheduled".to_string(),
            regarding: ObjectReference {
                kind: Some("Pod".to_string()),
                name: Some("my-pod".to_string()),
                ..Default::default()
            },
            related: Some(ObjectReference {
                kind: Some("Node".to_string()),
                name: Some("node-1".to_string()),
                ..Default::default()
            }),
            type_: event_type::NORMAL.to_string(),
            ..Default::default()
        };
        assert!(event.related.is_some());
        assert_eq!(
            event.related.as_ref().unwrap().kind,
            Some("Node".to_string())
        );
    }

    #[test]
    fn test_event_with_deprecated_fields() {
        let event = Event {
            event_time: MicroTime::from_str("2024-01-15T10:00:00.123456Z").unwrap(),
            reporting_controller: "kubelet".to_string(),
            reporting_instance: "kubelet-node-1".to_string(),
            action: "Started".to_string(),
            reason: "Started".to_string(),
            regarding: ObjectReference {
                kind: Some("Pod".to_string()),
                ..Default::default()
            },
            type_: event_type::NORMAL.to_string(),
            deprecated_source: Some(EventSource {
                component: Some("kubelet".to_string()),
                host: Some("node-1".to_string()),
            }),
            deprecated_first_timestamp: Some(Timestamp::from_str("2024-01-15T10:00:00Z").unwrap()),
            deprecated_last_timestamp: Some(Timestamp::from_str("2024-01-15T11:00:00Z").unwrap()),
            deprecated_count: 3,
            ..Default::default()
        };
        assert!(event.deprecated_source.is_some());
        assert_eq!(event.deprecated_count, 3);
    }

    // ============================================================================
    // EventSeries Tests
    // ============================================================================

    #[test]
    fn test_event_series_default() {
        let series = EventSeries::default();
        assert_eq!(series.count, 0);
        assert_eq!(series.last_observed_time.timestamp(), 0);
    }

    #[test]
    fn test_event_series_with_fields() {
        let series = EventSeries {
            count: 10,
            last_observed_time: MicroTime::from_str("2024-01-15T10:00:00.123456Z").unwrap(),
        };
        assert_eq!(series.count, 10);
        assert_eq!(
            &series.last_observed_time.to_rfc3339(),
            "2024-01-15T10:00:00.123456Z"
        );
    }

    #[test]
    fn test_event_series_serialize() {
        let series = EventSeries {
            count: 5,
            last_observed_time: MicroTime::from_str("2024-01-15T10:00:00.123456Z").unwrap(),
        };
        let json = serde_json::to_string(&series).unwrap();
        assert!(json.contains("\"count\":5"));
        assert!(json.contains("\"lastObservedTime\":\"2024-01-15T10:00:00.123456"));
    }

    #[test]
    fn test_event_series_deserialize() {
        let json = r#"{"count":5,"lastObservedTime":"2024-01-15T10:00:00.123456Z"}"#;
        let series: EventSeries = serde_json::from_str(json).unwrap();
        assert_eq!(series.count, 5);
        assert_eq!(
            &series.last_observed_time.to_rfc3339(),
            "2024-01-15T10:00:00.123456Z"
        );
    }

    #[test]
    fn test_event_series_round_trip() {
        let original = EventSeries {
            count: 10,
            last_observed_time: MicroTime::from_str("2024-01-15T10:00:00.123456Z").unwrap(),
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: EventSeries = serde_json::from_str(&json).unwrap();
        assert_eq!(original.count, deserialized.count);
        assert_eq!(original.last_observed_time, deserialized.last_observed_time);
    }

    // ============================================================================
    // EventList Tests
    // ============================================================================

    #[test]
    fn test_event_list_default() {
        let list = EventList::default();
        assert!(list.metadata.is_none());
        assert!(list.items.is_empty());
    }

    #[test]
    fn test_event_list_empty() {
        let list = EventList {
            metadata: None,
            items: vec![],
        };
        assert!(list.items.is_empty());
        // Empty vectors should be skipped during serialization
        let json = serde_json::to_string(&list).unwrap();
        assert!(!json.contains("items"));
    }

    #[test]
    fn test_event_list_with_items() {
        let event1 = Event {
            event_time: MicroTime::from_str("2024-01-15T10:00:00.123456Z").unwrap(),
            reporting_controller: "kubelet".to_string(),
            reporting_instance: "kubelet-node-1".to_string(),
            action: "Started".to_string(),
            reason: "Started".to_string(),
            regarding: ObjectReference {
                kind: Some("Pod".to_string()),
                name: Some("pod-1".to_string()),
                ..Default::default()
            },
            type_: event_type::NORMAL.to_string(),
            ..Default::default()
        };
        let event2 = Event {
            event_time: MicroTime::from_str("2024-01-15T10:01:00.123456Z").unwrap(),
            reporting_controller: "scheduler".to_string(),
            reporting_instance: "scheduler-0".to_string(),
            action: "Scheduled".to_string(),
            reason: "Scheduled".to_string(),
            regarding: ObjectReference {
                kind: Some("Pod".to_string()),
                name: Some("pod-2".to_string()),
                ..Default::default()
            },
            type_: event_type::NORMAL.to_string(),
            ..Default::default()
        };
        let list = EventList {
            metadata: Some(ListMeta {
                resource_version: Some("12345".to_string()),
                ..Default::default()
            }),
            items: vec![event1, event2],
        };
        assert_eq!(list.items.len(), 2);
    }

    #[test]
    fn test_event_list_serialize() {
        let list = EventList {
            metadata: Some(ListMeta {
                resource_version: Some("12345".to_string()),
                ..Default::default()
            }),
            items: vec![Event {
                metadata: Some(ObjectMeta {
                    name: Some("my-event".to_string()),
                    ..Default::default()
                }),
                event_time: MicroTime::from_str("2024-01-15T10:00:00.123456Z").unwrap(),
                reporting_controller: "kubelet".to_string(),
                reporting_instance: "kubelet-node-1".to_string(),
                action: "Started".to_string(),
                reason: "Started".to_string(),
                regarding: ObjectReference {
                    kind: Some("Pod".to_string()),
                    name: Some("my-pod".to_string()),
                    ..Default::default()
                },
                type_: event_type::NORMAL.to_string(),
                ..Default::default()
            }],
        };
        let json = serde_json::to_string(&list).unwrap();
        assert!(json.contains("\"resourceVersion\":\"12345\""));
        assert!(json.contains("\"my-event\""));
    }

    #[test]
    fn test_event_list_deserialize() {
        let json = r#"{"items":[{"metadata":{"name":"event-1"},"eventTime":"2024-01-15T10:00:00.123456Z","reportingController":"kubelet","action":"Started","reason":"Started","regarding":{"kind":"Pod","name":"pod-1"},"type":"Normal"}]}"#;
        let list: EventList = serde_json::from_str(json).unwrap();
        assert_eq!(list.items.len(), 1);
        assert_eq!(
            list.items[0].metadata.as_ref().unwrap().name,
            Some("event-1".to_string())
        );
    }

    // ============================================================================
    // EventSource Tests
    // ============================================================================

    #[test]
    fn test_event_source_default() {
        let source = EventSource::default();
        assert!(source.component.is_none());
        assert!(source.host.is_none());
    }

    #[test]
    fn test_event_source_with_fields() {
        let source = EventSource {
            component: Some("kubelet".to_string()),
            host: Some("node-1".to_string()),
        };
        assert_eq!(source.component, Some("kubelet".to_string()));
        assert_eq!(source.host, Some("node-1".to_string()));
    }

    #[test]
    fn test_event_source_serialize() {
        let source = EventSource {
            component: Some("kubelet".to_string()),
            host: Some("node-1".to_string()),
        };
        let json = serde_json::to_string(&source).unwrap();
        assert!(json.contains("\"component\":\"kubelet\""));
        assert!(json.contains("\"host\":\"node-1\""));
    }

    // ============================================================================
    // Constants Tests
    // ============================================================================

    #[test]
    fn test_event_type_constants() {
        assert_eq!(event_type::NORMAL, "Normal");
        assert_eq!(event_type::WARNING, "Warning");
    }

    // ============================================================================
    // MicroTime Integration Tests
    // ============================================================================

    #[test]
    fn test_event_with_micro_time() {
        let event = Event {
            event_time: MicroTime::from_str("2024-01-15T10:00:00.123456Z").unwrap(),
            reporting_controller: "kubelet".to_string(),
            reporting_instance: "kubelet-node-1".to_string(),
            action: "Started".to_string(),
            reason: "Started".to_string(),
            regarding: ObjectReference {
                kind: Some("Pod".to_string()),
                ..Default::default()
            },
            type_: event_type::NORMAL.to_string(),
            ..Default::default()
        };
        assert_eq!(event.event_time.to_rfc3339(), "2024-01-15T10:00:00.123456Z");
    }

    #[test]
    fn test_event_series_micro_time() {
        let series = EventSeries {
            count: 5,
            last_observed_time: MicroTime::from_str("2024-01-15T10:00:00.123456Z").unwrap(),
        };
        assert_eq!(
            &series.last_observed_time.to_rfc3339(),
            "2024-01-15T10:00:00.123456Z"
        );
    }
}
