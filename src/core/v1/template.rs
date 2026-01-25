//! Kubernetes Pod Template types
//!
//! This module contains pod template-related types from the Kubernetes core/v1 API.
//! Pod templates are used to define pod specifications that can be reused by controllers
//! such as Deployments, StatefulSets, DaemonSets, and Jobs.

use crate::common::meta::{ListMeta, ObjectMeta};
use crate::common::{ApplyDefault, HasTypeMeta, ResourceSchema, TypeMeta, UnimplementedConversion};
use crate::core::v1::pod::PodSpec;
use crate::{impl_unimplemented_prost_message, impl_versioned_object};
use serde::{Deserialize, Serialize};

/// PodTemplate describes a template for creating copies of a predefined pod.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodTemplate {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// Template defines the pods that will be created from this pod template.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<PodTemplateSpec>,
}
impl_versioned_object!(PodTemplate);

/// PodTemplateSpec describes the data a pod should have when created from a template.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodTemplateSpec {
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// Specification of the desired behavior of the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<PodSpec>,
}
impl_versioned_object!(PodTemplateSpec);

/// PodTemplateList is a list of PodTemplates.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodTemplateList {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// List of pod templates.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<PodTemplate>,
}

// ============================================================================
// Trait Implementations for PodTemplate Resources
// ============================================================================

// ----------------------------------------------------------------------------
// ResourceSchema Implementation
// ----------------------------------------------------------------------------

impl ResourceSchema for PodTemplate {
    type Meta = ();
    fn group(_: &Self::Meta) -> &str {
        ""
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "PodTemplate"
    }
    fn resource(_: &Self::Meta) -> &str {
        "podtemplates"
    }
    fn group_static() -> &'static str {
        ""
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "PodTemplate"
    }
    fn resource_static() -> &'static str {
        "podtemplates"
    }
}

impl ResourceSchema for PodTemplateList {
    type Meta = ();
    fn group(_: &Self::Meta) -> &str {
        ""
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "PodTemplateList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "podtemplates"
    }
    fn group_static() -> &'static str {
        ""
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "PodTemplateList"
    }
    fn resource_static() -> &'static str {
        "podtemplates"
    }
}

// ----------------------------------------------------------------------------
// HasTypeMeta Implementation
// ----------------------------------------------------------------------------

impl HasTypeMeta for PodTemplate {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for PodTemplateList {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

// Note: VersionedObject for PodTemplate is implemented by impl_versioned_object! macro
// Note: PodTemplateList does not implement VersionedObject because its metadata is ListMeta

// ----------------------------------------------------------------------------
// ApplyDefaults Implementation
// ----------------------------------------------------------------------------

impl ApplyDefault for PodTemplate {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "PodTemplate".to_string();
        }
    }
}

impl ApplyDefault for PodTemplateList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "PodTemplateList".to_string();
        }
    }
}

// ----------------------------------------------------------------------------
// Version Conversion Placeholder
// ----------------------------------------------------------------------------

impl UnimplementedConversion for PodTemplate {}

// ----------------------------------------------------------------------------
// Protobuf Placeholder
// ----------------------------------------------------------------------------

impl_unimplemented_prost_message!(PodTemplate);
impl_unimplemented_prost_message!(PodTemplateList);

#[cfg(test)]
mod tests {}
