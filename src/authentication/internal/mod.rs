//! Kubernetes Authentication API Internal Types
//!
//! This module contains type definitions from k8s.io/kubernetes/pkg/apis/authentication/types.go
//! that are used internally by the Kubernetes API.
//!
//! Source: https://github.com/kubernetes/apimachinery/blob/master/pkg/apis/meta/v1/types.go

use crate::common::{ObjectMeta, TypeMeta};
use crate::impl_has_object_meta;
use serde::{Deserialize, Serialize};

/// Impersonate-User header is used to impersonate a particular user during an API server request.
pub const IMPERSONATE_USER_HEADER: &str = "Impersonate-User";

/// Impersonate-UID header is used to impersonate a particular UID during an API server request.
pub const IMPERSONATE_UID_HEADER: &str = "Impersonate-Uid";

/// Impersonate-Group header is used to impersonate a particular group during an API server request.
pub const IMPERSONATE_GROUP_HEADER: &str = "Impersonate-Group";

/// Impersonate-Extra- header prefix is used to impersonate extra map entries.
pub const IMPERSONATE_USER_EXTRA_HEADER_PREFIX: &str = "Impersonate-Extra-";

// ============================================================================
// TokenReview
// ============================================================================

/// TokenReview attempts to authenticate a token to a known user.
///
/// Corresponds to [Kubernetes TokenReview](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/authentication/types.go#L44)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct TokenReview {
    /// TypeMeta describes the type of this object.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    pub metadata: ObjectMeta,
    /// Spec holds information about the request being evaluated.
    pub spec: TokenReviewSpec,
    /// Status is filled in by the server and indicates whether the request can be authenticated.
    pub status: TokenReviewStatus,
}
impl_has_object_meta!(TokenReview);

/// TokenReviewSpec is a description of the token authentication request.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TokenReviewSpec {
    /// Token is the opaque bearer token.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub token: String,
    /// Audiences is a list of the identifiers that the resource server presented
    /// with the token identifies as.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub audiences: Vec<String>,
}

/// TokenReviewStatus is the result of the token authentication request.
///
/// This type mirrors the authentication.Token interface.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TokenReviewStatus {
    /// Authenticated indicates that the token was associated with a known user.
    #[serde(default)]
    pub authenticated: bool,
    /// User is the UserInfo associated with the provided token.
    #[serde(default)]
    pub user: UserInfo,
    /// Audiences are audience identifiers chosen by the authenticator.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub audiences: Vec<String>,
    /// Error indicates that the token couldn't be checked.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub error: String,
}

// ============================================================================
// UserInfo
// ============================================================================

/// UserInfo holds the information about the user needed to implement the
/// user.Info interface.
///
/// Corresponds to [Kubernetes UserInfo](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/authentication/types.go#L91)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    /// The name that uniquely identifies this user among all active users.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub username: String,
    /// A unique value that identifies this user across time.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub uid: String,
    /// The names of groups this user is a part of.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<String>,
    /// Any additional information provided by the authenticator.
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub extra: std::collections::BTreeMap<String, ExtraValue>,
}

/// ExtraValue masks the value so protobuf can generate.
///
/// This is a type alias for Vec<String> for compatibility with Kubernetes API.
pub type ExtraValue = Vec<String>;

// ============================================================================
// TokenRequest
// ============================================================================

/// TokenRequest requests a token for a given service account.
///
/// Corresponds to [Kubernetes TokenRequest](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/authentication/types.go#L110)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct TokenRequest {
    /// TypeMeta describes the type of this object.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    pub metadata: ObjectMeta,
    /// Spec holds information about the request being evaluated.
    pub spec: TokenRequestSpec,
    /// Status is the result of a token request.
    pub status: TokenRequestStatus,
}
impl_has_object_meta!(TokenRequest);

/// TokenRequestSpec contains client provided parameters of a token request.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TokenRequestSpec {
    /// Audiences are the intended audiences of the token.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub audiences: Vec<String>,
    /// ExpirationSeconds is the requested duration of validity of the request.
    #[serde(default)]
    pub expiration_seconds: i64,
    /// BoundObjectRef is a reference to an object that the token will be bound to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bound_object_ref: Option<BoundObjectReference>,
}

/// TokenRequestStatus is the result of a token request.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TokenRequestStatus {
    /// Token is the opaque bearer token.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub token: String,
    /// ExpirationTimestamp is the time of expiration of the returned token.
    #[serde(default)]
    pub expiration_timestamp: crate::common::time::Timestamp,
}

/// BoundObjectReference is a reference to an object that a token is bound to.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct BoundObjectReference {
    /// Kind of the referent. Valid kinds are 'Pod' and 'Secret'.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub kind: String,
    /// API version of the referent.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub api_version: String,
    /// Name of the referent.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// UID of the referent.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub uid: String,
}

// ============================================================================
// SelfSubjectReview
// ============================================================================

/// SelfSubjectReview contains the user information that the kube-apiserver has
/// about the user making this request.
///
/// Corresponds to [Kubernetes SelfSubjectReview](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/authentication/types.go#L166)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SelfSubjectReview {
    /// TypeMeta describes the type of this object.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    pub metadata: ObjectMeta,
    /// Status is filled in by the server with the user attributes.
    pub status: SelfSubjectReviewStatus,
}
impl_has_object_meta!(SelfSubjectReview);

/// SelfSubjectReviewStatus is filled by the kube-apiserver and sent back to a user.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SelfSubjectReviewStatus {
    /// User attributes of the user making this request.
    #[serde(default)]
    pub user_info: UserInfo,
}

// ============================================================================
// Constants
// ============================================================================

/// Constants for impersonation headers.
pub mod header {
    pub use super::*;
    pub const IMPERSONATE_USER: &str = IMPERSONATE_USER_HEADER;
    pub const IMPERSONATE_UID: &str = IMPERSONATE_UID_HEADER;
    pub const IMPERSONATE_GROUP: &str = IMPERSONATE_GROUP_HEADER;
    pub const IMPERSONATE_EXTRA_PREFIX: &str = IMPERSONATE_USER_EXTRA_HEADER_PREFIX;
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_info_default() {
        let info = UserInfo::default();
        assert!(info.username.is_empty());
        assert!(info.uid.is_empty());
        assert!(info.groups.is_empty());
        assert!(info.extra.is_empty());
    }

    #[test]
    fn test_user_info_with_fields() {
        let info = UserInfo {
            username: "admin".to_string(),
            uid: "user-123".to_string(),
            groups: vec!["system:masters".to_string()],
            extra: std::collections::BTreeMap::new(),
        };
        assert_eq!(info.username, "admin");
        assert_eq!(info.uid, "user-123");
        assert_eq!(info.groups.len(), 1);
    }

    #[test]
    fn test_user_info_with_extra() {
        let mut extra = std::collections::BTreeMap::new();
        extra.insert(
            "scopes".to_string(),
            vec!["read".to_string(), "write".to_string()],
        );

        let info = UserInfo {
            username: "svc-account".to_string(),
            extra,
            ..Default::default()
        };
        assert!(info.extra.contains_key("scopes"));
        assert_eq!(info.extra.get("scopes").unwrap().len(), 2);
    }

    #[test]
    fn test_token_review_spec_default() {
        let spec = TokenReviewSpec::default();
        assert!(spec.token.is_empty());
        assert!(spec.audiences.is_empty());
    }

    #[test]
    fn test_token_review_status_default() {
        let status = TokenReviewStatus::default();
        assert!(!status.authenticated);
        assert_eq!(status.user.username, "");
        assert!(status.audiences.is_empty());
        assert!(status.error.is_empty());
    }

    #[test]
    fn test_bound_object_reference_default() {
        let ref_obj = BoundObjectReference::default();
        assert!(ref_obj.kind.is_empty());
        assert!(ref_obj.api_version.is_empty());
        assert!(ref_obj.name.is_empty());
        assert!(ref_obj.uid.is_empty());
    }

    #[test]
    fn test_token_request_spec_default() {
        let spec = TokenRequestSpec::default();
        assert!(spec.audiences.is_empty());
        assert_eq!(spec.expiration_seconds, 0);
        assert!(spec.bound_object_ref.is_none());
    }

    #[test]
    fn test_token_request_status_default() {
        let status = TokenRequestStatus::default();
        assert!(status.token.is_empty());
        assert_eq!(status.expiration_timestamp.timestamp(), 0);
    }

    #[test]
    fn test_self_subject_review_status_default() {
        let status = SelfSubjectReviewStatus::default();
        assert!(status.user_info.username.is_empty());
        assert!(status.user_info.uid.is_empty());
    }

    #[test]
    fn test_user_info_serialize() {
        let info = UserInfo {
            username: "test-user".to_string(),
            uid: "uid-456".to_string(),
            ..Default::default()
        };
        let json = serde_json::to_string(&info).unwrap();
        assert!(json.contains("\"username\":\"test-user\""));
        assert!(json.contains("\"uid\":\"uid-456\""));
    }

    #[test]
    fn test_user_info_deserialize() {
        let json = r#"{"username":"admin","uid":"admin-uid"}"#;
        let info: UserInfo = serde_json::from_str(json).unwrap();
        assert_eq!(info.username, "admin");
        assert_eq!(info.uid, "admin-uid");
    }

    #[test]
    fn test_token_review_serialize() {
        let review = TokenReview {
            type_meta: TypeMeta::default(),
            metadata: ObjectMeta::default(),
            spec: TokenReviewSpec {
                token: "my-token".to_string(),
                ..Default::default()
            },
            status: TokenReviewStatus::default(),
        };
        let json = serde_json::to_string(&review).unwrap();
        assert!(json.contains("\"token\":\"my-token\""));
    }

    #[test]
    fn test_bound_object_reference_with_fields() {
        let bound_ref = BoundObjectReference {
            kind: "Pod".to_string(),
            name: "my-pod".to_string(),
            uid: "pod-123".to_string(),
            ..Default::default()
        };
        assert_eq!(bound_ref.kind, "Pod");
        assert_eq!(bound_ref.name, "my-pod");
        assert_eq!(bound_ref.uid, "pod-123");
    }

    #[test]
    fn test_impersonation_headers() {
        assert_eq!(IMPERSONATE_USER_HEADER, "Impersonate-User");
        assert_eq!(IMPERSONATE_UID_HEADER, "Impersonate-Uid");
        assert_eq!(IMPERSONATE_GROUP_HEADER, "Impersonate-Group");
        assert_eq!(IMPERSONATE_USER_EXTRA_HEADER_PREFIX, "Impersonate-Extra-");
    }

    #[test]
    fn test_user_info_round_trip() {
        let mut extra = std::collections::BTreeMap::new();
        extra.insert("key".to_string(), vec!["value".to_string()]);

        let original = UserInfo {
            username: "user".to_string(),
            uid: "uid".to_string(),
            groups: vec!["group1".to_string()],
            extra,
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: UserInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(original.username, deserialized.username);
        assert_eq!(original.uid, deserialized.uid);
        assert_eq!(original.groups, deserialized.groups);
    }

    #[test]
    fn test_extra_value_type() {
        let extra_value: ExtraValue = vec!["value1".to_string(), "value2".to_string()];
        assert_eq!(extra_value.len(), 2);
        assert_eq!(extra_value[0], "value1");
    }
}
