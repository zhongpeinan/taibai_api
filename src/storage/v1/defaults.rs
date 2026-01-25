//! Defaulting functions for storage/v1 API types
//!
//! Ported from k8s.io/kubernetes/pkg/apis/storage/v1/defaults.go and zz_generated.defaults.go

use crate::common::PersistentVolumeReclaimPolicy;

use super::{
    CSIDriver, FSGroupPolicy, StorageClass, VolumeAttachment, VolumeBindingMode,
    VolumeLifecycleMode,
};

/// Apply defaults to StorageClass.
pub fn set_defaults_storage_class(obj: &mut StorageClass) {
    if obj.reclaim_policy.is_none() {
        obj.reclaim_policy = Some(PersistentVolumeReclaimPolicy::Delete);
    }
    if obj.volume_binding_mode.is_none() {
        obj.volume_binding_mode = Some(VolumeBindingMode::Immediate);
    }
}

/// Apply defaults to CSIDriver.
pub fn set_defaults_csi_driver(obj: &mut CSIDriver) {
    let spec = &mut obj.spec;
    if spec.attach_required.is_none() {
        spec.attach_required = Some(true);
    }
    if spec.pod_info_on_mount.is_none() {
        spec.pod_info_on_mount = Some(false);
    }
    if spec.storage_capacity.is_none() {
        spec.storage_capacity = Some(false);
    }
    if spec.fs_group_policy.is_none() {
        spec.fs_group_policy = Some(FSGroupPolicy::ReadWriteOnceWithFSType);
    }
    if spec.volume_lifecycle_modes.is_empty() {
        spec.volume_lifecycle_modes
            .push(VolumeLifecycleMode::Persistent);
    }
    if spec.requires_republish.is_none() {
        spec.requires_republish = Some(false);
    }
    if spec.se_linux_mount.is_none() {
        spec.se_linux_mount = Some(false);
    }
}

/// Apply defaults to VolumeAttachment.
pub fn set_defaults_volume_attachment(_obj: &mut VolumeAttachment) {
    // Inline volume spec defaults are not modeled in the minimal PersistentVolumeSpec.
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::TypeMeta;

    #[test]
    fn test_default_storage_class_defaults() {
        let mut storage_class = StorageClass {
            type_meta: TypeMeta::default(),
            metadata: None,
            provisioner: "example.com/prov".to_string(),
            parameters: Default::default(),
            reclaim_policy: None,
            mount_options: vec![],
            allow_volume_expansion: None,
            volume_binding_mode: None,
            allowed_topologies: vec![],
        };

        set_defaults_storage_class(&mut storage_class);

        assert_eq!(
            storage_class.reclaim_policy,
            Some(PersistentVolumeReclaimPolicy::Delete)
        );
        assert_eq!(
            storage_class.volume_binding_mode,
            Some(VolumeBindingMode::Immediate)
        );
    }

    #[test]
    fn test_default_csi_driver_defaults() {
        let mut driver = CSIDriver::default();
        set_defaults_csi_driver(&mut driver);

        assert_eq!(driver.spec.attach_required, Some(true));
        assert_eq!(driver.spec.pod_info_on_mount, Some(false));
        assert_eq!(driver.spec.storage_capacity, Some(false));
        assert_eq!(
            driver.spec.fs_group_policy,
            Some(FSGroupPolicy::ReadWriteOnceWithFSType)
        );
        assert_eq!(
            driver.spec.volume_lifecycle_modes,
            vec![VolumeLifecycleMode::Persistent]
        );
        assert_eq!(driver.spec.requires_republish, Some(false));
        assert_eq!(driver.spec.se_linux_mount, Some(false));
    }
}
