use crate::authorization::internal::LocalSubjectAccessReview;
use crate::common::validation::{BadValue, ErrorList, Path, invalid};

use super::{validate_metadata_empty, validate_subject_access_review_spec};

pub fn validate_local_subject_access_review(obj: &LocalSubjectAccessReview) -> ErrorList {
    let mut all_errs = validate_subject_access_review_spec(&obj.spec, &Path::new("spec"));
    all_errs.extend(validate_metadata_empty(
        &obj.metadata,
        true,
        &Path::new("metadata"),
    ));

    if let Some(resource_attributes) = obj.spec.resource_attributes.as_ref() {
        let namespace = obj
            .metadata
            .namespace
            .clone()
            .unwrap_or_default();
        if resource_attributes.namespace != namespace {
            all_errs.push(invalid(
                &Path::new("spec")
                    .child("resourceAttributes")
                    .child("namespace"),
                BadValue::String(resource_attributes.namespace.clone()),
                "must match metadata.namespace",
            ));
        }
    }

    if let Some(non_resource_attributes) = obj.spec.non_resource_attributes.as_ref() {
        all_errs.push(invalid(
            &Path::new("spec").child("nonResourceAttributes"),
            BadValue::String(format!("{:?}", non_resource_attributes)),
            "disallowed on this kind of request",
        ));
    }

    all_errs
}
