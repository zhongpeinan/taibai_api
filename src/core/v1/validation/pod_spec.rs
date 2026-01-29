//! PodSpec validation for Kubernetes core/v1 API
//!
//! This module implements the main PodSpec validation orchestration, tying together:
//! - Container validation (regular, init, ephemeral)
//! - Volume validation
//! - DNS policy and configuration
//! - Security context
//! - Service account, node name, and other pod-level settings

use crate::common::validation::{BadValue, ErrorList, Path, invalid, not_supported, required};
use crate::core::v1::pod::{PodReadinessGate, PodSchedulingGate, PodSpec};
use crate::core::v1::validation::container::{validate_containers, validate_init_containers};
use crate::core::v1::validation::dns::{validate_dns_policy, validate_pod_dns_config};
use crate::core::v1::validation::volume::validate_volumes;
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

// ============================================================================
// Constants
// ============================================================================

/// Supported restart policies
static SUPPORTED_RESTART_POLICIES: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| HashSet::from(["Always", "OnFailure", "Never"]));

// ============================================================================
// PodSpec Validation
// ============================================================================

/// Validates a PodSpec.
///
/// This is the main validation entry point for pod specifications, orchestrating
/// validation of all pod-level settings and containers.
///
/// Validates:
/// - Termination grace period (required)
/// - Restart policy (required, must be Always/OnFailure/Never)
/// - DNS policy and configuration
/// - Volumes
/// - Containers (regular, init, ephemeral)
/// - Service account name
/// - Node name
/// - Readiness gates
/// - Scheduling gates
/// - Node selector labels
pub fn validate_pod_spec(spec: &PodSpec, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate termination grace period (required)
    if spec.termination_grace_period_seconds.is_none() {
        all_errs.push(required(
            &path.child("terminationGracePeriodSeconds"),
            "terminationGracePeriodSeconds is required",
        ));
    }
    let grace_period = &spec.termination_grace_period_seconds;

    // Validate restart policy
    if let Some(ref policy) = spec.restart_policy {
        all_errs.extend(validate_restart_policy(
            policy,
            &path.child("restartPolicy"),
        ));
    } else {
        all_errs.push(required(
            &path.child("restartPolicy"),
            "restartPolicy is required",
        ));
    }

    // Validate DNS policy
    if let Some(ref dns_policy) = spec.dns_policy {
        all_errs.extend(validate_dns_policy(dns_policy, &path.child("dnsPolicy")));
    } else {
        all_errs.push(required(&path.child("dnsPolicy"), "dnsPolicy is required"));
    }

    // Validate DNS config
    all_errs.extend(validate_pod_dns_config(
        spec.dns_config.as_ref(),
        spec.dns_policy.as_deref(),
        &path.child("dnsConfig"),
    ));

    // Validate volumes
    let (volumes_by_source, volume_errs) = validate_volumes(&spec.volumes, &path.child("volumes"));
    all_errs.extend(volume_errs);

    // Gather pod resource claim names
    let pod_claim_names: HashSet<String> = spec
        .resource_claims
        .iter()
        .map(|claim| claim.name.clone())
        .collect();

    // Build volume name -> type mapping for container validation
    let volumes_by_name: HashMap<String, String> = volumes_by_source
        .iter()
        .map(|(name, _source)| (name.clone(), "Volume".to_string()))
        .collect();

    // Validate regular containers (at least one required)
    all_errs.extend(validate_containers(
        &spec.containers,
        &volumes_by_name,
        &pod_claim_names,
        grace_period,
        &path.child("containers"),
    ));

    // Validate init containers
    if !spec.init_containers.is_empty() {
        all_errs.extend(validate_init_containers(
            &spec.init_containers,
            &spec.containers,
            &volumes_by_name,
            &pod_claim_names,
            grace_period,
            &path.child("initContainers"),
        ));
    }

    // TODO: Validate ephemeral containers (Phase 6)
    // if !spec.ephemeral_containers.is_empty() {
    //     all_errs.extend(validate_ephemeral_containers(...));
    // }

    // Validate service account name (DNS subdomain if specified)
    if let Some(ref sa_name) = spec.service_account_name {
        if !sa_name.is_empty() {
            let dns_errs = crate::common::validation::is_dns1123_subdomain(sa_name);
            for err_msg in dns_errs {
                all_errs.push(invalid(
                    &path.child("serviceAccountName"),
                    BadValue::String(sa_name.clone()),
                    &err_msg,
                ));
            }
        }
    }

    // Validate node name (DNS subdomain if specified)
    if let Some(ref node_name) = spec.node_name {
        if !node_name.is_empty() {
            let dns_errs = crate::common::validation::is_dns1123_subdomain(node_name);
            for err_msg in dns_errs {
                all_errs.push(invalid(
                    &path.child("nodeName"),
                    BadValue::String(node_name.clone()),
                    &err_msg,
                ));
            }
        }
    }

    // Validate node selector labels
    if !spec.node_selector.is_empty() {
        all_errs.extend(crate::common::validation::validate_labels(
            &spec.node_selector,
            &path.child("nodeSelector"),
        ));
    }

    // Validate readiness gates
    if !spec.readiness_gates.is_empty() {
        all_errs.extend(validate_readiness_gates(
            &spec.readiness_gates,
            &path.child("readinessGates"),
        ));
    }

    // Validate scheduling gates
    if !spec.scheduling_gates.is_empty() {
        all_errs.extend(validate_scheduling_gates(
            &spec.scheduling_gates,
            &path.child("schedulingGates"),
        ));
    }

    // TODO: Validate affinity (Phase 6)
    // if let Some(ref affinity) = spec.affinity {
    //     all_errs.extend(validate_affinity(affinity, &path.child("affinity")));
    // }

    // TODO: Validate tolerations (Phase 6)
    // if !spec.tolerations.is_empty() {
    //     all_errs.extend(validate_tolerations(&spec.tolerations, &path.child("tolerations")));
    // }

    // TODO: Validate security context (Phase 6)
    // if let Some(ref sec_ctx) = spec.security_context {
    //     all_errs.extend(validate_pod_security_context(sec_ctx, &path.child("securityContext")));
    // }

    // TODO: Validate topology spread constraints (Phase 6)
    // if !spec.topology_spread_constraints.is_empty() {
    //     all_errs.extend(validate_topology_spread_constraints(...));
    // }

    all_errs
}

// ============================================================================
// Helper Validators
// ============================================================================

/// Validates restart policy.
fn validate_restart_policy(policy: &str, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if policy.is_empty() {
        all_errs.push(required(path, "restartPolicy is required"));
    } else if !SUPPORTED_RESTART_POLICIES.contains(policy) {
        let valid: Vec<&str> = SUPPORTED_RESTART_POLICIES.iter().copied().collect();
        all_errs.push(not_supported(
            path,
            BadValue::String(policy.to_string()),
            &valid,
        ));
    }

    all_errs
}

/// Validates readiness gates.
fn validate_readiness_gates(gates: &[PodReadinessGate], path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (i, gate) in gates.iter().enumerate() {
        let idx_path = path.index(i).child("conditionType");

        // Validate condition type is a qualified name
        let qual_errs =
            crate::common::validation::validate_qualified_name(&gate.condition_type, &idx_path);
        all_errs.extend(qual_errs);
    }

    all_errs
}

/// Validates scheduling gates.
fn validate_scheduling_gates(gates: &[PodSchedulingGate], path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let mut seen = HashSet::new();

    for (i, gate) in gates.iter().enumerate() {
        let idx_path = path.index(i);

        // Validate gate name is a qualified name
        let qual_errs = crate::common::validation::validate_qualified_name(&gate.name, &idx_path);
        all_errs.extend(qual_errs);

        // Check for duplicates
        if seen.contains(&gate.name) {
            all_errs.push(crate::common::validation::duplicate(
                &idx_path,
                BadValue::String(gate.name.clone()),
            ));
        } else {
            seen.insert(gate.name.clone());
        }
    }

    all_errs
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::core::v1::pod::Container;

    #[test]
    fn test_validate_restart_policy_valid() {
        assert!(validate_restart_policy("Always", &Path::nil()).is_empty());
        assert!(validate_restart_policy("OnFailure", &Path::nil()).is_empty());
        assert!(validate_restart_policy("Never", &Path::nil()).is_empty());
    }

    #[test]
    fn test_validate_restart_policy_invalid() {
        let errs = validate_restart_policy("InvalidPolicy", &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.error_type == crate::common::validation::ErrorType::NotSupported)
        );
    }

    #[test]
    fn test_validate_restart_policy_empty() {
        let errs = validate_restart_policy("", &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.detail.contains("restartPolicy is required"))
        );
    }

    #[test]
    fn test_validate_pod_spec_missing_termination_grace_period() {
        let spec = PodSpec {
            termination_grace_period_seconds: None,
            restart_policy: Some("Always".to_string()),
            dns_policy: Some("ClusterFirst".to_string()),
            containers: vec![Container {
                name: "nginx".to_string(),
                image: Some("nginx:latest".to_string()),
                termination_message_policy: Some("File".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        };

        let errs = validate_pod_spec(&spec, &Path::nil());
        assert!(!errs.is_empty());
        assert!(errs.errors.iter().any(|e| {
            e.detail
                .contains("terminationGracePeriodSeconds is required")
        }));
    }

    #[test]
    fn test_validate_pod_spec_missing_restart_policy() {
        let spec = PodSpec {
            termination_grace_period_seconds: Some(30),
            restart_policy: None,
            dns_policy: Some("ClusterFirst".to_string()),
            containers: vec![Container {
                name: "nginx".to_string(),
                image: Some("nginx:latest".to_string()),
                termination_message_policy: Some("File".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        };

        let errs = validate_pod_spec(&spec, &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.detail.contains("restartPolicy is required"))
        );
    }

    #[test]
    fn test_validate_pod_spec_missing_dns_policy() {
        let spec = PodSpec {
            termination_grace_period_seconds: Some(30),
            restart_policy: Some("Always".to_string()),
            dns_policy: None,
            containers: vec![Container {
                name: "nginx".to_string(),
                image: Some("nginx:latest".to_string()),
                termination_message_policy: Some("File".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        };

        let errs = validate_pod_spec(&spec, &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.detail.contains("dnsPolicy is required"))
        );
    }

    #[test]
    fn test_validate_pod_spec_no_containers() {
        let spec = PodSpec {
            termination_grace_period_seconds: Some(30),
            restart_policy: Some("Always".to_string()),
            dns_policy: Some("ClusterFirst".to_string()),
            containers: vec![],
            ..Default::default()
        };

        let errs = validate_pod_spec(&spec, &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.detail.contains("must specify at least one container"))
        );
    }

    #[test]
    fn test_validate_pod_spec_invalid_service_account_name() {
        let spec = PodSpec {
            termination_grace_period_seconds: Some(30),
            restart_policy: Some("Always".to_string()),
            dns_policy: Some("ClusterFirst".to_string()),
            service_account_name: Some("Invalid Service Account!".to_string()),
            containers: vec![Container {
                name: "nginx".to_string(),
                image: Some("nginx:latest".to_string()),
                termination_message_policy: Some("File".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        };

        let errs = validate_pod_spec(&spec, &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.field.ends_with("serviceAccountName"))
        );
    }

    #[test]
    fn test_validate_pod_spec_invalid_node_name() {
        let spec = PodSpec {
            termination_grace_period_seconds: Some(30),
            restart_policy: Some("Always".to_string()),
            dns_policy: Some("ClusterFirst".to_string()),
            node_name: Some("Invalid Node Name!".to_string()),
            containers: vec![Container {
                name: "nginx".to_string(),
                image: Some("nginx:latest".to_string()),
                termination_message_policy: Some("File".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        };

        let errs = validate_pod_spec(&spec, &Path::nil());
        assert!(!errs.is_empty());
        assert!(errs.errors.iter().any(|e| e.field.ends_with("nodeName")));
    }

    #[test]
    fn test_validate_scheduling_gates_duplicate() {
        let gates = vec![
            PodSchedulingGate {
                name: "example.com/gate1".to_string(),
            },
            PodSchedulingGate {
                name: "example.com/gate1".to_string(), // Duplicate
            },
        ];

        let errs = validate_scheduling_gates(&gates, &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.error_type == crate::common::validation::ErrorType::Duplicate)
        );
    }

    #[test]
    fn test_validate_pod_spec_valid() {
        let spec = PodSpec {
            termination_grace_period_seconds: Some(30),
            restart_policy: Some("Always".to_string()),
            dns_policy: Some("ClusterFirst".to_string()),
            containers: vec![Container {
                name: "nginx".to_string(),
                image: Some("nginx:latest".to_string()),
                termination_message_policy: Some("File".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        };

        let errs = validate_pod_spec(&spec, &Path::nil());
        assert!(errs.is_empty(), "Valid PodSpec should not produce errors");
    }
}
