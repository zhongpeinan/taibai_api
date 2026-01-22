//! IngressClass internal types
use crate::common::{ObjectMeta, TypeMeta};
use crate::impl_has_object_meta;
use serde::{Deserialize, Serialize};

/// IngressClass represents the class of the Ingress.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct IngressClass {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ObjectMeta,
    pub spec: IngressClassSpec,
}
impl_has_object_meta!(IngressClass);

impl Default for IngressClass {
    fn default() -> Self {
        Self {
            type_meta: TypeMeta::default(),
            metadata: ObjectMeta::default(),
            spec: IngressClassSpec::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct IngressClassSpec {
    #[serde(default)]
    pub controller: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<IngressClassParametersReference>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct IngressClassParametersReference {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub api_group: String,
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub namespace: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub scope: String,
}

impl crate::common::traits::ResourceSchema for IngressClass {
    type Meta = ();
    fn group(_meta: &Self::Meta) -> &str {
        "networking.k8s.io"
    }
    fn version(_meta: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_meta: &Self::Meta) -> &str {
        "IngressClass"
    }
    fn resource(_meta: &Self::Meta) -> &str {
        "ingressclasses"
    }
    fn group_static() -> &'static str {
        "networking.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "IngressClass"
    }
    fn resource_static() -> &'static str {
        "ingressclasses"
    }
}

impl crate::common::traits::HasTypeMeta for IngressClass {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}
