use super::{
    BadValue, ErrorList, Path, forbidden, invalid, is_dns1123_label, is_dns1123_subdomain, required,
};
use crate::common::ObjectMeta;

const IS_NEGATIVE_ERROR_MSG: &str = "must be greater than or equal to 0";

pub type ValidateNameFunc = fn(name: &str, prefix: bool) -> Vec<String>;

pub fn name_is_dns_subdomain(name: &str, prefix: bool) -> Vec<String> {
    let value = if prefix {
        mask_trailing_dash(name)
    } else {
        name.to_string()
    };
    is_dns1123_subdomain(&value)
}

pub fn name_is_dns_label(name: &str, prefix: bool) -> Vec<String> {
    let value = if prefix {
        mask_trailing_dash(name)
    } else {
        name.to_string()
    };
    is_dns1123_label(&value)
}

pub fn validate_object_meta(
    meta: &ObjectMeta,
    requires_namespace: bool,
    name_fn: ValidateNameFunc,
    fld_path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if let Some(generate_name) = meta.generate_name.as_deref() {
        if !generate_name.is_empty() {
            for msg in name_fn(generate_name, true) {
                all_errs.push(invalid(
                    &fld_path.child("generateName"),
                    BadValue::String(generate_name.to_string()),
                    &msg,
                ));
            }
        }
    }

    let name = meta.name.as_deref().unwrap_or("");
    if name.is_empty() {
        all_errs.push(required(
            &fld_path.child("name"),
            "name or generateName is required",
        ));
    } else {
        for msg in name_fn(name, false) {
            all_errs.push(invalid(
                &fld_path.child("name"),
                BadValue::String(name.to_string()),
                &msg,
            ));
        }
    }

    if requires_namespace {
        let namespace = meta.namespace.as_deref().unwrap_or("");
        if namespace.is_empty() {
            all_errs.push(required(&fld_path.child("namespace"), ""));
        } else {
            for msg in name_is_dns_label(namespace, false) {
                all_errs.push(invalid(
                    &fld_path.child("namespace"),
                    BadValue::String(namespace.to_string()),
                    &msg,
                ));
            }
        }
    } else if let Some(namespace) = meta.namespace.as_deref() {
        if !namespace.is_empty() {
            all_errs.push(forbidden(
                &fld_path.child("namespace"),
                "not allowed on this type",
            ));
        }
    }

    if let Some(generation) = meta.generation {
        if generation < 0 {
            all_errs.push(invalid(
                &fld_path.child("generation"),
                BadValue::Int(generation),
                IS_NEGATIVE_ERROR_MSG,
            ));
        }
    }

    all_errs
}

fn mask_trailing_dash(name: &str) -> String {
    if name.len() > 1 && name.ends_with('-') {
        let cut = name.len() - 2;
        format!("{}a", &name[..cut])
    } else {
        name.to_string()
    }
}
