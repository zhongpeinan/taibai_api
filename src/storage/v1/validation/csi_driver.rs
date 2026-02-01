use crate::common::validation::ErrorList;
use crate::storage::internal::validation as internal_validation;
use crate::storage::v1 as storage_v1;

pub fn validate_csi_driver_v1(obj: &storage_v1::CSIDriver) -> ErrorList {
    internal_validation::validate_csi_driver_v1(obj)
}
