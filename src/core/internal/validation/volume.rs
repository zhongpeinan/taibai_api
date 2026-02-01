//! Volume validation wrappers for internal core API types.

use crate::common::FromInternal;
use crate::common::validation::{ErrorList, Path};
use crate::core::internal::Volume;
use crate::core::v1::validation::volume as v1_volume_validation;
use crate::core::v1::volume::{Volume as V1Volume, VolumeSource};
use std::collections::HashMap;

pub fn validate_volumes(
    volumes: &[Volume],
    path: &Path,
) -> (HashMap<String, VolumeSource>, ErrorList) {
    let v1_volumes: Vec<V1Volume> = volumes
        .iter()
        .cloned()
        .map(V1Volume::from_internal)
        .collect();
    v1_volume_validation::validate_volumes(&v1_volumes, path)
}
