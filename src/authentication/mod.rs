//! Kubernetes Authentication API types
//!
//! This module contains the authentication API types.

pub mod internal;
pub mod v1;

pub use internal::{BoundObjectReference, SelfSubjectReview, TokenRequest, TokenReview};
pub use v1::{
    BoundObjectReference as BoundObjectReferenceV1, SelfSubjectReview as SelfSubjectReviewV1,
    TokenRequest as TokenRequestV1, TokenReview as TokenReviewV1, UserInfo,
};
