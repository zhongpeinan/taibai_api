//! Pod trait tests

use crate::core::v1::{Pod, PodList};
use crate::generate_trait_tests;

generate_trait_tests!(
    api_version: "v1",
    resources: [Pod],
    list_resources: [PodList]
);
