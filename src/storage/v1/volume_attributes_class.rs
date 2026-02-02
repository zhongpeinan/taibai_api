//! VolumeAttributesClass types
//!
//! VolumeAttributesClass represents a specification of mutable volume attributes
//! defined by the CSI driver.
//!
//! Corresponds to [Kubernetes VolumeAttributesClass](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/storage/types.go#L752)

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::OnceLock;

use crate::common::{
    ApplyDefault, HasTypeMeta, ListMeta, ObjectMeta, ResourceSchema, TypeMeta, VersionedObject,
};
use crate::impl_unimplemented_prost_message;

/// VolumeAttributesClass represents a specification of mutable volume attributes
/// defined by the CSI driver.
///
/// The class can be specified during dynamic provisioning of PersistentVolumeClaims,
/// and changed in the PersistentVolumeClaim spec after provisioning.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct VolumeAttributesClass {
    /// TypeMeta for this resource
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// Name of the CSI driver
    pub driver_name: String,

    /// parameters hold volume attributes defined by the CSI driver.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub parameters: BTreeMap<String, String>,
}

/// VolumeAttributesClassList is a collection of VolumeAttributesClass objects.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct VolumeAttributesClassList {
    /// TypeMeta for this resource
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard list metadata
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// items is the list of VolumeAttributesClass objects.
    #[serde(default)]
    pub items: Vec<VolumeAttributesClass>,
}

// ============================================================================
// Trait Implementations for VolumeAttributesClass and VolumeAttributesClassList
// ============================================================================

// ----------------------------------------------------------------------------
// ResourceSchema Implementation
// ----------------------------------------------------------------------------

impl ResourceSchema for VolumeAttributesClass {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "storage.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "VolumeAttributesClass"
    }
    fn resource(_: &Self::Meta) -> &str {
        "volumeattributesclasses"
    }

    fn group_static() -> &'static str {
        "storage.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "VolumeAttributesClass"
    }
    fn resource_static() -> &'static str {
        "volumeattributesclasses"
    }
}

impl ResourceSchema for VolumeAttributesClassList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "storage.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "VolumeAttributesClassList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "volumeattributesclasses"
    }

    fn group_static() -> &'static str {
        "storage.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "VolumeAttributesClassList"
    }
    fn resource_static() -> &'static str {
        "volumeattributesclasses"
    }
}

// ----------------------------------------------------------------------------
// HasTypeMeta Implementation
// ----------------------------------------------------------------------------

impl HasTypeMeta for VolumeAttributesClass {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for VolumeAttributesClassList {
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

impl VersionedObject for VolumeAttributesClass {
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

// Note: VolumeAttributesClassList does not implement VersionedObject because its metadata is ListMeta

// ----------------------------------------------------------------------------
// ApplyDefaults Implementation
// ----------------------------------------------------------------------------

impl ApplyDefault for VolumeAttributesClass {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "storage.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "VolumeAttributesClass".to_string();
        }
    }
}

impl ApplyDefault for VolumeAttributesClassList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "storage.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "VolumeAttributesClassList".to_string();
        }
        for item in &mut self.items {
            item.apply_default();
        }
    }
}

// ----------------------------------------------------------------------------
// Protobuf Placeholder (using macro)
// ----------------------------------------------------------------------------

impl_unimplemented_prost_message!(VolumeAttributesClass);
impl_unimplemented_prost_message!(VolumeAttributesClassList);

#[cfg(test)]
mod tests {}
