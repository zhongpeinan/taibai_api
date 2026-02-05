//! ReplicationController trait tests

use crate::core::v1::{ReplicationController, ReplicationControllerList};
use crate::generate_trait_tests;

generate_trait_tests!(
    api_version: "v1",
    resources: [ReplicationController],
    list_resources: [ReplicationControllerList]
);
