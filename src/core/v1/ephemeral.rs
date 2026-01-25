//! Kubernetes Ephemeral Container types
//!
//! This module contains ephemeral container-related types from the Kubernetes core/v1 API.
//! Ephemeral containers are special containers that can be added to running pods for debugging purposes.

use crate::common::ApplyDefault;
use crate::core::v1::ContainerPort;
use crate::core::v1::env::{EnvFromSource, EnvVar};
use crate::core::v1::probe::{Lifecycle, Probe};
use crate::core::v1::resource::ResourceRequirements;
use crate::core::v1::security::SecurityContext;
use crate::core::v1::volume::{VolumeDevice, VolumeMount};
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
    pub ports: Vec<ContainerPort>,
    /// List of environment variables to set in the ephemeral container.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub env: Vec<EnvVar>,
    /// List of sources to populate environment variables from.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub env_from: Vec<EnvFromSource>,
    /// Resources desired for the ephemeral container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<ResourceRequirements>,
    /// Volume mounts for the ephemeral container.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volume_mounts: Vec<VolumeMount>,
    /// Volume devices for the ephemeral container.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volume_devices: Vec<VolumeDevice>,
    /// Liveness probe for the ephemeral container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub liveness_probe: Option<Probe>,
    /// Readiness probe for the ephemeral container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readiness_probe: Option<Probe>,
    /// Startup probe for the ephemeral container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub startup_probe: Option<Probe>,
    /// Lifecycle hooks for the ephemeral container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<Lifecycle>,
    /// Security context for the ephemeral container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_context: Option<SecurityContext>,
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
    pub ports: Vec<ContainerPort>,
    /// List of environment variables to set in the container.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub env: Vec<EnvVar>,
    /// List of sources to populate environment variables from.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub env_from: Vec<EnvFromSource>,
    /// Compute Resources required by this container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<ResourceRequirements>,
    /// Pod volumes to mount into the container's filesystem.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volume_mounts: Vec<VolumeMount>,
    /// volumeDevices is the list of block devices to be used by the container.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volume_devices: Vec<VolumeDevice>,
    /// Liveness probe for the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub liveness_probe: Option<Probe>,
    /// Readiness probe for the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readiness_probe: Option<Probe>,
    /// Startup probe for the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub startup_probe: Option<Probe>,
    /// Lifecycle hooks for the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<Lifecycle>,
    /// Security options the container should be run with.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_context: Option<SecurityContext>,
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

// ----------------------------------------------------------------------------
// ApplyDefaults Implementation
// ----------------------------------------------------------------------------

impl ApplyDefault for EphemeralContainer {
    fn apply_default(&mut self) {
        // Set default image pull policy based on image tag if not specified
        if self.image_pull_policy.is_empty() {
            if !self.image.is_empty() {
                // Check if the image tag is "latest" or missing (implies latest)
                let is_latest = if let Some(tag_start) = self.image.rfind(':') {
                    let tag = &self.image[tag_start + 1..];
                    tag == "latest" || tag.is_empty()
                } else {
                    // No tag specified, defaults to latest
                    true
                };

                self.image_pull_policy = if is_latest {
                    "Always".to_string()
                } else {
                    "IfNotPresent".to_string()
                };
            } else {
                // No image specified, default to IfNotPresent
                self.image_pull_policy = "IfNotPresent".to_string();
            }
        }

        // Apply defaults to probes
        if let Some(ref mut probe) = self.liveness_probe {
            probe.apply_default();
        }
        if let Some(ref mut probe) = self.readiness_probe {
            probe.apply_default();
        }
        if let Some(ref mut probe) = self.startup_probe {
            probe.apply_default();
        }
    }
}

#[cfg(test)]
mod tests {}
