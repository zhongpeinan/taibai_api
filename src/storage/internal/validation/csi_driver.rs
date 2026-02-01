use std::collections::HashSet;

use crate::common::ObjectMeta;
use crate::common::validation::{
    BadValue, ErrorList, Path, duplicate, invalid, name_is_dns_subdomain, required,
    validate_object_meta,
};
use crate::storage::v1 as storage_v1;
use crate::storage::v1beta1 as storage_v1beta1;

use super::validate_csi_driver_name;

pub fn validate_csi_driver_v1(obj: &storage_v1::CSIDriver) -> ErrorList {
    validate_csi_driver_common(obj)
}

pub fn validate_csi_driver_v1beta1(obj: &storage_v1beta1::CSIDriver) -> ErrorList {
    validate_csi_driver_common(&storage_v1::CSIDriver {
        type_meta: obj.type_meta.clone(),
        metadata: obj.metadata.clone(),
        spec: storage_v1::CSIDriverSpec {
            attach_required: obj.spec.attach_required,
            pod_info_on_mount: obj.spec.pod_info_on_mount,
            volume_lifecycle_modes: obj
                .spec
                .volume_lifecycle_modes
                .iter()
                .map(|mode| match mode {
                    storage_v1beta1::VolumeLifecycleMode::Persistent => {
                        storage_v1::VolumeLifecycleMode::Persistent
                    }
                    storage_v1beta1::VolumeLifecycleMode::Ephemeral => {
                        storage_v1::VolumeLifecycleMode::Ephemeral
                    }
                })
                .collect(),
            storage_capacity: obj.spec.storage_capacity,
            fs_group_policy: obj
                .spec
                .fs_group_policy
                .as_ref()
                .map(|policy| match policy {
                    storage_v1beta1::FSGroupPolicy::ReadWriteOnceWithFSType => {
                        storage_v1::FSGroupPolicy::ReadWriteOnceWithFSType
                    }
                    storage_v1beta1::FSGroupPolicy::File => storage_v1::FSGroupPolicy::File,
                    storage_v1beta1::FSGroupPolicy::None => storage_v1::FSGroupPolicy::None,
                }),
            token_requests: obj
                .spec
                .token_requests
                .iter()
                .map(|req| storage_v1::TokenRequest {
                    audience: req.audience.clone(),
                    expiration_seconds: req.expiration_seconds,
                })
                .collect(),
            requires_republish: obj.spec.requires_republish,
            se_linux_mount: obj.spec.se_linux_mount,
            node_allocatable_update_period_seconds: obj.spec.node_allocatable_update_period_seconds,
            service_account_token_in_secrets: None,
        },
    })
}

fn validate_csi_driver_common(obj: &storage_v1::CSIDriver) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let base_path = Path::nil();
    let default_meta = ObjectMeta::default();
    let metadata = obj.metadata.as_ref().unwrap_or(&default_meta);

    all_errs.extend(validate_object_meta(
        metadata,
        false,
        name_is_dns_subdomain,
        &base_path.child("metadata"),
    ));
    all_errs.extend(validate_csi_driver_name(
        metadata.name.as_deref().unwrap_or(""),
        &base_path.child("metadata").child("name"),
    ));
    all_errs.extend(validate_csi_driver_spec(
        &obj.spec,
        &base_path.child("spec"),
    ));

    all_errs
}

fn validate_csi_driver_spec(spec: &storage_v1::CSIDriverSpec, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if spec.attach_required.is_none() {
        all_errs.push(required(&path.child("attachRequired"), ""));
    }
    if spec.pod_info_on_mount.is_none() {
        all_errs.push(required(&path.child("podInfoOnMount"), ""));
    }
    if spec.storage_capacity.is_none() {
        all_errs.push(required(&path.child("storageCapacity"), ""));
    }

    let mut audiences = HashSet::new();
    for (i, request) in spec.token_requests.iter().enumerate() {
        let req_path = path.child("tokenRequests").index(i);
        if !audiences.insert(request.audience.clone()) {
            all_errs.push(duplicate(
                &req_path.child("audience"),
                BadValue::String(request.audience.clone()),
            ));
        }
        if let Some(exp) = request.expiration_seconds {
            if exp < 600 {
                all_errs.push(invalid(
                    &req_path.child("expirationSeconds"),
                    BadValue::Int(exp),
                    "may not specify a duration less than 10 minutes",
                ));
            }
            if exp > (1u64 << 32) as i64 {
                all_errs.push(invalid(
                    &req_path.child("expirationSeconds"),
                    BadValue::Int(exp),
                    "may not specify a duration larger than 2^32 seconds",
                ));
            }
        }
    }

    if let Some(period) = spec.node_allocatable_update_period_seconds {
        if period < 10 {
            all_errs.push(invalid(
                &path.child("nodeAllocatableUpdatePeriodSeconds"),
                BadValue::Int(period),
                "must be greater than or equal to 10 seconds",
            ));
        }
    }

    all_errs
}
