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
    use super::*;

    // PodResourceClaim tests
    #[test]
    fn test_pod_resource_claim_default() {
        let claim = PodResourceClaim::default();
        assert!(claim.name.is_empty());
        assert!(claim.resource_claim_name.is_none());
        assert!(claim.resource_claim_template_name.is_none());
    }

    #[test]
    fn test_pod_resource_claim_serialize() {
        let claim = PodResourceClaim {
            name: "my-claim".to_string(),
            resource_claim_name: Some("claim-1".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_string(&claim).unwrap();
        assert!(json.contains("\"name\":\"my-claim\""));
        assert!(json.contains("\"resourceClaimName\":\"claim-1\""));
    }

    #[test]
    fn test_pod_resource_claim_deserialize() {
        let json = r#"{"name":"test-claim","resourceClaimName":"claim-2"}"#;
        let claim: PodResourceClaim = serde_json::from_str(json).unwrap();
        assert_eq!(claim.name, "test-claim");
        assert_eq!(claim.resource_claim_name, Some("claim-2".to_string()));
        assert!(claim.resource_claim_template_name.is_none());
    }

    #[test]
    fn test_pod_resource_claim_round_trip() {
        let original = PodResourceClaim {
            name: "test-claim".to_string(),
            resource_claim_template_name: Some("template-1".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: PodResourceClaim = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    // PodResourceClaimStatus tests
    #[test]
    fn test_pod_resource_claim_status_default() {
        let status = PodResourceClaimStatus::default();
        assert!(status.name.is_empty());
        assert!(status.resource_claim_name.is_none());
    }

    #[test]
    fn test_pod_resource_claim_status_serialize() {
        let status = PodResourceClaimStatus {
            name: "my-claim".to_string(),
            resource_claim_name: Some("bound-claim".to_string()),
        };
        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains("\"name\":\"my-claim\""));
        assert!(json.contains("\"resourceClaimName\":\"bound-claim\""));
    }

    #[test]
    fn test_pod_resource_claim_status_round_trip() {
        let original = PodResourceClaimStatus {
            name: "status-claim".to_string(),
            resource_claim_name: Some("resource-1".to_string()),
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: PodResourceClaimStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    // ContainerResizePolicy tests
    #[test]
    fn test_container_resize_policy_default() {
        let policy = ContainerResizePolicy::default();
        assert!(policy.resource_name.is_empty());
        assert!(policy.restart_policy.is_none());
    }

    #[test]
    fn test_container_resize_policy_serialize() {
        let policy = ContainerResizePolicy {
            resource_name: "cpu".to_string(),
            restart_policy: Some("RestartContainer".to_string()),
        };
        let json = serde_json::to_string(&policy).unwrap();
        assert!(json.contains("\"resourceName\":\"cpu\""));
        assert!(json.contains("\"restartPolicy\":\"RestartContainer\""));
    }

    #[test]
    fn test_container_resize_policy_round_trip() {
        let original = ContainerResizePolicy {
            resource_name: "memory".to_string(),
            restart_policy: Some("NoRestart".to_string()),
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: ContainerResizePolicy = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    // ContainerUser tests
    #[test]
    fn test_container_user_default() {
        let user = ContainerUser::default();
        assert!(user.linux.is_none());
    }

    #[test]
    fn test_container_user_with_linux() {
        let linux = LinuxContainerUser {
            uid: Some(1000),
            gid: Some(1000),
        };
        let user = ContainerUser { linux: Some(linux) };
        assert_eq!(user.linux.as_ref().unwrap().uid, Some(1000));
        assert_eq!(user.linux.as_ref().unwrap().gid, Some(1000));
    }

    #[test]
    fn test_container_user_serialize() {
        let user = ContainerUser {
            linux: Some(LinuxContainerUser {
                uid: Some(1001),
                gid: Some(1001),
            }),
        };
        let json = serde_json::to_string(&user).unwrap();
        assert!(json.contains("\"linux\""));
        assert!(json.contains("\"uid\":1001"));
        assert!(json.contains("\"gid\":1001"));
    }

    #[test]
    fn test_container_user_round_trip() {
        let original = ContainerUser {
            linux: Some(LinuxContainerUser {
                uid: Some(0),
                gid: Some(0),
            }),
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: ContainerUser = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    // LinuxContainerUser tests
    #[test]
    fn test_linux_container_user_default() {
        let user = LinuxContainerUser::default();
        assert!(user.uid.is_none());
        assert!(user.gid.is_none());
    }

    #[test]
    fn test_linux_container_user_serialize() {
        let user = LinuxContainerUser {
            uid: Some(1000),
            gid: Some(1000),
        };
        let json = serde_json::to_string(&user).unwrap();
        assert!(json.contains("\"uid\":1000"));
        assert!(json.contains("\"gid\":1000"));
    }

    #[test]
    fn test_linux_container_user_round_trip() {
        let original = LinuxContainerUser {
            uid: Some(65534),
            gid: Some(65534),
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: LinuxContainerUser = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }
}
