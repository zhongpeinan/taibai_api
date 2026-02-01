use crate::common::validation::ErrorList;
use crate::imagepolicy::internal::validation as internal_validation;
use crate::imagepolicy::v1alpha1::{ImageReview, ImageReviewList};

pub fn validate_image_review(obj: &ImageReview) -> ErrorList {
    internal_validation::validate_image_review(obj)
}

pub fn validate_image_review_list(obj: &ImageReviewList) -> ErrorList {
    internal_validation::validate_image_review_list(obj)
}
