//! Service trait tests

use crate::generate_trait_tests;
use crate::core::v1::{Service, ServiceList};

generate_trait_tests!(
    api_version: "v1",
    resources: [Service],
    list_resources: [ServiceList]
);



