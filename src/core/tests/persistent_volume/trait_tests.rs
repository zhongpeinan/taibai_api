//! PersistentVolume trait tests

use crate::core::v1::{PersistentVolume, PersistentVolumeList};
use crate::generate_trait_tests;

generate_trait_tests!(
    api_version: "v1",
    resources: [PersistentVolume],
    list_resources: [PersistentVolumeList]
);
