//! Storage validation (PersistentVolume and PersistentVolumeClaim)

use crate::common::validation::ErrorList;
use crate::core::v1::{PersistentVolume, PersistentVolumeClaim};

/// Validates a PersistentVolume
pub fn validate_persistent_volume(_pv: &PersistentVolume) -> ErrorList {
    ErrorList::new()
    // TODO: Implement in Phase 4
}

/// Validates a PersistentVolumeClaim
pub fn validate_persistent_volume_claim(_pvc: &PersistentVolumeClaim) -> ErrorList {
    ErrorList::new()
    // TODO: Implement in Phase 4
}

/// Validates PersistentVolume update
pub fn validate_persistent_volume_update(
    _new: &PersistentVolume,
    _old: &PersistentVolume,
) -> ErrorList {
    ErrorList::new()
    // TODO: Implement in Phase 4
}

/// Validates PersistentVolumeClaim update
pub fn validate_persistent_volume_claim_update(
    _new: &PersistentVolumeClaim,
    _old: &PersistentVolumeClaim,
) -> ErrorList {
    ErrorList::new()
    // TODO: Implement in Phase 4
}
