use crate::admission::internal::AdmissionResponse;
use crate::common::validation::{ErrorList, Path, required};

pub fn validate_admission_response(response: &AdmissionResponse) -> ErrorList {
    validate_admission_response_with_path(response, &Path::nil())
}

pub(crate) fn validate_admission_response_with_path(
    response: &AdmissionResponse,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if response.uid.is_empty() {
        all_errs.push(required(&path.child("uid"), "uid is required"));
    }

    if response.patch.is_some() && response.patch_type.is_none() {
        all_errs.push(required(
            &path.child("patchType"),
            "patchType is required when patch is set",
        ));
    }

    if response.patch.is_none() && response.patch_type.is_some() {
        all_errs.push(required(
            &path.child("patch"),
            "patch is required when patchType is set",
        ));
    }

    all_errs
}
