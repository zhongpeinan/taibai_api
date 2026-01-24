//! Conversions between v1 and internal admission types

use crate::admission::internal;
use crate::admission::v1;
use crate::authentication::internal::UserInfo as InternalUserInfo;
use crate::authentication::v1::UserInfo as V1UserInfo;
use crate::common::{ApplyDefault, TypeMeta};

// ============================================================================
// UserInfo Conversions
// ============================================================================

impl From<V1UserInfo> for InternalUserInfo {
    fn from(value: V1UserInfo) -> Self {
        Self {
            username: value.username,
            uid: value.uid,
            groups: value.groups,
            extra: value.extra,
        }
    }
}

impl From<InternalUserInfo> for V1UserInfo {
    fn from(value: InternalUserInfo) -> Self {
        Self {
            username: value.username,
            uid: value.uid,
            groups: value.groups,
            extra: value.extra,
        }
    }
}

// ============================================================================
// AdmissionReview Conversions
// ============================================================================

impl From<v1::AdmissionReview> for internal::AdmissionReview {
    fn from(value: v1::AdmissionReview) -> Self {
        Self {
            request: value.request.map(Into::into),
            response: value.response.map(Into::into),
        }
    }
}

impl From<internal::AdmissionReview> for v1::AdmissionReview {
    fn from(value: internal::AdmissionReview) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            request: value.request.map(Into::into),
            response: value.response.map(Into::into),
        };
        result.apply_default();
        result
    }
}

// ============================================================================
// AdmissionRequest Conversions
// ============================================================================

impl From<v1::AdmissionRequest> for internal::AdmissionRequest {
    fn from(value: v1::AdmissionRequest) -> Self {
        Self {
            uid: value.uid,
            kind: value.kind,
            resource: value.resource,
            sub_resource: value.sub_resource,
            request_kind: value.request_kind,
            request_resource: value.request_resource,
            request_sub_resource: value.request_sub_resource,
            name: value.name,
            namespace: value.namespace,
            operation: value.operation,
            user_info: value.user_info.into(),
            object: value.object,
            old_object: value.old_object,
            dry_run: value.dry_run,
            options: value.options,
        }
    }
}

impl From<internal::AdmissionRequest> for v1::AdmissionRequest {
    fn from(value: internal::AdmissionRequest) -> Self {
        Self {
            uid: value.uid,
            kind: value.kind,
            resource: value.resource,
            sub_resource: value.sub_resource,
            request_kind: value.request_kind,
            request_resource: value.request_resource,
            request_sub_resource: value.request_sub_resource,
            name: value.name,
            namespace: value.namespace,
            operation: value.operation,
            user_info: value.user_info.into(),
            object: value.object,
            old_object: value.old_object,
            dry_run: value.dry_run,
            options: value.options,
        }
    }
}

// ============================================================================
// AdmissionResponse Conversions
// ============================================================================

impl From<v1::AdmissionResponse> for internal::AdmissionResponse {
    fn from(value: v1::AdmissionResponse) -> Self {
        Self {
            uid: value.uid,
            allowed: value.allowed,
            result: value.result,
            patch: value.patch,
            patch_type: value.patch_type,
            audit_annotations: value.audit_annotations,
            warnings: value.warnings,
        }
    }
}

impl From<internal::AdmissionResponse> for v1::AdmissionResponse {
    fn from(value: internal::AdmissionResponse) -> Self {
        Self {
            uid: value.uid,
            allowed: value.allowed,
            result: value.result,
            patch: value.patch,
            patch_type: value.patch_type,
            audit_annotations: value.audit_annotations,
            warnings: value.warnings,
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::GroupVersionKind;

    #[test]
    fn test_admission_review_round_trip() {
        let v1_review = v1::AdmissionReview {
            type_meta: TypeMeta {
                api_version: "admission.k8s.io/v1".to_string(),
                kind: "AdmissionReview".to_string(),
            },
            request: Some(v1::AdmissionRequest {
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
                operation: v1::operation::CREATE.to_string(),
                ..Default::default()
            }),
            response: None,
        };

        // v1 -> internal
        let internal: internal::AdmissionReview = v1_review.clone().into();

        // internal -> v1
        let v1_back: v1::AdmissionReview = internal.into();

        // Verify TypeMeta is correctly applied
        assert_eq!(v1_back.type_meta.api_version, "admission.k8s.io/v1");
        assert_eq!(v1_back.type_meta.kind, "AdmissionReview");

        // Verify request is preserved
        assert!(v1_back.request.is_some());
        let request = v1_back.request.unwrap();
        assert_eq!(request.uid, "test-uid");
        assert_eq!(request.kind.kind, "Pod");
        assert_eq!(request.operation, "CREATE");
    }

    #[test]
    fn test_admission_response_round_trip() {
        let v1_response = v1::AdmissionResponse {
            uid: "test-uid".to_string(),
            allowed: true,
            result: None,
            patch: None,
            patch_type: None,
            audit_annotations: Default::default(),
            warnings: vec!["warning1".to_string()],
        };

        let internal: internal::AdmissionResponse = v1_response.clone().into();
        let v1_back: v1::AdmissionResponse = internal.into();

        assert_eq!(v1_back.uid, "test-uid");
        assert!(v1_back.allowed);
        assert_eq!(v1_back.warnings, vec!["warning1".to_string()]);
    }

    #[test]
    fn test_admission_request_conversion() {
        let v1_request = v1::AdmissionRequest {
            uid: "req-uid".to_string(),
            kind: GroupVersionKind {
                group: "apps".to_string(),
                version: "v1".to_string(),
                kind: "Deployment".to_string(),
            },
            resource: crate::common::GroupVersionResource {
                group: "apps".to_string(),
                version: "v1".to_string(),
                resource: "deployments".to_string(),
            },
            sub_resource: "status".to_string(),
            name: "my-deployment".to_string(),
            namespace: "default".to_string(),
            operation: v1::operation::UPDATE.to_string(),
            ..Default::default()
        };

        let internal: internal::AdmissionRequest = v1_request.clone().into();
        let v1_back: v1::AdmissionRequest = internal.into();

        assert_eq!(v1_back.uid, "req-uid");
        assert_eq!(v1_back.kind.kind, "Deployment");
        assert_eq!(v1_back.resource.resource, "deployments");
        assert_eq!(v1_back.sub_resource, "status");
        assert_eq!(v1_back.name, "my-deployment");
        assert_eq!(v1_back.namespace, "default");
        assert_eq!(v1_back.operation, "UPDATE");
    }

    #[test]
    fn test_admission_review_with_response() {
        let internal_review = internal::AdmissionReview {
            request: None,
            response: Some(internal::AdmissionResponse {
                uid: "resp-uid".to_string(),
                allowed: false,
                result: None,
                patch: None,
                patch_type: None,
                audit_annotations: Default::default(),
                warnings: vec!["access denied".to_string()],
            }),
        };

        let v1_review: v1::AdmissionReview = internal_review.into();

        // Verify TypeMeta is correctly applied
        assert_eq!(v1_review.type_meta.api_version, "admission.k8s.io/v1");
        assert_eq!(v1_review.type_meta.kind, "AdmissionReview");

        // Verify response is preserved
        assert!(v1_review.response.is_some());
        let response = v1_review.response.unwrap();
        assert_eq!(response.uid, "resp-uid");
        assert!(!response.allowed);
        assert_eq!(response.warnings, vec!["access denied".to_string()]);
    }
}
