//! Volume validation for core v1 API.

use crate::common::validation::{ErrorList, Path};
use crate::common::{FromInternal, ToInternal};
use crate::core::internal::validation::volume as internal_volume_validation;
use crate::core::v1::pod::Container;
use crate::core::v1::volume::{Volume, VolumeDevice, VolumeMount, VolumeSource};
use std::collections::HashMap;

/// Validates a list of volumes.
///
/// Returns a map of volume names to their sources for reference by volume mounts.
pub fn validate_volumes(
    volumes: &[Volume],
    path: &Path,
) -> (HashMap<String, VolumeSource>, ErrorList) {
    let internal_volumes: Vec<crate::core::internal::Volume> =
        volumes.iter().cloned().map(|v| v.to_internal()).collect();
    let (internal_map, errs) =
        internal_volume_validation::validate_volumes(&internal_volumes, path);
    let v1_map = internal_map
        .into_iter()
        .map(|(name, source)| (name, VolumeSource::from_internal(source)))
        .collect();
    (v1_map, errs)
}

/// Validates a single volume.
pub fn validate_volume(volume: &Volume, path: &Path) -> ErrorList {
    let internal_volume = volume.clone().to_internal();
    internal_volume_validation::validate_volume(&internal_volume, path)
}

/// Validates volume mounts.
pub fn validate_volume_mounts(
    mounts: &[VolumeMount],
    vol_devices: &HashMap<String, String>,
    volumes: &HashMap<String, VolumeSource>,
    container: &Container,
    path: &Path,
) -> ErrorList {
    let internal_volumes: HashMap<String, crate::core::internal::VolumeSource> = volumes
        .iter()
        .map(|(name, source)| (name.clone(), source.clone().to_internal()))
        .collect();
    internal_volume_validation::validate_volume_mounts(
        mounts,
        vol_devices,
        &internal_volumes,
        container,
        path,
    )
}

/// Validates volume devices.
pub fn validate_volume_devices(
    devices: &[VolumeDevice],
    vol_mounts: &HashMap<String, String>,
    volumes: &HashMap<String, VolumeSource>,
    path: &Path,
) -> ErrorList {
    let internal_volumes: HashMap<String, crate::core::internal::VolumeSource> = volumes
        .iter()
        .map(|(name, source)| (name.clone(), source.clone().to_internal()))
        .collect();
    internal_volume_validation::validate_volume_devices(
        devices,
        vol_mounts,
        &internal_volumes,
        path,
    )
}
