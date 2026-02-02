//! Phase 2 conversions: Selector/Affinity + Scheduling
//!
//! Toleration, Affinity hierarchy, PodDNSConfig, PodOS, PodSchedulingGate

use super::helpers::*;
use crate::common::{FromInternal, ToInternal};
use crate::core::internal;
use crate::core::v1::{affinity, pod, toleration};

// ============================================================================
// Toleration
// ============================================================================

impl ToInternal<internal::Toleration> for toleration::Toleration {
    fn to_internal(self) -> internal::Toleration {
        internal::Toleration {
            key: self.key,
            operator: option_toleration_operator_to_option_enum(self.operator),
            value: self.value,
            effect: option_taint_effect_to_option_enum(self.effect),
            toleration_seconds: self.toleration_seconds,
        }
    }
}

impl FromInternal<internal::Toleration> for toleration::Toleration {
    fn from_internal(value: internal::Toleration) -> Self {
        Self {
            key: value.key,
            operator: option_enum_to_toleration_operator(value.operator),
            value: value.value,
            effect: option_enum_to_taint_effect(value.effect),
            toleration_seconds: value.toleration_seconds,
        }
    }
}

// ============================================================================
// PodDNSConfig and PodDNSConfigOption
// ============================================================================

impl ToInternal<internal::PodDNSConfig> for pod::PodDNSConfig {
    fn to_internal(self) -> internal::PodDNSConfig {
        internal::PodDNSConfig {
            nameservers: self.nameservers,
            searches: self.searches,
            options: self.options.into_iter().map(|o| o.to_internal()).collect(),
        }
    }
}

impl FromInternal<internal::PodDNSConfig> for pod::PodDNSConfig {
    fn from_internal(value: internal::PodDNSConfig) -> Self {
        Self {
            nameservers: value.nameservers,
            searches: value.searches,
            options: value
                .options
                .into_iter()
                .map(pod::PodDNSConfigOption::from_internal)
                .collect(),
        }
    }
}

impl ToInternal<internal::PodDNSConfigOption> for pod::PodDNSConfigOption {
    fn to_internal(self) -> internal::PodDNSConfigOption {
        internal::PodDNSConfigOption {
            name: self.name,
            value: self.value,
        }
    }
}

impl FromInternal<internal::PodDNSConfigOption> for pod::PodDNSConfigOption {
    fn from_internal(value: internal::PodDNSConfigOption) -> Self {
        Self {
            name: value.name,
            value: value.value,
        }
    }
}

// ============================================================================
// PodOS
// ============================================================================

impl ToInternal<internal::PodOS> for pod::PodOS {
    fn to_internal(self) -> internal::PodOS {
        internal::PodOS {
            name: os_name_to_string(self.name),
        }
    }
}

impl FromInternal<internal::PodOS> for pod::PodOS {
    fn from_internal(value: internal::PodOS) -> Self {
        Self {
            name: string_to_os_name(value.name),
        }
    }
}

// ============================================================================
// PodSchedulingGate
// ============================================================================

impl ToInternal<internal::PodSchedulingGate> for pod::PodSchedulingGate {
    fn to_internal(self) -> internal::PodSchedulingGate {
        internal::PodSchedulingGate { name: self.name }
    }
}

impl FromInternal<internal::PodSchedulingGate> for pod::PodSchedulingGate {
    fn from_internal(value: internal::PodSchedulingGate) -> Self {
        Self { name: value.name }
    }
}

// ============================================================================
// Affinity types
// ============================================================================

impl ToInternal<internal::Affinity> for affinity::Affinity {
    fn to_internal(self) -> internal::Affinity {
        internal::Affinity {
            node_affinity: self.node_affinity.map(|a| a.to_internal()),
            pod_affinity: self.pod_affinity.map(|a| a.to_internal()),
            pod_anti_affinity: self.pod_anti_affinity.map(|a| a.to_internal()),
        }
    }
}

impl FromInternal<internal::Affinity> for affinity::Affinity {
    fn from_internal(value: internal::Affinity) -> Self {
        Self {
            node_affinity: value
                .node_affinity
                .map(affinity::NodeAffinity::from_internal),
            pod_affinity: value.pod_affinity.map(affinity::PodAffinity::from_internal),
            pod_anti_affinity: value
                .pod_anti_affinity
                .map(affinity::PodAntiAffinity::from_internal),
        }
    }
}

impl ToInternal<internal::NodeAffinity> for affinity::NodeAffinity {
    fn to_internal(self) -> internal::NodeAffinity {
        internal::NodeAffinity {
            required_during_scheduling_ignored_during_execution: self
                .required_during_scheduling_ignored_during_execution
                .map(|s| s.to_internal()),
            preferred_during_scheduling_ignored_during_execution: self
                .preferred_during_scheduling_ignored_during_execution
                .into_iter()
                .map(|t| t.to_internal())
                .collect(),
        }
    }
}

impl FromInternal<internal::NodeAffinity> for affinity::NodeAffinity {
    fn from_internal(value: internal::NodeAffinity) -> Self {
        Self {
            required_during_scheduling_ignored_during_execution: value
                .required_during_scheduling_ignored_during_execution
                .map(affinity::NodeSelector::from_internal),
            preferred_during_scheduling_ignored_during_execution: value
                .preferred_during_scheduling_ignored_during_execution
                .into_iter()
                .map(affinity::PreferredSchedulingTerm::from_internal)
                .collect(),
        }
    }
}

impl ToInternal<internal::NodeSelector> for affinity::NodeSelector {
    fn to_internal(self) -> internal::NodeSelector {
        internal::NodeSelector {
            node_selector_terms: self
                .node_selector_terms
                .into_iter()
                .map(|t| t.to_internal())
                .collect(),
        }
    }
}

impl FromInternal<internal::NodeSelector> for affinity::NodeSelector {
    fn from_internal(value: internal::NodeSelector) -> Self {
        Self {
            node_selector_terms: value
                .node_selector_terms
                .into_iter()
                .map(affinity::NodeSelectorTerm::from_internal)
                .collect(),
        }
    }
}

impl ToInternal<internal::NodeSelectorTerm> for affinity::NodeSelectorTerm {
    fn to_internal(self) -> internal::NodeSelectorTerm {
        internal::NodeSelectorTerm {
            match_expressions: self
                .match_expressions
                .into_iter()
                .map(|e| e.to_internal())
                .collect(),
            match_fields: self
                .match_fields
                .into_iter()
                .map(|f| f.to_internal())
                .collect(),
        }
    }
}

impl FromInternal<internal::NodeSelectorTerm> for affinity::NodeSelectorTerm {
    fn from_internal(value: internal::NodeSelectorTerm) -> Self {
        Self {
            match_expressions: value
                .match_expressions
                .into_iter()
                .map(affinity::NodeSelectorRequirement::from_internal)
                .collect(),
            match_fields: value
                .match_fields
                .into_iter()
                .map(affinity::NodeSelectorRequirement::from_internal)
                .collect(),
        }
    }
}

impl ToInternal<internal::NodeSelectorRequirement> for affinity::NodeSelectorRequirement {
    fn to_internal(self) -> internal::NodeSelectorRequirement {
        internal::NodeSelectorRequirement {
            key: self.key,
            operator: self.operator,
            values: self.values,
        }
    }
}

impl FromInternal<internal::NodeSelectorRequirement> for affinity::NodeSelectorRequirement {
    fn from_internal(value: internal::NodeSelectorRequirement) -> Self {
        Self {
            key: value.key,
            operator: value.operator,
            values: value.values,
        }
    }
}

// PreferredSchedulingTerm: v1 has Option<NodeSelectorTerm>, internal has NodeSelectorTerm
impl ToInternal<internal::PreferredSchedulingTerm> for affinity::PreferredSchedulingTerm {
    fn to_internal(self) -> internal::PreferredSchedulingTerm {
        internal::PreferredSchedulingTerm {
            weight: self.weight,
            preference: self.preference.unwrap_or_default().to_internal(),
        }
    }
}

impl FromInternal<internal::PreferredSchedulingTerm> for affinity::PreferredSchedulingTerm {
    fn from_internal(value: internal::PreferredSchedulingTerm) -> Self {
        Self {
            weight: value.weight,
            preference: Some(affinity::NodeSelectorTerm::from_internal(value.preference)),
        }
    }
}

impl ToInternal<internal::PodAffinity> for affinity::PodAffinity {
    fn to_internal(self) -> internal::PodAffinity {
        internal::PodAffinity {
            required_during_scheduling_ignored_during_execution: self
                .required_during_scheduling_ignored_during_execution
                .into_iter()
                .map(|t| t.to_internal())
                .collect(),
            preferred_during_scheduling_ignored_during_execution: self
                .preferred_during_scheduling_ignored_during_execution
                .into_iter()
                .map(|t| t.to_internal())
                .collect(),
        }
    }
}

impl FromInternal<internal::PodAffinity> for affinity::PodAffinity {
    fn from_internal(value: internal::PodAffinity) -> Self {
        Self {
            required_during_scheduling_ignored_during_execution: value
                .required_during_scheduling_ignored_during_execution
                .into_iter()
                .map(affinity::PodAffinityTerm::from_internal)
                .collect(),
            preferred_during_scheduling_ignored_during_execution: value
                .preferred_during_scheduling_ignored_during_execution
                .into_iter()
                .map(affinity::WeightedPodAffinityTerm::from_internal)
                .collect(),
        }
    }
}

impl ToInternal<internal::PodAntiAffinity> for affinity::PodAntiAffinity {
    fn to_internal(self) -> internal::PodAntiAffinity {
        internal::PodAntiAffinity {
            required_during_scheduling_ignored_during_execution: self
                .required_during_scheduling_ignored_during_execution
                .into_iter()
                .map(|t| t.to_internal())
                .collect(),
            preferred_during_scheduling_ignored_during_execution: self
                .preferred_during_scheduling_ignored_during_execution
                .into_iter()
                .map(|t| t.to_internal())
                .collect(),
        }
    }
}

impl FromInternal<internal::PodAntiAffinity> for affinity::PodAntiAffinity {
    fn from_internal(value: internal::PodAntiAffinity) -> Self {
        Self {
            required_during_scheduling_ignored_during_execution: value
                .required_during_scheduling_ignored_during_execution
                .into_iter()
                .map(affinity::PodAffinityTerm::from_internal)
                .collect(),
            preferred_during_scheduling_ignored_during_execution: value
                .preferred_during_scheduling_ignored_during_execution
                .into_iter()
                .map(affinity::WeightedPodAffinityTerm::from_internal)
                .collect(),
        }
    }
}

impl ToInternal<internal::PodAffinityTerm> for affinity::PodAffinityTerm {
    fn to_internal(self) -> internal::PodAffinityTerm {
        internal::PodAffinityTerm {
            label_selector: self.label_selector,
            namespaces: self.namespaces,
            topology_key: self.topology_key,
            namespace_selector: self.namespace_selector,
            match_label_keys: Vec::new(),    // v1 doesn't have this
            mismatch_label_keys: Vec::new(), // v1 doesn't have this
        }
    }
}

impl FromInternal<internal::PodAffinityTerm> for affinity::PodAffinityTerm {
    fn from_internal(value: internal::PodAffinityTerm) -> Self {
        Self {
            label_selector: value.label_selector,
            namespaces: value.namespaces,
            topology_key: value.topology_key,
            namespace_selector: value.namespace_selector,
            // match_label_keys and mismatch_label_keys are dropped (not in v1)
        }
    }
}

impl ToInternal<internal::WeightedPodAffinityTerm> for affinity::WeightedPodAffinityTerm {
    fn to_internal(self) -> internal::WeightedPodAffinityTerm {
        internal::WeightedPodAffinityTerm {
            weight: self.weight,
            pod_affinity_term: self.pod_affinity_term.unwrap_or_default().to_internal(),
        }
    }
}

impl FromInternal<internal::WeightedPodAffinityTerm> for affinity::WeightedPodAffinityTerm {
    fn from_internal(value: internal::WeightedPodAffinityTerm) -> Self {
        Self {
            weight: value.weight,
            pod_affinity_term: Some(affinity::PodAffinityTerm::from_internal(
                value.pod_affinity_term,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::internal::selector::LabelSelector;

    #[test]
    fn test_toleration_roundtrip() {
        let v1_toleration = toleration::Toleration {
            key: "node.kubernetes.io/memory-pressure".to_string(),
            operator: "Exists".to_string(),
            value: String::new(),
            effect: "NoExecute".to_string(),
            toleration_seconds: Some(300),
        };

        let internal_toleration = v1_toleration.clone().to_internal();
        let mut roundtrip = toleration::Toleration::from_internal(internal_toleration);

        assert_eq!(v1_toleration, roundtrip);
    }

    #[test]
    fn test_pod_dns_config_roundtrip() {
        let v1_dns = pod::PodDNSConfig {
            nameservers: vec!["1.2.3.4".to_string(), "8.8.8.8".to_string()],
            searches: vec!["ns1.svc.cluster-domain.example".to_string()],
            options: vec![pod::PodDNSConfigOption {
                name: "ndots".to_string(),
                value: Some("2".to_string()),
            }],
        };

        let internal_dns = v1_dns.clone().to_internal();
        let mut roundtrip = pod::PodDNSConfig::from_internal(internal_dns);

        assert_eq!(v1_dns, roundtrip);
    }

    #[test]
    fn test_affinity_roundtrip() {
        let v1_affinity = affinity::Affinity {
            node_affinity: Some(affinity::NodeAffinity {
                required_during_scheduling_ignored_during_execution: Some(affinity::NodeSelector {
                    node_selector_terms: vec![affinity::NodeSelectorTerm {
                        match_expressions: vec![affinity::NodeSelectorRequirement {
                            key: "kubernetes.io/e2e-az-name".to_string(),
                            operator: "In".to_string(),
                            values: vec!["e2e-az1".to_string(), "e2e-az2".to_string()],
                        }],
                        match_fields: vec![],
                    }],
                }),
                preferred_during_scheduling_ignored_during_execution: vec![],
            }),
            pod_affinity: Some(affinity::PodAffinity {
                required_during_scheduling_ignored_during_execution: vec![
                    affinity::PodAffinityTerm {
                        label_selector: Some(LabelSelector::default()),
                        namespaces: vec!["default".to_string()],
                        topology_key: "kubernetes.io/hostname".to_string(),
                        namespace_selector: None,
                    },
                ],
                preferred_during_scheduling_ignored_during_execution: vec![],
            }),
            pod_anti_affinity: None,
        };

        let internal_affinity = v1_affinity.clone().to_internal();
        let mut roundtrip = affinity::Affinity::from_internal(internal_affinity);

        assert_eq!(v1_affinity, roundtrip);
    }
}
