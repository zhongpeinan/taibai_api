//! Scheduling types from the Kubernetes Core API
//!
//! This module contains types for pod scheduling configuration,
//! including taints, tolerations, DNS config, and scheduling gates.
//!
//! Source: k8s-pkg/apis/core/types.go

use crate::common::Timestamp;
use crate::core::internal::{OSName, TaintEffect, TolerationOperator};
use serde::{Deserialize, Serialize};

// ============================================================================
// Taints and Tolerations
// ============================================================================

/// Taint represents a taint that can be applied to a node.
///
/// The node this Taint is attached to has the "effect" on
/// any pod that does not tolerate the Taint.
///
/// Corresponds to [Kubernetes Taint](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3567)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Taint {
    /// Required. The taint key to be applied to a node.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub key: String,
    /// Required. The taint value corresponding to the taint key.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub value: String,
    /// Required. The effect of the taint on pods
    /// that do not tolerate the taint.
    pub effect: TaintEffect,
    /// TimeAdded represents the time at which the taint was added.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_added: Option<Timestamp>,
}

/// Toleration represents the toleration object that can be attached to a pod.
///
/// The pod this Toleration is attached to tolerates any taint that matches
/// the triple <key,value,effect> using the matching operator <operator>.
///
/// Corresponds to [Kubernetes Toleration](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3610)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Toleration {
    /// Key is the taint key that the toleration applies to.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub key: String,
    /// Operator represents a key's relationship to the value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<TolerationOperator>,
    /// Value is the taint value the toleration matches to.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub value: String,
    /// Effect indicates the taint effect to match.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect: Option<TaintEffect>,
    /// TolerationSeconds represents the period of time the toleration
    /// (which must be of effect NoExecute, otherwise this field is ignored)
    /// tolerates the taint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub toleration_seconds: Option<i64>,
}

// ============================================================================
// DNS Configuration
// ============================================================================

/// PodDNSConfig defines the DNS configuration of a pod.
///
/// Corresponds to [Kubernetes PodDNSConfig](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4283)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodDNSConfig {
    /// A list of DNS name server IP addresses.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub nameservers: Vec<String>,
    /// A list of DNS search domains for host-name lookup.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub searches: Vec<String>,
    /// A list of DNS resolver options.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<PodDNSConfigOption>,
}

/// PodDNSConfigOption defines DNS resolver options of a pod.
///
/// Corresponds to [Kubernetes PodDNSConfigOption](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L4302)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodDNSConfigOption {
    /// Required.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// Value of the option.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

// ============================================================================
// Pod OS and Scheduling Gates
// ============================================================================

/// PodOS defines the OS parameters of a pod.
///
/// Corresponds to [Kubernetes PodOS](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3968)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PodOS {
    /// Name is the name of the operating system.
    pub name: OSName,
}

impl Default for PodOS {
    fn default() -> Self {
        Self {
            name: OSName::Linux,
        }
    }
}

/// PodSchedulingGate is associated to a Pod to guard its scheduling.
///
/// Corresponds to [Kubernetes PodSchedulingGate](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3977)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodSchedulingGate {
    /// Name of the scheduling gate.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    // Taint tests
    #[test]
    fn test_taint_serialize() {
        let taint = Taint {
            key: "key1".to_string(),
            value: "value1".to_string(),
            effect: TaintEffect::NoSchedule,
            time_added: None,
        };

        let json = serde_json::to_string(&taint).unwrap();
        assert!(json.contains("\"key\":\"key1\""));
        assert!(json.contains("\"value\":\"value1\""));
        assert!(json.contains("\"effect\":\"NoSchedule\""));
    }

    #[test]
    fn test_taint_deserialize() {
        let json = r#"{"key":"test-key","value":"test-value","effect":"NoExecute"}"#;
        let taint: Taint = serde_json::from_str(json).unwrap();

        assert_eq!(taint.key, "test-key");
        assert_eq!(taint.value, "test-value");
        assert_eq!(taint.effect, TaintEffect::NoExecute);
    }

    #[test]
    fn test_taint_round_trip() {
        let original = Taint {
            key: "node.kubernetes.io/not-ready".to_string(),
            value: String::new(),
            effect: TaintEffect::NoExecute,
            time_added: None,
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Taint = serde_json::from_str(&json).unwrap();

        assert_eq!(original.key, deserialized.key);
        assert_eq!(original.effect, deserialized.effect);
    }

    #[test]
    fn test_taint_with_time_added() {
        let taint = Taint {
            key: "key1".to_string(),
            value: "value1".to_string(),
            effect: TaintEffect::NoSchedule,
            time_added: Some(Timestamp::from_str("2024-01-01T00:00:00Z").unwrap()),
        };

        let json = serde_json::to_string(&taint).unwrap();
        assert!(json.contains("\"timeAdded\""));
    }

    // Toleration tests
    #[test]
    fn test_toleration_default() {
        let toleration = Toleration::default();
        assert!(toleration.key.is_empty());
        assert!(toleration.operator.is_none());
        assert!(toleration.value.is_empty());
        assert!(toleration.effect.is_none());
        assert!(toleration.toleration_seconds.is_none());
    }

    #[test]
    fn test_toleration_with_fields() {
        let toleration = Toleration {
            key: "key1".to_string(),
            operator: Some(TolerationOperator::Equal),
            value: "value1".to_string(),
            effect: Some(TaintEffect::NoSchedule),
            toleration_seconds: Some(3600),
        };

        assert_eq!(toleration.key, "key1");
        assert_eq!(toleration.operator, Some(TolerationOperator::Equal));
        assert_eq!(toleration.effect, Some(TaintEffect::NoSchedule));
        assert_eq!(toleration.toleration_seconds, Some(3600));
    }

    #[test]
    fn test_toleration_serialize() {
        let toleration = Toleration {
            key: "node.kubernetes.io/not-ready".to_string(),
            operator: Some(TolerationOperator::Exists),
            effect: Some(TaintEffect::NoExecute),
            ..Default::default()
        };

        let json = serde_json::to_string(&toleration).unwrap();
        assert!(json.contains("\"key\":\"node.kubernetes.io/not-ready\""));
        assert!(json.contains("\"operator\":\"Exists\""));
        assert!(json.contains("\"effect\":\"NoExecute\""));
    }

    #[test]
    fn test_toleration_deserialize() {
        let json =
            r#"{"key":"test-key","operator":"Equal","value":"test-value","effect":"NoSchedule"}"#;
        let toleration: Toleration = serde_json::from_str(json).unwrap();

        assert_eq!(toleration.key, "test-key");
        assert_eq!(toleration.operator, Some(TolerationOperator::Equal));
        assert_eq!(toleration.value, "test-value");
        assert_eq!(toleration.effect, Some(TaintEffect::NoSchedule));
    }

    #[test]
    fn test_toleration_round_trip() {
        let original = Toleration {
            key: "key1".to_string(),
            operator: Some(TolerationOperator::Exists),
            effect: Some(TaintEffect::NoExecute),
            toleration_seconds: Some(300),
            ..Default::default()
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Toleration = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_toleration_without_value() {
        let toleration = Toleration {
            key: "key1".to_string(),
            operator: Some(TolerationOperator::Exists),
            value: String::new(),
            ..Default::default()
        };

        let json = serde_json::to_string(&toleration).unwrap();
        // Empty value should be omitted
        assert!(!json.contains("\"value\""));
    }

    // PodDNSConfig tests
    #[test]
    fn test_pod_dns_config_default() {
        let config = PodDNSConfig::default();
        assert!(config.nameservers.is_empty());
        assert!(config.searches.is_empty());
        assert!(config.options.is_empty());
    }

    #[test]
    fn test_pod_dns_config_with_fields() {
        let config = PodDNSConfig {
            nameservers: vec!["8.8.8.8".to_string(), "8.8.4.4".to_string()],
            searches: vec!["example.com".to_string()],
            options: vec![PodDNSConfigOption {
                name: "ndots".to_string(),
                value: Some("2".to_string()),
            }],
        };

        assert_eq!(config.nameservers.len(), 2);
        assert_eq!(config.searches.len(), 1);
        assert_eq!(config.options.len(), 1);
    }

    #[test]
    fn test_pod_dns_config_serialize() {
        let config = PodDNSConfig {
            nameservers: vec!["1.1.1.1".to_string()],
            ..Default::default()
        };

        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("\"nameservers\""));
        assert!(json.contains("\"1.1.1.1\""));
    }

    #[test]
    fn test_pod_dns_config_round_trip() {
        let original = PodDNSConfig {
            nameservers: vec!["8.8.8.8".to_string()],
            searches: vec!["default.svc.cluster.local".to_string()],
            options: vec![PodDNSConfigOption {
                name: "timeout".to_string(),
                value: Some("1".to_string()),
            }],
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: PodDNSConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
    }

    // PodDNSConfigOption tests
    #[test]
    fn test_pod_dns_config_option_default() {
        let option = PodDNSConfigOption::default();
        assert!(option.name.is_empty());
        assert!(option.value.is_none());
    }

    #[test]
    fn test_pod_dns_config_option_with_value() {
        let option = PodDNSConfigOption {
            name: "ndots".to_string(),
            value: Some("2".to_string()),
        };

        assert_eq!(option.name, "ndots");
        assert_eq!(option.value, Some("2".to_string()));
    }

    #[test]
    fn test_pod_dns_config_option_serialize() {
        let option = PodDNSConfigOption {
            name: "attempts".to_string(),
            value: Some("3".to_string()),
        };

        let json = serde_json::to_string(&option).unwrap();
        assert!(json.contains("\"name\":\"attempts\""));
        assert!(json.contains("\"value\":\"3\""));
    }

    #[test]
    fn test_pod_dns_config_option_without_value() {
        let option = PodDNSConfigOption {
            name: "debug".to_string(),
            value: None,
        };

        let json = serde_json::to_string(&option).unwrap();
        assert!(json.contains("\"name\":\"debug\""));
        // Empty value should be omitted
        assert!(!json.contains("\"value\""));
    }

    #[test]
    fn test_pod_dns_config_option_round_trip() {
        let original = PodDNSConfigOption {
            name: "rotate".to_string(),
            value: Some("true".to_string()),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: PodDNSConfigOption = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
    }

    // PodOS tests
    #[test]
    fn test_pod_os_default() {
        let os = PodOS::default();
        assert_eq!(os.name, OSName::Linux);
    }

    #[test]
    fn test_pod_os_with_windows() {
        let os = PodOS {
            name: OSName::Windows,
        };

        assert_eq!(os.name, OSName::Windows);
    }

    #[test]
    fn test_pod_os_serialize() {
        let os = PodOS {
            name: OSName::Linux,
        };

        let json = serde_json::to_string(&os).unwrap();
        assert!(json.contains("\"name\":\"linux\""));
    }

    #[test]
    fn test_pod_os_deserialize() {
        let json = r#"{"name":"windows"}"#;
        let os: PodOS = serde_json::from_str(json).unwrap();

        assert_eq!(os.name, OSName::Windows);
    }

    #[test]
    fn test_pod_os_round_trip() {
        let original = PodOS {
            name: OSName::Linux,
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: PodOS = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
    }

    // PodSchedulingGate tests
    #[test]
    fn test_pod_scheduling_gate_default() {
        let gate = PodSchedulingGate::default();
        assert!(gate.name.is_empty());
    }

    #[test]
    fn test_pod_scheduling_gate_with_name() {
        let gate = PodSchedulingGate {
            name: "scheduling-gate.example.com".to_string(),
        };

        assert_eq!(gate.name, "scheduling-gate.example.com");
    }

    #[test]
    fn test_pod_scheduling_gate_serialize() {
        let gate = PodSchedulingGate {
            name: "my-gate".to_string(),
        };

        let json = serde_json::to_string(&gate).unwrap();
        assert!(json.contains("\"name\":\"my-gate\""));
    }

    #[test]
    fn test_pod_scheduling_gate_deserialize() {
        let json = r#"{"name":"example-gate"}"#;
        let gate: PodSchedulingGate = serde_json::from_str(json).unwrap();

        assert_eq!(gate.name, "example-gate");
    }

    #[test]
    fn test_pod_scheduling_gate_round_trip() {
        let original = PodSchedulingGate {
            name: "test-gate".to_string(),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: PodSchedulingGate = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
    }

    // Integration tests
    #[test]
    fn test_multiple_tolerations() {
        let tolerations = vec![
            Toleration {
                key: "node.kubernetes.io/not-ready".to_string(),
                operator: Some(TolerationOperator::Exists),
                effect: Some(TaintEffect::NoExecute),
                ..Default::default()
            },
            Toleration {
                key: "node.kubernetes.io/memory-pressure".to_string(),
                operator: Some(TolerationOperator::Exists),
                effect: Some(TaintEffect::NoSchedule),
                ..Default::default()
            },
        ];

        assert_eq!(tolerations.len(), 2);
    }

    #[test]
    fn test_pod_dns_config_full() {
        let config = PodDNSConfig {
            nameservers: vec!["8.8.8.8".to_string(), "8.8.4.4".to_string()],
            searches: vec![
                "default.svc.cluster.local".to_string(),
                "svc.cluster.local".to_string(),
                "cluster.local".to_string(),
            ],
            options: vec![
                PodDNSConfigOption {
                    name: "ndots".to_string(),
                    value: Some("2".to_string()),
                },
                PodDNSConfigOption {
                    name: "timeout".to_string(),
                    value: Some("1".to_string()),
                },
            ],
        };

        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("\"nameservers\""));
        assert!(json.contains("\"searches\""));
        assert!(json.contains("\"options\""));
    }
}
