use crate::common::ObjectMeta;
use crate::common::validation::{
    BadValue, ErrorList, Path, invalid, name_is_dns_subdomain, required, validate_object_meta,
};
use crate::storage::v1 as storage_v1;
use crate::storage::v1alpha1 as storage_v1alpha1;
use crate::storage::v1beta1 as storage_v1beta1;

use super::validate_label_selector;

pub fn validate_csi_storage_capacity_v1(obj: &storage_v1::CSIStorageCapacity) -> ErrorList {
    validate_csi_storage_capacity_common(obj)
}

pub fn validate_csi_storage_capacity_v1beta1(
    obj: &storage_v1beta1::CSIStorageCapacity,
) -> ErrorList {
    validate_csi_storage_capacity_common(&storage_v1::CSIStorageCapacity {
        type_meta: obj.type_meta.clone(),
        metadata: obj.metadata.clone(),
        node_topology: obj.node_topology.clone(),
        storage_class_name: obj.storage_class_name.clone(),
        capacity: obj.capacity.clone(),
        maximum_volume_size: obj.maximum_volume_size.clone(),
    })
}

pub fn validate_csi_storage_capacity_v1alpha1(
    obj: &storage_v1alpha1::CSIStorageCapacity,
) -> ErrorList {
    validate_csi_storage_capacity_common(&storage_v1::CSIStorageCapacity {
        type_meta: obj.type_meta.clone(),
        metadata: obj.metadata.clone(),
        node_topology: obj.node_topology.clone(),
        storage_class_name: obj.storage_class_name.clone(),
        capacity: obj.capacity.clone(),
        maximum_volume_size: obj.maximum_volume_size.clone(),
    })
}

fn validate_csi_storage_capacity_common(obj: &storage_v1::CSIStorageCapacity) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let base_path = Path::nil();
    let default_meta = ObjectMeta::default();
    let metadata = obj.metadata.as_ref().unwrap_or(&default_meta);

    all_errs.extend(validate_object_meta(
        metadata,
        true,
        name_is_dns_subdomain,
        &base_path.child("metadata"),
    ));

    if let Some(selector) = obj.node_topology.as_ref() {
        all_errs.extend(validate_label_selector(
            selector,
            &base_path.child("nodeTopology"),
        ));
    }

    if obj.storage_class_name.is_empty() {
        all_errs.push(required(
            &base_path.child("storageClassName"),
            "storageClassName is required",
        ));
    } else {
        for msg in crate::common::validation::is_dns1123_subdomain(&obj.storage_class_name) {
            all_errs.push(invalid(
                &base_path.child("storageClassName"),
                BadValue::String(obj.storage_class_name.clone()),
                &msg,
            ));
        }
    }

    all_errs
}
