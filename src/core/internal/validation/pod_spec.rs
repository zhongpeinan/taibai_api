//! PodSpec validation for Kubernetes core internal API types.

use crate::common::validation::{
    BadValue, ErrorList, Path, duplicate, invalid, is_valid_label_value, not_supported, required,
    validate_label_name,
};
use crate::core::internal::validation::affinity::{validate_affinity, validate_label_selector};
use crate::core::internal::validation::container::{validate_containers, validate_init_containers};
use crate::core::internal::validation::dns::{validate_dns_policy, validate_pod_dns_config};
use crate::core::internal::validation::resources::validate_pod_resource_requirements;
use crate::core::internal::validation::security::validate_pod_security_context;
use crate::core::internal::validation::volume::validate_volumes;
use crate::core::internal::{
    HostAlias, InternalPodReadinessGate, PodOS, PodSchedulingGate, PodSpec, TaintEffect,
    Toleration, TolerationOperator,
};
use crate::core::v1::validation::helpers::{validate_dns1123_label, validate_positive_field};
use std::collections::HashSet;

// ============================================================================
// PodSpec Validation
// ============================================================================

/// Validates a PodSpec.
///
/// This is the main validation entry point for pod specifications, orchestrating
/// validation of all pod-level settings and containers.
pub fn validate_pod_spec(spec: &PodSpec, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate termination grace period (required)
    if spec.termination_grace_period_seconds.is_none() {
        all_errs.push(required(
            &path.child("terminationGracePeriodSeconds"),
            "terminationGracePeriodSeconds is required",
        ));
    }

    // Validate termination grace period is non-negative
    if let Some(value) = spec.termination_grace_period_seconds {
        if value < 0 {
            all_errs.push(invalid(
                &path.child("terminationGracePeriodSeconds"),
                BadValue::Int(value),
                "must be greater than or equal to 0",
            ));
        }
    }

    let grace_period = &spec.termination_grace_period_seconds;

    // Validate restart policy
    all_errs.extend(validate_restart_policy(
        &spec.restart_policy,
        &path.child("restartPolicy"),
    ));

    // Validate DNS policy
    all_errs.extend(validate_dns_policy(
        &spec.dns_policy,
        &path.child("dnsPolicy"),
    ));

    // Validate DNS config
    all_errs.extend(validate_pod_dns_config(
        spec.dns_config.as_ref(),
        &spec.dns_policy,
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

    // Validate regular containers (at least one required)
    all_errs.extend(validate_containers(
        &spec.containers,
        &volumes_by_source,
        &pod_claim_names,
        grace_period,
        &path.child("containers"),
    ));

    // Validate init containers
    if !spec.init_containers.is_empty() {
        all_errs.extend(validate_init_containers(
            &spec.init_containers,
            &spec.containers,
            &volumes_by_source,
            &pod_claim_names,
            grace_period,
            &path.child("initContainers"),
        ));
    }

    // TODO: Validate ephemeral containers (Phase 6)

    // Validate service account name (DNS subdomain if specified)
    if !spec.service_account_name.is_empty() {
        let dns_errs = crate::common::validation::is_dns1123_subdomain(&spec.service_account_name);
        for err_msg in dns_errs {
            all_errs.push(invalid(
                &path.child("serviceAccountName"),
                BadValue::String(spec.service_account_name.clone()),
                &err_msg,
            ));
        }
    }

    // Validate node name (DNS subdomain if specified)
    if !spec.node_name.is_empty() {
        let dns_errs = crate::common::validation::is_dns1123_subdomain(&spec.node_name);
        for err_msg in dns_errs {
            all_errs.push(invalid(
                &path.child("nodeName"),
                BadValue::String(spec.node_name.clone()),
                &err_msg,
            ));
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

    // Validate affinity
    if let Some(ref affinity) = spec.affinity {
        all_errs.extend(validate_affinity(affinity, &path.child("affinity")));
    }

    // Validate tolerations
    if !spec.tolerations.is_empty() {
        all_errs.extend(validate_tolerations(
            &spec.tolerations,
            &path.child("tolerations"),
        ));
    }

    // Validate security context
    if let Some(ref sec_ctx) = spec.security_context {
        all_errs.extend(validate_pod_security_context(
            sec_ctx,
            &path.child("securityContext"),
        ));
    }

    // Validate topology spread constraints
    if !spec.topology_spread_constraints.is_empty() {
        all_errs.extend(validate_topology_spread_constraints(
            &spec.topology_spread_constraints,
            &path.child("topologySpreadConstraints"),
        ));
    }

    // Validate hostNetwork dependencies
    all_errs.extend(validate_host_network_deps(spec, &path.child("containers")));

    // Validate image pull secrets
    if !spec.image_pull_secrets.is_empty() {
        all_errs.extend(validate_image_pull_secrets(
            &spec.image_pull_secrets,
            &path.child("imagePullSecrets"),
        ));
    }

    // Validate pod-level resources
    if let Some(ref resources) = spec.resources {
        all_errs.extend(validate_pod_resource_requirements(
            resources,
            &pod_claim_names,
            &path.child("resources"),
        ));
    }

    // Validate overhead quantities (non-negative)
    if !spec.overhead.is_empty() {
        for (name, qty) in &spec.overhead {
            if qty.sign().unwrap_or(std::cmp::Ordering::Equal).is_lt() {
                all_errs.push(invalid(
                    &path.child("overhead").key(name),
                    BadValue::String(qty.to_string()),
                    "must be non-negative",
                ));
            }
        }
    }

    // Validate active deadline seconds
    if let Some(value) = spec.active_deadline_seconds {
        let max = i32::MAX as i64;
        if value < 1 || value > max {
            all_errs.push(invalid(
                &path.child("activeDeadlineSeconds"),
                BadValue::Int(value),
                &format!("must be in the range 1-{}", max),
            ));
        }
    }

    // Validate hostname/subdomain
    if !spec.hostname.is_empty() {
        all_errs.extend(validate_dns1123_label(
            &spec.hostname,
            &path.child("hostname"),
        ));
    }

    if !spec.subdomain.is_empty() {
        all_errs.extend(validate_dns1123_label(
            &spec.subdomain,
            &path.child("subdomain"),
        ));
    }

    // Validate host aliases
    if !spec.host_aliases.is_empty() {
        all_errs.extend(validate_host_aliases(
            &spec.host_aliases,
            &path.child("hostAliases"),
        ));
    }

    // Validate priority class name
    if !spec.priority_class_name.is_empty() {
        let dns_errs = crate::common::validation::is_dns1123_subdomain(&spec.priority_class_name);
        for msg in dns_errs {
            all_errs.push(invalid(
                &path.child("priorityClassName"),
                BadValue::String(spec.priority_class_name.clone()),
                &msg,
            ));
        }
    }

    // Validate runtime class name
    if let Some(ref name) = spec.runtime_class_name {
        if !name.is_empty() {
            let dns_errs = crate::common::validation::is_dns1123_subdomain(name);
            for msg in dns_errs {
                all_errs.push(invalid(
                    &path.child("runtimeClassName"),
                    BadValue::String(name.clone()),
                    &msg,
                ));
            }
        }
    }

    // Validate scheduler name
    if !spec.scheduler_name.is_empty() {
        let dns_errs = crate::common::validation::is_dns1123_subdomain(&spec.scheduler_name);
        for msg in dns_errs {
            all_errs.push(invalid(
                &path.child("schedulerName"),
                BadValue::String(spec.scheduler_name.clone()),
                &msg,
            ));
        }
    }

    // Validate OS
    if let Some(ref os) = spec.os {
        all_errs.extend(validate_pod_os(os, &path.child("os")));
    }

    all_errs
}
// ============================================================================
// Helper Validators
// ============================================================================

fn validate_restart_policy(
    policy: &crate::core::internal::RestartPolicy,
    path: &Path,
) -> ErrorList {
    let _ = (policy, path);
    ErrorList::new()
}

fn validate_readiness_gates(gates: &[InternalPodReadinessGate], path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (i, gate) in gates.iter().enumerate() {
        let idx_path = path.index(i).child("conditionType");

        let qual_errs =
            crate::common::validation::validate_qualified_name(&gate.condition_type, &idx_path);
        all_errs.extend(qual_errs);
    }

    all_errs
}

fn validate_scheduling_gates(gates: &[PodSchedulingGate], path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let mut seen = HashSet::new();

    for (i, gate) in gates.iter().enumerate() {
        let idx_path = path.index(i);

        let qual_errs = crate::common::validation::validate_qualified_name(&gate.name, &idx_path);
        all_errs.extend(qual_errs);

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

fn validate_host_network_deps(spec: &PodSpec, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    let host_network = spec
        .security_context
        .as_ref()
        .map(|ctx| ctx.host_network)
        .unwrap_or(false);
    if !host_network {
        return all_errs;
    }

    for (i, container) in spec.containers.iter().enumerate() {
        let ports_path = path.index(i).child("ports");
        for (j, port) in container.ports.iter().enumerate() {
            let idx_path = ports_path.index(j);
            let host_port = port.host_port.unwrap_or(0);
            if host_port != port.container_port {
                all_errs.push(invalid(
                    &idx_path.child("hostPort"),
                    BadValue::Int(host_port as i64),
                    "must match `containerPort` when `hostNetwork` is true",
                ));
            }
        }
    }

    all_errs
}

fn validate_image_pull_secrets(
    secrets: &[crate::core::internal::LocalObjectReference],
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (i, secret) in secrets.iter().enumerate() {
        let idx_path = path.index(i);
        let name = secret.name.as_deref().unwrap_or("");
        if name.is_empty() {
            all_errs.push(required(&idx_path.child("name"), "name is required"));
        }
    }

    all_errs
}

fn validate_tolerations(tolerations: &[Toleration], path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (i, toleration) in tolerations.iter().enumerate() {
        let idx_path = path.index(i);

        let operator = toleration
            .operator
            .clone()
            .unwrap_or(TolerationOperator::Equal);

        if !toleration.key.is_empty() {
            all_errs.extend(validate_label_name(&toleration.key, &idx_path.child("key")));
        } else if operator != TolerationOperator::Exists {
            all_errs.push(invalid(
                &idx_path.child("operator"),
                BadValue::String(format!("{:?}", operator)),
                "operator must be Exists when `key` is empty",
            ));
        }

        if toleration.toleration_seconds.is_some()
            && toleration.effect != Some(TaintEffect::NoExecute)
        {
            all_errs.push(invalid(
                &idx_path.child("effect"),
                BadValue::String(format!("{:?}", toleration.effect)),
                "effect must be 'NoExecute' when `tolerationSeconds` is set",
            ));
        }

        match operator {
            TolerationOperator::Equal => {
                for msg in is_valid_label_value(&toleration.value) {
                    all_errs.push(invalid(
                        &idx_path.child("value"),
                        BadValue::String(toleration.value.clone()),
                        &msg,
                    ));
                }
            }
            TolerationOperator::Exists => {
                if !toleration.value.is_empty() {
                    all_errs.push(invalid(
                        &idx_path.child("value"),
                        BadValue::String(toleration.value.clone()),
                        "value must be empty when `operator` is 'Exists'",
                    ));
                }
            }
        }
    }

    all_errs
}

fn validate_host_aliases(aliases: &[HostAlias], path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (i, alias) in aliases.iter().enumerate() {
        let idx_path = path.index(i);
        all_errs.extend(validate_host_alias_ip(&alias.ip, &idx_path.child("ip")));
        for (j, hostname) in alias.hostnames.iter().enumerate() {
            all_errs.extend(validate_dns1123_label(
                hostname,
                &idx_path.child("hostnames").index(j),
            ));
        }
    }

    all_errs
}

fn validate_host_alias_ip(ip_address: &str, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if ip_address.parse::<std::net::IpAddr>().is_err() {
        all_errs.push(invalid(
            path,
            BadValue::String(ip_address.to_string()),
            "must be a valid IP address",
        ));
        return all_errs;
    }

    all_errs
}

fn validate_pod_os(_os: &PodOS, _path: &Path) -> ErrorList {
    ErrorList::new()
}

fn validate_topology_spread_constraints(
    constraints: &[crate::core::internal::InternalTopologySpreadConstraint],
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let mut seen_pairs = HashSet::new();

    for (i, constraint) in constraints.iter().enumerate() {
        let idx_path = path.index(i);
        all_errs.extend(validate_positive_field(
            constraint.max_skew as i64,
            &idx_path.child("maxSkew"),
        ));

        if constraint.topology_key.is_empty() {
            all_errs.push(required(
                &idx_path.child("topologyKey"),
                "topologyKey is required",
            ));
        }

        if !matches!(
            constraint.when_unsatisfiable.as_str(),
            crate::core::v1::topology::when_unsatisfiable::DO_NOT_SCHEDULE
                | crate::core::v1::topology::when_unsatisfiable::SCHEDULE_ANYWAY
        ) {
            let valid = vec![
                crate::core::v1::topology::when_unsatisfiable::DO_NOT_SCHEDULE,
                crate::core::v1::topology::when_unsatisfiable::SCHEDULE_ANYWAY,
            ];
            all_errs.push(not_supported(
                &idx_path.child("whenUnsatisfiable"),
                BadValue::String(constraint.when_unsatisfiable.clone()),
                &valid,
            ));
        }

        let pair_key = format!(
            "{}:{}",
            constraint.topology_key, constraint.when_unsatisfiable
        );
        if !constraint.topology_key.is_empty()
            && !constraint.when_unsatisfiable.is_empty()
            && !seen_pairs.insert(pair_key.clone())
        {
            all_errs.push(duplicate(
                &idx_path.child("{topologyKey, whenUnsatisfiable}"),
                BadValue::String(pair_key),
            ));
        }

        if let Some(min_domains) = constraint.min_domains {
            all_errs.extend(validate_positive_field(
                min_domains as i64,
                &idx_path.child("minDomains"),
            ));
            if constraint.when_unsatisfiable
                != crate::core::v1::topology::when_unsatisfiable::DO_NOT_SCHEDULE
            {
                all_errs.push(invalid(
                    &idx_path.child("minDomains"),
                    BadValue::Int(min_domains as i64),
                    "can only use minDomains when whenUnsatisfiable=DoNotSchedule",
                ));
            }
        }

        if !constraint.node_affinity_policy.is_empty()
            && !matches!(
                constraint.node_affinity_policy.as_str(),
                crate::core::v1::topology::node_affinity_policy::IGNORE
                    | crate::core::v1::topology::node_affinity_policy::HONOR
            )
        {
            let valid = vec![
                crate::core::v1::topology::node_affinity_policy::IGNORE,
                crate::core::v1::topology::node_affinity_policy::HONOR,
            ];
            all_errs.push(not_supported(
                &idx_path.child("nodeAffinityPolicy"),
                BadValue::String(constraint.node_affinity_policy.clone()),
                &valid,
            ));
        }

        if !constraint.match_label_keys.is_empty() {
            let mut seen = HashSet::new();
            for (j, key) in constraint.match_label_keys.iter().enumerate() {
                if !seen.insert(key) {
                    all_errs.push(duplicate(
                        &idx_path.child("matchLabelKeys").index(j),
                        BadValue::String(key.clone()),
                    ));
                }
                all_errs.extend(validate_label_name(
                    key,
                    &idx_path.child("matchLabelKeys").index(j),
                ));
            }
        }

        if let Some(ref selector) = constraint.label_selector {
            all_errs.extend(validate_label_selector(
                selector,
                &idx_path.child("labelSelector"),
            ));
        }
    }

    all_errs
}
