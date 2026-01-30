use super::{Event, EventList, EventSeries, EventSource};
use crate::common::test_utils::assert_conversion_roundtrip;
use crate::common::{ApplyDefault, ListMeta, MicroTime, ObjectMeta, Timestamp, TypeMeta};
use crate::core::v1::ObjectReference;
use crate::events::internal;

fn event_basic() -> Event {
    Event {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("event-a".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        event_time: MicroTime::from_str("2024-01-15T10:00:00.123456Z").expect("parse microtime"),
        series: Some(EventSeries {
            count: 3,
            last_observed_time: MicroTime::from_str("2024-01-15T10:00:01.123456Z")
                .expect("parse microtime"),
        }),
        reporting_controller: "kubernetes.io/kubelet".to_string(),
        reporting_instance: "kubelet-xyz".to_string(),
        action: "Started".to_string(),
        reason: "Scheduled".to_string(),
        regarding: ObjectReference {
            kind: Some("Pod".to_string()),
            namespace: Some("default".to_string()),
            name: Some("pod-a".to_string()),
            ..Default::default()
        },
        related: None,
        note: "Started container".to_string(),
        type_: "Normal".to_string(),
        deprecated_source: Some(EventSource {
            component: Some("kubelet".to_string()),
            host: Some("node-a".to_string()),
        }),
        deprecated_first_timestamp: Some(
            Timestamp::from_str("2024-01-15T10:00:00Z").expect("parse timestamp"),
        ),
        deprecated_last_timestamp: Some(
            Timestamp::from_str("2024-01-15T10:00:02Z").expect("parse timestamp"),
        ),
        deprecated_count: 2,
    }
}

fn event_list_basic() -> EventList {
    let mut item = event_basic();
    item.apply_default();
    EventList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("1".to_string()),
            ..Default::default()
        }),
        items: vec![item],
    }
}

#[test]
fn conversion_roundtrip_event() {
    assert_conversion_roundtrip::<Event, internal::Event>(event_basic());
}

#[test]
fn conversion_roundtrip_event_list() {
    assert_conversion_roundtrip::<EventList, internal::EventList>(event_list_basic());
}
