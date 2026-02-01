use crate::common::test_utils::assert_conversion_roundtrip;
use crate::common::{ApplyDefault, ListMeta, ObjectMeta, Quantity, TypeMeta};
use crate::core::internal;
use crate::core::v1::{LimitRange, LimitRangeItem, LimitRangeList, LimitRangeSpec, limit_type};
use std::collections::BTreeMap;

fn limit_range_basic() -> LimitRange {
    let max = BTreeMap::from([("cpu".to_string(), Quantity("2".to_string()))]);
    let min = BTreeMap::from([("cpu".to_string(), Quantity("100m".to_string()))]);
    let default = BTreeMap::from([("cpu".to_string(), Quantity("500m".to_string()))]);
    let default_request = BTreeMap::from([("cpu".to_string(), Quantity("250m".to_string()))]);

    LimitRange {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("limits".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        spec: Some(LimitRangeSpec {
            limits: vec![LimitRangeItem {
                type_: limit_type::CONTAINER.to_string(),
                max: max.clone(),
                min: min.clone(),
                default: default.clone(),
                default_request,
                max_limit_request_ratio: BTreeMap::new(),
            }],
        }),
    }
}

fn limit_range_list_basic() -> LimitRangeList {
    let mut item = limit_range_basic();
    item.apply_default();
    LimitRangeList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("13".to_string()),
            ..Default::default()
        }),
        items: vec![item],
    }
}

#[test]
fn conversion_roundtrip_limit_range() {
    assert_conversion_roundtrip::<LimitRange, internal::quota::LimitRange>(limit_range_basic());
}

#[test]
fn conversion_roundtrip_limit_range_list() {
    assert_conversion_roundtrip::<LimitRangeList, internal::quota::LimitRangeList>(
        limit_range_list_basic(),
    );
}
