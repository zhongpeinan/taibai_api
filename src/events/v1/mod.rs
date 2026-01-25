//! Kubernetes Events API v1 types
//!
//! This module contains the events v1 API types.
//!
//! Source: https://github.com/kubernetes/api/blob/master/events/v1/types.go

use crate::common::{
    ApplyDefault, HasTypeMeta, ListMeta, MicroTime, ObjectMeta, ResourceSchema, Timestamp,
    TypeMeta, VersionedObject,
};
use crate::core::v1::reference::ObjectReference;
use crate::impl_unimplemented_prost_message;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

pub mod conversion;
pub mod validation;

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

impl ApplyDefault for Event {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "events.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "Event".to_string();
        }
    }
}

impl ApplyDefault for EventList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "events.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "EventList".to_string();
        }
    }
}

// ----------------------------------------------------------------------------
// Version Conversion - See conversion.rs module
// ----------------------------------------------------------------------------

// ToInternal and FromInternal are implemented in conversion.rs

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
    use crate::common::{FromInternal, ToInternal};
    use crate::events::internal;

    // ========================================================================
    // Compile-time Trait Checks
    // ========================================================================

    /// 编译时检查：确保顶级资源实现了必需的 traits
    #[test]
    fn top_level_resources_implement_required_traits() {
        fn check<T: VersionedObject + ApplyDefault>() {}

        check::<Event>();
    }

    /// 编译时检查：确保资源实现了版本转换 traits
    #[test]
    fn conversion_traits() {
        fn check<T, I>()
        where
            T: ToInternal<I> + FromInternal<I>,
        {
        }

        check::<Event, internal::Event>();
    }

    /// 编译时检查：确保资源实现了 prost::Message
    #[test]
    fn prost_message() {
        fn check<T: prost::Message>() {}

        check::<Event>();
        check::<EventList>();
    }
}

#[cfg(test)]
mod trait_tests;
