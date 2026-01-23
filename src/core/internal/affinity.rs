//! Affinity types from the Kubernetes Core API
//!
//! This module contains types for pod and node affinity scheduling rules.
//! These types control how pods are scheduled relative to other pods and nodes.
//!
//! Source: k8s-pkg/apis/core/types.go

use crate::core::internal::selector::{LabelSelector, NodeSelector, NodeSelectorTerm};
use serde::{Deserialize, Serialize};

// ============================================================================
// Affinity
// ============================================================================

/// Affinity is a group of affinity scheduling rules.
///
/// Corresponds to [Kubernetes Affinity](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3378)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Affinity {
    /// Describes node affinity scheduling rules for the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_affinity: Option<NodeAffinity>,
    /// Describes pod affinity scheduling rules.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pod_affinity: Option<PodAffinity>,
    /// Describes pod anti-affinity scheduling rules.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pod_anti_affinity: Option<PodAntiAffinity>,
}

// ============================================================================
// Node Affinity
// ============================================================================

/// NodeAffinity is a group of node affinity scheduling rules.
///
/// Corresponds to [Kubernetes NodeAffinity](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3527)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NodeAffinity {
    /// If the affinity requirements specified by this field are not met at
    /// scheduling time, the pod will not be scheduled onto the node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_during_scheduling_ignored_during_execution: Option<NodeSelector>,
    /// The scheduler will prefer to schedule pods to nodes that satisfy
    /// the affinity expressions specified by this field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub preferred_during_scheduling_ignored_during_execution: Vec<PreferredSchedulingTerm>,
}

/// PreferredSchedulingTerm represents an empty preferred scheduling term.
///
/// Corresponds to [Kubernetes PreferredSchedulingTerm](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3558)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PreferredSchedulingTerm {
    /// Weight associated with matching the corresponding nodeSelectorTerm, in the range 1-100.
    #[serde(default)]
    pub weight: i32,
    /// A node selector term, associated with the corresponding weight.
    #[serde(default)]
    pub preference: NodeSelectorTerm,
}

// ============================================================================
// Pod Affinity
// ============================================================================

/// PodAffinity is a group of inter pod affinity scheduling rules.
///
/// Corresponds to [Kubernetes PodAffinity](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3391)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodAffinity {
    /// If the affinity requirements specified by this field are not met at
    /// scheduling time, the pod will not be scheduled onto the node.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required_during_scheduling_ignored_during_execution: Vec<PodAffinityTerm>,
    /// The scheduler will prefer to schedule pods to nodes that satisfy
    /// the affinity expressions specified by this field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub preferred_during_scheduling_ignored_during_execution: Vec<WeightedPodAffinityTerm>,
}

/// PodAntiAffinity is a group of inter pod anti affinity scheduling rules.
///
/// Corresponds to [Kubernetes PodAntiAffinity](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3426)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodAntiAffinity {
    /// If the anti-affinity requirements specified by this field are not met at
    /// scheduling time, the pod will not be scheduled onto the node.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required_during_scheduling_ignored_during_execution: Vec<PodAffinityTerm>,
    /// The scheduler will prefer to schedule pods to nodes that satisfy
    /// the anti-affinity expressions specified by this field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub preferred_during_scheduling_ignored_during_execution: Vec<WeightedPodAffinityTerm>,
}

// ============================================================================
// Pod Affinity Term
// ============================================================================

/// PodAffinityTerm defines a set of pods that this pod should be
/// co-located (affinity) or not co-located (anti-affinity) with.
///
/// Corresponds to [Kubernetes PodAffinityTerm](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3471)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodAffinityTerm {
    /// A label query over a set of resources, in this case pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label_selector: Option<LabelSelector>,
    /// namespaces specifies a static list of namespace names that the term applies to.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub namespaces: Vec<String>,
    /// This pod should be co-located (affinity) or not co-located (anti-affinity)
    /// with the pods matching the labelSelector in the specified namespaces.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub topology_key: String,
    /// A label query over the set of namespaces that the term applies to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace_selector: Option<LabelSelector>,
    /// MatchLabelKeys is a set of pod label keys to select which pods will
    /// be taken into consideration.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub match_label_keys: Vec<String>,
    /// MismatchLabelKeys is a set of pod label keys to select which pods will
    /// be taken into consideration.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mismatch_label_keys: Vec<String>,
}

/// WeightedPodAffinityTerm represents the weights of all of the matched
/// WeightedPodAffinityTerm fields are added per-node to find the most preferred node(s).
///
/// Corresponds to [Kubernetes WeightedPodAffinityTerm](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3461)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct WeightedPodAffinityTerm {
    /// weight associated with matching the corresponding podAffinityTerm,
    /// in the range 1-100.
    #[serde(default)]
    pub weight: i32,
    /// Required. A pod affinity term, associated with the corresponding weight.
    #[serde(default)]
    pub pod_affinity_term: PodAffinityTerm,
}

#[cfg(test)]
mod tests {
}
