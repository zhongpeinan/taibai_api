//! Kubernetes Events API Internal Types
//!
//! This module contains type definitions from k8s.io/kubernetes/pkg/apis/events
//!
//! The internal events API re-exports the core Event types.
//!
//! Source: https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/events/register.go

use crate::common::{ListMeta, TypeMeta};
use serde::{Deserialize, Serialize};

// Re-export core Event types for internal use
pub use crate::core::internal::{Event, EventSeries, EventSource};

// EventList is not in core::internal, so we define it here
/// EventList is a list of Event objects.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct EventList {
    /// TypeMeta describes the type of this object.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    pub metadata: ListMeta,
    /// List of events.
    pub items: Vec<Event>,
}
