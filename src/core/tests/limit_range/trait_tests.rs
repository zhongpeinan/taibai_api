//! LimitRange trait tests

use crate::core::v1::{LimitRange, LimitRangeList};
use crate::generate_trait_tests;

generate_trait_tests!(
    api_version: "v1",
    resources: [LimitRange],
    list_resources: [LimitRangeList]
);
