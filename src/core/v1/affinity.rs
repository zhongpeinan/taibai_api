//! Kubernetes Affinity types
//!
//! This module contains affinity-related types from the Kubernetes core/v1 API.
//! These types control Pod scheduling through node and pod affinity/anti-affinity rules.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
        skip_serializing_if = "HashMap::is_empty",
        rename = "labelSelector"
    )]
    pub label_selector: HashMap<String, String>,
    /// Namespace selector for Pods.
    #[serde(
        default,
        skip_serializing_if = "HashMap::is_empty",
        rename = "namespaceSelector"
    )]
    pub namespace_selector: HashMap<String, String>,
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
pub type NodeSelectorSimple = HashMap<String, String>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_affinity_default() {
        let affinity = Affinity::default();
        assert!(affinity.node_affinity.is_none());
        assert!(affinity.pod_affinity.is_none());
        assert!(affinity.pod_anti_affinity.is_none());
    }

    #[test]
    fn test_affinity_with_node_affinity() {
        let affinity = Affinity {
            node_affinity: Some(NodeAffinity::default()),
            pod_affinity: None,
            pod_anti_affinity: None,
        };

        assert!(affinity.node_affinity.is_some());
        assert!(affinity.pod_affinity.is_none());
    }

    #[test]
    fn test_node_affinity_default() {
        let node_affinity = NodeAffinity::default();
        assert!(
            node_affinity
                .required_during_scheduling_ignored_during_execution
                .is_none()
        );
        assert!(
            node_affinity
                .preferred_during_scheduling_ignored_during_execution
                .is_empty()
        );
    }

    #[test]
    fn test_node_selector_default() {
        let selector = NodeSelector::default();
        assert!(selector.node_selector_terms.is_empty());
    }

    #[test]
    fn test_node_selector_term_default() {
        let term = NodeSelectorTerm::default();
        assert!(term.match_expressions.is_empty());
        assert!(term.match_fields.is_empty());
    }

    #[test]
    fn test_node_selector_requirement() {
        let requirement = NodeSelectorRequirement {
            key: "disktype".to_string(),
            operator: node_selector_operator::IN.to_string(),
            values: vec!["ssd".to_string(), "hdd".to_string()],
        };

        let json = serde_json::to_string(&requirement).unwrap();
        let deserialized: NodeSelectorRequirement = serde_json::from_str(&json).unwrap();

        assert_eq!(requirement.key, deserialized.key);
        assert_eq!(requirement.operator, deserialized.operator);
        assert_eq!(requirement.values, deserialized.values);
    }

    #[test]
    fn test_node_selector_requirement_exists() {
        let requirement = NodeSelectorRequirement {
            key: "zone".to_string(),
            operator: node_selector_operator::EXISTS.to_string(),
            values: vec![],
        };

        assert_eq!(requirement.operator, node_selector_operator::EXISTS);
        assert!(requirement.values.is_empty());
    }

    #[test]
    fn test_node_selector_operator_constants() {
        assert_eq!(node_selector_operator::IN, "In");
        assert_eq!(node_selector_operator::NOT_IN, "NotIn");
        assert_eq!(node_selector_operator::EXISTS, "Exists");
        assert_eq!(node_selector_operator::DOES_NOT_EXIST, "DoesNotExist");
        assert_eq!(node_selector_operator::GT, "Gt");
        assert_eq!(node_selector_operator::LT, "Lt");
    }

    #[test]
    fn test_preferred_scheduling_term() {
        let term = PreferredSchedulingTerm {
            weight: 50,
            preference: Some(NodeSelectorTerm::default()),
        };

        let json = serde_json::to_string(&term).unwrap();
        let deserialized: PreferredSchedulingTerm = serde_json::from_str(&json).unwrap();

        assert_eq!(term.weight, deserialized.weight);
        assert!(term.preference.is_some());
    }

    #[test]
    fn test_pod_affinity_default() {
        let affinity = PodAffinity::default();
        assert!(
            affinity
                .required_during_scheduling_ignored_during_execution
                .is_empty()
        );
        assert!(
            affinity
                .preferred_during_scheduling_ignored_during_execution
                .is_empty()
        );
    }

    #[test]
    fn test_pod_anti_affinity_default() {
        let affinity = PodAntiAffinity::default();
        assert!(
            affinity
                .required_during_scheduling_ignored_during_execution
                .is_empty()
        );
        assert!(
            affinity
                .preferred_during_scheduling_ignored_during_execution
                .is_empty()
        );
    }

    #[test]
    fn test_pod_affinity_term() {
        let mut label_selector = HashMap::new();
        label_selector.insert("app".to_string(), "web".to_string());

        let term = PodAffinityTerm {
            label_selector,
            namespace_selector: HashMap::new(),
            namespaces: vec![],
            topology_key: "kubernetes.io/hostname".to_string(),
        };

        let json = serde_json::to_string(&term).unwrap();
        let deserialized: PodAffinityTerm = serde_json::from_str(&json).unwrap();

        assert_eq!(term.topology_key, deserialized.topology_key);
        assert!(deserialized.label_selector.contains_key("app"));
    }

    #[test]
    fn test_weighted_pod_affinity_term() {
        let mut label_selector = HashMap::new();
        label_selector.insert("app".to_string(), "db".to_string());

        let term = WeightedPodAffinityTerm {
            weight: 100,
            pod_affinity_term: Some(PodAffinityTerm {
                label_selector,
                namespace_selector: HashMap::new(),
                namespaces: vec![],
                topology_key: "zone".to_string(),
            }),
        };

        let json = serde_json::to_string(&term).unwrap();
        let deserialized: WeightedPodAffinityTerm = serde_json::from_str(&json).unwrap();

        assert_eq!(term.weight, deserialized.weight);
        assert!(term.pod_affinity_term.is_some());
    }

    #[test]
    fn test_affinity_round_trip() {
        let mut label_selector = HashMap::new();
        label_selector.insert("app".to_string(), "nginx".to_string());

        let affinity = Affinity {
            node_affinity: Some(NodeAffinity {
                required_during_scheduling_ignored_during_execution: Some(NodeSelector {
                    node_selector_terms: vec![NodeSelectorTerm {
                        match_expressions: vec![NodeSelectorRequirement {
                            key: "disktype".to_string(),
                            operator: node_selector_operator::IN.to_string(),
                            values: vec!["ssd".to_string()],
                        }],
                        match_fields: vec![],
                    }],
                }),
                preferred_during_scheduling_ignored_during_execution: vec![],
            }),
            pod_affinity: Some(PodAffinity {
                required_during_scheduling_ignored_during_execution: vec![PodAffinityTerm {
                    label_selector: label_selector.clone(),
                    namespace_selector: HashMap::new(),
                    namespaces: vec![],
                    topology_key: "kubernetes.io/hostname".to_string(),
                }],
                preferred_during_scheduling_ignored_during_execution: vec![],
            }),
            pod_anti_affinity: None,
        };

        let json = serde_json::to_string(&affinity).unwrap();
        let deserialized: Affinity = serde_json::from_str(&json).unwrap();

        assert!(deserialized.node_affinity.is_some());
        assert!(deserialized.pod_affinity.is_some());
        assert!(deserialized.pod_anti_affinity.is_none());
    }

    #[test]
    fn test_node_selector_requirement_gt_operator() {
        let requirement = NodeSelectorRequirement {
            key: "cpu".to_string(),
            operator: node_selector_operator::GT.to_string(),
            values: vec!["4".to_string()],
        };

        assert_eq!(requirement.operator, "Gt");
        assert_eq!(requirement.values.len(), 1);
    }

    #[test]
    fn test_node_selector_requirement_serialization_skip_empty() {
        let requirement = NodeSelectorRequirement {
            key: "test".to_string(),
            operator: node_selector_operator::EXISTS.to_string(),
            values: vec![],
        };

        let json = serde_json::to_string(&requirement).unwrap();
        // Empty values array should not be in JSON due to skip_serializing_if
        assert!(!json.contains("values"));

        let deserialized: NodeSelectorRequirement = serde_json::from_str(&json).unwrap();
        assert!(deserialized.values.is_empty());
    }

    #[test]
    fn test_pod_affinity_with_namespaces() {
        let term = PodAffinityTerm {
            label_selector: HashMap::new(),
            namespace_selector: HashMap::new(),
            namespaces: vec!["production".to_string(), "staging".to_string()],
            topology_key: "topology.kubernetes.io/zone".to_string(),
        };

        let json = serde_json::to_string(&term).unwrap();
        let deserialized: PodAffinityTerm = serde_json::from_str(&json).unwrap();

        assert_eq!(term.namespaces, deserialized.namespaces);
        assert_eq!(term.namespaces.len(), 2);
    }
}
