//! Kubernetes Topology Spread Constraint types
//!
//! This module contains topology spread constraint types from the Kubernetes core/v1 API.
//! Topology spread constraints control how Pods are spread across a cluster.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// TopologySpreadConstraint specifies how to spread matching pods among the given topology.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TopologySpreadConstraint {
    /// MaxSkew describes the degree to which pods may be unevenly distributed.
    #[serde(default)]
    pub max_skew: i32,
    /// TopologyKey is the key of node labels.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub topology_key: String,
    /// WhenUnsatisfiable indicates how to deal with a pod if it does not satisfy the spread constraint.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub when_unsatisfiable: String,
    /// LabelSelector is used to find matching pods.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub label_selector: HashMap<String, String>,
    /// MinDomains indicates a minimum number of eligible domains.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_domains: Option<i32>,
    /// NodeAffinityPolicy indicates how we will treat Pod's node affinity.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub node_affinity_policy: String,
    /// MatchLabelKeys is a set of pod label keys to select pods.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub match_label_keys: Vec<String>,
}

/// Constants for WhenUnsatisfiable values
pub mod when_unsatisfiable {
    /// Do not schedule the pod
    pub const DO_NOT_SCHEDULE: &str = "DoNotSchedule";

    /// Schedule the pod anyway
    pub const SCHEDULE_ANYWAY: &str = "ScheduleAnyway";
}

/// Constants for NodeAffinityPolicy values
pub mod node_affinity_policy {
    /// Ignore the pod's node affinity
    pub const IGNORE: &str = "Ignore";

    /// Honor the pod's node affinity
    pub const HONOR: &str = "Honor";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topology_spread_constraint_default() {
        let constraint = TopologySpreadConstraint::default();
        assert_eq!(constraint.max_skew, 0);
        assert!(constraint.topology_key.is_empty());
        assert!(constraint.when_unsatisfiable.is_empty());
        assert!(constraint.label_selector.is_empty());
        assert!(constraint.min_domains.is_none());
        assert!(constraint.match_label_keys.is_empty());
    }

    #[test]
    fn test_topology_spread_constraint_with_fields() {
        let mut label_selector = HashMap::new();
        label_selector.insert("app".to_string(), "nginx".to_string());

        let constraint = TopologySpreadConstraint {
            max_skew: 1,
            topology_key: "topology.kubernetes.io/zone".to_string(),
            when_unsatisfiable: when_unsatisfiable::DO_NOT_SCHEDULE.to_string(),
            label_selector,
            min_domains: Some(2),
            node_affinity_policy: node_affinity_policy::HONOR.to_string(),
            match_label_keys: vec!["app".to_string()],
            ..Default::default()
        };

        assert_eq!(constraint.max_skew, 1);
        assert_eq!(constraint.topology_key, "topology.kubernetes.io/zone");
        assert_eq!(constraint.when_unsatisfiable, "DoNotSchedule");
        assert_eq!(constraint.min_domains, Some(2));
        assert_eq!(constraint.node_affinity_policy, "Honor");
        assert_eq!(constraint.match_label_keys.len(), 1);
    }

    #[test]
    fn test_topology_spread_constraint_serialization() {
        let mut label_selector = HashMap::new();
        label_selector.insert("app".to_string(), "web".to_string());

        let constraint = TopologySpreadConstraint {
            max_skew: 2,
            topology_key: "kubernetes.io/hostname".to_string(),
            when_unsatisfiable: when_unsatisfiable::SCHEDULE_ANYWAY.to_string(),
            label_selector,
            ..Default::default()
        };

        let json = serde_json::to_string(&constraint).unwrap();
        let deserialized: TopologySpreadConstraint = serde_json::from_str(&json).unwrap();

        assert_eq!(constraint.max_skew, deserialized.max_skew);
        assert_eq!(constraint.topology_key, deserialized.topology_key);
        assert_eq!(
            constraint.when_unsatisfiable,
            deserialized.when_unsatisfiable
        );
    }

    #[test]
    fn test_topology_spread_constraint_with_match_label_keys() {
        let constraint = TopologySpreadConstraint {
            max_skew: 1,
            topology_key: "zone".to_string(),
            when_unsatisfiable: when_unsatisfiable::DO_NOT_SCHEDULE.to_string(),
            match_label_keys: vec!["app".to_string(), "tier".to_string()],
            ..Default::default()
        };

        let json = serde_json::to_string(&constraint).unwrap();
        let deserialized: TopologySpreadConstraint = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.match_label_keys.len(), 2);
        assert!(deserialized.match_label_keys.contains(&"app".to_string()));
        assert!(deserialized.match_label_keys.contains(&"tier".to_string()));
    }

    #[test]
    fn test_topology_spread_constraint_with_min_domains() {
        let constraint = TopologySpreadConstraint {
            max_skew: 1,
            topology_key: "topology.kubernetes.io/zone".to_string(),
            when_unsatisfiable: when_unsatisfiable::DO_NOT_SCHEDULE.to_string(),
            min_domains: Some(3),
            ..Default::default()
        };

        assert_eq!(constraint.min_domains, Some(3));

        let json = serde_json::to_string(&constraint).unwrap();
        let deserialized: TopologySpreadConstraint = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.min_domains, Some(3));
    }

    #[test]
    fn test_when_unsatisfiable_constants() {
        assert_eq!(when_unsatisfiable::DO_NOT_SCHEDULE, "DoNotSchedule");
        assert_eq!(when_unsatisfiable::SCHEDULE_ANYWAY, "ScheduleAnyway");
    }

    #[test]
    fn test_node_affinity_policy_constants() {
        assert_eq!(node_affinity_policy::IGNORE, "Ignore");
        assert_eq!(node_affinity_policy::HONOR, "Honor");
    }

    #[test]
    fn test_topology_spread_constraint_with_node_affinity_policy() {
        let constraint = TopologySpreadConstraint {
            max_skew: 1,
            topology_key: "topology.kubernetes.io/zone".to_string(),
            when_unsatisfiable: when_unsatisfiable::DO_NOT_SCHEDULE.to_string(),
            node_affinity_policy: node_affinity_policy::IGNORE.to_string(),
            ..Default::default()
        };

        assert_eq!(constraint.node_affinity_policy, "Ignore");
    }

    #[test]
    fn test_topology_spread_constraint_round_trip() {
        let mut label_selector = HashMap::new();
        label_selector.insert("app".to_string(), "database".to_string());

        let constraint = TopologySpreadConstraint {
            max_skew: 1,
            topology_key: "topology.kubernetes.io/zone".to_string(),
            when_unsatisfiable: when_unsatisfiable::DO_NOT_SCHEDULE.to_string(),
            label_selector: label_selector.clone(),
            min_domains: Some(2),
            node_affinity_policy: node_affinity_policy::HONOR.to_string(),
            match_label_keys: vec!["app".to_string(), "tier".to_string()],
        };

        let json = serde_json::to_string(&constraint).unwrap();
        let deserialized: TopologySpreadConstraint = serde_json::from_str(&json).unwrap();

        assert_eq!(constraint, deserialized);
    }

    #[test]
    fn test_topology_spread_constraint_skip_empty_fields() {
        let constraint = TopologySpreadConstraint {
            max_skew: 1,
            topology_key: "zone".to_string(),
            when_unsatisfiable: when_unsatisfiable::DO_NOT_SCHEDULE.to_string(),
            ..Default::default()
        };

        let json = serde_json::to_string(&constraint).unwrap();
        // Empty fields should not be in JSON due to skip_serializing_if
        assert!(!json.contains("labelSelector"));
        assert!(!json.contains("minDomains"));
        assert!(!json.contains("nodeAffinityPolicy"));
        assert!(!json.contains("matchLabelKeys"));

        let deserialized: TopologySpreadConstraint = serde_json::from_str(&json).unwrap();
        assert!(deserialized.label_selector.is_empty());
        assert!(deserialized.min_domains.is_none());
    }

    #[test]
    fn test_topology_spread_constraint_with_label_selector() {
        let mut label_selector = HashMap::new();
        label_selector.insert("app".to_string(), "frontend".to_string());
        label_selector.insert("tier".to_string(), "web".to_string());

        let constraint = TopologySpreadConstraint {
            max_skew: 2,
            topology_key: "kubernetes.io/hostname".to_string(),
            when_unsatisfiable: when_unsatisfiable::SCHEDULE_ANYWAY.to_string(),
            label_selector,
            ..Default::default()
        };

        let json = serde_json::to_string(&constraint).unwrap();
        assert!(json.contains("labelSelector"));

        let deserialized: TopologySpreadConstraint = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.label_selector.len(), 2);
    }
}
