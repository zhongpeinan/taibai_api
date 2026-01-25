//! Default values for certificates v1 API types
//!
//! Ported from k8s/pkg/apis/certificates/v1/zz_generated.defaults.go

use crate::common::ApplyDefault;

use super::{CertificateSigningRequest, CertificateSigningRequestList};

impl ApplyDefault for CertificateSigningRequest {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "certificates.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "CertificateSigningRequest".to_string();
        }
    }
}

impl ApplyDefault for CertificateSigningRequestList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "certificates.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "CertificateSigningRequestList".to_string();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::TypeMeta;

    #[test]
    fn test_csr_apply_default_sets_type_meta() {
        let mut csr = CertificateSigningRequest::default();
        csr.apply_default();

        assert_eq!(csr.type_meta.api_version, "certificates.k8s.io/v1");
        assert_eq!(csr.type_meta.kind, "CertificateSigningRequest");
    }

    #[test]
    fn test_csr_apply_default_preserves_type_meta() {
        let mut csr = CertificateSigningRequest {
            type_meta: TypeMeta {
                api_version: "custom.io/v1".to_string(),
                kind: "CustomCSR".to_string(),
            },
            ..Default::default()
        };
        csr.apply_default();

        assert_eq!(csr.type_meta.api_version, "custom.io/v1");
        assert_eq!(csr.type_meta.kind, "CustomCSR");
    }
}
