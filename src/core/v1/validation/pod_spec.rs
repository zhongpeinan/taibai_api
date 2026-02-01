//! PodSpec validation for Kubernetes core/v1 API
//!
//! This module implements the main PodSpec validation orchestration, tying together:
//! - Container validation (regular, init, ephemeral)
//! - Volume validation
//! - DNS policy and configuration
//! - Security context
//! - Service account, node name, and other pod-level settings

use crate::common::validation::{
    BadValue, ErrorList, Path, duplicate, invalid, is_valid_label_value, not_supported, required,
    validate_label_name,
};
use crate::core::internal::selector::LabelSelector as InternalLabelSelector;
use crate::core::v1::affinity::{
    Affinity, NodeAffinity, NodeSelector, NodeSelectorRequirement, NodeSelectorTerm, PodAffinity,
    PodAffinityTerm, PodAntiAffinity, PreferredSchedulingTerm, WeightedPodAffinityTerm,
    node_selector_operator,
};
use crate::core::v1::pod::{HostAlias, PodOS, PodReadinessGate, PodSchedulingGate, PodSpec};
use crate::core::v1::security::{PodSecurityContext, Sysctl, supplemental_groups_policy};
use crate::core::v1::toleration::{Toleration, toleration_effect, toleration_operator};
use crate::core::v1::topology::{
    TopologySpreadConstraint, node_affinity_policy, when_unsatisfiable,
};
use crate::core::v1::validation::container::{validate_containers, validate_init_containers};
use crate::core::v1::validation::dns::{validate_dns_policy, validate_pod_dns_config};
use crate::core::v1::validation::helpers::{
    validate_dns1123_label, validate_nonnegative_field, validate_positive_field,
};
use crate::core::v1::validation::resources::validate_pod_resource_requirements;
use crate::core::v1::validation::volume::validate_volumes;
use std::collections::HashSet;
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
    if let Some(ref affinity) = spec.affinity {
        all_errs.extend(validate_affinity(affinity, &path.child("affinity")));
    }

    // TODO: Validate tolerations (Phase 6)
    if !spec.tolerations.is_empty() {
        all_errs.extend(validate_tolerations(
            &spec.tolerations,
            &path.child("tolerations"),
        ));
    }

    // TODO: Validate security context (Phase 6)
    if let Some(ref sec_ctx) = spec.security_context {
        all_errs.extend(validate_pod_security_context(
            sec_ctx,
            &path.child("securityContext"),
        ));
    }

    // TODO: Validate topology spread constraints (Phase 6)
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
    if let Some(ref hostname) = spec.hostname {
        if !hostname.is_empty() {
            all_errs.extend(validate_dns1123_label(hostname, &path.child("hostname")));
        }
    }

    if let Some(ref subdomain) = spec.subdomain {
        if !subdomain.is_empty() {
            all_errs.extend(validate_dns1123_label(subdomain, &path.child("subdomain")));
        }
    }

    // Validate host aliases
    if !spec.host_aliases.is_empty() {
        all_errs.extend(validate_host_aliases(
            &spec.host_aliases,
            &path.child("hostAliases"),
        ));
    }

    // Validate priority class name
    if let Some(ref name) = spec.priority_class_name {
        if !name.is_empty() {
            let dns_errs = crate::common::validation::is_dns1123_subdomain(name);
            for msg in dns_errs {
                all_errs.push(invalid(
                    &path.child("priorityClassName"),
                    BadValue::String(name.clone()),
                    &msg,
                ));
            }
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
    if let Some(ref name) = spec.scheduler_name {
        if !name.is_empty() {
            let dns_errs = crate::common::validation::is_dns1123_subdomain(name);
            for msg in dns_errs {
                all_errs.push(invalid(
                    &path.child("schedulerName"),
                    BadValue::String(name.clone()),
                    &msg,
                ));
            }
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
// Additional PodSpec Validators
// ============================================================================

fn validate_host_network_deps(spec: &PodSpec, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if !spec.host_network {
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
    secrets: &[crate::core::v1::reference::LocalObjectReference],
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

        if !toleration.key.is_empty() {
            all_errs.extend(validate_label_name(&toleration.key, &idx_path.child("key")));
        } else if toleration.operator != toleration_operator::EXISTS {
            all_errs.push(invalid(
                &idx_path.child("operator"),
                BadValue::String(toleration.operator.clone()),
                "operator must be Exists when `key` is empty",
            ));
        }

        if toleration.toleration_seconds.is_some()
            && toleration.effect != toleration_effect::NO_EXECUTE
        {
            all_errs.push(invalid(
                &idx_path.child("effect"),
                BadValue::String(toleration.effect.clone()),
                "effect must be 'NoExecute' when `tolerationSeconds` is set",
            ));
        }

        match toleration.operator.as_str() {
            "" | toleration_operator::EQUAL => {
                for msg in is_valid_label_value(&toleration.value) {
                    all_errs.push(invalid(
                        &idx_path.child("value"),
                        BadValue::String(toleration.value.clone()),
                        &msg,
                    ));
                }
            }
            toleration_operator::EXISTS => {
                if !toleration.value.is_empty() {
                    all_errs.push(invalid(
                        &idx_path.child("value"),
                        BadValue::String(toleration.value.clone()),
                        "value must be empty when `operator` is 'Exists'",
                    ));
                }
            }
            _ => {
                let valid = vec![toleration_operator::EQUAL, toleration_operator::EXISTS];
                all_errs.push(not_supported(
                    &idx_path.child("operator"),
                    BadValue::String(toleration.operator.clone()),
                    &valid,
                ));
            }
        }

        if !toleration.effect.is_empty()
            && !matches!(
                toleration.effect.as_str(),
                toleration_effect::NO_SCHEDULE
                    | toleration_effect::PREFER_NO_SCHEDULE
                    | toleration_effect::NO_EXECUTE
            )
        {
            let valid = vec![
                toleration_effect::NO_SCHEDULE,
                toleration_effect::PREFER_NO_SCHEDULE,
                toleration_effect::NO_EXECUTE,
            ];
            all_errs.push(not_supported(
                &idx_path.child("effect"),
                BadValue::String(toleration.effect.clone()),
                &valid,
            ));
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

fn validate_pod_security_context(sec_ctx: &PodSecurityContext, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if let Some(value) = sec_ctx.run_as_user {
        all_errs.extend(validate_nonnegative_field(value, &path.child("runAsUser")));
    }

    if let Some(value) = sec_ctx.run_as_group {
        all_errs.extend(validate_nonnegative_field(value, &path.child("runAsGroup")));
    }

    if let Some(value) = sec_ctx.fs_group {
        all_errs.extend(validate_nonnegative_field(value, &path.child("fsGroup")));
    }

    for (i, group) in sec_ctx.supplemental_groups.iter().enumerate() {
        all_errs.extend(validate_nonnegative_field(
            *group,
            &path.child("supplementalGroups").index(i),
        ));
    }

    if let Some(ref policy) = sec_ctx.supplemental_groups_policy {
        if !matches!(
            policy.as_str(),
            supplemental_groups_policy::MERGE | supplemental_groups_policy::STRICT
        ) {
            let valid = vec![
                supplemental_groups_policy::MERGE,
                supplemental_groups_policy::STRICT,
            ];
            all_errs.push(not_supported(
                &path.child("supplementalGroupsPolicy"),
                BadValue::String(policy.clone()),
                &valid,
            ));
        }
    }

    all_errs.extend(validate_sysctls(&sec_ctx.sysctls, &path.child("sysctls")));

    all_errs
}

fn validate_sysctls(sysctls: &[Sysctl], path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (i, sysctl) in sysctls.iter().enumerate() {
        let idx_path = path.index(i);
        if sysctl.name.is_empty() {
            all_errs.push(required(&idx_path.child("name"), "name is required"));
        }
        if sysctl.value.is_empty() {
            all_errs.push(required(&idx_path.child("value"), "value is required"));
        }
    }

    all_errs
}

fn validate_pod_os(os: &PodOS, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let name = os.name.to_lowercase();
    if name != crate::core::v1::pod::os_name::LINUX
        && name != crate::core::v1::pod::os_name::WINDOWS
    {
        let valid = vec![
            crate::core::v1::pod::os_name::LINUX,
            crate::core::v1::pod::os_name::WINDOWS,
        ];
        all_errs.push(not_supported(
            &path.child("name"),
            BadValue::String(os.name.clone()),
            &valid,
        ));
    }
    all_errs
}

fn validate_affinity(affinity: &Affinity, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if let Some(ref node_affinity) = affinity.node_affinity {
        all_errs.extend(validate_node_affinity(
            node_affinity,
            &path.child("nodeAffinity"),
        ));
    }

    if let Some(ref pod_affinity) = affinity.pod_affinity {
        all_errs.extend(validate_pod_affinity(
            pod_affinity,
            &path.child("podAffinity"),
        ));
    }

    if let Some(ref pod_anti_affinity) = affinity.pod_anti_affinity {
        all_errs.extend(validate_pod_anti_affinity(
            pod_anti_affinity,
            &path.child("podAntiAffinity"),
        ));
    }

    all_errs
}

fn validate_node_affinity(node_affinity: &NodeAffinity, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if let Some(ref required) = node_affinity.required_during_scheduling_ignored_during_execution {
        all_errs.extend(validate_node_selector(
            required,
            &path.child("requiredDuringSchedulingIgnoredDuringExecution"),
        ));
    }

    if !node_affinity
        .preferred_during_scheduling_ignored_during_execution
        .is_empty()
    {
        for (i, term) in node_affinity
            .preferred_during_scheduling_ignored_during_execution
            .iter()
            .enumerate()
        {
            let term_path = path
                .child("preferredDuringSchedulingIgnoredDuringExecution")
                .index(i);
            all_errs.extend(validate_preferred_scheduling_term(term, &term_path));
        }
    }

    all_errs
}

fn validate_preferred_scheduling_term(term: &PreferredSchedulingTerm, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if term.weight < 1 || term.weight > 100 {
        all_errs.push(invalid(
            &path.child("weight"),
            BadValue::Int(term.weight as i64),
            "must be in the range 1-100",
        ));
    }

    if let Some(ref preference) = term.preference {
        all_errs.extend(validate_node_selector_term(
            preference,
            &path.child("preference"),
        ));
    } else {
        all_errs.push(required(
            &path.child("preference"),
            "preference is required",
        ));
    }

    all_errs
}

fn validate_node_selector(selector: &NodeSelector, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let terms_path = path.child("nodeSelectorTerms");

    if selector.node_selector_terms.is_empty() {
        all_errs.push(required(
            &terms_path,
            "must have at least one node selector term",
        ));
        return all_errs;
    }

    for (i, term) in selector.node_selector_terms.iter().enumerate() {
        all_errs.extend(validate_node_selector_term(term, &terms_path.index(i)));
    }

    all_errs
}

fn validate_node_selector_term(term: &NodeSelectorTerm, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (i, req) in term.match_expressions.iter().enumerate() {
        all_errs.extend(validate_node_selector_requirement(
            req,
            &path.child("matchExpressions").index(i),
            true,
        ));
    }

    for (i, req) in term.match_fields.iter().enumerate() {
        all_errs.extend(validate_node_selector_requirement(
            req,
            &path.child("matchFields").index(i),
            false,
        ));
    }

    all_errs
}

fn validate_node_selector_requirement(
    req: &NodeSelectorRequirement,
    path: &Path,
    is_label: bool,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if req.key.is_empty() {
        all_errs.push(required(&path.child("key"), "key is required"));
    } else if is_label {
        all_errs.extend(validate_label_name(&req.key, &path.child("key")));
    } else {
        all_errs.extend(crate::common::validation::validate_qualified_name(
            &req.key,
            &path.child("key"),
        ));
    }

    match req.operator.as_str() {
        node_selector_operator::IN | node_selector_operator::NOT_IN => {
            if req.values.is_empty() {
                all_errs.push(required(
                    &path.child("values"),
                    "values are required for In/NotIn",
                ));
            }
        }
        node_selector_operator::EXISTS | node_selector_operator::DOES_NOT_EXIST => {
            if !req.values.is_empty() {
                all_errs.push(invalid(
                    &path.child("values"),
                    BadValue::String(format!("{:?}", req.values)),
                    "values must be empty for Exists/DoesNotExist",
                ));
            }
        }
        node_selector_operator::GT | node_selector_operator::LT => {
            if req.values.len() != 1 {
                all_errs.push(invalid(
                    &path.child("values"),
                    BadValue::String(format!("{:?}", req.values)),
                    "must have exactly one value for Gt/Lt",
                ));
            } else if req.values[0].parse::<i64>().is_err() {
                all_errs.push(invalid(
                    &path.child("values").index(0),
                    BadValue::String(req.values[0].clone()),
                    "must be an integer for Gt/Lt",
                ));
            }
        }
        _ => {
            let valid = vec![
                node_selector_operator::IN,
                node_selector_operator::NOT_IN,
                node_selector_operator::EXISTS,
                node_selector_operator::DOES_NOT_EXIST,
                node_selector_operator::GT,
                node_selector_operator::LT,
            ];
            all_errs.push(not_supported(
                &path.child("operator"),
                BadValue::String(req.operator.clone()),
                &valid,
            ));
        }
    }

    all_errs
}

fn validate_pod_affinity(affinity: &PodAffinity, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (i, term) in affinity
        .required_during_scheduling_ignored_during_execution
        .iter()
        .enumerate()
    {
        all_errs.extend(validate_pod_affinity_term(
            term,
            &path
                .child("requiredDuringSchedulingIgnoredDuringExecution")
                .index(i),
        ));
    }

    for (i, term) in affinity
        .preferred_during_scheduling_ignored_during_execution
        .iter()
        .enumerate()
    {
        all_errs.extend(validate_weighted_pod_affinity_term(
            term,
            &path
                .child("preferredDuringSchedulingIgnoredDuringExecution")
                .index(i),
        ));
    }

    all_errs
}

fn validate_pod_anti_affinity(affinity: &PodAntiAffinity, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (i, term) in affinity
        .required_during_scheduling_ignored_during_execution
        .iter()
        .enumerate()
    {
        all_errs.extend(validate_pod_affinity_term(
            term,
            &path
                .child("requiredDuringSchedulingIgnoredDuringExecution")
                .index(i),
        ));
    }

    for (i, term) in affinity
        .preferred_during_scheduling_ignored_during_execution
        .iter()
        .enumerate()
    {
        all_errs.extend(validate_weighted_pod_affinity_term(
            term,
            &path
                .child("preferredDuringSchedulingIgnoredDuringExecution")
                .index(i),
        ));
    }

    all_errs
}

fn validate_pod_affinity_term(term: &PodAffinityTerm, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if term.topology_key.is_empty() {
        all_errs.push(required(
            &path.child("topologyKey"),
            "topologyKey is required",
        ));
    }

    if let Some(ref selector) = term.label_selector {
        all_errs.extend(validate_label_selector(
            selector,
            &path.child("labelSelector"),
        ));
    }

    if let Some(ref selector) = term.namespace_selector {
        all_errs.extend(validate_label_selector(
            selector,
            &path.child("namespaceSelector"),
        ));
    }

    for (i, namespace) in term.namespaces.iter().enumerate() {
        all_errs.extend(validate_dns1123_label(
            namespace,
            &path.child("namespaces").index(i),
        ));
    }

    all_errs
}

fn validate_weighted_pod_affinity_term(term: &WeightedPodAffinityTerm, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if term.weight < 1 || term.weight > 100 {
        all_errs.push(invalid(
            &path.child("weight"),
            BadValue::Int(term.weight as i64),
            "must be in the range 1-100",
        ));
    }

    if let Some(ref term) = term.pod_affinity_term {
        all_errs.extend(validate_pod_affinity_term(
            term,
            &path.child("podAffinityTerm"),
        ));
    } else {
        all_errs.push(required(
            &path.child("podAffinityTerm"),
            "podAffinityTerm is required",
        ));
    }

    all_errs
}

fn validate_label_selector(selector: &InternalLabelSelector, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if !selector.match_labels.is_empty() {
        all_errs.extend(crate::common::validation::validate_labels(
            &selector.match_labels,
            &path.child("matchLabels"),
        ));
    }

    for (i, expr) in selector.match_expressions.iter().enumerate() {
        let expr_path = path.child("matchExpressions").index(i);
        all_errs.extend(validate_label_name(&expr.key, &expr_path.child("key")));
        match expr.operator.as_str() {
            crate::core::internal::selector::label_selector_operator::IN
            | crate::core::internal::selector::label_selector_operator::NOT_IN => {
                if expr.values.is_empty() {
                    all_errs.push(required(
                        &expr_path.child("values"),
                        "values are required for In/NotIn",
                    ));
                }
            }
            crate::core::internal::selector::label_selector_operator::EXISTS
            | crate::core::internal::selector::label_selector_operator::DOES_NOT_EXIST => {
                if !expr.values.is_empty() {
                    all_errs.push(invalid(
                        &expr_path.child("values"),
                        BadValue::String(format!("{:?}", expr.values)),
                        "values must be empty for Exists/DoesNotExist",
                    ));
                }
            }
            _ => {
                let valid = vec![
                    crate::core::internal::selector::label_selector_operator::IN,
                    crate::core::internal::selector::label_selector_operator::NOT_IN,
                    crate::core::internal::selector::label_selector_operator::EXISTS,
                    crate::core::internal::selector::label_selector_operator::DOES_NOT_EXIST,
                ];
                all_errs.push(not_supported(
                    &expr_path.child("operator"),
                    BadValue::String(expr.operator.clone()),
                    &valid,
                ));
            }
        }

        let mut seen = HashSet::new();
        for (j, value) in expr.values.iter().enumerate() {
            if !seen.insert(value) {
                all_errs.push(duplicate(
                    &expr_path.child("values").index(j),
                    BadValue::String(value.clone()),
                ));
            }
            for msg in is_valid_label_value(value) {
                all_errs.push(invalid(
                    &expr_path.child("values").index(j),
                    BadValue::String(value.clone()),
                    &msg,
                ));
            }
        }
    }

    all_errs
}

fn validate_topology_spread_constraints(
    constraints: &[TopologySpreadConstraint],
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
            when_unsatisfiable::DO_NOT_SCHEDULE | when_unsatisfiable::SCHEDULE_ANYWAY
        ) {
            let valid = vec![
                when_unsatisfiable::DO_NOT_SCHEDULE,
                when_unsatisfiable::SCHEDULE_ANYWAY,
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
            if constraint.when_unsatisfiable != when_unsatisfiable::DO_NOT_SCHEDULE {
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
                node_affinity_policy::IGNORE | node_affinity_policy::HONOR
            )
        {
            let valid = vec![node_affinity_policy::IGNORE, node_affinity_policy::HONOR];
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

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::core::v1::pod::{Container, ContainerPort};

    fn base_spec() -> PodSpec {
        PodSpec {
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
        }
    }

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
        let mut spec = base_spec();
        spec.termination_grace_period_seconds = None;

        let errs = validate_pod_spec(&spec, &Path::nil());
        assert!(!errs.is_empty());
        assert!(errs.errors.iter().any(|e| {
            e.detail
                .contains("terminationGracePeriodSeconds is required")
        }));
    }

    #[test]
    fn test_validate_pod_spec_missing_restart_policy() {
        let mut spec = base_spec();
        spec.restart_policy = None;

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
        let mut spec = base_spec();
        spec.dns_policy = None;

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
        let mut spec = base_spec();
        spec.containers = vec![];

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
        let mut spec = base_spec();
        spec.service_account_name = Some("Invalid Service Account!".to_string());

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
        let mut spec = base_spec();
        spec.node_name = Some("Invalid Node Name!".to_string());

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
        let spec = base_spec();

        let errs = validate_pod_spec(&spec, &Path::nil());
        assert!(errs.is_empty(), "Valid PodSpec should not produce errors");
    }

    #[test]
    fn test_validate_pod_spec_host_network_requires_matching_host_port() {
        let mut spec = base_spec();
        spec.host_network = true;
        spec.containers[0].ports = vec![ContainerPort {
            name: None,
            container_port: 80,
            protocol: None,
            host_port: Some(81),
            host_ip: None,
        }];

        let errs = validate_pod_spec(&spec, &Path::nil());
        assert!(errs.errors.iter().any(|e| e.field.contains("hostPort")));
    }

    #[test]
    fn test_validate_pod_spec_toleration_requires_exists_for_empty_key() {
        let mut spec = base_spec();
        spec.tolerations = vec![Toleration {
            key: "".to_string(),
            operator: toleration_operator::EQUAL.to_string(),
            value: "value".to_string(),
            effect: toleration_effect::NO_SCHEDULE.to_string(),
            toleration_seconds: None,
        }];

        let errs = validate_pod_spec(&spec, &Path::nil());
        assert!(errs.errors.iter().any(|e| e.field.contains("operator")));
    }

    #[test]
    fn test_validate_pod_spec_invalid_topology_spread_constraint() {
        let mut spec = base_spec();
        spec.topology_spread_constraints = vec![TopologySpreadConstraint {
            max_skew: 0,
            topology_key: "".to_string(),
            when_unsatisfiable: "Invalid".to_string(),
            ..Default::default()
        }];

        let errs = validate_pod_spec(&spec, &Path::nil());
        assert!(!errs.is_empty());
    }

    #[test]
    fn test_validate_pod_spec_invalid_os() {
        let mut spec = base_spec();
        spec.os = Some(PodOS {
            name: "plan9".to_string(),
        });

        let errs = validate_pod_spec(&spec, &Path::nil());
        assert!(errs.errors.iter().any(|e| e.field.ends_with("os.name")));
    }

    #[test]
    fn test_validate_pod_spec_negative_run_as_user() {
        let mut spec = base_spec();
        spec.security_context = Some(PodSecurityContext {
            run_as_user: Some(-1),
            ..Default::default()
        });

        let errs = validate_pod_spec(&spec, &Path::nil());
        assert!(errs.errors.iter().any(|e| e.field.ends_with("runAsUser")));
    }
}
