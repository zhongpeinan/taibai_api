//! Binding types from the Kubernetes Core API
//!
//! This module contains types for resource bindings and secret references.

use crate::common::ObjectMeta;
use crate::core::v1::reference::ObjectReference;
use crate::impl_has_object_meta;
use serde::{Deserialize, Serialize};

/// Binding binds an object to a target.
///
/// Corresponds to [Kubernetes Binding](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5901)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Binding {
    /// Standard object metadata.
    pub metadata: ObjectMeta,
    /// The target object that the object should be bound to.
    pub target: ObjectReference,
}
impl_has_object_meta!(Binding);

/// SecretReference represents a secret reference.
///
/// Corresponds to [Kubernetes SecretReference](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L1429)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SecretReference {
    /// Name is unique within a namespace to reference a secret resource.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// Namespace defines the space within which the secret name must be unique.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub namespace: String,
}

#[cfg(test)]
mod tests {}
