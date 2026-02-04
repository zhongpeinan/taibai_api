//! PersistentVolume trait tests

use crate::generate_trait_tests;
use crate::core::v1::{PersistentVolume, PersistentVolumeList};

generate_trait_tests!(
    api_version: "v1",
    resources: [PersistentVolume],
    list_resources: [PersistentVolumeList]
);



