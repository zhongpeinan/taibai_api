//! ResourceClaimTemplate internal types
use crate::common::{ObjectMeta, TypeMeta};
use crate::impl_has_object_meta;
use crate::resource::internal::resource_claim::ResourceClaimSpec;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ResourceClaimTemplate {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ObjectMeta,
    #[serde(default)]
    pub spec: ResourceClaimTemplateSpec,
}
impl_has_object_meta!(ResourceClaimTemplate);

impl Default for ResourceClaimTemplate {
    fn default() -> Self {
        Self {
            type_meta: TypeMeta::default(),
            metadata: ObjectMeta::default(),
            spec: ResourceClaimTemplateSpec::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceClaimTemplateSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    #[serde(default)]
    pub spec: ResourceClaimSpec,
}

impl crate::common::traits::ResourceSchema for ResourceClaimTemplate {
    type Meta = ();
    fn group(_meta: &Self::Meta) -> &str {
        "resource.k8s.io"
    }
    fn version(_meta: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_meta: &Self::Meta) -> &str {
        "ResourceClaimTemplate"
    }
    fn resource(_meta: &Self::Meta) -> &str {
        "resourceclaimtemplates"
    }
    fn group_static() -> &'static str {
        "resource.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "ResourceClaimTemplate"
    }
    fn resource_static() -> &'static str {
        "resourceclaimtemplates"
    }
}

impl crate::common::traits::HasTypeMeta for ResourceClaimTemplate {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}
