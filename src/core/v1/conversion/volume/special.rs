//! Special volume source conversions
//!
//! Includes: CSIVolumeSource, EphemeralVolumeSource, ImageVolumeSource,
//! PersistentVolumeClaimTemplate

use crate::common::traits::{FromInternal, ToInternal};
use crate::core::internal::volume as internal_volume;
use crate::core::v1::volume;

use super::helpers::*;

// ============================================================================
// CSI Volume Source
// ============================================================================

impl ToInternal<internal_volume::CSIVolumeSource> for volume::CSIVolumeSource {
    fn to_internal(self) -> internal_volume::CSIVolumeSource {
        internal_volume::CSIVolumeSource {
            driver: self.driver,
            read_only: self.read_only,
            fs_type: self.fs_type,
            volume_attributes: self.volume_attributes,
            node_publish_secret_ref: self.node_publish_secret_ref,
        }
    }
}

impl FromInternal<internal_volume::CSIVolumeSource> for volume::CSIVolumeSource {
    fn from_internal(value: internal_volume::CSIVolumeSource) -> Self {
        Self {
            driver: value.driver,
            read_only: value.read_only,
            fs_type: value.fs_type,
            volume_attributes: value.volume_attributes,
            node_publish_secret_ref: value.node_publish_secret_ref,
        }
    }
}

// ============================================================================
// Ephemeral Volume Source
// ============================================================================

impl ToInternal<internal_volume::EphemeralVolumeSource> for volume::EphemeralVolumeSource {
    fn to_internal(self) -> internal_volume::EphemeralVolumeSource {
        internal_volume::EphemeralVolumeSource {
            volume_claim_template: self.volume_claim_template.map(|t| t.to_internal()),
        }
    }
}

impl FromInternal<internal_volume::EphemeralVolumeSource> for volume::EphemeralVolumeSource {
    fn from_internal(value: internal_volume::EphemeralVolumeSource) -> Self {
        Self {
            volume_claim_template: value
                .volume_claim_template
                .map(volume::PersistentVolumeClaimTemplate::from_internal),
        }
    }
}

// PersistentVolumeClaimTemplate
impl ToInternal<internal_volume::PersistentVolumeClaimTemplate>
    for volume::PersistentVolumeClaimTemplate
{
    fn to_internal(self) -> internal_volume::PersistentVolumeClaimTemplate {
        use crate::core::v1::conversion::helpers::option_object_meta_to_meta;

        internal_volume::PersistentVolumeClaimTemplate {
            metadata: option_object_meta_to_meta(self.metadata),
            // TODO: Implement PersistentVolumeClaimSpec conversion (deferred for now)
            spec: Some(crate::core::internal::PersistentVolumeClaimSpec::default()),
        }
    }
}

impl FromInternal<internal_volume::PersistentVolumeClaimTemplate>
    for volume::PersistentVolumeClaimTemplate
{
    fn from_internal(value: internal_volume::PersistentVolumeClaimTemplate) -> Self {
        use crate::core::v1::conversion::helpers::meta_to_option_object_meta;

        Self {
            metadata: meta_to_option_object_meta(value.metadata),
            // TODO: Implement PersistentVolumeClaimSpec conversion (deferred for now)
            spec: Some(crate::core::v1::persistent_volume::PersistentVolumeClaimSpec::default()),
        }
    }
}

// ============================================================================
// Image Volume Source
// ============================================================================

impl ToInternal<internal_volume::ImageVolumeSource> for volume::ImageVolumeSource {
    fn to_internal(self) -> internal_volume::ImageVolumeSource {
        internal_volume::ImageVolumeSource {
            reference: self.reference,
            pull_policy: option_string_to_pull_policy(self.pull_policy),
        }
    }
}

impl FromInternal<internal_volume::ImageVolumeSource> for volume::ImageVolumeSource {
    fn from_internal(value: internal_volume::ImageVolumeSource) -> Self {
        Self {
            reference: value.reference,
            pull_policy: pull_policy_to_option_string(value.pull_policy),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csi_volume_source_roundtrip() {
        let v1_csi = volume::CSIVolumeSource {
            driver: "csi-driver".to_string(),
            read_only: Some(true),
            fs_type: Some("ext4".to_string()),
            volume_attributes: std::collections::BTreeMap::new(),
            node_publish_secret_ref: None,
        };

        let internal_csi = v1_csi.clone().to_internal();
        assert_eq!(internal_csi.driver, "csi-driver");
        assert_eq!(internal_csi.read_only, Some(true));
        assert_eq!(internal_csi.fs_type, Some("ext4".to_string()));

        let mut roundtrip = volume::CSIVolumeSource::from_internal(internal_csi);
        assert_eq!(roundtrip.driver, v1_csi.driver);
        assert_eq!(roundtrip.read_only, v1_csi.read_only);
    }

    #[test]
    fn test_image_volume_source_roundtrip() {
        let v1_image = volume::ImageVolumeSource {
            reference: "docker.io/library/nginx:latest".to_string(),
            pull_policy: Some("IfNotPresent".to_string()),
        };

        let internal_image = v1_image.clone().to_internal();
        assert_eq!(internal_image.reference, "docker.io/library/nginx:latest");

        let mut roundtrip = volume::ImageVolumeSource::from_internal(internal_image);
        assert_eq!(roundtrip.reference, v1_image.reference);
        assert_eq!(roundtrip.pull_policy, v1_image.pull_policy);
    }
}
