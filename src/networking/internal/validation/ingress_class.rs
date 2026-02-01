//! IngressClass validation.

use crate::common::validation::*;
use crate::networking::v1::ingress_class::{IngressClass, IngressClassList};

use super::helpers::*;

// IngressClass Validation
// ============================================================================

/// Validates an IngressClass object.
pub fn validate_ingress_class(ingress_class: &IngressClass) -> ErrorList {
    validate_ingress_class_with_path(ingress_class, &Path::nil())
}

fn validate_ingress_class_with_path(ingress_class: &IngressClass, base_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let default_meta = crate::common::ObjectMeta::default();
    let meta = ingress_class.metadata.as_ref().unwrap_or(&default_meta);

    all_errs.extend(validate_object_meta(
        meta,
        false, // IngressClass is not namespaced
        name_is_dns_subdomain,
        &base_path.child("metadata"),
    ));

    // Validate spec - controller is required
    if ingress_class.spec.controller.is_empty() {
        all_errs.push(required(
            &base_path.child("spec").child("controller"),
            "controller is required",
        ));
    } else if ingress_class.spec.controller.len() > MAX_INGRESS_CLASS_CONTROLLER_LEN {
        all_errs.push(too_long(
            &base_path.child("spec").child("controller"),
            MAX_INGRESS_CLASS_CONTROLLER_LEN,
        ));
    }

    if let Some(ref params) = ingress_class.spec.parameters {
        all_errs.extend(validate_ingress_class_parameters(
            params,
            &base_path.child("spec").child("parameters"),
        ));
    }

    all_errs
}

/// Validates an IngressClassList object.
pub fn validate_ingress_class_list(list: &IngressClassList) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (i, item) in list.items.iter().enumerate() {
        let item_path = Path::new("items").index(i);
        all_errs.extend(validate_ingress_class_with_path(item, &item_path));
    }

    all_errs
}

/// Validates an IngressClass update.
pub fn validate_ingress_class_update(
    new_ingress_class: &IngressClass,
    old_ingress_class: &IngressClass,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let default_meta = crate::common::ObjectMeta::default();
    let new_meta = new_ingress_class.metadata.as_ref().unwrap_or(&default_meta);
    let old_meta = old_ingress_class.metadata.as_ref().unwrap_or(&default_meta);

    all_errs.extend(validate_object_meta_update(
        new_meta,
        old_meta,
        &Path::new("metadata"),
    ));

    if new_ingress_class.spec.controller != old_ingress_class.spec.controller {
        all_errs.push(invalid(
            &Path::new("spec").child("controller"),
            BadValue::String(new_ingress_class.spec.controller.clone()),
            "field is immutable",
        ));
    }

    all_errs.extend(validate_ingress_class_with_path(
        new_ingress_class,
        &Path::nil(),
    ));

    all_errs
}

// ============================================================================
