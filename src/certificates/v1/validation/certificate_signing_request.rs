use crate::certificates::internal::validation as internal_validation;
use crate::certificates::v1::CertificateSigningRequest;
use crate::common::ToInternal;
use crate::common::validation::ErrorList;

pub fn validate_certificate_signing_request(csr: &CertificateSigningRequest) -> ErrorList {
    internal_validation::validate_certificate_signing_request(&csr.clone().to_internal())
}
