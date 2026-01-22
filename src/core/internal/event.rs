//! Event types from the Kubernetes Core API
//!
//! This module contains types for Kubernetes events.

use crate::common::{ObjectMeta, time::Timestamp};
use crate::impl_has_object_meta;
use crate::core::v1::reference::ObjectReference;
use serde::{Deserialize, Serialize};

/// Event is a report of an event somewhere in the cluster.
///
/// Corresponds to [Kubernetes Event](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L6160)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    /// Standard object metadata.
    pub metadata: ObjectMeta,
    /// The object that this event is about.
    pub involved_object: ObjectReference,
    /// Short, machine understandable string that gives the reason for this event.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    /// Human-readable description of the status of this operation.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
    /// The component reporting this event.
    #[serde(default)]
    pub source: EventSource,
    /// The time at which the event was first recorded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_timestamp: Option<Timestamp>,
    /// The time at which the most recent occurrence of this event was recorded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_timestamp: Option<Timestamp>,
    /// The number of times this event has occurred.
    #[serde(default)]
    pub count: i32,
    /// Type of this event (Normal, Warning).
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub r#type: String,
    /// Time when this Event was first observed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_time: Option<Timestamp>,
    /// Data about the Event series this event represents.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub series: Option<EventSeries>,
    /// What action was taken/failed regarding to the Regarding object.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub action: String,
    /// Optional secondary object for more complex actions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related: Option<ObjectReference>,
    /// Name of the controller that emitted this Event.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reporting_controller: String,
    /// ID of the controller instance.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reporting_instance: String,
}
    impl_has_object_meta!(Event);

/// EventSeries contains information about a series of events.
///
/// Corresponds to [Kubernetes EventSeries](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L6226)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventSeries {
    /// Number of occurrences in this series.
    #[serde(default)]
    pub count: i32,
    /// Time of the last occurrence observed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_observed_time: Option<Timestamp>,
}

/// EventSource represents the event source.
///
/// Corresponds to [Kubernetes EventSource](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L6134)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventSource {
    /// Component from which the event is generated.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub component: String,
    /// Node name on which the event is generated.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub host: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_default() {
        let event = Event::default();
        assert!(event.metadata.is_none());
        assert!(event.reason.is_empty());
        assert!(event.message.is_empty());
        assert_eq!(event.count, 0);
    }

    #[test]
    fn test_event_with_fields() {
        let event = Event {
            metadata: None,
            involved_object: ObjectReference {
                kind: Some("Pod".to_string()),
                name: Some("pod-1".to_string()),
                ..Default::default()
            },
            reason: "Started".to_string(),
            message: "Pod was started".to_string(),
            source: EventSource {
                component: "kubelet".to_string(),
                host: "node-1".to_string(),
            },
            first_timestamp: Some(Timestamp::from_str("2024-01-15T10:00:00Z").unwrap()),
            last_timestamp: Some(Timestamp::from_str("2024-01-15T10:00:00Z").unwrap()),
            count: 1,
            r#type: "Normal".to_string(),
            event_time: None,
            series: None,
            action: String::new(),
            related: None,
            reporting_controller: String::new(),
            reporting_instance: String::new(),
        };

        assert_eq!(event.reason, "Started");
        assert_eq!(event.message, "Pod was started");
        assert_eq!(event.source.component, "kubelet");
        assert_eq!(event.count, 1);
    }

    #[test]
    fn test_event_serialize() {
        let event = Event {
            metadata: None,
            involved_object: ObjectReference {
                kind: Some("Node".to_string()),
                name: Some("node-1".to_string()),
                ..Default::default()
            },
            reason: "Failed".to_string(),
            message: "Container failed".to_string(),
            source: EventSource::default(),
            first_timestamp: None,
            last_timestamp: None,
            count: 1,
            r#type: "Warning".to_string(),
            event_time: None,
            series: None,
            action: String::new(),
            related: None,
            reporting_controller: String::new(),
            reporting_instance: String::new(),
        };

        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("\"reason\":\"Failed\""));
        assert!(json.contains("\"type\":\"Warning\""));
        assert!(json.contains("\"count\":1"));
    }

    #[test]
    fn test_event_deserialize() {
        let json =
            r#"{"involvedObject":{"kind":"Pod","name":"pod-1"},"reason":"Created","count":1}"#;
        let event: Event = serde_json::from_str(json).unwrap();

        assert_eq!(event.involved_object.kind, Some("Pod".to_string()));
        assert_eq!(event.involved_object.name, Some("pod-1".to_string()));
        assert_eq!(event.reason, "Created");
    }

    #[test]
    fn test_event_round_trip() {
        let original = Event {
            metadata: None,
            involved_object: ObjectReference {
                kind: Some("Pod".to_string()),
                name: Some("test-pod".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            },
            reason: "Pulling".to_string(),
            message: "Pulling image".to_string(),
            source: EventSource {
                component: "kubelet".to_string(),
                host: "node-1".to_string(),
            },
            first_timestamp: Some(Timestamp::from_str("2024-01-15T09:00:00Z").unwrap()),
            last_timestamp: Some(Timestamp::from_str("2024-01-15T10:00:00Z").unwrap()),
            count: 3,
            r#type: "Normal".to_string(),
            event_time: Some(Timestamp::from_str("2024-01-15T10:00:00Z").unwrap()),
            series: None,
            action: "Pulling".to_string(),
            related: None,
            reporting_controller: "kubelet".to_string(),
            reporting_instance: "kubelet-node-1".to_string(),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Event = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_event_series_default() {
        let series = EventSeries::default();
        assert_eq!(series.count, 0);
        assert!(series.last_observed_time.is_none());
    }

    #[test]
    fn test_event_series_with_fields() {
        let series = EventSeries {
            count: 5,
            last_observed_time: Some(Timestamp::from_str("2024-01-15T10:00:00Z").unwrap()),
        };

        assert_eq!(series.count, 5);
        assert!(
            series
                .last_observed_time
                .as_ref()
                .unwrap()
                .to_rfc3339()
                .contains("2024-01-15")
        );
    }

    #[test]
    fn test_event_series_serialize() {
        let series = EventSeries {
            count: 10,
            last_observed_time: Some(Timestamp::from_str("2024-01-15T11:00:00Z").unwrap()),
        };

        let json = serde_json::to_string(&series).unwrap();
        assert!(json.contains("\"count\":10"));
        assert!(json.contains("\"lastObservedTime\":\"2024-01-15T11:00:00"));
    }

    #[test]
    fn test_event_series_deserialize() {
        let json = r#"{"count":3,"lastObservedTime":"2024-01-15T12:00:00Z"}"#;
        let series: EventSeries = serde_json::from_str(json).unwrap();

        assert_eq!(series.count, 3);
        assert_eq!(
            series.last_observed_time.as_ref().unwrap().to_rfc3339(),
            "2024-01-15T12:00:00Z"
        );
    }

    #[test]
    fn test_event_series_round_trip() {
        let original = EventSeries {
            count: 7,
            last_observed_time: Some(Timestamp::from_str("2024-01-15T13:00:00Z").unwrap()),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: EventSeries = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_event_source_default() {
        let source = EventSource::default();
        assert!(source.component.is_empty());
        assert!(source.host.is_empty());
    }

    #[test]
    fn test_event_source_with_fields() {
        let source = EventSource {
            component: "scheduler".to_string(),
            host: "control-plane".to_string(),
        };

        assert_eq!(source.component, "scheduler");
        assert_eq!(source.host, "control-plane");
    }

    #[test]
    fn test_event_source_serialize() {
        let source = EventSource {
            component: "kube-controller-manager".to_string(),
            host: String::new(),
        };

        let json = serde_json::to_string(&source).unwrap();
        assert!(json.contains("\"component\":\"kube-controller-manager\""));
        // empty host should be omitted
        assert!(!json.contains("\"host\""));
    }

    #[test]
    fn test_event_source_deserialize() {
        let json = r#"{"component":"kubelet","host":"node-1"}"#;
        let source: EventSource = serde_json::from_str(json).unwrap();

        assert_eq!(source.component, "kubelet");
        assert_eq!(source.host, "node-1");
    }

    #[test]
    fn test_event_source_round_trip() {
        let original = EventSource {
            component: "kube-scheduler".to_string(),
            host: "master-1".to_string(),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: EventSource = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
    }
}
