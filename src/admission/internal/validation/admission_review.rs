use crate::admission::internal::AdmissionReview;
use crate::common::validation::{ErrorList, Path, required};

use super::admission_request::validate_admission_request_with_path;
use super::admission_response::validate_admission_response_with_path;

pub fn validate_admission_review(review: &AdmissionReview) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if review.request.is_none() && review.response.is_none() {
        all_errs.push(required(
            &Path::new("request"),
            "request or response is required",
        ));
        return all_errs;
    }

    if let Some(ref request) = review.request {
        all_errs.extend(validate_admission_request_with_path(
            request,
            &Path::new("request"),
        ));
    }

    if let Some(ref response) = review.response {
        all_errs.extend(validate_admission_response_with_path(
            response,
            &Path::new("response"),
        ));
    }

    all_errs
}
