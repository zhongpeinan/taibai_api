//! DeviceClass internal types
use crate::common::{ObjectMeta, TypeMeta};
use crate::impl_has_object_meta;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct DeviceClass {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ObjectMeta,
    #[serde(default)]
    pub spec: DeviceClassSpec,
}
impl_has_object_meta!(DeviceClass);

/// DeviceClassSpec is used in a DeviceClass to define what can be allocated
/// and how to configure it.
///
/// Source: k8s/pkg/apis/resource/types.go
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceClassSpec {
    /// Each selector must be satisfied by a device which is claimed via this class.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub selectors: Vec<DeviceSelector>,
    /// Config defines configuration parameters that apply to each device that is claimed via this class.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub config: Vec<DeviceClassConfiguration>,
    /// ExtendedResourceName is the extended resource name for the devices of this class.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extended_resource_name: Option<String>,
}

/// DeviceSelector must have exactly one field set.
///
/// Source: k8s/pkg/apis/resource/types.go
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceSelector {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cel: Option<CELDeviceSelector>,
}

/// CELDeviceSelector contains a CEL expression for selecting a device.
///
/// Source: k8s/pkg/apis/resource/types.go
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CELDeviceSelector {
    #[serde(default)]
    pub expression: String,
}

/// DeviceClassConfiguration is used in DeviceClass.
///
/// Source: k8s/pkg/apis/resource/types.go
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceClassConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opaque: Option<OpaqueDeviceConfiguration>,
}

/// OpaqueDeviceConfiguration contains configuration parameters for a driver
/// in a format defined by the driver vendor.
///
/// Source: k8s/pkg/apis/resource/types.go
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct OpaqueDeviceConfiguration {
    #[serde(default)]
    pub driver: String,
    /// Parameters can contain arbitrary data.
    /// Note: Using serde_json::Value as a substitute for runtime.RawExtension
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
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

impl crate::common::traits::HasTypeMeta for DeviceClass {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}
