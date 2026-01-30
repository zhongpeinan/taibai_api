use super::{
    CertificateSigningRequest, CertificateSigningRequestList, CertificateSigningRequestSpec,
    KeyUsage,
};
use crate::certificates::internal;
use crate::common::test_utils::assert_conversion_roundtrip;
use crate::common::{ApplyDefault, ListMeta, ObjectMeta, TypeMeta};
use crate::core::internal::helper::ByteString;

fn csr_basic() -> CertificateSigningRequest {
    CertificateSigningRequest {
        type_meta: TypeMeta::default(),
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
    let mut item = csr_basic();
    item.apply_default();
    CertificateSigningRequestList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("10".to_string()),
            ..Default::default()
        }),
        items: vec![item],
    }
}

#[test]
fn conversion_roundtrip_csr() {
    assert_conversion_roundtrip::<CertificateSigningRequest, internal::CertificateSigningRequest>(
        csr_basic(),
    );
}

#[test]
fn conversion_roundtrip_csr_list() {
    assert_conversion_roundtrip::<
        CertificateSigningRequestList,
        internal::CertificateSigningRequestList,
    >(csr_list_basic());
}
