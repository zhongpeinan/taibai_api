//! Conversions between versioned and internal Authentication API types
//!
//! Source: k8s/pkg/apis/authentication/v1/zz_generated.conversion.go

use crate::authentication::internal as int;
use crate::authentication::v1;
use crate::common::{ApplyDefault, FromInternal, ToInternal, TypeMeta};

fn to_internal_user_info(value: v1::UserInfo) -> int::UserInfo {
    int::UserInfo {
        username: value.username,
        uid: value.uid,
        groups: value.groups,
        extra: value.extra,
    }
}

fn from_internal_user_info(value: int::UserInfo) -> v1::UserInfo {
    v1::UserInfo {
        username: value.username,
        uid: value.uid,
        groups: value.groups,
        extra: value.extra,
    }
}

fn to_internal_bound_object_reference(
    value: v1::BoundObjectReference,
) -> int::BoundObjectReference {
    int::BoundObjectReference {
        kind: value.kind,
        api_version: value.api_version,
        name: value.name,
        uid: value.uid,
    }
}

fn from_internal_bound_object_reference(
    value: int::BoundObjectReference,
) -> v1::BoundObjectReference {
    v1::BoundObjectReference {
        kind: value.kind,
        api_version: value.api_version,
        name: value.name,
        uid: value.uid,
    }
}

fn to_internal_token_review_spec(value: v1::TokenReviewSpec) -> int::TokenReviewSpec {
    int::TokenReviewSpec {
        token: value.token,
        audiences: value.audiences,
    }
}

fn from_internal_token_review_spec(value: int::TokenReviewSpec) -> v1::TokenReviewSpec {
    v1::TokenReviewSpec {
        token: value.token,
        audiences: value.audiences,
    }
}

fn to_internal_token_review_status(value: v1::TokenReviewStatus) -> int::TokenReviewStatus {
    int::TokenReviewStatus {
        authenticated: value.authenticated,
        user: value
            .user
            .map_or_else(int::UserInfo::default, to_internal_user_info),
        audiences: value.audiences,
        error: value.error,
    }
}

fn from_internal_token_review_status(
    value: int::TokenReviewStatus,
) -> Option<v1::TokenReviewStatus> {
    if value == int::TokenReviewStatus::default() {
        return None;
    }

    let user = if value.user == int::UserInfo::default() {
        None
    } else {
        Some(from_internal_user_info(value.user))
    };

    Some(v1::TokenReviewStatus {
        authenticated: value.authenticated,
        user,
        audiences: value.audiences,
        error: value.error,
    })
}

fn to_internal_token_request_spec(value: v1::TokenRequestSpec) -> int::TokenRequestSpec {
    int::TokenRequestSpec {
        audiences: value.audiences,
        expiration_seconds: value.expiration_seconds.unwrap_or(0),
        bound_object_ref: value
            .bound_object_ref
            .map(to_internal_bound_object_reference),
    }
}

fn from_internal_token_request_spec(value: int::TokenRequestSpec) -> v1::TokenRequestSpec {
    v1::TokenRequestSpec {
        audiences: value.audiences,
        expiration_seconds: Some(value.expiration_seconds),
        bound_object_ref: value
            .bound_object_ref
            .map(from_internal_bound_object_reference),
    }
}

fn to_internal_token_request_status(value: v1::TokenRequestStatus) -> int::TokenRequestStatus {
    int::TokenRequestStatus {
        token: value.token,
        expiration_timestamp: value.expiration_timestamp,
    }
}

fn from_internal_token_request_status(
    value: int::TokenRequestStatus,
) -> Option<v1::TokenRequestStatus> {
    if value == int::TokenRequestStatus::default() {
        return None;
    }

    Some(v1::TokenRequestStatus {
        token: value.token,
        expiration_timestamp: value.expiration_timestamp,
    })
}

fn to_internal_self_subject_review_status(
    value: v1::SelfSubjectReviewStatus,
) -> int::SelfSubjectReviewStatus {
    int::SelfSubjectReviewStatus {
        user_info: value
            .user_info
            .map_or_else(int::UserInfo::default, to_internal_user_info),
    }
}

fn from_internal_self_subject_review_status(
    value: int::SelfSubjectReviewStatus,
) -> Option<v1::SelfSubjectReviewStatus> {
    if value == int::SelfSubjectReviewStatus::default() {
        return None;
    }

    let user_info = if value.user_info == int::UserInfo::default() {
        None
    } else {
        Some(from_internal_user_info(value.user_info))
    };

    Some(v1::SelfSubjectReviewStatus { user_info })
}

impl ToInternal<int::TokenReview> for v1::TokenReview {
    fn to_internal(self) -> int::TokenReview {
        int::TokenReview {
            type_meta: TypeMeta::default(),
            metadata: self.metadata.unwrap_or_default(),
            spec: to_internal_token_review_spec(self.spec),
            status: self.status.map_or_else(
                int::TokenReviewStatus::default,
                to_internal_token_review_status,
            ),
        }
    }
}

impl FromInternal<int::TokenReview> for v1::TokenReview {
    fn from_internal(internal: int::TokenReview) -> Self {
        let mut out = v1::TokenReview {
            type_meta: TypeMeta::default(),
            metadata: Some(internal.metadata),
            spec: from_internal_token_review_spec(internal.spec),
            status: from_internal_token_review_status(internal.status),
        };
        out.apply_default();
        out
    }
}

impl ToInternal<int::TokenRequest> for v1::TokenRequest {
    fn to_internal(self) -> int::TokenRequest {
        int::TokenRequest {
            type_meta: TypeMeta::default(),
            metadata: self.metadata.unwrap_or_default(),
            spec: to_internal_token_request_spec(self.spec),
            status: self.status.map_or_else(
                int::TokenRequestStatus::default,
                to_internal_token_request_status,
            ),
        }
    }
}

impl FromInternal<int::TokenRequest> for v1::TokenRequest {
    fn from_internal(internal: int::TokenRequest) -> Self {
        let mut out = v1::TokenRequest {
            type_meta: TypeMeta::default(),
            metadata: Some(internal.metadata),
            spec: from_internal_token_request_spec(internal.spec),
            status: from_internal_token_request_status(internal.status),
        };
        out.apply_default();
        out
    }
}

impl ToInternal<int::SelfSubjectReview> for v1::SelfSubjectReview {
    fn to_internal(self) -> int::SelfSubjectReview {
        int::SelfSubjectReview {
            type_meta: TypeMeta::default(),
            metadata: self.metadata.unwrap_or_default(),
            status: self.status.map_or_else(
                int::SelfSubjectReviewStatus::default,
                to_internal_self_subject_review_status,
            ),
        }
    }
}

impl FromInternal<int::SelfSubjectReview> for v1::SelfSubjectReview {
    fn from_internal(internal: int::SelfSubjectReview) -> Self {
        let mut out = v1::SelfSubjectReview {
            type_meta: TypeMeta::default(),
            metadata: Some(internal.metadata),
            status: from_internal_self_subject_review_status(internal.status),
        };
        out.apply_default();
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{FromInternal, ToInternal};
    use std::collections::BTreeMap;

    #[test]
    fn test_token_review_roundtrip() {
        let v1_obj = v1::TokenReview {
            type_meta: TypeMeta {
                api_version: "authentication.k8s.io/v1".to_string(),
                kind: "TokenReview".to_string(),
            },
            metadata: Some(crate::common::ObjectMeta::default()),
            spec: v1::TokenReviewSpec {
                token: "my-token".to_string(),
                audiences: vec!["api-server".to_string()],
            },
            status: Some(v1::TokenReviewStatus {
                authenticated: true,
                user: Some(v1::UserInfo {
                    username: "user".to_string(),
                    uid: "uid".to_string(),
                    groups: vec!["group".to_string()],
                    extra: BTreeMap::new(),
                }),
                audiences: vec!["api-server".to_string()],
                error: String::new(),
            }),
        };

        let int_obj = v1_obj.clone().to_internal();
        let v1_obj_back = v1::TokenReview::from_internal(int_obj);

        // Check key fields are preserved
        assert_eq!(v1_obj.spec.token, v1_obj_back.spec.token);
        assert_eq!(
            v1_obj.status.as_ref().unwrap().authenticated,
            v1_obj_back.status.as_ref().unwrap().authenticated
        );
        assert_eq!(
            v1_obj
                .status
                .as_ref()
                .unwrap()
                .user
                .as_ref()
                .unwrap()
                .username,
            v1_obj_back
                .status
                .as_ref()
                .unwrap()
                .user
                .as_ref()
                .unwrap()
                .username
        );
    }

    #[test]
    fn test_token_request_roundtrip() {
        let v1_obj = v1::TokenRequest {
            type_meta: TypeMeta {
                api_version: "authentication.k8s.io/v1".to_string(),
                kind: "TokenRequest".to_string(),
            },
            metadata: Some(crate::common::ObjectMeta::default()),
            spec: v1::TokenRequestSpec {
                audiences: vec!["api-server".to_string()],
                expiration_seconds: Some(3600),
                bound_object_ref: None,
            },
            status: Some(v1::TokenRequestStatus {
                token: "token".to_string(),
                expiration_timestamp: crate::common::time::Timestamp::default(),
            }),
        };

        let int_obj = v1_obj.clone().to_internal();
        let v1_obj_back = v1::TokenRequest::from_internal(int_obj);

        assert_eq!(v1_obj.spec.audiences, v1_obj_back.spec.audiences);
        assert_eq!(
            v1_obj.spec.expiration_seconds,
            v1_obj_back.spec.expiration_seconds
        );
    }

    #[test]
    fn test_token_request_spec_expiration_zero_to_some() {
        let int_spec = int::TokenRequestSpec {
            audiences: vec!["api-server".to_string()],
            expiration_seconds: 0,
            bound_object_ref: None,
        };

        let v1_spec = from_internal_token_request_spec(int_spec);
        assert_eq!(v1_spec.expiration_seconds, Some(0));
    }

    #[test]
    fn test_user_info_roundtrip() {
        let mut extra = BTreeMap::new();
        extra.insert("key".to_string(), vec!["value".to_string()]);

        let v1_info = v1::UserInfo {
            username: "user".to_string(),
            uid: "uid".to_string(),
            groups: vec!["group".to_string()],
            extra: extra.clone(),
        };

        let int_info = to_internal_user_info(v1_info.clone());
        let v1_info_back = from_internal_user_info(int_info);

        assert_eq!(v1_info.username, v1_info_back.username);
        assert_eq!(v1_info.extra, v1_info_back.extra);
    }
}
