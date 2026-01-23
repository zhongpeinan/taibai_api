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
}
