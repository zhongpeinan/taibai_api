use super::{
    BadValue, ErrorList, Path, forbidden, invalid, is_dns1123_label, is_dns1123_subdomain, required,
};
use crate::common::ObjectMeta;
use std::collections::BTreeSet;

const IS_NEGATIVE_ERROR_MSG: &str = "must be greater than or equal to 0";
const FIELD_IMMUTABLE_ERROR_MSG: &str = "field is immutable";
const RESOURCE_VERSION_REQUIRED_ERROR_MSG: &str = "must be specified for an update";
const GENERATION_DECREMENT_ERROR_MSG: &str = "must not be decremented";

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

    if let Some(generate_name) = meta.generate_name.as_deref()
        && !generate_name.is_empty()
    {
        for msg in name_fn(generate_name, true) {
            all_errs.push(invalid(
                &fld_path.child("generateName"),
                BadValue::String(generate_name.to_string()),
                &msg,
            ));
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
    } else if let Some(namespace) = meta.namespace.as_deref()
        && !namespace.is_empty()
    {
        all_errs.push(forbidden(
            &fld_path.child("namespace"),
            "not allowed on this type",
        ));
    }

    if let Some(generation) = meta.generation
        && generation < 0
    {
        all_errs.push(invalid(
            &fld_path.child("generation"),
            BadValue::Int(generation),
            IS_NEGATIVE_ERROR_MSG,
        ));
    }

    all_errs
}

pub fn validate_object_meta_update(
    new_meta: &ObjectMeta,
    old_meta: &ObjectMeta,
    fld_path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if old_meta.deletion_timestamp.is_some() {
        let old_finalizers: BTreeSet<_> = old_meta.finalizers.iter().collect();
        let new_finalizers: BTreeSet<_> = new_meta.finalizers.iter().collect();
        let extra: Vec<_> = new_finalizers
            .difference(&old_finalizers)
            .map(|item| (*item).clone())
            .collect();
        if !extra.is_empty() {
            let detail = format!(
                "no new finalizers can be added if the object is being deleted, found new finalizers {:?}",
                extra
            );
            all_errs.push(forbidden(&fld_path.child("finalizers"), &detail));
        }
    }

    if new_meta
        .resource_version
        .as_deref()
        .unwrap_or("")
        .is_empty()
    {
        all_errs.push(invalid(
            &fld_path.child("resourceVersion"),
            BadValue::String(new_meta.resource_version.clone().unwrap_or_default()),
            RESOURCE_VERSION_REQUIRED_ERROR_MSG,
        ));
    }

    let new_generation = new_meta.generation.unwrap_or(0);
    let old_generation = old_meta.generation.unwrap_or(0);
    if new_generation < old_generation {
        all_errs.push(invalid(
            &fld_path.child("generation"),
            BadValue::Int(new_generation),
            GENERATION_DECREMENT_ERROR_MSG,
        ));
    }

    let new_name = new_meta.name.as_deref().unwrap_or("");
    let old_name = old_meta.name.as_deref().unwrap_or("");
    if new_name != old_name {
        all_errs.push(invalid(
            &fld_path.child("name"),
            BadValue::String(new_name.to_string()),
            FIELD_IMMUTABLE_ERROR_MSG,
        ));
    }

    let new_namespace = new_meta.namespace.as_deref().unwrap_or("");
    let old_namespace = old_meta.namespace.as_deref().unwrap_or("");
    if new_namespace != old_namespace {
        all_errs.push(invalid(
            &fld_path.child("namespace"),
            BadValue::String(new_namespace.to_string()),
            FIELD_IMMUTABLE_ERROR_MSG,
        ));
    }

    let new_uid = new_meta.uid.as_deref().unwrap_or("");
    let old_uid = old_meta.uid.as_deref().unwrap_or("");
    if new_uid != old_uid {
        all_errs.push(invalid(
            &fld_path.child("uid"),
            BadValue::String(new_uid.to_string()),
            FIELD_IMMUTABLE_ERROR_MSG,
        ));
    }

    if new_meta.creation_timestamp != old_meta.creation_timestamp {
        all_errs.push(invalid(
            &fld_path.child("creationTimestamp"),
            BadValue::String(format!("{:?}", new_meta.creation_timestamp)),
            FIELD_IMMUTABLE_ERROR_MSG,
        ));
    }

    if new_meta.deletion_timestamp != old_meta.deletion_timestamp {
        all_errs.push(invalid(
            &fld_path.child("deletionTimestamp"),
            BadValue::String(format!("{:?}", new_meta.deletion_timestamp)),
            FIELD_IMMUTABLE_ERROR_MSG,
        ));
    }

    if new_meta.deletion_grace_period_seconds != old_meta.deletion_grace_period_seconds {
        let value = new_meta.deletion_grace_period_seconds.unwrap_or_default();
        all_errs.push(invalid(
            &fld_path.child("deletionGracePeriodSeconds"),
            BadValue::Int(value),
            FIELD_IMMUTABLE_ERROR_MSG,
        ));
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
