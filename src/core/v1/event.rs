//! Event types from the Kubernetes Core v1 API
//!
//! This module contains types for Kubernetes events.

use crate::common::{ListMeta, ObjectMeta, Timestamp};
use crate::core::v1::reference::ObjectReference;
use serde::{Deserialize, Serialize};

/// EventSource contains information for an event.
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

/// EventSeries contains information about a series of events.
///
/// Corresponds to [Kubernetes EventSeries](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7607)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventSeries {
    /// Number of occurrences in this series up to the last heartbeat time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,

    /// Time of the last occurrence observed.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "lastObservedTime"
    )]
    pub last_observed_time: Option<Timestamp>,
}

/// Event is a report of an event somewhere in the cluster.
///
/// Corresponds to [Kubernetes Event](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7540)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// The object that this event is about.
    pub involved_object: ObjectReference,

    /// Short, machine understandable string for the reason.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    /// Human-readable description of the status of this operation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// The component reporting this event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<EventSource>,

    /// The time at which the event was first recorded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_timestamp: Option<Timestamp>,

    /// The time at which the most recent occurrence of this event was recorded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_timestamp: Option<Timestamp>,

    /// The number of times this event has occurred.
    #[serde(default)]
    pub count: i32,

    /// Type of this event (Normal, Warning), new types could be added in the future.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<String>,

    /// Time when this Event was first observed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_time: Option<Timestamp>,

    /// Data about the Event series this event represents.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub series: Option<EventSeries>,

    /// What action was taken/failed regarding to the regarding object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,

    /// Optional secondary object for more complex actions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related: Option<ObjectReference>,

    /// Name of the controller that emitted this Event.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "reportingComponent"
    )]
    pub reporting_controller: Option<String>,

    /// ID of the controller instance.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "reportingInstance"
    )]
    pub reporting_instance: Option<String>,
}

/// EventList is a list of events.
///
/// Corresponds to [Kubernetes EventList](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7620)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EventList {
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// List of events.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Event>,
}

/// Event type constants
pub mod event_type {
    /// Information only and will not cause any problems
    pub const NORMAL: &str = "Normal";
    /// These events are to warn that something might go wrong
    pub const WARNING: &str = "Warning";
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_event_source_deserialize() {
        let json = r#"{"component":"kubelet","host":"node-1"}"#;
        let source: EventSource = serde_json::from_str(json).unwrap();
        assert_eq!(source.component, Some("kubelet".to_string()));
        assert_eq!(source.host, Some("node-1".to_string()));
    }

    #[test]
    fn test_event_series_default() {
        let series = EventSeries::default();
        assert!(series.count.is_none());
        assert!(series.last_observed_time.is_none());
    }

    #[test]
    fn test_event_series_with_fields() {
        let series = EventSeries {
            count: Some(5),
            last_observed_time: Some(Timestamp::from_str("2024-01-15T10:00:00.123456Z").unwrap()),
        };
        assert_eq!(series.count, Some(5));
        assert_eq!(
            series.last_observed_time,
            Some(Timestamp::from_str("2024-01-15T10:00:00.123456Z").unwrap())
        );
    }

    #[test]
    fn test_event_series_serialize() {
        let series = EventSeries {
            count: Some(5),
            last_observed_time: Some(Timestamp::from_str("2024-01-15T10:00:00Z").unwrap()),
        };
        let json = serde_json::to_string(&series).unwrap();
        assert!(json.contains("\"count\":5"));
        assert!(json.contains("\"lastObservedTime\":\"2024-01-15T10:00:00Z\""));
    }

    #[test]
    fn test_event_series_deserialize() {
        let json = r#"{"count":5,"lastObservedTime":"2024-01-15T10:00:00.123456Z"}"#;
        let series: EventSeries = serde_json::from_str(json).unwrap();
        assert_eq!(series.count, Some(5));
        assert_eq!(
            series.last_observed_time,
            Some(Timestamp::from_str("2024-01-15T10:00:00.123456Z").unwrap())
        );
    }

    #[test]
    fn test_event_with_metadata() {
        let metadata = ObjectMeta {
            name: Some("my-event".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        };
        let event = Event {
            metadata: Some(metadata),
            involved_object: ObjectReference {
                kind: Some("Pod".to_string()),
                name: Some("my-pod".to_string()),
                ..Default::default()
            },
            count: 0,
            reason: None,
            message: None,
            source: None,
            first_timestamp: None,
            last_timestamp: None,
            type_: None,
            event_time: None,
            series: None,
            action: None,
            related: None,
            reporting_controller: None,
            reporting_instance: None,
        };
        assert_eq!(
            event.metadata.as_ref().unwrap().name,
            Some("my-event".to_string())
        );
    }

    #[test]
    fn test_event_with_involved_object() {
        let event = Event {
            involved_object: ObjectReference {
                kind: Some("Pod".to_string()),
                name: Some("my-pod".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            },
            count: 0,
            metadata: None,
            reason: None,
            message: None,
            source: None,
            first_timestamp: None,
            last_timestamp: None,
            type_: None,
            event_time: None,
            series: None,
            action: None,
            related: None,
            reporting_controller: None,
            reporting_instance: None,
        };
        assert_eq!(event.involved_object.kind, Some("Pod".to_string()));
        assert_eq!(event.involved_object.name, Some("my-pod".to_string()));
    }

    #[test]
    fn test_event_with_timestamps() {
        let event = Event {
            involved_object: ObjectReference {
                kind: Some("Pod".to_string()),
                ..Default::default()
            },
            first_timestamp: Some(Timestamp::from_str("2024-01-15T10:00:00Z").unwrap()),
            last_timestamp: Some(Timestamp::from_str("2024-01-15T11:00:00Z").unwrap()),
            event_time: Some(Timestamp::from_str("2024-01-15T10:00:00.123456Z").unwrap()),
            count: 1,
            metadata: None,
            reason: None,
            message: None,
            source: None,
            type_: None,
            series: None,
            action: None,
            related: None,
            reporting_controller: None,
            reporting_instance: None,
        };
        assert_eq!(
            event.first_timestamp,
            Some(Timestamp::from_str("2024-01-15T10:00:00Z").unwrap())
        );
        assert_eq!(
            event.last_timestamp,
            Some(Timestamp::from_str("2024-01-15T11:00:00Z").unwrap())
        );
        assert_eq!(
            event.event_time,
            Some(Timestamp::from_str("2024-01-15T10:00:00.123456Z").unwrap())
        );
    }

    #[test]
    fn test_event_serialize() {
        let event = Event {
            metadata: Some(ObjectMeta {
                name: Some("my-event".to_string()),
                ..Default::default()
            }),
            involved_object: ObjectReference {
                kind: Some("Pod".to_string()),
                name: Some("my-pod".to_string()),
                ..Default::default()
            },
            reason: Some("Started".to_string()),
            type_: Some(event_type::NORMAL.to_string()),
            count: 1,
            message: None,
            source: None,
            first_timestamp: None,
            last_timestamp: None,
            event_time: None,
            series: None,
            action: None,
            related: None,
            reporting_controller: None,
            reporting_instance: None,
        };
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("\"name\":\"my-event\""));
        assert!(json.contains("\"kind\":\"Pod\""));
        assert!(json.contains("\"reason\":\"Started\""));
        assert!(json.contains("\"type\":\"Normal\""));
    }

    #[test]
    fn test_event_deserialize() {
        let json = r#"{
            "metadata": {"name": "my-event"},
            "involvedObject": {"kind": "Pod", "name": "my-pod"},
            "reason": "Started",
            "message": "Container started",
            "type": "Normal",
            "count": 1
        }"#;
        let event: Event = serde_json::from_str(json).unwrap();
        assert_eq!(
            event.metadata.as_ref().unwrap().name,
            Some("my-event".to_string())
        );
        assert_eq!(event.involved_object.kind, Some("Pod".to_string()));
        assert_eq!(event.reason, Some("Started".to_string()));
        assert_eq!(event.message, Some("Container started".to_string()));
        assert_eq!(event.type_, Some(event_type::NORMAL.to_string()));
    }

    #[test]
    fn test_event_round_trip() {
        let original = Event {
            metadata: Some(ObjectMeta {
                name: Some("my-event".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            involved_object: ObjectReference {
                kind: Some("Pod".to_string()),
                name: Some("my-pod".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            },
            reason: Some("Started".to_string()),
            message: Some("Container started successfully".to_string()),
            source: Some(EventSource {
                component: Some("kubelet".to_string()),
                host: Some("node-1".to_string()),
            }),
            first_timestamp: Some(Timestamp::from_str("2024-01-15T10:00:00Z").unwrap()),
            last_timestamp: Some(Timestamp::from_str("2024-01-15T11:00:00Z").unwrap()),
            count: 3,
            type_: Some(event_type::NORMAL.to_string()),
            event_time: Some(Timestamp::from_str("2024-01-15T10:00:00Z").unwrap()),
            series: None,
            action: Some("Started".to_string()),
            related: None,
            reporting_controller: Some("kubelet".to_string()),
            reporting_instance: Some("kubelet-node-1".to_string()),
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Event = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_event_list_default() {
        let list = EventList {
            metadata: None,
            items: vec![],
        };
        assert!(list.metadata.is_none());
        assert!(list.items.is_empty());
    }

    #[test]
    fn test_event_list_with_items() {
        let event1 = Event {
            metadata: Some(ObjectMeta {
                name: Some("event-1".to_string()),
                ..Default::default()
            }),
            involved_object: ObjectReference {
                kind: Some("Pod".to_string()),
                name: Some("pod-1".to_string()),
                ..Default::default()
            },
            count: 0,
            reason: None,
            message: None,
            source: None,
            first_timestamp: None,
            last_timestamp: None,
            type_: None,
            event_time: None,
            series: None,
            action: None,
            related: None,
            reporting_controller: None,
            reporting_instance: None,
        };
        let event2 = Event {
            metadata: Some(ObjectMeta {
                name: Some("event-2".to_string()),
                ..Default::default()
            }),
            involved_object: ObjectReference {
                kind: Some("Pod".to_string()),
                name: Some("pod-2".to_string()),
                ..Default::default()
            },
            count: 0,
            reason: None,
            message: None,
            source: None,
            first_timestamp: None,
            last_timestamp: None,
            type_: None,
            event_time: None,
            series: None,
            action: None,
            related: None,
            reporting_controller: None,
            reporting_instance: None,
        };
        let list = EventList {
            metadata: Some(ListMeta {
                resource_version: Some("12345".to_string()),
                ..Default::default()
            }),
            items: vec![event1, event2],
        };
        assert_eq!(list.items.len(), 2);
        assert_eq!(
            list.metadata.as_ref().unwrap().resource_version,
            Some("12345".to_string())
        );
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
                involved_object: ObjectReference {
                    kind: Some("Pod".to_string()),
                    name: Some("my-pod".to_string()),
                    ..Default::default()
                },
                count: 0,
                reason: None,
                message: None,
                source: None,
                first_timestamp: None,
                last_timestamp: None,
                type_: None,
                event_time: None,
                series: None,
                action: None,
                related: None,
                reporting_controller: None,
                reporting_instance: None,
            }],
        };
        let json = serde_json::to_string(&list).unwrap();
        assert!(json.contains("\"resourceVersion\":\"12345\""));
        assert!(json.contains("\"name\":\"my-event\""));
    }

    #[test]
    fn test_event_type_constants() {
        assert_eq!(event_type::NORMAL, "Normal");
        assert_eq!(event_type::WARNING, "Warning");
    }

    #[test]
    fn test_event_series_round_trip() {
        let original = EventSeries {
            count: Some(10),
            last_observed_time: Some(Timestamp::from_str("2024-01-15T10:00:00Z").unwrap()),
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: EventSeries = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_event_source_round_trip() {
        let original = EventSource {
            component: Some("kubelet".to_string()),
            host: Some("node-1".to_string()),
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: EventSource = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_event_with_event_series() {
        let event = Event {
            metadata: Some(ObjectMeta {
                name: Some("my-event".to_string()),
                ..Default::default()
            }),
            involved_object: ObjectReference {
                kind: Some("Pod".to_string()),
                name: Some("my-pod".to_string()),
                ..Default::default()
            },
            count: 5,
            series: Some(EventSeries {
                count: Some(10),
                last_observed_time: Some(
                    Timestamp::from_str("2024-01-15T10:00:00.123456Z").unwrap(),
                ),
            }),
            reason: None,
            message: None,
            source: None,
            first_timestamp: None,
            last_timestamp: None,
            type_: None,
            event_time: None,
            action: None,
            related: None,
            reporting_controller: None,
            reporting_instance: None,
        };
        assert!(event.series.is_some());
        assert_eq!(event.series.as_ref().unwrap().count, Some(10));
    }

    #[test]
    fn test_event_with_related_object() {
        let event = Event {
            involved_object: ObjectReference {
                kind: Some("Pod".to_string()),
                name: Some("my-pod".to_string()),
                ..Default::default()
            },
            related: Some(ObjectReference {
                kind: Some("Node".to_string()),
                name: Some("node-1".to_string()),
                ..Default::default()
            }),
            count: 0,
            metadata: None,
            reason: None,
            message: None,
            source: None,
            first_timestamp: None,
            last_timestamp: None,
            type_: None,
            event_time: None,
            series: None,
            action: None,
            reporting_controller: None,
            reporting_instance: None,
        };
        assert!(event.related.is_some());
        assert_eq!(
            event.related.as_ref().unwrap().kind,
            Some("Node".to_string())
        );
    }
}
