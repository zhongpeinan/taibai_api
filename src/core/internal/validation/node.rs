//! Node validation for Kubernetes core internal API
//!
//! Ported from k8s.io/kubernetes/pkg/apis/core/validation/validation.go

use super::helpers::is_config_map_key;
use crate::common::validation::{
    BadValue, ErrorList, Path, forbidden, invalid, required, validate_label_name,
};
use crate::core::internal::{
    AvoidPods, Node, NodeConfigSource, NodeConfigStatus, NodeSwapStatus, Taint,
};
use std::collections::{BTreeMap, HashSet};

/// Validates a Node
pub fn validate_node(node: &Node) -> ErrorList {
    validate_node_with_path(node, &Path::nil())
}

fn validate_node_with_path(node: &Node, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate metadata (Node is cluster-scoped, so namespace=false)
    all_errs.extend(crate::common::validation::validate_object_meta(
        &node.metadata,
        false, // Node is cluster-scoped (not namespaced)
        validate_node_name,
        &path.child("metadata"),
    ));

    all_errs.extend(validate_node_specific_annotations(
        &node.metadata.annotations,
        &path.child("metadata").child("annotations"),
    ));

    // Validate spec
    all_errs.extend(validate_pod_cidrs(
        &node.spec.pod_cidrs,
        &path.child("spec").child("podCIDRs"),
    ));

    if !node.spec.taints.is_empty() {
        all_errs.extend(validate_node_taints(
            &node.spec.taints,
            &path.child("spec").child("taints"),
        ));
    }

    // Validate status
    all_errs.extend(validate_node_resources(node));
    all_errs.extend(validate_node_swap_status(
        node.status.node_info.swap.as_ref(),
        &path.child("status").child("nodeInfo").child("swap"),
    ));

    all_errs
}

/// Validates Node update
pub fn validate_node_update(new: &Node, old: &Node) -> ErrorList {
    validate_node_update_with_path(new, old, &Path::nil())
}

fn validate_node_update_with_path(new: &Node, old: &Node, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate metadata update
    all_errs.extend(crate::common::validation::validate_object_meta_update(
        &new.metadata,
        &old.metadata,
        &path.child("metadata"),
    ));

    // Validate the new node
    all_errs.extend(validate_node_with_path(new, path));

    // Check immutability constraints
    let new_spec = &new.spec;
    let old_spec = &old.spec;

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

    if let Some(ref config) = new.status.config {
        all_errs.extend(validate_node_config_status(
            config,
            &path.child("status").child("config"),
        ));
    }

    // Validate no duplicate addresses in node status.
    let mut seen_addresses = HashSet::new();
    for (i, address) in new.status.addresses.iter().enumerate() {
        let key = format!(
            "{}={}",
            node_address_type_to_str(&address.r#type),
            address.address
        );
        if !seen_addresses.insert(key) {
            all_errs.push(invalid(
                &path.child("status").child("addresses").index(i),
                BadValue::String(format!(
                    "{}={}",
                    node_address_type_to_str(&address.r#type),
                    address.address
                )),
                "duplicate address",
            ));
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
        for msg in crate::common::validation::is_valid_label_value(&taint.value) {
            all_errs.push(invalid(
                &idx_path.child("value"),
                BadValue::String(taint.value.clone()),
                &msg,
            ));
        }
        all_errs.extend(validate_taint_effect(
            &taint.effect,
            &idx_path.child("effect"),
        ));

        let effect = taint_effect_to_str(&taint.effect);
        let key = (taint.key.clone(), effect.to_string());
        if !unique.insert(key) {
            all_errs.push(invalid(
                &idx_path,
                BadValue::String(format!("{}:{}", taint.key, effect)),
                "taints must be unique by key and effect pair",
            ));
        }
    }

    all_errs
}

fn validate_taint_effect(effect: &crate::core::internal::TaintEffect, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let valid_effects = ["NoSchedule", "PreferNoSchedule", "NoExecute"];
    let effect_value = taint_effect_to_str(effect);
    if !valid_effects.contains(&effect_value) {
        all_errs.push(crate::common::validation::not_supported(
            path,
            BadValue::String(effect_value.to_string()),
            &valid_effects,
        ));
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
    entry: &crate::core::internal::PreferAvoidPodsEntry,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if entry.pod_signature.pod_controller.is_none() {
        all_errs.push(required(&path.child("podSignature"), ""));
    }
    all_errs
}

fn validate_node_resources(node: &Node) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (resource_name, quantity) in &node.status.capacity {
        let res_path = Path::nil()
            .child("status")
            .child("capacity")
            .key(resource_name);
        all_errs.extend(
            crate::core::internal::validation::resources::validate_resource_name_for_node(
                resource_name,
                &res_path,
            ),
        );
        all_errs.extend(
            crate::core::internal::validation::resources::validate_resource_quantity_value(
                resource_name,
                quantity,
                &res_path,
            ),
        );
    }

    for (resource_name, quantity) in &node.status.allocatable {
        let res_path = Path::nil()
            .child("status")
            .child("allocatable")
            .key(resource_name);
        all_errs.extend(
            crate::core::internal::validation::resources::validate_resource_name_for_node(
                resource_name,
                &res_path,
            ),
        );
        all_errs.extend(
            crate::core::internal::validation::resources::validate_resource_quantity_value(
                resource_name,
                quantity,
                &res_path,
            ),
        );
    }

    all_errs
}

fn validate_node_swap_status(status: Option<&NodeSwapStatus>, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let Some(status) = status else {
        return all_errs;
    };

    if let Some(capacity) = status.capacity {
        if capacity < 0 {
            all_errs.push(invalid(
                &path.child("capacity"),
                BadValue::Int(capacity),
                "must be greater than or equal to 0",
            ));
        }
    }

    all_errs
}

fn node_address_type_to_str(value: &crate::core::internal::NodeAddressType) -> &'static str {
    match value {
        crate::core::internal::NodeAddressType::Hostname => "Hostname",
        crate::core::internal::NodeAddressType::InternalIp => "InternalIP",
        crate::core::internal::NodeAddressType::ExternalIp => "ExternalIP",
        crate::core::internal::NodeAddressType::InternalDns => "InternalDNS",
        crate::core::internal::NodeAddressType::ExternalDns => "ExternalDNS",
    }
}

fn taint_effect_to_str(value: &crate::core::internal::TaintEffect) -> &'static str {
    match value {
        crate::core::internal::TaintEffect::NoSchedule => "NoSchedule",
        crate::core::internal::TaintEffect::PreferNoSchedule => "PreferNoSchedule",
        crate::core::internal::TaintEffect::NoExecute => "NoExecute",
    }
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
    source: &crate::core::internal::ConfigMapNodeConfigSource,
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
    source: &crate::core::internal::ConfigMapNodeConfigSource,
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
    source: &crate::core::internal::ConfigMapNodeConfigSource,
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
        for msg in is_config_map_key(key) {
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
