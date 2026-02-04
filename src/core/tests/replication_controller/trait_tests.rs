//! ReplicationController trait tests

use crate::generate_trait_tests;
use crate::core::v1::{ReplicationController, ReplicationControllerList};

generate_trait_tests!(
    api_version: "v1",
    resources: [ReplicationController],
    list_resources: [ReplicationControllerList]
);



