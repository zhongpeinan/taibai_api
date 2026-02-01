use crate::common::ObjectMeta;
use crate::common::validation::{
    ErrorList, Path, name_is_dns_subdomain, required, validate_object_meta,
};
use crate::storage::v1 as storage_v1;
use crate::storage::v1alpha1 as storage_v1alpha1;
use crate::storage::v1beta1 as storage_v1beta1;

use super::{
    validate_attachment_metadata, validate_csi_driver_name, validate_node_name,
    validate_volume_attachment_source, validate_volume_error,
};

pub fn validate_volume_attachment_v1(obj: &storage_v1::VolumeAttachment) -> ErrorList {
    let mut all_errs = validate_volume_attachment_common(
        obj.metadata.as_ref().unwrap_or(&ObjectMeta::default()),
        &obj.spec.attacher,
        &obj.spec.source,
        &obj.spec.node_name,
        &obj.status,
        &Path::nil(),
    );

    // v1 adds CSI driver name validation for attacher.
    all_errs.extend(validate_csi_driver_name(
        &obj.spec.attacher,
        &Path::nil().child("spec").child("attacher"),
    ));

    all_errs
}

pub fn validate_volume_attachment_v1beta1(obj: &storage_v1beta1::VolumeAttachment) -> ErrorList {
    validate_volume_attachment_common(
        obj.metadata.as_ref().unwrap_or(&ObjectMeta::default()),
        &obj.spec.attacher,
        &storage_v1::VolumeAttachmentSource {
            persistent_volume_name: obj.spec.source.persistent_volume_name.clone(),
            inline_volume_spec: obj.spec.source.inline_volume_spec.clone(),
        },
        &obj.spec.node_name,
        &obj.status
            .as_ref()
            .map(|status| storage_v1::VolumeAttachmentStatus {
                attached: status.attached,
                attachment_metadata: status.attachment_metadata.clone(),
                attach_error: status.attach_error.as_ref().map(volume_error_v1beta1_to_v1),
                detach_error: status.detach_error.as_ref().map(volume_error_v1beta1_to_v1),
            }),
        &Path::nil(),
    )
}

pub fn validate_volume_attachment_v1alpha1(obj: &storage_v1alpha1::VolumeAttachment) -> ErrorList {
    validate_volume_attachment_common(
        obj.metadata.as_ref().unwrap_or(&ObjectMeta::default()),
        &obj.spec.attacher,
        &storage_v1::VolumeAttachmentSource {
            persistent_volume_name: obj.spec.source.persistent_volume_name.clone(),
            inline_volume_spec: obj.spec.source.inline_volume_spec.clone(),
        },
        &obj.spec.node_name,
        &obj.status
            .as_ref()
            .map(|status| storage_v1::VolumeAttachmentStatus {
                attached: status.attached,
                attachment_metadata: status.attachment_metadata.clone(),
                attach_error: status
                    .attach_error
                    .as_ref()
                    .map(volume_error_v1alpha1_to_v1),
                detach_error: status
                    .detach_error
                    .as_ref()
                    .map(volume_error_v1alpha1_to_v1),
            }),
        &Path::nil(),
    )
}

fn validate_volume_attachment_common(
    metadata: &ObjectMeta,
    attacher: &str,
    source: &storage_v1::VolumeAttachmentSource,
    node_name: &str,
    status: &Option<storage_v1::VolumeAttachmentStatus>,
    base_path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    all_errs.extend(validate_object_meta(
        metadata,
        false,
        name_is_dns_subdomain,
        &base_path.child("metadata"),
    ));

    if attacher.is_empty() {
        all_errs.push(required(
            &base_path.child("spec").child("attacher"),
            "attacher is required",
        ));
    }

    all_errs.extend(validate_volume_attachment_source(
        source,
        &base_path.child("spec").child("source"),
    ));
    all_errs.extend(validate_node_name(
        node_name,
        &base_path.child("spec").child("nodeName"),
    ));

    if let Some(status) = status {
        all_errs.extend(validate_attachment_metadata(
            &status.attachment_metadata,
            &base_path.child("status").child("attachmentMetadata"),
        ));
        all_errs.extend(validate_volume_error(
            &status.attach_error,
            &base_path.child("status").child("attachError"),
        ));
        all_errs.extend(validate_volume_error(
            &status.detach_error,
            &base_path.child("status").child("detachError"),
        ));
    }

    all_errs
}

fn volume_error_v1beta1_to_v1(error: &storage_v1beta1::VolumeError) -> storage_v1::VolumeError {
    storage_v1::VolumeError {
        time: error.time.clone(),
        message: error.message.clone(),
        error_code: error.error_code,
    }
}

fn volume_error_v1alpha1_to_v1(error: &storage_v1alpha1::VolumeError) -> storage_v1::VolumeError {
    storage_v1::VolumeError {
        time: error.time.clone(),
        message: error.message.clone(),
        error_code: error.error_code,
    }
}
