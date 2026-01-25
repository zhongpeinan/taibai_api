//! Pod resource claim types from Kubernetes Core v1 API
//!
//! This module contains types for pod resource claims and container
//! resource management.

use serde::{Deserialize, Serialize};

// ============================================================================
// Pod Resource Claims
// ============================================================================

/// PodResourceClaim references a ResourceClaim through a claim name.
///
/// Corresponds to [Kubernetes PodResourceClaim](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7650)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodResourceClaim {
    /// Name uniquely identifies this claim within the pod.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
}

/// PodResourceClaimStatus contains the status for a resource claim.
///
/// Corresponds to [Kubernetes PodResourceClaimStatus](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7658)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodResourceClaimStatus {
    /// Name uniquely identifies this claim within the pod.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    /// Indicates whether the resource is available for use.
    #[serde(default)]
    pub res: bool,
}

// ============================================================================
// Container Resize Policy
// ============================================================================

/// ContainerResizePolicy represents the resize policy for a container.
///
/// Corresponds to [Kubernetes ContainerResizePolicy](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7667)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContainerResizePolicy {
    /// Name of the container to apply the resize policy to.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub container_name: String,

    /// The policy for resizing the container.
    #[serde(default)]
    pub policy: String,
}

// ============================================================================
// Container User
// ============================================================================

/// ContainerUser represents user information for running containers.
///
/// Corresponds to [Kubernetes ContainerUser](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7676)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContainerUser {
    /// The user ID.
    #[serde(default)]
    pub uid: i64,

    /// The group ID.
    #[serde(default)]
    pub gid: i64,

    /// The username.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub username: String,

    /// Additional group IDs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<i64>,
}

/// LinuxContainerUser represents Linux-specific user information for running containers.
///
/// Corresponds to [Kubernetes LinuxContainerUser](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7689)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct LinuxContainerUser {
    /// The user ID.
    #[serde(default)]
    pub uid: i64,

    /// The group ID.
    #[serde(default)]
    pub gid: i64,

    /// Additional group IDs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supplemental_groups: Vec<i64>,
}

#[cfg(test)]
mod tests {}
