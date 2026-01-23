//! Kubernetes Affinity types
//!
//! This module contains affinity-related types from the Kubernetes core/v1 API.
//! These types control Pod scheduling through node and pod affinity/anti-affinity rules.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Affinity defines scheduling constraints for Pods.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Affinity {
    /// Node affinity scheduling rules for the Pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_affinity: Option<NodeAffinity>,
    /// Pod affinity scheduling rules for the Pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pod_affinity: Option<PodAffinity>,
    /// Pod anti-affinity scheduling rules for the Pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pod_anti_affinity: Option<PodAntiAffinity>,
}

/// NodeAffinity defines node affinity scheduling rules for the Pod.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NodeAffinity {
    /// Required node selector terms during scheduling.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_during_scheduling_ignored_during_execution: Option<NodeSelector>,
    /// Preferred node selector terms during scheduling.
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        rename = "preferredDuringSchedulingIgnoredDuringExecution"
    )]
    pub preferred_during_scheduling_ignored_during_execution: Vec<PreferredSchedulingTerm>,
}

/// NodeSelector represents a simple node selector with required terms.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NodeSelector {
    /// Required node selector terms.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub node_selector_terms: Vec<NodeSelectorTerm>,
}

/// NodeSelectorTerm represents a single node selector term with match expressions and fields.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NodeSelectorTerm {
    /// Match expressions for node labels.
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        rename = "matchExpressions"
    )]
    pub match_expressions: Vec<NodeSelectorRequirement>,
    /// Match fields for node fields.
    #[serde(default, skip_serializing_if = "Vec::is_empty", rename = "matchFields")]
    pub match_fields: Vec<NodeSelectorRequirement>,
}

/// NodeSelectorRequirement represents a single node selector requirement.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NodeSelectorRequirement {
    /// Label key or field key to apply the operator.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub key: String,
    /// Operator for the requirement.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub operator: NodeSelectorOperator,
    /// Values for the operator (In, NotIn, Gt, Lt).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
}

/// NodeSelectorOperator is the operator for a node selector requirement.
pub type NodeSelectorOperator = String;

/// Constants for NodeSelectorOperator values
pub mod node_selector_operator {
    pub const IN: &str = "In";
    pub const NOT_IN: &str = "NotIn";
    pub const EXISTS: &str = "Exists";
    pub const DOES_NOT_EXIST: &str = "DoesNotExist";
    pub const GT: &str = "Gt";
    pub const LT: &str = "Lt";
}

/// PreferredSchedulingTerm represents a preferred scheduling term with weight.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PreferredSchedulingTerm {
    /// Weight associated with the preference (1-100).
    #[serde(default)]
    pub weight: i32,
    /// Node selector term indicating the preference.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preference: Option<NodeSelectorTerm>,
}

/// PodAffinity defines pod affinity scheduling rules for the Pod.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodAffinity {
    /// Required pod affinity rules during scheduling.
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        rename = "requiredDuringSchedulingIgnoredDuringExecution"
    )]
    pub required_during_scheduling_ignored_during_execution: Vec<PodAffinityTerm>,
    /// Preferred pod affinity rules during scheduling.
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        rename = "preferredDuringSchedulingIgnoredDuringExecution"
    )]
    pub preferred_during_scheduling_ignored_during_execution: Vec<WeightedPodAffinityTerm>,
}

/// PodAntiAffinity defines pod anti-affinity scheduling rules for the Pod.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodAntiAffinity {
    /// Required pod anti-affinity rules during scheduling.
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        rename = "requiredDuringSchedulingIgnoredDuringExecution"
    )]
    pub required_during_scheduling_ignored_during_execution: Vec<PodAffinityTerm>,
    /// Preferred pod anti-affinity rules during scheduling.
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        rename = "preferredDuringSchedulingIgnoredDuringExecution"
    )]
    pub preferred_during_scheduling_ignored_during_execution: Vec<WeightedPodAffinityTerm>,
}

/// PodAffinityTerm defines a single pod affinity/anti-affinity term.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodAffinityTerm {
    /// Label selector for Pods.
    #[serde(
        default,
        skip_serializing_if = "BTreeMap::is_empty",
        rename = "labelSelector"
    )]
    pub label_selector: BTreeMap<String, String>,
    /// Namespace selector for Pods.
    #[serde(
        default,
        skip_serializing_if = "BTreeMap::is_empty",
        rename = "namespaceSelector"
    )]
    pub namespace_selector: BTreeMap<String, String>,
    /// Namespaces for the label selector.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub namespaces: Vec<String>,
    /// Topology key for the affinity rule.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        rename = "topologyKey"
    )]
    pub topology_key: String,
}

/// WeightedPodAffinityTerm represents a weighted pod affinity/anti-affinity term.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct WeightedPodAffinityTerm {
    /// Weight associated with the term (1-100).
    #[serde(default)]
    pub weight: i32,
    /// Pod affinity term.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pod_affinity_term: Option<PodAffinityTerm>,
}

/// SimpleNodeSelector is a map of label key to value for node selection.
pub type NodeSelectorSimple = BTreeMap<String, String>;

#[cfg(test)]
mod tests {}
