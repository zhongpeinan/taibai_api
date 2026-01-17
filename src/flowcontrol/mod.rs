//! Kubernetes FlowControl API types
//!
//! This module contains types from the Kubernetes flowcontrol.apiserver.k8s.io API group.

pub mod internal;
pub mod v1;

// Re-export v1 types for convenience
pub use v1::{
    ConditionStatus, ExemptPriorityLevelConfiguration, FlowDistinguisherMethod,
    FlowDistinguisherMethodType, FlowSchema, FlowSchemaCondition, FlowSchemaConditionType,
    FlowSchemaList, FlowSchemaSpec, FlowSchemaStatus, GroupSubject, LimitResponse,
    LimitResponseType, LimitedPriorityLevelConfiguration, NonResourcePolicyRule,
    PolicyRulesWithSubjects, PriorityLevelConfiguration, PriorityLevelConfigurationCondition,
    PriorityLevelConfigurationConditionType, PriorityLevelConfigurationList,
    PriorityLevelConfigurationReference, PriorityLevelConfigurationSpec,
    PriorityLevelConfigurationStatus, PriorityLevelEnablement, QueuingConfiguration,
    ResourcePolicyRule, ServiceAccountSubject, Subject, SubjectKind, UserSubject,
};

// Re-export constants modules from v1
pub mod constants {
    pub use super::v1::{
        condition_status, flow_distinguisher_method_type, flow_schema_condition_type,
        flow_schema_names, limit_response_type, priority_level_condition_type,
        priority_level_enablement, priority_level_names, response_header, subject_kind, wildcards,
    };
}

pub use v1::AUTO_UPDATE_ANNOTATION_KEY;

// Re-export internal types
pub use internal::{
    ConditionStatus as InternalConditionStatus,
    ExemptPriorityLevelConfiguration as InternalExemptPriorityLevelConfiguration,
    FlowDistinguisherMethod as InternalFlowDistinguisherMethod,
    FlowDistinguisherMethodType as InternalFlowDistinguisherMethodType,
    FlowSchema as InternalFlowSchema, FlowSchemaCondition as InternalFlowSchemaCondition,
    FlowSchemaConditionType as InternalFlowSchemaConditionType,
    FlowSchemaList as InternalFlowSchemaList, FlowSchemaSpec as InternalFlowSchemaSpec,
    FlowSchemaStatus as InternalFlowSchemaStatus, GroupSubject as InternalGroupSubject,
    LimitResponse as InternalLimitResponse, LimitResponseType as InternalLimitResponseType,
    LimitedPriorityLevelConfiguration as InternalLimitedPriorityLevelConfiguration,
    NonResourcePolicyRule as InternalNonResourcePolicyRule,
    PolicyRulesWithSubjects as InternalPolicyRulesWithSubjects,
    PriorityLevelConfiguration as InternalPriorityLevelConfiguration,
    PriorityLevelConfigurationCondition as InternalPriorityLevelConfigurationCondition,
    PriorityLevelConfigurationConditionType as InternalPriorityLevelConfigurationConditionType,
    PriorityLevelConfigurationList as InternalPriorityLevelConfigurationList,
    PriorityLevelConfigurationReference as InternalPriorityLevelConfigurationReference,
    PriorityLevelConfigurationSpec as InternalPriorityLevelConfigurationSpec,
    PriorityLevelConfigurationStatus as InternalPriorityLevelConfigurationStatus,
    PriorityLevelEnablement as InternalPriorityLevelEnablement,
    QueuingConfiguration as InternalQueuingConfiguration,
    ResourcePolicyRule as InternalResourcePolicyRule,
    ServiceAccountSubject as InternalServiceAccountSubject, Subject as InternalSubject,
    SubjectKind as InternalSubjectKind, UserSubject as InternalUserSubject,
};
