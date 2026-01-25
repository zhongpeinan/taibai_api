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
//! - Compile-time trait checks for all core resources

use crate::common::{
    ApplyDefault, FromInternal, ObjectMeta, ToInternal, TypeMeta, VersionedObject,
};
use crate::core::v1::{Pod, PodSpec};

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
// Test 4: Compile-time Trait Checks for All Core Resources
// ============================================================================

/// 编译时检查：确保所有顶级资源实现了必需的 traits (VersionedObject + ApplyDefault)
#[test]
fn top_level_resources_implement_required_traits() {
    fn check<T: VersionedObject + ApplyDefault>() {}

    // Pod resources
    check::<crate::core::v1::Pod>();

    // Namespace resources
    check::<crate::core::v1::Namespace>();

    // ReplicationController resources
    check::<crate::core::v1::ReplicationController>();

    // Service resources
    check::<crate::core::v1::Service>();
    check::<crate::core::v1::Endpoints>();

    // Config resources
    check::<crate::core::v1::ConfigMap>();
    check::<crate::core::v1::Secret>();
    check::<crate::core::v1::ServiceAccount>();

    // Resource quota resources
    check::<crate::core::v1::LimitRange>();
    check::<crate::core::v1::ResourceQuota>();

    // Node resources
    check::<crate::core::v1::Node>();

    // PersistentVolume resources
    check::<crate::core::v1::PersistentVolume>();
    check::<crate::core::v1::PersistentVolumeClaim>();

    // Binding resources
    check::<crate::core::v1::Binding>();

    // Event resources
    check::<crate::core::v1::Event>();

    // Template resources
    check::<crate::core::v1::PodTemplate>();

    // ComponentStatus resources
    check::<crate::core::v1::ComponentStatus>();

    // PodStatusResult resource
    check::<crate::core::v1::PodStatusResult>();
}

/// 编译时检查：确保资源实现了版本转换 traits
///
/// Note: Core group uses UnimplementedConversion for all types since real conversion
/// logic is complex and will be implemented later.
#[test]
fn conversion_traits() {
    fn check<T, I>()
    where
        T: ToInternal<I> + FromInternal<I>,
    {
    }

    check::<crate::core::v1::Pod, crate::core::internal::Pod>();
    check::<crate::core::v1::Namespace, crate::core::internal::Namespace>();
    check::<crate::core::v1::ReplicationController, crate::core::internal::ReplicationController>();
    check::<crate::core::v1::Service, crate::core::internal::Service>();
    check::<crate::core::v1::Endpoints, crate::core::internal::Endpoints>();
    check::<crate::core::v1::ConfigMap, crate::core::internal::ConfigMap>();
    check::<crate::core::v1::Secret, crate::core::internal::Secret>();
    check::<crate::core::v1::ServiceAccount, crate::core::internal::ServiceAccount>();
    check::<crate::core::v1::LimitRange, crate::core::internal::LimitRange>();
    check::<crate::core::v1::ResourceQuota, crate::core::internal::ResourceQuota>();
    check::<crate::core::v1::Node, crate::core::internal::Node>();
    check::<crate::core::v1::PersistentVolume, crate::core::internal::PersistentVolume>();
    check::<crate::core::v1::PersistentVolumeClaim, crate::core::internal::PersistentVolumeClaim>();
    check::<crate::core::v1::Binding, crate::core::internal::Binding>();
    check::<crate::core::v1::Event, crate::core::internal::Event>();
    check::<crate::core::v1::PodTemplate, crate::core::internal::PodTemplate>();
    check::<crate::core::v1::ComponentStatus, crate::core::internal::ComponentStatus>();
    check::<crate::core::v1::PodStatusResult, crate::core::internal::PodStatusResult>();
}

/// 编译时检查：确保资源实现了 prost::Message
#[test]
fn prost_message_v1() {
    fn check<T: prost::Message>() {}

    // v1 resources and lists
    // check::<crate::core::v1::Pod>(); // TODO: Re-enable after implementing prost::Message
    // check::<crate::core::v1::PodList>(); // TODO: Re-enable after implementing prost::Message
    check::<crate::core::v1::Namespace>();
    check::<crate::core::v1::NamespaceList>();
    check::<crate::core::v1::ReplicationController>();
    check::<crate::core::v1::ReplicationControllerList>();
    check::<crate::core::v1::Service>();
    check::<crate::core::v1::ServiceList>();
    check::<crate::core::v1::Endpoints>();
    check::<crate::core::v1::EndpointsList>();
    check::<crate::core::v1::ConfigMap>();
    check::<crate::core::v1::ConfigMapList>();
    check::<crate::core::v1::Secret>();
    check::<crate::core::v1::SecretList>();
    check::<crate::core::v1::ServiceAccount>();
    check::<crate::core::v1::ServiceAccountList>();
    check::<crate::core::v1::LimitRange>();
    check::<crate::core::v1::LimitRangeList>();
    check::<crate::core::v1::ResourceQuota>();
    check::<crate::core::v1::ResourceQuotaList>();
    check::<crate::core::v1::Node>();
    check::<crate::core::v1::NodeList>();
    check::<crate::core::v1::PersistentVolume>();
    check::<crate::core::v1::PersistentVolumeList>();
    check::<crate::core::v1::PersistentVolumeClaim>();
    check::<crate::core::v1::PersistentVolumeClaimList>();
    check::<crate::core::v1::Binding>();
    check::<crate::core::v1::Event>();
    check::<crate::core::v1::EventList>();
    check::<crate::core::v1::PodTemplate>();
    check::<crate::core::v1::PodTemplateList>();
    check::<crate::core::v1::ComponentStatus>();
    check::<crate::core::v1::ComponentStatusList>();
    check::<crate::core::v1::PodStatusResult>();
}
