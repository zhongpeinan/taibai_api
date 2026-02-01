use crate::admission::internal::{AdmissionRequest, operation};
use crate::common::validation::{BadValue, ErrorList, Path, not_supported, required};

const SUPPORTED_OPERATIONS: [&str; 4] = [
    operation::CREATE,
    operation::UPDATE,
    operation::DELETE,
    operation::CONNECT,
];

pub fn validate_admission_request(request: &AdmissionRequest) -> ErrorList {
    validate_admission_request_with_path(request, &Path::nil())
}

pub(crate) fn validate_admission_request_with_path(
    request: &AdmissionRequest,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if request.uid.is_empty() {
        all_errs.push(required(&path.child("uid"), "uid is required"));
    }

    if request.kind.kind.is_empty() {
        all_errs.push(required(
            &path.child("kind").child("kind"),
            "kind is required",
        ));
    }
    if request.kind.version.is_empty() {
        all_errs.push(required(
            &path.child("kind").child("version"),
            "version is required",
        ));
    }

    if request.resource.resource.is_empty() {
        all_errs.push(required(
            &path.child("resource").child("resource"),
            "resource is required",
        ));
    }
    if request.resource.version.is_empty() {
        all_errs.push(required(
            &path.child("resource").child("version"),
            "version is required",
        ));
    }

    if request.operation.is_empty() {
        all_errs.push(required(&path.child("operation"), "operation is required"));
    } else if !SUPPORTED_OPERATIONS.contains(&request.operation.as_str()) {
        all_errs.push(not_supported(
            &path.child("operation"),
            BadValue::String(request.operation.clone()),
            &SUPPORTED_OPERATIONS,
        ));
    }

    if let Some(ref request_kind) = request.request_kind {
        if request_kind.kind.is_empty() {
            all_errs.push(required(
                &path.child("requestKind").child("kind"),
                "kind is required",
            ));
        }
        if request_kind.version.is_empty() {
            all_errs.push(required(
                &path.child("requestKind").child("version"),
                "version is required",
            ));
        }
    }

    if let Some(ref request_resource) = request.request_resource {
        if request_resource.resource.is_empty() {
            all_errs.push(required(
                &path.child("requestResource").child("resource"),
                "resource is required",
            ));
        }
        if request_resource.version.is_empty() {
            all_errs.push(required(
                &path.child("requestResource").child("version"),
                "version is required",
            ));
        }
    }

    all_errs
}
