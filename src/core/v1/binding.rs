//! Binding types from the Kubernetes Core v1 API
//!
//! This module contains types for binding objects to other objects.

use crate::common::{
    ApplyDefault, HasTypeMeta, ObjectMeta, ResourceSchema, TypeMeta, UnimplementedConversion,
};
use crate::core::v1::reference::ObjectReference;
use crate::{impl_unimplemented_prost_message, impl_versioned_object};
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

// ============================================================================
// Trait Implementations for Binding
// ============================================================================

// ----------------------------------------------------------------------------
// ResourceSchema Implementation
// ----------------------------------------------------------------------------

impl ResourceSchema for Binding {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        ""
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "Binding"
    }
    fn resource(_: &Self::Meta) -> &str {
        "bindings"
    }

    fn group_static() -> &'static str {
        ""
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "Binding"
    }
    fn resource_static() -> &'static str {
        "bindings"
    }
}

// ----------------------------------------------------------------------------
// HasTypeMeta Implementation
// ----------------------------------------------------------------------------

impl HasTypeMeta for Binding {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

// Note: VersionedObject is implemented by impl_versioned_object! macro above

// ----------------------------------------------------------------------------
// ApplyDefaults Implementation
// ----------------------------------------------------------------------------

impl ApplyDefault for Binding {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "Binding".to_string();
        }
    }
}

// ----------------------------------------------------------------------------
// Version Conversion Placeholder
// ----------------------------------------------------------------------------

impl UnimplementedConversion for Binding {}

// ----------------------------------------------------------------------------
// Protobuf Placeholder
// ----------------------------------------------------------------------------

impl_unimplemented_prost_message!(Binding);

#[cfg(test)]
mod tests {}
