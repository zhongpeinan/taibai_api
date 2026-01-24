//! Conversions between versioned and internal Authentication API types
//!
//! Source: k8s/pkg/apis/authentication/v1/zz_generated.conversion.go

use crate::authentication::internal as int;
use crate::authentication::v1;

// Note: UserInfo From implementations are defined in admission/internal/conversion.rs
// to avoid conflicts, as UserInfo is shared between authentication and admission modules.

impl From<v1::BoundObjectReference> for int::BoundObjectReference {
    fn from(v: v1::BoundObjectReference) -> Self {
        Self {
            kind: v.kind,
            api_version: v.api_version,
            name: v.name,
            uid: v.uid,
        }
    }
}

impl From<int::BoundObjectReference> for v1::BoundObjectReference {
    fn from(v: int::BoundObjectReference) -> Self {
        Self {
            kind: v.kind,
            api_version: v.api_version,
            name: v.name,
            uid: v.uid,
        }
    }
}

impl From<v1::TokenReviewSpec> for int::TokenReviewSpec {
    fn from(v: v1::TokenReviewSpec) -> Self {
        Self {
            token: v.token,
            audiences: v.audiences,
        }
    }
}

impl From<int::TokenReviewSpec> for v1::TokenReviewSpec {
    fn from(v: int::TokenReviewSpec) -> Self {
        Self {
            token: v.token,
            audiences: v.audiences,
        }
    }
}

impl From<v1::TokenReviewStatus> for int::TokenReviewStatus {
    fn from(v: v1::TokenReviewStatus) -> Self {
        Self {
            authenticated: v.authenticated,
            user: v.user.map_or_else(Default::default, |u| u.into()),
            audiences: v.audiences,
            error: v.error,
        }
    }
}

impl From<int::TokenReviewStatus> for v1::TokenReviewStatus {
    fn from(v: int::TokenReviewStatus) -> Self {
        Self {
            authenticated: v.authenticated,
            user: Some(v.user.into()),
            audiences: v.audiences,
            error: v.error,
        }
    }
}

impl From<v1::TokenReview> for int::TokenReview {
    fn from(v: v1::TokenReview) -> Self {
        Self {
            type_meta: v.type_meta,
            metadata: v.metadata.unwrap_or_default(),
            spec: v.spec.into(),
            status: v.status.map_or_else(Default::default, |s| s.into()),
        }
    }
}

impl From<int::TokenReview> for v1::TokenReview {
    fn from(v: int::TokenReview) -> Self {
        Self {
            type_meta: v.type_meta,
            metadata: Some(v.metadata),
            spec: v.spec.into(),
            status: Some(v.status.into()),
        }
    }
}

impl From<v1::TokenRequestSpec> for int::TokenRequestSpec {
    fn from(v: v1::TokenRequestSpec) -> Self {
        Self {
            audiences: v.audiences,
            expiration_seconds: v.expiration_seconds.unwrap_or(0),
            bound_object_ref: v.bound_object_ref.map(Into::into),
        }
    }
}

impl From<int::TokenRequestSpec> for v1::TokenRequestSpec {
    fn from(v: int::TokenRequestSpec) -> Self {
        Self {
            audiences: v.audiences,
            expiration_seconds: if v.expiration_seconds == 0 {
                None
            } else {
                Some(v.expiration_seconds)
            },
            bound_object_ref: v.bound_object_ref.map(Into::into),
        }
    }
}

impl From<v1::TokenRequestStatus> for int::TokenRequestStatus {
    fn from(v: v1::TokenRequestStatus) -> Self {
        Self {
            token: v.token,
            expiration_timestamp: v.expiration_timestamp,
        }
    }
}

impl From<int::TokenRequestStatus> for v1::TokenRequestStatus {
    fn from(v: int::TokenRequestStatus) -> Self {
        Self {
            token: v.token,
            expiration_timestamp: v.expiration_timestamp,
        }
    }
}

impl From<v1::TokenRequest> for int::TokenRequest {
    fn from(v: v1::TokenRequest) -> Self {
        Self {
            type_meta: v.type_meta,
            metadata: v.metadata.unwrap_or_default(),
            spec: v.spec.into(),
            status: v.status.map_or_else(Default::default, |s| s.into()),
        }
    }
}

impl From<int::TokenRequest> for v1::TokenRequest {
    fn from(v: int::TokenRequest) -> Self {
        Self {
            type_meta: v.type_meta,
            metadata: Some(v.metadata),
            spec: v.spec.into(),
            status: Some(v.status.into()),
        }
    }
}

impl From<v1::SelfSubjectReviewStatus> for int::SelfSubjectReviewStatus {
    fn from(v: v1::SelfSubjectReviewStatus) -> Self {
        Self {
            user_info: v.user_info.map_or_else(Default::default, |u| u.into()),
        }
    }
}

impl From<int::SelfSubjectReviewStatus> for v1::SelfSubjectReviewStatus {
    fn from(v: int::SelfSubjectReviewStatus) -> Self {
        Self {
            user_info: Some(v.user_info.into()),
        }
    }
}

impl From<v1::SelfSubjectReview> for int::SelfSubjectReview {
    fn from(v: v1::SelfSubjectReview) -> Self {
        Self {
            type_meta: v.type_meta,
            metadata: v.metadata.unwrap_or_default(),
            status: v.status.map_or_else(Default::default, |s| s.into()),
        }
    }
}

impl From<int::SelfSubjectReview> for v1::SelfSubjectReview {
    fn from(v: int::SelfSubjectReview) -> Self {
        Self {
            type_meta: v.type_meta,
            metadata: Some(v.metadata),
            status: Some(v.status.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::TypeMeta;
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

        let int_obj: int::TokenReview = v1_obj.clone().into();
        let v1_obj_back: v1::TokenReview = int_obj.into();

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

        let int_obj: int::TokenRequest = v1_obj.clone().into();
        let v1_obj_back: v1::TokenRequest = int_obj.into();

        assert_eq!(v1_obj.spec.audiences, v1_obj_back.spec.audiences);
        assert_eq!(
            v1_obj.spec.expiration_seconds,
            v1_obj_back.spec.expiration_seconds
        );
    }

    #[test]
    fn test_token_request_spec_expiration_zero_to_none() {
        // Test that expiration_seconds=0 converts to None in v1
        let int_spec = int::TokenRequestSpec {
            audiences: vec!["api-server".to_string()],
            expiration_seconds: 0,
            bound_object_ref: None,
        };

        let v1_spec: v1::TokenRequestSpec = int_spec.into();
        assert_eq!(v1_spec.expiration_seconds, None);
    }

    #[test]
    fn test_user_info_roundtrip() {
        // UserInfo conversions are defined in admission/internal/conversion.rs
        // to avoid conflicts, as UserInfo is shared between modules.
        let mut extra = BTreeMap::new();
        extra.insert("key".to_string(), vec!["value".to_string()]);

        let v1_info = v1::UserInfo {
            username: "user".to_string(),
            uid: "uid".to_string(),
            groups: vec!["group".to_string()],
            extra: extra.clone(),
        };

        // Test that UserInfo can be converted (via admission module's From impl)
        let int_info: int::UserInfo = v1_info.clone().into();
        assert_eq!(v1_info.username, int_info.username);
        assert_eq!(v1_info.extra, int_info.extra);
    }
}
