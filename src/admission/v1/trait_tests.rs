use super::*;
use crate::admission::internal;
use crate::common::{ApplyDefault, FromInternal, ResourceSchema, ToInternal};

#[test]
fn top_level_resources_implement_required_traits() {
    fn check_default<T: Default>() {}
    fn check_apply<T: ApplyDefault>() {}
    fn check_schema<T: ResourceSchema>() {}

    check_apply::<AdmissionReview>();
    check_default::<AdmissionReview>();
    check_schema::<AdmissionReview>();
}

#[test]
fn top_level_resources_have_conversion_traits() {
    fn check_conversion<T, I>()
    where
        T: ToInternal<I> + FromInternal<I>,
    {
    }

    check_conversion::<AdmissionReview, internal::AdmissionReview>();
    check_conversion::<AdmissionRequest, internal::AdmissionRequest>();
    check_conversion::<AdmissionResponse, internal::AdmissionResponse>();
}

#[test]
fn top_level_resources_implement_prost_message() {
    fn check_prost<T: prost::Message>() {}

    check_prost::<AdmissionReview>();
    check_prost::<AdmissionRequest>();
    check_prost::<AdmissionResponse>();
}

#[test]
fn apply_default_sets_type_meta() {
    let mut review = AdmissionReview::default();
    review.apply_default();
    assert_eq!(review.type_meta.api_version, "admission.k8s.io/v1");
    assert_eq!(review.type_meta.kind, "AdmissionReview");
}
