//! Node validation for Kubernetes core/v1 API
//!
//! Ported from k8s.io/kubernetes/pkg/apis/core/validation/validation.go

use crate::common::validation::{BadValue, ErrorList, Path, forbidden, invalid, required};
use crate::core::v1::Node;

/// Validates a Node
pub fn validate_node(node: &Node) -> ErrorList {
    validate_node_with_path(node, &Path::nil())
}

fn validate_node_with_path(node: &Node, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate metadata (Node is cluster-scoped, so namespace=false)
    if let Some(ref metadata) = node.metadata {
        all_errs.extend(crate::common::validation::validate_object_meta(
            metadata,
            false, // Node is cluster-scoped (not namespaced)
            validate_node_name,
            &path.child("metadata"),
        ));
    } else {
        all_errs.push(required(&path.child("metadata"), "metadata is required"));
    }

    // Validate spec
    if let Some(ref spec) = node.spec {
        // Validate PodCIDRs
        if !spec.pod_cidrs.is_empty() {
            for (i, cidr) in spec.pod_cidrs.iter().enumerate() {
                // Basic CIDR validation - check format
                if !is_valid_cidr(cidr) {
                    all_errs.push(invalid(
                        &path.child("spec").child("podCIDRs").index(i),
                        BadValue::String(cidr.clone()),
                        "must be a valid CIDR",
                    ));
                }
            }

            // Check for dual-stack constraints (at most 2 CIDRs, one IPv4 and one IPv6)
            if spec.pod_cidrs.len() > 2 {
                all_errs.push(invalid(
                    &path.child("spec").child("podCIDRs"),
                    BadValue::String(format!("{} CIDRs", spec.pod_cidrs.len())),
                    "may specify no more than one CIDR for each IP family",
                ));
            }
        }

        // Validate taints
        for (i, taint) in spec.taints.iter().enumerate() {
            all_errs.extend(validate_taint(
                taint,
                &path.child("spec").child("taints").index(i),
            ));
        }
    }

    // Validate status if present
    if let Some(ref status) = node.status {
        // Validate capacity resources
        for (resource_name, quantity) in &status.capacity {
            all_errs.extend(validate_resource_quantity(
                resource_name,
                quantity,
                &path.child("status").child("capacity").key(resource_name),
            ));
        }

        // Validate allocatable resources
        for (resource_name, quantity) in &status.allocatable {
            all_errs.extend(validate_resource_quantity(
                resource_name,
                quantity,
                &path.child("status").child("allocatable").key(resource_name),
            ));
        }

        // Check for duplicate addresses
        let mut seen_addresses = std::collections::HashSet::new();
        for (i, address) in status.addresses.iter().enumerate() {
            let key = (&address.type_, &address.address);
            if seen_addresses.contains(&key) {
                all_errs.push(invalid(
                    &path.child("status").child("addresses").index(i),
                    BadValue::String(format!("{}={}", address.type_, address.address)),
                    "duplicate address",
                ));
            }
            seen_addresses.insert(key);
        }
    }

    all_errs
}

/// Validates Node update
pub fn validate_node_update(new: &Node, old: &Node) -> ErrorList {
    validate_node_update_with_path(new, old, &Path::nil())
}

fn validate_node_update_with_path(new: &Node, old: &Node, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate metadata update
    if let (Some(new_meta), Some(old_meta)) = (&new.metadata, &old.metadata) {
        all_errs.extend(crate::common::validation::validate_object_meta_update(
            new_meta,
            old_meta,
            &path.child("metadata"),
        ));
    }

    // Validate the new node
    all_errs.extend(validate_node_with_path(new, path));

    // Check immutability constraints
    if let (Some(new_spec), Some(old_spec)) = (&new.spec, &old.spec) {
        // PodCIDRs cannot be changed (except from empty to non-empty)
        if !old_spec.pod_cidrs.is_empty() && new_spec.pod_cidrs != old_spec.pod_cidrs {
            all_errs.push(forbidden(
                &path.child("spec").child("podCIDRs"),
                "node updates may not change podCIDR except from \"\" to valid",
            ));
        }

        // ProviderID cannot be changed (except from empty to non-empty)
        if let Some(ref old_provider_id) = old_spec.provider_id {
            if !old_provider_id.is_empty() {
                if new_spec.provider_id.as_ref() != Some(old_provider_id) {
                    all_errs.push(forbidden(
                        &path.child("spec").child("providerID"),
                        "node updates may not change providerID except from \"\" to valid",
                    ));
                }
            }
        }
    }

    all_errs
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Validates a node name (DNS subdomain)
fn validate_node_name(name: &str, _prefix: bool) -> Vec<String> {
    crate::common::validation::is_dns1123_subdomain(name)
}

/// Basic CIDR validation
fn is_valid_cidr(cidr: &str) -> bool {
    // Check for basic CIDR format: address/prefix
    if !cidr.contains('/') {
        return false;
    }

    let parts: Vec<&str> = cidr.split('/').collect();
    if parts.len() != 2 {
        return false;
    }

    // Check prefix length is numeric
    if parts[1].parse::<u8>().is_err() {
        return false;
    }

    // Basic check for IPv4 or IPv6 address format
    let addr = parts[0];
    // IPv4: contains dots
    // IPv6: contains colons
    addr.contains('.') || addr.contains(':')
}

/// Validates a taint
fn validate_taint(taint: &crate::core::v1::Taint, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate key (must be a qualified name or DNS subdomain)
    if taint.key.is_empty() {
        all_errs.push(required(&path.child("key"), "key is required"));
    } else {
        let key_errors = crate::common::validation::is_qualified_name(&taint.key);
        for err in key_errors {
            all_errs.push(invalid(
                &path.child("key"),
                BadValue::String(taint.key.clone()),
                &err,
            ));
        }
    }

    // Validate effect (must be one of NoSchedule, PreferNoSchedule, NoExecute)
    let valid_effects = ["NoSchedule", "PreferNoSchedule", "NoExecute"];
    if let Some(ref effect) = taint.effect {
        if !valid_effects.contains(&effect.as_str()) {
            all_errs.push(crate::common::validation::not_supported(
                &path.child("effect"),
                BadValue::String(effect.clone()),
                &valid_effects,
            ));
        }
    } else {
        all_errs.push(required(&path.child("effect"), "effect is required"));
    }

    all_errs
}

/// Validates a resource quantity
fn validate_resource_quantity(
    resource_name: &str,
    quantity: &crate::common::Quantity,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Check that quantity is not negative or empty
    let value = &quantity.0;

    // Parse the quantity string to check for validity
    if value.is_empty() {
        all_errs.push(invalid(
            path,
            BadValue::String(value.clone()),
            "must be a valid quantity",
        ));
    }

    // Check for negative values (basic check for leading minus)
    if value.starts_with('-') {
        all_errs.push(invalid(
            path,
            BadValue::String(value.clone()),
            "must be greater than or equal to 0",
        ));
    }

    // Validate specific resource names
    match resource_name {
        "cpu" | "memory" | "storage" | "ephemeral-storage" => {
            // Standard resources - already validated above
        }
        _ => {
            // Extended resources must be fully qualified
            if !resource_name.contains('/') {
                all_errs.push(invalid(
                    path,
                    BadValue::String(resource_name.to_string()),
                    "extended resource name must be fully qualified",
                ));
            }
        }
    }

    all_errs
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{ObjectMeta, Quantity, TypeMeta};
    use crate::core::v1::{NodeAddress, NodeSpec, NodeStatus, Taint};
    use std::collections::BTreeMap;

    fn create_test_node(name: &str) -> Node {
        Node {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some(name.to_string()),
                ..Default::default()
            }),
            spec: None,
            status: None,
        }
    }

    #[test]
    fn test_validate_node_valid() {
        let node = create_test_node("test-node");

        let errs = validate_node(&node);
        assert!(errs.is_empty(), "Expected no errors, got: {:?}", errs);
    }

    #[test]
    fn test_validate_node_missing_name() {
        let mut node = create_test_node("test-node");
        node.metadata.as_mut().unwrap().name = None;

        let errs = validate_node(&node);
        assert!(!errs.is_empty(), "Expected errors for missing name");
    }

    #[test]
    fn test_validate_node_invalid_name() {
        let node = create_test_node("INVALID_NAME");

        let errs = validate_node(&node);
        assert!(!errs.is_empty(), "Expected errors for invalid name");
    }

    #[test]
    fn test_validate_node_valid_pod_cidrs() {
        let mut node = create_test_node("test-node");
        node.spec = Some(NodeSpec {
            pod_cidrs: vec!["10.244.0.0/24".to_string(), "fd00::/64".to_string()],
            ..Default::default()
        });

        let errs = validate_node(&node);
        assert!(errs.is_empty(), "Expected no errors, got: {:?}", errs);
    }

    #[test]
    fn test_validate_node_too_many_pod_cidrs() {
        let mut node = create_test_node("test-node");
        node.spec = Some(NodeSpec {
            pod_cidrs: vec![
                "10.244.0.0/24".to_string(),
                "10.244.1.0/24".to_string(),
                "10.244.2.0/24".to_string(),
            ],
            ..Default::default()
        });

        let errs = validate_node(&node);
        assert!(!errs.is_empty(), "Expected errors for too many podCIDRs");
    }

    #[test]
    fn test_validate_node_invalid_cidr() {
        let mut node = create_test_node("test-node");
        node.spec = Some(NodeSpec {
            pod_cidrs: vec!["invalid-cidr".to_string()],
            ..Default::default()
        });

        let errs = validate_node(&node);
        assert!(!errs.is_empty(), "Expected errors for invalid CIDR");
    }

    #[test]
    fn test_validate_node_valid_taint() {
        let mut node = create_test_node("test-node");
        node.spec = Some(NodeSpec {
            taints: vec![Taint {
                key: "node-role.kubernetes.io/master".to_string(),
                value: Some("true".to_string()),
                effect: Some("NoSchedule".to_string()),
                time_added: None,
            }],
            ..Default::default()
        });

        let errs = validate_node(&node);
        assert!(errs.is_empty(), "Expected no errors, got: {:?}", errs);
    }

    #[test]
    fn test_validate_node_invalid_taint_effect() {
        let mut node = create_test_node("test-node");
        node.spec = Some(NodeSpec {
            taints: vec![Taint {
                key: "key1".to_string(),
                value: None,
                effect: Some("InvalidEffect".to_string()),
                time_added: None,
            }],
            ..Default::default()
        });

        let errs = validate_node(&node);
        assert!(!errs.is_empty(), "Expected errors for invalid taint effect");
    }

    #[test]
    fn test_validate_node_duplicate_addresses() {
        let mut node = create_test_node("test-node");
        node.status = Some(NodeStatus {
            addresses: vec![
                NodeAddress {
                    type_: "InternalIP".to_string(),
                    address: "192.168.1.10".to_string(),
                },
                NodeAddress {
                    type_: "InternalIP".to_string(),
                    address: "192.168.1.10".to_string(),
                },
            ],
            ..Default::default()
        });

        let errs = validate_node(&node);
        assert!(!errs.is_empty(), "Expected errors for duplicate addresses");
    }

    #[test]
    fn test_validate_node_update_cannot_change_pod_cidrs() {
        let mut old_node = create_test_node("test-node");
        old_node.spec = Some(NodeSpec {
            pod_cidrs: vec!["10.244.0.0/24".to_string()],
            ..Default::default()
        });
        old_node.metadata.as_mut().unwrap().resource_version = Some("123".to_string());

        let mut new_node = create_test_node("test-node");
        new_node.spec = Some(NodeSpec {
            pod_cidrs: vec!["10.244.1.0/24".to_string()], // Changed
            ..Default::default()
        });
        new_node.metadata.as_mut().unwrap().resource_version = Some("123".to_string());

        let errs = validate_node_update(&new_node, &old_node);
        let cidr_errs: Vec<_> = errs
            .errors
            .iter()
            .filter(|e| e.field.contains("podCIDRs"))
            .collect();
        assert!(
            !cidr_errs.is_empty(),
            "Expected errors for changing podCIDRs"
        );
    }

    #[test]
    fn test_validate_node_update_can_add_pod_cidrs() {
        let mut old_node = create_test_node("test-node");
        old_node.spec = Some(NodeSpec {
            pod_cidrs: vec![],
            ..Default::default()
        });
        old_node.metadata.as_mut().unwrap().resource_version = Some("123".to_string());

        let mut new_node = create_test_node("test-node");
        new_node.spec = Some(NodeSpec {
            pod_cidrs: vec!["10.244.0.0/24".to_string()], // Added
            ..Default::default()
        });
        new_node.metadata.as_mut().unwrap().resource_version = Some("123".to_string());

        let errs = validate_node_update(&new_node, &old_node);
        let cidr_errs: Vec<_> = errs
            .errors
            .iter()
            .filter(|e| e.field.contains("podCIDRs"))
            .collect();
        assert!(
            cidr_errs.is_empty(),
            "Expected no errors for adding podCIDRs from empty"
        );
    }
}
