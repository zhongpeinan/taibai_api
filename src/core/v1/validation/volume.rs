//! Volume validation
//!
//! Validates Volume and VolumeMount specifications.

use crate::common::validation::{ErrorList, Path};
use crate::core::v1::Volume;

/// Validates a list of volumes
pub fn validate_volumes(_volumes: &[Volume], _path: &Path) -> ErrorList {
    ErrorList::new()
    // TODO: Implement in Phase 2
}

/// Validates a single volume
pub fn validate_volume(_volume: &Volume, _path: &Path) -> ErrorList {
    ErrorList::new()
    // TODO: Implement in Phase 2
}
