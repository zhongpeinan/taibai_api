//! AdmissionRegistration API types
//!
//! This module re-exports the AdmissionRegistration API types from their respective version directories.

pub mod internal;
pub mod v1;

#[cfg(test)]
pub mod tests;
pub mod v1beta1;
pub mod validation;

// Re-export commonly used v1 types
pub use v1::{
    AuditAnnotation, ExpressionWarning, FailurePolicyType, MatchCondition, MatchPolicyType,
    MatchResources, MutatingWebhook, MutatingWebhookConfiguration,
    MutatingWebhookConfigurationList, NamedRuleWithOperations, OperationType, ParamKind, ParamRef,
    ParameterNotFoundActionType, ReinvocationPolicyType, Rule, RuleWithOperations, ScopeType,
    ServiceReference, SideEffectClass, TypeChecking, ValidatingAdmissionPolicy,
    ValidatingAdmissionPolicyBinding, ValidatingAdmissionPolicyBindingList,
    ValidatingAdmissionPolicyList, ValidatingAdmissionPolicySpec, ValidatingAdmissionPolicyStatus,
    ValidatingWebhook, ValidatingWebhookConfiguration, ValidatingWebhookConfigurationList,
    Validation, ValidationAction, Variable, WebhookClientConfig,
};

// Re-export commonly used internal types
pub use internal::{
    ApplyConfiguration, JSONPatch, MutatingAdmissionPolicy, MutatingAdmissionPolicyBinding,
    MutatingAdmissionPolicyBindingList, MutatingAdmissionPolicyList, MutatingAdmissionPolicySpec,
    Mutation, PatchType,
};

// Re-export commonly used v1beta1 types
pub use v1beta1::{
    ApplyConfiguration as ApplyConfigurationV1Beta1, JSONPatch as JSONPatchV1Beta1,
    MutatingAdmissionPolicy as MutatingAdmissionPolicyV1Beta1,
    MutatingAdmissionPolicyBinding as MutatingAdmissionPolicyBindingV1Beta1,
    MutatingAdmissionPolicyBindingList as MutatingAdmissionPolicyBindingListV1Beta1,
    MutatingAdmissionPolicyBindingSpec as MutatingAdmissionPolicyBindingSpecV1Beta1,
    MutatingAdmissionPolicyList as MutatingAdmissionPolicyListV1Beta1,
    MutatingAdmissionPolicySpec as MutatingAdmissionPolicySpecV1Beta1, Mutation as MutationV1Beta1,
    PatchType as PatchTypeV1Beta1,
};
