//! Kubernetes FlowControl API Internal Types
//!
//! This module contains type definitions from k8s-pkg/apis/flowcontrol/types.go
//! that are used internally by the Kubernetes API.
//!
//! Source: k8s.io/kubernetes/pkg/apis/flowcontrol

use crate::common::{ListMeta, ObjectMeta, Timestamp, TypeMeta};
use crate::impl_has_object_meta;
use serde::{Deserialize, Serialize};

// ============================================================================
// Constants
// ============================================================================

/// Wildcard constants
pub mod wildcards {
    pub const API_GROUP_ALL: &str = "*";
    pub const RESOURCE_ALL: &str = "*";
    pub const VERB_ALL: &str = "*";
    pub const NON_RESOURCE_ALL: &str = "*";
    pub const NAME_ALL: &str = "*";
    pub const NAMESPACE_EVERY: &str = "*";
}

/// System preset priority level names
pub mod priority_level_names {
    pub const EXEMPT: &str = "exempt";
    pub const CATCH_ALL: &str = "catch-all";
}

/// FlowSchema preset names
pub mod flow_schema_names {
    pub const EXEMPT: &str = "exempt";
    pub const CATCH_ALL: &str = "catch-all";
}

/// FlowDistinguisherMethodType constants
pub mod flow_distinguisher_method_type {
    pub const BY_USER: &str = "ByUser";
    pub const BY_NAMESPACE: &str = "ByNamespace";
}

/// SubjectKind constants
pub mod subject_kind {
    pub const USER: &str = "User";
    pub const GROUP: &str = "Group";
    pub const SERVICE_ACCOUNT: &str = "ServiceAccount";
}

/// PriorityLevelEnablement constants
pub mod priority_level_enablement {
    pub const EXEMPT: &str = "Exempt";
    pub const LIMITED: &str = "Limited";
}

/// LimitResponseType constants
pub mod limit_response_type {
    pub const QUEUE: &str = "Queue";
    pub const REJECT: &str = "Reject";
}

/// ConditionStatus constants
pub mod condition_status {
    pub const TRUE: &str = "True";
    pub const FALSE: &str = "False";
    pub const UNKNOWN: &str = "Unknown";
}

pub mod flow_schema_condition_type {
    pub const DANGLING: &str = "Dangling";
}

pub mod priority_level_condition_type {
    pub const CONCURRENCY_SHARED: &str = "ConcurrencyShared";
}

// ============================================================================
// FlowSchema
// ============================================================================

/// FlowSchema defines the schema of a group of flows.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct FlowSchema {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    pub metadata: ObjectMeta,
    /// Spec is the specification of the desired behavior of a FlowSchema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<FlowSchemaSpec>,
    /// Status is the current status of a FlowSchema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<FlowSchemaStatus>,
}
impl_has_object_meta!(FlowSchema);

/// FlowSchemaList is a list of FlowSchema objects.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct FlowSchemaList {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,
    /// Items is a list of FlowSchemas.
    #[serde(default)]
    pub items: Vec<FlowSchema>,
}

/// FlowSchemaSpec describes how the FlowSchema's specification looks like.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct FlowSchemaSpec {
    /// PriorityLevelConfiguration should reference a PriorityLevelConfiguration in the cluster.
    pub priority_level_configuration: PriorityLevelConfigurationReference,
    /// MatchingPrecedence is used to choose among the FlowSchemas that match a given request.
    #[serde(default)]
    pub matching_precedence: i32,
    /// DistinguisherMethod defines how to compute the flow distinguisher for requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distinguisher_method: Option<FlowDistinguisherMethod>,
    /// Rules describes which requests will match this flow schema.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub rules: Vec<PolicyRulesWithSubjects>,
}

/// FlowDistinguisherMethod specifies the method of a flow distinguisher.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct FlowDistinguisherMethod {
    /// Type is the type of flow distinguisher method.
    #[serde(rename = "type")]
    pub r#type: FlowDistinguisherMethodType,
}

/// FlowDistinguisherMethodType is the type of flow distinguisher method.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum FlowDistinguisherMethodType {
    /// ByUser specifies that the flow distinguisher is the username in the request.
    #[serde(rename = "ByUser")]
    #[default]
    ByUser,
    /// ByNamespace specifies that the flow distinguisher is the namespace of the object.
    #[serde(rename = "ByNamespace")]
    ByNamespace,
}

/// PriorityLevelConfigurationReference contains information that points to the priority level being used.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PriorityLevelConfigurationReference {
    /// Name is the name of the priority level configuration being referenced.
    pub name: String,
}

/// PolicyRulesWithSubjects prescribes a test that applies to a request to an apiserver.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PolicyRulesWithSubjects {
    /// Subjects is the list of normal user, serviceaccount, or group that this rule cares about.
    #[serde(default)]
    pub subjects: Vec<Subject>,
    /// ResourceRules is a slice of ResourcePolicyRules that identify matching requests.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub resource_rules: Vec<ResourcePolicyRule>,
    /// NonResourceRules is a list of NonResourcePolicyRules that identify matching requests.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub non_resource_rules: Vec<NonResourcePolicyRule>,
}

/// Subject matches the originator of a request.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Subject {
    /// Kind indicates which one of the other fields is non-empty.
    pub kind: SubjectKind,
    /// User matches based on username.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserSubject>,
    /// Group matches based on user group name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<GroupSubject>,
    /// ServiceAccount matches ServiceAccounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account: Option<ServiceAccountSubject>,
}

/// SubjectKind is the kind of subject.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum SubjectKind {
    /// User matches a regular user.
    #[serde(rename = "User")]
    #[default]
    User,
    /// Group matches a user group.
    #[serde(rename = "Group")]
    Group,
    /// ServiceAccount matches a service account.
    #[serde(rename = "ServiceAccount")]
    ServiceAccount,
}

/// UserSubject holds detailed information for user-kind subject.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserSubject {
    /// Name is the username that matches, or "*" to match all usernames.
    pub name: String,
}

/// GroupSubject holds detailed information for group-kind subject.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GroupSubject {
    /// Name is the user group that matches, or "*" to match all user groups.
    pub name: String,
}

/// ServiceAccountSubject holds detailed information for service-account-kind subject.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceAccountSubject {
    /// Namespace is the namespace of matching ServiceAccount objects.
    pub namespace: String,
    /// Name is the name of matching ServiceAccount objects, or "*" to match all.
    pub name: String,
}

/// ResourcePolicyRule is a predicate that matches some resource requests.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourcePolicyRule {
    /// Verbs is a list of matching verbs.
    #[serde(default)]
    pub verbs: Vec<String>,
    /// APIGroups is a list of matching API groups.
    #[serde(default)]
    pub api_groups: Vec<String>,
    /// Resources is a list of matching resources.
    #[serde(default)]
    pub resources: Vec<String>,
    /// ClusterScope indicates whether to match requests that do not specify a namespace.
    #[serde(default)]
    pub cluster_scope: bool,
    /// Namespaces is a list of target namespaces that restricts matches.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub namespaces: Vec<String>,
}

/// NonResourcePolicyRule is a predicate that matches non-resource requests.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NonResourcePolicyRule {
    /// Verbs is a list of matching verbs.
    #[serde(default)]
    pub verbs: Vec<String>,
    /// NonResourceURLs is a set of url prefixes that a user should have access to.
    #[serde(default)]
    pub non_resource_urls: Vec<String>,
}

/// FlowSchemaStatus represents the current state of a FlowSchema.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct FlowSchemaStatus {
    /// Conditions is a list of the current states of FlowSchema.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub conditions: Vec<FlowSchemaCondition>,
}

/// FlowSchemaCondition describes conditions for a FlowSchema.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct FlowSchemaCondition {
    /// Type is the type of the condition.
    pub r#type: FlowSchemaConditionType,
    /// Status is the status of the condition.
    pub status: ConditionStatus,
    /// LastTransitionTime is the last time the condition transitioned.
    pub last_transition_time: Timestamp,
    /// Reason is a unique, one-word, CamelCase reason for the condition's last transition.
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub reason: String,
    /// Message is a human-readable message indicating details about last transition.
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub message: String,
}

/// FlowSchemaConditionType is a valid value for FlowSchemaCondition.Type.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum FlowSchemaConditionType {
    /// Dangling indicates the FlowSchema is not referenced by any PriorityLevelConfiguration.
    #[serde(rename = "Dangling")]
    #[default]
    Dangling,
}

// ============================================================================
// PriorityLevelConfiguration
// ============================================================================

/// PriorityLevelConfiguration represents the configuration of a priority level.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct PriorityLevelConfiguration {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    pub metadata: ObjectMeta,
    /// Spec is the specification of the desired behavior of a priority level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<PriorityLevelConfigurationSpec>,
    /// Status is the current status of a priority level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PriorityLevelConfigurationStatus>,
}
impl_has_object_meta!(PriorityLevelConfiguration);

/// PriorityLevelConfigurationList is a list of PriorityLevelConfiguration objects.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PriorityLevelConfigurationList {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,
    /// Items is a list of priority levels.
    #[serde(default)]
    pub items: Vec<PriorityLevelConfiguration>,
}

/// PriorityLevelConfigurationSpec specifies the configuration of a priority level.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PriorityLevelConfigurationSpec {
    /// Type indicates whether this priority level is subject to limitation.
    #[serde(default)]
    pub r#type: PriorityLevelEnablement,
    /// Limited specifies how requests are handled for a Limited priority level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limited: Option<LimitedPriorityLevelConfiguration>,
    /// Exempt specifies how requests are handled for an exempt priority level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exempt: Option<ExemptPriorityLevelConfiguration>,
}

/// PriorityLevelEnablement indicates whether limits on execution are enabled.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum PriorityLevelEnablement {
    /// Exempt means that requests are not subject to limits.
    #[serde(rename = "Exempt")]
    #[default]
    Exempt,
    /// Limited means that requests are subject to limits.
    #[serde(rename = "Limited")]
    Limited,
}

/// LimitedPriorityLevelConfiguration specifies how to handle requests that are subject to limits.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct LimitedPriorityLevelConfiguration {
    /// NominalConcurrencyShares contributes to the computation of the NominalConcurrencyLimit.
    #[serde(default)]
    pub nominal_concurrency_shares: i32,
    /// LimitResponse indicates what to do with requests that can not be executed right now.
    #[serde(default)]
    pub limit_response: LimitResponse,
    /// LendablePercent prescribes the fraction of the level's NominalCL that can be borrowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lendable_percent: Option<i32>,
    /// BorrowingLimitPercent configures a limit on how many seats this priority level can borrow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borrowing_limit_percent: Option<i32>,
}

/// ExemptPriorityLevelConfiguration describes the configurable aspects of exempt requests.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExemptPriorityLevelConfiguration {
    /// NominalConcurrencyShares contributes to the computation of the NominalConcurrencyLimit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nominal_concurrency_shares: Option<i32>,
    /// LendablePercent prescribes the fraction of the level's NominalCL that can be borrowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lendable_percent: Option<i32>,
}

/// LimitResponse defines how to handle requests that can not be executed right now.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct LimitResponse {
    /// Type is "Queue" or "Reject".
    #[serde(rename = "type")]
    pub r#type: LimitResponseType,
    /// Queuing holds the configuration parameters for queuing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queuing: Option<QueuingConfiguration>,
}

/// LimitResponseType identifies how a Limited priority level handles a request.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum LimitResponseType {
    /// Queue means that requests are queued until they can be executed.
    #[serde(rename = "Queue")]
    #[default]
    Queue,
    /// Reject means that requests are rejected.
    #[serde(rename = "Reject")]
    Reject,
}

/// QueuingConfiguration holds the configuration parameters for queuing.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct QueuingConfiguration {
    /// Queues is the number of queues for this priority level.
    #[serde(default)]
    pub queues: i32,
    /// HandSize is a small positive number that configures the shuffle sharding.
    #[serde(default)]
    pub hand_size: i32,
    /// QueueLengthLimit is the maximum number of requests allowed to be waiting in a queue.
    #[serde(default)]
    pub queue_length_limit: i32,
}

/// PriorityLevelConfigurationStatus represents the current state of a priority level.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PriorityLevelConfigurationStatus {
    /// Conditions is the current state of the priority level.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub conditions: Vec<PriorityLevelConfigurationCondition>,
}

/// PriorityLevelConfigurationCondition defines the condition of priority level.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PriorityLevelConfigurationCondition {
    /// Type is the type of the condition.
    pub r#type: PriorityLevelConfigurationConditionType,
    /// Status is the status of the condition.
    pub status: ConditionStatus,
    /// LastTransitionTime is the last time the condition transitioned.
    pub last_transition_time: Timestamp,
    /// Reason is a unique, one-word, CamelCase reason for the condition's last transition.
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub reason: String,
    /// Message is a human-readable message indicating details about last transition.
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub message: String,
}

/// PriorityLevelConfigurationConditionType is a valid value for PriorityLevelConfigurationCondition.Type.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum PriorityLevelConfigurationConditionType {
    /// ConcurrencyShared indicates that the concurrency limit is shared.
    #[serde(rename = "ConcurrencyShared")]
    #[default]
    ConcurrencyShared,
}

/// ConditionStatus is the status of the condition.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum ConditionStatus {
    /// True indicates the condition is true.
    #[serde(rename = "True")]
    #[default]
    True,
    /// False indicates the condition is false.
    #[serde(rename = "False")]
    False,
    /// Unknown indicates the condition status is unknown.
    #[serde(rename = "Unknown")]
    Unknown,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
}
