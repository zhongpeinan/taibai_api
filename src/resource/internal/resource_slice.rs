//! ResourceSlice internal types
use crate::common::{ObjectMeta, TypeMeta};
use crate::impl_has_object_meta;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct ResourceSlice {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ObjectMeta,
    #[serde(default)]
    pub spec: ResourceSliceSpec,
}
impl_has_object_meta!(ResourceSlice);


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceSliceSpec {
    #[serde(default)]
    pub driver: String,
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

impl crate::common::traits::HasTypeMeta for ResourceSlice {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}
