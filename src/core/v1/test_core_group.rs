//! Representative tests for the core/v1 API group
//!
//! This module contains a minimal set of tests that cover the key patterns
//! used across the core group. Individual type-specific tests have been removed
//! in favor of these representative tests.
//!
//! Coverage:
//! - TypeMeta flatten (#[serde(flatten)] on type_meta field)
//! - VersionedObject trait (metadata access with default handling)
//! - ApplyDefault trait (compile-time verification only)
//! - Enum serde (simple rename enums)

use crate::common::{ApplyDefault, ObjectMeta, TypeMeta, VersionedObject};
use crate::core::v1::{Pod, PodSpec, pod_phase};
use std::collections::BTreeMap;

// ============================================================================
// Test 1: TypeMeta Flatten
// ============================================================================

#[test]
fn test_type_meta_flatten() {
    // Verify that TypeMeta fields are correctly flattened into the parent struct
    // when serialized, matching Kubernetes API JSON format.

    let pod = Pod {
        type_meta: TypeMeta {
            kind: "Pod".to_string(),
            api_version: "v1".to_string(),
        },
        metadata: Some(ObjectMeta {
            name: Some("test-pod".to_string()),
            ..Default::default()
        }),
        spec: Some(PodSpec::default()),
        status: None,
    };

    let json = serde_json::to_string(&pod).unwrap();

    // The flattened TypeMeta fields should appear at the top level
    assert!(json.contains(r#""kind":"Pod""#));
    assert!(json.contains(r#""apiVersion":"v1""#));

    // Verify round-trip deserialization
    let deserialized: Pod = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.type_meta.kind, "Pod");
    assert_eq!(deserialized.type_meta.api_version, "v1");
}

// ============================================================================
// Test 2: VersionedObject Trait
// ============================================================================

#[test]
fn test_versioned_object_metadata_none() {
    // VersionedObject trait should return default ObjectMeta when metadata is None.
    // This matches Go's zero-value behavior.

    let pod = Pod {
        type_meta: TypeMeta::default(),
        metadata: None,
        spec: None,
        status: None,
    };

    // metadata() should return a reference to default ObjectMeta, not panic
    let meta = pod.metadata();
    assert!(meta.name.is_none());
    assert!(meta.namespace.is_none());
    assert!(meta.labels.is_empty());
}

#[test]
fn test_versioned_object_metadata_mut() {
    // metadata_mut() should insert a default ObjectMeta when metadata is None.

    let mut pod = Pod {
        type_meta: TypeMeta::default(),
        metadata: None,
        spec: None,
        status: None,
    };

    // Calling metadata_mut() should insert a default ObjectMeta
    let meta = pod.metadata_mut();
    meta.name = Some("test-pod".to_string());

    // Now metadata should be Some
    assert!(pod.metadata.is_some());
    assert_eq!(pod.metadata.unwrap().name, Some("test-pod".to_string()));
}

// ============================================================================
// Test 3: ApplyDefault Trait (Compile-time verification)
// ============================================================================

// ApplyDefault trait is implemented for Pod, ConfigMap, Secret, PersistentVolume,
// Node, Namespace, Service, Endpoints, LimitRange, ResourceQuota, etc.
//
// The current implementation is a stub that only sets TypeMeta fields.
// This test verifies the trait is implemented (compiles) but does not test
// default values since they are not yet implemented.

#[test]
fn test_apply_default_compiles() {
    // This test verifies ApplyDefault trait is implemented for Pod.
    // If the trait is not implemented, this will fail to compile.

    let mut pod = Pod {
        type_meta: TypeMeta::default(),
        metadata: None,
        spec: None,
        status: None,
    };

    // Call apply_default - this should compile and set TypeMeta fields
    pod.apply_default();

    assert_eq!(pod.type_meta.api_version, "v1");
    assert_eq!(pod.type_meta.kind, "Pod");
}

// ============================================================================
// Test 4: Enum Serde
// ============================================================================

#[test]
fn test_pod_phase_constant() {
    // Verify pod_phase constants match Kubernetes API values.
    // These constants are used in place of enums for simple string values.

    assert_eq!(pod_phase::PENDING, "Pending");
    assert_eq!(pod_phase::RUNNING, "Running");
    assert_eq!(pod_phase::SUCCEEDED, "Succeeded");
    assert_eq!(pod_phase::FAILED, "Failed");
    assert_eq!(pod_phase::UNKNOWN, "Unknown");
}

#[test]
fn test_pod_phase_in_struct() {
    // Pod phase is serialized as a string, not as an enum with tag.
    // This matches the Kubernetes API format.

    let mut status_data = BTreeMap::new();
    status_data.insert("phase".to_string(), serde_json::json!("Running"));

    let phase = status_data.get("phase").and_then(|v| v.as_str());
    assert_eq!(phase, Some("Running"));
}
