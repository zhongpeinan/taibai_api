//! Kubernetes Toleration types
//!
//! This module contains toleration-related types from the Kubernetes core/v1 API.
//! These types allow Pods to schedule onto nodes with matching taints.

use serde::{Deserialize, Serialize};

/// Toleration represents a toleration for a node taint.
///
/// Tolerations allow Pods to be scheduled onto nodes with matching taints.
/// A toleration matches a taint if the keys are the same and the effects are the same,
/// and the operator is "Exists" (ignoring value) or "Equal" (values must match).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Toleration {
    /// Key is the taint key that the toleration applies to.
    /// Empty means match all taint keys.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub key: String,
    /// Operator represents a key's relationship to the value.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub operator: TolerationOperator,
    /// Value is the taint value the toleration matches to.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub value: String,
    /// Effect indicates the taint effect to match.
    /// Empty means match all taint effects.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub effect: String,
    /// TolerationSeconds is the period of time the toleration (which must be
    /// of effect NoExecute) will remain. The default value is 0, meaning the
    /// toleration will never be removed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub toleration_seconds: Option<i64>,
}

/// TolerationOperator is the operator for a toleration.
pub type TolerationOperator = String;

/// Constants for TolerationOperator values
pub mod toleration_operator {
    /// Exists operator - value must be empty; matches all values
    pub const EXISTS: &str = "Exists";

    /// Equal operator - value must equal the taint value
    pub const EQUAL: &str = "Equal";
}

/// Constants for Toleration effect values (common taint effects)
pub mod toleration_effect {
    /// Do not allow new pods to schedule onto the node unless they have a matching toleration
    pub const NO_SCHEDULE: &str = "NoSchedule";

    /// Prefer not to schedule new pods onto the node unless they have a matching toleration
    pub const PREFER_NO_SCHEDULE: &str = "PreferNoSchedule";

    /// Evict any already-running pods that do not have a matching toleration
    pub const NO_EXECUTE: &str = "NoExecute";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toleration_default() {
        let toleration = Toleration::default();
        assert!(toleration.key.is_empty());
        assert!(toleration.operator.is_empty());
        assert!(toleration.value.is_empty());
        assert!(toleration.effect.is_empty());
        assert!(toleration.toleration_seconds.is_none());
    }

    #[test]
    fn test_toleration_with_equal_operator() {
        let toleration = Toleration {
            key: "key1".to_string(),
            operator: toleration_operator::EQUAL.to_string(),
            value: "value1".to_string(),
            effect: toleration_effect::NO_SCHEDULE.to_string(),
            toleration_seconds: None,
        };

        assert_eq!(toleration.key, "key1");
        assert_eq!(toleration.operator, "Equal");
        assert_eq!(toleration.value, "value1");
        assert_eq!(toleration.effect, "NoSchedule");
        assert!(toleration.toleration_seconds.is_none());
    }

    #[test]
    fn test_toleration_with_exists_operator() {
        let toleration = Toleration {
            key: "node.kubernetes.io/not-ready".to_string(),
            operator: toleration_operator::EXISTS.to_string(),
            value: String::new(),
            effect: toleration_effect::NO_EXECUTE.to_string(),
            toleration_seconds: Some(300),
        };

        assert_eq!(toleration.key, "node.kubernetes.io/not-ready");
        assert_eq!(toleration.operator, "Exists");
        assert!(toleration.value.is_empty());
        assert_eq!(toleration.effect, "NoExecute");
        assert_eq!(toleration.toleration_seconds, Some(300));
    }

    #[test]
    fn test_toleration_serialization() {
        let toleration = Toleration {
            key: "test-key".to_string(),
            operator: toleration_operator::EQUAL.to_string(),
            value: "test-value".to_string(),
            effect: toleration_effect::PREFER_NO_SCHEDULE.to_string(),
            toleration_seconds: None,
        };

        let json = serde_json::to_string(&toleration).unwrap();
        let deserialized: Toleration = serde_json::from_str(&json).unwrap();

        assert_eq!(toleration.key, deserialized.key);
        assert_eq!(toleration.operator, deserialized.operator);
        assert_eq!(toleration.value, deserialized.value);
        assert_eq!(toleration.effect, deserialized.effect);
    }

    #[test]
    fn test_toleration_with_toleration_seconds() {
        let toleration = Toleration {
            key: "example.com/taint".to_string(),
            operator: toleration_operator::EQUAL.to_string(),
            value: "true".to_string(),
            effect: toleration_effect::NO_EXECUTE.to_string(),
            toleration_seconds: Some(3600),
        };

        let json = serde_json::to_string(&toleration).unwrap();
        let deserialized: Toleration = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.toleration_seconds, Some(3600));
    }

    #[test]
    fn test_toleration_operator_constants() {
        assert_eq!(toleration_operator::EXISTS, "Exists");
        assert_eq!(toleration_operator::EQUAL, "Equal");
    }

    #[test]
    fn test_toleration_effect_constants() {
        assert_eq!(toleration_effect::NO_SCHEDULE, "NoSchedule");
        assert_eq!(toleration_effect::PREFER_NO_SCHEDULE, "PreferNoSchedule");
        assert_eq!(toleration_effect::NO_EXECUTE, "NoExecute");
    }

    #[test]
    fn test_toleration_match_all_taints() {
        // A toleration with empty key and effect matches all taints
        let toleration = Toleration {
            key: String::new(),
            operator: toleration_operator::EXISTS.to_string(),
            value: String::new(),
            effect: String::new(),
            toleration_seconds: None,
        };

        assert!(toleration.key.is_empty());
        assert!(toleration.effect.is_empty());
        assert_eq!(toleration.operator, "Exists");
    }

    #[test]
    fn test_toleration_round_trip() {
        let toleration = Toleration {
            key: "dedicated".to_string(),
            operator: toleration_operator::EQUAL.to_string(),
            value: "gpu".to_string(),
            effect: toleration_effect::NO_SCHEDULE.to_string(),
            toleration_seconds: None,
        };

        let json = serde_json::to_string(&toleration).unwrap();
        let deserialized: Toleration = serde_json::from_str(&json).unwrap();

        assert_eq!(toleration, deserialized);
    }

    #[test]
    fn test_toleration_skip_empty_fields() {
        let toleration = Toleration {
            key: "test".to_string(),
            operator: toleration_operator::EXISTS.to_string(),
            value: String::new(),
            effect: toleration_effect::NO_EXECUTE.to_string(),
            toleration_seconds: None,
        };

        let json = serde_json::to_string(&toleration).unwrap();
        // Empty value should not be in JSON due to skip_serializing_if
        assert!(!json.contains("\"value\""));
        // Empty tolerationSeconds should not be in JSON
        assert!(!json.contains("tolerationSeconds"));

        let deserialized: Toleration = serde_json::from_str(&json).unwrap();
        assert!(deserialized.value.is_empty());
        assert!(deserialized.toleration_seconds.is_none());
    }

    #[test]
    fn test_toleration_with_prefer_no_schedule() {
        let toleration = Toleration {
            key: "spot-instance".to_string(),
            operator: toleration_operator::EXISTS.to_string(),
            value: String::new(),
            effect: toleration_effect::PREFER_NO_SCHEDULE.to_string(),
            toleration_seconds: None,
        };

        let json = serde_json::to_string(&toleration).unwrap();
        let deserialized: Toleration = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.effect, "PreferNoSchedule");
    }
}
