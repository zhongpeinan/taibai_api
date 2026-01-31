use super::{
    CertificateSigningRequest, CertificateSigningRequestList, CertificateSigningRequestSpec,
    KeyUsage,
};
use crate::common::test_utils::assert_serde_roundtrip;
use crate::common::{ListMeta, ObjectMeta, TypeMeta};
use crate::core::internal::helper::ByteString;

fn csr_basic() -> CertificateSigningRequest {
    CertificateSigningRequest {
        type_meta: TypeMeta {
            api_version: "certificates.k8s.io/v1".to_string(),
            kind: "CertificateSigningRequest".to_string(),
        },
        metadata: Some(ObjectMeta {
            name: Some("csr-1".to_string()),
            ..Default::default()
        }),
        spec: CertificateSigningRequestSpec {
            request: ByteString(vec![1, 2, 3]),
            signer_name: "example.com/signer".to_string(),
            usages: vec![KeyUsage::ClientAuth, KeyUsage::DigitalSignature],
            ..Default::default()
        },
        status: Some(Default::default()),
    }
}

fn csr_list_basic() -> CertificateSigningRequestList {
    CertificateSigningRequestList {
        type_meta: TypeMeta {
            api_version: "certificates.k8s.io/v1".to_string(),
            kind: "CertificateSigningRequestList".to_string(),
        },
        metadata: Some(ListMeta {
            resource_version: Some("10".to_string()),
            ..Default::default()
        }),
        items: vec![csr_basic()],
    }
}

#[test]
fn serde_roundtrip_csr() {
    assert_serde_roundtrip(&csr_basic());
}

#[test]
fn serde_roundtrip_csr_list() {
    assert_serde_roundtrip(&csr_list_basic());
}
