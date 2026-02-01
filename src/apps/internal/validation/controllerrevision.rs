//! ControllerRevision validation for Kubernetes apps internal API

use crate::apps::internal::ControllerRevision;
use crate::common::validation::{
    ErrorList, Path, forbidden, name_is_dns_subdomain, required, validate_object_meta,
    validate_object_meta_update,
};
use crate::core::v1::validation::helpers::validate_nonnegative_field;

// =============================================================================
// ControllerRevision validation
// =============================================================================

pub fn validate_controller_revision_create(revision: &ControllerRevision) -> ErrorList {
    let mut all_errs = ErrorList::new();

    all_errs.extend(validate_object_meta(
        &revision.metadata,
        true,
        name_is_dns_subdomain,
        &Path::new("metadata"),
    ));
    all_errs.extend(validate_nonnegative_field(
        revision.revision,
        &Path::new("revision"),
    ));

    match revision.data {
        serde_json::Value::Null => {
            all_errs.push(required(&Path::new("data"), "data is mandatory"));
        }
        serde_json::Value::Object(_) => {}
        _ => {
            all_errs.push(required(
                &Path::new("data"),
                "data must be a valid JSON object",
            ));
        }
    }

    all_errs
}

pub fn validate_controller_revision_update(
    new_revision: &ControllerRevision,
    old_revision: &ControllerRevision,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    all_errs.extend(validate_object_meta_update(
        &new_revision.metadata,
        &old_revision.metadata,
        &Path::new("metadata"),
    ));

    all_errs.extend(validate_controller_revision_create(new_revision));
    if new_revision.data != old_revision.data {
        all_errs.push(forbidden(&Path::new("data"), "field is immutable"));
    }

    all_errs
}
