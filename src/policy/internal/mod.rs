//! Kubernetes Policy API Internal Types
//!
//! This module contains type definitions from k8s.io/kubernetes/pkg/apis/policy/types.go
//! that are used internally by the Kubernetes API.
//!
//! Source: https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/policy/types.go

use crate::common::meta::{Condition, LabelSelector};
use crate::common::time::Timestamp;
use crate::common::util::IntOrString;
use crate::common::{ListMeta, ObjectMeta, TypeMeta};
use crate::impl_has_object_meta;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// ============================================================================
// UnhealthyPodEvictionPolicyType
// ============================================================================

/// UnhealthyPodEvictionPolicyType defines the criteria for when unhealthy pods
/// should be considered for eviction.
///
/// Corresponds to [Kubernetes UnhealthyPodEvictionPolicyType](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/policy/types.go#L71)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum UnhealthyPodEvictionPolicyType {
    /// IfHealthyBudget policy means that running pods (status.phase="Running"),
    /// but not yet healthy can be evicted only if the guarded application is not
    /// disrupted (status.currentHealthy is at least equal to status.desiredHealthy).
    /// Healthy pods will be subject to the PDB for eviction.
    #[serde(rename = "IfHealthyBudget")]
    IfHealthyBudget,

    /// AlwaysAllow policy means that all running pods (status.phase="Running"),
    /// but not yet healthy are considered disrupted and can be evicted regardless
    /// of whether the criteria in a PDB is met. This means perspective running
    /// pods of a disrupted application might not get a chance to become healthy.
    /// Healthy pods will be subject to the PDB for eviction.
    #[serde(rename = "AlwaysAllow")]
    AlwaysAllow,
}

/// UnhealthyPodEvictionPolicyType constants
pub mod unhealthy_pod_eviction_policy_type {
    pub const IF_HEALTHY_BUDGET: &str = "IfHealthyBudget";
    pub const ALWAYS_ALLOW: &str = "AlwaysAllow";
}

// ============================================================================
// PodDisruptionBudgetSpec
// ============================================================================

/// PodDisruptionBudgetSpec is a description of a PodDisruptionBudget.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodDisruptionBudgetSpec {
    /// An eviction is allowed if at least "minAvailable" pods selected by
    /// "selector" will still be available after the eviction, i.e. even in the
    /// absence of the evicted pod. So for example you can prevent all voluntary
    /// evictions by specifying "100%".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_available: Option<IntOrString>,

    /// Label query over pods whose evictions are managed by the disruption budget.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<LabelSelector>,

    /// An eviction is allowed if at most "maxUnavailable" pods selected by
    /// "selector" are unavailable after the eviction, i.e. even in absence
    /// of the evicted pod. For example, one can prevent all voluntary evictions
    /// by specifying 0. This is a mutually exclusive setting with "minAvailable".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_unavailable: Option<IntOrString>,

    /// UnhealthyPodEvictionPolicy defines the criteria for when unhealthy pods
    /// should be considered for eviction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unhealthy_pod_eviction_policy: Option<UnhealthyPodEvictionPolicyType>,
}

// ============================================================================
// PodDisruptionBudgetStatus
// ============================================================================

/// PodDisruptionBudgetStatus represents information about the status of a
/// PodDisruptionBudget. Status may trail the actual state of a system.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodDisruptionBudgetStatus {
    /// Most recent generation observed when updating this PDB status. DisruptionsAllowed and other
    /// status information is valid only if observedGeneration equals to PDB's object generation.
    #[serde(default, skip_serializing_if = "crate::common::util::is_zero_i64")]
    pub observed_generation: i64,

    /// DisruptedPods contains information about pods whose eviction was
    /// processed by the API server eviction subresource handler but has not
    /// yet been observed by the PodDisruptionBudget controller.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub disrupted_pods: BTreeMap<String, Timestamp>,

    /// Number of pod disruptions that are currently allowed.
    #[serde(default)]
    pub disruptions_allowed: i32,

    /// current number of healthy pods
    #[serde(default)]
    pub current_healthy: i32,

    /// minimum desired number of healthy pods
    #[serde(default)]
    pub desired_healthy: i32,

    /// total number of pods counted by this disruption budget
    #[serde(default)]
    pub expected_pods: i32,

    /// Conditions contain conditions for PDB
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<Condition>,
}

// ============================================================================
// PodDisruptionBudget
// ============================================================================

/// PodDisruptionBudget is an object to define the max disruption that can be caused to a collection of pods.
///
/// Corresponds to [Kubernetes PodDisruptionBudget](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/policy/types.go#L130)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct PodDisruptionBudget {
    /// TypeMeta describes the type of this object.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    pub metadata: ObjectMeta,
    /// Specification of the desired behavior of the PodDisruptionBudget.
    pub spec: PodDisruptionBudgetSpec,
    /// Most recently observed status of the PodDisruptionBudget.
    pub status: PodDisruptionBudgetStatus,
}
impl_has_object_meta!(PodDisruptionBudget);

// ============================================================================
// PodDisruptionBudgetList
// ============================================================================

/// PodDisruptionBudgetList is a collection of PodDisruptionBudgets.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct PodDisruptionBudgetList {
    /// TypeMeta describes the type of this object.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    pub metadata: ListMeta,
    /// Items is a list of PodDisruptionBudgets.
    pub items: Vec<PodDisruptionBudget>,
}

// ============================================================================
// Eviction
// ============================================================================

/// Eviction evicts a pod from its node subject to certain policies and safety constraints.
///
/// This is a subresource of Pod. A request to cause such an eviction is
/// created by POSTing to .../pods/<pod name>/eviction.
///
/// Corresponds to [Kubernetes Eviction](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/policy/types.go#L157)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Eviction {
    /// TypeMeta describes the type of this object.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// ObjectMeta describes the pod that is being evicted.
    pub metadata: ObjectMeta,
    /// DeleteOptions may be provided.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete_options: Option<ObjectMeta>,
}
impl_has_object_meta!(Eviction);

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {}
