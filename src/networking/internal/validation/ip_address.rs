//! IPAddress validation.

use crate::common::validation::*;
use crate::networking::v1::ip_address::{IPAddress, IPAddressList, ParentReference};

use super::helpers::*;

// IPAddress Validation
// ============================================================================

/// Validates an IPAddress object.
pub fn validate_ip_address(ip_address: &IPAddress) -> ErrorList {
    validate_ip_address_with_path(ip_address, &Path::nil())
}

fn validate_ip_address_with_path(ip_address: &IPAddress, base_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let default_meta = crate::common::ObjectMeta::default();
    let meta = ip_address.metadata.as_ref().unwrap_or(&default_meta);

    all_errs.extend(validate_object_meta(
        meta,
        false,
        validate_ip_address_name,
        &base_path.child("metadata"),
    ));

    all_errs.extend(validate_ip_address_parent_reference(
        &ip_address.spec.parent_ref,
        &base_path.child("spec"),
    ));

    all_errs
}

fn validate_ip_address_parent_reference(
    parent_ref: &Option<ParentReference>,
    base_path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let parent_ref = match parent_ref.as_ref() {
        Some(value) => value,
        None => {
            all_errs.push(required(&base_path.child("parentRef"), ""));
            return all_errs;
        }
    };

    let path = base_path.child("parentRef");
    if !parent_ref.group.is_empty() {
        all_errs.extend(validate_dns1123_subdomain(
            &parent_ref.group,
            &path.child("group"),
        ));
    }

    if parent_ref.resource.is_empty() {
        all_errs.push(required(&path.child("resource"), ""));
    } else {
        all_errs.extend(validate_path_segment_name(
            &parent_ref.resource,
            &path.child("resource"),
        ));
    }

    if parent_ref.name.is_empty() {
        all_errs.push(required(&path.child("name"), ""));
    } else {
        all_errs.extend(validate_path_segment_name(
            &parent_ref.name,
            &path.child("name"),
        ));
    }

    if !parent_ref.namespace.is_empty() {
        all_errs.extend(validate_path_segment_name(
            &parent_ref.namespace,
            &path.child("namespace"),
        ));
    }

    all_errs
}

/// Validates an IPAddressList object.
pub fn validate_ip_address_list(list: &IPAddressList) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (i, item) in list.items.iter().enumerate() {
        let item_path = Path::new("items").index(i);
        all_errs.extend(validate_ip_address_with_path(item, &item_path));
    }

    all_errs
}

/// Validates an IPAddress update.
pub fn validate_ip_address_update(new_obj: &IPAddress, old_obj: &IPAddress) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let default_meta = crate::common::ObjectMeta::default();
    let new_meta = new_obj.metadata.as_ref().unwrap_or(&default_meta);
    let old_meta = old_obj.metadata.as_ref().unwrap_or(&default_meta);

    all_errs.extend(validate_object_meta_update(
        new_meta,
        old_meta,
        &Path::new("metadata"),
    ));

    if new_obj.spec.parent_ref != old_obj.spec.parent_ref {
        all_errs.push(invalid(
            &Path::new("spec").child("parentRef"),
            BadValue::String(format!("{:?}", new_obj.spec.parent_ref)),
            "field is immutable",
        ));
    }

    all_errs
}

// ============================================================================
