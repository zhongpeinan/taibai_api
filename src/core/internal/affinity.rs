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
    use super::*;

    // Affinity tests
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
            ..Default::default()
        };
        assert!(affinity.node_affinity.is_some());
        assert!(affinity.pod_affinity.is_none());
    }

    #[test]
    fn test_affinity_serialize() {
        let affinity = Affinity {
            node_affinity: Some(NodeAffinity::default()),
            ..Default::default()
        };
        let json = serde_json::to_string(&affinity).unwrap();
        assert!(json.contains("\"nodeAffinity\""));
    }

    #[test]
    fn test_affinity_round_trip() {
        let original = Affinity {
            pod_affinity: Some(PodAffinity::default()),
            pod_anti_affinity: Some(PodAntiAffinity::default()),
            ..Default::default()
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Affinity = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    // NodeAffinity tests
    #[test]
    fn test_node_affinity_default() {
        let affinity = NodeAffinity::default();
        assert!(
            affinity
                .required_during_scheduling_ignored_during_execution
                .is_none()
        );
        assert!(
            affinity
                .preferred_during_scheduling_ignored_during_execution
                .is_empty()
        );
    }

    #[test]
    fn test_node_affinity_with_fields() {
        let mut match_labels = std::collections::HashMap::new();
        match_labels.insert("disktype".to_string(), "ssd".to_string());

        let affinity = NodeAffinity {
            preferred_during_scheduling_ignored_during_execution: vec![PreferredSchedulingTerm {
                weight: 100,
                preference: NodeSelectorTerm {
                    match_expressions: vec![],
                    match_fields: vec![],
                },
            }],
            ..Default::default()
        };
        assert_eq!(
            affinity
                .preferred_during_scheduling_ignored_during_execution
                .len(),
            1
        );
        assert_eq!(
            affinity.preferred_during_scheduling_ignored_during_execution[0].weight,
            100
        );
    }

    #[test]
    fn test_node_affinity_serialize() {
        let affinity = NodeAffinity {
            preferred_during_scheduling_ignored_during_execution: vec![PreferredSchedulingTerm {
                weight: 50,
                ..Default::default()
            }],
            ..Default::default()
        };
        let json = serde_json::to_string(&affinity).unwrap();
        assert!(json.contains("\"preferredDuringSchedulingIgnoredDuringExecution\""));
        assert!(json.contains("\"weight\":50"));
    }

    #[test]
    fn test_node_affinity_round_trip() {
        let original = NodeAffinity {
            required_during_scheduling_ignored_during_execution: Some(NodeSelector {
                node_selector_terms: vec![],
            }),
            preferred_during_scheduling_ignored_during_execution: vec![],
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: NodeAffinity = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    // PreferredSchedulingTerm tests
    #[test]
    fn test_preferred_scheduling_term_default() {
        let term = PreferredSchedulingTerm::default();
        assert_eq!(term.weight, 0);
    }

    #[test]
    fn test_preferred_scheduling_term_with_fields() {
        let term = PreferredSchedulingTerm {
            weight: 100,
            preference: NodeSelectorTerm {
                match_expressions: vec![],
                match_fields: vec![],
            },
        };
        assert_eq!(term.weight, 100);
    }

    #[test]
    fn test_preferred_scheduling_term_serialize() {
        let term = PreferredSchedulingTerm {
            weight: 75,
            ..Default::default()
        };
        let json = serde_json::to_string(&term).unwrap();
        assert!(json.contains("\"weight\":75"));
        assert!(json.contains("\"preference\""));
    }

    #[test]
    fn test_preferred_scheduling_term_round_trip() {
        let original = PreferredSchedulingTerm {
            weight: 42,
            ..Default::default()
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: PreferredSchedulingTerm = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    // PodAffinity tests
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
    fn test_pod_affinity_with_required() {
        let affinity = PodAffinity {
            required_during_scheduling_ignored_during_execution: vec![PodAffinityTerm {
                topology_key: "kubernetes.io/hostname".to_string(),
                ..Default::default()
            }],
            ..Default::default()
        };
        assert_eq!(
            affinity
                .required_during_scheduling_ignored_during_execution
                .len(),
            1
        );
    }

    #[test]
    fn test_pod_affinity_serialize() {
        let affinity = PodAffinity {
            required_during_scheduling_ignored_during_execution: vec![PodAffinityTerm {
                topology_key: "zone".to_string(),
                ..Default::default()
            }],
            ..Default::default()
        };
        let json = serde_json::to_string(&affinity).unwrap();
        assert!(json.contains("\"requiredDuringSchedulingIgnoredDuringExecution\""));
        assert!(json.contains("\"zone\""));
    }

    #[test]
    fn test_pod_affinity_round_trip() {
        let original = PodAffinity {
            preferred_during_scheduling_ignored_during_execution: vec![WeightedPodAffinityTerm {
                weight: 100,
                pod_affinity_term: PodAffinityTerm::default(),
            }],
            ..Default::default()
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: PodAffinity = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    // PodAntiAffinity tests
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
    fn test_pod_anti_affinity_with_preferred() {
        let affinity = PodAntiAffinity {
            preferred_during_scheduling_ignored_during_execution: vec![WeightedPodAffinityTerm {
                weight: 50,
                pod_affinity_term: PodAffinityTerm {
                    topology_key: "kubernetes.io/hostname".to_string(),
                    ..Default::default()
                },
            }],
            ..Default::default()
        };
        assert_eq!(
            affinity
                .preferred_during_scheduling_ignored_during_execution
                .len(),
            1
        );
        assert_eq!(
            affinity.preferred_during_scheduling_ignored_during_execution[0].weight,
            50
        );
    }

    #[test]
    fn test_pod_anti_affinity_serialize() {
        let affinity = PodAntiAffinity {
            preferred_during_scheduling_ignored_during_execution: vec![WeightedPodAffinityTerm {
                weight: 100,
                ..Default::default()
            }],
            ..Default::default()
        };
        let json = serde_json::to_string(&affinity).unwrap();
        assert!(json.contains("\"preferredDuringSchedulingIgnoredDuringExecution\""));
        assert!(json.contains("\"weight\":100"));
    }

    #[test]
    fn test_pod_anti_affinity_round_trip() {
        let original = PodAntiAffinity {
            required_during_scheduling_ignored_during_execution: vec![PodAffinityTerm {
                topology_key: "zone".to_string(),
                ..Default::default()
            }],
            ..Default::default()
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: PodAntiAffinity = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    // PodAffinityTerm tests
    #[test]
    fn test_pod_affinity_term_default() {
        let term = PodAffinityTerm::default();
        assert!(term.label_selector.is_none());
        assert!(term.namespaces.is_empty());
        assert!(term.topology_key.is_empty());
    }

    #[test]
    fn test_pod_affinity_term_with_fields() {
        let term = PodAffinityTerm {
            topology_key: "kubernetes.io/hostname".to_string(),
            namespaces: vec!["default".to_string()],
            ..Default::default()
        };
        assert_eq!(term.topology_key, "kubernetes.io/hostname");
        assert_eq!(term.namespaces.len(), 1);
    }

    #[test]
    fn test_pod_affinity_term_serialize() {
        let term = PodAffinityTerm {
            topology_key: "zone".to_string(),
            ..Default::default()
        };
        let json = serde_json::to_string(&term).unwrap();
        assert!(json.contains("\"topologyKey\":\"zone\""));
    }

    #[test]
    fn test_pod_affinity_term_with_label_selector() {
        let mut match_labels = std::collections::HashMap::new();
        match_labels.insert("app".to_string(), "nginx".to_string());

        let term = PodAffinityTerm {
            label_selector: Some(LabelSelector {
                match_labels,
                match_expressions: vec![],
            }),
            topology_key: "kubernetes.io/hostname".to_string(),
            ..Default::default()
        };
        assert!(term.label_selector.is_some());
    }

    #[test]
    fn test_pod_affinity_term_round_trip() {
        let original = PodAffinityTerm {
            topology_key: "zone".to_string(),
            namespaces: vec!["default".to_string(), "kube-system".to_string()],
            match_label_keys: vec!["app".to_string()],
            ..Default::default()
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: PodAffinityTerm = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    // WeightedPodAffinityTerm tests
    #[test]
    fn test_weighted_pod_affinity_term_default() {
        let term = WeightedPodAffinityTerm::default();
        assert_eq!(term.weight, 0);
    }

    #[test]
    fn test_weighted_pod_affinity_term_with_fields() {
        let term = WeightedPodAffinityTerm {
            weight: 100,
            pod_affinity_term: PodAffinityTerm {
                topology_key: "zone".to_string(),
                ..Default::default()
            },
        };
        assert_eq!(term.weight, 100);
        assert_eq!(term.pod_affinity_term.topology_key, "zone");
    }

    #[test]
    fn test_weighted_pod_affinity_term_serialize() {
        let term = WeightedPodAffinityTerm {
            weight: 75,
            ..Default::default()
        };
        let json = serde_json::to_string(&term).unwrap();
        assert!(json.contains("\"weight\":75"));
        assert!(json.contains("\"podAffinityTerm\""));
    }

    #[test]
    fn test_weighted_pod_affinity_term_round_trip() {
        let original = WeightedPodAffinityTerm {
            weight: 42,
            pod_affinity_term: PodAffinityTerm {
                topology_key: "kubernetes.io/hostname".to_string(),
                ..Default::default()
            },
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: WeightedPodAffinityTerm = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    // Integration tests
    #[test]
    fn test_affinity_with_all_rules() {
        let affinity = Affinity {
            node_affinity: Some(NodeAffinity {
                preferred_during_scheduling_ignored_during_execution: vec![
                    PreferredSchedulingTerm {
                        weight: 100,
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }),
            pod_affinity: Some(PodAffinity {
                required_during_scheduling_ignored_during_execution: vec![PodAffinityTerm {
                    topology_key: "zone".to_string(),
                    ..Default::default()
                }],
                ..Default::default()
            }),
            pod_anti_affinity: Some(PodAntiAffinity {
                preferred_during_scheduling_ignored_during_execution: vec![
                    WeightedPodAffinityTerm {
                        weight: 50,
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }),
        };

        assert!(affinity.node_affinity.is_some());
        assert!(affinity.pod_affinity.is_some());
        assert!(affinity.pod_anti_affinity.is_some());
    }

    #[test]
    fn test_empty_affinity_serializes_to_empty_object() {
        let affinity = Affinity::default();
        let json = serde_json::to_string(&affinity).unwrap();
        assert_eq!(json, "{}");
    }
}
