//! AdmissionRegistration v1beta1 API types
//!
//! This module contains the AdmissionRegistration v1beta1 API types.

use crate::admissionregistration::v1::{
    FailurePolicyType, MatchCondition, MatchResources, ParamKind, ParamRef, ReinvocationPolicyType,
    Variable,
};
use crate::common::{
    ApplyDefault, ListMeta, ObjectMeta, ResourceSchema, TypeMeta, UnimplementedConversion,
    VersionedObject,
};
use crate::impl_unimplemented_prost_message;
use serde::{Deserialize, Serialize};

// ============================================================================
// Patch Types
// ============================================================================

/// PatchType specifies the type of patch operation for a mutation.
///
/// Corresponds to [Kubernetes PatchType](https://github.com/kubernetes/api/blob/master/admissionregistration/v1beta1/types.go#L1338)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum PatchType {
    /// ApplyConfiguration indicates that the mutation is using apply configuration to mutate the object.
    #[serde(rename = "ApplyConfiguration")]
    #[default]
    ApplyConfiguration,
    /// JSONPatch indicates that the object is mutated through JSON Patch.
    #[serde(rename = "JSONPatch")]
    JSONPatch,
}

/// PatchType constants
pub mod patch_type {
    pub const APPLY_CONFIGURATION: &str = "ApplyConfiguration";
    pub const JSON_PATCH: &str = "JSONPatch";
}

/// ApplyConfiguration defines the desired configuration values of an object.
///
/// Corresponds to [Kubernetes ApplyConfiguration](https://github.com/kubernetes/api/blob/master/admissionregistration/v1beta1/types.go#L1349)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ApplyConfiguration {
    /// expression will be evaluated by CEL to create an apply configuration.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub expression: String,
}

/// JSONPatch defines a JSON Patch.
///
/// Corresponds to [Kubernetes JSONPatch](https://github.com/kubernetes/api/blob/master/admissionregistration/v1beta1/types.go#L1394)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct JSONPatch {
    /// expression will be evaluated by CEL to create a JSON patch.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub expression: String,
}

/// Mutation specifies the CEL expression which is used to apply the Mutation.
///
/// Corresponds to [Kubernetes Mutation](https://github.com/kubernetes/api/blob/master/admissionregistration/v1beta1/types.go#L1318)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Mutation {
    /// patchType indicates the patch strategy used.
    /// Allowed values are "ApplyConfiguration" and "JSONPatch".
    /// Required.
    #[serde(default)]
    pub patch_type: PatchType,

    /// applyConfiguration defines the desired configuration values of an object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apply_configuration: Option<ApplyConfiguration>,

    /// jsonPatch defines a JSON patch operation to perform a mutation to the object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub json_patch: Option<JSONPatch>,
}

// ============================================================================
// MutatingAdmissionPolicy Types
// ============================================================================

/// MutatingAdmissionPolicySpec is the specification of the desired behavior of the admission policy.
///
/// Corresponds to [Kubernetes MutatingAdmissionPolicySpec](https://github.com/kubernetes/api/blob/master/admissionregistration/v1beta1/types.go#L1230)
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

/// MutatingAdmissionPolicy describes an admission policy that mutates the object coming into admission chain.
///
/// Corresponds to [Kubernetes MutatingAdmissionPolicy](https://github.com/kubernetes/api/blob/master/admissionregistration/v1beta1/types.go#L1207)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MutatingAdmissionPolicy {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    #[serde(default)]
    pub metadata: ObjectMeta,

    /// Specification of the desired behavior of the MutatingAdmissionPolicy.
    #[serde(default)]
    pub spec: MutatingAdmissionPolicySpec,
}

/// MutatingAdmissionPolicyList is a list of MutatingAdmissionPolicy.
///
/// Corresponds to [Kubernetes MutatingAdmissionPolicyList](https://github.com/kubernetes/api/blob/master/admissionregistration/v1beta1/types.go#L1219)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MutatingAdmissionPolicyList {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(default)]
    pub metadata: ListMeta,

    /// List of MutatingAdmissionPolicy.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<MutatingAdmissionPolicy>,
}

// ============================================================================
// MutatingAdmissionPolicyBinding Types
// ============================================================================

/// MutatingAdmissionPolicyBindingSpec is the specification of the MutatingAdmissionPolicyBinding.
///
/// Corresponds to [Kubernetes MutatingAdmissionPolicyBindingSpec](https://github.com/kubernetes/api/blob/master/admissionregistration/v1beta1/types.go#L1503)
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
/// Corresponds to [Kubernetes MutatingAdmissionPolicyBinding](https://github.com/kubernetes/api/blob/master/admissionregistration/v1beta1/types.go#L1480)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MutatingAdmissionPolicyBinding {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    #[serde(default)]
    pub metadata: ObjectMeta,

    /// Specification of the desired behavior of the MutatingAdmissionPolicyBinding.
    #[serde(default)]
    pub spec: MutatingAdmissionPolicyBindingSpec,
}

/// MutatingAdmissionPolicyBindingList is a list of MutatingAdmissionPolicyBinding.
///
/// Corresponds to [Kubernetes MutatingAdmissionPolicyBindingList](https://github.com/kubernetes/api/blob/master/admissionregistration/v1beta1/types.go#L1492)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MutatingAdmissionPolicyBindingList {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(default)]
    pub metadata: ListMeta,

    /// List of MutatingAdmissionPolicyBinding.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<MutatingAdmissionPolicyBinding>,
}

// ============================================================================
// Trait Implementations
// ============================================================================

impl VersionedObject for MutatingAdmissionPolicy {
    fn metadata(&self) -> &ObjectMeta {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut ObjectMeta {
        &mut self.metadata
    }
}

impl VersionedObject for MutatingAdmissionPolicyBinding {
    fn metadata(&self) -> &ObjectMeta {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut ObjectMeta {
        &mut self.metadata
    }
}

impl ApplyDefault for MutatingAdmissionPolicy {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "admissionregistration.k8s.io/v1beta1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "MutatingAdmissionPolicy".to_string();
        }
    }
}

impl ApplyDefault for MutatingAdmissionPolicyList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "admissionregistration.k8s.io/v1beta1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "MutatingAdmissionPolicyList".to_string();
        }
    }
}

impl ApplyDefault for MutatingAdmissionPolicyBinding {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "admissionregistration.k8s.io/v1beta1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "MutatingAdmissionPolicyBinding".to_string();
        }
    }
}

impl ApplyDefault for MutatingAdmissionPolicyBindingList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "admissionregistration.k8s.io/v1beta1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "MutatingAdmissionPolicyBindingList".to_string();
        }
    }
}

impl ResourceSchema for MutatingAdmissionPolicy {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "admissionregistration.k8s.io"
    }

    fn version(_: &Self::Meta) -> &str {
        "v1beta1"
    }

    fn kind(_: &Self::Meta) -> &str {
        "MutatingAdmissionPolicy"
    }

    fn resource(_: &Self::Meta) -> &str {
        "mutatingadmissionpolicies"
    }

    fn group_static() -> &'static str {
        "admissionregistration.k8s.io"
    }

    fn version_static() -> &'static str {
        "v1beta1"
    }

    fn kind_static() -> &'static str {
        "MutatingAdmissionPolicy"
    }

    fn resource_static() -> &'static str {
        "mutatingadmissionpolicies"
    }
}

impl ResourceSchema for MutatingAdmissionPolicyList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "admissionregistration.k8s.io"
    }

    fn version(_: &Self::Meta) -> &str {
        "v1beta1"
    }

    fn kind(_: &Self::Meta) -> &str {
        "MutatingAdmissionPolicyList"
    }

    fn resource(_: &Self::Meta) -> &str {
        "mutatingadmissionpolicies"
    }

    fn group_static() -> &'static str {
        "admissionregistration.k8s.io"
    }

    fn version_static() -> &'static str {
        "v1beta1"
    }

    fn kind_static() -> &'static str {
        "MutatingAdmissionPolicyList"
    }

    fn resource_static() -> &'static str {
        "mutatingadmissionpolicies"
    }
}

impl ResourceSchema for MutatingAdmissionPolicyBinding {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "admissionregistration.k8s.io"
    }

    fn version(_: &Self::Meta) -> &str {
        "v1beta1"
    }

    fn kind(_: &Self::Meta) -> &str {
        "MutatingAdmissionPolicyBinding"
    }

    fn resource(_: &Self::Meta) -> &str {
        "mutatingadmissionpolicybindings"
    }

    fn group_static() -> &'static str {
        "admissionregistration.k8s.io"
    }

    fn version_static() -> &'static str {
        "v1beta1"
    }

    fn kind_static() -> &'static str {
        "MutatingAdmissionPolicyBinding"
    }

    fn resource_static() -> &'static str {
        "mutatingadmissionpolicybindings"
    }
}

impl ResourceSchema for MutatingAdmissionPolicyBindingList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "admissionregistration.k8s.io"
    }

    fn version(_: &Self::Meta) -> &str {
        "v1beta1"
    }

    fn kind(_: &Self::Meta) -> &str {
        "MutatingAdmissionPolicyBindingList"
    }

    fn resource(_: &Self::Meta) -> &str {
        "mutatingadmissionpolicybindings"
    }

    fn group_static() -> &'static str {
        "admissionregistration.k8s.io"
    }

    fn version_static() -> &'static str {
        "v1beta1"
    }

    fn kind_static() -> &'static str {
        "MutatingAdmissionPolicyBindingList"
    }

    fn resource_static() -> &'static str {
        "mutatingadmissionpolicybindings"
    }
}

impl UnimplementedConversion for MutatingAdmissionPolicy {}
impl UnimplementedConversion for MutatingAdmissionPolicyList {}
impl UnimplementedConversion for MutatingAdmissionPolicyBinding {}
impl UnimplementedConversion for MutatingAdmissionPolicyBindingList {}

impl_unimplemented_prost_message!(MutatingAdmissionPolicy);
impl_unimplemented_prost_message!(MutatingAdmissionPolicyList);
impl_unimplemented_prost_message!(MutatingAdmissionPolicyBinding);
impl_unimplemented_prost_message!(MutatingAdmissionPolicyBindingList);

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {}

#[cfg(test)]
mod trait_tests;
