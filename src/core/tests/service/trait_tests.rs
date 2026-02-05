//! Service trait tests

use crate::core::v1::{Service, ServiceList};
use crate::generate_trait_tests;

generate_trait_tests!(
    api_version: "v1",
    resources: [Service],
    list_resources: [ServiceList]
);
