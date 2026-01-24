//! Validation rules for admission v1 API types
//!
//! Ported from k8s/pkg/apis/admission/validation (if present)
//!
//! Since Kubernetes does not have explicit validation files for admission types,
//! this module provides minimal required-field checks.

use crate::admission::v1::{AdmissionRequest, AdmissionResponse, AdmissionReview};

/// Validation error type
#[derive(Debug, PartialEq, Eq)]
pub enum ValidationError {
    /// Missing required field
    MissingField(&'static str),
    /// Invalid field value
    InvalidField(&'static str, String),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::MissingField(field) => write!(f, "missing required field: {}", field),
            ValidationError::InvalidField(field, msg) => {
                write!(f, "invalid field '{}': {}", field, msg)
            }
        }
    }
}

impl std::error::Error for ValidationError {}

/// ValidationResult type
pub type ValidationResult<T = ()> = Result<T, ValidationError>;

// ============================================================================
// AdmissionReview Validation
// ============================================================================

impl AdmissionReview {
    /// Validates the AdmissionReview.
    ///
    /// An AdmissionReview must have either a request or a response (or both).
    pub fn validate(&self) -> ValidationResult {
        if self.request.is_none() && self.response.is_none() {
            return Err(ValidationError::MissingField("request or response"));
        }

        if let Some(ref request) = self.request {
            request.validate()?;
        }

        if let Some(ref response) = self.response {
            response.validate()?;
        }

        Ok(())
    }
}

// ============================================================================
// AdmissionRequest Validation
// ============================================================================

impl AdmissionRequest {
    /// Validates the AdmissionRequest.
    ///
    /// Required fields: uid, kind, resource, operation.
    pub fn validate(&self) -> ValidationResult {
        // uid is required
        if self.uid.is_empty() {
            return Err(ValidationError::MissingField("uid"));
        }

        // kind is required
        self.validate_gvk()?;

        // resource is required
        self.validate_gvr()?;

        // operation is required
        if self.operation.is_empty() {
            return Err(ValidationError::MissingField("operation"));
        }

        if let Some(ref request_kind) = self.request_kind {
            if request_kind.kind.is_empty() {
                return Err(ValidationError::MissingField("request_kind.kind"));
            }
            if request_kind.version.is_empty() {
                return Err(ValidationError::MissingField("request_kind.version"));
            }
        }

        if let Some(ref request_resource) = self.request_resource {
            if request_resource.resource.is_empty() {
                return Err(ValidationError::MissingField("request_resource.resource"));
            }
            if request_resource.version.is_empty() {
                return Err(ValidationError::MissingField("request_resource.version"));
            }
        }

        // Validate operation value
        match self.operation.as_str() {
            "CREATE" | "UPDATE" | "DELETE" | "CONNECT" => {}
            op => {
                return Err(ValidationError::InvalidField(
                    "operation",
                    format!("unknown operation: {}", op),
                ));
            }
        }

        Ok(())
    }

    fn validate_gvk(&self) -> ValidationResult {
        // kind.kind must be non-empty (most specific check first)
        if self.kind.kind.is_empty() {
            return Err(ValidationError::MissingField("kind.kind"));
        }
        if self.kind.version.is_empty() {
            return Err(ValidationError::MissingField("kind.version"));
        }
        // Check if entire kind is completely empty
        if self.kind.group.is_empty() && self.kind.version.is_empty() {
            return Err(ValidationError::MissingField("kind"));
        }
        Ok(())
    }

    fn validate_gvr(&self) -> ValidationResult {
        // resource.resource must be non-empty (most specific check first)
        if self.resource.resource.is_empty() {
            return Err(ValidationError::MissingField("resource.resource"));
        }
        if self.resource.version.is_empty() {
            return Err(ValidationError::MissingField("resource.version"));
        }
        // Check if entire resource is completely empty
        if self.resource.group.is_empty() && self.resource.version.is_empty() {
            return Err(ValidationError::MissingField("resource"));
        }
        Ok(())
    }
}

// ============================================================================
// AdmissionResponse Validation
// ============================================================================

impl AdmissionResponse {
    /// Validates the AdmissionResponse.
    ///
    /// Required fields: uid, allowed.
    pub fn validate(&self) -> ValidationResult {
        // uid is required
        if self.uid.is_empty() {
            return Err(ValidationError::MissingField("uid"));
        }

        // patch and patch_type must be provided together
        if self.patch.is_some() ^ self.patch_type.is_some() {
            return Err(ValidationError::MissingField("patch/patchType"));
        }

        // allowed is always present (bool), but we validate that it's explicitly set
        // in context of the response being valid

        Ok(())
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::authentication::v1::UserInfo;
    use crate::common::GroupVersionKind;
    use crate::core::internal::ByteString;

    fn make_valid_request() -> AdmissionRequest {
        AdmissionRequest {
            uid: "test-uid".to_string(),
            kind: GroupVersionKind {
                group: "".to_string(),
                version: "v1".to_string(),
                kind: "Pod".to_string(),
            },
            resource: crate::common::GroupVersionResource {
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
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_admission_request_missing_uid() {
        let mut request = make_valid_request();
        request.uid = "".to_string();
        assert_eq!(
            request.validate(),
            Err(ValidationError::MissingField("uid"))
        );
    }

    #[test]
    fn test_admission_request_missing_kind() {
        let mut request = make_valid_request();
        request.kind = GroupVersionKind {
            group: "".to_string(),
            version: "".to_string(),
            kind: "".to_string(),
        };
        assert_eq!(
            request.validate(),
            Err(ValidationError::MissingField("kind.kind"))
        );
    }

    #[test]
    fn test_admission_request_missing_kind_version() {
        let mut request = make_valid_request();
        request.kind.version = "".to_string();
        assert_eq!(
            request.validate(),
            Err(ValidationError::MissingField("kind.version"))
        );
    }

    #[test]
    fn test_admission_request_missing_resource() {
        let mut request = make_valid_request();
        request.resource = crate::common::GroupVersionResource {
            group: "".to_string(),
            version: "".to_string(),
            resource: "".to_string(),
        };
        assert_eq!(
            request.validate(),
            Err(ValidationError::MissingField("resource.resource"))
        );
    }

    #[test]
    fn test_admission_request_missing_resource_version() {
        let mut request = make_valid_request();
        request.resource.version = "".to_string();
        assert_eq!(
            request.validate(),
            Err(ValidationError::MissingField("resource.version"))
        );
    }

    #[test]
    fn test_admission_request_missing_operation() {
        let mut request = make_valid_request();
        request.operation = "".to_string();
        assert_eq!(
            request.validate(),
            Err(ValidationError::MissingField("operation"))
        );
    }

    #[test]
    fn test_admission_request_invalid_operation() {
        let mut request = make_valid_request();
        request.operation = "INVALID".to_string();
        assert!(matches!(
            request.validate(),
            Err(ValidationError::InvalidField(_, _))
        ));
    }

    #[test]
    fn test_admission_request_valid_operations() {
        for op in ["CREATE", "UPDATE", "DELETE", "CONNECT"] {
            let mut request = make_valid_request();
            request.operation = op.to_string();
            assert!(
                request.validate().is_ok(),
                "operation {} should be valid",
                op
            );
        }
    }

    #[test]
    fn test_valid_admission_response() {
        let response = make_valid_response();
        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_admission_response_missing_uid() {
        let mut response = make_valid_response();
        response.uid = "".to_string();
        assert_eq!(
            response.validate(),
            Err(ValidationError::MissingField("uid"))
        );
    }

    #[test]
    fn test_valid_admission_review_with_request() {
        let review = AdmissionReview {
            type_meta: Default::default(),
            request: Some(make_valid_request()),
            response: None,
        };
        assert!(review.validate().is_ok());
    }

    #[test]
    fn test_valid_admission_review_with_response() {
        let review = AdmissionReview {
            type_meta: Default::default(),
            request: None,
            response: Some(make_valid_response()),
        };
        assert!(review.validate().is_ok());
    }

    #[test]
    fn test_admission_review_missing_both() {
        let review = AdmissionReview {
            type_meta: Default::default(),
            request: None,
            response: None,
        };
        assert_eq!(
            review.validate(),
            Err(ValidationError::MissingField("request or response"))
        );
    }

    #[test]
    fn test_admission_review_with_invalid_request() {
        let invalid_request = AdmissionRequest {
            uid: "".to_string(),
            ..Default::default()
        };
        let review = AdmissionReview {
            type_meta: Default::default(),
            request: Some(invalid_request),
            response: None,
        };
        assert!(review.validate().is_err());
    }

    #[test]
    fn test_admission_request_missing_request_kind_version() {
        let mut request = make_valid_request();
        request.request_kind = Some(Box::new(GroupVersionKind {
            group: "".to_string(),
            version: "".to_string(),
            kind: "Pod".to_string(),
        }));
        assert_eq!(
            request.validate(),
            Err(ValidationError::MissingField("request_kind.version"))
        );
    }

    #[test]
    fn test_admission_request_missing_request_resource_version() {
        let mut request = make_valid_request();
        request.request_resource = Some(Box::new(crate::common::GroupVersionResource {
            group: "".to_string(),
            version: "".to_string(),
            resource: "pods".to_string(),
        }));
        assert_eq!(
            request.validate(),
            Err(ValidationError::MissingField("request_resource.version"))
        );
    }

    #[test]
    fn test_admission_response_patch_requires_patch_type() {
        let mut response = make_valid_response();
        response.patch = Some(ByteString(vec![1, 2, 3]));
        assert_eq!(
            response.validate(),
            Err(ValidationError::MissingField("patch/patchType"))
        );
    }

    #[test]
    fn test_admission_response_patch_type_requires_patch() {
        let mut response = make_valid_response();
        response.patch_type = Some(Box::new("JSONPatch".to_string()));
        assert_eq!(
            response.validate(),
            Err(ValidationError::MissingField("patch/patchType"))
        );
    }

    #[test]
    fn test_validation_error_display() {
        let err = ValidationError::MissingField("uid");
        assert_eq!(format!("{}", err), "missing required field: uid");

        let err = ValidationError::InvalidField("operation", "unknown value".to_string());
        assert_eq!(
            format!("{}", err),
            "invalid field 'operation': unknown value"
        );
    }
}
