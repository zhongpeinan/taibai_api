//! IPAddress internal types
use crate::common::{ObjectMeta, TypeMeta};
use crate::impl_has_object_meta;
use serde::{Deserialize, Serialize};

/// IPAddress represents a single IP address.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct IPAddress {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ObjectMeta,
    pub spec: IPAddressSpec,
}
impl_has_object_meta!(IPAddress);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct IPAddressSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_ref: Option<String>,
}

impl crate::common::traits::ResourceSchema for IPAddress {
    type Meta = ();
    fn group(_meta: &Self::Meta) -> &str {
        "networking.k8s.io"
    }
    fn version(_meta: &Self::Meta) -> &str {
        "v1alpha1"
    }
    fn kind(_meta: &Self::Meta) -> &str {
        "IPAddress"
    }
    fn resource(_meta: &Self::Meta) -> &str {
        "ipaddresses"
    }
    fn group_static() -> &'static str {
        "networking.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1alpha1"
    }
    fn kind_static() -> &'static str {
        "IPAddress"
    }
    fn resource_static() -> &'static str {
        "ipaddresses"
    }
}

impl crate::common::traits::HasTypeMeta for IPAddress {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}
