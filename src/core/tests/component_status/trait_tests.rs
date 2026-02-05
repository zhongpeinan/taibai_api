//! ComponentStatus trait tests

use crate::core::v1::{ComponentStatus, ComponentStatusList};
use crate::generate_trait_tests;

generate_trait_tests!(
    api_version: "v1",
    resources: [ComponentStatus],
    list_resources: [ComponentStatusList]
);
