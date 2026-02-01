use std::collections::HashSet;

use crate::common::ObjectMeta;
use crate::common::validation::{
    BadValue, ErrorList, Path, duplicate, invalid, name_is_dns_subdomain, required,
    validate_object_meta, validate_qualified_name,
};
use crate::storage::v1 as storage_v1;
use crate::storage::v1beta1 as storage_v1beta1;

use super::validate_csi_driver_name;

const CSI_NODE_ID_MAX_LENGTH: usize = 192;
const CSI_NODE_ID_LONGER_MAX_LENGTH: usize = 256;

/// Options for CSINode validation.
#[derive(Clone, Copy, Debug, Default)]
pub struct CSINodeValidationOptions {
    pub allow_long_node_id: bool,
}

pub fn validate_csi_node_v1(
    obj: &storage_v1::CSINode,
    opts: CSINodeValidationOptions,
) -> ErrorList {
    validate_csi_node_common(
        obj.metadata.as_ref().unwrap_or(&ObjectMeta::default()),
        &obj.spec,
        opts,
    )
}

pub fn validate_csi_node_v1beta1(
    obj: &storage_v1beta1::CSINode,
    opts: CSINodeValidationOptions,
) -> ErrorList {
    let spec = storage_v1::CSINodeSpec {
        drivers: obj
            .spec
            .drivers
            .iter()
            .map(|driver| storage_v1::CSINodeDriver {
                name: driver.name.clone(),
                node_id: driver.node_id.clone(),
                topology_keys: driver.topology_keys.clone(),
                allocatable: driver
                    .allocatable
                    .as_ref()
                    .map(|alloc| storage_v1::VolumeNodeResources { count: alloc.count }),
            })
            .collect(),
    };
    validate_csi_node_common(
        obj.metadata.as_ref().unwrap_or(&ObjectMeta::default()),
        &spec,
        opts,
    )
}

fn validate_csi_node_common(
    metadata: &ObjectMeta,
    spec: &storage_v1::CSINodeSpec,
    opts: CSINodeValidationOptions,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let base_path = Path::nil();

    all_errs.extend(validate_object_meta(
        metadata,
        false,
        name_is_dns_subdomain,
        &base_path.child("metadata"),
    ));

    let mut driver_names = HashSet::new();
    for (i, driver) in spec.drivers.iter().enumerate() {
        let driver_path = base_path.child("spec").child("drivers").index(i);
        all_errs.extend(validate_csi_driver_name(
            &driver.name,
            &driver_path.child("name"),
        ));
        all_errs.extend(validate_csi_node_driver_node_id(
            &driver.node_id,
            &driver_path.child("nodeID"),
            opts.allow_long_node_id,
        ));
        if driver_names.contains(&driver.name) {
            all_errs.push(duplicate(
                &driver_path.child("name"),
                BadValue::String(driver.name.clone()),
            ));
        }
        driver_names.insert(driver.name.clone());

        let mut topo_keys = HashSet::new();
        for key in &driver.topology_keys {
            if key.is_empty() {
                all_errs.push(required(&driver_path.child("topologyKeys"), ""));
            }
            if !topo_keys.insert(key.clone()) {
                all_errs.push(duplicate(
                    &driver_path.child("topologyKeys"),
                    BadValue::String(key.clone()),
                ));
            }
            all_errs.extend(validate_qualified_name(
                key,
                &driver_path.child("topologyKeys"),
            ));
        }

        if let Some(alloc) = &driver.allocatable {
            if let Some(count) = alloc.count {
                if count < 0 {
                    all_errs.push(invalid(
                        &driver_path.child("allocatable").child("count"),
                        BadValue::Int(count as i64),
                        "must be non-negative",
                    ));
                }
            }
        }
    }

    all_errs
}

fn validate_csi_node_driver_node_id(node_id: &str, path: &Path, allow_long: bool) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if node_id.is_empty() {
        all_errs.push(required(path, "nodeID is required"));
        return all_errs;
    }
    let max = if allow_long {
        CSI_NODE_ID_LONGER_MAX_LENGTH
    } else {
        CSI_NODE_ID_MAX_LENGTH
    };
    if node_id.len() > max {
        all_errs.push(invalid(
            path,
            BadValue::String(node_id.to_string()),
            &format!("must be {max} characters or less"),
        ));
    }
    all_errs
}
