use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::Debug;

use crate::common::{ApplyDefault, FromInternal, ToInternal};

pub fn assert_serde_roundtrip<T>(value: &T)
where
    T: Serialize + DeserializeOwned + PartialEq + Debug,
{
    let json1 = serde_json::to_string(value).expect("serialize to json");
    let restored: T = serde_json::from_str(&json1).expect("deserialize from json");
    assert_eq!(value, &restored, "serde roundtrip mismatch");

    let json2 = serde_json::to_string(&restored).expect("serialize to json again");
    assert_eq!(json1, json2, "serde output not stable");
}

pub fn assert_conversion_roundtrip<V, I>(value: V)
where
    V: ApplyDefault + Clone + PartialEq + Debug + ToInternal<I> + FromInternal<I>,
{
    let mut original = value.clone();
    original.apply_default();

    let internal = original.clone().to_internal();
    let mut back = V::from_internal(internal);
    back.apply_default();

    assert_eq!(original, back, "conversion roundtrip mismatch");
}
