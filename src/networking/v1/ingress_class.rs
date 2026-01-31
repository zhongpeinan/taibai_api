//! IngressClass types from the Kubernetes Networking API
//!
//! This module contains types for ingress class resources.
//!
//! Source: k8s.io/api/networking/v1/types.go

use crate::common::{ListMeta, ObjectMeta, TypeMeta};
use crate::impl_unimplemented_prost_message;
use crate::impl_versioned_object;
use serde::{Deserialize, Serialize};

// ============================================================================
// IngressClass
// ============================================================================

/// IngressClass represents the class of the Ingress.
///
/// Corresponds to [Kubernetes IngressClass](https://github.com/kubernetes/api/blob/master/networking/v1/types.go)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct IngressClass {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// spec is the desired state of the IngressClass.
    #[serde(default)]
    pub spec: IngressClassSpec,
}
impl_versioned_object!(IngressClass);

/// IngressClassList is a collection of IngressClass objects.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct IngressClassList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(default)]
    pub metadata: Option<ListMeta>,
    /// Items is the list of IngressClass objects.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<IngressClass>,
}

// ============================================================================
// IngressClassSpec
// ============================================================================

/// IngressClassSpec provides information about the class of an Ingress.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct IngressClassSpec {
    /// controller refers to the name of the controller that should handle this class.
    #[serde(default)]
    pub controller: String,
    /// parameters is a link to a custom resource containing additional configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<IngressClassParametersReference>,
}

// ============================================================================
// IngressClassParametersReference
// ============================================================================

/// IngressClassParametersReference identifies an API object.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct IngressClassParametersReference {
    /// apiGroup is the group for the resource being referenced.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_group: Option<String>,
    /// kind is the type of resource being referenced.
    #[serde(default)]
    pub kind: String,
    /// name is the name of resource being referenced.
    #[serde(default)]
    pub name: String,
    /// namespace is the namespace of the resource being referenced.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// scope represents if this refers to a cluster or namespace scoped resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

// ============================================================================
// Trait Implementations
// ============================================================================

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

    fn group_static() -> &'static str
    where
        Self::Meta: Default,
    {
        "networking.k8s.io"
    }

    fn version_static() -> &'static str
    where
        Self::Meta: Default,
    {
        "v1"
    }

    fn kind_static() -> &'static str
    where
        Self::Meta: Default,
    {
        "IngressClass"
    }

    fn resource_static() -> &'static str
    where
        Self::Meta: Default,
    {
        "ingressclasses"
    }
}

impl crate::common::traits::ResourceSchema for IngressClassList {
    type Meta = ();

    fn group(_meta: &Self::Meta) -> &str {
        "networking.k8s.io"
    }

    fn version(_meta: &Self::Meta) -> &str {
        "v1"
    }

    fn kind(_meta: &Self::Meta) -> &str {
        "IngressClassList"
    }

    fn resource(_meta: &Self::Meta) -> &str {
        "ingressclasses"
    }

    fn group_static() -> &'static str
    where
        Self::Meta: Default,
    {
        "networking.k8s.io"
    }

    fn version_static() -> &'static str
    where
        Self::Meta: Default,
    {
        "v1"
    }

    fn kind_static() -> &'static str
    where
        Self::Meta: Default,
    {
        "IngressClassList"
    }

    fn resource_static() -> &'static str
    where
        Self::Meta: Default,
    {
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

impl crate::common::traits::ApplyDefault for IngressClass {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "networking.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "IngressClass".to_string();
        }

        crate::networking::v1::defaults::set_defaults_ingress_class(self);
    }
}

impl crate::common::traits::ApplyDefault for IngressClassList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "networking.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "IngressClassList".to_string();
        }
    }
}

// Version Conversion - See conversion.rs module

// Protobuf Placeholder (using macro)
impl_unimplemented_prost_message!(IngressClass);
impl_unimplemented_prost_message!(IngressClassList);

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {}
