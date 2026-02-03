//! CSIStorageCapacity types
//!
//! CSIStorageCapacity stores the result of one CSI GetCapacity call.
//!
//! Corresponds to [Kubernetes CSIStorageCapacity](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/storage/types.go#L648)

use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

use crate::common::{
    ApplyDefault, HasTypeMeta, LabelSelector, ListMeta, ObjectMeta, Quantity, ResourceSchema,
    TypeMeta, VersionedObject,
};
use crate::impl_unimplemented_prost_message;

/// CSIStorageCapacity stores the result of one CSI GetCapacity call.
///
/// For a given StorageClass, this describes the available capacity in a
/// particular topology segment.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSIStorageCapacity {
    /// TypeMeta for this resource
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// nodeTopology defines which nodes have access to the storage
    /// for which capacity was reported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_topology: Option<LabelSelector>,

    /// storageClassName represents the name of the StorageClass.
    pub storage_class_name: String,

    /// capacity is the value reported by the CSI driver.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<Quantity>,

    /// maximumVolumeSize is the largest size that may be used to create a volume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_volume_size: Option<Quantity>,
}

/// CSIStorageCapacityList is a collection of CSIStorageCapacity objects.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSIStorageCapacityList {
    /// TypeMeta for this resource
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard list metadata
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// items is the list of CSIStorageCapacity objects.
    #[serde(default)]
    pub items: Vec<CSIStorageCapacity>,
}

// ============================================================================
// Trait Implementations for CSIStorageCapacity and CSIStorageCapacityList
// ============================================================================

// ----------------------------------------------------------------------------
// ResourceSchema Implementation
// ----------------------------------------------------------------------------

impl ResourceSchema for CSIStorageCapacity {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "storage.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "CSIStorageCapacity"
    }
    fn resource(_: &Self::Meta) -> &str {
        "csistoragecapacities"
    }

    fn group_static() -> &'static str {
        "storage.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "CSIStorageCapacity"
    }
    fn resource_static() -> &'static str {
        "csistoragecapacities"
    }
}

impl ResourceSchema for CSIStorageCapacityList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "storage.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "CSIStorageCapacityList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "csistoragecapacities"
    }

    fn group_static() -> &'static str {
        "storage.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "CSIStorageCapacityList"
    }
    fn resource_static() -> &'static str {
        "csistoragecapacities"
    }
}

// ----------------------------------------------------------------------------
// HasTypeMeta Implementation
// ----------------------------------------------------------------------------

impl HasTypeMeta for CSIStorageCapacity {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for CSIStorageCapacityList {
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

impl VersionedObject for CSIStorageCapacity {
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

// Note: CSIStorageCapacityList does not implement VersionedObject because its metadata is ListMeta

// ----------------------------------------------------------------------------
// ApplyDefaults Implementation
// ----------------------------------------------------------------------------

impl ApplyDefault for CSIStorageCapacity {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "storage.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "CSIStorageCapacity".to_string();
        }
    }
}

impl ApplyDefault for CSIStorageCapacityList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "storage.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "CSIStorageCapacityList".to_string();
        }
        for item in &mut self.items {
            item.apply_default();
        }
    }
}

// ----------------------------------------------------------------------------
// Protobuf Placeholder (using macro)
// ----------------------------------------------------------------------------

impl_unimplemented_prost_message!(CSIStorageCapacity);
impl_unimplemented_prost_message!(CSIStorageCapacityList);

#[cfg(test)]
mod tests {}
