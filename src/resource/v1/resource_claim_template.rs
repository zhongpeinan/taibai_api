//! ResourceClaimTemplate types from the Kubernetes Resource API
//!
//! Source: k8s.io/api/resource/v1/types.go

use crate::common::{ListMeta, ObjectMeta, TypeMeta};
use crate::impl_versioned_object;
use crate::resource::v1::ResourceClaimSpec;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ResourceClaimTemplate {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    #[serde(default)]
    pub spec: ResourceClaimTemplateSpec,
}
impl_versioned_object!(ResourceClaimTemplate);

impl Default for ResourceClaimTemplate {
    fn default() -> Self {
        Self {
            type_meta: TypeMeta::default(),
            metadata: None,
            spec: ResourceClaimTemplateSpec::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceClaimTemplateList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: Option<ListMeta>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ResourceClaimTemplate>,
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

impl crate::common::traits::ResourceSchema for ResourceClaimTemplateList {
    type Meta = ();
    fn group(_meta: &Self::Meta) -> &str {
        "resource.k8s.io"
    }
    fn version(_meta: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_meta: &Self::Meta) -> &str {
        "ResourceClaimTemplateList"
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
        "ResourceClaimTemplateList"
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

impl crate::common::traits::ApplyDefault for ResourceClaimTemplate {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "resource.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "ResourceClaimTemplate".to_string();
        }
    }
}

impl crate::common::traits::ApplyDefault for ResourceClaimTemplateList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "resource.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "ResourceClaimTemplateList".to_string();
        }
    }
}

impl crate::common::traits::UnimplementedConversion for ResourceClaimTemplate {}
impl crate::common::traits::UnimplementedConversion for ResourceClaimTemplateList {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_resource_claim_template_default() {
        let rct = ResourceClaimTemplate::default();
        assert!(rct.metadata.is_none());
    }
}
