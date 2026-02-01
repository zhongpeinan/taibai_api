//! Conversions between v1beta1 and internal certificates types

use crate::certificates::internal;
use crate::common::{ApplyDefault, FromInternal, ListMeta, ObjectMeta, ToInternal, TypeMeta};

use super::{
    CertificateSigningRequest, CertificateSigningRequestList, CertificateSigningRequestSpec,
    ClusterTrustBundle, ClusterTrustBundleList, ClusterTrustBundleSpec,
};

// ============================================================================
// Conversion Helper Functions
// ============================================================================

fn is_empty_object_meta(meta: &ObjectMeta) -> bool {
    meta.name.is_none()
        && meta.generate_name.is_none()
        && meta.namespace.is_none()
        && meta.uid.is_none()
        && meta.resource_version.is_none()
        && meta.generation.is_none()
        && meta.self_link.is_none()
        && meta.labels.is_empty()
        && meta.annotations.is_empty()
        && meta.owner_references.is_empty()
        && meta.finalizers.is_empty()
        && meta.managed_fields.is_empty()
        && meta.creation_timestamp.is_none()
        && meta.deletion_timestamp.is_none()
        && meta.deletion_grace_period_seconds.is_none()
}

fn is_empty_list_meta(meta: &ListMeta) -> bool {
    meta.continue_.is_none()
        && meta.remaining_item_count.is_none()
        && meta.resource_version.is_none()
        && meta.self_link.is_none()
}

fn option_object_meta_to_meta(meta: Option<ObjectMeta>) -> ObjectMeta {
    meta.unwrap_or_default()
}

fn meta_to_option_object_meta(meta: ObjectMeta) -> Option<ObjectMeta> {
    if is_empty_object_meta(&meta) {
        None
    } else {
        Some(meta)
    }
}

fn option_list_meta_to_meta(meta: Option<ListMeta>) -> ListMeta {
    meta.unwrap_or_default()
}

fn meta_to_option_list_meta(meta: ListMeta) -> Option<ListMeta> {
    if is_empty_list_meta(&meta) {
        None
    } else {
        Some(meta)
    }
}

fn option_string_to_string(value: Option<String>) -> String {
    value.unwrap_or_default()
}

fn string_to_option_string(value: String) -> Option<String> {
    if value.is_empty() { None } else { Some(value) }
}

fn v1beta1_spec_to_internal(
    spec: CertificateSigningRequestSpec,
) -> internal::CertificateSigningRequestSpec {
    internal::CertificateSigningRequestSpec {
        request: spec.request,
        signer_name: option_string_to_string(spec.signer_name),
        expiration_seconds: spec.expiration_seconds,
        usages: spec.usages,
        username: spec.username,
        uid: spec.uid,
        groups: spec.groups,
        extra: spec.extra,
    }
}

fn internal_spec_to_v1beta1(
    spec: internal::CertificateSigningRequestSpec,
) -> CertificateSigningRequestSpec {
    CertificateSigningRequestSpec {
        request: spec.request,
        signer_name: string_to_option_string(spec.signer_name),
        expiration_seconds: spec.expiration_seconds,
        usages: spec.usages,
        username: spec.username,
        uid: spec.uid,
        groups: spec.groups,
        extra: spec.extra,
    }
}

fn v1beta1_bundle_spec_to_internal(
    spec: ClusterTrustBundleSpec,
) -> internal::ClusterTrustBundleSpec {
    internal::ClusterTrustBundleSpec {
        signer_name: spec.signer_name,
        trust_bundle: spec.trust_bundle,
    }
}

fn internal_bundle_spec_to_v1beta1(
    spec: internal::ClusterTrustBundleSpec,
) -> ClusterTrustBundleSpec {
    ClusterTrustBundleSpec {
        signer_name: spec.signer_name,
        trust_bundle: spec.trust_bundle,
    }
}

// ============================================================================
// CertificateSigningRequest Conversions
// ============================================================================

impl ToInternal<internal::CertificateSigningRequest> for CertificateSigningRequest {
    fn to_internal(self) -> internal::CertificateSigningRequest {
        internal::CertificateSigningRequest {
            type_meta: TypeMeta::default(),
            metadata: option_object_meta_to_meta(self.metadata),
            spec: v1beta1_spec_to_internal(self.spec),
            status: self.status.unwrap_or_default(),
        }
    }
}

impl FromInternal<internal::CertificateSigningRequest> for CertificateSigningRequest {
    fn from_internal(value: internal::CertificateSigningRequest) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: meta_to_option_object_meta(value.metadata),
            spec: internal_spec_to_v1beta1(value.spec),
            status: Some(value.status),
        };

        result
    }
}

// ============================================================================
// CertificateSigningRequestList Conversions
// ============================================================================

impl ToInternal<internal::CertificateSigningRequestList> for CertificateSigningRequestList {
    fn to_internal(self) -> internal::CertificateSigningRequestList {
        internal::CertificateSigningRequestList {
            type_meta: TypeMeta::default(),
            metadata: option_list_meta_to_meta(self.metadata),
            items: self
                .items
                .into_iter()
                .map(CertificateSigningRequest::to_internal)
                .collect(),
        }
    }
}

impl FromInternal<internal::CertificateSigningRequestList> for CertificateSigningRequestList {
    fn from_internal(value: internal::CertificateSigningRequestList) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: meta_to_option_list_meta(value.metadata),
            items: value
                .items
                .into_iter()
                .map(CertificateSigningRequest::from_internal)
                .collect(),
        };

        result
    }
}

// ============================================================================
// ClusterTrustBundle Conversions
// ============================================================================

impl ToInternal<internal::ClusterTrustBundle> for ClusterTrustBundle {
    fn to_internal(self) -> internal::ClusterTrustBundle {
        internal::ClusterTrustBundle {
            metadata: option_object_meta_to_meta(self.metadata),
            spec: v1beta1_bundle_spec_to_internal(self.spec),
        }
    }
}

impl FromInternal<internal::ClusterTrustBundle> for ClusterTrustBundle {
    fn from_internal(value: internal::ClusterTrustBundle) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: meta_to_option_object_meta(value.metadata),
            spec: internal_bundle_spec_to_v1beta1(value.spec),
        };

        result
    }
}

// ============================================================================
// ClusterTrustBundleList Conversions
// ============================================================================

impl ToInternal<internal::ClusterTrustBundleList> for ClusterTrustBundleList {
    fn to_internal(self) -> internal::ClusterTrustBundleList {
        internal::ClusterTrustBundleList {
            metadata: self.metadata,
            items: self
                .items
                .into_iter()
                .map(ClusterTrustBundle::to_internal)
                .collect(),
        }
    }
}

impl FromInternal<internal::ClusterTrustBundleList> for ClusterTrustBundleList {
    fn from_internal(value: internal::ClusterTrustBundleList) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: value.metadata,
            items: value
                .items
                .into_iter()
                .map(ClusterTrustBundle::from_internal)
                .collect(),
        };

        result
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::certificates::v1::KeyUsage;

    #[test]
    fn test_csr_round_trip() {
        let original = CertificateSigningRequest {
            type_meta: TypeMeta {
                api_version: "certificates.k8s.io/v1beta1".to_string(),
                kind: "CertificateSigningRequest".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("csr-1".to_string()),
                ..Default::default()
            }),
            spec: CertificateSigningRequestSpec {
                signer_name: Some("example.com/signer".to_string()),
                usages: vec![KeyUsage::ClientAuth],
                ..Default::default()
            },
            status: None,
        };

        let internal = original.clone().to_internal();
        let mut round_trip = CertificateSigningRequest::from_internal(internal);
        round_trip.apply_default();

        assert_eq!(round_trip.metadata, original.metadata);
        assert_eq!(
            round_trip.spec.signer_name.as_deref(),
            Some("example.com/signer")
        );
        assert_eq!(
            round_trip.type_meta.api_version,
            "certificates.k8s.io/v1beta1"
        );
        assert_eq!(round_trip.type_meta.kind, "CertificateSigningRequest");
    }

    #[test]
    fn test_csr_list_round_trip() {
        let original = CertificateSigningRequestList {
            type_meta: TypeMeta {
                api_version: "certificates.k8s.io/v1beta1".to_string(),
                kind: "CertificateSigningRequestList".to_string(),
            },
            metadata: Some(ListMeta {
                resource_version: Some("10".to_string()),
                ..Default::default()
            }),
            items: vec![CertificateSigningRequest {
                type_meta: TypeMeta::default(),
                metadata: Some(ObjectMeta {
                    name: Some("csr-1".to_string()),
                    ..Default::default()
                }),
                spec: CertificateSigningRequestSpec::default(),
                status: None,
            }],
        };

        let internal = original.clone().to_internal();
        let mut round_trip = CertificateSigningRequestList::from_internal(internal);
        round_trip.apply_default();

        assert_eq!(round_trip.items.len(), 1);
        assert_eq!(round_trip.items[0].metadata, original.items[0].metadata);
        assert_eq!(
            round_trip.type_meta.api_version,
            "certificates.k8s.io/v1beta1"
        );
        assert_eq!(round_trip.type_meta.kind, "CertificateSigningRequestList");
    }

    #[test]
    fn test_cluster_trust_bundle_round_trip() {
        let original = ClusterTrustBundle {
            type_meta: TypeMeta {
                api_version: "certificates.k8s.io/v1beta1".to_string(),
                kind: "ClusterTrustBundle".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("bundle-1".to_string()),
                ..Default::default()
            }),
            spec: ClusterTrustBundleSpec {
                signer_name: "example.com/signer".to_string(),
                trust_bundle: "bundle".to_string(),
            },
        };

        let internal = original.clone().to_internal();
        let mut round_trip = ClusterTrustBundle::from_internal(internal);
        round_trip.apply_default();

        assert_eq!(round_trip.metadata, original.metadata);
        assert_eq!(round_trip.spec.signer_name, "example.com/signer");
        assert_eq!(
            round_trip.type_meta.api_version,
            "certificates.k8s.io/v1beta1"
        );
        assert_eq!(round_trip.type_meta.kind, "ClusterTrustBundle");
    }
}
