//! Kubernetes Topology Spread Constraint types
//!
//! This module contains topology spread constraint types from the Kubernetes core/v1 API.
//! Topology spread constraints control how Pods are spread across a cluster.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

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
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub label_selector: BTreeMap<String, String>,
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
mod tests {}
