//! Namespace trait tests

use crate::generate_trait_tests;
use crate::core::v1::{Namespace, NamespaceList};

generate_trait_tests!(
    api_version: "v1",
    resources: [Namespace],
    list_resources: [NamespaceList]
);



