//! DeviceClass internal types
use crate::common::{ObjectMeta, TypeMeta};
use crate::impl_has_object_meta;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DeviceClass {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ObjectMeta,
    #[serde(default)]
    pub spec: DeviceClassSpec,
}
impl_has_object_meta!(DeviceClass);

impl Default for DeviceClass {
    fn default() -> Self {
        Self {
            type_meta: TypeMeta::default(),
            metadata: ObjectMeta::default(),
            spec: DeviceClassSpec::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceClassSpec {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub selectors: Vec<String>,
}

impl crate::common::traits::ResourceSchema for DeviceClass {
    type Meta = ();
    fn group(_meta: &Self::Meta) -> &str { "resource.k8s.io" }
    fn version(_meta: &Self::Meta) -> &str { "v1" }
    fn kind(_meta: &Self::Meta) -> &str { "DeviceClass" }
    fn resource(_meta: &Self::Meta) -> &str { "deviceclasses" }
    fn group_static() -> &'static str { "resource.k8s.io" }
    fn version_static() -> &'static str { "v1" }
    fn kind_static() -> &'static str { "DeviceClass" }
    fn resource_static() -> &'static str { "deviceclasses" }
}

impl crate::common::traits::HasTypeMeta for DeviceClass {
    fn type_meta(&self) -> &TypeMeta { &self.type_meta }
    fn type_meta_mut(&mut self) -> &mut TypeMeta { &mut self.type_meta }
}
