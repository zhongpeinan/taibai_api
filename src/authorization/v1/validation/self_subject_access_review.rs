use crate::authorization::internal::validation as internal_validation;
use crate::authorization::v1::SelfSubjectAccessReview;
use crate::common::ToInternal;
use crate::common::validation::ErrorList;

pub fn validate_self_subject_access_review(obj: &SelfSubjectAccessReview) -> ErrorList {
    internal_validation::validate_self_subject_access_review(&obj.clone().to_internal())
}
