//! ServiceCIDR internal types
use crate::common::{ObjectMeta, TypeMeta};
use crate::impl_has_object_meta;
use serde::{Deserialize, Serialize};

/// ServiceCIDR defines a range of IP addresses.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct ServiceCIDR {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ObjectMeta,
    pub spec: ServiceCIDRSpec,
}
impl_has_object_meta!(ServiceCIDR);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceCIDRSpec {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cidrs: Vec<String>,
}

impl crate::common::traits::ResourceSchema for ServiceCIDR {
    type Meta = ();
    fn group(_meta: &Self::Meta) -> &str {
        "networking.k8s.io"
    }
    fn version(_meta: &Self::Meta) -> &str {
        "v1alpha1"
    }
    fn kind(_meta: &Self::Meta) -> &str {
        "ServiceCIDR"
    }
    fn resource(_meta: &Self::Meta) -> &str {
        "servicecidrs"
    }
    fn group_static() -> &'static str {
        "networking.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1alpha1"
    }
    fn kind_static() -> &'static str {
        "ServiceCIDR"
    }
    fn resource_static() -> &'static str {
        "servicecidrs"
    }
}

impl crate::common::traits::HasTypeMeta for ServiceCIDR {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}
