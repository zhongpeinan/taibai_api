//! Volume validation wrappers for internal core API types.

use crate::common::FromInternal;
use crate::common::validation::{ErrorList, Path};
use crate::core::internal::InternalContainer as Container;
use crate::core::internal::Volume;
use crate::core::v1::validation::volume as v1_volume_validation;
use crate::core::v1::volume::{Volume as V1Volume, VolumeSource};
use crate::core::v1::volume::{VolumeDevice, VolumeMount};
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

pub fn validate_volume_mounts(
    mounts: &[VolumeMount],
    vol_devices: &HashMap<String, String>,
    volumes: &HashMap<String, VolumeSource>,
    container: &Container,
    path: &Path,
) -> ErrorList {
    v1_volume_validation::validate_volume_mounts(mounts, vol_devices, volumes, container, path)
}

pub fn validate_volume_devices(
    devices: &[VolumeDevice],
    vol_mounts: &HashMap<String, String>,
    volumes: &HashMap<String, VolumeSource>,
    path: &Path,
) -> ErrorList {
    v1_volume_validation::validate_volume_devices(devices, vol_mounts, volumes, path)
}
