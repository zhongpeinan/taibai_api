//! ResourceSlice types from the Kubernetes Resource API
//!
//! Source: k8s.io/api/resource/v1/types.go

use crate::common::{ListMeta, ObjectMeta, TypeMeta};
use crate::impl_versioned_object;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ResourceSlice {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    #[serde(default)]
    pub spec: ResourceSliceSpec,
}
impl_versioned_object!(ResourceSlice);

impl Default for ResourceSlice {
    fn default() -> Self {
        Self {
            type_meta: TypeMeta::default(),
            metadata: None,
            spec: ResourceSliceSpec::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceSliceList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: Option<ListMeta>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ResourceSlice>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceSliceSpec {
    #[serde(default)]
    pub driver: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pool: Option<ResourcePool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub devices: Vec<Device>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourcePool {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub generation: i64,
    #[serde(default)]
    pub resource_slice_count: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    #[serde(default)]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub basic: Option<BasicDevice>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct BasicDevice {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attributes: Vec<DeviceAttribute>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capacity: Vec<DeviceCapacity>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceAttribute {
    #[serde(default)]
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceCapacity {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub value: String,
}

impl crate::common::traits::ResourceSchema for ResourceSlice {
    type Meta = ();
    fn group(_meta: &Self::Meta) -> &str {
        "resource.k8s.io"
    }
    fn version(_meta: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_meta: &Self::Meta) -> &str {
        "ResourceSlice"
    }
    fn resource(_meta: &Self::Meta) -> &str {
        "resourceslices"
    }
    fn group_static() -> &'static str {
        "resource.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "ResourceSlice"
    }
    fn resource_static() -> &'static str {
        "resourceslices"
    }
}

impl crate::common::traits::ResourceSchema for ResourceSliceList {
    type Meta = ();
    fn group(_meta: &Self::Meta) -> &str {
        "resource.k8s.io"
    }
    fn version(_meta: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_meta: &Self::Meta) -> &str {
        "ResourceSliceList"
    }
    fn resource(_meta: &Self::Meta) -> &str {
        "resourceslices"
    }
    fn group_static() -> &'static str {
        "resource.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "ResourceSliceList"
    }
    fn resource_static() -> &'static str {
        "resourceslices"
    }
}

impl crate::common::traits::HasTypeMeta for ResourceSlice {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl crate::common::traits::ApplyDefault for ResourceSlice {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "resource.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "ResourceSlice".to_string();
        }
    }
}

impl crate::common::traits::ApplyDefault for ResourceSliceList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "resource.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "ResourceSliceList".to_string();
        }
    }
}

impl crate::common::traits::UnimplementedConversion for ResourceSlice {}
impl crate::common::traits::UnimplementedConversion for ResourceSliceList {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_resource_slice_default() {
        let rs = ResourceSlice::default();
        assert!(rs.metadata.is_none());
    }
}
