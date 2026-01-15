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
pub struct SeccompProfile {
    /// Type indicates which kind of seccomp profile will be applied.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub type_: SeccompProfileType,
    /// LocalhostProfile indicates a profile defined in a file on the node should be used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub localhost_profile: Option<String>,
}

impl Default for SeccompProfile {
    fn default() -> Self {
        Self {
            type_: String::new(),
            localhost_profile: None,
        }
    }
}

/// AppArmorProfile defines the AppArmor profile to use.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AppArmorProfile {
    /// Type indicates which kind of AppArmor profile will be applied.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub type_: AppArmorProfileType,
    /// LocalhostProfile indicates a profile loaded on the node that should be used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub localhost_profile: Option<String>,
}

impl Default for AppArmorProfile {
    fn default() -> Self {
        Self {
            type_: String::new(),
            localhost_profile: None,
        }
    }
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
    /// UnsafeSysctls hold a list of namespaced sysctls with unsafe values.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unsafe_sysctls: Vec<Sysctl>,
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
mod tests {
    use super::*;

    // Capabilities tests
    #[test]
    fn test_capabilities_default() {
        let caps = Capabilities::default();
        assert!(caps.add.is_empty());
        assert!(caps.drop.is_empty());
    }

    #[test]
    fn test_capabilities_with_add() {
        let caps = Capabilities {
            add: vec![capability::CAP_NET_ADMIN.to_string()],
            drop: vec![],
        };

        assert_eq!(caps.add.len(), 1);
        assert_eq!(caps.add[0], capability::CAP_NET_ADMIN);
    }

    #[test]
    fn test_capabilities_serialize() {
        let caps = Capabilities {
            add: vec![capability::CAP_CHOWN.to_string()],
            drop: vec![capability::CAP_KILL.to_string()],
        };

        let json = serde_json::to_string(&caps).unwrap();
        assert!(json.contains(r#""add""#));
        assert!(json.contains(r#""drop""#));
        assert!(json.contains("CAP_CHOWN"));
        assert!(json.contains("CAP_KILL"));
    }

    #[test]
    fn test_capabilities_round_trip() {
        let caps = Capabilities {
            add: vec![
                capability::CAP_NET_ADMIN.to_string(),
                capability::CAP_SYS_ADMIN.to_string(),
            ],
            drop: vec![capability::CAP_KILL.to_string()],
        };

        let json = serde_json::to_string(&caps).unwrap();
        let deserialized: Capabilities = serde_json::from_str(&json).unwrap();

        assert_eq!(caps, deserialized);
    }

    // SELinuxOptions tests
    #[test]
    fn test_selinux_options_default() {
        let opts = SELinuxOptions::default();
        assert!(opts.user.is_empty());
        assert!(opts.role.is_empty());
        assert!(opts.type_.is_empty());
        assert!(opts.level.is_empty());
    }

    #[test]
    fn test_selinux_options_with_fields() {
        let opts = SELinuxOptions {
            user: "system_u".to_string(),
            role: "system_r".to_string(),
            type_: "svirt_lxc_net_t".to_string(),
            level: "s0:c100,c200".to_string(),
        };

        assert_eq!(opts.user, "system_u");
        assert_eq!(opts.role, "system_r");
    }

    #[test]
    fn test_selinux_options_serialize() {
        let opts = SELinuxOptions {
            user: "user".to_string(),
            role: "role".to_string(),
            type_: "type".to_string(),
            level: "level".to_string(),
        };

        let json = serde_json::to_string(&opts).unwrap();
        assert!(json.contains(r#""user":"user""#));
        assert!(json.contains(r#""role":"role""#));
        assert!(json.contains(r#""type":"type""#));
        assert!(json.contains(r#""level":"level""#));
    }

    #[test]
    fn test_selinux_options_round_trip() {
        let opts = SELinuxOptions {
            user: "test_u".to_string(),
            role: String::new(),
            type_: "test_t".to_string(),
            level: String::new(),
        };

        let json = serde_json::to_string(&opts).unwrap();
        let deserialized: SELinuxOptions = serde_json::from_str(&json).unwrap();

        assert_eq!(opts, deserialized);
    }

    // SeccompProfile tests
    #[test]
    fn test_seccomp_profile_default() {
        let profile = SeccompProfile::default();
        assert!(profile.type_.is_empty());
        assert!(profile.localhost_profile.is_none());
    }

    #[test]
    fn test_seccomp_profile_runtime_default() {
        let profile = SeccompProfile {
            type_: seccomp_profile_type::RUNTIME_DEFAULT.to_string(),
            localhost_profile: None,
        };

        assert_eq!(profile.type_, seccomp_profile_type::RUNTIME_DEFAULT);
    }

    #[test]
    fn test_seccomp_profile_localhost() {
        let profile = SeccompProfile {
            type_: seccomp_profile_type::LOCALHOST.to_string(),
            localhost_profile: Some("profiles/profile.json".to_string()),
        };

        assert_eq!(profile.type_, seccomp_profile_type::LOCALHOST);
        assert_eq!(
            profile.localhost_profile,
            Some("profiles/profile.json".to_string())
        );
    }

    #[test]
    fn test_seccomp_profile_serialize() {
        let profile = SeccompProfile {
            type_: seccomp_profile_type::UNCONFINED.to_string(),
            localhost_profile: None,
        };

        let json = serde_json::to_string(&profile).unwrap();
        assert!(json.contains(r#""type":"Unconfined""#));
    }

    #[test]
    fn test_seccomp_profile_round_trip() {
        let profile = SeccompProfile {
            type_: seccomp_profile_type::LOCALHOST.to_string(),
            localhost_profile: Some("my-profile".to_string()),
        };

        let json = serde_json::to_string(&profile).unwrap();
        let deserialized: SeccompProfile = serde_json::from_str(&json).unwrap();

        assert_eq!(profile, deserialized);
    }

    // AppArmorProfile tests
    #[test]
    fn test_app_armor_profile_default() {
        let profile = AppArmorProfile::default();
        assert!(profile.type_.is_empty());
        assert!(profile.localhost_profile.is_none());
    }

    #[test]
    fn test_app_armor_profile_runtime_default() {
        let profile = AppArmorProfile {
            type_: app_armor_profile_type::RUNTIME_DEFAULT.to_string(),
            localhost_profile: None,
        };

        assert_eq!(profile.type_, app_armor_profile_type::RUNTIME_DEFAULT);
    }

    #[test]
    fn test_app_armor_profile_localhost() {
        let profile = AppArmorProfile {
            type_: app_armor_profile_type::LOCALHOST.to_string(),
            localhost_profile: Some("docker-default".to_string()),
        };

        assert_eq!(profile.type_, app_armor_profile_type::LOCALHOST);
        assert_eq!(
            profile.localhost_profile,
            Some("docker-default".to_string())
        );
    }

    #[test]
    fn test_app_armor_profile_serialize() {
        let profile = AppArmorProfile {
            type_: app_armor_profile_type::UNCONFINED.to_string(),
            localhost_profile: None,
        };

        let json = serde_json::to_string(&profile).unwrap();
        assert!(json.contains(r#""type":"Unconfined""#));
    }

    #[test]
    fn test_app_armor_profile_round_trip() {
        let profile = AppArmorProfile {
            type_: app_armor_profile_type::LOCALHOST.to_string(),
            localhost_profile: Some("my-profile".to_string()),
        };

        let json = serde_json::to_string(&profile).unwrap();
        let deserialized: AppArmorProfile = serde_json::from_str(&json).unwrap();

        assert_eq!(profile, deserialized);
    }

    // SecurityContext tests
    #[test]
    fn test_security_context_default() {
        let ctx = SecurityContext::default();
        assert!(ctx.capabilities.is_none());
        assert!(ctx.privileged.is_none());
        assert!(ctx.run_as_user.is_none());
    }

    #[test]
    fn test_security_context_with_capabilities() {
        let ctx = SecurityContext {
            capabilities: Some(Capabilities {
                add: vec![capability::CAP_NET_ADMIN.to_string()],
                drop: vec![],
            }),
            ..Default::default()
        };

        assert!(ctx.capabilities.is_some());
        assert_eq!(ctx.capabilities.unwrap().add.len(), 1);
    }

    #[test]
    fn test_security_context_with_run_as_user() {
        let ctx = SecurityContext {
            run_as_user: Some(1000),
            ..Default::default()
        };

        assert_eq!(ctx.run_as_user, Some(1000));
    }

    #[test]
    fn test_security_context_serialize() {
        let ctx = SecurityContext {
            run_as_user: Some(1000),
            run_as_non_root: Some(true),
            ..Default::default()
        };

        let json = serde_json::to_string(&ctx).unwrap();
        assert!(json.contains(r#""runAsUser":1000"#));
        assert!(json.contains(r#""runAsNonRoot":true"#));
    }

    #[test]
    fn test_security_context_round_trip() {
        let ctx = SecurityContext {
            capabilities: Some(Capabilities {
                add: vec![capability::CAP_CHOWN.to_string()],
                drop: vec![],
            }),
            run_as_user: Some(1001),
            run_as_non_root: Some(true),
            read_only_root_filesystem: Some(true),
            ..Default::default()
        };

        let json = serde_json::to_string(&ctx).unwrap();
        let deserialized: SecurityContext = serde_json::from_str(&json).unwrap();

        assert_eq!(ctx, deserialized);
    }

    // PodSecurityContext tests
    #[test]
    fn test_pod_security_context_default() {
        let ctx = PodSecurityContext::default();
        assert!(ctx.run_as_user.is_none());
        assert!(ctx.supplemental_groups.is_empty());
    }

    #[test]
    fn test_pod_security_context_with_fs_group() {
        let ctx = PodSecurityContext {
            fs_group: Some(2000),
            ..Default::default()
        };

        assert_eq!(ctx.fs_group, Some(2000));
    }

    #[test]
    fn test_pod_security_context_with_supplemental_groups() {
        let ctx = PodSecurityContext {
            supplemental_groups: vec![1001, 1002, 1003],
            ..Default::default()
        };

        assert_eq!(ctx.supplemental_groups.len(), 3);
        assert_eq!(ctx.supplemental_groups[0], 1001);
    }

    #[test]
    fn test_pod_security_context_serialize() {
        let ctx = PodSecurityContext {
            run_as_user: Some(1000),
            fs_group: Some(2000),
            supplemental_groups: vec![100],
            ..Default::default()
        };

        let json = serde_json::to_string(&ctx).unwrap();
        assert!(json.contains(r#""runAsUser":1000"#));
        assert!(json.contains(r#""fsGroup":2000"#));
        assert!(json.contains(r#""supplementalGroups":[100]"#));
    }

    #[test]
    fn test_pod_security_context_round_trip() {
        let ctx = PodSecurityContext {
            run_as_user: Some(1000),
            fs_group: Some(2000),
            supplemental_groups: vec![100],
            run_as_non_root: Some(true),
            ..Default::default()
        };

        let json = serde_json::to_string(&ctx).unwrap();
        let deserialized: PodSecurityContext = serde_json::from_str(&json).unwrap();

        assert_eq!(ctx, deserialized);
    }

    // Sysctl tests
    #[test]
    fn test_sysctl_default() {
        let sysctl = Sysctl::default();
        assert!(sysctl.name.is_empty());
        assert!(sysctl.value.is_empty());
    }

    #[test]
    fn test_sysctl_with_fields() {
        let sysctl = Sysctl {
            name: "kernel.shm_rmid_forced".to_string(),
            value: "1".to_string(),
        };

        assert_eq!(sysctl.name, "kernel.shm_rmid_forced");
        assert_eq!(sysctl.value, "1");
    }

    #[test]
    fn test_sysctl_serialize() {
        let sysctl = Sysctl {
            name: "kernel.msgmax".to_string(),
            value: "8192".to_string(),
        };

        let json = serde_json::to_string(&sysctl).unwrap();
        assert!(json.contains(r#""name":"kernel.msgmax""#));
        assert!(json.contains(r#""value":"8192""#));
    }

    #[test]
    fn test_sysctl_round_trip() {
        let sysctl = Sysctl {
            name: "kernel.sem".to_string(),
            value: "320 102400 128 256".to_string(),
        };

        let json = serde_json::to_string(&sysctl).unwrap();
        let deserialized: Sysctl = serde_json::from_str(&json).unwrap();

        assert_eq!(sysctl, deserialized);
    }

    // Constants tests
    #[test]
    fn test_capability_constants() {
        assert_eq!(capability::CAP_NET_ADMIN, "CAP_NET_ADMIN");
        assert_eq!(capability::CAP_SYS_ADMIN, "CAP_SYS_ADMIN");
        assert_eq!(capability::CAP_CHOWN, "CAP_CHOWN");
        assert_eq!(capability::CAP_KILL, "CAP_KILL");
    }

    #[test]
    fn test_seccomp_profile_type_constants() {
        assert_eq!(seccomp_profile_type::UNCONFINED, "Unconfined");
        assert_eq!(seccomp_profile_type::RUNTIME_DEFAULT, "RuntimeDefault");
        assert_eq!(seccomp_profile_type::LOCALHOST, "Localhost");
    }

    #[test]
    fn test_app_armor_profile_type_constants() {
        assert_eq!(app_armor_profile_type::UNCONFINED, "Unconfined");
        assert_eq!(app_armor_profile_type::RUNTIME_DEFAULT, "RuntimeDefault");
        assert_eq!(app_armor_profile_type::LOCALHOST, "Localhost");
    }

    #[test]
    fn test_supplemental_groups_policy_constants() {
        assert_eq!(supplemental_groups_policy::MERGE, "Merge");
        assert_eq!(supplemental_groups_policy::STRICT, "Strict");
    }

    #[test]
    fn test_proc_mount_type_constants() {
        assert_eq!(proc_mount_type::DEFAULT, "Default");
        assert_eq!(proc_mount_type::UNMASKED, "Unmasked");
    }
}
