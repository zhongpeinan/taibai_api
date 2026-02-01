//! NetworkPolicy validation.

use crate::common::validation::*;
use crate::networking::v1::network_policy::{NetworkPolicy, NetworkPolicyList};
use std::collections::BTreeSet;

use super::helpers::*;

// NetworkPolicy Validation
// ============================================================================

/// Validates a NetworkPolicy object.
pub fn validate_network_policy(policy: &NetworkPolicy) -> ErrorList {
    validate_network_policy_with_path(policy, &Path::nil())
}

fn validate_network_policy_with_path(policy: &NetworkPolicy, base_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let default_meta = crate::common::ObjectMeta::default();
    let meta = policy.metadata.as_ref().unwrap_or(&default_meta);

    all_errs.extend(validate_object_meta(
        meta,
        true,
        name_is_dns_subdomain,
        &base_path.child("metadata"),
    ));

    // Validate spec if present
    if let Some(spec) = &policy.spec {
        let spec_path = base_path.child("spec");

        all_errs.extend(validate_label_selector(
            &spec.pod_selector,
            &spec_path.child("podSelector"),
        ));

        // Validate ingress rules
        for (i, rule) in spec.ingress.iter().enumerate() {
            let rule_path = spec_path.child("ingress").index(i);

            // Validate ports
            for (j, port) in rule.ports.iter().enumerate() {
                let port_path = rule_path.child("ports").index(j);
                all_errs.extend(validate_network_policy_port(port, &port_path));
            }

            // Validate peers
            for (k, peer) in rule.from.iter().enumerate() {
                let peer_path = rule_path.child("from").index(k);
                all_errs.extend(validate_network_policy_peer(peer, &peer_path));
            }
        }

        // Validate egress rules
        for (i, rule) in spec.egress.iter().enumerate() {
            let rule_path = spec_path.child("egress").index(i);

            // Validate ports
            for (j, port) in rule.ports.iter().enumerate() {
                let port_path = rule_path.child("ports").index(j);
                all_errs.extend(validate_network_policy_port(port, &port_path));
            }

            // Validate peers
            for (k, peer) in rule.to.iter().enumerate() {
                let peer_path = rule_path.child("to").index(k);
                all_errs.extend(validate_network_policy_peer(peer, &peer_path));
            }
        }

        if spec.policy_types.len() > SUPPORTED_POLICY_TYPES.len() {
            all_errs.push(invalid(
                &spec_path.child("policyTypes"),
                BadValue::String(format!("{:?}", spec.policy_types)),
                "may not specify more than two policyTypes",
            ));
            return all_errs;
        }

        let mut seen = BTreeSet::new();
        for (i, policy_type) in spec.policy_types.iter().enumerate() {
            let policy_path = spec_path.child("policyTypes").index(i);
            let value = match policy_type {
                crate::networking::v1::network_policy::PolicyType::Ingress => "Ingress",
                crate::networking::v1::network_policy::PolicyType::Egress => "Egress",
            };

            if !SUPPORTED_POLICY_TYPES.contains(&value) {
                all_errs.push(not_supported(
                    &policy_path,
                    BadValue::String(value.to_string()),
                    &SUPPORTED_POLICY_TYPES,
                ));
            }

            if !seen.insert(value) {
                all_errs.push(duplicate(&policy_path, BadValue::String(value.to_string())));
            }
        }
    }

    all_errs
}

/// Validates a NetworkPolicyList object.
pub fn validate_network_policy_list(list: &NetworkPolicyList) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (i, item) in list.items.iter().enumerate() {
        let item_path = Path::new("items").index(i);
        all_errs.extend(validate_network_policy_with_path(item, &item_path));
    }

    all_errs
}

/// Validates a NetworkPolicy update.
pub fn validate_network_policy_update(
    new_policy: &NetworkPolicy,
    old_policy: &NetworkPolicy,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let default_meta = crate::common::ObjectMeta::default();
    let new_meta = new_policy.metadata.as_ref().unwrap_or(&default_meta);
    let old_meta = old_policy.metadata.as_ref().unwrap_or(&default_meta);

    all_errs.extend(validate_object_meta_update(
        new_meta,
        old_meta,
        &Path::new("metadata"),
    ));

    all_errs.extend(validate_network_policy_with_path(new_policy, &Path::nil()));

    all_errs
}

// ============================================================================
