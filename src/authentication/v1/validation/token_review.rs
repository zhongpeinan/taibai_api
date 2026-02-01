use crate::authentication::internal::validation as internal_validation;
use crate::authentication::v1::TokenReview;
use crate::common::ToInternal;
use crate::common::validation::ErrorList;

pub fn validate_token_review(obj: &TokenReview) -> ErrorList {
    internal_validation::validate_token_review(&obj.clone().to_internal())
}
