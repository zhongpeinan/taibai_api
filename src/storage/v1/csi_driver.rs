//! CSIDriver types
//!
//! CSIDriver captures information about a Container Storage Interface (CSI)
//! volume driver deployed on the cluster.
//!
//! Corresponds to [Kubernetes CSIDriver](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/storage/types.go#L246)

use serde::{Deserialize, Serialize};

use crate::common::{ListMeta, ObjectMeta, TypeMeta};

/// CSIDriver captures information about a Container Storage Interface (CSI)
/// volume driver deployed on the cluster.
///
/// CSIDriver objects are non-namespaced.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSIDriver {
    /// Standard object metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// spec represents the specification of the CSI Driver.
    pub spec: CSIDriverSpec,
}

/// CSIDriverList is a collection of CSIDriver objects.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSIDriverList {
    /// Standard list metadata
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// items is the list of CSIDriver
    #[serde(default)]
    pub items: Vec<CSIDriver>,
}

/// CSIDriverSpec is the specification of a CSIDriver.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSIDriverSpec {
    /// attachRequired indicates this CSI volume driver requires an attach operation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attach_required: Option<bool>,

    /// podInfoOnMount indicates this CSI volume driver requires additional pod information
    /// during mount operations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pod_info_on_mount: Option<bool>,

    /// volumeLifecycleModes defines what kind of volumes this CSI volume driver supports.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volume_lifecycle_modes: Vec<VolumeLifecycleMode>,

    /// storageCapacity indicates that the CSI volume driver wants pod scheduling to consider
    /// the storage capacity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage_capacity: Option<bool>,

    /// fsGroupPolicy defines if the underlying volume supports changing ownership and
    /// permission of the volume before being mounted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fs_group_policy: Option<FSGroupPolicy>,

    /// tokenRequests indicates the CSI driver needs pods' service account tokens.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub token_requests: Vec<TokenRequest>,

    /// requiresRepublish indicates the CSI driver wants `NodePublishVolume` being periodically called.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requires_republish: Option<bool>,

    /// seLinuxMount specifies if the CSI driver supports "-o context" mount option.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub se_linux_mount: Option<bool>,

    /// nodeAllocatableUpdatePeriodSeconds specifies the interval between periodic updates.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_allocatable_update_period_seconds: Option<i64>,

    /// serviceAccountTokenInSecrets indicates that service account tokens should be passed
    /// via the Secrets field instead of VolumeContext.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_account_token_in_secrets: Option<bool>,
}

/// FSGroupPolicy specifies if a CSI Driver supports modifying volume ownership
/// and permissions of the volume to be mounted.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum FSGroupPolicy {
    /// ReadWriteOnceWithFSType indicates that each volume will be examined
    /// to determine if the volume ownership and permissions should be modified.
    ReadWriteOnceWithFSType,

    /// File indicates that CSI driver supports volume ownership and permission
    /// change via fsGroup for all volumes.
    File,

    /// None indicates that volumes will be mounted without performing any
    /// ownership or permission modifications.
    None,
}

/// FSGroupPolicy constants
pub mod fs_group_policy {
    /// ReadWriteOnceWithFSType is the default policy
    pub const READ_WRITE_ONCE_WITH_FS_TYPE: &str = "ReadWriteOnceWithFSType";

    /// File policy for all volumes
    pub const FILE: &str = "File";

    /// None policy for no modifications
    pub const NONE: &str = "None";
}

/// VolumeLifecycleMode is an enumeration of possible usage modes for a volume
/// provided by a CSI driver.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum VolumeLifecycleMode {
    /// Persistent mode is the default CSI spec mode
    Persistent,

    /// Ephemeral mode for inline volumes
    Ephemeral,
}

/// VolumeLifecycleMode constants
pub mod volume_lifecycle_mode {
    /// Persistent mode
    pub const PERSISTENT: &str = "Persistent";

    /// Ephemeral mode
    pub const EPHEMERAL: &str = "Ephemeral";
}

/// TokenRequest contains parameters of a service account token.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TokenRequest {
    /// audience is the intended audience of the token.
    pub audience: String,

    /// expirationSeconds is the duration of validity of the token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_seconds: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csi_driver_default() {
        let driver = CSIDriver::default();
        assert!(driver.metadata.is_none());
    }

    #[test]
    fn test_csi_driver_with_spec() {
        let spec = CSIDriverSpec {
            attach_required: Some(true),
            ..Default::default()
        };

        let driver = CSIDriver {
            spec,
            ..Default::default()
        };
        assert_eq!(driver.spec.attach_required, Some(true));
    }

    #[test]
    fn test_csi_driver_serialize() {
        let spec = CSIDriverSpec {
            attach_required: Some(true),
            pod_info_on_mount: Some(false),
            ..Default::default()
        };

        let driver = CSIDriver {
            spec,
            ..Default::default()
        };
        let json = serde_json::to_string(&driver).unwrap();
        assert!(json.contains("\"attachRequired\":true"));
    }

    #[test]
    fn test_csi_driver_deserialize() {
        let json = "{\"spec\":{\"attachRequired\":true}}";
        let driver: CSIDriver = serde_json::from_str(json).unwrap();
        assert_eq!(driver.spec.attach_required, Some(true));
    }

    #[test]
    fn test_csi_driver_round_trip() {
        let original = CSIDriver {
            spec: CSIDriverSpec {
                attach_required: Some(true),
                ..Default::default()
            },
            ..Default::default()
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: CSIDriver = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_csi_driver_spec_with_fs_group_policy() {
        let spec = CSIDriverSpec {
            fs_group_policy: Some(FSGroupPolicy::File),
            ..Default::default()
        };
        assert_eq!(spec.fs_group_policy, Some(FSGroupPolicy::File));
    }

    #[test]
    fn test_csi_driver_spec_with_volume_lifecycle_modes() {
        let spec = CSIDriverSpec {
            volume_lifecycle_modes: vec![VolumeLifecycleMode::Persistent],
            ..Default::default()
        };
        assert_eq!(spec.volume_lifecycle_modes.len(), 1);
    }

    #[test]
    fn test_csi_driver_spec_serialize() {
        let spec = CSIDriverSpec {
            attach_required: Some(true),
            fs_group_policy: Some(FSGroupPolicy::File),
            ..Default::default()
        };
        let json = serde_json::to_string(&spec).unwrap();
        assert!(json.contains("\"attachRequired\":true"));
        assert!(json.contains("\"fsGroupPolicy\":\"File\""));
    }

    #[test]
    fn test_fs_group_policy_serialize() {
        let policy = FSGroupPolicy::ReadWriteOnceWithFSType;
        let json = serde_json::to_string(&policy).unwrap();
        assert_eq!(json, r#""ReadWriteOnceWithFSType""#);
    }

    #[test]
    fn test_fs_group_policy_deserialize() {
        let json = r#""File""#;
        let policy: FSGroupPolicy = serde_json::from_str(json).unwrap();
        assert_eq!(policy, FSGroupPolicy::File);
    }

    #[test]
    fn test_fs_group_policy_round_trip() {
        let policies = vec![
            FSGroupPolicy::ReadWriteOnceWithFSType,
            FSGroupPolicy::File,
            FSGroupPolicy::None,
        ];
        for policy in policies {
            let json = serde_json::to_string(&policy).unwrap();
            let deserialized: FSGroupPolicy = serde_json::from_str(&json).unwrap();
            assert_eq!(policy, deserialized);
        }
    }

    #[test]
    fn test_fs_group_policy_constants() {
        assert_eq!(
            fs_group_policy::READ_WRITE_ONCE_WITH_FS_TYPE,
            "ReadWriteOnceWithFSType"
        );
        assert_eq!(fs_group_policy::FILE, "File");
        assert_eq!(fs_group_policy::NONE, "None");
    }

    #[test]
    fn test_volume_lifecycle_mode_serialize() {
        let mode = VolumeLifecycleMode::Persistent;
        let json = serde_json::to_string(&mode).unwrap();
        assert_eq!(json, r#""Persistent""#);
    }

    #[test]
    fn test_volume_lifecycle_mode_deserialize() {
        let json = r#""Ephemeral""#;
        let mode: VolumeLifecycleMode = serde_json::from_str(json).unwrap();
        assert_eq!(mode, VolumeLifecycleMode::Ephemeral);
    }

    #[test]
    fn test_volume_lifecycle_mode_round_trip() {
        let modes = vec![
            VolumeLifecycleMode::Persistent,
            VolumeLifecycleMode::Ephemeral,
        ];
        for mode in modes {
            let json = serde_json::to_string(&mode).unwrap();
            let deserialized: VolumeLifecycleMode = serde_json::from_str(&json).unwrap();
            assert_eq!(mode, deserialized);
        }
    }

    #[test]
    fn test_volume_lifecycle_mode_constants() {
        assert_eq!(volume_lifecycle_mode::PERSISTENT, "Persistent");
        assert_eq!(volume_lifecycle_mode::EPHEMERAL, "Ephemeral");
    }

    #[test]
    fn test_token_request_default() {
        let tr = TokenRequest::default();
        assert!(tr.audience.is_empty());
        assert!(tr.expiration_seconds.is_none());
    }

    #[test]
    fn test_token_request_with_expiration() {
        let tr = TokenRequest {
            audience: "api-server".to_string(),
            expiration_seconds: Some(3600),
        };
        assert_eq!(tr.audience, "api-server");
        assert_eq!(tr.expiration_seconds, Some(3600));
    }

    #[test]
    fn test_token_request_serialize() {
        let tr = TokenRequest {
            audience: "api-server".to_string(),
            expiration_seconds: Some(3600),
        };
        let json = serde_json::to_string(&tr).unwrap();
        assert!(json.contains("\"audience\":\"api-server\""));
        assert!(json.contains("\"expirationSeconds\":3600"));
    }

    #[test]
    fn test_token_request_deserialize() {
        let json = "{\"audience\":\"api-server\",\"expirationSeconds\":3600}";
        let tr: TokenRequest = serde_json::from_str(json).unwrap();
        assert_eq!(tr.audience, "api-server");
        assert_eq!(tr.expiration_seconds, Some(3600));
    }

    #[test]
    fn test_token_request_round_trip() {
        let original = TokenRequest {
            audience: "api-server".to_string(),
            expiration_seconds: Some(3600),
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: TokenRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_csi_driver_list() {
        let driver = CSIDriver {
            spec: CSIDriverSpec::default(),
            ..Default::default()
        };

        let list = CSIDriverList {
            items: vec![driver],
            ..Default::default()
        };
        assert_eq!(list.items.len(), 1);
    }

    #[test]
    fn test_csi_driver_list_serialize() {
        let list = CSIDriverList {
            items: vec![],
            ..Default::default()
        };
        let json = serde_json::to_string(&list).unwrap();
        assert!(json.contains("\"items\":[]"));
    }
}
