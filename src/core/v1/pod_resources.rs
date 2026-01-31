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

    /// ResourceClaimName is the name of a ResourceClaim in the same namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_claim_name: Option<String>,

    /// ResourceClaimTemplateName is the name of a ResourceClaimTemplate in the same namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_claim_template_name: Option<String>,
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

    /// ResourceClaimName is the name of the ResourceClaim that was bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_claim_name: Option<String>,
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

/// ContainerUser represents user identity information.
///
/// Corresponds to [Kubernetes ContainerUser](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3447)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContainerUser {
    /// Linux holds user identity information for Linux containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linux: Option<LinuxContainerUser>,
}

/// LinuxContainerUser represents user identity information in Linux containers.
///
/// Corresponds to [Kubernetes LinuxContainerUser](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3456)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct LinuxContainerUser {
    /// UID is the primary uid initially attached to the first process in the container.
    #[serde(default)]
    pub uid: i64,
    /// GID is the primary gid initially attached to the first process in the container.
    #[serde(default)]
    pub gid: i64,
    /// SupplementalGroups are the supplemental groups initially attached to the first process.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supplemental_groups: Vec<i64>,
}

#[cfg(test)]
mod tests {}
