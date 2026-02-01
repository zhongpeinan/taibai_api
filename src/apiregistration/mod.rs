//! APIRegistration API types
//!
//! This module re-exports the APIRegistration API types from their respective version directories.

pub mod internal;
pub mod v1;
pub mod v1beta1;
pub mod validation;

// Re-export commonly used v1 types
pub use v1::{
    APIService, APIServiceCondition, APIServiceConditionType, APIServiceList, APIServiceSpec,
    APIServiceStatus, ConditionStatus, ServiceReference,
};

// Re-export commonly used v1beta1 types
pub use v1beta1::{
    APIService as APIServiceV1Beta1, APIServiceCondition as APIServiceConditionV1Beta1,
    APIServiceConditionType as APIServiceConditionTypeV1Beta1,
    APIServiceList as APIServiceListV1Beta1, APIServiceSpec as APIServiceSpecV1Beta1,
    APIServiceStatus as APIServiceStatusV1Beta1, ConditionStatus as ConditionStatusV1Beta1,
    ServiceReference as ServiceReferenceV1Beta1,
};
