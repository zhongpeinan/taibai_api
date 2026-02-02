//! Unit tests for core v1 defaults
//!
//! This module contains tests for ApplyDefault implementations

use crate::common::ApplyDefault;
use crate::core::internal::{ServiceAffinity, ServiceInternalTrafficPolicy, ServiceType};
use crate::core::v1::persistent_volume::{
    PersistentVolumeClaimSpec, PersistentVolumeClaimStatus, PersistentVolumeSpec,
    PersistentVolumeStatus,
};
use crate::core::v1::pod::{Container, PodSpec};
use crate::core::v1::probe::Probe;
use crate::core::v1::service::{ServicePort, ServiceSpec};

// ============================================================================
// Pod/Container/Probe Tests
// ============================================================================

#[test]
fn test_pod_spec_defaults() {
    let mut spec = PodSpec::default();
    spec.apply_default();

    // Check default values
    assert_eq!(spec.dns_policy, Some("ClusterFirst".to_string()));
    assert_eq!(spec.restart_policy, Some("Always".to_string()));
    assert_eq!(spec.termination_grace_period_seconds, Some(30));
    assert_eq!(spec.scheduler_name, Some("default-scheduler".to_string()));
}

#[test]
fn test_container_defaults_with_latest_tag() {
    let mut container = Container {
        name: "test".to_string(),
        image: Some("nginx:latest".to_string()),
        ..Default::default()
    };
    container.apply_default();

    // Check default values
    assert_eq!(
        container.termination_message_path,
        Some("/dev/termination-log".to_string())
    );
    assert_eq!(
        container.termination_message_policy,
        Some("File".to_string())
    );
    assert_eq!(container.image_pull_policy, Some("Always".to_string()));
}

#[test]
fn test_container_image_pull_policy_for_non_latest() {
    let mut container = Container {
        name: "test".to_string(),
        image: Some("nginx:1.19".to_string()),
        ..Default::default()
    };
    container.apply_default();

    // Should be IfNotPresent for non-latest tag
    assert_eq!(
        container.image_pull_policy,
        Some("IfNotPresent".to_string())
    );
}

#[test]
fn test_container_image_pull_policy_no_tag() {
    let mut container = Container {
        name: "test".to_string(),
        image: Some("nginx".to_string()),
        ..Default::default()
    };
    container.apply_default();

    // No tag implies latest, should be Always
    assert_eq!(container.image_pull_policy, Some("Always".to_string()));
}

#[test]
fn test_probe_defaults() {
    let mut probe = Probe::default();
    probe.apply_default();

    // Check default values
    assert_eq!(probe.timeout_seconds, Some(1));
    assert_eq!(probe.period_seconds, Some(10));
    assert_eq!(probe.success_threshold, Some(1));
    assert_eq!(probe.failure_threshold, Some(3));
}

// ============================================================================
// Service Tests
// ============================================================================

#[test]
fn test_service_spec_defaults() {
    let mut spec = ServiceSpec::default();
    spec.apply_default();

    // Check default service type
    assert_eq!(spec.type_, Some(ServiceType::ClusterIp));

    // Check default internal traffic policy for ClusterIP
    assert_eq!(
        spec.internal_traffic_policy,
        Some(ServiceInternalTrafficPolicy::Cluster)
    );
}

#[test]
fn test_service_spec_load_balancer_defaults() {
    let mut spec = ServiceSpec {
        type_: Some(ServiceType::LoadBalancer),
        ..Default::default()
    };
    spec.apply_default();

    // Check default allocate load balancer node ports
    assert_eq!(spec.allocate_load_balancer_node_ports, Some(true));

    // Check default internal traffic policy
    assert_eq!(
        spec.internal_traffic_policy,
        Some(ServiceInternalTrafficPolicy::Cluster)
    );
}

#[test]
fn test_service_spec_session_affinity_none() {
    let mut spec = ServiceSpec {
        session_affinity: Some(ServiceAffinity::None),
        session_affinity_config: Some(crate::core::v1::service::SessionAffinityConfig {
            client_ip: None,
        }),
        ..Default::default()
    };
    spec.apply_default();

    // Session affinity config should be cleared when session affinity is None
    assert_eq!(spec.session_affinity_config, None);
}

#[test]
fn test_service_port_target_port_default() {
    let mut port = ServicePort {
        port: 8080,
        ..Default::default()
    };
    port.apply_default();

    // Target port should default to port
    assert_eq!(
        port.target_port,
        Some(crate::common::IntOrString::Int(8080))
    );
}

// ============================================================================
// PersistentVolume Tests
// ============================================================================

#[test]
fn test_persistent_volume_spec_defaults() {
    let mut spec = PersistentVolumeSpec::default();
    spec.apply_default();

    // Check default reclaim policy
    assert_eq!(
        spec.persistent_volume_reclaim_policy,
        Some("Retain".to_string())
    );

    // Check default volume mode
    assert_eq!(spec.volume_mode, Some("Filesystem".to_string()));
}

#[test]
fn test_persistent_volume_status_defaults() {
    let mut status = PersistentVolumeStatus::default();
    status.apply_default();

    // Check default phase
    assert_eq!(status.phase, Some("Pending".to_string()));
}

#[test]
fn test_persistent_volume_claim_spec_defaults() {
    let mut spec = PersistentVolumeClaimSpec::default();
    spec.apply_default();

    // Check default volume mode
    assert_eq!(spec.volume_mode, Some("Filesystem".to_string()));
}

#[test]
fn test_persistent_volume_claim_status_defaults() {
    let mut status = PersistentVolumeClaimStatus::default();
    status.apply_default();

    // Check default phase
    assert_eq!(status.phase, Some("Pending".to_string()));
}
