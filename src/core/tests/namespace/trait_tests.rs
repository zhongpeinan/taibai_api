//! Namespace trait tests

use crate::core::v1::{Namespace, NamespaceList};
use crate::generate_trait_tests;

generate_trait_tests!(
    api_version: "v1",
    resources: [Namespace],
    list_resources: [NamespaceList]
);
