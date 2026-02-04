//! Node trait tests

use crate::generate_trait_tests;
use crate::core::v1::{Node, NodeList};

generate_trait_tests!(
    api_version: "v1",
    resources: [Node],
    list_resources: [NodeList]
);



