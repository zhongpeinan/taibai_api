//! Kubernetes Events API types
//!
//! This module contains the events API types.

pub mod internal;
pub mod v1;

pub use internal::{Event as EventInternal, EventList as EventListInternal};
pub use v1::{Event, EventList, EventSeries, EventSource};

// Re-export constants at module level
pub use v1::event_type;
