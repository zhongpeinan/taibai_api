//! Conversions between v1 and internal certificates types

use crate::common::{ApplyDefault, FromInternal, ListMeta, ObjectMeta, ToInternal, TypeMeta};

use super::{
    CertificateSigningRequest, CertificateSigningRequestList, CertificateSigningRequestSpec,
    CertificateSigningRequestStatus,
};

// Internal types are re-exports in certificates/internal, but those carry TypeMeta and
// Option metadata. These wrappers represent the normalized internal shapes.
mod internal {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct CertificateSigningRequestInternal {
        pub metadata: ObjectMeta,
        pub spec: CertificateSigningRequestSpec,
        pub status: CertificateSigningRequestStatus,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct CertificateSigningRequestListInternal {
        pub metadata: ListMeta,
        pub items: Vec<CertificateSigningRequestInternal>,
    }
}

pub use internal::{CertificateSigningRequestInternal, CertificateSigningRequestListInternal};

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

// ============================================================================
// CertificateSigningRequest Conversions
// ============================================================================

impl ToInternal<CertificateSigningRequestInternal> for CertificateSigningRequest {
    fn to_internal(self) -> CertificateSigningRequestInternal {
        CertificateSigningRequestInternal {
            metadata: option_object_meta_to_meta(self.metadata),
            spec: self.spec,
            status: self.status.unwrap_or_default(),
        }
    }
}

impl FromInternal<CertificateSigningRequestInternal> for CertificateSigningRequest {
    fn from_internal(value: CertificateSigningRequestInternal) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: meta_to_option_object_meta(value.metadata),
            spec: value.spec,
            status: Some(value.status),
        };
        result.apply_default();
        result
    }
}

// ============================================================================
// CertificateSigningRequestList Conversions
// ============================================================================

impl ToInternal<CertificateSigningRequestListInternal> for CertificateSigningRequestList {
    fn to_internal(self) -> CertificateSigningRequestListInternal {
        CertificateSigningRequestListInternal {
            metadata: option_list_meta_to_meta(self.metadata),
            items: self
                .items
                .into_iter()
                .map(CertificateSigningRequest::to_internal)
                .collect(),
        }
    }
}

impl FromInternal<CertificateSigningRequestListInternal> for CertificateSigningRequestList {
    fn from_internal(value: CertificateSigningRequestListInternal) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: meta_to_option_list_meta(value.metadata),
            items: value
                .items
                .into_iter()
                .map(CertificateSigningRequest::from_internal)
                .collect(),
        };
        result.apply_default();
        result
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csr_round_trip() {
        let original = CertificateSigningRequest {
            type_meta: TypeMeta {
                api_version: "certificates.k8s.io/v1".to_string(),
                kind: "CertificateSigningRequest".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("csr-1".to_string()),
                ..Default::default()
            }),
            spec: CertificateSigningRequestSpec {
                signer_name: "example.com/signer".to_string(),
                usages: vec![super::super::KeyUsage::ClientAuth],
                ..Default::default()
            },
            status: None,
        };

        let internal = original.clone().to_internal();
        let round_trip = CertificateSigningRequest::from_internal(internal);

        assert_eq!(round_trip.metadata, original.metadata);
        assert_eq!(round_trip.spec.signer_name, "example.com/signer");
        assert_eq!(round_trip.type_meta.api_version, "certificates.k8s.io/v1");
        assert_eq!(round_trip.type_meta.kind, "CertificateSigningRequest");
    }

    #[test]
    fn test_csr_list_round_trip() {
        let original = CertificateSigningRequestList {
            type_meta: TypeMeta {
                api_version: "certificates.k8s.io/v1".to_string(),
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
        let round_trip = CertificateSigningRequestList::from_internal(internal);

        assert_eq!(round_trip.items.len(), 1);
        assert_eq!(round_trip.items[0].metadata, original.items[0].metadata);
        assert_eq!(round_trip.type_meta.api_version, "certificates.k8s.io/v1");
        assert_eq!(round_trip.type_meta.kind, "CertificateSigningRequestList");
    }
}
