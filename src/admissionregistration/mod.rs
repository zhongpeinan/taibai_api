//! AdmissionRegistration API types
//!
//! This module re-exports the AdmissionRegistration API types from their respective version directories.

pub mod internal;
pub mod v1;
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
