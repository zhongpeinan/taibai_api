//! Binding trait tests

use crate::core::v1::Binding;
use crate::generate_trait_tests;

generate_trait_tests!(
    api_version: "v1",
    resources: [Binding]
);
