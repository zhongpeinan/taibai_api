//! Kubernetes Ephemeral Container types
//!
//! This module contains ephemeral container-related types from the Kubernetes core/v1 API.
//! Ephemeral containers are special containers that can be added to running pods for debugging purposes.

use serde::{Deserialize, Serialize};

/// EphemeralContainer is a temporary container that may be added to an existing pod for
/// user-initiated activities such as debugging. Ephemeral containers have no resource
/// guarantees and will not be restarted when they exit.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct EphemeralContainer {
    /// Name of the ephemeral container specified as a DNS_LABEL.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// Container image name.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub image: String,
    /// Image pull policy.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub image_pull_policy: String,
    /// Entrypoint array. Not executed within a shell.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub command: Vec<String>,
    /// Arguments to the entrypoint.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,
    /// Container's working directory.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub working_dir: String,
    /// List of ports to expose from the ephemeral container.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<serde_json::Value>,
    /// List of environment variables to set in the ephemeral container.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub env: Vec<serde_json::Value>,
    /// List of sources to populate environment variables from.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub env_from: Vec<serde_json::Value>,
    /// Resources desired for the ephemeral container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<serde_json::Value>,
    /// Volume mounts for the ephemeral container.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volume_mounts: Vec<serde_json::Value>,
    /// Volume devices for the ephemeral container.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volume_devices: Vec<serde_json::Value>,
    /// Probes for the ephemeral container.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub probes: Vec<serde_json::Value>,
    /// Security context for the ephemeral container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_context: Option<serde_json::Value>,
    /// Whether the ephemeral container's filesystem should be read-only.
    #[serde(default)]
    pub read_only_root_filesystem: bool,
    /// Path at which the ephemeral container is restarted.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub restart_policy: String,
    /// Target container name for the ephemeral container.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub target_container_name: String,
}

/// EphemeralContainerCommon contains fields that are common to both Container and
/// EphemeralContainer types. This is used for code reuse.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct EphemeralContainerCommon {
    /// Name of the container.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// Container image name.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub image: String,
    /// Image pull policy.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub image_pull_policy: String,
    /// Entrypoint array.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub command: Vec<String>,
    /// Arguments to the entrypoint.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,
    /// Container's working directory.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub working_dir: String,
    /// List of ports to expose from the container.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<serde_json::Value>,
    /// List of environment variables to set in the container.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub env: Vec<serde_json::Value>,
    /// List of sources to populate environment variables from.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub env_from: Vec<serde_json::Value>,
    /// Compute Resources required by this container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<serde_json::Value>,
    /// Pod volumes to mount into the container's filesystem.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volume_mounts: Vec<serde_json::Value>,
    /// volumeDevices is the list of block devices to be used by the container.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volume_devices: Vec<serde_json::Value>,
    /// Probes that are run on the container.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub probes: Vec<serde_json::Value>,
    /// Security options the container should be run with.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_context: Option<serde_json::Value>,
    /// Whether this container has a read-only root filesystem.
    #[serde(default)]
    pub read_only_root_filesystem: bool,
}

/// Constants for image pull policy
pub mod image_pull_policy {
    /// Always pull the image
    pub const ALWAYS: &str = "Always";

    /// Pull the image only if not present
    pub const IF_NOT_PRESENT: &str = "IfNotPresent";

    /// Never pull the image
    pub const NEVER: &str = "Never";
}

/// Constants for restart policy
pub mod restart_policy {
    /// Always restart the container
    pub const ALWAYS: &str = "Always";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ephemeral_container_default() {
        let container = EphemeralContainer::default();
        assert!(container.name.is_empty());
        assert!(container.image.is_empty());
        assert!(container.command.is_empty());
        assert!(container.args.is_empty());
        assert!(!container.read_only_root_filesystem);
    }

    #[test]
    fn test_ephemeral_container_with_fields() {
        let container = EphemeralContainer {
            name: "debugger".to_string(),
            image: "debug-image:latest".to_string(),
            image_pull_policy: image_pull_policy::IF_NOT_PRESENT.to_string(),
            command: vec!["sh".to_string()],
            args: vec!["-c".to_string(), "sleep 3600".to_string()],
            working_dir: "/debug".to_string(),
            target_container_name: "app".to_string(),
            ..Default::default()
        };

        assert_eq!(container.name, "debugger");
        assert_eq!(container.image, "debug-image:latest");
        assert_eq!(container.image_pull_policy, "IfNotPresent");
        assert_eq!(container.command.len(), 1);
        assert_eq!(container.target_container_name, "app");
    }

    #[test]
    fn test_ephemeral_container_serialization() {
        let container = EphemeralContainer {
            name: "debug".to_string(),
            image: "busybox".to_string(),
            command: vec!["/bin/sh".to_string()],
            ..Default::default()
        };

        let json = serde_json::to_string(&container).unwrap();
        let deserialized: EphemeralContainer = serde_json::from_str(&json).unwrap();

        assert_eq!(container.name, deserialized.name);
        assert_eq!(container.image, deserialized.image);
    }

    #[test]
    fn test_ephemeral_container_with_read_only_filesystem() {
        let container = EphemeralContainer {
            name: "secure-debug".to_string(),
            image: "debug:latest".to_string(),
            read_only_root_filesystem: true,
            ..Default::default()
        };

        assert!(container.read_only_root_filesystem);

        let json = serde_json::to_string(&container).unwrap();
        let deserialized: EphemeralContainer = serde_json::from_str(&json).unwrap();

        assert!(deserialized.read_only_root_filesystem);
    }

    #[test]
    fn test_ephemeral_container_with_target_container() {
        let container = EphemeralContainer {
            name: "sidecar".to_string(),
            image: "sidecar:latest".to_string(),
            target_container_name: "main-container".to_string(),
            ..Default::default()
        };

        let json = serde_json::to_string(&container).unwrap();
        assert!(json.contains("targetContainerName"));

        let deserialized: EphemeralContainer = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.target_container_name, "main-container");
    }

    #[test]
    fn test_ephemeral_container_common_default() {
        let common = EphemeralContainerCommon::default();
        assert!(common.name.is_empty());
        assert!(common.image.is_empty());
        assert!(common.command.is_empty());
        assert!(!common.read_only_root_filesystem);
    }

    #[test]
    fn test_ephemeral_container_common_with_fields() {
        let common = EphemeralContainerCommon {
            name: "app".to_string(),
            image: "nginx:latest".to_string(),
            working_dir: "/app".to_string(),
            read_only_root_filesystem: true,
            ..Default::default()
        };

        assert_eq!(common.name, "app");
        assert_eq!(common.working_dir, "/app");
        assert!(common.read_only_root_filesystem);
    }

    #[test]
    fn test_ephemeral_container_round_trip() {
        let container = EphemeralContainer {
            name: "debug".to_string(),
            image: "debug:1.0".to_string(),
            image_pull_policy: image_pull_policy::ALWAYS.to_string(),
            command: vec!["sleep".to_string()],
            args: vec!["30".to_string()],
            target_container_name: "web".to_string(),
            ..Default::default()
        };

        let json = serde_json::to_string(&container).unwrap();
        let deserialized: EphemeralContainer = serde_json::from_str(&json).unwrap();

        assert_eq!(container, deserialized);
    }

    #[test]
    fn test_image_pull_policy_constants() {
        assert_eq!(image_pull_policy::ALWAYS, "Always");
        assert_eq!(image_pull_policy::IF_NOT_PRESENT, "IfNotPresent");
        assert_eq!(image_pull_policy::NEVER, "Never");
    }

    #[test]
    fn test_restart_policy_constants() {
        assert_eq!(restart_policy::ALWAYS, "Always");
    }

    #[test]
    fn test_ephemeral_container_skip_empty_fields() {
        let container = EphemeralContainer {
            name: "minimal".to_string(),
            image: "busybox".to_string(),
            ..Default::default()
        };

        let json = serde_json::to_string(&container).unwrap();
        // Empty fields should not be in JSON
        assert!(!json.contains("command"));
        assert!(!json.contains("args"));
        assert!(!json.contains("workingDir"));

        let deserialized: EphemeralContainer = serde_json::from_str(&json).unwrap();
        assert!(deserialized.command.is_empty());
        assert!(deserialized.args.is_empty());
    }

    #[test]
    fn test_ephemeral_container_with_restart_policy() {
        let container = EphemeralContainer {
            name: "debug-pod".to_string(),
            image: "debug:latest".to_string(),
            restart_policy: restart_policy::ALWAYS.to_string(),
            ..Default::default()
        };

        let json = serde_json::to_string(&container).unwrap();
        let deserialized: EphemeralContainer = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.restart_policy, "Always");
    }

    #[test]
    fn test_ephemeral_container_with_resources() {
        let resources_value = serde_json::json!({
            "limits": {
                "cpu": "500m",
                "memory": "512Mi"
            },
            "requests": {
                "cpu": "250m",
                "memory": "256Mi"
            }
        });

        let container = EphemeralContainer {
            name: "debug".to_string(),
            image: "debug:latest".to_string(),
            resources: Some(resources_value),
            ..Default::default()
        };

        let json = serde_json::to_string(&container).unwrap();
        let deserialized: EphemeralContainer = serde_json::from_str(&json).unwrap();

        assert!(deserialized.resources.is_some());
    }
}
