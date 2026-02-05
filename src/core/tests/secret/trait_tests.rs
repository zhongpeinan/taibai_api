//! Secret trait tests

use crate::core::v1::{Secret, SecretList};
use crate::generate_trait_tests;

generate_trait_tests!(
    api_version: "v1",
    resources: [Secret],
    list_resources: [SecretList]
);
