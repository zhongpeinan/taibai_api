//! Binding types from the Kubernetes Core v1 API
//!
//! This module contains types for binding objects to other objects.

use crate::common::{ObjectMeta, TypeMeta};
use crate::core::v1::reference::ObjectReference;
use crate::impl_versioned_object;
use serde::{Deserialize, Serialize};

/// Binding binds one object to another.
///
/// For example, a pod is bound to a node by a Binding.
///
/// Corresponds to [Kubernetes Binding](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7159)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Binding {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// The target object that you want to bind to the standard object.
    pub target: ObjectReference,
}

impl_versioned_object!(Binding);

/// Preconditions must be fulfilled before an operation (update, delete, etc.) is carried out.
///
/// Corresponds to [Kubernetes Preconditions](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7172)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Preconditions {
    /// Specifies the target UID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

#[cfg(test)]
mod tests {
}
