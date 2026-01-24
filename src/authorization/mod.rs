//! Kubernetes Authorization API types
//!
//! This module contains the authorization API types.

pub mod internal;
pub mod v1;
pub mod validation;

pub use internal::{
    FieldSelectorAttributes, LabelSelectorAttributes, LocalSubjectAccessReview,
    NonResourceAttributes, NonResourceRule, ResourceAttributes, ResourceRule,
    SelfSubjectAccessReview, SelfSubjectRulesReview, SubjectAccessReview, SubjectAccessReviewSpec,
    SubjectAccessReviewStatus, SubjectRulesReviewStatus,
};
pub use v1::{
    FieldSelectorAttributes as FieldSelectorAttributesV1,
    LabelSelectorAttributes as LabelSelectorAttributesV1,
    LocalSubjectAccessReview as LocalSubjectAccessReviewV1,
    NonResourceAttributes as NonResourceAttributesV1, NonResourceRule as NonResourceRuleV1,
    ResourceAttributes as ResourceAttributesV1, ResourceRule as ResourceRuleV1,
    SelfSubjectAccessReview as SelfSubjectAccessReviewV1,
    SelfSubjectRulesReview as SelfSubjectRulesReviewV1,
    SubjectAccessReview as SubjectAccessReviewV1,
    SubjectAccessReviewSpec as SubjectAccessReviewSpecV1,
    SubjectAccessReviewStatus as SubjectAccessReviewStatusV1,
    SubjectRulesReviewStatus as SubjectRulesReviewStatusV1,
};
