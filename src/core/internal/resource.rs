//! Resource-related types from the Kubernetes Core API
//!
//! This module contains types for managing compute resources in Kubernetes.

use crate::common::util::Quantity;
use crate::core::internal::Protocol;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// ResourceList maps a ResourceName to a Quantity.
///
/// Corresponds to [Kubernetes ResourceList](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5779)
pub type ResourceList = BTreeMap<String, Quantity>;

/// ResourceRequirements describes the compute resource requirements.
///
/// Corresponds to [Kubernetes ResourceRequirements](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2559)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceRequirements {
    /// Limits describes the maximum amount of compute resources allowed.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub limits: ResourceList,
    /// Requests describes the minimum amount of compute resources required.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub requests: ResourceList,
}

/// PortStatus represents the status of a service port.
///
/// Corresponds to [Kubernetes PortStatus](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7052)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PortStatus {
    /// Port is the port number of the service port.
    pub port: i32,
    /// Protocol is the protocol of the service port.
    pub protocol: Protocol,
    /// Error is to record the problem with the service port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_requirements_default() {
        let req = ResourceRequirements::default();
        assert!(req.limits.is_empty());
        assert!(req.requests.is_empty());
    }

    #[test]
    fn test_resource_requirements_with_limits() {
        let mut limits = BTreeMap::new();
        limits.insert("cpu".to_string(), Quantity::from_str("500m"));
        limits.insert("memory".to_string(), Quantity::from_str("128Mi"));

        let req = ResourceRequirements {
            limits: limits.clone(),
            requests: BTreeMap::new(),
        };

        assert_eq!(req.limits.len(), 2);
        assert_eq!(req.limits.get("cpu").unwrap().as_str(), "500m");
    }

    #[test]
    fn test_resource_requirements_serialize() {
        let mut requests = BTreeMap::new();
        requests.insert("cpu".to_string(), Quantity::from_str("250m"));

        let req = ResourceRequirements {
            limits: BTreeMap::new(),
            requests,
        };

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"requests\""));
        assert!(json.contains("\"cpu\""));
        assert!(json.contains("\"250m\""));
    }

    #[test]
    fn test_resource_requirements_deserialize() {
        let json = r#"{"limits":{"cpu":"500m"},"requests":{"memory":"128Mi"}}"#;
        let req: ResourceRequirements = serde_json::from_str(json).unwrap();

        assert_eq!(req.limits.get("cpu").unwrap().as_str(), "500m");
        assert_eq!(req.requests.get("memory").unwrap().as_str(), "128Mi");
    }

    #[test]
    fn test_resource_requirements_round_trip() {
        let mut limits = BTreeMap::new();
        limits.insert("cpu".to_string(), Quantity::from_str("1000m"));
        limits.insert("memory".to_string(), Quantity::from_str("512Mi"));

        let original = ResourceRequirements {
            limits: limits.clone(),
            requests: BTreeMap::new(),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: ResourceRequirements = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_port_status_default() {
        let status = PortStatus::default();
        assert_eq!(status.port, 0);
    }

    #[test]
    fn test_port_status_with_fields() {
        let status = PortStatus {
            port: 8080,
            protocol: Protocol::Tcp,
            error: Some("connection refused".to_string()),
        };

        assert_eq!(status.port, 8080);
        assert_eq!(status.protocol, Protocol::Tcp);
        assert_eq!(status.error, Some("connection refused".to_string()));
    }

    #[test]
    fn test_port_status_serialize() {
        let status = PortStatus {
            port: 443,
            protocol: Protocol::Tcp,
            error: None,
        };

        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains("\"port\":443"));
        assert!(json.contains("\"protocol\":\"TCP\""));
        // error should be omitted when None
        assert!(!json.contains("\"error\""));
    }

    #[test]
    fn test_port_status_deserialize() {
        let json = r#"{"port":80,"protocol":"TCP","error":"timeout"}"#;
        let status: PortStatus = serde_json::from_str(json).unwrap();

        assert_eq!(status.port, 80);
        assert_eq!(status.protocol, Protocol::Tcp);
        assert_eq!(status.error, Some("timeout".to_string()));
    }

    #[test]
    fn test_port_status_round_trip() {
        let original = PortStatus {
            port: 9090,
            protocol: Protocol::Udp,
            error: Some("network unreachable".to_string()),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: PortStatus = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_resource_list_type() {
        let mut list: ResourceList = BTreeMap::new();
        list.insert("cpu".to_string(), Quantity::from_str("100m"));
        list.insert("memory".to_string(), Quantity::from_str("64Mi"));

        assert_eq!(list.len(), 2);
        assert_eq!(list.get("cpu").unwrap().as_str(), "100m");
    }
}
