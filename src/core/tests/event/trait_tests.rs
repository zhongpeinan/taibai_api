//! Event trait tests

use crate::generate_trait_tests;
use crate::core::v1::{Event, EventList};

generate_trait_tests!(
    api_version: "v1",
    resources: [Event],
    list_resources: [EventList]
);



