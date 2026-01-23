//! AdmissionRegistration internal API types
//!
//! This module contains the internal API types for admissionregistration.

use crate::admissionregistration::v1::{
    FailurePolicyType, MatchCondition, MatchResources, ParamKind, ParamRef, ReinvocationPolicyType,
    Variable,
};
use crate::common::{ListMeta, ObjectMeta};
use crate::impl_has_object_meta;
use serde::{Deserialize, Serialize};

// ============================================================================
// Patch Types (Union Type)
// ============================================================================

/// PatchType specifies the type of patch operation for a mutation.
///
/// Corresponds to [Kubernetes PatchType](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/admissionregistration/types.go#L1308)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "PascalCase")]
pub enum PatchType {
    /// ApplyConfiguration indicates that the mutation is using apply configuration.
    #[default]
    ApplyConfiguration,
    /// JSONPatch indicates that the object is mutated through JSON Patch.
    JSONPatch,
}

/// PatchType constants
pub mod patch_type {
    pub const APPLY_CONFIGURATION: &str = "ApplyConfiguration";
    pub const JSON_PATCH: &str = "JSONPatch";
}

/// ApplyConfiguration defines the desired configuration values of an object.
///
/// Corresponds to [Kubernetes ApplyConfiguration](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/admissionregistration/types.go#L1319)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ApplyConfiguration {
    /// expression will be evaluated by CEL to create an apply configuration.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub expression: String,
}

/// JSONPatch defines a JSON Patch.
///
/// Corresponds to [Kubernetes JSONPatch](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/admissionregistration/types.go#L1363)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct JSONPatch {
    /// expression will be evaluated by CEL to create a JSON patch.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub expression: String,
}

/// Mutation specifies the operation that performs a Mutation.
/// This is a union type where patchType determines which variant is active.
///
/// Corresponds to [Kubernetes Mutation](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/admissionregistration/types.go#L1288)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(tag = "patchType", rename_all = "PascalCase")]
pub enum Mutation {
    /// ApplyConfiguration defines the desired configuration values of an object.
    ApplyConfiguration(ApplyConfiguration),
    /// JSONPatch defines a JSON Patch.
    JSONPatch(JSONPatch),
}

// ============================================================================
// MutatingAdmissionPolicy Types
// ============================================================================

/// MutatingAdmissionPolicySpec is the specification of the desired behavior of the admission policy.
///
/// Corresponds to [Kubernetes MutatingAdmissionPolicySpec](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/admissionregistration/types.go#L1200)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MutatingAdmissionPolicySpec {
    /// paramKind specifies the kind of resources used to parameterize this policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub param_kind: Option<ParamKind>,

    /// matchConstraints specifies what resources this policy is designed to validate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_constraints: Option<MatchResources>,

    /// variables contain definitions of variables that can be used in composition of other expressions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub variables: Vec<Variable>,

    /// mutations contain operations to perform on matching objects.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mutations: Vec<Mutation>,

    /// failurePolicy defines how to handle failures for the admission policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_policy: Option<FailurePolicyType>,

    /// matchConditions is a list of conditions that must be met for a request to be validated.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub match_conditions: Vec<MatchCondition>,

    /// reinvocationPolicy indicates whether mutations may be called multiple times.
    #[serde(default)]
    pub reinvocation_policy: ReinvocationPolicyType,
}

/// MutatingAdmissionPolicy describes an admission policy that may mutate an object.
///
/// Corresponds to [Kubernetes MutatingAdmissionPolicy](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/admissionregistration/types.go#L1176)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MutatingAdmissionPolicy {
    /// Standard object's metadata.
    pub metadata: ObjectMeta,

    /// Specification of the desired behavior of the MutatingAdmissionPolicy.
    #[serde(default)]
    pub spec: MutatingAdmissionPolicySpec,
}
impl_has_object_meta!(MutatingAdmissionPolicy);

/// MutatingAdmissionPolicyList is a list of MutatingAdmissionPolicy.
///
/// Corresponds to [Kubernetes MutatingAdmissionPolicyList](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/admissionregistration/types.go#L1189)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MutatingAdmissionPolicyList {
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// List of MutatingAdmissionPolicy.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<MutatingAdmissionPolicy>,
}

// ============================================================================
// MutatingAdmissionPolicyBinding Types
// ============================================================================

/// MutatingAdmissionPolicyBindingSpec is the specification of the MutatingAdmissionPolicyBinding.
///
/// Corresponds to [Kubernetes MutatingAdmissionPolicyBindingSpec](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/admissionregistration/types.go#L1472)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MutatingAdmissionPolicyBindingSpec {
    /// policyName references a MutatingAdmissionPolicy name.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub policy_name: String,

    /// paramRef specifies the parameter resource used to configure the admission control policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub param_ref: Option<ParamRef>,

    /// matchResources limits what resources match this binding.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_resources: Option<MatchResources>,
}

/// MutatingAdmissionPolicyBinding binds the MutatingAdmissionPolicy with parametrized resources.
///
/// Corresponds to [Kubernetes MutatingAdmissionPolicyBinding](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/admissionregistration/types.go#L1438)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MutatingAdmissionPolicyBinding {
    /// Standard object's metadata.
    pub metadata: ObjectMeta,

    /// Specification of the desired behavior of the MutatingAdmissionPolicyBinding.
    #[serde(default)]
    pub spec: MutatingAdmissionPolicyBindingSpec,
}
impl_has_object_meta!(MutatingAdmissionPolicyBinding);

/// MutatingAdmissionPolicyBindingList is a list of MutatingAdmissionPolicyBinding.
///
/// Corresponds to [Kubernetes MutatingAdmissionPolicyBindingList](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/admissionregistration/types.go#L1461)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MutatingAdmissionPolicyBindingList {
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// List of PolicyBinding.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<MutatingAdmissionPolicyBinding>,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
}
