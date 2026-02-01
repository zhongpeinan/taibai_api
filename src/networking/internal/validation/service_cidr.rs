//! ServiceCIDR validation.

use crate::common::validation::*;
use crate::networking::v1::service_cidr::{ServiceCIDR, ServiceCIDRList, ServiceCIDRSpec};
use std::net::IpAddr;

use super::helpers::*;

// ServiceCIDR Validation
// ============================================================================

/// Validates a ServiceCIDR object.
pub fn validate_service_cidr(service_cidr: &ServiceCIDR) -> ErrorList {
    validate_service_cidr_with_path(service_cidr, &Path::nil())
}

fn validate_service_cidr_with_path(service_cidr: &ServiceCIDR, base_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let default_meta = crate::common::ObjectMeta::default();
    let meta = service_cidr.metadata.as_ref().unwrap_or(&default_meta);

    all_errs.extend(validate_object_meta(
        meta,
        false,
        name_is_dns_subdomain,
        &base_path.child("metadata"),
    ));

    all_errs.extend(validate_service_cidr_spec(
        &service_cidr.spec,
        &base_path.child("spec").child("cidrs"),
    ));

    all_errs
}

fn validate_service_cidr_spec(spec: &ServiceCIDRSpec, field_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if spec.cidrs.is_empty() {
        all_errs.push(required(field_path, "at least one CIDR required"));
        return all_errs;
    }

    if spec.cidrs.len() > 2 {
        all_errs.push(invalid(
            field_path,
            BadValue::String(format!("{:?}", spec.cidrs)),
            "may only hold up to 2 values",
        ));
        return all_errs;
    }

    let mut families: Vec<IpAddr> = Vec::new();
    for (i, cidr) in spec.cidrs.iter().enumerate() {
        let idx_path = field_path.index(i);
        let (errs, parsed) = validate_cidr_strict(cidr, &idx_path);
        all_errs.extend(errs);
        if let Some((ip, _)) = parsed {
            families.push(ip);
        }
    }

    if spec.cidrs.len() == 2 && families.len() == 2 {
        let same_family = matches!(
            (&families[0], &families[1]),
            (IpAddr::V4(_), IpAddr::V4(_)) | (IpAddr::V6(_), IpAddr::V6(_))
        );
        if same_family {
            all_errs.push(invalid(
                field_path,
                BadValue::String(format!("{:?}", spec.cidrs)),
                "may specify no more than one IP for each IP family, i.e 192.168.0.0/24 and 2001:db8::/64",
            ));
        }
    }

    all_errs
}

/// Validates a ServiceCIDRList object.
pub fn validate_service_cidr_list(list: &ServiceCIDRList) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (i, item) in list.items.iter().enumerate() {
        let item_path = Path::new("items").index(i);
        all_errs.extend(validate_service_cidr_with_path(item, &item_path));
    }

    all_errs
}

/// Validates a ServiceCIDR update.
pub fn validate_service_cidr_update(new_obj: &ServiceCIDR, old_obj: &ServiceCIDR) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let default_meta = crate::common::ObjectMeta::default();
    let new_meta = new_obj.metadata.as_ref().unwrap_or(&default_meta);
    let old_meta = old_obj.metadata.as_ref().unwrap_or(&default_meta);

    all_errs.extend(validate_object_meta_update(
        new_meta,
        old_meta,
        &Path::new("metadata"),
    ));

    match (old_obj.spec.cidrs.len(), new_obj.spec.cidrs.len()) {
        (len_old, len_new) if len_old == len_new => {
            for (i, old_cidr) in old_obj.spec.cidrs.iter().enumerate() {
                if Some(old_cidr) != new_obj.spec.cidrs.get(i) {
                    all_errs.push(invalid(
                        &Path::new("spec").child("cidrs").index(i),
                        BadValue::String(new_obj.spec.cidrs.get(i).cloned().unwrap_or_default()),
                        "field is immutable",
                    ));
                }
            }
        }
        (1, 2) => {
            if new_obj.spec.cidrs.get(0) != old_obj.spec.cidrs.get(0) {
                all_errs.push(invalid(
                    &Path::new("spec").child("cidrs").index(0),
                    BadValue::String(new_obj.spec.cidrs.get(0).cloned().unwrap_or_default()),
                    "field is immutable",
                ));
            }
            all_errs.extend(validate_service_cidr_spec(
                &new_obj.spec,
                &Path::new("spec").child("cidrs"),
            ));
        }
        _ => {
            all_errs.push(invalid(
                &Path::new("spec").child("cidrs"),
                BadValue::String(format!("{:?}", new_obj.spec.cidrs)),
                "field is immutable",
            ));
        }
    }

    all_errs
}

/// Validates a ServiceCIDR status update.
pub fn validate_service_cidr_status_update(
    new_obj: &ServiceCIDR,
    old_obj: &ServiceCIDR,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let default_meta = crate::common::ObjectMeta::default();
    let new_meta = new_obj.metadata.as_ref().unwrap_or(&default_meta);
    let old_meta = old_obj.metadata.as_ref().unwrap_or(&default_meta);

    all_errs.extend(validate_object_meta_update(
        new_meta,
        old_meta,
        &Path::new("metadata"),
    ));

    all_errs
}

// ============================================================================
