//! Certificates v1 type registrations for the test harness.

use crate::harness::helpers::register_type;
use crate::harness::registry::Registry;

pub fn register(registry: &mut Registry) {
    // ---- certificates.k8s.io/v1/CertificateSigningRequest ----
    register_type::<
        crate::certificates::v1::CertificateSigningRequest,
        crate::certificates::internal::CertificateSigningRequest,
        _,
    >(
        registry,
        "certificates.k8s.io/v1/CertificateSigningRequest",
        crate::certificates::v1::validation::validate_certificate_signing_request,
    );
}
