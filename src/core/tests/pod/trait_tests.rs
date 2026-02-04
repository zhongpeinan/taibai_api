//! Pod trait tests

use crate::generate_trait_tests;
use crate::core::v1::{Pod, PodList};

generate_trait_tests!(
    api_version: "v1",
    resources: [Pod],
    list_resources: [PodList]
);



