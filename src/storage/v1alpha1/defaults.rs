//! Defaulting functions for storage/v1alpha1 API types
//!
//! Ported from k8s.io/kubernetes/pkg/apis/storage/v1alpha1/zz_generated.defaults.go

use super::VolumeAttachment;

/// Apply defaults to VolumeAttachment.
pub fn set_defaults_volume_attachment(_obj: &mut VolumeAttachment) {
    // Inline volume spec defaults are not modeled in the minimal PersistentVolumeSpec.
}
