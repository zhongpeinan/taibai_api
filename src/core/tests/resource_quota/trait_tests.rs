//! ResourceQuota trait tests

use crate::core::v1::{ResourceQuota, ResourceQuotaList};
use crate::generate_trait_tests;

generate_trait_tests!(
    api_version: "v1",
    resources: [ResourceQuota],
    list_resources: [ResourceQuotaList]
);
