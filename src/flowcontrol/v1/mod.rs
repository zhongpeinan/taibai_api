//! Kubernetes FlowControl v1 API types
//!
//! This module contains types from the Kubernetes flowcontrol.apiserver.k8s.io/v1 API group.

use crate::common::{
    ApplyDefault, HasTypeMeta, ListMeta, ObjectMeta, ResourceSchema, TypeMeta,
    UnimplementedConversion, VersionedObject,
};
use crate::impl_unimplemented_prost_message;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

pub mod validation;

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

/// Auto-update annotation key
pub const AUTO_UPDATE_ANNOTATION_KEY: &str = "apf.kubernetes.io/autoupdate-spec";

/// Response header constants
pub mod response_header {
    pub const MATCHED_PRIORITY_LEVEL_CONFIGURATION_UID: &str = "X-Kubernetes-PF-PriorityLevel-UID";
    pub const MATCHED_FLOW_SCHEMA_UID: &str = "X-Kubernetes-PF-FlowSchema-UID";
}

// ============================================================================
// FlowSchema
// ============================================================================

/// FlowSchema defines the schema of a group of flows.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct FlowSchema {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// Spec is the specification of the desired behavior of a FlowSchema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<FlowSchemaSpec>,
    /// Status is the current status of a FlowSchema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<FlowSchemaStatus>,
}

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matching_precedence: Option<i32>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_scope: Option<bool>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<FlowSchemaConditionType>,
    /// Status is the status of the condition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ConditionStatus>,
    /// LastTransitionTime is the last time the condition transitioned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<String>,
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
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PriorityLevelConfiguration {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// Spec is the specification of the desired behavior of a priority level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<PriorityLevelConfigurationSpec>,
    /// Status is the current status of a priority level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PriorityLevelConfigurationStatus>,
}

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<PriorityLevelEnablement>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nominal_concurrency_shares: Option<i32>,
    /// LimitResponse indicates what to do with requests that can not be executed right now.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_response: Option<LimitResponse>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queues: Option<i32>,
    /// HandSize is a small positive number that configures the shuffle sharding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hand_size: Option<i32>,
    /// QueueLengthLimit is the maximum number of requests allowed to be waiting in a queue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_length_limit: Option<i32>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<PriorityLevelConfigurationConditionType>,
    /// Status is the status of the condition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ConditionStatus>,
    /// LastTransitionTime is the last time the condition transitioned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<String>,
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
// Trait Implementations for FlowSchema and FlowSchemaList
// ============================================================================

// ----------------------------------------------------------------------------
// ResourceSchema Implementation
// ----------------------------------------------------------------------------

impl ResourceSchema for FlowSchema {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "flowcontrol.apiserver.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "FlowSchema"
    }
    fn resource(_: &Self::Meta) -> &str {
        "flowschemas"
    }

    fn group_static() -> &'static str {
        "flowcontrol.apiserver.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "FlowSchema"
    }
    fn resource_static() -> &'static str {
        "flowschemas"
    }
}

impl ResourceSchema for FlowSchemaList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "flowcontrol.apiserver.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "FlowSchemaList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "flowschemas"
    }

    fn group_static() -> &'static str {
        "flowcontrol.apiserver.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "FlowSchemaList"
    }
    fn resource_static() -> &'static str {
        "flowschemas"
    }
}

impl ResourceSchema for PriorityLevelConfiguration {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "flowcontrol.apiserver.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "PriorityLevelConfiguration"
    }
    fn resource(_: &Self::Meta) -> &str {
        "prioritylevelconfigurations"
    }

    fn group_static() -> &'static str {
        "flowcontrol.apiserver.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "PriorityLevelConfiguration"
    }
    fn resource_static() -> &'static str {
        "prioritylevelconfigurations"
    }
}

impl ResourceSchema for PriorityLevelConfigurationList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "flowcontrol.apiserver.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "PriorityLevelConfigurationList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "prioritylevelconfigurations"
    }

    fn group_static() -> &'static str {
        "flowcontrol.apiserver.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "PriorityLevelConfigurationList"
    }
    fn resource_static() -> &'static str {
        "prioritylevelconfigurations"
    }
}

// ----------------------------------------------------------------------------
// HasTypeMeta Implementation
// ----------------------------------------------------------------------------

impl HasTypeMeta for FlowSchema {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for FlowSchemaList {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for PriorityLevelConfiguration {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for PriorityLevelConfigurationList {
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

impl VersionedObject for FlowSchema {
    fn metadata(&self) -> &ObjectMeta {
        self.metadata
            .as_ref()
            .unwrap_or_else(|| static_default_object_meta())
    }

    fn metadata_mut(&mut self) -> &mut ObjectMeta {
        self.metadata.get_or_insert_with(ObjectMeta::default)
    }
}

impl VersionedObject for PriorityLevelConfiguration {
    fn metadata(&self) -> &ObjectMeta {
        self.metadata
            .as_ref()
            .unwrap_or_else(|| static_default_object_meta())
    }

    fn metadata_mut(&mut self) -> &mut ObjectMeta {
        self.metadata.get_or_insert_with(ObjectMeta::default)
    }
}

// Helper function for static default ObjectMeta
fn static_default_object_meta() -> &'static ObjectMeta {
    static DEFAULT: OnceLock<ObjectMeta> = OnceLock::new();
    DEFAULT.get_or_init(ObjectMeta::default)
}

// Note: FlowSchemaList and PriorityLevelConfigurationList do not implement VersionedObject because their metadata is ListMeta

// ----------------------------------------------------------------------------
// ApplyDefaults Implementation
// ----------------------------------------------------------------------------

impl ApplyDefault for FlowSchema {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "flowcontrol.apiserver.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "FlowSchema".to_string();
        }
    }
}

impl ApplyDefault for FlowSchemaList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "flowcontrol.apiserver.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "FlowSchemaList".to_string();
        }
    }
}

impl ApplyDefault for PriorityLevelConfiguration {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "flowcontrol.apiserver.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "PriorityLevelConfiguration".to_string();
        }
    }
}

impl ApplyDefault for PriorityLevelConfigurationList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "flowcontrol.apiserver.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "PriorityLevelConfigurationList".to_string();
        }
    }
}

// ----------------------------------------------------------------------------
// Version Conversion Placeholder (using UnimplementedConversion)
// ----------------------------------------------------------------------------

impl UnimplementedConversion for FlowSchema {}
impl UnimplementedConversion for FlowSchemaList {}
impl UnimplementedConversion for PriorityLevelConfiguration {}
impl UnimplementedConversion for PriorityLevelConfigurationList {}

// ----------------------------------------------------------------------------
// Protobuf Placeholder (using macro)
// ----------------------------------------------------------------------------

impl_unimplemented_prost_message!(FlowSchema);
impl_unimplemented_prost_message!(FlowSchemaList);
impl_unimplemented_prost_message!(PriorityLevelConfiguration);
impl_unimplemented_prost_message!(PriorityLevelConfigurationList);

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {}

#[cfg(test)]
mod trait_tests;

mod as_str_ref_impls;
