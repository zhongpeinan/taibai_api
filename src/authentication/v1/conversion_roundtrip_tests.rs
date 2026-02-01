use super::{
    BoundObjectReference, SelfSubjectReview, SelfSubjectReviewStatus, TokenRequest,
    TokenRequestSpec, TokenRequestStatus, TokenReview, TokenReviewSpec, TokenReviewStatus,
    UserInfo,
};
use crate::authentication::internal;
use crate::common::test_utils::assert_conversion_roundtrip;
use crate::common::time::Timestamp;
use crate::common::{ObjectMeta, TypeMeta};
use std::collections::BTreeMap;

fn token_review_basic() -> TokenReview {
    let mut extra = BTreeMap::new();
    extra.insert("tenant".to_string(), vec!["team-a".to_string()]);

    TokenReview {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("token-review-b".to_string()),
            ..Default::default()
        }),
        spec: TokenReviewSpec {
            token: "token-b".to_string(),
            audiences: vec!["api".to_string()],
        },
        status: Some(TokenReviewStatus {
            authenticated: true,
            user: Some(UserInfo {
                username: "user-b".to_string(),
                uid: "uid-b".to_string(),
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
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("token-request-b".to_string()),
            ..Default::default()
        }),
        spec: TokenRequestSpec {
            audiences: vec!["api".to_string()],
            expiration_seconds: Some(600),
            bound_object_ref: Some(BoundObjectReference {
                kind: "Secret".to_string(),
                api_version: "v1".to_string(),
                name: "secret-a".to_string(),
                uid: "uid-b".to_string(),
            }),
        },
        status: Some(TokenRequestStatus {
            token: "token-b".to_string(),
            expiration_timestamp: Timestamp::from_str("2024-01-02T00:00:00Z").unwrap(),
        }),
    }
}

fn self_subject_review_basic() -> SelfSubjectReview {
    let user_info = UserInfo {
        username: "user-c".to_string(),
        uid: "uid-c".to_string(),
        groups: vec!["system:authenticated".to_string()],
        extra: BTreeMap::new(),
    };

    SelfSubjectReview {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("self-subject-review-b".to_string()),
            ..Default::default()
        }),
        status: Some(SelfSubjectReviewStatus {
            user_info: Some(user_info),
        }),
    }
}

#[test]
fn conversion_roundtrip_token_review() {
    assert_conversion_roundtrip::<TokenReview, internal::TokenReview>(token_review_basic());
}

#[test]
fn conversion_roundtrip_token_request() {
    assert_conversion_roundtrip::<TokenRequest, internal::TokenRequest>(token_request_basic());
}

#[test]
fn conversion_roundtrip_self_subject_review() {
    assert_conversion_roundtrip::<SelfSubjectReview, internal::SelfSubjectReview>(
        self_subject_review_basic(),
    );
}
