//! Trait implementation tests for extensions/v1beta1
//!
//! This module verifies that all top-level resources implement required traits.

use super::*;
use crate::common::{
    ApplyDefault, FromInternal, HasObjectMeta, ResourceSchema, ToInternal, VersionedObject,
};
use crate::extensions::internal;

/// Compile-time check: verify all top-level resources implement required traits
#[test]
fn top_level_resources_implement_required_traits() {
    // Helper functions that enforce trait bounds (compile-time only)
    fn check_versioned<T: VersionedObject + ApplyDefault>() {}
    fn check_default<T: Default>() {}
    fn check_schema<T: ResourceSchema>() {}

    // Top-level resources in extensions/v1beta1
    check_versioned::<Deployment>();
    check_default::<Deployment>();
    check_schema::<Deployment>();
    check_default::<DeploymentList>();
    check_schema::<DeploymentList>();

    check_versioned::<DaemonSet>();
    check_default::<DaemonSet>();
    check_schema::<DaemonSet>();
    check_default::<DaemonSetList>();
    check_schema::<DaemonSetList>();

    check_versioned::<ReplicaSet>();
    check_default::<ReplicaSet>();
    check_schema::<ReplicaSet>();
    check_default::<ReplicaSetList>();
    check_schema::<ReplicaSetList>();

    check_versioned::<Ingress>();
    check_default::<Ingress>();
    check_schema::<Ingress>();
    check_default::<IngressList>();
    check_schema::<IngressList>();

    check_versioned::<NetworkPolicy>();
    check_default::<NetworkPolicy>();
    check_schema::<NetworkPolicy>();
    check_default::<NetworkPolicyList>();
    check_schema::<NetworkPolicyList>();
}

/// Compile-time check: verify prost::Message trait is implemented
#[test]
fn top_level_resources_implement_prost_message() {
    fn check_prost<T: prost::Message>() {}

    // All top-level resources implement prost::Message
    check_prost::<Deployment>();
    check_prost::<DeploymentList>();
    check_prost::<DaemonSet>();
    check_prost::<DaemonSetList>();
    check_prost::<ReplicaSet>();
    check_prost::<ReplicaSetList>();
    check_prost::<Ingress>();
    check_prost::<IngressList>();
    check_prost::<NetworkPolicy>();
    check_prost::<NetworkPolicyList>();
}

/// Compile-time check: verify conversion traits are implemented
#[test]
fn top_level_resources_have_conversion_traits() {
    fn check_conversion<T, I>()
    where
        T: ToInternal<I> + FromInternal<I>,
    {
    }

    // Note: Internal types are re-exported from v1beta1, so they're identical
    // but we still verify the conversion traits are implemented
    check_conversion::<Deployment, internal::Deployment>();
    check_conversion::<DeploymentList, internal::DeploymentList>();
    check_conversion::<DaemonSet, internal::DaemonSet>();
    check_conversion::<DaemonSetList, internal::DaemonSetList>();
    check_conversion::<ReplicaSet, internal::ReplicaSet>();
    check_conversion::<ReplicaSetList, internal::ReplicaSetList>();
    check_conversion::<Ingress, internal::Ingress>();
    check_conversion::<IngressList, internal::IngressList>();
    check_conversion::<NetworkPolicy, internal::NetworkPolicy>();
    check_conversion::<NetworkPolicyList, internal::NetworkPolicyList>();
}

/// Compile-time check: verify internal resources implement HasObjectMeta
#[test]
fn internal_resources_implement_required_traits() {
    fn check<T: HasObjectMeta>() {}

    // Note: Internal types are re-exported from v1beta1, so they're identical
    // but we still verify HasObjectMeta is implemented
    check::<internal::Deployment>();
    check::<internal::DaemonSet>();
    check::<internal::ReplicaSet>();
    check::<internal::Ingress>();
    check::<internal::NetworkPolicy>();
    // Note: List types don't implement HasObjectMeta (they have ListMeta)
}

/// Runtime check: VersionedObject trait provides correct metadata access
#[test]
fn versioned_object_metadata_access() {
    // Test with Deployment
    let resource = Deployment::default();
    let meta = resource.metadata();
    assert!(meta.name.is_none());

    // Test metadata_mut auto-inserts default
    let mut resource = Deployment::default();
    resource.metadata_mut().name = Some("test".to_string());
    assert_eq!(resource.metadata().name.as_deref(), Some("test"));
}

/// Runtime check: ApplyDefault correctly sets TypeMeta
#[test]
fn apply_default_sets_type_meta() {
    // Test Deployment
    let mut deployment = Deployment::default();
    deployment.apply_default();
    assert_eq!(deployment.type_meta.api_version, "extensions/v1beta1");
    assert_eq!(deployment.type_meta.kind, "Deployment");

    // Test DaemonSet
    let mut daemonset = DaemonSet::default();
    daemonset.apply_default();
    assert_eq!(daemonset.type_meta.api_version, "extensions/v1beta1");
    assert_eq!(daemonset.type_meta.kind, "DaemonSet");

    // Test ReplicaSet
    let mut replicaset = ReplicaSet::default();
    replicaset.apply_default();
    assert_eq!(replicaset.type_meta.api_version, "extensions/v1beta1");
    assert_eq!(replicaset.type_meta.kind, "ReplicaSet");

    // Test Ingress
    let mut ingress = Ingress::default();
    ingress.apply_default();
    assert_eq!(ingress.type_meta.api_version, "extensions/v1beta1");
    assert_eq!(ingress.type_meta.kind, "Ingress");

    // Test NetworkPolicy
    let mut policy = NetworkPolicy::default();
    policy.apply_default();
    assert_eq!(policy.type_meta.api_version, "extensions/v1beta1");
    assert_eq!(policy.type_meta.kind, "NetworkPolicy");
}
