//! Endpoints trait tests

use crate::core::v1::{Endpoints, EndpointsList};
use crate::generate_trait_tests;

generate_trait_tests!(
    api_version: "v1",
    resources: [Endpoints],
    list_resources: [EndpointsList]
);
