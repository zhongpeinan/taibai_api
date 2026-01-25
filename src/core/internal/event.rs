//! Event types from the Kubernetes Core API
//!
//! This module contains types for Kubernetes events.

use crate::common::{ListMeta, ObjectMeta, TypeMeta, time::Timestamp};
use crate::core::v1::reference::ObjectReference;
use crate::impl_has_object_meta;
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

/// EventList is a list of events.
///
/// Corresponds to [Kubernetes EventList](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L6240)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ListMeta,
    /// List of events.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Event>,
}

#[cfg(test)]
mod tests {}
