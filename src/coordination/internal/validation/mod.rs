//! Validation for Kubernetes Coordination API types
//!
//! Ported from k8s.io/kubernetes/pkg/apis/coordination/validation/validation.go

use crate::common::validation::{
    BadValue, ErrorList, Path, forbidden, invalid, name_is_dns_subdomain, not_supported, required,
    validate_object_meta, validate_object_meta_update,
};
use crate::coordination::internal;
use regex::Regex;
use semver::Version;
use std::sync::OnceLock;

const VALID_LEASE_STRATEGIES: [&str; 1] =
    [internal::coordinated_lease_strategy::OLDEST_EMULATION_VERSION];

const CONFIG_MAP_KEY_FMT: &str = "[-._a-zA-Z0-9]+";
const CONFIG_MAP_KEY_ERR_MSG: &str =
    "a valid config key must consist of alphanumeric characters, '-', '_' or '.'";
const DNS1123_SUBDOMAIN_MAX_LENGTH: usize = 253;

static CONFIG_MAP_KEY_RE: OnceLock<Regex> = OnceLock::new();

// ============================================================================
// Lease Validation
// ============================================================================

/// Validates a Lease.
pub fn validate_lease(obj: &internal::Lease) -> ErrorList {
    let mut all_errs = validate_object_meta(
        &obj.metadata,
        true,
        name_is_dns_subdomain,
        &Path::new("metadata"),
    );
    all_errs.extend(validate_lease_spec(&obj.spec, &Path::new("spec")));
    all_errs
}

/// Validates a Lease update.
pub fn validate_lease_update(obj: &internal::Lease, old: &internal::Lease) -> ErrorList {
    let mut all_errs =
        validate_object_meta_update(&obj.metadata, &old.metadata, &Path::new("metadata"));
    all_errs.extend(validate_lease_spec(&obj.spec, &Path::new("spec")));
    all_errs
}

/// Validates a LeaseSpec.
pub fn validate_lease_spec(spec: &internal::LeaseSpec, fld_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if let Some(value) = spec.lease_duration_seconds {
        if value <= 0 {
            all_errs.push(invalid(
                &fld_path.child("leaseDurationSeconds"),
                BadValue::Int(value as i64),
                "must be greater than 0",
            ));
        }
    }

    if let Some(value) = spec.lease_transitions {
        if value < 0 {
            all_errs.push(invalid(
                &fld_path.child("leaseTransitions"),
                BadValue::Int(value as i64),
                "must be greater than or equal to 0",
            ));
        }
    }

    if let Some(ref strategy) = spec.strategy {
        all_errs.extend(validate_coordinated_lease_strategy(
            strategy,
            &fld_path.child("strategy"),
        ));
    }

    if let Some(ref preferred_holder) = spec.preferred_holder {
        if !preferred_holder.is_empty()
            && spec
                .strategy
                .as_ref()
                .map(|value| value.is_empty())
                .unwrap_or(true)
        {
            all_errs.push(forbidden(
                &fld_path.child("preferredHolder"),
                "may only be specified if `strategy` is defined",
            ));
        }
    }

    all_errs
}

// ============================================================================
// LeaseCandidate Validation
// ============================================================================

/// Validates a LeaseCandidate.
pub fn validate_lease_candidate(obj: &internal::LeaseCandidate) -> ErrorList {
    let mut all_errs = validate_object_meta(
        &obj.metadata,
        true,
        valid_lease_candidate_name,
        &Path::new("metadata"),
    );
    all_errs.extend(validate_lease_candidate_spec(&obj.spec, &Path::new("spec")));
    all_errs
}

/// Validates a LeaseCandidate update.
pub fn validate_lease_candidate_update(
    obj: &internal::LeaseCandidate,
    old: &internal::LeaseCandidate,
) -> ErrorList {
    let mut all_errs =
        validate_object_meta_update(&obj.metadata, &old.metadata, &Path::new("metadata"));
    all_errs.extend(validate_lease_candidate_spec(&obj.spec, &Path::new("spec")));
    all_errs.extend(validate_lease_candidate_spec_update(&obj.spec, &old.spec));
    all_errs
}

fn validate_lease_candidate_spec_update(
    spec: &internal::LeaseCandidateSpec,
    old: &internal::LeaseCandidateSpec,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if spec.lease_name != old.lease_name {
        all_errs.push(invalid(
            &Path::new("spec").child("leaseName"),
            BadValue::String(spec.lease_name.clone()),
            "field is immutable",
        ));
    }

    all_errs
}

/// Validates a LeaseCandidateSpec.
pub fn validate_lease_candidate_spec(
    spec: &internal::LeaseCandidateSpec,
    fld_path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if spec.lease_name.is_empty() {
        all_errs.push(required(&fld_path.child("leaseName"), ""));
    }

    let mut ev = Version::new(0, 0, 0);
    if !spec.emulation_version.is_empty() {
        if let Ok(parsed) = Version::parse(&spec.emulation_version) {
            ev = parsed;
        } else {
            all_errs.push(invalid(
                &fld_path.child("emulationVersion"),
                BadValue::String(spec.emulation_version.clone()),
                "must be a valid semantic version",
            ));
        }
    }

    let mut bv = Version::new(0, 0, 0);
    if spec.binary_version.is_empty() {
        all_errs.push(required(&fld_path.child("binaryVersion"), ""));
    } else if let Ok(parsed) = Version::parse(&spec.binary_version) {
        bv = parsed;
    } else {
        all_errs.push(invalid(
            &fld_path.child("binaryVersion"),
            BadValue::String(spec.binary_version.clone()),
            "must be a valid semantic version",
        ));
    }

    if !spec.binary_version.is_empty() && !spec.emulation_version.is_empty() && bv < ev {
        all_errs.push(invalid(
            &fld_path.child("binaryVersion"),
            BadValue::String(spec.binary_version.clone()),
            "must be greater than or equal to `emulationVersion`",
        ));
    }

    if spec.strategy.is_empty() {
        all_errs.push(required(&fld_path.child("strategy"), ""));
    } else {
        if spec.strategy == internal::coordinated_lease_strategy::OLDEST_EMULATION_VERSION {
            let zero = Version::new(0, 0, 0);
            if ev == zero {
                all_errs.push(required(
                    &fld_path.child("emulationVersion"),
                    "must be specified when `strategy` is 'OldestEmulationVersion'",
                ));
            }
        }

        all_errs.extend(validate_coordinated_lease_strategy(
            &spec.strategy,
            &fld_path.child("strategy"),
        ));
    }

    all_errs
}

// ============================================================================
// Strategy Validation
// ============================================================================

/// Validates the Strategy field in both Lease and LeaseCandidate.
pub fn validate_coordinated_lease_strategy(
    strategy: &internal::CoordinatedLeaseStrategy,
    fld_path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let parts: Vec<&str> = strategy.split('/').collect();

    if parts.len() == 1 {
        if !VALID_LEASE_STRATEGIES.iter().any(|value| value == strategy) {
            all_errs.push(not_supported(
                fld_path,
                BadValue::String(strategy.to_string()),
                &VALID_LEASE_STRATEGIES,
            ));
        }
    } else {
        for msg in crate::common::validation::is_qualified_name(strategy) {
            all_errs.push(invalid(
                fld_path,
                BadValue::String(strategy.to_string()),
                &msg,
            ));
        }
    }

    all_errs
}

// ============================================================================
// Name Validation Helpers
// ============================================================================

fn valid_lease_candidate_name(name: &str, _prefix: bool) -> Vec<String> {
    is_config_map_key(name)
}

fn is_config_map_key(value: &str) -> Vec<String> {
    let mut errs = Vec::new();

    if value.len() > DNS1123_SUBDOMAIN_MAX_LENGTH {
        errs.push(max_len_error(DNS1123_SUBDOMAIN_MAX_LENGTH));
    }

    let re =
        CONFIG_MAP_KEY_RE.get_or_init(|| Regex::new(&format!("^{}$", CONFIG_MAP_KEY_FMT)).unwrap());
    if !re.is_match(value) {
        errs.push(regex_error(
            CONFIG_MAP_KEY_ERR_MSG,
            CONFIG_MAP_KEY_FMT,
            &["key.name", "KEY_NAME", "key-name"],
        ));
    }

    errs.extend(has_ch_dir_prefix(value));
    errs
}

fn has_ch_dir_prefix(value: &str) -> Vec<String> {
    match value {
        "." => vec!["must not be '.'".to_string()],
        ".." => vec!["must not be '..'".to_string()],
        _ if value.starts_with("..") => vec!["must not start with '..'".to_string()],
        _ => Vec::new(),
    }
}

fn max_len_error(length: usize) -> String {
    format!("must be no more than {} characters", length)
}

fn regex_error(msg: &str, fmt: &str, examples: &[&str]) -> String {
    if examples.is_empty() {
        return format!("{} (regex used for validation is '{}')", msg, fmt);
    }

    let mut out = format!("{} (e.g. ", msg);
    for (i, example) in examples.iter().enumerate() {
        if i > 0 {
            out.push_str(" or ");
        }
        out.push('\'');
        out.push_str(example);
        out.push_str("', ");
    }
    out.push_str(&format!("regex used for validation is '{}')", fmt));
    out
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::validation::ErrorType;
    use crate::common::{ObjectMeta, TypeMeta};

    fn lease_meta(name: &str) -> ObjectMeta {
        ObjectMeta {
            name: Some(name.to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }
    }

    #[test]
    fn test_validate_lease_valid() {
        let obj = internal::Lease {
            type_meta: TypeMeta::default(),
            metadata: lease_meta("lease-a"),
            spec: internal::LeaseSpec {
                lease_duration_seconds: Some(15),
                ..Default::default()
            },
        };

        let errs = validate_lease(&obj);
        assert!(errs.is_empty(), "expected no errors, got {:?}", errs);
    }

    #[test]
    fn test_validate_lease_invalid_duration() {
        let obj = internal::Lease {
            type_meta: TypeMeta::default(),
            metadata: lease_meta("lease-a"),
            spec: internal::LeaseSpec {
                lease_duration_seconds: Some(0),
                ..Default::default()
            },
        };

        let errs = validate_lease(&obj);
        assert!(
            errs.errors
                .iter()
                .any(|e| e.field == "spec.leaseDurationSeconds")
        );
    }

    #[test]
    fn test_validate_lease_preferred_holder_requires_strategy() {
        let obj = internal::Lease {
            type_meta: TypeMeta::default(),
            metadata: lease_meta("lease-a"),
            spec: internal::LeaseSpec {
                preferred_holder: Some("holder".to_string()),
                ..Default::default()
            },
        };

        let errs = validate_lease(&obj);
        assert!(
            errs.errors
                .iter()
                .any(|e| e.error_type == ErrorType::Forbidden)
        );
    }

    #[test]
    fn test_validate_lease_candidate_required_fields() {
        let obj = internal::LeaseCandidate {
            type_meta: TypeMeta::default(),
            metadata: lease_meta("candidate-a"),
            spec: internal::LeaseCandidateSpec {
                lease_name: String::new(),
                binary_version: String::new(),
                emulation_version: String::new(),
                strategy: String::new(),
                ..Default::default()
            },
        };

        let errs = validate_lease_candidate(&obj);
        assert!(
            errs.errors
                .iter()
                .any(|e| e.error_type == ErrorType::Required)
        );
    }

    #[test]
    fn test_validate_lease_candidate_semver_bounds() {
        let obj = internal::LeaseCandidate {
            type_meta: TypeMeta::default(),
            metadata: lease_meta("candidate-a"),
            spec: internal::LeaseCandidateSpec {
                lease_name: "lease-a".to_string(),
                binary_version: "1.0.0".to_string(),
                emulation_version: "2.0.0".to_string(),
                strategy: internal::coordinated_lease_strategy::OLDEST_EMULATION_VERSION
                    .to_string(),
                ..Default::default()
            },
        };

        let errs = validate_lease_candidate(&obj);
        assert!(
            errs.errors
                .iter()
                .any(|e| e.field == "spec.binaryVersion" && e.error_type == ErrorType::Invalid)
        );
    }
}
