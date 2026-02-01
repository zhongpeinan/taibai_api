use crate::common::test_utils::assert_serde_roundtrip;
use crate::common::{ListMeta, ObjectMeta, Quantity, TypeMeta};
use crate::core::v1::{LimitRange, LimitRangeItem, LimitRangeList, LimitRangeSpec, limit_type};
use std::collections::BTreeMap;

fn limit_range_basic() -> LimitRange {
    let max = BTreeMap::from([("cpu".to_string(), Quantity("2".to_string()))]);
    let min = BTreeMap::from([("cpu".to_string(), Quantity("100m".to_string()))]);
    let default = BTreeMap::from([("cpu".to_string(), Quantity("500m".to_string()))]);
    let default_request = BTreeMap::from([("cpu".to_string(), Quantity("250m".to_string()))]);

    LimitRange {
        type_meta: TypeMeta {
            api_version: "v1".to_string(),
            kind: "LimitRange".to_string(),
        },
        metadata: Some(ObjectMeta {
            name: Some("limits".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        spec: Some(LimitRangeSpec {
            limits: vec![LimitRangeItem {
                type_: limit_type::CONTAINER.to_string(),
                max,
                min,
                default,
                default_request,
                max_limit_request_ratio: BTreeMap::new(),
            }],
        }),
    }
}

fn limit_range_list_basic() -> LimitRangeList {
    LimitRangeList {
        type_meta: TypeMeta {
            api_version: "v1".to_string(),
            kind: "LimitRangeList".to_string(),
        },
        metadata: Some(ListMeta {
            resource_version: Some("13".to_string()),
            ..Default::default()
        }),
        items: vec![limit_range_basic()],
    }
}

#[test]
fn serde_roundtrip_limit_range() {
    assert_serde_roundtrip(&limit_range_basic());
}

#[test]
fn serde_roundtrip_limit_range_list() {
    assert_serde_roundtrip(&limit_range_list_basic());
}
