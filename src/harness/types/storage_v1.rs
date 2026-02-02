//! Storage v1 type registrations for the test harness.

use crate::harness::helpers::register_type;
use crate::harness::registry::Registry;

pub fn register(registry: &mut Registry) {
    // ---- storage/v1/StorageClass ----
    register_type::<crate::storage::v1::StorageClass, crate::storage::internal::StorageClass, _>(
        registry,
        "storage/v1/StorageClass",
        crate::storage::v1::validation::validate_storage_class_v1,
    );

    // ---- storage/v1/CSIDriver ----
    register_type::<crate::storage::v1::CSIDriver, crate::storage::internal::CSIDriver, _>(
        registry,
        "storage/v1/CSIDriver",
        crate::storage::v1::validation::validate_csi_driver_v1,
    );

    // Note: CSINode validation requires CSINodeValidationOptions, so we skip it for now
    // TODO: Add CSINode when we have a way to handle validation options

    // ---- storage/v1/VolumeAttachment ----
    register_type::<
        crate::storage::v1::VolumeAttachment,
        crate::storage::internal::VolumeAttachment,
        _,
    >(
        registry,
        "storage/v1/VolumeAttachment",
        crate::storage::v1::validation::validate_volume_attachment_v1,
    );
}
