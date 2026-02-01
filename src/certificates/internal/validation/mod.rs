//! Validation for Kubernetes Certificates internal API types
//!
//! Ported from k8s.io/kubernetes/pkg/apis/certificates/validation/validation.go

use std::collections::BTreeSet;

use crate::certificates::internal;
use crate::certificates::internal::{
    CertificateSigningRequest, CertificateSigningRequestCondition, CertificateSigningRequestSpec,
    CertificateSigningRequestStatus, KeyUsage, RequestConditionType,
};
use crate::common::validation::{
    BadValue, ErrorList, Path, invalid, required, too_long, validate_object_meta,
    validate_qualified_name,
};
use crate::core::internal::ConditionStatus;

pub fn validate_certificate_signing_request(csr: &CertificateSigningRequest) -> ErrorList {
    let mut all_errs = validate_object_meta(
        &csr.metadata,
        false,
        validate_certificate_request_name,
        &Path::new("metadata"),
    );
    all_errs.extend(validate_certificate_signing_request_spec(
        &csr.spec,
        &Path::new("spec"),
    ));
    all_errs.extend(validate_certificate_signing_request_status(
        &csr.status,
        &Path::new("status"),
    ));
    all_errs
}

fn validate_certificate_request_name(_name: &str, _prefix: bool) -> Vec<String> {
    Vec::new()
}

fn validate_certificate_signing_request_spec(
    spec: &CertificateSigningRequestSpec,
    fld_path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if spec.request.0.is_empty() {
        all_errs.push(required(&fld_path.child("request"), ""));
    }

    if spec.usages.is_empty() {
        all_errs.push(required(&fld_path.child("usages"), ""));
    }

    let mut seen_usages = BTreeSet::new();
    for (i, usage) in spec.usages.iter().enumerate() {
        let value = key_usage_value(usage);
        if !is_valid_key_usage(value) {
            all_errs.push(invalid(
                &fld_path.child("usages").index(i),
                BadValue::String(value.to_string()),
                "unsupported key usage",
            ));
        }
        if !seen_usages.insert(value.to_string()) {
            all_errs.push(invalid(
                &fld_path.child("usages").index(i),
                BadValue::String(value.to_string()),
                "duplicate key usage",
            ));
        }
    }

    if spec.signer_name == internal::LEGACY_UNKNOWN_SIGNER_NAME {
        all_errs.push(invalid(
            &fld_path.child("signerName"),
            BadValue::String(spec.signer_name.clone()),
            "the legacy signerName is not allowed via this API version",
        ));
    } else {
        all_errs.extend(validate_qualified_name(
            &spec.signer_name,
            &fld_path.child("signerName"),
        ));
    }

    if let Some(expiration) = spec.expiration_seconds {
        if expiration < 600 {
            all_errs.push(invalid(
                &fld_path.child("expirationSeconds"),
                BadValue::Int(expiration as i64),
                "may not specify a duration less than 600 seconds (10 minutes)",
            ));
        }
    }

    all_errs
}

fn validate_certificate_signing_request_status(
    status: &CertificateSigningRequestStatus,
    fld_path: &Path,
) -> ErrorList {
    validate_conditions(&status.conditions, &fld_path.child("conditions"))
}

fn validate_conditions(
    conditions: &[CertificateSigningRequestCondition],
    fld_path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let mut seen_types = BTreeSet::new();
    let mut has_approved = false;
    let mut has_denied = false;

    for (i, cond) in conditions.iter().enumerate() {
        let cond_path = fld_path.index(i);
        let type_value = request_condition_value(&cond.type_);
        if type_value.is_empty() {
            all_errs.push(required(&cond_path.child("type"), ""));
        }
        if !seen_types.insert(type_value.to_string()) {
            all_errs.push(invalid(
                &cond_path.child("type"),
                BadValue::String(type_value.to_string()),
                "duplicate condition type",
            ));
        }

        let allowed_status = if is_true_only_condition(&cond.type_) {
            vec![ConditionStatus::True]
        } else {
            vec![
                ConditionStatus::True,
                ConditionStatus::False,
                ConditionStatus::Unknown,
            ]
        };
        if !allowed_status.contains(&cond.status) {
            all_errs.push(invalid(
                &cond_path.child("status"),
                BadValue::String(condition_status_value(&cond.status).to_string()),
                "unsupported status for condition type",
            ));
        }

        match cond.type_ {
            RequestConditionType::Approved => {
                has_approved = true;
                if has_denied {
                    all_errs.push(invalid(
                        fld_path,
                        BadValue::String("Approved".to_string()),
                        "Approved and Denied conditions are mutually exclusive",
                    ));
                }
            }
            RequestConditionType::Denied => {
                has_denied = true;
                if has_approved {
                    all_errs.push(invalid(
                        fld_path,
                        BadValue::String("Denied".to_string()),
                        "Approved and Denied conditions are mutually exclusive",
                    ));
                }
            }
            _ => {}
        }
    }

    all_errs
}

fn is_true_only_condition(cond: &RequestConditionType) -> bool {
    matches!(
        cond,
        RequestConditionType::Approved
            | RequestConditionType::Denied
            | RequestConditionType::Failed
    )
}

fn key_usage_value(usage: &KeyUsage) -> &'static str {
    match usage {
        KeyUsage::Signing => "signing",
        KeyUsage::DigitalSignature => "digital signature",
        KeyUsage::ContentCommitment => "content commitment",
        KeyUsage::KeyEncipherment => "key encipherment",
        KeyUsage::KeyAgreement => "key agreement",
        KeyUsage::DataEncipherment => "data encipherment",
        KeyUsage::CertSign => "cert sign",
        KeyUsage::CrlSign => "crl sign",
        KeyUsage::EncipherOnly => "encipher only",
        KeyUsage::DecipherOnly => "decipher only",
        KeyUsage::Any => "any",
        KeyUsage::ServerAuth => "server auth",
        KeyUsage::ClientAuth => "client auth",
        KeyUsage::CodeSigning => "code signing",
        KeyUsage::EmailProtection => "email protection",
        KeyUsage::Smime => "s/mime",
        KeyUsage::IpsecEndSystem => "ipsec end system",
        KeyUsage::IpsecTunnel => "ipsec tunnel",
        KeyUsage::IpsecUser => "ipsec user",
        KeyUsage::Timestamping => "timestamping",
        KeyUsage::OcspSigning => "ocsp signing",
        KeyUsage::MicrosoftSgc => "microsoft sgc",
        KeyUsage::NetscapeSgc => "netscape sgc",
    }
}

fn is_valid_key_usage(value: &str) -> bool {
    matches!(
        value,
        "signing"
            | "digital signature"
            | "content commitment"
            | "key encipherment"
            | "key agreement"
            | "data encipherment"
            | "cert sign"
            | "crl sign"
            | "encipher only"
            | "decipher only"
            | "any"
            | "server auth"
            | "client auth"
            | "code signing"
            | "email protection"
            | "s/mime"
            | "ipsec end system"
            | "ipsec tunnel"
            | "ipsec user"
            | "timestamping"
            | "ocsp signing"
            | "microsoft sgc"
            | "netscape sgc"
    )
}

fn request_condition_value(cond: &RequestConditionType) -> &'static str {
    match cond {
        RequestConditionType::Approved => "Approved",
        RequestConditionType::Denied => "Denied",
        RequestConditionType::Failed => "Failed",
    }
}

fn condition_status_value(status: &ConditionStatus) -> &'static str {
    match status {
        ConditionStatus::True => "True",
        ConditionStatus::False => "False",
        ConditionStatus::Unknown => "Unknown",
    }
}

// ============================================================================
// ClusterTrustBundle Validation
// ============================================================================

pub fn validate_cluster_trust_bundle(bundle: &internal::ClusterTrustBundle) -> ErrorList {
    let mut all_errs = validate_object_meta(
        &bundle.metadata,
        false,
        validate_certificate_request_name,
        &Path::new("metadata"),
    );

    if !bundle.spec.signer_name.is_empty() {
        all_errs.extend(validate_qualified_name(
            &bundle.spec.signer_name,
            &Path::new("spec").child("signerName"),
        ));
    }

    if bundle.spec.trust_bundle.len() > internal::MAX_TRUST_BUNDLE_SIZE {
        all_errs.push(too_long(
            &Path::new("spec").child("trustBundle"),
            internal::MAX_TRUST_BUNDLE_SIZE,
        ));
    }

    all_errs
}

// ============================================================================
// PodCertificateRequest Validation
// ============================================================================

pub fn validate_pod_certificate_request(req: &internal::PodCertificateRequest) -> ErrorList {
    let mut all_errs = validate_object_meta(
        &req.metadata,
        true,
        validate_certificate_request_name,
        &Path::new("metadata"),
    );

    all_errs.extend(validate_qualified_name(
        &req.spec.signer_name,
        &Path::new("spec").child("signerName"),
    ));

    if req.spec.pod_name.is_empty() {
        all_errs.push(required(&Path::new("spec").child("podName"), ""));
    }
    if req.spec.pod_uid.is_empty() {
        all_errs.push(required(&Path::new("spec").child("podUID"), ""));
    } else if req.spec.pod_uid.len() > 128 {
        all_errs.push(too_long(&Path::new("spec").child("podUID"), 128));
    }
    if req.spec.service_account_name.is_empty() {
        all_errs.push(required(&Path::new("spec").child("serviceAccountName"), ""));
    }
    if req.spec.service_account_uid.is_empty() {
        all_errs.push(required(&Path::new("spec").child("serviceAccountUID"), ""));
    } else if req.spec.service_account_uid.len() > 128 {
        all_errs.push(too_long(&Path::new("spec").child("serviceAccountUID"), 128));
    }
    if req.spec.node_name.is_empty() {
        all_errs.push(required(&Path::new("spec").child("nodeName"), ""));
    }
    if req.spec.node_uid.is_empty() {
        all_errs.push(required(&Path::new("spec").child("nodeUID"), ""));
    } else if req.spec.node_uid.len() > 128 {
        all_errs.push(too_long(&Path::new("spec").child("nodeUID"), 128));
    }

    let max_exp = req.spec.max_expiration_seconds;
    if max_exp.is_none() {
        all_errs.push(required(
            &Path::new("spec").child("maxExpirationSeconds"),
            "must be set",
        ));
    } else if let Some(expiration) = max_exp {
        let (min, max) = if req.spec.signer_name.starts_with("kubernetes.io/") {
            (
                internal::MIN_MAX_EXPIRATION_SECONDS,
                internal::KUBERNETES_MAX_MAX_EXPIRATION_SECONDS,
            )
        } else {
            (
                internal::MIN_MAX_EXPIRATION_SECONDS,
                internal::MAX_MAX_EXPIRATION_SECONDS,
            )
        };
        if expiration < min || expiration > max {
            all_errs.push(invalid(
                &Path::new("spec").child("maxExpirationSeconds"),
                BadValue::Int(expiration as i64),
                &format!("must be in the range [{}, {}]", min, max),
            ));
        }
    }

    if req.spec.pkix_public_key.0.len() > internal::MAX_PKIX_PUBLIC_KEY_SIZE {
        all_errs.push(too_long(
            &Path::new("spec").child("pkixPublicKey"),
            internal::MAX_PKIX_PUBLIC_KEY_SIZE,
        ));
    }

    if req.spec.proof_of_possession.0.len() > internal::MAX_PROOF_OF_POSSESSION_SIZE {
        all_errs.push(too_long(
            &Path::new("spec").child("proofOfPossession"),
            internal::MAX_PROOF_OF_POSSESSION_SIZE,
        ));
    }

    all_errs
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{Timestamp, TypeMeta};

    #[test]
    fn test_validate_csr_accepts_valid() {
        let csr = CertificateSigningRequest {
            type_meta: TypeMeta::default(),
            metadata: crate::common::ObjectMeta {
                name: Some("csr".to_string()),
                ..Default::default()
            },
            spec: CertificateSigningRequestSpec {
                request: crate::core::internal::helper::ByteString(vec![1, 2, 3]),
                signer_name: "example.com/signer".to_string(),
                usages: vec![KeyUsage::ClientAuth],
                ..Default::default()
            },
            status: CertificateSigningRequestStatus {
                conditions: vec![CertificateSigningRequestCondition {
                    type_: RequestConditionType::Approved,
                    status: ConditionStatus::True,
                    reason: "ok".to_string(),
                    message: "ok".to_string(),
                    last_update_time: Some(Timestamp::zero()),
                    last_transition_time: Some(Timestamp::zero()),
                }],
                certificate: None,
            },
        };

        let errs = validate_certificate_signing_request(&csr);
        assert!(errs.is_empty(), "expected no errors, got {:?}", errs);
    }

    #[test]
    fn test_validate_csr_rejects_empty_usages() {
        let csr = CertificateSigningRequest {
            type_meta: TypeMeta::default(),
            metadata: crate::common::ObjectMeta {
                name: Some("csr".to_string()),
                ..Default::default()
            },
            spec: CertificateSigningRequestSpec {
                request: crate::core::internal::helper::ByteString(vec![1]),
                signer_name: "example.com/signer".to_string(),
                usages: vec![],
                ..Default::default()
            },
            status: CertificateSigningRequestStatus::default(),
        };

        let errs = validate_certificate_signing_request(&csr);
        assert!(!errs.is_empty());
    }

    #[test]
    fn test_validate_pod_certificate_request_requires_expiration() {
        let req = internal::PodCertificateRequest {
            metadata: crate::common::ObjectMeta {
                name: Some("pcr".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            },
            spec: internal::PodCertificateRequestSpec {
                signer_name: "example.com/signer".to_string(),
                pod_name: "pod".to_string(),
                pod_uid: "uid".to_string(),
                service_account_name: "sa".to_string(),
                service_account_uid: "uid".to_string(),
                node_name: "node".to_string(),
                node_uid: "uid".to_string(),
                max_expiration_seconds: None,
                pkix_public_key: crate::core::internal::helper::ByteString(vec![]),
                proof_of_possession: crate::core::internal::helper::ByteString(vec![]),
            },
            status: None,
        };

        let errs = validate_pod_certificate_request(&req);
        assert!(!errs.is_empty());
    }
}
