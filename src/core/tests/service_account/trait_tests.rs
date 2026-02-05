//! ServiceAccount trait tests

use crate::core::v1::{ServiceAccount, ServiceAccountList};
use crate::generate_trait_tests;

generate_trait_tests!(
    api_version: "v1",
    resources: [ServiceAccount],
    list_resources: [ServiceAccountList]
);
