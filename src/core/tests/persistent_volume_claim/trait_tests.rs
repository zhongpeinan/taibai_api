//! PersistentVolumeClaim trait tests

use crate::core::v1::{PersistentVolumeClaim, PersistentVolumeClaimList};
use crate::generate_trait_tests;

generate_trait_tests!(
    api_version: "v1",
    resources: [PersistentVolumeClaim],
    list_resources: [PersistentVolumeClaimList]
);
