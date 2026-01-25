//! StorageClass types
//!
//! StorageClass describes the parameters for a class of storage for
//! which PersistentVolumes can be dynamically provisioned.
//!
//! Corresponds to [Kubernetes StorageClass](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/storage/types.go#L30)

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::OnceLock;

use crate::common::{
    ApplyDefault, HasTypeMeta, ListMeta, ObjectMeta, PersistentVolumeReclaimPolicy, ResourceSchema,
    TopologySelectorTerm, TypeMeta, VersionedObject,
};
use crate::impl_unimplemented_prost_message;

use super::defaults::set_defaults_storage_class;

/// StorageClass describes the parameters for a class of storage for
/// which PersistentVolumes can be dynamically provisioned.
///
/// StorageClasses are non-namespaced; the name of the storage class
/// according to etcd is in ObjectMeta.Name.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StorageClass {
    /// TypeMeta for this resource
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// provisioner indicates the type of the provisioner.
    pub provisioner: String,

    /// parameters holds the parameters for the provisioner that should
    /// create volumes of this storage class.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub parameters: BTreeMap<String, String>,

    /// reclaimPolicy controls the reclaimPolicy for dynamically provisioned PersistentVolumes.
    /// Defaults to Delete.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reclaim_policy: Option<PersistentVolumeReclaimPolicy>,

    /// mountOptions controls the mountOptions for dynamically provisioned PersistentVolumes.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mount_options: Vec<String>,

    /// allowVolumeExpansion shows whether the storage class allow volume expand.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_volume_expansion: Option<bool>,

    /// volumeBindingMode indicates how PersistentVolumeClaims should be
    /// provisioned and bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume_binding_mode: Option<VolumeBindingMode>,

    /// allowedTopologies restrict the node topologies where volumes can be dynamically provisioned.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_topologies: Vec<TopologySelectorTerm>,
}

/// StorageClassList is a collection of storage classes.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StorageClassList {
    /// TypeMeta for this resource
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard list metadata
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// items is the list of StorageClasses
    #[serde(default)]
    pub items: Vec<StorageClass>,
}

/// VolumeBindingMode indicates how PersistentVolumeClaims should be bound.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum VolumeBindingMode {
    /// Immediate indicates that PersistentVolumeClaims should be
    /// immediately provisioned and bound.
    Immediate,

    /// WaitForFirstConsumer indicates that PersistentVolumeClaims
    /// should not be provisioned and bound until the first Pod is created.
    WaitForFirstConsumer,
}

/// VolumeBindingMode constants
pub mod volume_binding_mode {
    /// Immediate indicates that PersistentVolumeClaims should be immediately provisioned
    pub const IMMEDIATE: &str = "Immediate";

    /// WaitForFirstConsumer indicates that PersistentVolumeClaims should wait for first consumer
    pub const WAIT_FOR_FIRST_CONSUMER: &str = "WaitForFirstConsumer";
}

// ============================================================================
// Trait Implementations for StorageClass and StorageClassList
// ============================================================================

// ----------------------------------------------------------------------------
// ResourceSchema Implementation
// ----------------------------------------------------------------------------

impl ResourceSchema for StorageClass {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "storage.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "StorageClass"
    }
    fn resource(_: &Self::Meta) -> &str {
        "storageclasses"
    }

    fn group_static() -> &'static str {
        "storage.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "StorageClass"
    }
    fn resource_static() -> &'static str {
        "storageclasses"
    }
}

impl ResourceSchema for StorageClassList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "storage.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "StorageClassList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "storageclasses"
    }

    fn group_static() -> &'static str {
        "storage.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "StorageClassList"
    }
    fn resource_static() -> &'static str {
        "storageclasses"
    }
}

// ----------------------------------------------------------------------------
// HasTypeMeta Implementation
// ----------------------------------------------------------------------------

impl HasTypeMeta for StorageClass {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for StorageClassList {
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

impl VersionedObject for StorageClass {
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

// Note: StorageClassList does not implement VersionedObject because its metadata is ListMeta

// ----------------------------------------------------------------------------
// ApplyDefaults Implementation
// ----------------------------------------------------------------------------

impl ApplyDefault for StorageClass {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "storage.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "StorageClass".to_string();
        }
        set_defaults_storage_class(self);
    }
}

impl ApplyDefault for StorageClassList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "storage.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "StorageClassList".to_string();
        }
    }
}

// ----------------------------------------------------------------------------
// Protobuf Placeholder (using macro)
// ----------------------------------------------------------------------------

impl_unimplemented_prost_message!(StorageClass);
impl_unimplemented_prost_message!(StorageClassList);

#[cfg(test)]
mod tests {}
