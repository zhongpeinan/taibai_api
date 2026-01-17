//! Kubernetes API Discovery Internal Types
//!
//! This module contains type definitions from k8s.io/kubernetes/pkg/apis/apidiscovery/types.go
//! that are used internally by the Kubernetes API.
//!
//! Source: https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/apidiscovery/types.go

use serde::{Deserialize, Serialize};

/// ResourceScope is an enum defining the different scopes available to a resource.
///
/// Source: https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/apidiscovery/types.go#L115
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum ResourceScope {
    /// Cluster-scoped resources
    #[serde(rename = "Cluster")]
    #[default]
    Cluster,
    /// Namespaced-scoped resources
    #[serde(rename = "Namespaced")]
    Namespaced,
}

pub mod resource_scope {
    pub const CLUSTER: &str = "Cluster";
    pub const NAMESPACED: &str = "Namespaced";
}

/// DiscoveryFreshness is an enum defining whether the Discovery document published by an apiservice is up to date (fresh).
///
/// Source: https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/apidiscovery/types.go#L123
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum DiscoveryFreshness {
    /// The discovery document was recently refreshed
    #[serde(rename = "Current")]
    #[default]
    Current,
    /// The discovery document could not be retrieved and may be significantly out of date
    #[serde(rename = "Stale")]
    Stale,
}

pub mod discovery_freshness {
    pub const CURRENT: &str = "Current";
    pub const STALE: &str = "Stale";
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ResourceScope tests
    #[test]
    fn test_resource_scope_serialize() {
        assert_eq!(
            serde_json::to_string(&ResourceScope::Cluster).unwrap(),
            r#""Cluster""#
        );
    }

    #[test]
    fn test_resource_scope_deserialize() {
        assert_eq!(
            serde_json::from_str::<ResourceScope>(r#""Cluster""#).unwrap(),
            ResourceScope::Cluster
        );
    }

    #[test]
    fn test_resource_scope_constants() {
        assert_eq!(resource_scope::CLUSTER, "Cluster");
        assert_eq!(resource_scope::NAMESPACED, "Namespaced");
    }

    // DiscoveryFreshness tests
    #[test]
    fn test_discovery_freshness_serialize() {
        assert_eq!(
            serde_json::to_string(&DiscoveryFreshness::Current).unwrap(),
            r#""Current""#
        );
    }

    #[test]
    fn test_discovery_freshness_deserialize() {
        assert_eq!(
            serde_json::from_str::<DiscoveryFreshness>(r#""Current""#).unwrap(),
            DiscoveryFreshness::Current
        );
    }

    #[test]
    fn test_discovery_freshness_constants() {
        assert_eq!(discovery_freshness::CURRENT, "Current");
        assert_eq!(discovery_freshness::STALE, "Stale");
    }

    #[test]
    fn test_resource_scope_namespaced() {
        assert_eq!(
            serde_json::to_string(&ResourceScope::Namespaced).unwrap(),
            r#""Namespaced""#
        );
    }

    #[test]
    fn test_discovery_freshness_stale() {
        assert_eq!(
            serde_json::to_string(&DiscoveryFreshness::Stale).unwrap(),
            r#""Stale""#
        );
    }
}
