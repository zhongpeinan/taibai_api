use crate::common::test_utils::assert_conversion_roundtrip;
use crate::common::{ApplyDefault, ListMeta, MicroTime, ObjectMeta, Timestamp, TypeMeta};
use crate::core::internal;
use crate::core::v1::{Event, EventList, EventSeries, EventSource, ObjectReference};

fn event_basic() -> Event {
    Event {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("event-a".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        involved_object: ObjectReference {
            kind: Some("Pod".to_string()),
            namespace: Some("default".to_string()),
            name: Some("pod-a".to_string()),
            uid: Some("uid-1".to_string()),
            ..Default::default()
        },
        reason: Some("Started".to_string()),
        message: Some("Container started".to_string()),
        source: Some(EventSource {
            component: Some("kubelet".to_string()),
            host: Some("node-a".to_string()),
        }),
        first_timestamp: Some(
            Timestamp::from_str("2024-01-15T10:00:00Z").expect("parse timestamp"),
        ),
        last_timestamp: Some(Timestamp::from_str("2024-01-15T10:00:02Z").expect("parse timestamp")),
        count: 2,
        type_: Some("Normal".to_string()),
        event_time: Some(
            MicroTime::from_str("2024-01-15T10:00:00.123456Z").expect("parse microtime"),
        ),
        series: Some(EventSeries {
            count: Some(3),
            last_observed_time: Some(
                MicroTime::from_str("2024-01-15T10:00:01.123456Z").expect("parse microtime"),
            ),
        }),
        action: Some("Started".to_string()),
        related: Some(ObjectReference {
            kind: Some("Node".to_string()),
            name: Some("node-a".to_string()),
            ..Default::default()
        }),
        reporting_controller: Some("kubernetes.io/kubelet".to_string()),
        reporting_instance: Some("kubelet-xyz".to_string()),
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
