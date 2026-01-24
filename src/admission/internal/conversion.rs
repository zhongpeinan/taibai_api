//! Conversions between v1 and internal admission types

use crate::admission::internal;
use crate::admission::v1;
use crate::authentication::internal::UserInfo as InternalUserInfo;
use crate::authentication::v1::UserInfo as V1UserInfo;
use crate::common::{ApplyDefault, FromInternal, ToInternal, TypeMeta};

// ============================================================================
// UserInfo Conversions
// ============================================================================

impl ToInternal<InternalUserInfo> for V1UserInfo {
    fn to_internal(self) -> InternalUserInfo {
        InternalUserInfo {
            username: self.username,
            uid: self.uid,
            groups: self.groups,
            extra: self.extra,
        }
    }
}

impl FromInternal<InternalUserInfo> for V1UserInfo {
    fn from_internal(value: InternalUserInfo) -> Self {
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

impl ToInternal<internal::AdmissionReview> for v1::AdmissionReview {
    fn to_internal(self) -> internal::AdmissionReview {
        internal::AdmissionReview {
            request: self.request.map(|request| request.to_internal()),
            response: self.response.map(|response| response.to_internal()),
        }
    }
}

impl FromInternal<internal::AdmissionReview> for v1::AdmissionReview {
    fn from_internal(value: internal::AdmissionReview) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            request: value
                .request
                .map(|request| v1::AdmissionRequest::from_internal(request)),
            response: value
                .response
                .map(|response| v1::AdmissionResponse::from_internal(response)),
        };
        result.apply_default();
        result
    }
}

// ============================================================================
// AdmissionRequest Conversions
// ============================================================================

impl ToInternal<internal::AdmissionRequest> for v1::AdmissionRequest {
    fn to_internal(self) -> internal::AdmissionRequest {
        internal::AdmissionRequest {
            uid: self.uid,
            kind: self.kind,
            resource: self.resource,
            sub_resource: self.sub_resource,
            request_kind: self.request_kind,
            request_resource: self.request_resource,
            request_sub_resource: self.request_sub_resource,
            name: self.name,
            namespace: self.namespace,
            operation: self.operation,
            user_info: self.user_info.to_internal(),
            object: self.object,
            old_object: self.old_object,
            dry_run: self.dry_run,
            options: self.options,
        }
    }
}

impl FromInternal<internal::AdmissionRequest> for v1::AdmissionRequest {
    fn from_internal(value: internal::AdmissionRequest) -> Self {
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
            user_info: V1UserInfo::from_internal(value.user_info),
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

impl ToInternal<internal::AdmissionResponse> for v1::AdmissionResponse {
    fn to_internal(self) -> internal::AdmissionResponse {
        internal::AdmissionResponse {
            uid: self.uid,
            allowed: self.allowed,
            result: self.result,
            patch: self.patch,
            patch_type: self.patch_type,
            audit_annotations: self.audit_annotations,
            warnings: self.warnings,
        }
    }
}

impl FromInternal<internal::AdmissionResponse> for v1::AdmissionResponse {
    fn from_internal(value: internal::AdmissionResponse) -> Self {
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
    use crate::common::{FromInternal, ToInternal};

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
        let internal = v1_review.clone().to_internal();

        // internal -> v1
        let v1_back = v1::AdmissionReview::from_internal(internal);

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

        let internal = v1_response.clone().to_internal();
        let v1_back = v1::AdmissionResponse::from_internal(internal);

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

        let internal = v1_request.clone().to_internal();
        let v1_back = v1::AdmissionRequest::from_internal(internal);

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

        let v1_review = v1::AdmissionReview::from_internal(internal_review);

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
