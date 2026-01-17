//! Kubernetes Authentication API v1 types
//!
//! This module contains the authentication v1 API types.
//!
//! Source: https://github.com/kubernetes/api/blob/master/authentication/v1/types.go

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::common::ObjectMeta;

// ============================================================================
// Constants
// ============================================================================

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
/// Note: TokenReview requests may be cached by the webhook token authenticator
/// plugin in the kube-apiserver.
///
/// Corresponds to [Kubernetes TokenReview](https://github.com/kubernetes/api/blob/master/authentication/v1/types.go#L50)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TokenReview {
    /// Standard object's metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// Spec holds information about the request being evaluated.
    pub spec: TokenReviewSpec,
    /// Status is filled in by the server and indicates whether the request can be authenticated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<TokenReviewStatus>,
}

impl TokenReview {
    /// Creates a new TokenReview with the given spec.
    pub fn new(spec: TokenReviewSpec) -> Self {
        Self {
            metadata: None,
            spec,
            status: None,
        }
    }
}

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
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TokenReviewStatus {
    /// Authenticated indicates that the token was associated with a known user.
    #[serde(default)]
    pub authenticated: bool,
    /// User is the UserInfo associated with the provided token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<UserInfo>,
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
/// Corresponds to [Kubernetes UserInfo](https://github.com/kubernetes/api/blob/master/authentication/v1/types.go#L110)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    /// The name that uniquely identifies this user among all active users.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub username: String,
    /// A unique value that identifies this user across time. If this user is
    /// deleted and another user by the same name is added, they will have
    /// different UIDs.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub uid: String,
    /// The names of groups this user is a part of.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<String>,
    /// Any additional information provided by the authenticator.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extra: BTreeMap<String, ExtraValue>,
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
/// Corresponds to [Kubernetes TokenRequest](https://github.com/kubernetes/api/blob/master/authentication/v1/types.go#L140)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TokenRequest {
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// Spec holds information about the request being evaluated.
    pub spec: TokenRequestSpec,
    /// Status is filled in by the server and indicates whether the token can be authenticated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<TokenRequestStatus>,
}

impl TokenRequest {
    /// Creates a new TokenRequest with the given spec.
    pub fn new(spec: TokenRequestSpec) -> Self {
        Self {
            metadata: None,
            spec,
            status: None,
        }
    }
}

/// TokenRequestSpec contains client provided parameters of a token request.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TokenRequestSpec {
    /// Audiences are the intended audiences of the token.
    pub audiences: Vec<String>,
    /// ExpirationSeconds is the requested duration of validity of the request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_seconds: Option<i64>,
    /// BoundObjectRef is a reference to an object that the token will be bound to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bound_object_ref: Option<BoundObjectReference>,
}

impl Default for TokenRequestSpec {
    fn default() -> Self {
        Self {
            audiences: Vec::new(),
            expiration_seconds: None,
            bound_object_ref: None,
        }
    }
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
/// When using impersonation, users will receive the user info of the user being impersonated.
///
/// Corresponds to [Kubernetes SelfSubjectReview](https://github.com/kubernetes/api/blob/master/authentication/v1/types.go#L213)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SelfSubjectReview {
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// Status is filled in by the server with the user attributes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<SelfSubjectReviewStatus>,
}

impl SelfSubjectReview {
    /// Creates a new SelfSubjectReview.
    pub fn new() -> Self {
        Self::default()
    }
}

/// SelfSubjectReviewStatus is filled by the kube-apiserver and sent back to a user.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SelfSubjectReviewStatus {
    /// User attributes of the user making this request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_info: Option<UserInfo>,
}

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================================
    // UserInfo Tests
    // ============================================================================

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
            ..Default::default()
        };
        assert_eq!(info.username, "admin");
        assert_eq!(info.uid, "user-123");
        assert_eq!(info.groups.len(), 1);
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
    fn test_user_info_with_groups() {
        let info = UserInfo {
            username: "developer".to_string(),
            groups: vec!["developers".to_string(), "readers".to_string()],
            ..Default::default()
        };
        assert_eq!(info.groups.len(), 2);
        assert!(info.groups.contains(&"developers".to_string()));
    }

    #[test]
    fn test_user_info_with_extra() {
        let mut extra = BTreeMap::new();
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
    fn test_user_info_round_trip() {
        let mut extra = BTreeMap::new();
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
    fn test_user_info_empty_fields_omitted() {
        let info = UserInfo {
            username: "minimal".to_string(),
            ..Default::default()
        };
        let json = serde_json::to_string(&info).unwrap();
        assert!(json.contains("\"username\":\"minimal\""));
        assert!(!json.contains("uid"));
        assert!(!json.contains("groups"));
        assert!(!json.contains("extra"));
    }

    // ============================================================================
    // TokenReview Tests
    // ============================================================================

    #[test]
    fn test_token_review_default() {
        let review = TokenReview::default();
        assert!(review.metadata.is_none());
        assert!(review.status.is_none());
    }

    #[test]
    fn test_token_review_new() {
        let spec = TokenReviewSpec {
            token: "my-token".to_string(),
            ..Default::default()
        };
        let review = TokenReview::new(spec);
        assert!(review.metadata.is_none());
        assert!(review.status.is_none());
        assert_eq!(review.spec.token, "my-token");
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
        assert!(status.user.is_none());
        assert!(status.audiences.is_empty());
        assert!(status.error.is_empty());
    }

    #[test]
    fn test_token_review_serialize() {
        let review = TokenReview {
            metadata: None,
            spec: TokenReviewSpec {
                token: "test-token".to_string(),
                audiences: vec!["api-server".to_string()],
            },
            status: Some(TokenReviewStatus {
                authenticated: true,
                user: Some(UserInfo {
                    username: "admin".to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
        };
        let json = serde_json::to_string(&review).unwrap();
        assert!(json.contains("\"token\":\"test-token\""));
        assert!(json.contains("\"authenticated\":true"));
    }

    #[test]
    fn test_token_review_deserialize() {
        let json = r#"{"spec":{"token":"my-token"},"status":{"authenticated":true}}"#;
        let review: TokenReview = serde_json::from_str(json).unwrap();
        assert_eq!(review.spec.token, "my-token");
        assert!(review.status.is_some());
        assert!(review.status.unwrap().authenticated);
    }

    // ============================================================================
    // TokenRequest Tests
    // ============================================================================

    #[test]
    fn test_token_request_default() {
        let request = TokenRequest::default();
        assert!(request.metadata.is_none());
        assert!(request.status.is_none());
    }

    #[test]
    fn test_token_request_new() {
        let spec = TokenRequestSpec {
            audiences: vec!["api-server".to_string()],
            expiration_seconds: Some(3600),
            ..Default::default()
        };
        let request = TokenRequest::new(spec);
        assert_eq!(request.spec.audiences.len(), 1);
        assert_eq!(request.spec.expiration_seconds, Some(3600));
    }

    #[test]
    fn test_token_request_spec_default() {
        let spec = TokenRequestSpec::default();
        assert!(spec.audiences.is_empty());
        assert!(spec.expiration_seconds.is_none());
        assert!(spec.bound_object_ref.is_none());
    }

    #[test]
    fn test_token_request_status_default() {
        let status = TokenRequestStatus::default();
        assert!(status.token.is_empty());
        assert!(status.expiration_timestamp.as_str().is_empty());
    }

    #[test]
    fn test_bound_object_reference_default() {
        let bound_ref = BoundObjectReference::default();
        assert!(bound_ref.kind.is_empty());
        assert!(bound_ref.api_version.is_empty());
        assert!(bound_ref.name.is_empty());
        assert!(bound_ref.uid.is_empty());
    }

    #[test]
    fn test_token_request_serialize() {
        let request = TokenRequest {
            metadata: None,
            spec: TokenRequestSpec {
                audiences: vec!["api".to_string()],
                expiration_seconds: Some(7200),
                bound_object_ref: Some(BoundObjectReference {
                    kind: "Pod".to_string(),
                    name: "my-pod".to_string(),
                    ..Default::default()
                }),
            },
            status: None,
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"audiences\":[\"api\"]"));
        assert!(json.contains("\"expirationSeconds\":7200"));
    }

    // ============================================================================
    // SelfSubjectReview Tests
    // ============================================================================

    #[test]
    fn test_self_subject_review_default() {
        let review = SelfSubjectReview::default();
        assert!(review.metadata.is_none());
        assert!(review.status.is_none());
    }

    #[test]
    fn test_self_subject_review_new() {
        let review = SelfSubjectReview::new();
        assert!(review.metadata.is_none());
        assert!(review.status.is_none());
    }

    #[test]
    fn test_self_subject_review_status_default() {
        let status = SelfSubjectReviewStatus::default();
        assert!(status.user_info.is_none());
    }

    #[test]
    fn test_self_subject_review_with_status() {
        let review = SelfSubjectReview {
            metadata: None,
            status: Some(SelfSubjectReviewStatus {
                user_info: Some(UserInfo {
                    username: "test-user".to_string(),
                    ..Default::default()
                }),
            }),
        };
        assert!(review.status.is_some());
        assert_eq!(
            review.status.unwrap().user_info.unwrap().username,
            "test-user"
        );
    }

    // ============================================================================
    // Constants Tests
    // ============================================================================

    #[test]
    fn test_impersonation_headers() {
        assert_eq!(IMPERSONATE_USER_HEADER, "Impersonate-User");
        assert_eq!(IMPERSONATE_UID_HEADER, "Impersonate-Uid");
        assert_eq!(IMPERSONATE_GROUP_HEADER, "Impersonate-Group");
        assert_eq!(IMPERSONATE_USER_EXTRA_HEADER_PREFIX, "Impersonate-Extra-");
    }

    // ============================================================================
    // Round-trip Tests
    // ============================================================================

    #[test]
    fn test_token_review_round_trip() {
        let original = TokenReview {
            metadata: None,
            spec: TokenReviewSpec {
                token: "abc123".to_string(),
                audiences: vec!["kubernetes".to_string()],
            },
            status: Some(TokenReviewStatus {
                authenticated: true,
                user: Some(UserInfo {
                    username: "jane".to_string(),
                    uid: "uid-123".to_string(),
                    groups: vec!["developers".to_string()],
                    extra: BTreeMap::new(),
                }),
                audiences: vec!["kubernetes".to_string()],
                error: String::new(),
            }),
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: TokenReview = serde_json::from_str(&json).unwrap();
        assert_eq!(original.spec.token, deserialized.spec.token);
        assert_eq!(
            original.status.as_ref().unwrap().authenticated,
            deserialized.status.as_ref().unwrap().authenticated
        );
    }

    #[test]
    fn test_token_request_round_trip() {
        let original = TokenRequest {
            metadata: None,
            spec: TokenRequestSpec {
                audiences: vec!["api".to_string()],
                expiration_seconds: Some(3600),
                bound_object_ref: Some(BoundObjectReference {
                    kind: "Secret".to_string(),
                    name: "my-secret".to_string(),
                    uid: "secret-uid".to_string(),
                    ..Default::default()
                }),
            },
            status: Some(TokenRequestStatus {
                token: "generated-token".to_string(),
                expiration_timestamp: crate::common::time::Timestamp::from_str(
                    "2024-01-15T10:00:00Z",
                ),
            }),
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: TokenRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(
            original.spec.expiration_seconds,
            deserialized.spec.expiration_seconds
        );
    }

    #[test]
    fn test_extra_value_type() {
        let extra_value: ExtraValue = vec!["value1".to_string(), "value2".to_string()];
        assert_eq!(extra_value.len(), 2);
        assert_eq!(extra_value[0], "value1");
    }
}
