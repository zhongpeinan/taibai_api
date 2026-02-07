//! AdmissionRegistration internal API types
//!
//! This module contains the internal API types for admissionregistration.

use crate::admissionregistration::v1::{
    AuditAnnotation, FailurePolicyType, MatchCondition, MatchPolicyType, MatchResources, ParamKind,
    ParamRef, ReinvocationPolicyType, RuleWithOperations, SideEffectClass, TypeChecking,
    Validation, ValidationAction, Variable, WebhookClientConfig,
};
use crate::common::{Condition, LabelSelector, ListMeta, ObjectMeta, TypeMeta};
use crate::impl_has_object_meta;
use serde::{Deserialize, Serialize};

pub mod validation;

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
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MutatingAdmissionPolicy {
    /// Standard type metadata (not serialized in internal version).
    #[serde(skip)]
    pub type_meta: TypeMeta,
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
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MutatingAdmissionPolicyList {
    /// Standard type metadata (not serialized in internal version).
    #[serde(skip)]
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
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MutatingAdmissionPolicyBinding {
    /// Standard type metadata (not serialized in internal version).
    #[serde(skip)]
    pub type_meta: TypeMeta,
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
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MutatingAdmissionPolicyBindingList {
    /// Standard type metadata (not serialized in internal version).
    #[serde(skip)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(default)]
    pub metadata: ListMeta,

    /// List of PolicyBinding.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<MutatingAdmissionPolicyBinding>,
}

// ============================================================================
// ValidatingAdmissionPolicy Types
// ============================================================================

/// ValidatingAdmissionPolicySpec is the specification of the desired behavior of the AdmissionPolicy.
///
/// Corresponds to [Kubernetes ValidatingAdmissionPolicySpec](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/admissionregistration/types.go#L201)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ValidatingAdmissionPolicySpec {
    /// ParamKind specifies the kind of resources used to parameterize this policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub param_kind: Option<ParamKind>,

    /// MatchConstraints specifies what resources this policy is designed to validate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_constraints: Option<MatchResources>,

    /// Validations contain CEL expressions which are used to apply the validation.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub validations: Vec<Validation>,

    /// MatchConditions is a list of conditions that must be met for a request to be validated.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub match_conditions: Vec<MatchCondition>,

    /// failurePolicy defines how to handle failures for the admission policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_policy: Option<FailurePolicyType>,

    /// auditAnnotations contains CEL expressions which are used to produce audit annotations.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub audit_annotations: Vec<AuditAnnotation>,

    /// Variables contain definitions of variables that can be used in composition of other expressions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub variables: Vec<Variable>,
}

/// ValidatingAdmissionPolicyStatus represents the status of an admission validation policy.
///
/// Corresponds to [Kubernetes ValidatingAdmissionPolicyStatus](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/admissionregistration/types.go#L148)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ValidatingAdmissionPolicyStatus {
    /// The generation observed by the controller.
    #[serde(default)]
    pub observed_generation: i64,

    /// The results of type checking for each expression.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_checking: Option<TypeChecking>,

    /// The conditions represent the latest available observations of a policy's current state.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<Condition>,
}

/// ValidatingAdmissionPolicy describes the definition of an admission validation policy.
///
/// Corresponds to [Kubernetes ValidatingAdmissionPolicy](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/admissionregistration/types.go#L132)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ValidatingAdmissionPolicy {
    /// Standard type metadata (not serialized in internal version).
    #[serde(skip)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    pub metadata: ObjectMeta,

    /// Specification of the desired behavior of the ValidatingAdmissionPolicy.
    #[serde(default)]
    pub spec: ValidatingAdmissionPolicySpec,

    /// The status of the ValidatingAdmissionPolicy.
    #[serde(default)]
    pub status: ValidatingAdmissionPolicyStatus,
}
impl_has_object_meta!(ValidatingAdmissionPolicy);

/// ValidatingAdmissionPolicyList is a list of ValidatingAdmissionPolicy.
///
/// Corresponds to [Kubernetes ValidatingAdmissionPolicyList](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/admissionregistration/types.go#L190)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ValidatingAdmissionPolicyList {
    /// Standard type metadata (not serialized in internal version).
    #[serde(skip)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(default)]
    pub metadata: ListMeta,

    /// List of ValidatingAdmissionPolicy.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ValidatingAdmissionPolicy>,
}

// ============================================================================
// ValidatingAdmissionPolicyBinding Types
// ============================================================================

/// ValidatingAdmissionPolicyBindingSpec is the specification of the ValidatingAdmissionPolicyBinding.
///
/// Corresponds to [Kubernetes ValidatingAdmissionPolicyBindingSpec](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/admissionregistration/types.go#L450)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ValidatingAdmissionPolicyBindingSpec {
    /// PolicyName references a ValidatingAdmissionPolicy name.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub policy_name: String,

    /// paramRef specifies the parameter resource used to configure the admission control policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub param_ref: Option<ParamRef>,

    /// MatchResources declares what resources match this binding.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_resources: Option<MatchResources>,

    /// validationActions declares how Validations of the referenced ValidatingAdmissionPolicy are enforced.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub validation_actions: Vec<ValidationAction>,
}

/// ValidatingAdmissionPolicyBinding binds the ValidatingAdmissionPolicy with paramerized resources.
///
/// Corresponds to [Kubernetes ValidatingAdmissionPolicyBinding](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/admissionregistration/types.go#L428)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ValidatingAdmissionPolicyBinding {
    /// Standard type metadata (not serialized in internal version).
    #[serde(skip)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    pub metadata: ObjectMeta,

    /// Specification of the desired behavior of the ValidatingAdmissionPolicyBinding.
    #[serde(default)]
    pub spec: ValidatingAdmissionPolicyBindingSpec,
}
impl_has_object_meta!(ValidatingAdmissionPolicyBinding);

/// ValidatingAdmissionPolicyBindingList is a list of ValidatingAdmissionPolicyBinding.
///
/// Corresponds to [Kubernetes ValidatingAdmissionPolicyBindingList](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/admissionregistration/types.go#L439)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ValidatingAdmissionPolicyBindingList {
    /// Standard type metadata (not serialized in internal version).
    #[serde(skip)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(default)]
    pub metadata: ListMeta,

    /// List of ValidatingAdmissionPolicyBinding.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ValidatingAdmissionPolicyBinding>,
}

// ============================================================================
// Webhook Types
// ============================================================================

/// ValidatingWebhook describes an admission webhook and the resources and operations it applies to.
///
/// Corresponds to [Kubernetes ValidatingWebhook](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/admissionregistration/types.go#L741)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ValidatingWebhook {
    /// The name of the admission webhook.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    /// ClientConfig defines how to communicate with the hook.
    #[serde(default)]
    pub client_config: WebhookClientConfig,

    /// Rules describes what operations on what resources/subresources the webhook cares about.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<RuleWithOperations>,

    /// FailurePolicy defines how unrecognized errors from the admission endpoint are handled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_policy: Option<FailurePolicyType>,

    /// matchPolicy defines how the "rules" list is used to match incoming requests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_policy: Option<MatchPolicyType>,

    /// NamespaceSelector decides whether to run the webhook on an object based on namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace_selector: Option<LabelSelector>,

    /// ObjectSelector decides whether to run the webhook based on object labels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_selector: Option<LabelSelector>,

    /// SideEffects states whether this webhook has side effects.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub side_effects: Option<SideEffectClass>,

    /// TimeoutSeconds specifies the timeout for this webhook.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i32>,

    /// AdmissionReviewVersions is an ordered list of preferred AdmissionReview versions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub admission_review_versions: Vec<String>,

    /// MatchConditions is a list of conditions that must be met for a request to be sent to this webhook.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub match_conditions: Vec<MatchCondition>,
}

/// MutatingWebhook describes an admission webhook and the resources and operations it applies to.
///
/// Corresponds to [Kubernetes MutatingWebhook](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/admissionregistration/types.go#L880)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MutatingWebhook {
    /// The name of the admission webhook.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    /// ClientConfig defines how to communicate with the hook.
    #[serde(default)]
    pub client_config: WebhookClientConfig,

    /// Rules describes what operations on what resources/subresources the webhook cares about.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<RuleWithOperations>,

    /// FailurePolicy defines how unrecognized errors from the admission endpoint are handled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_policy: Option<FailurePolicyType>,

    /// matchPolicy defines how the "rules" list is used to match incoming requests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_policy: Option<MatchPolicyType>,

    /// NamespaceSelector decides whether to run the webhook on an object based on namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace_selector: Option<LabelSelector>,

    /// ObjectSelector decides whether to run the webhook based on object labels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_selector: Option<LabelSelector>,

    /// SideEffects states whether this webhook has side effects.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub side_effects: Option<SideEffectClass>,

    /// TimeoutSeconds specifies the timeout for this webhook.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i32>,

    /// AdmissionReviewVersions is an ordered list of preferred AdmissionReview versions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub admission_review_versions: Vec<String>,

    /// reinvocationPolicy indicates whether this webhook should be called multiple times.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reinvocation_policy: Option<ReinvocationPolicyType>,

    /// MatchConditions is a list of conditions that must be met for a request to be sent to this webhook.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub match_conditions: Vec<MatchCondition>,
}

// ============================================================================
// Webhook Configuration Types
// ============================================================================

/// ValidatingWebhookConfiguration describes the configuration of an admission webhook.
///
/// Corresponds to [Kubernetes ValidatingWebhookConfiguration](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/admissionregistration/types.go#L690)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ValidatingWebhookConfiguration {
    /// Standard type metadata (not serialized in internal version).
    #[serde(skip)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    pub metadata: ObjectMeta,

    /// Webhooks is a list of webhooks and the affected resources and operations.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub webhooks: Vec<ValidatingWebhook>,
}
impl_has_object_meta!(ValidatingWebhookConfiguration);

/// ValidatingWebhookConfigurationList is a list of ValidatingWebhookConfiguration.
///
/// Corresponds to [Kubernetes ValidatingWebhookConfigurationList](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/admissionregistration/types.go#L704)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ValidatingWebhookConfigurationList {
    /// Standard type metadata (not serialized in internal version).
    #[serde(skip)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(default)]
    pub metadata: ListMeta,

    /// List of ValidatingWebhookConfiguration.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ValidatingWebhookConfiguration>,
}

/// MutatingWebhookConfiguration describes the configuration of an admission webhook.
///
/// Corresponds to [Kubernetes MutatingWebhookConfiguration](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/admissionregistration/types.go#L717)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MutatingWebhookConfiguration {
    /// Standard type metadata (not serialized in internal version).
    #[serde(skip)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    pub metadata: ObjectMeta,

    /// Webhooks is a list of webhooks and the affected resources and operations.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub webhooks: Vec<MutatingWebhook>,
}
impl_has_object_meta!(MutatingWebhookConfiguration);

/// MutatingWebhookConfigurationList is a list of MutatingWebhookConfiguration.
///
/// Corresponds to [Kubernetes MutatingWebhookConfigurationList](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/admissionregistration/types.go#L730)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MutatingWebhookConfigurationList {
    /// Standard type metadata (not serialized in internal version).
    #[serde(skip)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(default)]
    pub metadata: ListMeta,

    /// List of MutatingWebhookConfiguration.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<MutatingWebhookConfiguration>,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {}

// AsRefStr / AsRef<str> implementations for enums
crate::impl_as_str_ref!(PatchType, {
    ApplyConfiguration => "ApplyConfiguration",
    JSONPatch => "JSONPatch",
});
