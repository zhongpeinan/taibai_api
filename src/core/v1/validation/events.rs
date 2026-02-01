//! Validation for Event resources.
//!
//! Delegates to internal validation for consistency.

use crate::common::ToInternal;
use crate::common::validation::ErrorList;
use crate::core::internal::validation::events as internal_events_validation;
use crate::core::v1::event::Event;

pub use crate::core::internal::validation::events::{
    EVENT_REQUEST_VERSION_CORE_V1, EVENT_REQUEST_VERSION_EVENTS_V1,
    EVENT_REQUEST_VERSION_EVENTS_V1BETA1, EventRequestVersion,
};

pub fn validate_event_create(event: &Event, request_version: EventRequestVersion) -> ErrorList {
    let internal_event = event.clone().to_internal();
    internal_events_validation::validate_event_create(&internal_event, request_version)
}

pub fn validate_event_update(
    new_event: &Event,
    old_event: &Event,
    request_version: EventRequestVersion,
) -> ErrorList {
    let internal_new = new_event.clone().to_internal();
    let internal_old = old_event.clone().to_internal();
    internal_events_validation::validate_event_update(&internal_new, &internal_old, request_version)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::time::MicroTime;
    use crate::core::v1::event::{Event, EventSeries, EventSource, event_type};
    use crate::core::v1::reference::ObjectReference;
    use chrono::Utc;

    fn base_event() -> Event {
        Event {
            involved_object: ObjectReference::default(),
            ..Default::default()
        }
    }

    #[test]
    fn test_legacy_event_namespace_mismatch() {
        let mut event = base_event();
        event.metadata = Some(crate::common::ObjectMeta {
            namespace: Some("ns-a".to_string()),
            ..Default::default()
        });
        event.involved_object.namespace = Some("ns-b".to_string());

        let errs = validate_event_create(&event, EVENT_REQUEST_VERSION_CORE_V1);
        assert!(!errs.is_empty());
    }

    #[test]
    fn test_new_event_requires_reporting_fields() {
        let mut event = base_event();
        event.metadata = Some(crate::common::ObjectMeta {
            namespace: Some("default".to_string()),
            ..Default::default()
        });
        event.event_time = Some(MicroTime::from_datetime(Utc::now()));

        let errs = validate_event_create(&event, EVENT_REQUEST_VERSION_CORE_V1);
        assert!(!errs.is_empty());
    }

    #[test]
    fn test_strict_event_requires_event_time() {
        let event = base_event();
        let errs = validate_event_create(&event, EVENT_REQUEST_VERSION_EVENTS_V1);
        assert!(!errs.is_empty());
    }

    #[test]
    fn test_strict_event_rejects_legacy_fields() {
        let mut event = base_event();
        event.metadata = Some(crate::common::ObjectMeta {
            name: Some("e".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        });
        event.event_time = Some(MicroTime::from_datetime(Utc::now()));
        event.first_timestamp = Some(crate::common::time::Timestamp::from_datetime(Utc::now()));
        event.type_ = Some(event_type::NORMAL.to_string());
        event.source = Some(EventSource {
            component: Some("kubelet".to_string()),
            ..Default::default()
        });
        event.series = Some(EventSeries::default());

        let errs = validate_event_create(&event, EVENT_REQUEST_VERSION_EVENTS_V1);
        assert!(!errs.is_empty());
    }
}
