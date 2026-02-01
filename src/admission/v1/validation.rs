//! Validation for Kubernetes Admission v1 API types
//!
//! Wrapper around internal validation.

use crate::admission::internal::validation as internal_validation;
use crate::admission::v1::{AdmissionRequest, AdmissionResponse, AdmissionReview};
use crate::common::ToInternal;
use crate::common::validation::ErrorList;

pub fn validate_admission_review(review: &AdmissionReview) -> ErrorList {
    internal_validation::validate_admission_review(&review.clone().to_internal())
}

pub fn validate_admission_request(request: &AdmissionRequest) -> ErrorList {
    internal_validation::validate_admission_request(&request.clone().to_internal())
}

pub fn validate_admission_response(response: &AdmissionResponse) -> ErrorList {
    internal_validation::validate_admission_response(&response.clone().to_internal())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::authentication::v1::UserInfo;
    use crate::common::validation::ErrorType;
    use crate::common::{GroupVersionKind, GroupVersionResource, TypeMeta};
    use crate::core::internal::ByteString;

    fn make_valid_request() -> AdmissionRequest {
        AdmissionRequest {
            uid: "test-uid".to_string(),
            kind: GroupVersionKind {
                group: "".to_string(),
                version: "v1".to_string(),
                kind: "Pod".to_string(),
            },
            resource: GroupVersionResource {
                group: "".to_string(),
                version: "v1".to_string(),
                resource: "pods".to_string(),
            },
            operation: "CREATE".to_string(),
            user_info: UserInfo::default(),
            ..Default::default()
        }
    }

    fn make_valid_response() -> AdmissionResponse {
        AdmissionResponse {
            uid: "test-uid".to_string(),
            allowed: true,
            ..Default::default()
        }
    }

    #[test]
    fn test_valid_admission_request() {
        let request = make_valid_request();
        assert!(validate_admission_request(&request).is_empty());
    }

    #[test]
    fn test_admission_request_missing_uid() {
        let mut request = make_valid_request();
        request.uid = "".to_string();
        let errs = validate_admission_request(&request);
        assert!(
            errs.errors
                .iter()
                .any(|e| { e.error_type == ErrorType::Required && e.field.ends_with("uid") })
        );
    }

    #[test]
    fn test_admission_request_missing_kind() {
        let mut request = make_valid_request();
        request.kind = GroupVersionKind::default();
        let errs = validate_admission_request(&request);
        assert!(errs.errors.iter().any(|e| e.field.ends_with("kind.kind")));
    }

    #[test]
    fn test_admission_request_missing_resource_version() {
        let mut request = make_valid_request();
        request.resource.version = "".to_string();
        let errs = validate_admission_request(&request);
        assert!(
            errs.errors
                .iter()
                .any(|e| e.field.ends_with("resource.version"))
        );
    }

    #[test]
    fn test_admission_request_invalid_operation() {
        let mut request = make_valid_request();
        request.operation = "INVALID".to_string();
        let errs = validate_admission_request(&request);
        assert!(errs.errors.iter().any(|e| {
            e.error_type == ErrorType::NotSupported && e.field.ends_with("operation")
        }));
    }

    #[test]
    fn test_valid_admission_response() {
        let response = make_valid_response();
        assert!(validate_admission_response(&response).is_empty());
    }

    #[test]
    fn test_admission_response_missing_uid() {
        let mut response = make_valid_response();
        response.uid = "".to_string();
        let errs = validate_admission_response(&response);
        assert!(errs.errors.iter().any(|e| e.field.ends_with("uid")));
    }

    #[test]
    fn test_admission_response_patch_requires_patch_type() {
        let mut response = make_valid_response();
        response.patch = Some(ByteString(vec![1, 2, 3]));
        let errs = validate_admission_response(&response);
        assert!(errs.errors.iter().any(|e| e.field.ends_with("patchType")));
    }

    #[test]
    fn test_admission_response_patch_type_requires_patch() {
        let mut response = make_valid_response();
        response.patch_type = Some(Box::new("JSONPatch".to_string()));
        let errs = validate_admission_response(&response);
        assert!(errs.errors.iter().any(|e| e.field.ends_with("patch")));
    }

    #[test]
    fn test_valid_admission_review_with_request() {
        let review = AdmissionReview {
            type_meta: TypeMeta::default(),
            request: Some(make_valid_request()),
            response: None,
        };
        assert!(validate_admission_review(&review).is_empty());
    }

    #[test]
    fn test_valid_admission_review_with_response() {
        let review = AdmissionReview {
            type_meta: TypeMeta::default(),
            request: None,
            response: Some(make_valid_response()),
        };
        assert!(validate_admission_review(&review).is_empty());
    }

    #[test]
    fn test_admission_review_missing_both() {
        let review = AdmissionReview {
            type_meta: TypeMeta::default(),
            request: None,
            response: None,
        };
        let errs = validate_admission_review(&review);
        assert!(errs.errors.iter().any(|e| e.field == "request"));
    }
}
