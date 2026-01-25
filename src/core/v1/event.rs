//! Event types from the Kubernetes Core v1 API
//!
//! This module contains types for Kubernetes events.

use crate::common::{
    ApplyDefault, HasTypeMeta, ListMeta, ObjectMeta, ResourceSchema, Timestamp, TypeMeta,
};
use crate::core::v1::reference::ObjectReference;
use crate::{impl_unimplemented_prost_message, impl_versioned_object};
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
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
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

impl_versioned_object!(Event);

/// EventList is a list of events.
///
/// Corresponds to [Kubernetes EventList](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7620)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EventList {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,

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

// ============================================================================
// Trait Implementations for Event Resources
// ============================================================================

// ----------------------------------------------------------------------------
// ResourceSchema Implementation
// ----------------------------------------------------------------------------

impl ResourceSchema for Event {
    type Meta = ();
    fn group(_: &Self::Meta) -> &str {
        ""
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
        ""
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
        ""
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
        ""
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

// Note: VersionedObject for Event is implemented by impl_versioned_object! macro
// Note: EventList does not implement VersionedObject because its metadata is ListMeta

// ----------------------------------------------------------------------------
// ApplyDefaults Implementation
// ----------------------------------------------------------------------------

impl ApplyDefault for Event {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "Event".to_string();
        }
    }
}

impl ApplyDefault for EventList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "EventList".to_string();
        }
    }
}

// ----------------------------------------------------------------------------
// Protobuf Placeholder
// ----------------------------------------------------------------------------

impl_unimplemented_prost_message!(Event);
impl_unimplemented_prost_message!(EventList);

#[cfg(test)]
mod tests {}
