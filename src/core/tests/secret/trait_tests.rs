//! Secret trait tests

use crate::generate_trait_tests;
use crate::core::v1::{Secret, SecretList};

generate_trait_tests!(
    api_version: "v1",
    resources: [Secret],
    list_resources: [SecretList]
);



