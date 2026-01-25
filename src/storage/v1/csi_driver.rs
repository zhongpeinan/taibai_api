//! CSIDriver types
//!
//! CSIDriver captures information about a Container Storage Interface (CSI)
//! volume driver deployed on the cluster.
//!
//! Corresponds to [Kubernetes CSIDriver](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/storage/types.go#L246)

use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

use crate::common::{
    ApplyDefault, HasTypeMeta, ListMeta, ObjectMeta, ResourceSchema, TypeMeta, VersionedObject,
};
use crate::impl_unimplemented_prost_message;

use super::defaults::set_defaults_csi_driver;

/// CSIDriver captures information about a Container Storage Interface (CSI)
/// volume driver deployed on the cluster.
///
/// CSIDriver objects are non-namespaced.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSIDriver {
    /// TypeMeta for this resource
    #[serde(flatten)]
    pub type_meta: TypeMeta,

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
    /// TypeMeta for this resource
    #[serde(flatten)]
    pub type_meta: TypeMeta,

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

// ============================================================================
// Trait Implementations for CSIDriver and CSIDriverList
// ============================================================================

// ----------------------------------------------------------------------------
// ResourceSchema Implementation
// ----------------------------------------------------------------------------

impl ResourceSchema for CSIDriver {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "storage.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "CSIDriver"
    }
    fn resource(_: &Self::Meta) -> &str {
        "csidrivers"
    }

    fn group_static() -> &'static str {
        "storage.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "CSIDriver"
    }
    fn resource_static() -> &'static str {
        "csidrivers"
    }
}

impl ResourceSchema for CSIDriverList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "storage.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "CSIDriverList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "csidrivers"
    }

    fn group_static() -> &'static str {
        "storage.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "CSIDriverList"
    }
    fn resource_static() -> &'static str {
        "csidrivers"
    }
}

// ----------------------------------------------------------------------------
// HasTypeMeta Implementation
// ----------------------------------------------------------------------------

impl HasTypeMeta for CSIDriver {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for CSIDriverList {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

// ----------------------------------------------------------------------------
// VersionedObject Implementation
// ----------------------------------------------------------------------------

impl VersionedObject for CSIDriver {
    fn metadata(&self) -> &ObjectMeta {
        self.metadata
            .as_ref()
            .unwrap_or_else(|| static_default_object_meta())
    }

    fn metadata_mut(&mut self) -> &mut ObjectMeta {
        self.metadata.get_or_insert_with(ObjectMeta::default)
    }
}

// Helper function for static default ObjectMeta
fn static_default_object_meta() -> &'static ObjectMeta {
    static DEFAULT: OnceLock<ObjectMeta> = OnceLock::new();
    DEFAULT.get_or_init(ObjectMeta::default)
}

// Note: CSIDriverList does not implement VersionedObject because its metadata is ListMeta

// ----------------------------------------------------------------------------
// ApplyDefaults Implementation
// ----------------------------------------------------------------------------

impl ApplyDefault for CSIDriver {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "storage.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "CSIDriver".to_string();
        }
        set_defaults_csi_driver(self);
    }
}

impl ApplyDefault for CSIDriverList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "storage.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "CSIDriverList".to_string();
        }
    }
}

// ----------------------------------------------------------------------------
// Protobuf Placeholder (using macro)
// ----------------------------------------------------------------------------

impl_unimplemented_prost_message!(CSIDriver);
impl_unimplemented_prost_message!(CSIDriverList);

#[cfg(test)]
mod tests {}
