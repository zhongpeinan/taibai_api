//! Kubernetes Policy API v1 Types
//!
//! This module contains type definitions from k8s.io/api/policy/v1/types.go
//!
//! Source: https://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/api/policy/v1/types.go

use crate::common::meta::{Condition, LabelSelector};
use crate::common::time::Timestamp;
use crate::common::util::IntOrString;
use crate::common::{
    ApplyDefault, HasTypeMeta, ListMeta, ObjectMeta, ResourceSchema, TypeMeta,
    UnimplementedConversion, VersionedObject,
};
use crate::impl_unimplemented_prost_message;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// ============================================================================
// Constants
// ============================================================================

/// DisruptionBudgetCause is the status cause returned for eviction failures
/// caused by PodDisruptionBudget violations.
pub const DISRUPTION_BUDGET_CAUSE: &str = "DisruptionBudget";

// ============================================================================
// UnhealthyPodEvictionPolicyType
// ============================================================================

/// UnhealthyPodEvictionPolicyType defines the criteria for when unhealthy pods
/// should be considered for eviction.
///
/// Corresponds to [Kubernetes UnhealthyPodEvictionPolicyType](https://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/api/policy/v1/types.go#L80)
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
// Condition Constants
// ============================================================================

/// DisruptionAllowedCondition is a condition set by the disruption controller
/// that signal whether any of the pods covered by the PDB can be disrupted.
pub const DISRUPTION_ALLOWED_CONDITION: &str = "DisruptionAllowed";

/// SyncFailedReason is set on the DisruptionAllowed condition if reconcile
/// of the PDB failed and therefore disruption of pods are not allowed.
pub const SYNC_FAILED_REASON: &str = "SyncFailed";

/// SufficientPodsReason is set on the DisruptionAllowed condition if there are
/// more pods covered by the PDB than required and at least one can be disrupted.
pub const SUFFICIENT_PODS_REASON: &str = "SufficientPods";

/// InsufficientPodsReason is set on the DisruptionAllowed condition if the number
/// of pods are equal to or fewer than required by the PDB.
pub const INSUFFICIENT_PODS_REASON: &str = "InsufficientPods";

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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,

    /// DisruptedPods contains information about pods whose eviction was
    /// processed by the API server eviction subresource handler but has not
    /// yet been observed by the PodDisruptionBudget controller.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub disrupted_pods: BTreeMap<String, Timestamp>,

    /// Number of pod disruptions that are currently allowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disruptions_allowed: Option<i32>,

    /// current number of healthy pods
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_healthy: Option<i32>,

    /// minimum desired number of healthy pods
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub desired_healthy: Option<i32>,

    /// total number of pods counted by this disruption budget
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_pods: Option<i32>,

    /// Conditions contain conditions for PDB
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<Condition>,
}

// ============================================================================
// PodDisruptionBudget
// ============================================================================

/// PodDisruptionBudget is an object to define the max disruption that can be caused to a collection of pods.
///
/// Corresponds to [Kubernetes PodDisruptionBudget](https://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/api/policy/v1/types.go#L173)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodDisruptionBudget {
    /// TypeMeta describes the type of this object.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// Specification of the desired behavior of the PodDisruptionBudget.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<PodDisruptionBudgetSpec>,
    /// Most recently observed status of the PodDisruptionBudget.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PodDisruptionBudgetStatus>,
}

// ============================================================================
// PodDisruptionBudgetList
// ============================================================================

/// PodDisruptionBudgetList is a collection of PodDisruptionBudgets.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodDisruptionBudgetList {
    /// TypeMeta describes the type of this object.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,
    /// Items is a list of PodDisruptionBudgets.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
/// Corresponds to [Kubernetes Eviction](https://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/api/policy/v1/types.go#L210)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Eviction {
    /// TypeMeta describes the type of this object.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// ObjectMeta describes the pod that is being evicted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// DeleteOptions may be provided
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete_options: Option<DeleteOptions>,
}

/// DeleteOptions represents options for deleting a resource.
///
/// This is a simplified version suitable for the Eviction API.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeleteOptions {
    /// GracePeriodSeconds is the duration in seconds before the object should be deleted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grace_period_seconds: Option<i64>,

    /// Preconditions must be fulfilled before the operation can be executed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preconditions: Option<Preconditions>,

    /// PropagationPolicy determines whether and how garbage collection will be performed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub propagation_policy: Option<String>,

    /// DryRun will cause the request to be executed without persisting the resource.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dry_run: Vec<String>,

    /// When present, indicates that modifications should not be persisted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run_all: Option<bool>,
}

/// Preconditions must be fulfilled before an operation can be executed.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Preconditions {
    /// UID specifies the UID of the resource being operated on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,

    /// ResourceVersion specifies the version of the resource being operated on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_version: Option<String>,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disruption_budget_cause_constant() {
        assert_eq!(DISRUPTION_BUDGET_CAUSE, "DisruptionBudget");
    }

    #[test]
    fn test_unhealthy_pod_eviction_policy_type_constants() {
        assert_eq!(
            unhealthy_pod_eviction_policy_type::IF_HEALTHY_BUDGET,
            "IfHealthyBudget"
        );
        assert_eq!(
            unhealthy_pod_eviction_policy_type::ALWAYS_ALLOW,
            "AlwaysAllow"
        );
    }

    #[test]
    fn test_condition_constants() {
        assert_eq!(DISRUPTION_ALLOWED_CONDITION, "DisruptionAllowed");
        assert_eq!(SYNC_FAILED_REASON, "SyncFailed");
        assert_eq!(SUFFICIENT_PODS_REASON, "SufficientPods");
        assert_eq!(INSUFFICIENT_PODS_REASON, "InsufficientPods");
    }

    #[test]
    fn test_unhealthy_pod_eviction_policy_type_serialize() {
        let policy = UnhealthyPodEvictionPolicyType::IfHealthyBudget;
        let json = serde_json::to_string(&policy).unwrap();
        assert_eq!(json, r#""IfHealthyBudget""#);
    }

    #[test]
    fn test_unhealthy_pod_eviction_policy_type_deserialize() {
        let json = r#""AlwaysAllow""#;
        let policy: UnhealthyPodEvictionPolicyType = serde_json::from_str(json).unwrap();
        assert!(matches!(
            policy,
            UnhealthyPodEvictionPolicyType::AlwaysAllow
        ));
    }

    #[test]
    fn test_pod_disruption_budget_spec_default() {
        let spec = PodDisruptionBudgetSpec::default();
        assert!(spec.min_available.is_none());
        assert!(spec.selector.is_none());
        assert!(spec.max_unavailable.is_none());
        assert!(spec.unhealthy_pod_eviction_policy.is_none());
    }

    #[test]
    fn test_pod_disruption_budget_spec_with_min_available() {
        let spec = PodDisruptionBudgetSpec {
            min_available: Some(IntOrString::String("50%".to_string())),
            ..Default::default()
        };
        assert!(spec.min_available.is_some());
    }

    #[test]
    fn test_pod_disruption_budget_status_default() {
        let status = PodDisruptionBudgetStatus::default();
        assert!(status.observed_generation.is_none());
        assert!(status.disrupted_pods.is_empty());
        assert!(status.disruptions_allowed.is_none());
        assert!(status.current_healthy.is_none());
        assert!(status.desired_healthy.is_none());
        assert!(status.expected_pods.is_none());
        assert!(status.conditions.is_empty());
    }

    #[test]
    fn test_pod_disruption_budget_default() {
        let pdb = PodDisruptionBudget::default();
        assert!(pdb.metadata.is_none());
        assert!(pdb.spec.is_none());
        assert!(pdb.status.is_none());
    }

    #[test]
    fn test_pod_disruption_budget_serialize() {
        let pdb = PodDisruptionBudget {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("my-pdb".to_string()),
                ..Default::default()
            }),
            spec: Some(PodDisruptionBudgetSpec {
                min_available: Some(IntOrString::String("3".to_string())),
                ..Default::default()
            }),
            status: None,
        };
        let json = serde_json::to_string(&pdb).unwrap();
        assert!(json.contains(r#""name":"my-pdb""#));
        assert!(json.contains(r#""minAvailable""#));
    }

    #[test]
    fn test_pod_disruption_budget_deserialize() {
        let json = r#"{
            "kind": "PodDisruptionBudget",
            "metadata": {"name": "my-pdb"},
            "spec": {
                "minAvailable": "3"
            }
        }"#;
        let pdb: PodDisruptionBudget = serde_json::from_str(json).unwrap();
        assert_eq!(pdb.metadata.unwrap().name.unwrap(), "my-pdb");
        assert!(pdb.spec.is_some());
    }

    #[test]
    fn test_eviction_default() {
        let eviction = Eviction::default();
        assert!(eviction.metadata.is_none());
        assert!(eviction.delete_options.is_none());
    }

    #[test]
    fn test_eviction_with_metadata() {
        let eviction = Eviction {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("my-pod".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            delete_options: None,
        };
        assert_eq!(
            eviction.metadata.as_ref().unwrap().name.as_ref().unwrap(),
            "my-pod"
        );
    }

    #[test]
    fn test_eviction_serialize() {
        let eviction = Eviction {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("my-pod".to_string()),
                ..Default::default()
            }),
            delete_options: Some(DeleteOptions {
                grace_period_seconds: Some(30),
                ..Default::default()
            }),
        };
        let json = serde_json::to_string(&eviction).unwrap();
        assert!(json.contains(r#""name":"my-pod""#));
        assert!(json.contains(r#""gracePeriodSeconds":30"#));
    }

    #[test]
    fn test_eviction_deserialize() {
        let json = r#"{
            "kind": "Eviction",
            "metadata": {"name": "my-pod"},
            "deleteOptions": {
                "gracePeriodSeconds": 30
            }
        }"#;
        let eviction: Eviction = serde_json::from_str(json).unwrap();
        assert_eq!(eviction.metadata.unwrap().name.unwrap(), "my-pod");
        assert_eq!(
            eviction
                .delete_options
                .unwrap()
                .grace_period_seconds
                .unwrap(),
            30
        );
    }

    #[test]
    fn test_delete_options_default() {
        let opts = DeleteOptions::default();
        assert!(opts.grace_period_seconds.is_none());
        assert!(opts.preconditions.is_none());
        assert!(opts.propagation_policy.is_none());
        assert!(opts.dry_run.is_empty());
    }

    #[test]
    fn test_delete_options_with_grace_period() {
        let opts = DeleteOptions {
            grace_period_seconds: Some(30),
            ..Default::default()
        };
        assert_eq!(opts.grace_period_seconds.unwrap(), 30);
    }

    #[test]
    fn test_preconditions_default() {
        let preconds = Preconditions::default();
        assert!(preconds.uid.is_none());
        assert!(preconds.resource_version.is_none());
    }

    #[test]
    fn test_pod_disruption_budget_list_default() {
        let list = PodDisruptionBudgetList::default();
        assert!(list.metadata.is_none());
        assert!(list.items.is_empty());
    }

    #[test]
    fn test_pod_disruption_budget_list_serialize() {
        let list = PodDisruptionBudgetList {
            type_meta: TypeMeta::default(),
            metadata: Some(ListMeta {
                resource_version: Some("1".to_string()),
                ..Default::default()
            }),
            items: vec![PodDisruptionBudget {
                type_meta: TypeMeta {
                    kind: "PodDisruptionBudget".to_string(),
                    ..Default::default()
                },
                metadata: Some(ObjectMeta {
                    name: Some("pdb-1".to_string()),
                    ..Default::default()
                }),
                spec: None,
                status: None,
            }],
        };
        let json = serde_json::to_string(&list).unwrap();
        assert!(json.contains(r#""resourceVersion":"1""#));
        assert!(json.contains(r#""name":"pdb-1""#));
    }

    #[test]
    fn test_pod_disruption_budget_status_with_conditions() {
        let status = PodDisruptionBudgetStatus {
            conditions: vec![Condition {
                type_: "DisruptionAllowed".to_string(),
                status: "True".to_string(),
                observed_generation: None,
                last_transition_time: None,
                reason: Some("SufficientPods".to_string()),
                message: None,
            }],
            ..Default::default()
        };
        assert_eq!(status.conditions.len(), 1);
        assert_eq!(status.conditions[0].type_, "DisruptionAllowed");
    }

    #[test]
    fn test_pod_disruption_budget_spec_with_unhealthy_policy() {
        let spec = PodDisruptionBudgetSpec {
            unhealthy_pod_eviction_policy: Some(UnhealthyPodEvictionPolicyType::AlwaysAllow),
            ..Default::default()
        };
        assert!(spec.unhealthy_pod_eviction_policy.is_some());
        let json = serde_json::to_string(&spec).unwrap();
        assert!(json.contains(r#""unhealthyPodEvictionPolicy":"AlwaysAllow""#));
    }
}

// ============================================================================
// Trait Implementations for Policy Resources
// ============================================================================

// ----------------------------------------------------------------------------
// ResourceSchema Implementation
// ----------------------------------------------------------------------------

impl ResourceSchema for PodDisruptionBudget {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "policy"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "PodDisruptionBudget"
    }
    fn resource(_: &Self::Meta) -> &str {
        "poddisruptionbudgets"
    }

    fn group_static() -> &'static str {
        "policy"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "PodDisruptionBudget"
    }
    fn resource_static() -> &'static str {
        "poddisruptionbudgets"
    }
}

impl ResourceSchema for PodDisruptionBudgetList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "policy"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "PodDisruptionBudgetList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "poddisruptionbudgets"
    }

    fn group_static() -> &'static str {
        "policy"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "PodDisruptionBudgetList"
    }
    fn resource_static() -> &'static str {
        "poddisruptionbudgets"
    }
}

impl ResourceSchema for Eviction {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "policy"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "Eviction"
    }
    fn resource(_: &Self::Meta) -> &str {
        "evictions"
    }

    fn group_static() -> &'static str {
        "policy"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "Eviction"
    }
    fn resource_static() -> &'static str {
        "evictions"
    }
}

// ----------------------------------------------------------------------------
// HasTypeMeta Implementation
// ----------------------------------------------------------------------------

impl HasTypeMeta for PodDisruptionBudget {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for PodDisruptionBudgetList {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for Eviction {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

// ----------------------------------------------------------------------------
// VersionedObject Implementation
// ----------------------------------------------------------------------------

impl VersionedObject for PodDisruptionBudget {
    fn metadata(&self) -> &ObjectMeta {
        use std::sync::OnceLock;
        self.metadata.as_ref().unwrap_or_else(|| {
            static DEFAULT: OnceLock<ObjectMeta> = OnceLock::new();
            DEFAULT.get_or_init(ObjectMeta::default)
        })
    }

    fn metadata_mut(&mut self) -> &mut ObjectMeta {
        self.metadata.get_or_insert_with(ObjectMeta::default)
    }
}

impl VersionedObject for Eviction {
    fn metadata(&self) -> &ObjectMeta {
        use std::sync::OnceLock;
        self.metadata.as_ref().unwrap_or_else(|| {
            static DEFAULT: OnceLock<ObjectMeta> = OnceLock::new();
            DEFAULT.get_or_init(ObjectMeta::default)
        })
    }

    fn metadata_mut(&mut self) -> &mut ObjectMeta {
        self.metadata.get_or_insert_with(ObjectMeta::default)
    }
}

// ----------------------------------------------------------------------------
// ApplyDefaults Implementation
// ----------------------------------------------------------------------------

impl ApplyDefault for PodDisruptionBudget {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "policy/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "PodDisruptionBudget".to_string();
        }
    }
}

impl ApplyDefault for PodDisruptionBudgetList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "policy/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "PodDisruptionBudgetList".to_string();
        }
    }
}

impl ApplyDefault for Eviction {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "policy/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "Eviction".to_string();
        }
    }
}

// ----------------------------------------------------------------------------
// Version Conversion Placeholder
// ----------------------------------------------------------------------------

impl UnimplementedConversion for PodDisruptionBudget {}
impl UnimplementedConversion for PodDisruptionBudgetList {}
impl UnimplementedConversion for Eviction {}

// ----------------------------------------------------------------------------
// Protobuf Placeholder
// ----------------------------------------------------------------------------

impl_unimplemented_prost_message!(PodDisruptionBudget);
impl_unimplemented_prost_message!(PodDisruptionBudgetList);
impl_unimplemented_prost_message!(Eviction);
