use crate::authorization::internal::validation as internal_validation;
use crate::authorization::v1::SubjectAccessReview;
use crate::common::validation::ErrorList;
use crate::common::ToInternal;

pub fn validate_subject_access_review(obj: &SubjectAccessReview) -> ErrorList {
    internal_validation::validate_subject_access_review(&obj.clone().to_internal())
}
