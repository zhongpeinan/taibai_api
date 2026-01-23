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
}
