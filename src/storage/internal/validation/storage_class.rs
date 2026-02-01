use crate::common::validation::{ErrorList, Path, name_is_dns_subdomain, validate_object_meta};
use crate::common::{ObjectMeta, PersistentVolumeReclaimPolicy, TopologySelectorTerm};
use crate::storage::v1 as storage_v1;
use crate::storage::v1beta1 as storage_v1beta1;

use super::{
    validate_allowed_topologies, validate_parameters, validate_provisioner,
    validate_reclaim_policy, validate_volume_binding_mode,
};

pub fn validate_storage_class_v1(obj: &storage_v1::StorageClass) -> ErrorList {
    let base_path = Path::nil();
    validate_storage_class_common(
        obj.metadata.as_ref().unwrap_or(&ObjectMeta::default()),
        &obj.provisioner,
        &obj.parameters,
        &obj.reclaim_policy,
        obj.volume_binding_mode.as_ref().map(|mode| match mode {
            storage_v1::VolumeBindingMode::Immediate => "Immediate".to_string(),
            storage_v1::VolumeBindingMode::WaitForFirstConsumer => {
                "WaitForFirstConsumer".to_string()
            }
        }),
        &obj.allowed_topologies,
        &base_path,
    )
}

pub fn validate_storage_class_v1beta1(obj: &storage_v1beta1::StorageClass) -> ErrorList {
    let base_path = Path::nil();
    validate_storage_class_common(
        obj.metadata.as_ref().unwrap_or(&ObjectMeta::default()),
        &obj.provisioner,
        &obj.parameters,
        &obj.reclaim_policy,
        obj.volume_binding_mode.as_ref().map(|mode| match mode {
            storage_v1beta1::VolumeBindingMode::Immediate => "Immediate".to_string(),
            storage_v1beta1::VolumeBindingMode::WaitForFirstConsumer => {
                "WaitForFirstConsumer".to_string()
            }
        }),
        &obj.allowed_topologies,
        &base_path,
    )
}

fn validate_storage_class_common(
    metadata: &ObjectMeta,
    provisioner: &str,
    parameters: &std::collections::BTreeMap<String, String>,
    reclaim_policy: &Option<PersistentVolumeReclaimPolicy>,
    volume_binding_mode: Option<String>,
    allowed_topologies: &[TopologySelectorTerm],
    base_path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    all_errs.extend(validate_object_meta(
        metadata,
        false,
        name_is_dns_subdomain,
        &base_path.child("metadata"),
    ));

    all_errs.extend(validate_provisioner(
        provisioner,
        &base_path.child("provisioner"),
    ));
    all_errs.extend(validate_parameters(
        parameters,
        true,
        &base_path.child("parameters"),
    ));
    all_errs.extend(validate_reclaim_policy(
        reclaim_policy,
        &base_path.child("reclaimPolicy"),
    ));
    all_errs.extend(validate_volume_binding_mode(
        &volume_binding_mode,
        &base_path.child("volumeBindingMode"),
    ));
    all_errs.extend(validate_allowed_topologies(
        allowed_topologies,
        &base_path.child("allowedTopologies"),
    ));

    all_errs
}
