//! Validation for Kubernetes Authorization v1 API types

mod local_subject_access_review;
mod self_subject_access_review;
mod subject_access_review;

pub use local_subject_access_review::validate_local_subject_access_review;
pub use self_subject_access_review::validate_self_subject_access_review;
pub use subject_access_review::validate_subject_access_review;
