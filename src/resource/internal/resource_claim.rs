//! ResourceClaim internal types
use crate::common::{ObjectMeta, TypeMeta};
use crate::impl_has_object_meta;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ResourceClaim {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ObjectMeta,
    #[serde(default)]
    pub spec: ResourceClaimSpec,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ResourceClaimStatus>,
}
impl_has_object_meta!(ResourceClaim);

impl Default for ResourceClaim {
    fn default() -> Self {
        Self {
            type_meta: TypeMeta::default(),
            metadata: ObjectMeta::default(),
            spec: ResourceClaimSpec::default(),
            status: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceClaimSpec {
    #[serde(default)]
    pub devices: DeviceClaim,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceClaim {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requests: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceClaimStatus {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reserved_for: Vec<String>,
}

impl crate::common::traits::ResourceSchema for ResourceClaim {
    type Meta = ();
    fn group(_meta: &Self::Meta) -> &str { "resource.k8s.io" }
    fn version(_meta: &Self::Meta) -> &str { "v1" }
    fn kind(_meta: &Self::Meta) -> &str { "ResourceClaim" }
    fn resource(_meta: &Self::Meta) -> &str { "resourceclaims" }
    fn group_static() -> &'static str { "resource.k8s.io" }
    fn version_static() -> &'static str { "v1" }
    fn kind_static() -> &'static str { "ResourceClaim" }
    fn resource_static() -> &'static str { "resourceclaims" }
}

impl crate::common::traits::HasTypeMeta for ResourceClaim {
    fn type_meta(&self) -> &TypeMeta { &self.type_meta }
    fn type_meta_mut(&mut self) -> &mut TypeMeta { &mut self.type_meta }
}
