//! Binding trait tests

use crate::generate_trait_tests;
use crate::core::v1::Binding;

generate_trait_tests!(
    api_version: "v1",
    resources: [Binding]
);

