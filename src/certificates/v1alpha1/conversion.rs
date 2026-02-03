//! Conversions between v1alpha1 and internal certificates types

use crate::certificates::internal;
#[allow(unused_imports)]
use crate::common::{ApplyDefault, FromInternal, ObjectMeta, ToInternal, TypeMeta};

use super::{
    ClusterTrustBundle, ClusterTrustBundleList, ClusterTrustBundleSpec, PodCertificateRequest,
    PodCertificateRequestList, PodCertificateRequestSpec, PodCertificateRequestStatus,
};

fn v1alpha1_bundle_spec_to_internal(
    spec: ClusterTrustBundleSpec,
) -> internal::ClusterTrustBundleSpec {
    internal::ClusterTrustBundleSpec {
        signer_name: spec.signer_name,
        trust_bundle: spec.trust_bundle,
    }
}

fn internal_bundle_spec_to_v1alpha1(
    spec: internal::ClusterTrustBundleSpec,
) -> ClusterTrustBundleSpec {
    ClusterTrustBundleSpec {
        signer_name: spec.signer_name,
        trust_bundle: spec.trust_bundle,
    }
}

fn v1alpha1_pod_spec_to_internal(
    spec: PodCertificateRequestSpec,
) -> internal::PodCertificateRequestSpec {
    internal::PodCertificateRequestSpec {
        signer_name: spec.signer_name,
        pod_name: spec.pod_name,
        pod_uid: spec.pod_uid,
        service_account_name: spec.service_account_name,
        service_account_uid: spec.service_account_uid,
        node_name: spec.node_name,
        node_uid: spec.node_uid,
        max_expiration_seconds: spec.max_expiration_seconds,
        pkix_public_key: spec.pkix_public_key,
        proof_of_possession: spec.proof_of_possession,
    }
}

fn internal_pod_spec_to_v1alpha1(
    spec: internal::PodCertificateRequestSpec,
) -> PodCertificateRequestSpec {
    PodCertificateRequestSpec {
        signer_name: spec.signer_name,
        pod_name: spec.pod_name,
        pod_uid: spec.pod_uid,
        service_account_name: spec.service_account_name,
        service_account_uid: spec.service_account_uid,
        node_name: spec.node_name,
        node_uid: spec.node_uid,
        max_expiration_seconds: spec.max_expiration_seconds,
        pkix_public_key: spec.pkix_public_key,
        proof_of_possession: spec.proof_of_possession,
    }
}

fn option_object_meta_to_meta(meta: Option<ObjectMeta>) -> ObjectMeta {
    meta.unwrap_or_default()
}

fn meta_to_option_object_meta(meta: ObjectMeta) -> Option<ObjectMeta> {
    if meta.name.is_none()
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
    {
        None
    } else {
        Some(meta)
    }
}

fn v1alpha1_status_to_internal(
    status: PodCertificateRequestStatus,
) -> internal::PodCertificateRequestStatus {
    internal::PodCertificateRequestStatus {
        conditions: status.conditions,
        certificate_chain: status.certificate_chain,
        not_before: status.not_before,
        begin_refresh_at: status.begin_refresh_at,
        not_after: status.not_after,
    }
}

fn internal_status_to_v1alpha1(
    status: internal::PodCertificateRequestStatus,
) -> PodCertificateRequestStatus {
    PodCertificateRequestStatus {
        conditions: status.conditions,
        certificate_chain: status.certificate_chain,
        not_before: status.not_before,
        begin_refresh_at: status.begin_refresh_at,
        not_after: status.not_after,
    }
}

// ============================================================================
// ClusterTrustBundle Conversions
// ============================================================================

impl ToInternal<internal::ClusterTrustBundle> for ClusterTrustBundle {
    fn to_internal(self) -> internal::ClusterTrustBundle {
        internal::ClusterTrustBundle {
            metadata: option_object_meta_to_meta(self.metadata),
            spec: v1alpha1_bundle_spec_to_internal(self.spec),
        }
    }
}

impl FromInternal<internal::ClusterTrustBundle> for ClusterTrustBundle {
    fn from_internal(value: internal::ClusterTrustBundle) -> Self {
        let result = Self {
            type_meta: TypeMeta::default(),
            metadata: meta_to_option_object_meta(value.metadata),
            spec: internal_bundle_spec_to_v1alpha1(value.spec),
        };

        result
    }
}

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
        let result = Self {
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
// PodCertificateRequest Conversions
// ============================================================================

impl ToInternal<internal::PodCertificateRequest> for PodCertificateRequest {
    fn to_internal(self) -> internal::PodCertificateRequest {
        internal::PodCertificateRequest {
            metadata: option_object_meta_to_meta(self.metadata),
            spec: v1alpha1_pod_spec_to_internal(self.spec),
            status: self.status.map(v1alpha1_status_to_internal),
        }
    }
}

impl FromInternal<internal::PodCertificateRequest> for PodCertificateRequest {
    fn from_internal(value: internal::PodCertificateRequest) -> Self {
        let result = Self {
            type_meta: TypeMeta::default(),
            metadata: meta_to_option_object_meta(value.metadata),
            spec: internal_pod_spec_to_v1alpha1(value.spec),
            status: value.status.map(internal_status_to_v1alpha1),
        };

        result
    }
}

impl ToInternal<internal::PodCertificateRequestList> for PodCertificateRequestList {
    fn to_internal(self) -> internal::PodCertificateRequestList {
        internal::PodCertificateRequestList {
            metadata: self.metadata,
            items: self
                .items
                .into_iter()
                .map(PodCertificateRequest::to_internal)
                .collect(),
        }
    }
}

impl FromInternal<internal::PodCertificateRequestList> for PodCertificateRequestList {
    fn from_internal(value: internal::PodCertificateRequestList) -> Self {
        let result = Self {
            type_meta: TypeMeta::default(),
            metadata: value.metadata,
            items: value
                .items
                .into_iter()
                .map(PodCertificateRequest::from_internal)
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
    use crate::common::ObjectMeta;
    use crate::core::internal::helper::ByteString;

    #[test]
    fn test_pod_request_round_trip() {
        let original = PodCertificateRequest {
            type_meta: TypeMeta {
                api_version: "certificates.k8s.io/v1alpha1".to_string(),
                kind: "PodCertificateRequest".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("pcr-1".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: PodCertificateRequestSpec {
                signer_name: "example.com/signer".to_string(),
                pod_name: "pod".to_string(),
                pod_uid: "uid".to_string(),
                service_account_name: "sa".to_string(),
                service_account_uid: "uid".to_string(),
                node_name: "node".to_string(),
                node_uid: "uid".to_string(),
                max_expiration_seconds: Some(3600),
                pkix_public_key: ByteString(vec![1, 2, 3]),
                proof_of_possession: ByteString(vec![4, 5, 6]),
            },
            status: None,
        };

        let internal = original.clone().to_internal();
        let mut round_trip = PodCertificateRequest::from_internal(internal);
        round_trip.apply_default();

        assert_eq!(round_trip.metadata, original.metadata);
        assert_eq!(round_trip.spec.signer_name, "example.com/signer");
        assert_eq!(
            round_trip.type_meta.api_version,
            "certificates.k8s.io/v1alpha1"
        );
        assert_eq!(round_trip.type_meta.kind, "PodCertificateRequest");
    }

    #[test]
    fn test_bundle_round_trip() {
        let original = ClusterTrustBundle {
            type_meta: TypeMeta {
                api_version: "certificates.k8s.io/v1alpha1".to_string(),
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
            "certificates.k8s.io/v1alpha1"
        );
        assert_eq!(round_trip.type_meta.kind, "ClusterTrustBundle");
    }
}
