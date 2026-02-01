use crate::common::validation::ErrorList;
use crate::storage::internal::validation as internal_validation;
use crate::storage::v1 as storage_v1;

pub fn validate_volume_attributes_class_v1(obj: &storage_v1::VolumeAttributesClass) -> ErrorList {
    internal_validation::validate_volume_attributes_class_v1(obj)
}
