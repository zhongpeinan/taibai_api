use crate::authorization::internal::validation as internal_validation;
use crate::authorization::v1::LocalSubjectAccessReview;
use crate::common::validation::ErrorList;
use crate::common::ToInternal;

pub fn validate_local_subject_access_review(obj: &LocalSubjectAccessReview) -> ErrorList {
    internal_validation::validate_local_subject_access_review(&obj.clone().to_internal())
}
