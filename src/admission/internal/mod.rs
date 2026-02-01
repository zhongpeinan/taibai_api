//! Kubernetes Admission internal API types
//!
//! This module contains the admission internal API types.
//!
//! Internal types are identical to v1 types but without TypeMeta.
//! The TypeMeta is dropped during v1 -> internal conversion
//! and reapplied during internal -> v1 conversion.

use crate::authentication::internal::UserInfo;
use crate::common::{GroupVersionKind, GroupVersionResource, Status};
use crate::core::internal::ByteString;
use crate::impl_unimplemented_prost_message;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

pub mod conversion;
pub mod validation;

/// PatchType is the type of patch being used to represent the mutated object.
pub type PatchType = String;

/// PatchType constants.
pub mod patch_type {
    /// JSONPatch is the only supported patch type
    pub const JSON_PATCH: &str = "JSONPatch";
}

/// Operation is the type of resource operation being checked for admission control.
pub type Operation = String;

/// Operation constants
pub mod operation {
    /// CREATE operation
    pub const CREATE: &str = "CREATE";
    /// UPDATE operation
    pub const UPDATE: &str = "UPDATE";
    /// DELETE operation
    pub const DELETE: &str = "DELETE";
    /// CONNECT operation
    pub const CONNECT: &str = "CONNECT";
}

/// AdmissionReview describes an admission review request/response.
///
/// Internal version does not have TypeMeta (it's dropped during conversion).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AdmissionReview {
    /// Request describes the attributes for the admission request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<AdmissionRequest>,
    /// Response describes the attributes for the admission response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response: Option<AdmissionResponse>,
}

/// AdmissionRequest describes the admission.Attributes for the admission request.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AdmissionRequest {
    /// UID is an identifier for the individual request/response.
    pub uid: String,
    /// Kind is the fully-qualified type of object being submitted.
    pub kind: GroupVersionKind,
    /// Resource is the fully-qualified resource being requested.
    pub resource: GroupVersionResource,
    /// SubResource is the subresource being requested, if any.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub sub_resource: String,
    /// RequestKind is the fully-qualified type of the original API request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_kind: Option<Box<GroupVersionKind>>,
    /// RequestResource is the fully-qualified resource of the original API request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_resource: Option<Box<GroupVersionResource>>,
    /// RequestSubResource is the name of the subresource of the original API request.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub request_sub_resource: String,
    /// Name is the name of the object as presented in the request.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// Namespace is the namespace associated with the request.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub namespace: String,
    /// Operation is the operation being performed.
    pub operation: Operation,
    /// UserInfo is information about the requesting user.
    #[serde(rename = "userInfo")]
    pub user_info: UserInfo,
    /// Object is the object from the incoming request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<Value>,
    /// OldObject is the existing object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub old_object: Option<Value>,
    /// DryRun indicates that modifications will definitely not be persisted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// Options is the operation option structure.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<Value>,
}

/// AdmissionResponse describes an admission response.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AdmissionResponse {
    /// UID is an identifier for the individual request/response.
    pub uid: String,
    /// Allowed indicates whether or not the admission request was permitted.
    pub allowed: bool,
    /// Result contains extra details into why an admission request was denied.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub result: Option<Status>,
    /// Patch is the patch body.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patch: Option<ByteString>,
    /// PatchType is the type of Patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patch_type: Option<Box<PatchType>>,
    /// AuditAnnotations is an unstructured key value map.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub audit_annotations: BTreeMap<String, String>,
    /// Warnings is a list of warning messages.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<String>,
}

// ===========================================================================
// Protobuf Placeholder Implementations
// ===========================================================================

impl_unimplemented_prost_message!(AdmissionReview);
impl_unimplemented_prost_message!(AdmissionRequest);
impl_unimplemented_prost_message!(AdmissionResponse);

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // Compile-time Trait Checks
    // ========================================================================

    /// 编译时检查：确保内部版本资源实现了 prost::Message
    #[test]
    fn prost_message() {
        fn check<T: prost::Message>() {}

        check::<AdmissionReview>();
        check::<AdmissionRequest>();
        check::<AdmissionResponse>();
    }

    // ========================================================================
    // Runtime Behavior Tests
    // ========================================================================

    #[test]
    fn test_internal_admission_review_default() {
        let review = AdmissionReview::default();
        assert!(review.request.is_none());
        assert!(review.response.is_none());
    }

    #[test]
    fn test_internal_admission_request_default() {
        let request = AdmissionRequest::default();
        assert_eq!(request.uid, "");
        assert_eq!(request.operation, "");
    }

    #[test]
    fn test_internal_admission_response_default() {
        let response = AdmissionResponse::default();
        assert_eq!(response.uid, "");
        assert!(!response.allowed);
        assert!(response.warnings.is_empty());
    }
}
