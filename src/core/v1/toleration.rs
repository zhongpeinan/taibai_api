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
}
