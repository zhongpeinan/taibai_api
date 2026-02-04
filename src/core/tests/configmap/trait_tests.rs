//! ConfigMap trait tests

use crate::generate_trait_tests;
use crate::core::v1::{ConfigMap, ConfigMapList};

generate_trait_tests!(
    api_version: "v1",
    resources: [ConfigMap],
    list_resources: [ConfigMapList]
);



