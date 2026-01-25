//! Projected volume source conversions
//!
//! Includes: ProjectedVolumeSource, VolumeProjection, SecretProjection,
//! ConfigMapProjection, DownwardAPIProjection, ServiceAccountTokenProjection

use crate::common::traits::{FromInternal, ToInternal};
use crate::core::internal::volume as internal_volume;
use crate::core::v1::volume;

// ============================================================================
// ProjectedVolumeSource
// ============================================================================

impl ToInternal<internal_volume::ProjectedVolumeSource> for volume::ProjectedVolumeSource {
    fn to_internal(self) -> internal_volume::ProjectedVolumeSource {
        internal_volume::ProjectedVolumeSource {
            sources: self.sources.into_iter().map(|s| s.to_internal()).collect(),
            default_mode: self.default_mode,
        }
    }
}

impl FromInternal<internal_volume::ProjectedVolumeSource> for volume::ProjectedVolumeSource {
    fn from_internal(value: internal_volume::ProjectedVolumeSource) -> Self {
        Self {
            sources: value
                .sources
                .into_iter()
                .map(volume::VolumeProjection::from_internal)
                .collect(),
            default_mode: value.default_mode,
        }
    }
}

// ============================================================================
// VolumeProjection
// ============================================================================

impl ToInternal<internal_volume::VolumeProjection> for volume::VolumeProjection {
    fn to_internal(self) -> internal_volume::VolumeProjection {
        // Note: cluster_trust_bundle and pod_certificate don't exist in internal,
        // they are v1-only fields and are dropped during conversion
        internal_volume::VolumeProjection {
            secret: self.secret.map(|s| s.to_internal()),
            downward_api: self.downward_api.map(|d| d.to_internal()),
            config_map: self.config_map.map(|c| c.to_internal()),
            service_account_token: self.service_account_token.map(|s| s.to_internal()),
        }
    }
}

impl FromInternal<internal_volume::VolumeProjection> for volume::VolumeProjection {
    fn from_internal(value: internal_volume::VolumeProjection) -> Self {
        Self {
            secret: value.secret.map(volume::SecretProjection::from_internal),
            downward_api: value
                .downward_api
                .map(volume::DownwardAPIProjection::from_internal),
            config_map: value
                .config_map
                .map(volume::ConfigMapProjection::from_internal),
            service_account_token: value
                .service_account_token
                .map(volume::ServiceAccountTokenProjection::from_internal),
            cluster_trust_bundle: None, // v1-only field, not in internal
            pod_certificate: None,      // v1-only field, not in internal
        }
    }
}

// ============================================================================
// Projection Types
// ============================================================================

// SecretProjection
impl ToInternal<internal_volume::SecretProjection> for volume::SecretProjection {
    fn to_internal(self) -> internal_volume::SecretProjection {
        internal_volume::SecretProjection {
            name: self.name,
            items: self.items,
            optional: self.optional,
        }
    }
}

impl FromInternal<internal_volume::SecretProjection> for volume::SecretProjection {
    fn from_internal(value: internal_volume::SecretProjection) -> Self {
        Self {
            name: value.name,
            items: value.items,
            optional: value.optional,
        }
    }
}

// ConfigMapProjection
impl ToInternal<internal_volume::ConfigMapProjection> for volume::ConfigMapProjection {
    fn to_internal(self) -> internal_volume::ConfigMapProjection {
        internal_volume::ConfigMapProjection {
            name: self.name,
            items: self.items,
            optional: self.optional,
        }
    }
}

impl FromInternal<internal_volume::ConfigMapProjection> for volume::ConfigMapProjection {
    fn from_internal(value: internal_volume::ConfigMapProjection) -> Self {
        Self {
            name: value.name,
            items: value.items,
            optional: value.optional,
        }
    }
}

// DownwardAPIProjection
impl ToInternal<internal_volume::DownwardAPIProjection> for volume::DownwardAPIProjection {
    fn to_internal(self) -> internal_volume::DownwardAPIProjection {
        internal_volume::DownwardAPIProjection {
            items: self.items.into_iter().map(|i| i.to_internal()).collect(),
        }
    }
}

impl FromInternal<internal_volume::DownwardAPIProjection> for volume::DownwardAPIProjection {
    fn from_internal(value: internal_volume::DownwardAPIProjection) -> Self {
        Self {
            items: value
                .items
                .into_iter()
                .map(volume::DownwardAPIVolumeFile::from_internal)
                .collect(),
        }
    }
}

// ServiceAccountTokenProjection
impl ToInternal<internal_volume::ServiceAccountTokenProjection>
    for volume::ServiceAccountTokenProjection
{
    fn to_internal(self) -> internal_volume::ServiceAccountTokenProjection {
        internal_volume::ServiceAccountTokenProjection {
            audience: self.audience,
            expiration_seconds: self.expiration_seconds.unwrap_or_default(),
            path: self.path,
        }
    }
}

impl FromInternal<internal_volume::ServiceAccountTokenProjection>
    for volume::ServiceAccountTokenProjection
{
    fn from_internal(value: internal_volume::ServiceAccountTokenProjection) -> Self {
        Self {
            audience: value.audience,
            expiration_seconds: if value.expiration_seconds == 0 {
                None
            } else {
                Some(value.expiration_seconds)
            },
            path: value.path,
        }
    }
}

// Note: ClusterTrustBundleProjection and PodCertificateProjection are v1-only types
// that don't exist in internal API, so no conversion implementations are needed.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_projected_volume_roundtrip() {
        let v1_projected = volume::ProjectedVolumeSource {
            sources: vec![volume::VolumeProjection {
                secret: Some(volume::SecretProjection {
                    name: Some("my-secret".to_string()),
                    items: vec![],
                    optional: Some(false),
                }),
                downward_api: None,
                config_map: None,
                service_account_token: None,
                cluster_trust_bundle: None,
                pod_certificate: None,
            }],
            default_mode: Some(0o644),
        };

        let internal_projected = v1_projected.clone().to_internal();
        assert_eq!(internal_projected.sources.len(), 1);
        assert!(internal_projected.sources[0].secret.is_some());

        let roundtrip = volume::ProjectedVolumeSource::from_internal(internal_projected);
        assert_eq!(roundtrip.sources.len(), 1);
        assert_eq!(
            roundtrip.sources[0].secret.as_ref().unwrap().name,
            Some("my-secret".to_string())
        );
    }

    #[test]
    fn test_volume_projection_v1_only_fields_dropped() {
        // Test that v1-only fields (cluster_trust_bundle, pod_certificate) are dropped
        let v1_projection = volume::VolumeProjection {
            secret: Some(volume::SecretProjection {
                name: Some("secret".to_string()),
                items: vec![],
                optional: None,
            }),
            cluster_trust_bundle: Some(volume::ClusterTrustBundleProjection {
                name: Some("bundle".to_string()),
                signer_name: None,
                label_selector: None,
                optional: None,
                path: "path".to_string(),
            }),
            pod_certificate: Some(volume::PodCertificateProjection {
                signer_name: "kubernetes.io/kubelet-serving".to_string(),
                key_type: "RSA".to_string(),
                max_expiration_seconds: None,
                credential_bundle_path: Some("/var/run/secrets/cert".to_string()),
                key_path: None,
                certificate_chain_path: None,
                user_annotations: std::collections::BTreeMap::new(),
            }),
            downward_api: None,
            config_map: None,
            service_account_token: None,
        };

        let internal_projection = v1_projection.to_internal();
        // Internal should only have secret, cluster_trust_bundle and pod_certificate are dropped
        assert!(internal_projection.secret.is_some());
        assert!(internal_projection.downward_api.is_none());
        assert!(internal_projection.config_map.is_none());
        assert!(internal_projection.service_account_token.is_none());

        // Round-trip should set v1-only fields to None
        let roundtrip = volume::VolumeProjection::from_internal(internal_projection);
        assert!(roundtrip.secret.is_some());
        assert!(roundtrip.cluster_trust_bundle.is_none());
        assert!(roundtrip.pod_certificate.is_none());
    }
}
