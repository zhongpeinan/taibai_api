//! ConfigMap trait tests

use crate::core::v1::{ConfigMap, ConfigMapList};
use crate::generate_trait_tests;

generate_trait_tests!(
    api_version: "v1",
    resources: [ConfigMap],
    list_resources: [ConfigMapList]
);
