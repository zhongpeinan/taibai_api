use super::{
    BoundObjectReference, SelfSubjectReview, SelfSubjectReviewStatus, TokenRequest,
    TokenRequestSpec, TokenRequestStatus, TokenReview, TokenReviewSpec, TokenReviewStatus,
    UserInfo,
};
use crate::common::test_utils::assert_serde_roundtrip;
use crate::common::time::Timestamp;
use crate::common::{ObjectMeta, TypeMeta};
use std::collections::BTreeMap;

fn token_review_basic() -> TokenReview {
    let mut extra = BTreeMap::new();
    extra.insert(
        "scopes".to_string(),
        vec!["read".to_string(), "write".to_string()],
    );

    TokenReview {
        type_meta: TypeMeta {
            api_version: "authentication.k8s.io/v1".to_string(),
            kind: "TokenReview".to_string(),
        },
        metadata: Some(ObjectMeta {
            name: Some("token-review-a".to_string()),
            ..Default::default()
        }),
        spec: TokenReviewSpec {
            token: "token-a".to_string(),
            audiences: vec!["api".to_string()],
        },
        status: Some(TokenReviewStatus {
            authenticated: true,
            user: Some(UserInfo {
                username: "user-a".to_string(),
                uid: "uid-a".to_string(),
                groups: vec!["system:authenticated".to_string()],
                extra,
            }),
            audiences: vec!["api".to_string()],
            error: String::new(),
        }),
    }
}

fn token_request_basic() -> TokenRequest {
    TokenRequest {
        type_meta: TypeMeta {
            api_version: "authentication.k8s.io/v1".to_string(),
            kind: "TokenRequest".to_string(),
        },
        metadata: Some(ObjectMeta {
            name: Some("token-request-a".to_string()),
            ..Default::default()
        }),
        spec: TokenRequestSpec {
            audiences: vec!["api".to_string()],
            expiration_seconds: Some(600),
            bound_object_ref: Some(BoundObjectReference {
                kind: "Pod".to_string(),
                api_version: "v1".to_string(),
                name: "pod-a".to_string(),
                uid: "uid-a".to_string(),
            }),
        },
        status: Some(TokenRequestStatus {
            token: "token-a".to_string(),
            expiration_timestamp: Timestamp::from_str("2024-01-01T00:00:00Z").unwrap(),
        }),
    }
}

fn self_subject_review_basic() -> SelfSubjectReview {
    let user_info = UserInfo {
        username: "user-b".to_string(),
        uid: "uid-b".to_string(),
        groups: vec!["system:authenticated".to_string()],
        extra: BTreeMap::new(),
    };

    SelfSubjectReview {
        type_meta: TypeMeta {
            api_version: "authentication.k8s.io/v1".to_string(),
            kind: "SelfSubjectReview".to_string(),
        },
        metadata: Some(ObjectMeta {
            name: Some("self-subject-review-a".to_string()),
            ..Default::default()
        }),
        status: Some(SelfSubjectReviewStatus {
            user_info: Some(user_info),
        }),
    }
}

#[test]
fn serde_roundtrip_token_review() {
    assert_serde_roundtrip(&token_review_basic());
}

#[test]
fn serde_roundtrip_token_request() {
    assert_serde_roundtrip(&token_request_basic());
}

#[test]
fn serde_roundtrip_self_subject_review() {
    assert_serde_roundtrip(&self_subject_review_basic());
}
