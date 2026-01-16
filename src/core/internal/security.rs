//! Security Context types from the Kubernetes Core API
//!
//! This module contains security-related types for configuring containers and pods.
//! These types control the security context, capabilities, and access controls.
//!
//! Source: k8s-pkg/apis/core/types.go

use crate::core::internal::{
    AppArmorProfileType, PodFSGroupChangePolicy, PodSELinuxChangePolicy, ProcMountType,
    SeccompProfileType, SupplementalGroupsPolicy,
};
use serde::{Deserialize, Serialize};

// ============================================================================
// Seccomp and AppArmor Profiles
// ============================================================================

/// SeccompProfile defines a pod/container's seccomp profile settings.
///
/// Corresponds to [Kubernetes SeccompProfile](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4214)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SeccompProfile {
    /// Type indicates which kind of seccomp profile will be applied.
    pub r#type: SeccompProfileType,
    /// Load a profile defined in static file on the node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub localhost_profile: Option<String>,
}

/// AppArmorProfile defines a pod or container's AppArmor settings.
///
/// Corresponds to [Kubernetes AppArmorProfile](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4238)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AppArmorProfile {
    /// Type indicates which kind of AppArmor profile will be applied.
    pub r#type: AppArmorProfileType,
    /// localhostProfile indicates a profile loaded on the node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub localhost_profile: Option<String>,
}

// ============================================================================
// Pod Security Context
// ============================================================================

/// PodSecurityContext holds pod-level security attributes and common container settings.
///
/// Some fields are also present in container.securityContext.
/// Field values of container.securityContext take precedence over field values of PodSecurityContext.
///
/// Corresponds to [Kubernetes PodSecurityContext](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4050)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodSecurityContext {
    /// Use the host's network namespace.
    /// If this option is set, the ports that will be used must be specified.
    /// Default to false.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub host_network: bool,

    /// Use the host's pid namespace.
    /// Default to false.
    /// Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub host_pid: bool,

    /// Use the host's ipc namespace.
    /// Default to false.
    /// Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub host_ipc: bool,

    /// Share a single process namespace between all of the containers in a pod.
    /// When this is set containers will be able to view and signal processes from other containers
    /// in the same pod, and the first process in each container will not be assigned PID 1.
    /// HostPID and ShareProcessNamespace cannot both be set.
    /// Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub share_process_namespace: Option<bool>,

    /// Use the host's user namespace.
    /// Default to true.
    /// If set to true or not present, the pod will be run in the host user namespace.
    /// When set to false, a new user namespace is created for the pod.
    /// Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_users: Option<bool>,

    /// The SELinux context to be applied to all containers.
    /// If unspecified, the container runtime will allocate a random SELinux context for each container.
    /// May also be set in SecurityContext. If set in both SecurityContext and PodSecurityContext,
    /// the value specified in SecurityContext takes precedence for that container.
    /// Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selinux_options: Option<SELinuxOptions>,

    /// The Windows specific settings applied to all containers.
    /// If unspecified, the options within a container's SecurityContext will be used.
    /// Note that this field cannot be set when spec.os.name is linux.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub windows_options: Option<WindowsSecurityContextOptions>,

    /// The UID to run the entrypoint of the container process.
    /// Defaults to user specified in image metadata if unspecified.
    /// Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_as_user: Option<i64>,

    /// The GID to run the entrypoint of the container process.
    /// Uses runtime default if unset.
    /// Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_as_group: Option<i64>,

    /// Indicates that the container must run as a non-root user.
    /// If true, the Kubelet will validate the image at runtime to ensure that it
    /// does not run as UID 0 (root) and fail to start the container if it does.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_as_non_root: Option<bool>,

    /// A list of groups applied to the first process run in each container.
    /// Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supplemental_groups: Vec<i64>,

    /// Defines how supplemental groups of the first container processes are calculated.
    /// Valid values are "Merge" and "Strict". If not specified, "Merge" is used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supplemental_groups_policy: Option<SupplementalGroupsPolicy>,

    /// A special supplemental group that applies to all containers in a pod.
    /// Some volume types allow the Kubelet to change the ownership of that volume
    /// to be owned by the pod.
    /// Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fs_group: Option<i64>,

    /// fsGroupChangePolicy defines behavior of changing ownership and permission of the volume
    /// before being exposed inside Pod.
    /// Valid values are "OnRootMismatch" and "Always". If not specified, "Always" is used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fs_group_change_policy: Option<PodFSGroupChangePolicy>,

    /// Sysctls hold a list of namespaced sysctls used for the pod.
    /// Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sysctls: Vec<Sysctl>,

    /// The seccomp options to use by the containers in this pod.
    /// Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seccomp_profile: Option<SeccompProfile>,

    /// appArmorProfile is the AppArmor options to use by the containers in this pod.
    /// Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_armor_profile: Option<AppArmorProfile>,

    /// seLinuxChangePolicy defines how the container's SELinux label is applied to all volumes used by the Pod.
    /// Valid values are "MountOption" and "Recursive".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selinux_change_policy: Option<PodSELinuxChangePolicy>,
}

// ============================================================================
// Container Security Context
// ============================================================================

/// SecurityContext holds security configuration that will be applied to a container.
///
/// Some fields are present in both SecurityContext and PodSecurityContext.
/// When both are set, the values in SecurityContext take precedence.
///
/// Corresponds to [Kubernetes SecurityContext](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L6730)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SecurityContext {
    /// The capabilities to add/drop when running containers.
    /// Defaults to the default set of capabilities granted by the container runtime.
    /// Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Capabilities>,

    /// Run container in privileged mode.
    /// Processes in privileged containers are essentially equivalent to root on the host.
    /// Defaults to false.
    /// Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,

    /// The SELinux context to be applied to the container.
    /// Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selinux_options: Option<SELinuxOptions>,

    /// The Windows specific settings applied to all containers.
    /// Note that this field cannot be set when spec.os.name is linux.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub windows_options: Option<WindowsSecurityContextOptions>,

    /// The UID to run the entrypoint of the container process.
    /// Defaults to user specified in image metadata if unspecified.
    /// Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_as_user: Option<i64>,

    /// The GID to run the entrypoint of the container process.
    /// Uses runtime default if unset.
    /// Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_as_group: Option<i64>,

    /// Indicates that the container must run as a non-root user.
    /// If true, the Kubelet will validate the image at runtime to ensure that it
    /// does not run as UID 0 (root) and fail to start the container if it does.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_as_non_root: Option<bool>,

    /// The read-only root filesystem allows you to restrict the locations that an application can write
    /// files to, ensuring the persistent data can only be written to mounts.
    /// Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_only_root_filesystem: Option<bool>,

    /// AllowPrivilegeEscalation controls whether a process can gain more
    /// privileges than its parent process.
    /// Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_privilege_escalation: Option<bool>,

    /// ProcMount denotes the type of proc mount to use for the containers.
    /// Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proc_mount: Option<ProcMountType>,

    /// The seccomp options to use by this container.
    /// Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seccomp_profile: Option<SeccompProfile>,

    /// appArmorProfile is the AppArmor options to use by this container.
    /// Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_armor_profile: Option<AppArmorProfile>,
}

// ============================================================================
// Supporting Types
// ============================================================================

/// SELinuxOptions are the labels to be applied to the container.
///
/// Corresponds to [Kubernetes SELinuxOptions](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L6825)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SELinuxOptions {
    /// SELinux user label.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub user: String,

    /// SELinux role label.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub role: String,

    /// SELinux type label.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub level: String,

    /// SELinux level label.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub r#type: String,
}

/// WindowsSecurityContextOptions contain Windows-specific options and credentials.
///
/// Corresponds to [Kubernetes WindowsSecurityContextOptions](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L6841)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct WindowsSecurityContextOptions {
    /// GMSACredentialSpecName is the name of the GMSA credential spec to use.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gmsa_credential_spec_name: Option<String>,

    /// GMSACredentialSpec is where the GMSA admission webhook inlines
    /// the contents of the GMSA credential spec named by the GMSACredentialSpecName field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gmsa_credential_spec: Option<String>,

    /// The UserName in Windows to run the entrypoint of the container process.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_as_user_name: Option<String>,

    /// HostProcess determines if a container should be run as a 'Host Process' container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_process: Option<bool>,
}

/// Capabilities represent POSIX capabilities that can be added or removed to a running container.
///
/// Corresponds to [Kubernetes Capabilities](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2549)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Capabilities {
    /// Added capabilities.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub add: Vec<Capability>,

    /// Removed capabilities.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub drop: Vec<Capability>,
}

/// Capability represents a POSIX capability.
///
/// Corresponds to [Kubernetes Capability](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2546)
pub type Capability = String;

/// Sysctl defines a kernel parameter to be set.
///
/// Corresponds to [Kubernetes Sysctl](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3992)
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

    #[test]
    fn test_pod_security_context_default() {
        let ctx = PodSecurityContext::default();
        assert!(!ctx.host_network);
        assert!(!ctx.host_pid);
        assert!(!ctx.host_ipc);
        assert!(ctx.supplemental_groups.is_empty());
        assert!(ctx.sysctls.is_empty());
    }

    #[test]
    fn test_pod_security_context_with_fields() {
        let ctx = PodSecurityContext {
            host_network: true,
            host_pid: false,
            run_as_user: Some(1000),
            supplemental_groups: vec![1001, 1002],
            ..Default::default()
        };

        assert!(ctx.host_network);
        assert!(!ctx.host_pid);
        assert_eq!(ctx.run_as_user, Some(1000));
        assert_eq!(ctx.supplemental_groups.len(), 2);
    }

    #[test]
    fn test_pod_security_context_serialize() {
        let ctx = PodSecurityContext {
            host_network: true,
            run_as_user: Some(1000),
            ..Default::default()
        };

        let json = serde_json::to_string(&ctx).unwrap();
        assert!(json.contains("\"hostNetwork\":true"));
        assert!(json.contains("\"runAsUser\":1000"));
    }

    #[test]
    fn test_pod_security_context_deserialize() {
        let json = r#"{"hostNetwork":true,"runAsUser":1000}"#;
        let ctx: PodSecurityContext = serde_json::from_str(json).unwrap();

        assert!(ctx.host_network);
        assert_eq!(ctx.run_as_user, Some(1000));
    }

    #[test]
    fn test_pod_security_context_round_trip() {
        let original = PodSecurityContext {
            host_pid: true,
            run_as_group: Some(1000),
            ..Default::default()
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: PodSecurityContext = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_security_context_default() {
        let ctx = SecurityContext::default();
        assert!(ctx.capabilities.is_none());
        assert!(ctx.privileged.is_none());
        assert!(ctx.run_as_user.is_none());
    }

    #[test]
    fn test_security_context_with_fields() {
        let ctx = SecurityContext {
            privileged: Some(true),
            run_as_user: Some(0),
            run_as_non_root: Some(false),
            ..Default::default()
        };

        assert_eq!(ctx.privileged, Some(true));
        assert_eq!(ctx.run_as_user, Some(0));
        assert_eq!(ctx.run_as_non_root, Some(false));
    }

    #[test]
    fn test_security_context_serialize() {
        let ctx = SecurityContext {
            privileged: Some(true),
            ..Default::default()
        };

        let json = serde_json::to_string(&ctx).unwrap();
        assert!(json.contains("\"privileged\":true"));
    }

    #[test]
    fn test_security_context_deserialize() {
        let json = r#"{"privileged":false,"runAsUser":1000}"#;
        let ctx: SecurityContext = serde_json::from_str(json).unwrap();

        assert_eq!(ctx.privileged, Some(false));
        assert_eq!(ctx.run_as_user, Some(1000));
    }

    #[test]
    fn test_security_context_round_trip() {
        let original = SecurityContext {
            privileged: Some(false),
            run_as_non_root: Some(true),
            ..Default::default()
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: SecurityContext = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_selinux_options_default() {
        let opts = SELinuxOptions::default();
        assert!(opts.user.is_empty());
        assert!(opts.role.is_empty());
        assert!(opts.level.is_empty());
        assert!(opts.r#type.is_empty());
    }

    #[test]
    fn test_selinux_options_with_fields() {
        let opts = SELinuxOptions {
            user: "user_u".to_string(),
            role: "role_r".to_string(),
            r#type: "type_t".to_string(),
            level: "s0:c100,c200".to_string(),
        };

        assert_eq!(opts.user, "user_u");
        assert_eq!(opts.role, "role_r");
        assert_eq!(opts.r#type, "type_t");
        assert_eq!(opts.level, "s0:c100,c200");
    }

    #[test]
    fn test_selinux_options_serialize() {
        let opts = SELinuxOptions {
            user: "user_u".to_string(),
            role: "role_r".to_string(),
            ..Default::default()
        };

        let json = serde_json::to_string(&opts).unwrap();
        assert!(json.contains("\"user\":\"user_u\""));
        assert!(json.contains("\"role\":\"role_r\""));
    }

    #[test]
    fn test_selinux_options_round_trip() {
        let original = SELinuxOptions {
            user: "user_u".to_string(),
            role: "role_r".to_string(),
            r#type: "type_t".to_string(),
            level: "level_l".to_string(),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: SELinuxOptions = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_windows_security_context_options_default() {
        let opts = WindowsSecurityContextOptions::default();
        assert!(opts.gmsa_credential_spec_name.is_none());
        assert!(opts.gmsa_credential_spec.is_none());
        assert!(opts.run_as_user_name.is_none());
        assert!(opts.host_process.is_none());
    }

    #[test]
    fn test_windows_security_context_options_with_fields() {
        let opts = WindowsSecurityContextOptions {
            gmsa_credential_spec_name: Some("spec1".to_string()),
            run_as_user_name: Some("ContainerUser".to_string()),
            host_process: Some(true),
            ..Default::default()
        };

        assert_eq!(opts.gmsa_credential_spec_name, Some("spec1".to_string()));
        assert_eq!(opts.run_as_user_name, Some("ContainerUser".to_string()));
        assert_eq!(opts.host_process, Some(true));
    }

    #[test]
    fn test_windows_security_context_options_round_trip() {
        let original = WindowsSecurityContextOptions {
            gmsa_credential_spec: Some("inline-spec".to_string()),
            run_as_user_name: Some("user".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: WindowsSecurityContextOptions = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_capabilities_default() {
        let caps = Capabilities::default();
        assert!(caps.add.is_empty());
        assert!(caps.drop.is_empty());
    }

    #[test]
    fn test_capabilities_with_fields() {
        let caps = Capabilities {
            add: vec!["NET_ADMIN".to_string(), "SYS_TIME".to_string()],
            drop: vec!["KILL".to_string()],
        };

        assert_eq!(caps.add.len(), 2);
        assert_eq!(caps.drop.len(), 1);
    }

    #[test]
    fn test_capabilities_serialize() {
        let caps = Capabilities {
            add: vec!["NET_ADMIN".to_string()],
            drop: vec![],
        };

        let json = serde_json::to_string(&caps).unwrap();
        assert!(json.contains("\"add\":[\"NET_ADMIN\"]"));
        assert!(!json.contains("\"drop\""));
    }

    #[test]
    fn test_capabilities_round_trip() {
        let original = Capabilities {
            add: vec!["SYS_ADMIN".to_string()],
            drop: vec!["KILL".to_string()],
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Capabilities = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
    }

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
        assert!(json.contains("\"name\":\"kernel.msgmax\""));
        assert!(json.contains("\"value\":\"8192\""));
    }

    #[test]
    fn test_sysctl_deserialize() {
        let json = r#"{"name":"kernel.shmmax","value":"68719476736"}"#;
        let sysctl: Sysctl = serde_json::from_str(json).unwrap();

        assert_eq!(sysctl.name, "kernel.shmmax");
        assert_eq!(sysctl.value, "68719476736");
    }

    #[test]
    fn test_sysctl_round_trip() {
        let original = Sysctl {
            name: "kernel.sem".to_string(),
            value: "250 32000 100 128".to_string(),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Sysctl = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
    }
}
