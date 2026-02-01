//! Node validation for Kubernetes core/v1 API
//!
//! Ported from k8s.io/kubernetes/pkg/apis/core/validation/validation.go

use crate::common::validation::{
    BadValue, ErrorList, Path, forbidden, invalid, required, validate_label_name,
};
use crate::core::v1::{AvoidPods, Node, NodeConfigSource, NodeConfigStatus, NodeSwapStatus, Taint};
use std::collections::{BTreeMap, HashSet};

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

    if let Some(ref metadata) = node.metadata {
        all_errs.extend(validate_node_specific_annotations(
            &metadata.annotations,
            &path.child("metadata").child("annotations"),
        ));
    }

    // Validate spec
    if let Some(ref spec) = node.spec {
        all_errs.extend(validate_pod_cidrs(
            &spec.pod_cidrs,
            &path.child("spec").child("podCIDRs"),
        ));

        if !spec.taints.is_empty() {
            all_errs.extend(validate_node_taints(
                &spec.taints,
                &path.child("spec").child("taints"),
            ));
        }
    }

    // Validate status if present
    if let Some(ref status) = node.status {
        all_errs.extend(validate_node_resources(node));
        if let Some(ref node_info) = status.node_info {
            all_errs.extend(validate_node_swap_status(
                node_info.swap.as_ref(),
                &path.child("status").child("nodeInfo").child("swap"),
            ));
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
            if !old_provider_id.is_empty() && new_spec.provider_id != old_spec.provider_id {
                all_errs.push(forbidden(
                    &path.child("spec").child("providerID"),
                    "node updates may not change providerID except from \"\" to valid",
                ));
            }
        }

        if new_spec.external_id != old_spec.external_id {
            all_errs.push(forbidden(
                &path.child("spec").child("externalID"),
                "may not be updated",
            ));
        }

        if let Some(ref config_source) = new_spec.config_source {
            all_errs.extend(validate_node_config_source_spec(
                config_source,
                &path.child("spec").child("configSource"),
            ));
        }
    }

    if let Some(ref status) = new.status {
        if let Some(ref config) = status.config {
            all_errs.extend(validate_node_config_status(
                config,
                &path.child("status").child("config"),
            ));
        }

        // Validate no duplicate addresses in node status.
        let mut seen_addresses = HashSet::new();
        for (i, address) in status.addresses.iter().enumerate() {
            let key = (&address.type_, &address.address);
            if !seen_addresses.insert(key) {
                all_errs.push(invalid(
                    &path.child("status").child("addresses").index(i),
                    BadValue::String(format!("{}={}", address.type_, address.address)),
                    "duplicate address",
                ));
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

/// Validates pod CIDRs and dual-stack constraints.
fn validate_pod_cidrs(values: &[String], path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if values.is_empty() {
        return all_errs;
    }

    let mut families = HashSet::new();
    for (i, value) in values.iter().enumerate() {
        match parse_cidr(value) {
            Ok(family) => {
                families.insert(family);
            }
            Err(msg) => {
                all_errs.push(invalid(
                    &path.index(i),
                    BadValue::String(value.clone()),
                    &msg,
                ));
            }
        }
    }

    if values.len() > 1 {
        if values.len() > 2 || families.len() != values.len() {
            all_errs.push(invalid(
                path,
                BadValue::String(format!("{} CIDRs", values.len())),
                "may specify no more than one CIDR for each IP family",
            ));
        }
    }

    all_errs
}

/// Validates a taint
fn validate_node_taints(taints: &[Taint], path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let mut unique = HashSet::new();
    for (i, taint) in taints.iter().enumerate() {
        let idx_path = path.index(i);
        all_errs.extend(validate_label_name(&taint.key, &idx_path.child("key")));
        for msg in
            crate::common::validation::is_valid_label_value(taint.value.as_deref().unwrap_or(""))
        {
            all_errs.push(invalid(
                &idx_path.child("value"),
                BadValue::String(taint.value.clone().unwrap_or_default()),
                &msg,
            ));
        }
        all_errs.extend(validate_taint_effect(
            taint.effect.as_deref(),
            &idx_path.child("effect"),
        ));

        if let Some(effect) = taint.effect.as_deref() {
            let key = (taint.key.clone(), effect.to_string());
            if !unique.insert(key) {
                all_errs.push(invalid(
                    &idx_path,
                    BadValue::String(format!("{}:{}", taint.key, effect)),
                    "taints must be unique by key and effect pair",
                ));
            }
        }
    }

    all_errs
}

fn validate_taint_effect(effect: Option<&str>, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let valid_effects = ["NoSchedule", "PreferNoSchedule", "NoExecute"];
    match effect {
        Some(value) if !value.is_empty() => {
            if !valid_effects.contains(&value) {
                all_errs.push(crate::common::validation::not_supported(
                    path,
                    BadValue::String(value.to_string()),
                    &valid_effects,
                ));
            }
        }
        _ => {
            all_errs.push(required(path, "effect is required"));
        }
    }
    all_errs
}

fn validate_node_specific_annotations(
    annotations: &BTreeMap<String, String>,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if let Some(value) = annotations.get(TAINTS_ANNOTATION_KEY) {
        if !value.is_empty() {
            all_errs.extend(validate_taints_in_node_annotations(annotations, path));
        }
    }

    if let Some(value) = annotations.get(PREFER_AVOID_PODS_ANNOTATION_KEY) {
        if !value.is_empty() {
            all_errs.extend(validate_avoid_pods_in_node_annotations(annotations, path));
        }
    }

    all_errs
}

fn validate_taints_in_node_annotations(
    annotations: &BTreeMap<String, String>,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let raw = match annotations.get(TAINTS_ANNOTATION_KEY) {
        Some(value) => value,
        None => return all_errs,
    };

    let taints: Result<Vec<Taint>, _> = serde_json::from_str(raw);
    match taints {
        Ok(taints) => {
            if !taints.is_empty() {
                all_errs.extend(validate_node_taints(
                    &taints,
                    &path.child(TAINTS_ANNOTATION_KEY),
                ));
            }
        }
        Err(err) => {
            all_errs.push(invalid(
                path,
                BadValue::String(TAINTS_ANNOTATION_KEY.to_string()),
                &err.to_string(),
            ));
        }
    }

    all_errs
}

fn validate_avoid_pods_in_node_annotations(
    annotations: &BTreeMap<String, String>,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let raw = match annotations.get(PREFER_AVOID_PODS_ANNOTATION_KEY) {
        Some(value) => value,
        None => return all_errs,
    };

    let avoids: Result<AvoidPods, _> = serde_json::from_str(raw);
    match avoids {
        Ok(avoids) => {
            for (i, entry) in avoids.prefer_avoid_pods.iter().enumerate() {
                let idx_path = path.child(PREFER_AVOID_PODS_ANNOTATION_KEY).index(i);
                all_errs.extend(validate_prefer_avoid_pods_entry(entry, &idx_path));
            }
        }
        Err(err) => {
            all_errs.push(invalid(
                &path.child("avoidPods"),
                BadValue::String(PREFER_AVOID_PODS_ANNOTATION_KEY.to_string()),
                &err.to_string(),
            ));
        }
    }

    all_errs
}

fn validate_prefer_avoid_pods_entry(
    entry: &crate::core::v1::PreferAvoidPodsEntry,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    match entry.pod_signature.as_ref() {
        Some(signature) if !signature.pod_signature.is_empty() => {}
        _ => {
            all_errs.push(required(&path.child("podSignature"), ""));
        }
    }
    all_errs
}

fn validate_node_resources(node: &Node) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if let Some(ref status) = node.status {
        for (resource_name, quantity) in &status.capacity {
            let res_path = Path::nil()
                .child("status")
                .child("capacity")
                .key(resource_name);
            all_errs.extend(
                crate::core::v1::validation::resources::validate_resource_name_for_node(
                    resource_name,
                    &res_path,
                ),
            );
            all_errs.extend(
                crate::core::v1::validation::resources::validate_resource_quantity_value(
                    resource_name,
                    quantity,
                    &res_path,
                ),
            );
        }

        for (resource_name, quantity) in &status.allocatable {
            let res_path = Path::nil()
                .child("status")
                .child("allocatable")
                .key(resource_name);
            all_errs.extend(
                crate::core::v1::validation::resources::validate_resource_name_for_node(
                    resource_name,
                    &res_path,
                ),
            );
            all_errs.extend(
                crate::core::v1::validation::resources::validate_resource_quantity_value(
                    resource_name,
                    quantity,
                    &res_path,
                ),
            );
        }
    }

    all_errs
}

fn validate_node_swap_status(status: Option<&NodeSwapStatus>, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let Some(status) = status else {
        return all_errs;
    };

    if status.capacity < 0 {
        all_errs.push(invalid(
            &path.child("capacity"),
            BadValue::Int(status.capacity as i64),
            "must be greater than or equal to 0",
        ));
    }

    all_errs
}

fn validate_node_config_source_spec(source: &NodeConfigSource, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let mut count = 0;

    if let Some(ref config_map) = source.config_map {
        count += 1;
        all_errs.extend(validate_config_map_node_config_source_spec(
            config_map,
            &path.child("configMap"),
        ));
    }

    if count != 1 {
        all_errs.push(invalid(
            path,
            BadValue::String("configSource".to_string()),
            "exactly one reference subfield must be non-nil",
        ));
    }

    all_errs
}

fn validate_node_config_status(status: &NodeConfigStatus, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if let Some(ref assigned) = status.assigned {
        all_errs.extend(validate_node_config_source_status(
            assigned,
            &path.child("assigned"),
        ));
    }
    if let Some(ref active) = status.active {
        all_errs.extend(validate_node_config_source_status(
            active,
            &path.child("active"),
        ));
    }
    if let Some(ref last_known_good) = status.last_known_good {
        all_errs.extend(validate_node_config_source_status(
            last_known_good,
            &path.child("lastKnownGood"),
        ));
    }
    all_errs
}

fn validate_node_config_source_status(source: &NodeConfigSource, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let mut count = 0;

    if let Some(ref config_map) = source.config_map {
        count += 1;
        all_errs.extend(validate_config_map_node_config_source_status(
            config_map,
            &path.child("configMap"),
        ));
    }

    if count != 1 {
        all_errs.push(invalid(
            path,
            BadValue::String("configSource".to_string()),
            "exactly one reference subfield must be non-nil",
        ));
    }

    all_errs
}

fn validate_config_map_node_config_source_spec(
    source: &crate::core::v1::ConfigMapNodeConfigSource,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if !source.uid.as_deref().unwrap_or("").is_empty() {
        all_errs.push(forbidden(&path.child("uid"), "uid must not be set in spec"));
    }
    if !source.resource_version.as_deref().unwrap_or("").is_empty() {
        all_errs.push(forbidden(
            &path.child("resourceVersion"),
            "resourceVersion must not be set in spec",
        ));
    }

    all_errs.extend(validate_config_map_node_config_source(source, path));
    all_errs
}

fn validate_config_map_node_config_source_status(
    source: &crate::core::v1::ConfigMapNodeConfigSource,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if source.uid.as_deref().unwrap_or("").is_empty() {
        all_errs.push(required(&path.child("uid"), ""));
    }
    if source.resource_version.as_deref().unwrap_or("").is_empty() {
        all_errs.push(required(&path.child("resourceVersion"), ""));
    }
    all_errs.extend(validate_config_map_node_config_source(source, path));
    all_errs
}

fn validate_config_map_node_config_source(
    source: &crate::core::v1::ConfigMapNodeConfigSource,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let namespace = source.namespace.as_deref().unwrap_or("");
    if namespace.is_empty() {
        all_errs.push(required(&path.child("namespace"), ""));
    } else {
        for msg in crate::common::validation::name_is_dns_label(namespace, false) {
            all_errs.push(invalid(
                &path.child("namespace"),
                BadValue::String(namespace.to_string()),
                &msg,
            ));
        }
    }

    let name = source.name.as_deref().unwrap_or("");
    if name.is_empty() {
        all_errs.push(required(&path.child("name"), ""));
    } else {
        for msg in crate::common::validation::name_is_dns_subdomain(name, false) {
            all_errs.push(invalid(
                &path.child("name"),
                BadValue::String(name.to_string()),
                &msg,
            ));
        }
    }

    let key = source.kubelet_config_key.as_deref().unwrap_or("");
    if key.is_empty() {
        all_errs.push(required(&path.child("kubeletConfigKey"), ""));
    } else {
        for msg in crate::core::v1::validation::config::is_config_map_key(key) {
            all_errs.push(invalid(
                &path.child("kubeletConfigKey"),
                BadValue::String(key.to_string()),
                &msg,
            ));
        }
    }

    all_errs
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum IpFamily {
    V4,
    V6,
}

fn parse_cidr(value: &str) -> Result<IpFamily, String> {
    let (ip_part, prefix_part) = value
        .split_once('/')
        .ok_or_else(|| "must be a valid CIDR".to_string())?;
    let ip: std::net::IpAddr = ip_part
        .parse()
        .map_err(|_| "must be a valid CIDR".to_string())?;
    let prefix: u8 = prefix_part
        .parse()
        .map_err(|_| "must be a valid CIDR".to_string())?;
    match ip {
        std::net::IpAddr::V4(_) => {
            if prefix > 32 {
                Err("must be a valid CIDR".to_string())
            } else {
                Ok(IpFamily::V4)
            }
        }
        std::net::IpAddr::V6(_) => {
            if prefix > 128 {
                Err("must be a valid CIDR".to_string())
            } else {
                Ok(IpFamily::V6)
            }
        }
    }
}

const TAINTS_ANNOTATION_KEY: &str = "scheduler.alpha.kubernetes.io/taints";
const PREFER_AVOID_PODS_ANNOTATION_KEY: &str = "scheduler.alpha.kubernetes.io/preferAvoidPods";

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{ObjectMeta, TypeMeta};
    use crate::core::v1::{
        ConfigMapNodeConfigSource, NodeAddress, NodeConfigSource, NodeConfigStatus, NodeSpec,
        NodeStatus, NodeSwapStatus, NodeSystemInfo, Taint,
    };

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
        let mut old_node = create_test_node("test-node");
        old_node.metadata.as_mut().unwrap().resource_version = Some("1".to_string());

        let mut new_node = create_test_node("test-node");
        new_node.metadata.as_mut().unwrap().resource_version = Some("1".to_string());
        new_node.status = Some(NodeStatus {
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

        let errs = validate_node_update(&new_node, &old_node);
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

    #[test]
    fn test_validate_node_annotations_prefer_avoid_pods_requires_signature() {
        let mut node = create_test_node("test-node");
        let mut metadata = node.metadata.take().unwrap_or_default();
        metadata.annotations.insert(
            PREFER_AVOID_PODS_ANNOTATION_KEY.to_string(),
            r#"{"preferAvoidPods":[{}]}"#.to_string(),
        );
        node.metadata = Some(metadata);

        let errs = validate_node(&node);
        assert!(
            errs.errors
                .iter()
                .any(|e| e.field.contains("preferAvoidPods")),
            "Expected errors for missing podSignature in avoid pods annotation"
        );
    }

    #[test]
    fn test_validate_node_taints_unique_key_effect() {
        let mut node = create_test_node("test-node");
        node.spec = Some(NodeSpec {
            taints: vec![
                Taint {
                    key: "key1".to_string(),
                    value: Some("v1".to_string()),
                    effect: Some("NoSchedule".to_string()),
                    time_added: None,
                },
                Taint {
                    key: "key1".to_string(),
                    value: Some("v2".to_string()),
                    effect: Some("NoSchedule".to_string()),
                    time_added: None,
                },
            ],
            ..Default::default()
        });

        let errs = validate_node(&node);
        assert!(
            errs.errors
                .iter()
                .any(|e| e.detail.contains("taints must be unique")),
            "Expected errors for duplicate taints"
        );
    }

    #[test]
    fn test_validate_node_swap_status_negative_capacity() {
        let mut node = create_test_node("test-node");
        node.status = Some(NodeStatus {
            node_info: Some(NodeSystemInfo {
                swap: Some(NodeSwapStatus { capacity: -1 }),
                ..Default::default()
            }),
            ..Default::default()
        });

        let errs = validate_node(&node);
        assert!(
            errs.errors.iter().any(|e| e.field.contains("swap")),
            "Expected errors for negative swap capacity"
        );
    }

    #[test]
    fn test_validate_node_update_config_source_spec() {
        let mut old_node = create_test_node("test-node");
        old_node.metadata.as_mut().unwrap().resource_version = Some("1".to_string());
        old_node.spec = Some(NodeSpec::default());

        let mut new_node = create_test_node("test-node");
        new_node.metadata.as_mut().unwrap().resource_version = Some("1".to_string());
        new_node.spec = Some(NodeSpec {
            config_source: Some(NodeConfigSource {
                config_map: Some(ConfigMapNodeConfigSource {
                    namespace: Some("default".to_string()),
                    name: Some("config".to_string()),
                    uid: Some("uid".to_string()),
                    resource_version: None,
                    kubelet_config_key: None,
                }),
            }),
            ..Default::default()
        });

        let errs = validate_node_update(&new_node, &old_node);
        assert!(
            errs.errors.iter().any(|e| e.field.contains("configSource")),
            "Expected errors for invalid configSource in spec"
        );
    }

    #[test]
    fn test_validate_node_update_config_status_requires_uid() {
        let mut old_node = create_test_node("test-node");
        old_node.metadata.as_mut().unwrap().resource_version = Some("1".to_string());

        let mut new_node = create_test_node("test-node");
        new_node.metadata.as_mut().unwrap().resource_version = Some("1".to_string());
        new_node.status = Some(NodeStatus {
            config: Some(NodeConfigStatus {
                assigned: Some(NodeConfigSource {
                    config_map: Some(ConfigMapNodeConfigSource {
                        namespace: Some("default".to_string()),
                        name: Some("config".to_string()),
                        uid: None,
                        resource_version: Some("1".to_string()),
                        kubelet_config_key: None,
                    }),
                }),
                ..Default::default()
            }),
            ..Default::default()
        });

        let errs = validate_node_update(&new_node, &old_node);
        assert!(
            errs.errors.iter().any(|e| e.field.contains("uid")),
            "Expected errors for missing uid in config status"
        );
    }
}
