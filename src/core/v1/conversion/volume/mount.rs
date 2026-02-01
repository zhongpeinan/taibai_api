//! VolumeMount and VolumeDevice conversions

use crate::common::traits::{FromInternal, ToInternal};
use crate::core::internal::volume as internal_volume;
use crate::core::v1::volume;

use super::helpers::*;

// ============================================================================
// VolumeMount
// ============================================================================

impl ToInternal<internal_volume::VolumeMount> for volume::VolumeMount {
    fn to_internal(self) -> internal_volume::VolumeMount {
        internal_volume::VolumeMount {
            name: self.name,
            read_only: self.read_only,
            recursive_read_only: option_string_to_recursive_readonly(self.recursive_read_only),
            mount_path: self.mount_path,
            sub_path: self.sub_path,
            mount_propagation: option_string_to_mount_propagation(self.mount_propagation),
            sub_path_expr: self.sub_path_expr,
        }
    }
}

impl FromInternal<internal_volume::VolumeMount> for volume::VolumeMount {
    fn from_internal(value: internal_volume::VolumeMount) -> Self {
        Self {
            name: value.name,
            read_only: value.read_only,
            mount_path: value.mount_path,
            sub_path: value.sub_path,
            mount_propagation: mount_propagation_to_option_string(value.mount_propagation),
            sub_path_expr: value.sub_path_expr,
            recursive_read_only: recursive_readonly_to_option_string(value.recursive_read_only),
        }
    }
}

// ============================================================================
// VolumeDevice
// ============================================================================

impl ToInternal<internal_volume::VolumeDevice> for volume::VolumeDevice {
    fn to_internal(self) -> internal_volume::VolumeDevice {
        internal_volume::VolumeDevice {
            name: self.name,
            device_path: self.device_path,
        }
    }
}

impl FromInternal<internal_volume::VolumeDevice> for volume::VolumeDevice {
    fn from_internal(value: internal_volume::VolumeDevice) -> Self {
        Self {
            name: value.name,
            device_path: value.device_path,
        }
    }
}

// ============================================================================
// VolumeMountStatus
// ============================================================================

impl ToInternal<internal_volume::VolumeMountStatus> for volume::VolumeMountStatus {
    fn to_internal(self) -> internal_volume::VolumeMountStatus {
        internal_volume::VolumeMountStatus {
            name: self.name,
            mount_path: self.mount_path,
            read_only: self.read_only,
            recursive_read_only: option_string_to_recursive_readonly(self.recursive_read_only),
        }
    }
}

impl FromInternal<internal_volume::VolumeMountStatus> for volume::VolumeMountStatus {
    fn from_internal(value: internal_volume::VolumeMountStatus) -> Self {
        Self {
            name: value.name,
            mount_path: value.mount_path,
            read_only: value.read_only,
            recursive_read_only: recursive_readonly_to_option_string(value.recursive_read_only),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::internal;

    #[test]
    fn test_volume_mount_roundtrip() {
        let v1_mount = volume::VolumeMount {
            name: "test-volume".to_string(),
            read_only: true,
            mount_path: "/mnt/data".to_string(),
            sub_path: "subdir".to_string(),
            mount_propagation: Some("Bidirectional".to_string()),
            sub_path_expr: "".to_string(),
            recursive_read_only: Some("Enabled".to_string()),
        };

        let internal_mount = v1_mount.clone().to_internal();
        assert_eq!(internal_mount.name, "test-volume");
        assert_eq!(internal_mount.read_only, true);
        assert_eq!(internal_mount.mount_path, "/mnt/data");
        assert!(matches!(
            internal_mount.mount_propagation,
            Some(internal::MountPropagationMode::Bidirectional)
        ));
        assert!(matches!(
            internal_mount.recursive_read_only,
            Some(internal::RecursiveReadOnlyMode::Enabled)
        ));

        let mut roundtrip = volume::VolumeMount::from_internal(internal_mount);
        assert_eq!(roundtrip.name, v1_mount.name);
        assert_eq!(roundtrip.read_only, v1_mount.read_only);
        assert_eq!(roundtrip.mount_propagation, v1_mount.mount_propagation);
    }

    #[test]
    fn test_volume_device_roundtrip() {
        let v1_device = volume::VolumeDevice {
            name: "disk1".to_string(),
            device_path: "/dev/xvda".to_string(),
        };

        let internal_device = v1_device.clone().to_internal();
        assert_eq!(internal_device.name, "disk1");
        assert_eq!(internal_device.device_path, "/dev/xvda");

        let mut roundtrip = volume::VolumeDevice::from_internal(internal_device);
        assert_eq!(roundtrip, v1_device);
    }
}
