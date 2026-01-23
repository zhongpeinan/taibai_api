//! Kubernetes Pod Template types
//!
//! This module contains pod template-related types from the Kubernetes core/v1 API.
//! Pod templates are used to define pod specifications that can be reused by controllers
//! such as Deployments, StatefulSets, DaemonSets, and Jobs.

use crate::common::TypeMeta;
use crate::common::meta::{ListMeta, ObjectMeta};
use crate::impl_versioned_object;
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
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// Specification of the desired behavior of the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<serde_json::Value>,
}
impl_versioned_object!(PodTemplateSpec);

/// PodTemplateList is a list of PodTemplates.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodTemplateList {
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,
    /// List of pod templates.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<PodTemplate>,
}

#[cfg(test)]
mod tests {
}
