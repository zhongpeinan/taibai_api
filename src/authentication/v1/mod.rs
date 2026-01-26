//! Kubernetes Authentication API v1 types
//!
//! This module contains the authentication v1 API types.
//!
//! Source: https://github.com/kubernetes/api/blob/master/authentication/v1/types.go

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::common::ObjectMeta;
use crate::common::{ApplyDefault, HasTypeMeta, ResourceSchema, TypeMeta, VersionedObject};
use crate::impl_unimplemented_prost_message;

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
    #[serde(flatten)]
    pub type_meta: TypeMeta,
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
            type_meta: TypeMeta::default(),
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
    #[serde(flatten)]
    pub type_meta: TypeMeta,
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
            type_meta: TypeMeta::default(),
            metadata: None,
            spec,
            status: None,
        }
    }
}

/// TokenRequestSpec contains client provided parameters of a token request.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
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
    #[serde(flatten)]
    pub type_meta: TypeMeta,
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
        <Self as Default>::default()
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

    #[test]
    fn test_token_review_apply_default() {
        let mut obj = TokenReview::default();
        obj.apply_default();
        assert_eq!(obj.type_meta.api_version, "authentication.k8s.io/v1");
        assert_eq!(obj.type_meta.kind, "TokenReview");
    }

    #[test]
    fn test_token_request_apply_default() {
        let mut obj = TokenRequest::default();
        obj.apply_default();
        assert_eq!(obj.type_meta.api_version, "authentication.k8s.io/v1");
        assert_eq!(obj.type_meta.kind, "TokenRequest");
        // Check default expiration_seconds
        assert_eq!(obj.spec.expiration_seconds, Some(3600));
    }

    #[test]
    fn test_token_request_spec_apply_default_sets_expiration() {
        let mut spec = TokenRequestSpec {
            audiences: vec!["api-server".to_string()],
            expiration_seconds: None,
            bound_object_ref: None,
        };
        spec.apply_default();
        assert_eq!(spec.expiration_seconds, Some(3600));
    }

    #[test]
    fn test_token_request_spec_apply_default_preserves_expiration() {
        let mut spec = TokenRequestSpec {
            audiences: vec!["api-server".to_string()],
            expiration_seconds: Some(7200),
            bound_object_ref: None,
        };
        spec.apply_default();
        assert_eq!(spec.expiration_seconds, Some(7200));
    }

    #[test]
    fn test_self_subject_review_apply_default() {
        let mut obj = SelfSubjectReview::default();
        obj.apply_default();
        assert_eq!(obj.type_meta.api_version, "authentication.k8s.io/v1");
        assert_eq!(obj.type_meta.kind, "SelfSubjectReview");
    }

    #[test]
    fn test_user_info_with_extra() {
        let mut extra = BTreeMap::new();
        extra.insert(
            "key".to_string(),
            vec!["value1".to_string(), "value2".to_string()],
        );

        let info = UserInfo {
            username: "test-user".to_string(),
            uid: "12345".to_string(),
            groups: vec!["system:authenticated".to_string()],
            extra,
        };

        assert_eq!(info.username, "test-user");
        assert_eq!(info.extra.len(), 1);
        assert_eq!(info.extra.get("key").unwrap().len(), 2);
    }
}

// ============================================================================
// Trait Implementations for Authentication Resources
// ============================================================================

// ----------------------------------------------------------------------------
// ResourceSchema Implementation
// ----------------------------------------------------------------------------

impl ResourceSchema for TokenReview {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "authentication.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "TokenReview"
    }
    fn resource(_: &Self::Meta) -> &str {
        "tokenreviews"
    }

    fn group_static() -> &'static str {
        "authentication.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "TokenReview"
    }
    fn resource_static() -> &'static str {
        "tokenreviews"
    }
}

impl ResourceSchema for TokenRequest {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "authentication.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "TokenRequest"
    }
    fn resource(_: &Self::Meta) -> &str {
        "tokenrequests"
    }

    fn group_static() -> &'static str {
        "authentication.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "TokenRequest"
    }
    fn resource_static() -> &'static str {
        "tokenrequests"
    }
}

impl ResourceSchema for SelfSubjectReview {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "authentication.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "SelfSubjectReview"
    }
    fn resource(_: &Self::Meta) -> &str {
        "selfsubjectreviews"
    }

    fn group_static() -> &'static str {
        "authentication.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "SelfSubjectReview"
    }
    fn resource_static() -> &'static str {
        "selfsubjectreviews"
    }
}

// ----------------------------------------------------------------------------
// HasTypeMeta Implementation
// ----------------------------------------------------------------------------

impl HasTypeMeta for TokenReview {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for TokenRequest {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for SelfSubjectReview {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

// ----------------------------------------------------------------------------
// VersionedObject Implementation
// ----------------------------------------------------------------------------

impl VersionedObject for TokenReview {
    fn metadata(&self) -> &ObjectMeta {
        use std::sync::OnceLock;
        self.metadata.as_ref().unwrap_or_else(|| {
            static DEFAULT: OnceLock<ObjectMeta> = OnceLock::new();
            DEFAULT.get_or_init(ObjectMeta::default)
        })
    }

    fn metadata_mut(&mut self) -> &mut ObjectMeta {
        self.metadata.get_or_insert_with(ObjectMeta::default)
    }
}

impl VersionedObject for TokenRequest {
    fn metadata(&self) -> &ObjectMeta {
        use std::sync::OnceLock;
        self.metadata.as_ref().unwrap_or_else(|| {
            static DEFAULT: OnceLock<ObjectMeta> = OnceLock::new();
            DEFAULT.get_or_init(ObjectMeta::default)
        })
    }

    fn metadata_mut(&mut self) -> &mut ObjectMeta {
        self.metadata.get_or_insert_with(ObjectMeta::default)
    }
}

impl VersionedObject for SelfSubjectReview {
    fn metadata(&self) -> &ObjectMeta {
        self.metadata
            .as_ref()
            .unwrap_or_else(|| static_default_object_meta())
    }

    fn metadata_mut(&mut self) -> &mut ObjectMeta {
        self.metadata.get_or_insert_with(ObjectMeta::default)
    }
}

// ----------------------------------------------------------------------------
// ApplyDefaults Implementation
// ----------------------------------------------------------------------------

impl ApplyDefault for TokenReview {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "authentication.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "TokenReview".to_string();
        }
    }
}

impl ApplyDefault for TokenRequest {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "authentication.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "TokenRequest".to_string();
        }
        self.spec.apply_default();
    }
}

impl ApplyDefault for TokenRequestSpec {
    fn apply_default(&mut self) {
        // Set default expiration to 1 hour if not specified
        // Source: k8s/pkg/apis/authentication/v1/defaults.go
        if self.expiration_seconds.is_none() {
            self.expiration_seconds = Some(60 * 60); // 1 hour
        }
    }
}

impl ApplyDefault for SelfSubjectReview {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "authentication.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "SelfSubjectReview".to_string();
        }
    }
}

// Helper function for static default ObjectMeta
fn static_default_object_meta() -> &'static ObjectMeta {
    use std::sync::OnceLock;
    static DEFAULT: OnceLock<ObjectMeta> = OnceLock::new();
    DEFAULT.get_or_init(ObjectMeta::default)
}

// ----------------------------------------------------------------------------
// Protobuf Placeholder
// ----------------------------------------------------------------------------

impl_unimplemented_prost_message!(TokenReview);
impl_unimplemented_prost_message!(TokenRequest);
impl_unimplemented_prost_message!(SelfSubjectReview);

#[cfg(test)]
mod trait_tests;
