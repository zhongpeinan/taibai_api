//! DeviceClass types from the Kubernetes Resource API
//!
//! Source: k8s.io/api/resource/v1/types.go

use crate::common::{ListMeta, ObjectMeta, TypeMeta};
use crate::impl_versioned_object;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DeviceClass {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    #[serde(default)]
    pub spec: DeviceClassSpec,
}
impl_versioned_object!(DeviceClass);

impl Default for DeviceClass {
    fn default() -> Self {
        Self {
            type_meta: TypeMeta::default(),
            metadata: None,
            spec: DeviceClassSpec::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceClassList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: Option<ListMeta>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<DeviceClass>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceClassSpec {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub selectors: Vec<DeviceSelector>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub config: Vec<DeviceClassConfiguration>,
    /// ExtendedResourceName is the extended resource name for the devices of this class.
    /// The devices of this class can be used to satisfy a pod's extended resource requests.
    /// This is an alpha field (feature gate: DRAExtendedResource).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extended_resource_name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceSelector {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cel: Option<CELDeviceSelector>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CELDeviceSelector {
    #[serde(default)]
    pub expression: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceClassConfiguration {
    #[serde(default)]
    pub opaque: OpaqueDeviceConfiguration,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct OpaqueDeviceConfiguration {
    #[serde(default)]
    pub driver: String,
}

impl crate::common::traits::ResourceSchema for DeviceClass {
    type Meta = ();
    fn group(_meta: &Self::Meta) -> &str {
        "resource.k8s.io"
    }
    fn version(_meta: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_meta: &Self::Meta) -> &str {
        "DeviceClass"
    }
    fn resource(_meta: &Self::Meta) -> &str {
        "deviceclasses"
    }
    fn group_static() -> &'static str {
        "resource.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "DeviceClass"
    }
    fn resource_static() -> &'static str {
        "deviceclasses"
    }
}

impl crate::common::traits::ResourceSchema for DeviceClassList {
    type Meta = ();
    fn group(_meta: &Self::Meta) -> &str {
        "resource.k8s.io"
    }
    fn version(_meta: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_meta: &Self::Meta) -> &str {
        "DeviceClassList"
    }
    fn resource(_meta: &Self::Meta) -> &str {
        "deviceclasses"
    }
    fn group_static() -> &'static str {
        "resource.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "DeviceClassList"
    }
    fn resource_static() -> &'static str {
        "deviceclasses"
    }
}

impl crate::common::traits::HasTypeMeta for DeviceClass {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl crate::common::traits::ApplyDefault for DeviceClass {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "resource.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "DeviceClass".to_string();
        }
    }
}

impl crate::common::traits::ApplyDefault for DeviceClassList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "resource.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "DeviceClassList".to_string();
        }
    }
}

impl crate::common::traits::UnimplementedConversion for DeviceClass {}
impl crate::common::traits::UnimplementedConversion for DeviceClassList {}

#[cfg(test)]
mod tests {}
