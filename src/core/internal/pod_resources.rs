//! Pod Resource Claim types from the Kubernetes Core API
//!
//! This module contains types for dynamic resource allocation in pods.

use serde::{Deserialize, Serialize};

/// PodResourceClaim references exactly one ResourceClaim.
///
/// Corresponds to [Kubernetes PodResourceClaim](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3891)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodResourceClaim {
    /// Name uniquely identifying this claim within the pod.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    /// ResourceClaimName is the name of a ResourceClaim in the same namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_claim_name: Option<String>,

    /// ResourceClaimTemplateName is the name of a ResourceClaimTemplate in the same namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_claim_template_name: Option<String>,
}

/// PodResourceClaimStatus is stored in the PodStatus for each PodResourceClaim.
///
/// Corresponds to [Kubernetes PodResourceClaimStatus](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3924)
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

/// ContainerResizePolicy represents resource resize policy for the container.
///
/// Corresponds to [Kubernetes ContainerResizePolicy](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2513)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContainerResizePolicy {
    /// ResourceName is the name of the resource.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub resource_name: String,

    /// RestartPolicy is the restart policy for containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restart_policy: Option<String>,
}

/// ContainerUser represents user identity information.
///
/// Corresponds to [Kubernetes ContainerUser](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3078)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContainerUser {
    /// Linux container user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linux: Option<LinuxContainerUser>,
}

/// LinuxContainerUser defines user identity information for Linux containers.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct LinuxContainerUser {
    /// The UID to run the entrypoint of the container process.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<i64>,

    /// The GID to run the entrypoint of the container process.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gid: Option<i64>,
}

#[cfg(test)]
mod tests {
}
