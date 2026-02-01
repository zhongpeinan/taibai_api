//! Validation wrappers for Kubernetes Events API v1 types.
//!
//! These wrappers convert v1 types to internal types before validation.

use crate::common::ToInternal;
use crate::common::validation::ErrorList;
use crate::events::internal;

use super::{Event, EventList};

// ============================================================================
// Event Validation
// ============================================================================

/// Validates a v1 Event by converting to internal and delegating validation.
pub fn validate_event(event: &Event) -> ErrorList {
    let internal_event = event.clone().to_internal();
    internal::validation::validate_event(&internal_event)
}

/// Validates a v1 EventList by converting to internal and delegating validation.
pub fn validate_event_list(list: &EventList) -> ErrorList {
    let internal_list = list.clone().to_internal();
    internal::validation::validate_event_list(&internal_list)
}
