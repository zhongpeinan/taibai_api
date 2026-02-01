//! Kubernetes Events API Internal Types
//!
//! This module contains type definitions from k8s.io/kubernetes/pkg/apis/events
//!
//! The internal events API re-exports the core Event types.
//!
//! Source: https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/events/register.go

use crate::common::{ListMeta, TypeMeta};
use crate::impl_unimplemented_prost_message;
use serde::{Deserialize, Serialize};

pub mod validation;

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

// ===========================================================================
// Protobuf Placeholder Implementations
// ===========================================================================

impl_unimplemented_prost_message!(EventList);

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // Compile-time Trait Checks
    // ========================================================================

    /// 编译时检查：确保内部版本 EventList 实现了 prost::Message
    ///
    /// Note: Event is re-exported from core::internal, which should already
    /// implement prost::Message.
    #[test]
    fn prost_message() {
        fn check<T: prost::Message>() {}

        check::<EventList>();
    }
}
