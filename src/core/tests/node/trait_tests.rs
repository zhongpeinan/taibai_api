//! Node trait tests

use crate::core::v1::{Node, NodeList};
use crate::generate_trait_tests;

generate_trait_tests!(
    api_version: "v1",
    resources: [Node],
    list_resources: [NodeList]
);
