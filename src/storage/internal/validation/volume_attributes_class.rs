use crate::common::ObjectMeta;
use crate::common::validation::{ErrorList, Path, name_is_dns_subdomain, validate_object_meta};
use crate::storage::v1 as storage_v1;
use crate::storage::v1alpha1 as storage_v1alpha1;
use crate::storage::v1beta1 as storage_v1beta1;

use super::{validate_parameters, validate_provisioner};

pub fn validate_volume_attributes_class_v1(obj: &storage_v1::VolumeAttributesClass) -> ErrorList {
    validate_volume_attributes_class_common(obj)
}

pub fn validate_volume_attributes_class_v1beta1(
    obj: &storage_v1beta1::VolumeAttributesClass,
) -> ErrorList {
    validate_volume_attributes_class_common(&storage_v1::VolumeAttributesClass {
        type_meta: obj.type_meta.clone(),
        metadata: obj.metadata.clone(),
        driver_name: obj.driver_name.clone(),
        parameters: obj.parameters.clone(),
    })
}

pub fn validate_volume_attributes_class_v1alpha1(
    obj: &storage_v1alpha1::VolumeAttributesClass,
) -> ErrorList {
    validate_volume_attributes_class_common(&storage_v1::VolumeAttributesClass {
        type_meta: obj.type_meta.clone(),
        metadata: obj.metadata.clone(),
        driver_name: obj.driver_name.clone(),
        parameters: obj.parameters.clone(),
    })
}

fn validate_volume_attributes_class_common(obj: &storage_v1::VolumeAttributesClass) -> ErrorList {
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

    all_errs.extend(validate_provisioner(
        &obj.driver_name,
        &base_path.child("driverName"),
    ));

    all_errs.extend(validate_parameters(
        &obj.parameters,
        false,
        &base_path.child("parameters"),
    ));

    all_errs
}
