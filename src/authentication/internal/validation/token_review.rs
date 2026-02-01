use crate::authentication::internal::TokenReview;
use crate::common::validation::ErrorList;

/// ValidateTokenReview validates a TokenReview.
///
/// Upstream validation does not apply additional checks.
pub fn validate_token_review(_obj: &TokenReview) -> ErrorList {
    ErrorList::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::authentication::internal::{TokenReviewSpec, TokenReviewStatus};
    use crate::common::{ObjectMeta, TypeMeta};

    #[test]
    fn test_validate_token_review_allows_empty() {
        let review = TokenReview {
            type_meta: TypeMeta::default(),
            metadata: ObjectMeta::default(),
            spec: TokenReviewSpec::default(),
            status: TokenReviewStatus::default(),
        };

        let errs = validate_token_review(&review);
        assert!(errs.is_empty());
    }
}
