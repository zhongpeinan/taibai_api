use super::{Event, EventList, EventSeries, EventSource};
use crate::common::test_utils::assert_serde_roundtrip;
use crate::common::{ListMeta, MicroTime, ObjectMeta, Timestamp, TypeMeta};
use crate::core::v1::ObjectReference;

fn event_basic() -> Event {
    Event {
        type_meta: TypeMeta {
            api_version: "events.k8s.io/v1".to_string(),
            kind: "Event".to_string(),
        },
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
    EventList {
        type_meta: TypeMeta {
            api_version: "events.k8s.io/v1".to_string(),
            kind: "EventList".to_string(),
        },
        metadata: Some(ListMeta {
            resource_version: Some("1".to_string()),
            ..Default::default()
        }),
        items: vec![event_basic()],
    }
}

#[test]
fn serde_roundtrip_event() {
    assert_serde_roundtrip(&event_basic());
}

#[test]
fn serde_roundtrip_event_list() {
    assert_serde_roundtrip(&event_list_basic());
}
