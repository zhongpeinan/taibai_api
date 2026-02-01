use crate::authentication::internal::validation as internal_validation;
use crate::authentication::v1::TokenRequest;
use crate::common::ToInternal;
use crate::common::validation::ErrorList;

pub fn validate_token_request(obj: &TokenRequest) -> ErrorList {
    internal_validation::validate_token_request(&obj.clone().to_internal())
}
