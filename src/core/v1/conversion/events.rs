//! Event conversion implementations
//!
//! Converts between core v1 and internal Event types.
//! Based on k8s.io/kubernetes/pkg/apis/core/v1/conversion.go

use super::helpers::*;
use crate::common::{ApplyDefault, FromInternal, ToInternal};
use crate::core::internal;
use crate::core::v1::event;

// ============================================================================
// Event
// ============================================================================

impl ToInternal<internal::Event> for event::Event {
    fn to_internal(self) -> internal::Event {
        internal::Event {
            metadata: option_object_meta_to_meta(self.metadata),
            involved_object: self.involved_object,
            reason: self.reason.unwrap_or_default(),
            message: self.message.unwrap_or_default(),
            source: self.source.map(|s| s.to_internal()).unwrap_or_default(),
            first_timestamp: self.first_timestamp,
            last_timestamp: self.last_timestamp,
            count: self.count,
            r#type: self.type_.unwrap_or_default(),
            event_time: self.event_time,
            series: self.series.map(|s| s.to_internal()),
            action: self.action.unwrap_or_default(),
            related: self.related,
            reporting_controller: self.reporting_controller.unwrap_or_default(),
            reporting_instance: self.reporting_instance.unwrap_or_default(),
        }
    }
}

impl FromInternal<internal::Event> for event::Event {
    fn from_internal(value: internal::Event) -> Self {
        let mut result = Self {
            type_meta: crate::common::TypeMeta::default(),
            metadata: meta_to_option_object_meta(value.metadata),
            involved_object: value.involved_object,
            reason: if value.reason.is_empty() {
                None
            } else {
                Some(value.reason)
            },
            message: if value.message.is_empty() {
                None
            } else {
                Some(value.message)
            },
            source: {
                let mut source = event::EventSource::from_internal(value.source);
                if source.component.is_none() && source.host.is_none() {
                    None
                } else {
                    Some(source)
                }
            },
            first_timestamp: value.first_timestamp,
            last_timestamp: value.last_timestamp,
            count: value.count,
            type_: if value.r#type.is_empty() {
                None
            } else {
                Some(value.r#type)
            },
            event_time: value.event_time,
            series: value.series.map(event::EventSeries::from_internal),
            action: if value.action.is_empty() {
                None
            } else {
                Some(value.action)
            },
            related: value.related,
            reporting_controller: if value.reporting_controller.is_empty() {
                None
            } else {
                Some(value.reporting_controller)
            },
            reporting_instance: if value.reporting_instance.is_empty() {
                None
            } else {
                Some(value.reporting_instance)
            },
        };

        result
    }
}

// ============================================================================
// EventList
// ============================================================================

impl ToInternal<internal::EventList> for event::EventList {
    fn to_internal(self) -> internal::EventList {
        internal::EventList {
            type_meta: crate::common::TypeMeta::default(),
            metadata: option_list_meta_to_meta(self.metadata),
            items: self.items.into_iter().map(|i| i.to_internal()).collect(),
        }
    }
}

impl FromInternal<internal::EventList> for event::EventList {
    fn from_internal(value: internal::EventList) -> Self {
        let mut result = Self {
            type_meta: crate::common::TypeMeta::default(),
            metadata: meta_to_option_list_meta(value.metadata),
            items: value
                .items
                .into_iter()
                .map(event::Event::from_internal)
                .collect(),
        };

        result
    }
}

// ============================================================================
// EventSource
// ============================================================================

impl ToInternal<internal::EventSource> for event::EventSource {
    fn to_internal(self) -> internal::EventSource {
        internal::EventSource {
            component: self.component.unwrap_or_default(),
            host: self.host.unwrap_or_default(),
        }
    }
}

impl FromInternal<internal::EventSource> for event::EventSource {
    fn from_internal(value: internal::EventSource) -> Self {
        Self {
            component: if value.component.is_empty() {
                None
            } else {
                Some(value.component)
            },
            host: if value.host.is_empty() {
                None
            } else {
                Some(value.host)
            },
        }
    }
}

// ============================================================================
// EventSeries
// ============================================================================

impl ToInternal<internal::EventSeries> for event::EventSeries {
    fn to_internal(self) -> internal::EventSeries {
        internal::EventSeries {
            count: self.count.unwrap_or_default(),
            last_observed_time: self.last_observed_time,
        }
    }
}

impl FromInternal<internal::EventSeries> for event::EventSeries {
    fn from_internal(value: internal::EventSeries) -> Self {
        Self {
            count: if value.count == 0 {
                None
            } else {
                Some(value.count)
            },
            last_observed_time: value.last_observed_time,
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::internal;

    #[test]
    fn test_event_roundtrip() {
        let v1_event = event::Event {
            type_meta: crate::common::TypeMeta {
                api_version: "v1".to_string(),
                kind: "Event".to_string(),
            },
            metadata: Some(crate::common::ObjectMeta {
                name: Some("my-event".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            involved_object: crate::core::v1::reference::ObjectReference {
                kind: Some("Pod".to_string()),
                name: Some("my-pod".to_string()),
                ..Default::default()
            },
            reason: Some("Started".to_string()),
            message: Some("Container started".to_string()),
            source: Some(event::EventSource {
                component: Some("kubelet".to_string()),
                host: Some("node-1".to_string()),
            }),
            first_timestamp: None,
            last_timestamp: None,
            count: 1,
            type_: Some("Normal".to_string()),
            event_time: None,
            series: None,
            action: Some("Started".to_string()),
            related: None,
            reporting_controller: Some("kubelet".to_string()),
            reporting_instance: Some("node-1".to_string()),
        };

        let internal_event = v1_event.clone().to_internal();
        assert_eq!(internal_event.metadata.name, Some("my-event".to_string()));
        assert_eq!(internal_event.reason, "Started");
        assert_eq!(internal_event.message, "Container started");
        assert_eq!(internal_event.source.component, "kubelet");
        assert_eq!(internal_event.source.host, "node-1");
        assert_eq!(internal_event.r#type, "Normal");

        let mut roundtrip = event::Event::from_internal(internal_event);
        roundtrip.apply_default();
        assert_eq!(
            roundtrip.metadata.as_ref().unwrap().name,
            Some("my-event".to_string())
        );
        assert_eq!(roundtrip.reason, Some("Started".to_string()));
        assert_eq!(roundtrip.message, Some("Container started".to_string()));
        assert_eq!(roundtrip.type_, Some("Normal".to_string()));
        assert_eq!(roundtrip.type_meta.api_version, "v1");
        assert_eq!(roundtrip.type_meta.kind, "Event");
    }

    #[test]
    fn test_event_empty_strings_to_none() {
        let internal_event = internal::Event {
            metadata: crate::common::ObjectMeta::default(),
            involved_object: crate::core::v1::reference::ObjectReference::default(),
            reason: String::new(),
            message: String::new(),
            source: internal::EventSource::default(),
            first_timestamp: None,
            last_timestamp: None,
            count: 0,
            r#type: String::new(),
            event_time: None,
            series: None,
            action: String::new(),
            related: None,
            reporting_controller: String::new(),
            reporting_instance: String::new(),
        };

        let mut v1_event = event::Event::from_internal(internal_event);
        v1_event.apply_default();
        assert_eq!(v1_event.reason, None);
        assert_eq!(v1_event.message, None);
        assert_eq!(v1_event.source, None);
        assert_eq!(v1_event.type_, None);
        assert_eq!(v1_event.action, None);
        assert_eq!(v1_event.reporting_controller, None);
        assert_eq!(v1_event.reporting_instance, None);
    }

    #[test]
    fn test_event_source_roundtrip() {
        let v1_source = event::EventSource {
            component: Some("kubelet".to_string()),
            host: Some("node-1".to_string()),
        };

        let internal_source = v1_source.clone().to_internal();
        assert_eq!(internal_source.component, "kubelet");
        assert_eq!(internal_source.host, "node-1");

        let mut roundtrip = event::EventSource::from_internal(internal_source);
        assert_eq!(roundtrip.component, Some("kubelet".to_string()));
        assert_eq!(roundtrip.host, Some("node-1".to_string()));
    }

    #[test]
    fn test_event_series_roundtrip() {
        let v1_series = event::EventSeries {
            count: Some(10),
            last_observed_time: None,
        };

        let internal_series = v1_series.clone().to_internal();
        assert_eq!(internal_series.count, 10);

        let mut roundtrip = event::EventSeries::from_internal(internal_series);
        assert_eq!(roundtrip.count, Some(10));
    }

    #[test]
    fn test_event_list_roundtrip() {
        let v1_list = event::EventList {
            type_meta: crate::common::TypeMeta {
                api_version: "v1".to_string(),
                kind: "EventList".to_string(),
            },
            metadata: Some(crate::common::ListMeta::default()),
            items: vec![
                event::Event {
                    type_meta: crate::common::TypeMeta::default(),
                    metadata: Some(crate::common::ObjectMeta {
                        name: Some("event1".to_string()),
                        ..Default::default()
                    }),
                    involved_object: crate::core::v1::reference::ObjectReference::default(),
                    reason: Some("Started".to_string()),
                    message: None,
                    source: None,
                    first_timestamp: None,
                    last_timestamp: None,
                    count: 1,
                    type_: Some("Normal".to_string()),
                    event_time: None,
                    series: None,
                    action: None,
                    related: None,
                    reporting_controller: None,
                    reporting_instance: None,
                },
                event::Event {
                    type_meta: crate::common::TypeMeta::default(),
                    metadata: Some(crate::common::ObjectMeta {
                        name: Some("event2".to_string()),
                        ..Default::default()
                    }),
                    involved_object: crate::core::v1::reference::ObjectReference::default(),
                    reason: Some("Failed".to_string()),
                    message: None,
                    source: None,
                    first_timestamp: None,
                    last_timestamp: None,
                    count: 1,
                    type_: Some("Warning".to_string()),
                    event_time: None,
                    series: None,
                    action: None,
                    related: None,
                    reporting_controller: None,
                    reporting_instance: None,
                },
            ],
        };

        let internal_list = v1_list.clone().to_internal();
        assert_eq!(internal_list.items.len(), 2);

        let mut roundtrip = event::EventList::from_internal(internal_list);
        roundtrip.apply_default();
        assert_eq!(roundtrip.items.len(), 2);
        assert_eq!(roundtrip.type_meta.api_version, "v1");
        assert_eq!(roundtrip.type_meta.kind, "EventList");
    }
}
