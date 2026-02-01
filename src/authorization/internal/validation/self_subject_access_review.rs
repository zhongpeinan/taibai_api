use crate::authorization::internal::SelfSubjectAccessReview;
use crate::common::validation::{ErrorList, Path};

use super::{validate_metadata_empty, validate_self_subject_access_review_spec};

pub fn validate_self_subject_access_review(obj: &SelfSubjectAccessReview) -> ErrorList {
    let mut all_errs = validate_self_subject_access_review_spec(&obj.spec, &Path::new("spec"));
    all_errs.extend(validate_metadata_empty(
        &obj.metadata,
        false,
        &Path::new("metadata"),
    ));
    all_errs
}
