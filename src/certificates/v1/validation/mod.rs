//! Validation for Kubernetes Certificates v1 API types

mod certificate_signing_request;

pub use certificate_signing_request::validate_certificate_signing_request;
