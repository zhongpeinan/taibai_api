use crate::authentication::internal::TokenRequest;
use crate::common::validation::{BadValue, ErrorList, Path, invalid};

const MIN_TOKEN_AGE_SEC: i64 = 10 * 60; // 10 minutes
const MAX_TOKEN_AGE_SEC: i64 = 1 << 32; // 2^32 seconds

/// ValidateTokenRequest validates a TokenRequest.
pub fn validate_token_request(obj: &TokenRequest) -> ErrorList {
    validate_token_request_with_path(obj, &Path::new("spec"))
}

fn validate_token_request_with_path(obj: &TokenRequest, base_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    let expiration_path = base_path.child("expirationSeconds");

    if obj.spec.expiration_seconds < MIN_TOKEN_AGE_SEC {
        all_errs.push(invalid(
            &expiration_path,
            BadValue::Int(obj.spec.expiration_seconds),
            "may not specify a duration less than 10 minutes",
        ));
    }

    if obj.spec.expiration_seconds > MAX_TOKEN_AGE_SEC {
        all_errs.push(invalid(
            &expiration_path,
            BadValue::Int(obj.spec.expiration_seconds),
            "may not specify a duration larger than 2^32 seconds",
        ));
    }

    all_errs
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::authentication::internal::{TokenRequestSpec, TokenRequestStatus};
    use crate::common::{ObjectMeta, TypeMeta};

    fn make_token_request(expiration_seconds: i64) -> TokenRequest {
        TokenRequest {
            type_meta: TypeMeta::default(),
            metadata: ObjectMeta::default(),
            spec: TokenRequestSpec {
                audiences: vec!["api-server".to_string()],
                expiration_seconds,
                bound_object_ref: None,
            },
            status: TokenRequestStatus::default(),
        }
    }

    #[test]
    fn test_validate_token_request_valid() {
        let obj = make_token_request(3600);
        let errs = validate_token_request(&obj);
        assert!(
            errs.is_empty(),
            "expected no errors, got: {:?}",
            errs.errors
        );
    }

    #[test]
    fn test_validate_token_request_too_short() {
        let obj = make_token_request(300);
        let errs = validate_token_request(&obj);
        assert_eq!(errs.len(), 1);
        assert!(errs.errors[0].field.contains("expirationSeconds"));
        assert!(errs.errors[0].detail.contains("less than 10 minutes"));
    }

    #[test]
    fn test_validate_token_request_exactly_min() {
        let obj = make_token_request(MIN_TOKEN_AGE_SEC);
        let errs = validate_token_request(&obj);
        assert!(errs.is_empty());
    }

    #[test]
    fn test_validate_token_request_too_large() {
        let obj = make_token_request(MAX_TOKEN_AGE_SEC + 1);
        let errs = validate_token_request(&obj);
        assert_eq!(errs.len(), 1);
        assert!(errs.errors[0].field.contains("expirationSeconds"));
        assert!(errs.errors[0].detail.contains("2^32"));
    }

    #[test]
    fn test_validate_token_request_exactly_max() {
        let obj = make_token_request(MAX_TOKEN_AGE_SEC);
        let errs = validate_token_request(&obj);
        assert!(errs.is_empty());
    }
}
