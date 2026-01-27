//! AdmissionRegistration v1 API types
//!
//! This module contains the AdmissionRegistration v1 API types.

pub mod conversion;
pub mod defaults;

use crate::common::{
    HasTypeMeta, LabelSelector, ListMeta, ObjectMeta, ResourceSchema, TypeMeta, VersionedObject,
};
use crate::core::internal::ByteString;
use crate::impl_unimplemented_prost_message;
use crate::impl_versioned_object;
use serde::{Deserialize, Serialize};

// ============================================================================
// Enums
// ============================================================================

/// ScopeType specifies a scope for a Rule.
///
/// Corresponds to [Kubernetes ScopeType](https://github.com/kubernetes/api/blob/master/admissionregistration/v1/types.go#L69)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "PascalCase")]
pub enum ScopeType {
    /// ClusterScope means that scope is limited to cluster-scoped objects.
    /// Namespace objects are cluster-scoped.
    #[default]
    Cluster,
    /// NamespacedScope means that scope is limited to namespaced objects.
    Namespaced,
    /// AllScopes means that all scopes are included.
    AllScopes,
}

/// ScopeType constants
pub mod scope_type {
    pub const CLUSTER: &str = "Cluster";
    pub const NAMESPACED: &str = "Namespaced";
    pub const ALL_SCOPES: &str = "*";
}

/// FailurePolicyType specifies a failure policy that defines how unrecognized errors from the admission endpoint are handled.
///
/// Corresponds to [Kubernetes FailurePolicyType](https://github.com/kubernetes/api/blob/master/admissionregistration/v1/types.go#L83)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "PascalCase")]
pub enum FailurePolicyType {
    /// Ignore means that an error calling the webhook is ignored.
    #[default]
    Ignore,
    /// Fail means that an error calling the webhook causes the admission to fail.
    Fail,
}

/// FailurePolicyType constants
pub mod failure_policy_type {
    pub const IGNORE: &str = "Ignore";
    pub const FAIL: &str = "Fail";
}

/// ParameterNotFoundActionType specifies a failure policy that defines how a binding
/// is evaluated when the param referred by its perNamespaceParamRef is not found.
///
/// Corresponds to [Kubernetes ParameterNotFoundActionType](https://github.com/kubernetes/api/blob/master/admissionregistration/v1/types.go#L94)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "PascalCase")]
pub enum ParameterNotFoundActionType {
    /// Allow means all requests will be admitted if no param resources could be found.
    #[default]
    Allow,
    /// Deny means all requests will be denied if no param resources are found.
    Deny,
}

/// ParameterNotFoundActionType constants
pub mod parameter_not_found_action_type {
    pub const ALLOW: &str = "Allow";
    pub const DENY: &str = "Deny";
}

/// MatchPolicyType specifies the type of match policy.
///
/// Corresponds to [Kubernetes MatchPolicyType](https://github.com/kubernetes/api/blob/master/admissionregistration/v1/types.go#L106)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "PascalCase")]
pub enum MatchPolicyType {
    /// Exact means requests should only be sent to the webhook if they exactly match a given rule.
    #[default]
    Exact,
    /// Equivalent means requests should be sent to the webhook if they modify a resource listed in rules via another API group or version.
    Equivalent,
}

/// MatchPolicyType constants
pub mod match_policy_type {
    pub const EXACT: &str = "Exact";
    pub const EQUIVALENT: &str = "Equivalent";
}

/// SideEffectClass specifies the types of side effects a webhook may have.
///
/// Corresponds to [Kubernetes SideEffectClass](https://github.com/kubernetes/api/blob/master/admissionregistration/v1/types.go#L117)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "PascalCase")]
pub enum SideEffectClass {
    /// SideEffectClassUnknown means that no information is known about the side effects of calling the webhook.
    #[default]
    Unknown,
    /// SideEffectClassNone means that calling the webhook will have no side effects.
    None,
    /// SideEffectClassSome means that calling the webhook will possibly have side effects.
    Some,
    /// SideEffectClassNoneOnDryRun means that calling the webhook will possibly have side effects, but if the
    /// request being reviewed has the dry-run attribute, the side effects will be suppressed.
    NoneOnDryRun,
}

/// SideEffectClass constants
pub mod side_effect_class {
    pub const UNKNOWN: &str = "Unknown";
    pub const NONE: &str = "None";
    pub const SOME: &str = "Some";
    pub const NONE_ON_DRY_RUN: &str = "NoneOnDryRun";
}

/// ValidationAction specifies a policy enforcement action.
///
/// Corresponds to [Kubernetes ValidationAction](https://github.com/kubernetes/api/blob/master/admissionregistration/v1/types.go#L686)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "PascalCase")]
pub enum ValidationAction {
    /// Deny specifies that a validation failure results in a denied request.
    #[default]
    Deny,
    /// Warn specifies that a validation failure is reported to the request client in HTTP Warning headers.
    Warn,
    /// Audit specifies that a validation failure is included in the published audit event.
    Audit,
}

/// ValidationAction constants
pub mod validation_action {
    pub const DENY: &str = "Deny";
    pub const WARN: &str = "Warn";
    pub const AUDIT: &str = "Audit";
}

/// OperationType specifies an operation for a request.
///
/// Corresponds to [Kubernetes OperationType](https://github.com/kubernetes/api/blob/master/admissionregistration/v1/types.go#L1130)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "PascalCase")]
pub enum OperationType {
    /// All operations
    #[default]
    All,
    /// Create operation
    Create,
    /// Update operation
    Update,
    /// Delete operation
    Delete,
    /// Connect operation
    Connect,
}

/// OperationType constants
pub mod operation_type {
    pub const ALL: &str = "*";
    pub const CREATE: &str = "CREATE";
    pub const UPDATE: &str = "UPDATE";
    pub const DELETE: &str = "DELETE";
    pub const CONNECT: &str = "CONNECT";
}

/// ReinvocationPolicyType specifies what type of policy is used when other admission plugins also perform modifications.
///
/// Corresponds to [Kubernetes ReinvocationPolicyType](https://github.com/kubernetes/api/blob/master/admissionregistration/v1/types.go#L1101)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "PascalCase")]
pub enum ReinvocationPolicyType {
    /// NeverReinvocationPolicy indicates that the mutation must not be called more than once in a single admission evaluation.
    #[default]
    Never,
    /// IfNeededReinvocationPolicy indicates that the mutation may be called at least one additional time as part of the admission evaluation.
    IfNeeded,
}

/// ReinvocationPolicyType constants
pub mod reinvocation_policy_type {
    pub const NEVER: &str = "Never";
    pub const IF_NEEDED: &str = "IfNeeded";
}

// ============================================================================
// Rule Types
// ============================================================================

/// Rule is a tuple of APIGroups, APIVersion, and Resources.
///
/// Corresponds to [Kubernetes Rule](https://github.com/kubernetes/api/blob/master/admissionregistration/v1/types.go#L23)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Rule {
    /// APIGroups is the API groups the resources belong to. '*' is all groups.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub api_groups: Vec<String>,

    /// APIVersions is the API versions the resources belong to. '*' is all versions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub api_versions: Vec<String>,

    /// Resources is a list of resources this rule applies to.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<String>,

    /// scope specifies the scope of this rule.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<ScopeType>,
}

/// RuleWithOperations is a tuple of Operations and Resources.
///
/// Corresponds to [Kubernetes RuleWithOperations](https://github.com/kubernetes/api/blob/master/admissionregistration/v1/types.go#L1116)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RuleWithOperations {
    /// Operations is the operations the admission hook cares about.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operations: Vec<OperationType>,

    /// APIGroups is the API groups the resources belong to. '*' is all groups.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub api_groups: Vec<String>,

    /// APIVersions is the API versions the resources belong to. '*' is all versions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub api_versions: Vec<String>,

    /// Resources is a list of resources this rule applies to.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<String>,

    /// scope specifies the scope of this rule.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<ScopeType>,
}

/// NamedRuleWithOperations is a tuple of Operations and Resources with ResourceNames.
///
/// Corresponds to [Kubernetes NamedRuleWithOperations](https://github.com/kubernetes/api/blob/master/admissionregistration/v1/types.go#L704)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NamedRuleWithOperations {
    /// ResourceNames is an optional white list of names that the rule applies to.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resource_names: Vec<String>,

    /// Operations is the operations the admission hook cares about.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operations: Vec<OperationType>,

    /// APIGroups is the API groups the resources belong to. '*' is all groups.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub api_groups: Vec<String>,

    /// APIVersions is the API versions the resources belong to. '*' is all versions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub api_versions: Vec<String>,

    /// Resources is a list of resources this rule applies to.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<String>,

    /// scope specifies the scope of this rule.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<ScopeType>,
}

// ============================================================================
// Param and Match Types
// ============================================================================

/// ParamKind is a tuple of Group Kind and Version.
///
/// Corresponds to [Kubernetes ParamKind](https://github.com/kubernetes/api/blob/master/admissionregistration/v1/types.go#L295)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ParamKind {
    /// APIVersion is the API group version the resources belong to.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub api_version: String,

    /// Kind is the API kind the resources belong to.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub kind: String,
}

/// ParamRef describes how to locate the params to be used as input to expressions of rules applied by a policy binding.
///
/// Corresponds to [Kubernetes ParamRef](https://github.com/kubernetes/api/blob/master/admissionregistration/v1/types.go#L537)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ParamRef {
    /// name is the name of the resource being referenced.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    /// namespace is the namespace of the referenced resource.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub namespace: String,

    /// selector can be used to match multiple param objects based on their labels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<LabelSelector>,

    /// parameterNotFoundAction controls the behavior when params are not found.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameter_not_found_action: Option<ParameterNotFoundActionType>,
}

/// MatchResources decides whether to run the admission control policy on an object based on whether it meets the match criteria.
///
/// Corresponds to [Kubernetes MatchResources](https://github.com/kubernetes/api/blob/master/admissionregistration/v1/types.go#L595)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MatchResources {
    /// NamespaceSelector decides whether to run the admission control policy on an object based on namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace_selector: Option<LabelSelector>,

    /// ObjectSelector decides whether to run the validation based on object labels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_selector: Option<LabelSelector>,

    /// ResourceRules describes what operations on what resources/subresources the policy matches.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resource_rules: Vec<NamedRuleWithOperations>,

    /// ExcludeResourceRules describes what operations should be excluded.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exclude_resource_rules: Vec<NamedRuleWithOperations>,

    /// matchPolicy defines how the "MatchResources" list is used to match incoming requests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_policy: Option<MatchPolicyType>,
}

// ============================================================================
// Validation and Expression Types
// ============================================================================

/// Validation specifies the CEL expression which is used to apply the validation.
///
/// Corresponds to [Kubernetes Validation](https://github.com/kubernetes/api/blob/master/admissionregistration/v1/types.go#L308)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Validation {
    /// Expression represents the expression which will be evaluated by CEL.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub expression: String,

    /// Message represents the message displayed when validation fails.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,

    /// messageExpression declares a CEL expression for the validation failure message.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message_expression: String,
}

/// Variable is the definition of a variable that is used for composition.
///
/// Corresponds to [Kubernetes Variable](https://github.com/kubernetes/api/blob/master/admissionregistration/v1/types.go#L383)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Variable {
    /// Name is the name of the variable.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    /// Expression is the expression that will be evaluated as the value of the variable.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub expression: String,
}

/// AuditAnnotation describes how to produce an audit annotation for an API request.
///
/// Corresponds to [Kubernetes AuditAnnotation](https://github.com/kubernetes/api/blob/master/admissionregistration/v1/types.go#L396)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AuditAnnotation {
    /// key specifies the audit annotation key.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub key: String,

    /// valueExpression represents the expression which is evaluated by CEL to produce an audit annotation value.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub value_expression: String,
}

/// MatchCondition represents a condition which must be fulfilled for a request to be sent to a webhook.
///
/// Corresponds to [Kubernetes MatchCondition](https://github.com/kubernetes/api/blob/master/admissionregistration/v1/types.go#L1210)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MatchCondition {
    /// Name is an identifier for this match condition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    /// Expression represents the expression which will be evaluated by CEL.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub expression: String,
}

/// ExpressionWarning is a warning information that targets a specific expression.
///
/// Corresponds to [Kubernetes ExpressionWarning](https://github.com/kubernetes/api/blob/master/admissionregistration/v1/types.go#L185)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExpressionWarning {
    /// The path to the field that refers the expression.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub field_ref: String,

    /// The content of type checking information in a human-readable form.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub warning: String,
}

// ============================================================================
// Webhook Types
// ============================================================================

/// ServiceReference holds a reference to Service.legacy.k8s.io
///
/// Corresponds to [Kubernetes ServiceReference](https://github.com/kubernetes/api/blob/master/admissionregistration/v1/types.go#L1189)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceReference {
    /// namespace is the namespace of the service.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub namespace: String,

    /// name is the name of the service.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    /// path is an optional URL path which will be sent in any request to this service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// If specified, the port on the service that hosting webhook.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}

/// WebhookClientConfig contains the information to make a TLS connection with the webhook.
///
/// Corresponds to [Kubernetes WebhookClientConfig](https://github.com/kubernetes/api/blob/master/admissionregistration/v1/types.go#L1143)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct WebhookClientConfig {
    /// url gives the location of the webhook, in standard URL form.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// service is a reference to the service for this webhook.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<ServiceReference>,

    /// caBundle is a PEM encoded CA bundle which will be used to validate the webhook's server certificate.
    #[serde(default)]
    pub ca_bundle: ByteString,
}

/// ValidatingWebhook describes an admission webhook and the resources and operations it applies to.
///
/// Corresponds to [Kubernetes ValidatingWebhook](https://github.com/kubernetes/api/blob/master/admissionregistration/v1/types.go#L783)
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
/// Corresponds to [Kubernetes MutatingWebhook](https://github.com/kubernetes/api/blob/master/admissionregistration/v1/types.go#L933)
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
// ValidatingAdmissionPolicy Types
// ============================================================================

/// ValidatingAdmissionPolicySpec is the specification of the desired behavior of the AdmissionPolicy.
///
/// Corresponds to [Kubernetes ValidatingAdmissionPolicySpec](https://github.com/kubernetes/api/blob/master/admissionregistration/v1/types.go#L211)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ValidatingAdmissionPolicySpec {
    /// ParamKind specifies the kind of resources used to parameterize this policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub param_kind: Option<ParamKind>,

    /// MatchConstraints specifies what resources this policy is designed to validate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_constraints: Option<MatchResources>,

    /// Validations contain CEL expressions which is used to apply the validation.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub validations: Vec<Validation>,

    /// failurePolicy defines how to handle failures for the admission policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_policy: Option<FailurePolicyType>,

    /// auditAnnotations contains CEL expressions which are used to produce audit annotations.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub audit_annotations: Vec<AuditAnnotation>,

    /// MatchConditions is a list of conditions that must be met for a request to be validated.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub match_conditions: Vec<MatchCondition>,

    /// Variables contain definitions of variables that can be used in composition of other expressions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub variables: Vec<Variable>,
}

/// ValidatingAdmissionPolicyStatus represents the status of an admission validation policy.
///
/// Corresponds to [Kubernetes ValidatingAdmissionPolicyStatus](https://github.com/kubernetes/api/blob/master/admissionregistration/v1/types.go#L157)
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
    pub conditions: Vec<crate::common::Condition>,
}

/// TypeChecking contains results of type checking the expressions in the ValidatingAdmissionPolicy.
///
/// Corresponds to [Kubernetes TypeChecking](https://github.com/kubernetes/api/blob/master/admissionregistration/v1/types.go#L176)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TypeChecking {
    /// The type checking warnings for each expression.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub expression_warnings: Vec<ExpressionWarning>,
}

/// ValidatingAdmissionPolicy describes the definition of an admission validation policy.
///
/// Corresponds to [Kubernetes ValidatingAdmissionPolicy](https://github.com/kubernetes/api/blob/master/admissionregistration/v1/types.go#L141)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ValidatingAdmissionPolicy {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// Specification of the desired behavior of the ValidatingAdmissionPolicy.
    #[serde(default)]
    pub spec: ValidatingAdmissionPolicySpec,

    /// The status of the ValidatingAdmissionPolicy.
    #[serde(default)]
    pub status: ValidatingAdmissionPolicyStatus,
}
impl_versioned_object!(ValidatingAdmissionPolicy);

/// ValidatingAdmissionPolicyList is a list of ValidatingAdmissionPolicy.
///
/// Corresponds to [Kubernetes ValidatingAdmissionPolicyList](https://github.com/kubernetes/api/blob/master/admissionregistration/v1/types.go#L200)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ValidatingAdmissionPolicyList {
    /// TypeMeta describes the type of this object.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// List of ValidatingAdmissionPolicy.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ValidatingAdmissionPolicy>,
}

// ============================================================================
// ValidatingAdmissionPolicyBinding Types
// ============================================================================

/// ValidatingAdmissionPolicyBindingSpec is the specification of the ValidatingAdmissionPolicyBinding.
///
/// Corresponds to [Kubernetes ValidatingAdmissionPolicyBindingSpec](https://github.com/kubernetes/api/blob/master/admissionregistration/v1/types.go#L472)
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
/// Corresponds to [Kubernetes ValidatingAdmissionPolicyBinding](https://github.com/kubernetes/api/blob/master/admissionregistration/v1/types.go#L438)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ValidatingAdmissionPolicyBinding {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// Specification of the desired behavior of the ValidatingAdmissionPolicyBinding.
    #[serde(default)]
    pub spec: ValidatingAdmissionPolicyBindingSpec,
}
impl_versioned_object!(ValidatingAdmissionPolicyBinding);

/// ValidatingAdmissionPolicyBindingList is a list of ValidatingAdmissionPolicyBinding.
///
/// Corresponds to [Kubernetes ValidatingAdmissionPolicyBindingList](https://github.com/kubernetes/api/blob/master/admissionregistration/v1/types.go#L461)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ValidatingAdmissionPolicyBindingList {
    /// TypeMeta describes the type of this object.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// List of ValidatingAdmissionPolicyBinding.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ValidatingAdmissionPolicyBinding>,
}

// ============================================================================
// ValidatingWebhookConfiguration Types
// ============================================================================

/// ValidatingWebhookConfiguration describes the configuration of an admission webhook.
///
/// Corresponds to [Kubernetes ValidatingWebhookConfiguration](https://github.com/kubernetes/api/blob/master/admissionregistration/v1/types.go#L720)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ValidatingWebhookConfiguration {
    /// TypeMeta describes the type of this object.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// Webhooks is a list of webhooks and the affected resources and operations.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub webhooks: Vec<ValidatingWebhook>,
}

/// ValidatingWebhookConfigurationList is a list of ValidatingWebhookConfiguration.
///
/// Corresponds to [Kubernetes ValidatingWebhookConfigurationList](https://github.com/kubernetes/api/blob/master/admissionregistration/v1/types.go#L738)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ValidatingWebhookConfigurationList {
    /// TypeMeta describes the type of this object.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// List of ValidatingWebhookConfiguration.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ValidatingWebhookConfiguration>,
}

// ============================================================================
// MutatingWebhookConfiguration Types
// ============================================================================

/// MutatingWebhookConfiguration describes the configuration of an admission webhook.
///
/// Corresponds to [Kubernetes MutatingWebhookConfiguration](https://github.com/kubernetes/api/blob/master/admissionregistration/v1/types.go#L754)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MutatingWebhookConfiguration {
    /// TypeMeta describes the type of this object.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// Webhooks is a list of webhooks and the affected resources and operations.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub webhooks: Vec<MutatingWebhook>,
}

/// MutatingWebhookConfigurationList is a list of MutatingWebhookConfiguration.
///
/// Corresponds to [Kubernetes MutatingWebhookConfigurationList](https://github.com/kubernetes/api/blob/master/admissionregistration/v1/types.go#L772)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MutatingWebhookConfigurationList {
    /// TypeMeta describes the type of this object.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// List of MutatingWebhookConfiguration.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<MutatingWebhookConfiguration>,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {}

#[cfg(test)]
mod trait_tests;

// ============================================================================
// Trait Implementations for AdmissionRegistration Resources
// ============================================================================

// ----------------------------------------------------------------------------
// ResourceSchema Implementation
// ----------------------------------------------------------------------------

impl ResourceSchema for ValidatingWebhookConfiguration {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "admissionregistration.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "ValidatingWebhookConfiguration"
    }
    fn resource(_: &Self::Meta) -> &str {
        "validatingwebhookconfigurations"
    }

    fn group_static() -> &'static str {
        "admissionregistration.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "ValidatingWebhookConfiguration"
    }
    fn resource_static() -> &'static str {
        "validatingwebhookconfigurations"
    }
}

impl ResourceSchema for ValidatingWebhookConfigurationList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "admissionregistration.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "ValidatingWebhookConfigurationList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "validatingwebhookconfigurations"
    }

    fn group_static() -> &'static str {
        "admissionregistration.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "ValidatingWebhookConfigurationList"
    }
    fn resource_static() -> &'static str {
        "validatingwebhookconfigurations"
    }
}

impl ResourceSchema for MutatingWebhookConfiguration {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "admissionregistration.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "MutatingWebhookConfiguration"
    }
    fn resource(_: &Self::Meta) -> &str {
        "mutatingwebhookconfigurations"
    }

    fn group_static() -> &'static str {
        "admissionregistration.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "MutatingWebhookConfiguration"
    }
    fn resource_static() -> &'static str {
        "mutatingwebhookconfigurations"
    }
}

impl ResourceSchema for MutatingWebhookConfigurationList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "admissionregistration.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "MutatingWebhookConfigurationList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "mutatingwebhookconfigurations"
    }

    fn group_static() -> &'static str {
        "admissionregistration.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "MutatingWebhookConfigurationList"
    }
    fn resource_static() -> &'static str {
        "mutatingwebhookconfigurations"
    }
}

impl ResourceSchema for ValidatingAdmissionPolicy {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "admissionregistration.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "ValidatingAdmissionPolicy"
    }
    fn resource(_: &Self::Meta) -> &str {
        "validatingadmissionpolicies"
    }

    fn group_static() -> &'static str {
        "admissionregistration.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "ValidatingAdmissionPolicy"
    }
    fn resource_static() -> &'static str {
        "validatingadmissionpolicies"
    }
}

impl ResourceSchema for ValidatingAdmissionPolicyList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "admissionregistration.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "ValidatingAdmissionPolicyList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "validatingadmissionpolicies"
    }

    fn group_static() -> &'static str {
        "admissionregistration.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "ValidatingAdmissionPolicyList"
    }
    fn resource_static() -> &'static str {
        "validatingadmissionpolicies"
    }
}

impl ResourceSchema for ValidatingAdmissionPolicyBinding {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "admissionregistration.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "ValidatingAdmissionPolicyBinding"
    }
    fn resource(_: &Self::Meta) -> &str {
        "validatingadmissionpolicybindings"
    }

    fn group_static() -> &'static str {
        "admissionregistration.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "ValidatingAdmissionPolicyBinding"
    }
    fn resource_static() -> &'static str {
        "validatingadmissionpolicybindings"
    }
}

impl ResourceSchema for ValidatingAdmissionPolicyBindingList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "admissionregistration.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "ValidatingAdmissionPolicyBindingList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "validatingadmissionpolicybindings"
    }

    fn group_static() -> &'static str {
        "admissionregistration.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "ValidatingAdmissionPolicyBindingList"
    }
    fn resource_static() -> &'static str {
        "validatingadmissionpolicybindings"
    }
}

// ----------------------------------------------------------------------------
// HasTypeMeta Implementation
// ----------------------------------------------------------------------------

impl HasTypeMeta for ValidatingWebhookConfiguration {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for ValidatingWebhookConfigurationList {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for MutatingWebhookConfiguration {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for MutatingWebhookConfigurationList {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for ValidatingAdmissionPolicy {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for ValidatingAdmissionPolicyList {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for ValidatingAdmissionPolicyBinding {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for ValidatingAdmissionPolicyBindingList {
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

impl VersionedObject for ValidatingWebhookConfiguration {
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

impl VersionedObject for MutatingWebhookConfiguration {
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
// Protobuf Placeholder
// ----------------------------------------------------------------------------

impl_unimplemented_prost_message!(ValidatingWebhookConfiguration);
impl_unimplemented_prost_message!(ValidatingWebhookConfigurationList);
impl_unimplemented_prost_message!(MutatingWebhookConfiguration);
impl_unimplemented_prost_message!(MutatingWebhookConfigurationList);
impl_unimplemented_prost_message!(ValidatingAdmissionPolicy);
impl_unimplemented_prost_message!(ValidatingAdmissionPolicyList);
impl_unimplemented_prost_message!(ValidatingAdmissionPolicyBinding);
impl_unimplemented_prost_message!(ValidatingAdmissionPolicyBindingList);
