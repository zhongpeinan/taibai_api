//! Kubernetes Security Context types
//!
//! This module contains security-related types from the Kubernetes core/v1 API.
//! These types are used for configuring security contexts for pods and containers.

use serde::{Deserialize, Serialize};

// ============================================================================
// Type Aliases
// ============================================================================

/// Capability represents a Linux capability.
pub type Capability = String;

/// Constants for common Linux capabilities
pub mod capability {
    pub const CAP_AUDIT_CONTROL: &str = "CAP_AUDIT_CONTROL";
    pub const CAP_AUDIT_READ: &str = "CAP_AUDIT_READ";
    pub const CAP_AUDIT_WRITE: &str = "CAP_AUDIT_WRITE";
    pub const CAP_BLOCK_SUSPEND: &str = "CAP_BLOCK_SUSPEND";
    pub const CAP_CHOWN: &str = "CAP_CHOWN";
    pub const CAP_DAC_OVERRIDE: &str = "CAP_DAC_OVERRIDE";
    pub const CAP_DAC_READ_SEARCH: &str = "CAP_DAC_READ_SEARCH";
    pub const CAP_FOWNER: &str = "CAP_FOWNER";
    pub const CAP_FSETID: &str = "CAP_FSETID";
    pub const CAP_IPC_LOCK: &str = "CAP_IPC_LOCK";
    pub const CAP_IPC_OWNER: &str = "CAP_IPC_OWNER";
    pub const CAP_KILL: &str = "CAP_KILL";
    pub const CAP_LEASE: &str = "CAP_LEASE";
    pub const CAP_LINUX_IMMUTABLE: &str = "CAP_LINUX_IMMUTABLE";
    pub const CAP_MAC_ADMIN: &str = "CAP_MAC_ADMIN";
    pub const CAP_MAC_OVERRIDE: &str = "CAP_MAC_OVERRIDE";
    pub const CAP_MKNOD: &str = "CAP_MKNOD";
    pub const CAP_NET_ADMIN: &str = "CAP_NET_ADMIN";
    pub const CAP_NET_BIND_SERVICE: &str = "CAP_NET_BIND_SERVICE";
    pub const CAP_NET_BROADCAST: &str = "CAP_NET_BROADCAST";
    pub const CAP_NET_RAW: &str = "CAP_NET_RAW";
    pub const CAP_SETFCAP: &str = "CAP_SETFCAP";
    pub const CAP_SETGID: &str = "CAP_SETGID";
    pub const CAP_SETPCAP: &str = "CAP_SETPCAP";
    pub const CAP_SETUID: &str = "CAP_SETUID";
    pub const CAP_SYSLOG: &str = "CAP_SYSLOG";
    pub const CAP_SYS_ADMIN: &str = "CAP_SYS_ADMIN";
    pub const CAP_SYS_BOOT: &str = "CAP_SYS_BOOT";
    pub const CAP_SYS_CHROOT: &str = "CAP_SYS_CHROOT";
    pub const CAP_SYS_MODULE: &str = "CAP_SYS_MODULE";
    pub const CAP_SYS_NICE: &str = "CAP_SYS_NICE";
    pub const CAP_SYS_PACCT: &str = "CAP_SYS_PACCT";
    pub const CAP_SYS_PTRACE: &str = "CAP_SYS_PTRACE";
    pub const CAP_SYS_RAWIO: &str = "CAP_SYS_RAWIO";
    pub const CAP_SYS_RESOURCE: &str = "CAP_SYS_RESOURCE";
    pub const CAP_SYS_TIME: &str = "CAP_SYS_TIME";
    pub const CAP_SYS_TTY_CONFIG: &str = "CAP_SYS_TTY_CONFIG";
    pub const CAP_WAKE_ALARM: &str = "CAP_WAKE_ALARM";
}

/// SeccompProfileType defines the supported seccomp profile types.
pub type SeccompProfileType = String;

/// Constants for SeccompProfileType
pub mod seccomp_profile_type {
    pub const UNCONFINED: &str = "Unconfined";
    pub const RUNTIME_DEFAULT: &str = "RuntimeDefault";
    pub const LOCALHOST: &str = "Localhost";
}

/// AppArmorProfileType defines the supported AppArmor profile types.
pub type AppArmorProfileType = String;

/// Constants for AppArmorProfileType
pub mod app_armor_profile_type {
    pub const UNCONFINED: &str = "Unconfined";
    pub const RUNTIME_DEFAULT: &str = "RuntimeDefault";
    pub const LOCALHOST: &str = "Localhost";
}

/// SupplementalGroupsPolicy defines how supplemental groups are calculated.
pub type SupplementalGroupsPolicy = String;

/// Constants for SupplementalGroupsPolicy
pub mod supplemental_groups_policy {
    pub const MERGE: &str = "Merge";
    pub const STRICT: &str = "Strict";
}

/// ProcMountType denotes the type of proc mount to use for the containers.
pub type ProcMountType = String;

/// Constants for ProcMountType
pub mod proc_mount_type {
    pub const DEFAULT: &str = "Default";
    pub const UNMASKED: &str = "Unmasked";
}

// ============================================================================
// Security Types
// ============================================================================

/// Capabilities add or drop Linux capabilities for running containers.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Capabilities {
    /// Added capabilities
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub add: Vec<Capability>,
    /// Removed capabilities
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub drop: Vec<Capability>,
}

/// SELinuxOptions are the labels to be applied to the container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SELinuxOptions {
    /// User is a SELinux user label that applies to the container.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub user: String,
    /// Role is a SELinux role label that applies to the container.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub role: String,
    /// Type is a SELinux type label that applies to the container.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub type_: String,
    /// Level is SELinux level label that applies to the container.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub level: String,
}

/// WindowsSecurityContextOptions contain Windows-specific options and credentials.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct WindowsSecurityContextOptions {
    /// GMSACredentialSpecName is the name of the GMSA credential spec to use.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gmsa_credential_spec_name: Option<String>,
    /// GMSACredentialSpec is where the GMSA admission webhook inlines the contents of the GMSA credential spec.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gmsa_credential_spec: Option<String>,
    /// The UserName in Windows to run the entrypoint of the container process.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_as_user_name: Option<String>,
    /// HostProcess determines if a container should be run as a 'Host Process' container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_process: Option<bool>,
}

/// SeccompProfile defines the seccomp profile to use.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct SeccompProfile {
    /// Type indicates which kind of seccomp profile will be applied.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub type_: SeccompProfileType,
    /// LocalhostProfile indicates a profile defined in a file on the node should be used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub localhost_profile: Option<String>,
}

/// AppArmorProfile defines the AppArmor profile to use.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct AppArmorProfile {
    /// Type indicates which kind of AppArmor profile will be applied.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub type_: AppArmorProfileType,
    /// LocalhostProfile indicates a profile loaded on the node that should be used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub localhost_profile: Option<String>,
}

/// SecurityContext holds security configuration that will be applied to a container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SecurityContext {
    /// The capabilities to add/drop when running containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Capabilities>,
    /// Run container in privileged mode.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    /// The SELinux context to be applied to the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub se_linux_options: Option<SELinuxOptions>,
    /// The Windows specific settings applied to all containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub windows_options: Option<WindowsSecurityContextOptions>,
    /// The UID to run the entrypoint of the container process.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_as_user: Option<i64>,
    /// The GID to run the entrypoint of the container process.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_as_group: Option<i64>,
    /// Indicates that the container must run as a non-root user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_as_non_root: Option<bool>,
    /// Whether this container has a read-only root filesystem.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_only_root_filesystem: Option<bool>,
    /// AllowPrivilegeEscalation controls whether a process can gain more privileges than its parent process.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_privilege_escalation: Option<bool>,
    /// procMount denotes the type of proc mount to use for the containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proc_mount: Option<ProcMountType>,
    /// The seccomp options to use by this container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seccomp_profile: Option<SeccompProfile>,
    /// appArmorProfile is the AppArmor options to use by this container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_armor_profile: Option<AppArmorProfile>,
}

/// PodSecurityContext holds pod-level security attributes and common container settings.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodSecurityContext {
    /// The SELinux context to be applied to all containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub se_linux_options: Option<SELinuxOptions>,
    /// The Windows specific settings applied to all containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub windows_options: Option<WindowsSecurityContextOptions>,
    /// The UID to run the entrypoint of the container process.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_as_user: Option<i64>,
    /// The GID to run the entrypoint of the container process.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_as_group: Option<i64>,
    /// Indicates that the container must run as a non-root user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_as_non_root: Option<bool>,
    /// A list of groups applied to the first process run in each container.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supplemental_groups: Vec<i64>,
    /// Defines how supplemental groups of the first container processes are calculated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supplemental_groups_policy: Option<SupplementalGroupsPolicy>,
    /// A special supplemental group that applies to all containers in a pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fs_group: Option<i64>,
    /// Controls the fsGroup change policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fs_group_change_policy: Option<String>,
    /// The seccomp options to use by the pods in this namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seccomp_profile: Option<SeccompProfile>,
    /// appArmorProfile is the AppArmor options to use by the pods in this namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_armor_profile: Option<AppArmorProfile>,
    /// Sysctls hold a list of namespaced sysctls used for the pod.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sysctls: Vec<Sysctl>,
}

/// Sysctl defines a kernel parameter to be set.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Sysctl {
    /// Name of a property to set.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// Value of a property to set.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub value: String,
}

#[cfg(test)]
mod tests {}
