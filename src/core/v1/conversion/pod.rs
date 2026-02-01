//! Phase 3 conversions: Pod/Container + Status + Port
//!
//! PodIP, HostIP, HostAlias, PodCondition, ContainerPort, Container State, ContainerStatus

use super::helpers::*;
use crate::common::{ApplyDefault, FromInternal, ToInternal};
use crate::core::internal;
use crate::core::v1::pod;
use crate::core::v1::{pod_resources, resource, security, template, volume};
use serde_json;
use std::collections::BTreeMap;

// ============================================================================
// Simple Pod-related types
// ============================================================================

impl ToInternal<internal::PodIP> for pod::PodIP {
    fn to_internal(self) -> internal::PodIP {
        internal::PodIP { ip: self.ip }
    }
}

impl FromInternal<internal::PodIP> for pod::PodIP {
    fn from_internal(value: internal::PodIP) -> Self {
        Self { ip: value.ip }
    }
}

impl ToInternal<internal::HostIP> for pod::HostIP {
    fn to_internal(self) -> internal::HostIP {
        internal::HostIP { ip: self.ip }
    }
}

impl FromInternal<internal::HostIP> for pod::HostIP {
    fn from_internal(value: internal::HostIP) -> Self {
        Self { ip: value.ip }
    }
}

impl ToInternal<internal::HostAlias> for pod::HostAlias {
    fn to_internal(self) -> internal::HostAlias {
        internal::HostAlias {
            ip: self.ip,
            hostnames: self.hostnames,
        }
    }
}

impl FromInternal<internal::HostAlias> for pod::HostAlias {
    fn from_internal(value: internal::HostAlias) -> Self {
        Self {
            ip: value.ip,
            hostnames: value.hostnames,
        }
    }
}

// ============================================================================
// PodCondition
// ============================================================================

impl ToInternal<internal::PodCondition> for pod::PodCondition {
    fn to_internal(self) -> internal::PodCondition {
        internal::PodCondition {
            r#type: self.type_,
            observed_generation: self.observed_generation.unwrap_or(0),
            status: self.status,
            last_probe_time: self.last_probe_time,
            last_transition_time: self.last_transition_time,
            reason: self.reason.unwrap_or_default(),
            message: self.message.unwrap_or_default(),
        }
    }
}

impl FromInternal<internal::PodCondition> for pod::PodCondition {
    fn from_internal(value: internal::PodCondition) -> Self {
        Self {
            type_: value.r#type,
            status: value.status,
            last_probe_time: value.last_probe_time,
            last_transition_time: value.last_transition_time,
            reason: if value.reason.is_empty() {
                None
            } else {
                Some(value.reason)
            },
            message: if value.message.is_empty() {
                None
            } else {
                Some(value.message)
            },
            observed_generation: if value.observed_generation == 0 {
                None
            } else {
                Some(value.observed_generation)
            },
        }
    }
}

// ============================================================================
// ContainerPort
// ============================================================================

impl ToInternal<internal::ContainerPort> for pod::ContainerPort {
    fn to_internal(self) -> internal::ContainerPort {
        internal::ContainerPort {
            name: self.name.unwrap_or_default(),
            host_port: self.host_port,
            container_port: self.container_port,
            protocol: option_string_to_protocol(self.protocol),
            host_ip: self.host_ip.unwrap_or_default(),
        }
    }
}

impl FromInternal<internal::ContainerPort> for pod::ContainerPort {
    fn from_internal(value: internal::ContainerPort) -> Self {
        Self {
            name: if value.name.is_empty() {
                None
            } else {
                Some(value.name)
            },
            host_port: value.host_port,
            container_port: value.container_port,
            protocol: protocol_to_option_string(value.protocol),
            host_ip: if value.host_ip.is_empty() {
                None
            } else {
                Some(value.host_ip)
            },
        }
    }
}

// ============================================================================
// ContainerState variants
// ============================================================================

impl ToInternal<internal::ContainerStateRunning> for pod::ContainerStateRunning {
    fn to_internal(self) -> internal::ContainerStateRunning {
        internal::ContainerStateRunning {
            started_at: self.started_at,
        }
    }
}

impl FromInternal<internal::ContainerStateRunning> for pod::ContainerStateRunning {
    fn from_internal(value: internal::ContainerStateRunning) -> Self {
        Self {
            started_at: value.started_at,
        }
    }
}

impl ToInternal<internal::ContainerStateTerminated> for pod::ContainerStateTerminated {
    fn to_internal(self) -> internal::ContainerStateTerminated {
        internal::ContainerStateTerminated {
            exit_code: Some(self.exit_code),
            signal: self.signal.unwrap_or(0),
            reason: self.reason.unwrap_or_default(),
            message: self.message.unwrap_or_default(),
            started_at: self.started_at,
            finished_at: self.finished_at,
        }
    }
}

impl FromInternal<internal::ContainerStateTerminated> for pod::ContainerStateTerminated {
    fn from_internal(value: internal::ContainerStateTerminated) -> Self {
        Self {
            exit_code: value.exit_code.unwrap_or(0),
            signal: if value.signal == 0 {
                None
            } else {
                Some(value.signal)
            },
            reason: if value.reason.is_empty() {
                None
            } else {
                Some(value.reason)
            },
            message: if value.message.is_empty() {
                None
            } else {
                Some(value.message)
            },
            started_at: value.started_at,
            finished_at: value.finished_at,
        }
    }
}

impl ToInternal<internal::ContainerStateWaiting> for pod::ContainerStateWaiting {
    fn to_internal(self) -> internal::ContainerStateWaiting {
        internal::ContainerStateWaiting {
            reason: self.reason.unwrap_or_default(),
            message: self.message.unwrap_or_default(),
        }
    }
}

impl FromInternal<internal::ContainerStateWaiting> for pod::ContainerStateWaiting {
    fn from_internal(value: internal::ContainerStateWaiting) -> Self {
        Self {
            reason: if value.reason.is_empty() {
                None
            } else {
                Some(value.reason)
            },
            message: if value.message.is_empty() {
                None
            } else {
                Some(value.message)
            },
        }
    }
}

impl ToInternal<internal::ContainerState> for pod::ContainerState {
    fn to_internal(self) -> internal::ContainerState {
        internal::ContainerState {
            waiting: self.waiting.map(|w| w.to_internal()),
            running: self.running.map(|r| r.to_internal()),
            terminated: self.terminated.map(|t| t.to_internal()),
        }
    }
}

impl FromInternal<internal::ContainerState> for pod::ContainerState {
    fn from_internal(value: internal::ContainerState) -> Self {
        Self {
            waiting: value.waiting.map(pod::ContainerStateWaiting::from_internal),
            running: value.running.map(pod::ContainerStateRunning::from_internal),
            terminated: value
                .terminated
                .map(pod::ContainerStateTerminated::from_internal),
        }
    }
}

// ============================================================================
// ContainerStatus
// ============================================================================

impl ToInternal<internal::ContainerStatus> for pod::ContainerStatus {
    fn to_internal(self) -> internal::ContainerStatus {
        internal::ContainerStatus {
            name: self.name,
            state: self
                .state
                .map(|s| s.to_internal())
                .unwrap_or(internal::ContainerState::default()),
            last_termination_state: self.last_state.map(|s| s.to_internal()).unwrap_or_default(),
            ready: self.ready,
            restart_count: self.restart_count,
            image: self.image.unwrap_or_default(),
            image_id: self.image_id.unwrap_or_default(),
            container_id: self.container_id.unwrap_or_default(),
            started: self.started,
            allocated_resources: self.allocated_resources.unwrap_or_default(),
            resources: self.resources.map(|r| r.to_internal()),
            volume_mounts: self
                .volume_mounts
                .into_iter()
                .map(|mount| mount.to_internal())
                .collect(),
            user: self.user.map(|user| user.to_internal()),
            allocated_resources_status: self
                .allocated_resources_status
                .into_iter()
                .map(|status| status.to_internal())
                .collect(),
            stop_signal: option_string_to_signal(self.stop_signal),
        }
    }
}

impl FromInternal<internal::ContainerStatus> for pod::ContainerStatus {
    fn from_internal(value: internal::ContainerStatus) -> Self {
        Self {
            name: value.name,
            state: Some(pod::ContainerState::from_internal(value.state)),
            last_state: if value.last_termination_state == internal::ContainerState::default() {
                None
            } else {
                Some(pod::ContainerState::from_internal(
                    value.last_termination_state,
                ))
            },
            ready: value.ready,
            restart_count: value.restart_count,
            image: if value.image.is_empty() {
                None
            } else {
                Some(value.image)
            },
            image_id: if value.image_id.is_empty() {
                None
            } else {
                Some(value.image_id)
            },
            container_id: if value.container_id.is_empty() {
                None
            } else {
                Some(value.container_id)
            },
            started: value.started,
            allocated_resources: if value.allocated_resources.is_empty() {
                None
            } else {
                Some(value.allocated_resources)
            },
            resources: value
                .resources
                .map(resource::ResourceRequirements::from_internal),
            volume_mounts: value
                .volume_mounts
                .into_iter()
                .map(volume::VolumeMountStatus::from_internal)
                .collect(),
            user: value.user.map(pod_resources::ContainerUser::from_internal),
            allocated_resources_status: value
                .allocated_resources_status
                .into_iter()
                .map(pod::ResourceStatus::from_internal)
                .collect(),
            stop_signal: signal_to_option_string(value.stop_signal),
        }
    }
}

// ============================================================================
// PodResourceClaim
// ============================================================================

impl ToInternal<internal::PodResourceClaim> for pod_resources::PodResourceClaim {
    fn to_internal(self) -> internal::PodResourceClaim {
        internal::PodResourceClaim {
            name: self.name,
            resource_claim_name: self.resource_claim_name,
            resource_claim_template_name: self.resource_claim_template_name,
        }
    }
}

impl FromInternal<internal::PodResourceClaim> for pod_resources::PodResourceClaim {
    fn from_internal(value: internal::PodResourceClaim) -> Self {
        Self {
            name: value.name,
            resource_claim_name: value.resource_claim_name,
            resource_claim_template_name: value.resource_claim_template_name,
        }
    }
}

impl ToInternal<internal::PodResourceClaimStatus> for pod_resources::PodResourceClaimStatus {
    fn to_internal(self) -> internal::PodResourceClaimStatus {
        internal::PodResourceClaimStatus {
            name: self.name,
            resource_claim_name: self.resource_claim_name,
        }
    }
}

impl FromInternal<internal::PodResourceClaimStatus> for pod_resources::PodResourceClaimStatus {
    fn from_internal(value: internal::PodResourceClaimStatus) -> Self {
        Self {
            name: value.name,
            resource_claim_name: value.resource_claim_name,
        }
    }
}

// ============================================================================
// ContainerUser
// ============================================================================

impl ToInternal<internal::ContainerUser> for pod_resources::ContainerUser {
    fn to_internal(self) -> internal::ContainerUser {
        internal::ContainerUser {
            linux: self.linux.map(|user| user.to_internal()),
        }
    }
}

impl FromInternal<internal::ContainerUser> for pod_resources::ContainerUser {
    fn from_internal(value: internal::ContainerUser) -> Self {
        Self {
            linux: value
                .linux
                .map(pod_resources::LinuxContainerUser::from_internal),
        }
    }
}

impl ToInternal<internal::LinuxContainerUser> for pod_resources::LinuxContainerUser {
    fn to_internal(self) -> internal::LinuxContainerUser {
        internal::LinuxContainerUser {
            uid: self.uid,
            gid: self.gid,
            supplemental_groups: self.supplemental_groups,
        }
    }
}

impl FromInternal<internal::LinuxContainerUser> for pod_resources::LinuxContainerUser {
    fn from_internal(value: internal::LinuxContainerUser) -> Self {
        Self {
            uid: value.uid,
            gid: value.gid,
            supplemental_groups: value.supplemental_groups,
        }
    }
}

// ============================================================================
// PodSecurityContext and related types
// ============================================================================

impl ToInternal<internal::SELinuxOptions> for security::SELinuxOptions {
    fn to_internal(self) -> internal::SELinuxOptions {
        internal::SELinuxOptions {
            user: self.user,
            role: self.role,
            level: self.level,
            r#type: self.type_,
        }
    }
}

impl FromInternal<internal::SELinuxOptions> for security::SELinuxOptions {
    fn from_internal(value: internal::SELinuxOptions) -> Self {
        Self {
            user: value.user,
            role: value.role,
            level: value.level,
            type_: value.r#type,
        }
    }
}

impl ToInternal<internal::WindowsSecurityContextOptions>
    for security::WindowsSecurityContextOptions
{
    fn to_internal(self) -> internal::WindowsSecurityContextOptions {
        internal::WindowsSecurityContextOptions {
            gmsa_credential_spec_name: self.gmsa_credential_spec_name,
            gmsa_credential_spec: self.gmsa_credential_spec,
            run_as_user_name: self.run_as_user_name,
            host_process: self.host_process,
        }
    }
}

impl FromInternal<internal::WindowsSecurityContextOptions>
    for security::WindowsSecurityContextOptions
{
    fn from_internal(value: internal::WindowsSecurityContextOptions) -> Self {
        Self {
            gmsa_credential_spec_name: value.gmsa_credential_spec_name,
            gmsa_credential_spec: value.gmsa_credential_spec,
            run_as_user_name: value.run_as_user_name,
            host_process: value.host_process,
        }
    }
}

impl ToInternal<internal::SeccompProfile> for security::SeccompProfile {
    fn to_internal(self) -> internal::SeccompProfile {
        internal::SeccompProfile {
            r#type: seccomp_profile_type_from_string(self.type_),
            localhost_profile: self.localhost_profile,
        }
    }
}

impl FromInternal<internal::SeccompProfile> for security::SeccompProfile {
    fn from_internal(value: internal::SeccompProfile) -> Self {
        Self {
            type_: seccomp_profile_type_to_string(value.r#type),
            localhost_profile: value.localhost_profile,
        }
    }
}

impl ToInternal<internal::AppArmorProfile> for security::AppArmorProfile {
    fn to_internal(self) -> internal::AppArmorProfile {
        internal::AppArmorProfile {
            r#type: app_armor_profile_type_from_string(self.type_),
            localhost_profile: self.localhost_profile,
        }
    }
}

impl FromInternal<internal::AppArmorProfile> for security::AppArmorProfile {
    fn from_internal(value: internal::AppArmorProfile) -> Self {
        Self {
            type_: app_armor_profile_type_to_string(value.r#type),
            localhost_profile: value.localhost_profile,
        }
    }
}

impl ToInternal<internal::Sysctl> for security::Sysctl {
    fn to_internal(self) -> internal::Sysctl {
        internal::Sysctl {
            name: self.name,
            value: self.value,
        }
    }
}

impl FromInternal<internal::Sysctl> for security::Sysctl {
    fn from_internal(value: internal::Sysctl) -> Self {
        Self {
            name: value.name,
            value: value.value,
        }
    }
}

impl ToInternal<internal::PodSecurityContext> for security::PodSecurityContext {
    fn to_internal(self) -> internal::PodSecurityContext {
        internal::PodSecurityContext {
            host_network: false,
            host_pid: false,
            host_ipc: false,
            share_process_namespace: None,
            host_users: None,
            selinux_options: self.se_linux_options.map(|v| v.to_internal()),
            windows_options: self.windows_options.map(|v| v.to_internal()),
            run_as_user: self.run_as_user,
            run_as_group: self.run_as_group,
            run_as_non_root: self.run_as_non_root,
            supplemental_groups: self.supplemental_groups,
            supplemental_groups_policy: supplemental_groups_policy_from_string(
                self.supplemental_groups_policy,
            ),
            fs_group: self.fs_group,
            fs_group_change_policy: fs_group_change_policy_from_string(self.fs_group_change_policy),
            sysctls: self.sysctls.into_iter().map(|s| s.to_internal()).collect(),
            seccomp_profile: self.seccomp_profile.map(|v| v.to_internal()),
            app_armor_profile: self.app_armor_profile.map(|v| v.to_internal()),
            selinux_change_policy: None,
        }
    }
}

impl FromInternal<internal::PodSecurityContext> for security::PodSecurityContext {
    fn from_internal(value: internal::PodSecurityContext) -> Self {
        Self {
            se_linux_options: value
                .selinux_options
                .map(security::SELinuxOptions::from_internal),
            windows_options: value
                .windows_options
                .map(security::WindowsSecurityContextOptions::from_internal),
            run_as_user: value.run_as_user,
            run_as_group: value.run_as_group,
            run_as_non_root: value.run_as_non_root,
            supplemental_groups: value.supplemental_groups,
            supplemental_groups_policy: supplemental_groups_policy_to_string(
                value.supplemental_groups_policy,
            ),
            fs_group: value.fs_group,
            fs_group_change_policy: fs_group_change_policy_to_string(value.fs_group_change_policy),
            seccomp_profile: value
                .seccomp_profile
                .map(security::SeccompProfile::from_internal),
            app_armor_profile: value
                .app_armor_profile
                .map(security::AppArmorProfile::from_internal),
            sysctls: value
                .sysctls
                .into_iter()
                .map(security::Sysctl::from_internal)
                .collect(),
        }
    }
}

fn supplemental_groups_policy_from_string(
    value: Option<security::SupplementalGroupsPolicy>,
) -> Option<internal::SupplementalGroupsPolicy> {
    match value.as_deref() {
        Some("Merge") => Some(internal::SupplementalGroupsPolicy::Merge),
        Some("Strict") => Some(internal::SupplementalGroupsPolicy::Strict),
        _ => None,
    }
}

fn supplemental_groups_policy_to_string(
    value: Option<internal::SupplementalGroupsPolicy>,
) -> Option<security::SupplementalGroupsPolicy> {
    match value {
        Some(internal::SupplementalGroupsPolicy::Merge) => Some("Merge".to_string()),
        Some(internal::SupplementalGroupsPolicy::Strict) => Some("Strict".to_string()),
        None => None,
    }
}

fn fs_group_change_policy_from_string(
    value: Option<String>,
) -> Option<internal::PodFSGroupChangePolicy> {
    match value.as_deref() {
        Some("OnRootMismatch") => Some(internal::PodFSGroupChangePolicy::OnRootMismatch),
        Some("Always") => Some(internal::PodFSGroupChangePolicy::Always),
        _ => None,
    }
}

fn fs_group_change_policy_to_string(
    value: Option<internal::PodFSGroupChangePolicy>,
) -> Option<String> {
    match value {
        Some(internal::PodFSGroupChangePolicy::OnRootMismatch) => {
            Some("OnRootMismatch".to_string())
        }
        Some(internal::PodFSGroupChangePolicy::Always) => Some("Always".to_string()),
        None => None,
    }
}

fn seccomp_profile_type_from_string(value: String) -> internal::SeccompProfileType {
    match value.as_str() {
        "Unconfined" => internal::SeccompProfileType::Unconfined,
        "RuntimeDefault" => internal::SeccompProfileType::RuntimeDefault,
        "Localhost" => internal::SeccompProfileType::Localhost,
        _ => internal::SeccompProfileType::RuntimeDefault,
    }
}

fn seccomp_profile_type_to_string(value: internal::SeccompProfileType) -> String {
    match value {
        internal::SeccompProfileType::Unconfined => "Unconfined".to_string(),
        internal::SeccompProfileType::RuntimeDefault => "RuntimeDefault".to_string(),
        internal::SeccompProfileType::Localhost => "Localhost".to_string(),
    }
}

fn app_armor_profile_type_from_string(value: String) -> internal::AppArmorProfileType {
    match value.as_str() {
        "Unconfined" => internal::AppArmorProfileType::Unconfined,
        "RuntimeDefault" => internal::AppArmorProfileType::RuntimeDefault,
        "Localhost" => internal::AppArmorProfileType::Localhost,
        _ => internal::AppArmorProfileType::RuntimeDefault,
    }
}

fn app_armor_profile_type_to_string(value: internal::AppArmorProfileType) -> String {
    match value {
        internal::AppArmorProfileType::Unconfined => "Unconfined".to_string(),
        internal::AppArmorProfileType::RuntimeDefault => "RuntimeDefault".to_string(),
        internal::AppArmorProfileType::Localhost => "Localhost".to_string(),
    }
}

// ============================================================================
// PodSpec - Note: PodDNSConfig, PodOS, and PodSchedulingGate conversions
// are implemented in scheduling.rs.
// ============================================================================

impl ToInternal<internal::PodSpec> for pod::PodSpec {
    fn to_internal(self) -> internal::PodSpec {
        let mut security_context = self.security_context.map(|sc| sc.to_internal());
        if self.host_network
            || self.host_pid
            || self.host_ipc
            || self.share_process_namespace.is_some()
            || self.host_users.is_some()
        {
            let context =
                security_context.get_or_insert_with(internal::PodSecurityContext::default);
            context.host_network = self.host_network;
            context.host_pid = self.host_pid;
            context.host_ipc = self.host_ipc;
            context.share_process_namespace = self.share_process_namespace;
            context.host_users = self.host_users;
        }

        let service_account_name = self
            .service_account_name
            .or(self.deprecated_service_account)
            .unwrap_or_default();

        internal::PodSpec {
            volumes: self.volumes.into_iter().map(|v| v.to_internal()).collect(),
            init_containers: self.init_containers,
            containers: self.containers,
            ephemeral_containers: self.ephemeral_containers,
            restart_policy: option_string_to_restart_policy(self.restart_policy),
            termination_grace_period_seconds: self.termination_grace_period_seconds,
            active_deadline_seconds: self.active_deadline_seconds,
            dns_policy: option_string_to_dns_policy(self.dns_policy),
            node_selector: self.node_selector,
            service_account_name,
            automount_service_account_token: self.automount_service_account_token,
            node_name: self.node_name.unwrap_or_default(),
            security_context,
            image_pull_secrets: self
                .image_pull_secrets
                .into_iter()
                .map(|s| s.to_internal())
                .collect(),
            hostname: self.hostname.unwrap_or_default(),
            subdomain: self.subdomain.unwrap_or_default(),
            set_hostname_as_fqdn: self.set_hostname_as_fqdn,
            affinity: self.affinity.map(|a| a.to_internal()),
            scheduler_name: self.scheduler_name.unwrap_or_default(),
            tolerations: self
                .tolerations
                .into_iter()
                .map(|t| t.to_internal())
                .collect(),
            host_aliases: self
                .host_aliases
                .into_iter()
                .map(|h| h.to_internal())
                .collect(),
            priority_class_name: self.priority_class_name.unwrap_or_default(),
            priority: self.priority,
            preemption_policy: self.preemption_policy.and_then(|s| match s.as_str() {
                "PreemptLowerPriority" => Some(internal::PreemptionPolicy::PreemptLowerPriority),
                "Never" => Some(internal::PreemptionPolicy::Never),
                _ => None,
            }),
            dns_config: self.dns_config.map(|dc| dc.to_internal()),
            readiness_gates: self.readiness_gates,
            runtime_class_name: self.runtime_class_name,
            overhead: self.overhead,
            enable_service_links: self.enable_service_links,
            topology_spread_constraints: self.topology_spread_constraints,
            os: self.os.map(|os| os.to_internal()),
            scheduling_gates: self
                .scheduling_gates
                .into_iter()
                .map(|sg| sg.to_internal())
                .collect(),
            resource_claims: self
                .resource_claims
                .into_iter()
                .map(|claim| claim.to_internal())
                .collect(),
            resources: self.resources.map(|resources| resources.to_internal()),
        }
    }
}

impl FromInternal<internal::PodSpec> for pod::PodSpec {
    fn from_internal(value: internal::PodSpec) -> Self {
        use crate::core::v1::{affinity, reference, toleration};

        let (host_network, host_pid, host_ipc, share_process_namespace, host_users, security_ctx) =
            if let Some(security_context) = value.security_context {
                let host_network = security_context.host_network;
                let host_pid = security_context.host_pid;
                let host_ipc = security_context.host_ipc;
                let share_process_namespace = security_context.share_process_namespace;
                let host_users = security_context.host_users;
                let security_ctx = security::PodSecurityContext::from_internal(security_context);
                let security_ctx = if is_empty_pod_security_context(&security_ctx) {
                    None
                } else {
                    Some(security_ctx)
                };
                (
                    host_network,
                    host_pid,
                    host_ipc,
                    share_process_namespace,
                    host_users,
                    security_ctx,
                )
            } else {
                (false, false, false, None, None, None)
            };

        Self {
            containers: value.containers,
            init_containers: value.init_containers,
            ephemeral_containers: value.ephemeral_containers,
            restart_policy: restart_policy_to_option_string(value.restart_policy),
            termination_grace_period_seconds: value.termination_grace_period_seconds,
            active_deadline_seconds: value.active_deadline_seconds,
            dns_policy: dns_policy_to_option_string(value.dns_policy),
            dns_config: value.dns_config.map(pod::PodDNSConfig::from_internal),
            node_selector: value.node_selector,
            deprecated_service_account: if value.service_account_name.is_empty() {
                None
            } else {
                Some(value.service_account_name.clone())
            },
            service_account_name: if value.service_account_name.is_empty() {
                None
            } else {
                Some(value.service_account_name)
            },
            automount_service_account_token: value.automount_service_account_token,
            node_name: if value.node_name.is_empty() {
                None
            } else {
                Some(value.node_name)
            },
            host_network,
            host_pid,
            host_ipc,
            share_process_namespace,
            security_context: security_ctx,
            image_pull_secrets: value
                .image_pull_secrets
                .into_iter()
                .map(reference::LocalObjectReference::from_internal)
                .collect(),
            hostname: if value.hostname.is_empty() {
                None
            } else {
                Some(value.hostname)
            },
            subdomain: if value.subdomain.is_empty() {
                None
            } else {
                Some(value.subdomain)
            },
            affinity: value.affinity.map(affinity::Affinity::from_internal),
            scheduler_name: if value.scheduler_name.is_empty() {
                None
            } else {
                Some(value.scheduler_name)
            },
            tolerations: value
                .tolerations
                .into_iter()
                .map(toleration::Toleration::from_internal)
                .collect(),
            host_aliases: value
                .host_aliases
                .into_iter()
                .map(pod::HostAlias::from_internal)
                .collect(),
            priority_class_name: if value.priority_class_name.is_empty() {
                None
            } else {
                Some(value.priority_class_name)
            },
            priority: value.priority,
            readiness_gates: value.readiness_gates,
            runtime_class_name: value.runtime_class_name,
            enable_service_links: value.enable_service_links,
            os: value.os.map(pod::PodOS::from_internal),
            host_users,
            scheduling_gates: value
                .scheduling_gates
                .into_iter()
                .map(pod::PodSchedulingGate::from_internal)
                .collect(),
            volumes: value
                .volumes
                .into_iter()
                .map(volume::Volume::from_internal)
                .collect(),
            resource_claims: value
                .resource_claims
                .into_iter()
                .map(pod_resources::PodResourceClaim::from_internal)
                .collect(),
            overhead: value.overhead,
            topology_spread_constraints: value.topology_spread_constraints,
            resources: value
                .resources
                .map(resource::ResourceRequirements::from_internal),
            set_hostname_as_fqdn: value.set_hostname_as_fqdn,
            preemption_policy: value.preemption_policy.map(|p| match p {
                internal::PreemptionPolicy::PreemptLowerPriority => {
                    "PreemptLowerPriority".to_string()
                }
                internal::PreemptionPolicy::Never => "Never".to_string(),
            }),
        }
    }
}

fn is_empty_pod_security_context(value: &security::PodSecurityContext) -> bool {
    value.se_linux_options.is_none()
        && value.windows_options.is_none()
        && value.run_as_user.is_none()
        && value.run_as_group.is_none()
        && value.run_as_non_root.is_none()
        && value.supplemental_groups.is_empty()
        && value.supplemental_groups_policy.is_none()
        && value.fs_group.is_none()
        && value.fs_group_change_policy.is_none()
        && value.seccomp_profile.is_none()
        && value.app_armor_profile.is_none()
        && value.sysctls.is_empty()
}

// ============================================================================
// ResourceStatus
// ============================================================================

impl ToInternal<internal::ResourceHealth> for pod::ResourceHealth {
    fn to_internal(self) -> internal::ResourceHealth {
        internal::ResourceHealth {
            resource_id: self.resource_id,
            health: self.health,
        }
    }
}

impl FromInternal<internal::ResourceHealth> for pod::ResourceHealth {
    fn from_internal(value: internal::ResourceHealth) -> Self {
        Self {
            resource_id: value.resource_id,
            health: value.health,
        }
    }
}

impl ToInternal<internal::ResourceStatus> for pod::ResourceStatus {
    fn to_internal(self) -> internal::ResourceStatus {
        internal::ResourceStatus {
            name: self.name,
            resources: self
                .resources
                .into_iter()
                .map(|resource| resource.to_internal())
                .collect(),
        }
    }
}

impl FromInternal<internal::ResourceStatus> for pod::ResourceStatus {
    fn from_internal(value: internal::ResourceStatus) -> Self {
        Self {
            name: value.name,
            resources: value
                .resources
                .into_iter()
                .map(pod::ResourceHealth::from_internal)
                .collect(),
        }
    }
}

fn option_string_to_pod_resize_status(value: Option<String>) -> Option<internal::PodResizeStatus> {
    match value.as_deref() {
        Some("InProgress") => Some(internal::PodResizeStatus::InProgress),
        Some("Deferred") => Some(internal::PodResizeStatus::Deferred),
        Some("Infeasible") => Some(internal::PodResizeStatus::Infeasible),
        _ => None,
    }
}

fn pod_resize_status_to_option_string(value: Option<internal::PodResizeStatus>) -> Option<String> {
    match value {
        Some(internal::PodResizeStatus::InProgress) => Some("InProgress".to_string()),
        Some(internal::PodResizeStatus::Deferred) => Some("Deferred".to_string()),
        Some(internal::PodResizeStatus::Infeasible) => Some("Infeasible".to_string()),
        None => None,
    }
}

fn option_string_to_signal(value: Option<String>) -> Option<internal::Signal> {
    let signal = value?;
    let encoded = serde_json::to_string(&signal).ok()?;
    serde_json::from_str::<internal::Signal>(&encoded).ok()
}

fn signal_to_option_string(value: Option<internal::Signal>) -> Option<String> {
    let signal = value?;
    let encoded = serde_json::to_string(&signal).ok()?;
    serde_json::from_str::<String>(&encoded).ok()
}

// ============================================================================
// PodStatus
// ============================================================================

impl ToInternal<internal::PodStatus> for pod::PodStatus {
    fn to_internal(self) -> internal::PodStatus {
        // Special handling for PodIP/PodIPs dual-field compatibility
        // (from upstream conversion.go lines 258-294)
        let pod_ips = if let Some(ref pod_ip) = self.pod_ip {
            if !self.pod_ips.is_empty() && pod_ip != &self.pod_ips[0].ip {
                // If both differ, pod_ip is authoritative
                vec![internal::PodIP { ip: pod_ip.clone() }]
            } else if !self.pod_ips.is_empty() {
                // Use pod_ips if non-empty
                self.pod_ips.into_iter().map(|p| p.to_internal()).collect()
            } else {
                // Use pod_ip if set
                vec![internal::PodIP { ip: pod_ip.clone() }]
            }
        } else if !self.pod_ips.is_empty() {
            // Use pod_ips if pod_ip is None
            self.pod_ips.into_iter().map(|p| p.to_internal()).collect()
        } else {
            vec![]
        };

        // Similar handling for host_ip/host_ips
        let (host_ip, host_ips) = if let Some(host_ip_str) = self.host_ip {
            if !self.host_ips.is_empty() && host_ip_str != self.host_ips[0].ip {
                // If both differ, host_ip is authoritative
                (
                    host_ip_str.clone(),
                    vec![internal::HostIP { ip: host_ip_str }],
                )
            } else if !self.host_ips.is_empty() {
                // Use host_ips if non-empty
                let first_ip = self.host_ips[0].ip.clone();
                (
                    first_ip,
                    self.host_ips.into_iter().map(|h| h.to_internal()).collect(),
                )
            } else {
                // Use host_ip if set
                (
                    host_ip_str.clone(),
                    vec![internal::HostIP { ip: host_ip_str }],
                )
            }
        } else if !self.host_ips.is_empty() {
            let first_ip = self.host_ips[0].ip.clone();
            (
                first_ip,
                self.host_ips.into_iter().map(|h| h.to_internal()).collect(),
            )
        } else {
            (String::new(), vec![])
        };

        internal::PodStatus {
            observed_generation: self.observed_generation.unwrap_or(0),
            phase: option_string_to_pod_phase(self.phase),
            conditions: self
                .conditions
                .into_iter()
                .map(|c| c.to_internal())
                .collect(),
            message: self.message.unwrap_or_default(),
            reason: self.reason.unwrap_or_default(),
            nominated_node_name: self.nominated_node_name.unwrap_or_default(),
            host_ip,
            host_ips,
            pod_ips,
            start_time: self.start_time,
            qos_class: self.qos_class.unwrap_or_default(),
            init_container_statuses: self
                .init_container_statuses
                .into_iter()
                .map(|s| s.to_internal())
                .collect(),
            container_statuses: self
                .container_statuses
                .into_iter()
                .map(|s| s.to_internal())
                .collect(),
            ephemeral_container_statuses: self
                .ephemeral_container_statuses
                .into_iter()
                .map(|s| s.to_internal())
                .collect(),
            resource_claim_statuses: self
                .resource_claim_statuses
                .into_iter()
                .map(|status| status.to_internal())
                .collect(),
            resize: option_string_to_pod_resize_status(self.resize),
        }
    }
}

impl FromInternal<internal::PodStatus> for pod::PodStatus {
    fn from_internal(value: internal::PodStatus) -> Self {
        // Extract pod_ip from pod_ips if available
        let (pod_ip, pod_ips) = if !value.pod_ips.is_empty() {
            let first_ip = Some(value.pod_ips[0].ip.clone());
            let ips = value
                .pod_ips
                .into_iter()
                .map(pod::PodIP::from_internal)
                .collect();
            (first_ip, ips)
        } else {
            (None, vec![])
        };

        // Extract host_ip from host_ips if available
        let (host_ip, host_ips) = if !value.host_ips.is_empty() {
            let first_ip = Some(value.host_ips[0].ip.clone());
            let ips = value
                .host_ips
                .into_iter()
                .map(pod::HostIP::from_internal)
                .collect();
            (first_ip, ips)
        } else {
            (None, vec![])
        };

        Self {
            phase: pod_phase_to_option_string(value.phase),
            host_ip,
            pod_ip,
            host_ips,
            conditions: value
                .conditions
                .into_iter()
                .map(pod::PodCondition::from_internal)
                .collect(),
            container_statuses: value
                .container_statuses
                .into_iter()
                .map(pod::ContainerStatus::from_internal)
                .collect(),
            init_container_statuses: value
                .init_container_statuses
                .into_iter()
                .map(pod::ContainerStatus::from_internal)
                .collect(),
            qos_class: if value.qos_class.is_empty() {
                None
            } else {
                Some(value.qos_class)
            },
            start_time: value.start_time,
            pod_ips,
            reason: if value.reason.is_empty() {
                None
            } else {
                Some(value.reason)
            },
            message: if value.message.is_empty() {
                None
            } else {
                Some(value.message)
            },
            ephemeral_container_statuses: value
                .ephemeral_container_statuses
                .into_iter()
                .map(pod::ContainerStatus::from_internal)
                .collect(),
            resource_claim_statuses: value
                .resource_claim_statuses
                .into_iter()
                .map(pod_resources::PodResourceClaimStatus::from_internal)
                .collect(),
            resize: pod_resize_status_to_option_string(value.resize),
            observed_generation: if value.observed_generation == 0 {
                None
            } else {
                Some(value.observed_generation)
            },
            nominated_node_name: if value.nominated_node_name.is_empty() {
                None
            } else {
                Some(value.nominated_node_name)
            },
        }
    }
}

// ============================================================================
// Init Container Annotation Cleanup
// ============================================================================

const INIT_CONTAINER_ANNOTATIONS: &[&str] = &[
    "pod.beta.kubernetes.io/init-containers",
    "pod.alpha.kubernetes.io/init-containers",
    "pod.beta.kubernetes.io/init-container-statuses",
    "pod.alpha.kubernetes.io/init-container-statuses",
];

fn drop_init_container_annotations(annotations: &mut BTreeMap<String, String>) {
    for key in INIT_CONTAINER_ANNOTATIONS {
        annotations.remove(*key);
    }
}

// ============================================================================
// Pod
// ============================================================================

impl ToInternal<internal::Pod> for pod::Pod {
    fn to_internal(self) -> internal::Pod {
        let mut result = internal::Pod {
            type_meta: crate::common::TypeMeta::default(),
            metadata: option_object_meta_to_meta(self.metadata),
            spec: self.spec.unwrap_or_default().to_internal(),
            status: self.status.unwrap_or_default().to_internal(),
        };
        drop_init_container_annotations(&mut result.metadata.annotations);
        // Clamp negative grace period to 1 (Pod-level only, not PodSpec)
        if let Some(grace) = result.spec.termination_grace_period_seconds {
            if grace < 0 {
                result.spec.termination_grace_period_seconds = Some(1);
            }
        }
        result
    }
}

impl FromInternal<internal::Pod> for pod::Pod {
    fn from_internal(value: internal::Pod) -> Self {
        let mut result = Self {
            type_meta: crate::common::TypeMeta::default(),
            metadata: meta_to_option_object_meta(value.metadata),
            spec: Some(pod::PodSpec::from_internal(value.spec)),
            status: Some(pod::PodStatus::from_internal(value.status)),
        };
        result.apply_default();
        if let Some(ref mut meta) = result.metadata {
            drop_init_container_annotations(&mut meta.annotations);
        }
        // Clamp negative grace period to 1
        if let Some(ref mut spec) = result.spec {
            if let Some(grace) = spec.termination_grace_period_seconds {
                if grace < 0 {
                    spec.termination_grace_period_seconds = Some(1);
                }
            }
        }
        result
    }
}

// ============================================================================
// PodList
// ============================================================================

impl ToInternal<internal::PodList> for pod::PodList {
    fn to_internal(self) -> internal::PodList {
        internal::PodList {
            type_meta: crate::common::TypeMeta::default(),
            metadata: option_list_meta_to_meta(self.metadata),
            items: self.items.into_iter().map(|p| p.to_internal()).collect(),
        }
    }
}

impl FromInternal<internal::PodList> for pod::PodList {
    fn from_internal(value: internal::PodList) -> Self {
        let mut result = Self {
            type_meta: crate::common::TypeMeta::default(),
            metadata: meta_to_option_list_meta(value.metadata),
            items: value
                .items
                .into_iter()
                .map(pod::Pod::from_internal)
                .collect(),
        };
        result.apply_default();
        result
    }
}

// ============================================================================
// PodTemplateSpec
// ============================================================================

impl ToInternal<internal::PodTemplateSpec> for template::PodTemplateSpec {
    fn to_internal(self) -> internal::PodTemplateSpec {
        let mut result = internal::PodTemplateSpec {
            metadata: option_object_meta_to_meta(self.metadata),
            spec: self.spec.unwrap_or_default().to_internal(),
        };
        drop_init_container_annotations(&mut result.metadata.annotations);
        result
    }
}

impl FromInternal<internal::PodTemplateSpec> for template::PodTemplateSpec {
    fn from_internal(value: internal::PodTemplateSpec) -> Self {
        let mut meta = meta_to_option_object_meta(value.metadata);
        if let Some(ref mut m) = meta {
            drop_init_container_annotations(&mut m.annotations);
        }
        Self {
            metadata: meta,
            spec: Some(pod::PodSpec::from_internal(value.spec)),
        }
    }
}

// ============================================================================
// PodTemplate
// ============================================================================

impl ToInternal<internal::PodTemplate> for template::PodTemplate {
    fn to_internal(self) -> internal::PodTemplate {
        internal::PodTemplate {
            type_meta: crate::common::TypeMeta::default(),
            metadata: option_object_meta_to_meta(self.metadata),
            template: self.template.unwrap_or_default().to_internal(),
        }
    }
}

impl FromInternal<internal::PodTemplate> for template::PodTemplate {
    fn from_internal(value: internal::PodTemplate) -> Self {
        let mut result = Self {
            type_meta: crate::common::TypeMeta::default(),
            metadata: meta_to_option_object_meta(value.metadata),
            template: Some(template::PodTemplateSpec::from_internal(value.template)),
        };
        result.apply_default();
        result
    }
}

// ============================================================================
// PodTemplateList
// ============================================================================

impl ToInternal<internal::PodTemplateList> for template::PodTemplateList {
    fn to_internal(self) -> internal::PodTemplateList {
        internal::PodTemplateList {
            type_meta: crate::common::TypeMeta::default(),
            metadata: option_list_meta_to_meta(self.metadata),
            items: self
                .items
                .into_iter()
                .map(|item| item.to_internal())
                .collect(),
        }
    }
}

impl FromInternal<internal::PodTemplateList> for template::PodTemplateList {
    fn from_internal(value: internal::PodTemplateList) -> Self {
        let mut result = Self {
            type_meta: crate::common::TypeMeta::default(),
            metadata: meta_to_option_list_meta(value.metadata),
            items: value
                .items
                .into_iter()
                .map(template::PodTemplate::from_internal)
                .collect(),
        };
        result.apply_default();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Timestamp;

    #[test]
    fn test_pod_ip_roundtrip() {
        let v1_ip = pod::PodIP {
            ip: "192.168.1.1".to_string(),
        };

        let internal_ip = v1_ip.clone().to_internal();
        let roundtrip = pod::PodIP::from_internal(internal_ip);

        assert_eq!(v1_ip, roundtrip);
    }

    #[test]
    fn test_host_ip_roundtrip() {
        let v1_ip = pod::HostIP {
            ip: "10.0.0.1".to_string(),
        };

        let internal_ip = v1_ip.clone().to_internal();
        let roundtrip = pod::HostIP::from_internal(internal_ip);

        assert_eq!(v1_ip, roundtrip);
    }

    #[test]
    fn test_host_alias_roundtrip() {
        let v1_alias = pod::HostAlias {
            ip: "127.0.0.1".to_string(),
            hostnames: vec!["localhost".to_string(), "localhost.localdomain".to_string()],
        };

        let internal_alias = v1_alias.clone().to_internal();
        let roundtrip = pod::HostAlias::from_internal(internal_alias);

        assert_eq!(v1_alias, roundtrip);
    }

    #[test]
    fn test_pod_condition_roundtrip() {
        let v1_condition = pod::PodCondition {
            type_: "Ready".to_string(),
            status: "True".to_string(),
            last_probe_time: Some(Timestamp::from_str("2009-02-13T23:31:30Z").unwrap()),
            last_transition_time: Some(Timestamp::from_str("2009-02-13T23:31:30Z").unwrap()),
            reason: Some("ContainersReady".to_string()),
            message: Some("All containers are ready".to_string()),
            observed_generation: None,
        };

        let internal_condition = v1_condition.clone().to_internal();
        let roundtrip = pod::PodCondition::from_internal(internal_condition);

        assert_eq!(v1_condition, roundtrip);
    }

    #[test]
    fn test_container_port_roundtrip() {
        let v1_port = pod::ContainerPort {
            name: Some("http".to_string()),
            host_port: Some(8080),
            container_port: 80,
            protocol: Some("TCP".to_string()),
            host_ip: Some("0.0.0.0".to_string()),
        };

        let internal_port = v1_port.clone().to_internal();
        let roundtrip = pod::ContainerPort::from_internal(internal_port);

        assert_eq!(v1_port, roundtrip);
    }

    #[test]
    fn test_container_state_roundtrip() {
        let v1_state = pod::ContainerState {
            waiting: None,
            running: Some(pod::ContainerStateRunning {
                started_at: Some(Timestamp::from_str("2009-02-13T23:31:30Z").unwrap()),
            }),
            terminated: None,
        };

        let internal_state = v1_state.clone().to_internal();
        let roundtrip = pod::ContainerState::from_internal(internal_state);

        assert_eq!(v1_state, roundtrip);
    }

    #[test]
    fn test_container_status_conversion() {
        let v1_status = pod::ContainerStatus {
            name: "nginx".to_string(),
            state: Some(pod::ContainerState {
                waiting: None,
                running: Some(pod::ContainerStateRunning {
                    started_at: Some(Timestamp::from_str("2009-02-13T23:31:30Z").unwrap()),
                }),
                terminated: None,
            }),
            last_state: None,
            ready: true,
            restart_count: 0,
            image: Some("nginx:latest".to_string()),
            image_id: Some("docker-pullable://nginx@sha256:abc123".to_string()),
            container_id: Some("docker://abc123".to_string()),
            started: Some(true),
            allocated_resources: None,
            resources: None,
            volume_mounts: vec![],
            user: None,
            allocated_resources_status: vec![],
            stop_signal: None,
        };

        let internal_status = v1_status.clone().to_internal();
        let roundtrip = pod::ContainerStatus::from_internal(internal_status);

        // Core fields should survive the round trip
        assert_eq!(v1_status.name, roundtrip.name);
        assert_eq!(v1_status.state, roundtrip.state);
        assert_eq!(v1_status.last_state, roundtrip.last_state);
        assert_eq!(v1_status.ready, roundtrip.ready);
        assert_eq!(v1_status.restart_count, roundtrip.restart_count);
        assert_eq!(v1_status.image, roundtrip.image);
        assert_eq!(v1_status.image_id, roundtrip.image_id);
        assert_eq!(v1_status.container_id, roundtrip.container_id);
        assert_eq!(v1_status.started, roundtrip.started);
        // New fields are not in internal, so they won't survive round trip
    }

    #[test]
    fn test_pod_status_podip_compatibility() {
        // Test case 1: pod_ip and pod_ips[0] match - should use pod_ips
        let v1_status = pod::PodStatus {
            pod_ip: Some("10.0.0.1".to_string()),
            pod_ips: vec![
                pod::PodIP {
                    ip: "10.0.0.1".to_string(),
                },
                pod::PodIP {
                    ip: "10.0.0.2".to_string(),
                },
            ],
            ..Default::default()
        };

        let internal_status = v1_status.to_internal();
        assert_eq!(internal_status.pod_ips.len(), 2);
        assert_eq!(internal_status.pod_ips[0].ip, "10.0.0.1");
        assert_eq!(internal_status.pod_ips[1].ip, "10.0.0.2");

        // Test case 2: pod_ip and pod_ips[0] differ - pod_ip is authoritative
        let v1_status = pod::PodStatus {
            pod_ip: Some("192.168.1.1".to_string()),
            pod_ips: vec![pod::PodIP {
                ip: "10.0.0.1".to_string(),
            }],
            ..Default::default()
        };

        let internal_status = v1_status.to_internal();
        assert_eq!(internal_status.pod_ips.len(), 1);
        assert_eq!(internal_status.pod_ips[0].ip, "192.168.1.1"); // pod_ip wins

        // Test case 3: Only pod_ip is set
        let v1_status = pod::PodStatus {
            pod_ip: Some("172.16.0.1".to_string()),
            pod_ips: vec![],
            ..Default::default()
        };

        let internal_status = v1_status.to_internal();
        assert_eq!(internal_status.pod_ips.len(), 1);
        assert_eq!(internal_status.pod_ips[0].ip, "172.16.0.1");
    }

    #[test]
    fn test_pod_spec_basic_roundtrip() {
        use crate::core::v1::Container;

        let v1_spec = pod::PodSpec {
            containers: vec![Container {
                name: "nginx".to_string(),
                image: Some("nginx:latest".to_string()),
                ..Default::default()
            }],
            restart_policy: Some("Always".to_string()),
            dns_policy: Some("ClusterFirst".to_string()),
            service_account_name: Some("default".to_string()),
            node_name: Some("node-1".to_string()),
            hostname: Some("my-pod".to_string()),
            ..Default::default()
        };

        let internal_spec = v1_spec.clone().to_internal();
        let roundtrip = pod::PodSpec::from_internal(internal_spec);

        // Check key fields that survive roundtrip
        assert_eq!(v1_spec.containers[0].name, roundtrip.containers[0].name);
        assert_eq!(v1_spec.restart_policy, roundtrip.restart_policy);
        assert_eq!(v1_spec.dns_policy, roundtrip.dns_policy);
        assert_eq!(v1_spec.service_account_name, roundtrip.service_account_name);
        assert_eq!(v1_spec.node_name, roundtrip.node_name);
        assert_eq!(v1_spec.hostname, roundtrip.hostname);
    }

    #[test]
    fn test_pod_roundtrip() {
        use crate::common::ObjectMeta;
        use crate::core::v1::Container;

        let v1_pod = pod::Pod {
            type_meta: crate::common::TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("my-pod".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(pod::PodSpec {
                containers: vec![Container {
                    name: "nginx".to_string(),
                    image: Some("nginx:latest".to_string()),
                    ..Default::default()
                }],
                ..Default::default()
            }),
            status: Some(pod::PodStatus {
                phase: Some("Running".to_string()),
                pod_ip: Some("10.0.0.1".to_string()),
                ..Default::default()
            }),
        };

        let internal_pod = v1_pod.clone().to_internal();
        let roundtrip = pod::Pod::from_internal(internal_pod);

        // Check metadata
        assert_eq!(
            v1_pod.metadata.as_ref().unwrap().name,
            roundtrip.metadata.as_ref().unwrap().name
        );
        assert_eq!(
            v1_pod.metadata.as_ref().unwrap().namespace,
            roundtrip.metadata.as_ref().unwrap().namespace
        );

        // Check spec
        assert_eq!(
            v1_pod.spec.as_ref().unwrap().containers[0].name,
            roundtrip.spec.as_ref().unwrap().containers[0].name
        );

        // Check status
        assert_eq!(
            v1_pod.status.as_ref().unwrap().phase,
            roundtrip.status.as_ref().unwrap().phase
        );
        assert_eq!(
            v1_pod.status.as_ref().unwrap().pod_ip,
            roundtrip.status.as_ref().unwrap().pod_ip
        );
    }

    #[test]
    fn test_pod_template_roundtrip() {
        use crate::common::ObjectMeta;
        use crate::core::v1::Container;

        let v1_template = template::PodTemplate {
            metadata: Some(ObjectMeta {
                name: Some("tmpl".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            template: Some(template::PodTemplateSpec {
                metadata: Some(ObjectMeta {
                    labels: [("app".to_string(), "demo".to_string())].into(),
                    ..Default::default()
                }),
                spec: Some(pod::PodSpec {
                    containers: vec![Container {
                        name: "nginx".to_string(),
                        image: Some("nginx:latest".to_string()),
                        ..Default::default()
                    }],
                    ..Default::default()
                }),
            }),
            ..Default::default()
        };

        let internal_template = v1_template.clone().to_internal();
        let roundtrip = template::PodTemplate::from_internal(internal_template);

        assert_eq!(
            v1_template.metadata.as_ref().unwrap().name,
            roundtrip.metadata.as_ref().unwrap().name
        );
        assert_eq!(
            v1_template
                .template
                .as_ref()
                .unwrap()
                .spec
                .as_ref()
                .unwrap()
                .containers[0]
                .name,
            roundtrip
                .template
                .as_ref()
                .unwrap()
                .spec
                .as_ref()
                .unwrap()
                .containers[0]
                .name
        );
    }
}
