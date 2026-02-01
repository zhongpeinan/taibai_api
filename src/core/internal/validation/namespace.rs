//! Namespace validation for Kubernetes core internal API
//!
//! Ported from k8s.io/kubernetes/pkg/apis/core/validation/validation.go

use crate::common::validation::{BadValue, ErrorList, Path, invalid};
use crate::core::internal::{Namespace, NamespacePhase};

/// Validates a Namespace
pub fn validate_namespace(namespace: &Namespace) -> ErrorList {
    validate_namespace_with_path(namespace, &Path::nil())
}

fn validate_namespace_with_path(namespace: &Namespace, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate metadata (namespace is cluster-scoped, so namespace=false)
    all_errs.extend(crate::common::validation::validate_object_meta(
        &namespace.metadata,
        false, // Namespace is cluster-scoped (not namespaced)
        validate_namespace_name,
        &path.child("metadata"),
    ));

    // Validate spec
    if let Some(ref spec) = namespace.spec {
        // Validate finalizers
        for (i, finalizer) in spec.finalizers.iter().enumerate() {
            all_errs.extend(validate_finalizer_name(
                finalizer,
                &path.child("spec").child("finalizers").index(i),
            ));
        }
    }

    all_errs
}

/// Validates Namespace update
pub fn validate_namespace_update(new: &Namespace, old: &Namespace) -> ErrorList {
    validate_namespace_update_with_path(new, old, &Path::nil())
}

fn validate_namespace_update_with_path(new: &Namespace, old: &Namespace, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate metadata update
    all_errs.extend(crate::common::validation::validate_object_meta_update(
        &new.metadata,
        &old.metadata,
        &path.child("metadata"),
    ));

    // Validate the new namespace
    all_errs.extend(validate_namespace_with_path(new, path));

    all_errs
}

/// Validates Namespace status update
///
/// This validates phase transitions based on deletionTimestamp:
/// - If deletionTimestamp is zero/empty, phase must be "Active"
/// - If deletionTimestamp is set, phase must be "Terminating"
pub fn validate_namespace_status_update(new: &Namespace, old: &Namespace) -> ErrorList {
    validate_namespace_status_update_with_path(new, old, &Path::nil())
}

fn validate_namespace_status_update_with_path(
    new: &Namespace,
    old: &Namespace,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate metadata update
    all_errs.extend(crate::common::validation::validate_object_meta_update(
        &new.metadata,
        &old.metadata,
        &path.child("metadata"),
    ));

    // Validate phase based on deletionTimestamp
    if let Some(ref phase) = new.status.phase {
        let deletion_timestamp_is_zero = new.metadata.deletion_timestamp.as_ref().is_none();

        if deletion_timestamp_is_zero {
            // If deletionTimestamp is empty, phase must be Active
            if !matches!(phase, NamespacePhase::Active) {
                all_errs.push(invalid(
                    &path.child("status").child("phase"),
                    BadValue::String(namespace_phase_to_str(phase).to_string()),
                    "may only be 'Active' if `deletionTimestamp` is empty",
                ));
            }
        } else {
            // If deletionTimestamp is set, phase must be Terminating
            if !matches!(phase, NamespacePhase::Terminating) {
                all_errs.push(invalid(
                    &path.child("status").child("phase"),
                    BadValue::String(namespace_phase_to_str(phase).to_string()),
                    "may only be 'Terminating' if `deletionTimestamp` is not empty",
                ));
            }
        }
    }

    all_errs
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Validates a namespace name (DNS subdomain)
fn validate_namespace_name(name: &str, _prefix: bool) -> Vec<String> {
    crate::common::validation::is_dns1123_subdomain(name)
}

/// Validates a finalizer name
///
/// Finalizers must be either:
/// 1. A qualified name (contains '/') validated by apimachinery validation
/// 2. A standard Kubernetes finalizer name (e.g., "kubernetes")
fn validate_finalizer_name(finalizer: &str, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Check if it's a qualified name (contains '/')
    if finalizer.contains('/') {
        // Validate as a qualified name
        let errors = crate::common::validation::is_qualified_name(finalizer);
        for err in errors {
            all_errs.push(invalid(path, BadValue::String(finalizer.to_string()), &err));
        }
    } else {
        // Must be a standard Kubernetes finalizer
        if !is_standard_finalizer(finalizer) {
            all_errs.push(invalid(
                path,
                BadValue::String(finalizer.to_string()),
                "name is neither a standard finalizer name nor is it fully qualified",
            ));
        }
    }

    all_errs
}

/// Checks if a finalizer name is a standard Kubernetes finalizer
fn is_standard_finalizer(name: &str) -> bool {
    matches!(name, "kubernetes")
}

fn namespace_phase_to_str(value: &NamespacePhase) -> &'static str {
    match value {
        NamespacePhase::Active => "Active",
        NamespacePhase::Terminating => "Terminating",
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{ObjectMeta, Timestamp};
    use crate::core::internal::namespace::{NamespaceSpec, NamespaceStatus};

    fn create_test_namespace(name: &str) -> Namespace {
        use crate::common::TypeMeta;
        Namespace {
            type_meta: TypeMeta::default(),
            metadata: ObjectMeta {
                name: Some(name.to_string()),
                ..Default::default()
            },
            spec: None,
            status: NamespaceStatus::default(),
        }
    }

    #[test]
    fn test_validate_namespace_valid() {
        let mut namespace = create_test_namespace("test-namespace");
        namespace.spec = Some(NamespaceSpec {
            finalizers: vec!["kubernetes".to_string()],
        });

        let errs = validate_namespace(&namespace);
        assert!(errs.is_empty(), "Expected no errors, got: {:?}", errs);
    }

    #[test]
    fn test_validate_namespace_missing_name() {
        use crate::common::TypeMeta;
        let namespace = Namespace {
            type_meta: TypeMeta::default(),
            metadata: ObjectMeta::default(),
            spec: None,
            status: NamespaceStatus::default(),
        };

        let errs = validate_namespace(&namespace);
        assert!(!errs.is_empty(), "Expected errors for missing name");
    }

    #[test]
    fn test_validate_namespace_invalid_name() {
        let namespace = create_test_namespace("INVALID_NAME");

        let errs = validate_namespace(&namespace);
        assert!(!errs.is_empty(), "Expected errors for invalid name");
    }

    #[test]
    fn test_validate_namespace_valid_qualified_finalizer() {
        let mut namespace = create_test_namespace("test-namespace");
        namespace.spec = Some(NamespaceSpec {
            finalizers: vec!["example.com/my-finalizer".to_string()],
        });

        let errs = validate_namespace(&namespace);
        assert!(errs.is_empty(), "Expected no errors, got: {:?}", errs);
    }

    #[test]
    fn test_validate_namespace_invalid_finalizer() {
        let mut namespace = create_test_namespace("test-namespace");
        namespace.spec = Some(NamespaceSpec {
            finalizers: vec!["invalid-finalizer".to_string()],
        });

        let errs = validate_namespace(&namespace);
        assert!(
            !errs.is_empty(),
            "Expected errors for invalid finalizer name"
        );
    }

    #[test]
    fn test_validate_namespace_status_update_active_phase() {
        let mut namespace = create_test_namespace("test-namespace");
        namespace.metadata.resource_version = Some("123".to_string());
        namespace.metadata.uid = Some("test-uid".to_string());
        namespace.status = NamespaceStatus {
            phase: Some(NamespacePhase::Active),
            ..Default::default()
        };

        let mut old_namespace = create_test_namespace("test-namespace");
        old_namespace.metadata.resource_version = Some("123".to_string());
        old_namespace.metadata.uid = Some("test-uid".to_string());

        let errs = validate_namespace_status_update(&namespace, &old_namespace);
        assert!(errs.is_empty(), "Expected no errors, got: {:?}", errs);
    }

    #[test]
    fn test_validate_namespace_status_update_terminating_phase() {
        let deletion_ts = Timestamp::now();
        let mut namespace = create_test_namespace("test-namespace");
        namespace.metadata.resource_version = Some("123".to_string());
        namespace.metadata.uid = Some("test-uid".to_string());
        namespace.metadata.deletion_timestamp = Some(deletion_ts.clone());
        namespace.status = NamespaceStatus {
            phase: Some(NamespacePhase::Terminating),
            ..Default::default()
        };

        let mut old_namespace = create_test_namespace("test-namespace");
        old_namespace.metadata.resource_version = Some("123".to_string());
        old_namespace.metadata.uid = Some("test-uid".to_string());
        old_namespace.metadata.deletion_timestamp = Some(deletion_ts);

        let errs = validate_namespace_status_update(&namespace, &old_namespace);
        assert!(errs.is_empty(), "Expected no errors, got: {:?}", errs);
    }

    #[test]
    fn test_validate_namespace_status_update_invalid_phase_active_with_deletion() {
        let deletion_ts = Timestamp::now();
        let mut namespace = create_test_namespace("test-namespace");
        namespace.metadata.resource_version = Some("123".to_string());
        namespace.metadata.uid = Some("test-uid".to_string());
        namespace.metadata.deletion_timestamp = Some(deletion_ts.clone());
        namespace.status = NamespaceStatus {
            phase: Some(NamespacePhase::Active), // Invalid: should be Terminating
            ..Default::default()
        };

        let mut old_namespace = create_test_namespace("test-namespace");
        old_namespace.metadata.resource_version = Some("123".to_string());
        old_namespace.metadata.uid = Some("test-uid".to_string());
        old_namespace.metadata.deletion_timestamp = Some(deletion_ts);

        let errs = validate_namespace_status_update(&namespace, &old_namespace);
        // Find the phase validation error (ignore metadata update errors)
        let phase_errors: Vec<_> = errs
            .errors
            .iter()
            .filter(|e| e.field.contains("phase"))
            .collect();
        assert!(
            !phase_errors.is_empty(),
            "Expected phase validation error for invalid phase with deletion timestamp"
        );
    }

    #[test]
    fn test_validate_namespace_status_update_invalid_phase_terminating_without_deletion() {
        let mut namespace = create_test_namespace("test-namespace");
        namespace.metadata.resource_version = Some("123".to_string());
        namespace.metadata.uid = Some("test-uid".to_string());
        namespace.status = NamespaceStatus {
            phase: Some(NamespacePhase::Terminating), // Invalid: should be Active
            ..Default::default()
        };

        let mut old_namespace = create_test_namespace("test-namespace");
        old_namespace.metadata.resource_version = Some("123".to_string());
        old_namespace.metadata.uid = Some("test-uid".to_string());

        let errs = validate_namespace_status_update(&namespace, &old_namespace);
        // Find the phase validation error (ignore metadata update errors)
        let phase_errors: Vec<_> = errs
            .errors
            .iter()
            .filter(|e| e.field.contains("phase"))
            .collect();
        assert!(
            !phase_errors.is_empty(),
            "Expected phase validation error for invalid phase without deletion timestamp"
        );
    }
}
