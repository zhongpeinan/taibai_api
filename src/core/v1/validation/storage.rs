//! PersistentVolume and PersistentVolumeClaim validation for Kubernetes core/v1 API.

use crate::common::ToInternal;
use crate::common::validation::{ErrorList, Path};
use crate::core::internal::validation::storage as internal_storage_validation;
use crate::core::v1::persistent_volume::{
    PersistentVolume, PersistentVolumeClaim, PersistentVolumeClaimSpec, PersistentVolumeSpec,
};

/// Validates a PersistentVolume.
pub fn validate_persistent_volume(pv: &PersistentVolume, path: &Path) -> ErrorList {
    let internal_pv = pv.clone().to_internal();
    internal_storage_validation::validate_persistent_volume(&internal_pv, path)
}

/// Validates a PersistentVolumeSpec.
pub fn validate_persistent_volume_spec(spec: &PersistentVolumeSpec, path: &Path) -> ErrorList {
    let internal_spec = spec.clone().to_internal();
    internal_storage_validation::validate_persistent_volume_spec(&internal_spec, path)
}

/// Validates a PersistentVolume update.
pub fn validate_persistent_volume_update(
    new_pv: &PersistentVolume,
    old_pv: &PersistentVolume,
    path: &Path,
) -> ErrorList {
    let internal_new = new_pv.clone().to_internal();
    let internal_old = old_pv.clone().to_internal();
    internal_storage_validation::validate_persistent_volume_update(
        &internal_new,
        &internal_old,
        path,
    )
}

/// Validates a PersistentVolumeClaim.
pub fn validate_persistent_volume_claim(pvc: &PersistentVolumeClaim, path: &Path) -> ErrorList {
    let internal_pvc = pvc.clone().to_internal();
    internal_storage_validation::validate_persistent_volume_claim(&internal_pvc, path)
}

/// Validates a PersistentVolumeClaimSpec.
pub fn validate_persistent_volume_claim_spec(
    spec: &PersistentVolumeClaimSpec,
    path: &Path,
) -> ErrorList {
    let internal_spec = spec.clone().to_internal();
    internal_storage_validation::validate_persistent_volume_claim_spec(&internal_spec, path)
}

/// Validates a PersistentVolumeClaim update.
pub fn validate_persistent_volume_claim_update(
    new_pvc: &PersistentVolumeClaim,
    old_pvc: &PersistentVolumeClaim,
    path: &Path,
) -> ErrorList {
    let internal_new = new_pvc.clone().to_internal();
    let internal_old = old_pvc.clone().to_internal();
    internal_storage_validation::validate_persistent_volume_claim_update(
        &internal_new,
        &internal_old,
        path,
    )
}
