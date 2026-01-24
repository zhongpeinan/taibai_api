//! ComponentStatus types from the Kubernetes Core API (Internal)
//!
//! This module contains internal types for component status resources.
//!
//! Source: k8s.io/kubernetes/pkg/apis/core/types.go

use crate::common::{ObjectMeta, TypeMeta};
use crate::impl_has_object_meta;
use serde::{Deserialize, Serialize};

// Re-export ComponentCondition and ComponentConditionType from parent module
pub use super::{ComponentCondition, ComponentConditionType};

// ============================================================================
// ComponentStatus
// ============================================================================

/// ComponentStatus (and ComponentStatusList) holds the cluster validation info.
///
/// Deprecated: This API is deprecated in v1.19+
///
/// Corresponds to [Kubernetes ComponentStatus](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/core/types.go)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct ComponentStatus {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    pub metadata: ObjectMeta,
    /// List of component conditions observed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<ComponentCondition>,
}
impl_has_object_meta!(ComponentStatus);


// ============================================================================
// Trait Implementations
// ============================================================================

impl crate::common::traits::ResourceSchema for ComponentStatus {
    type Meta = ();

    fn group(_meta: &Self::Meta) -> &str {
        ""
    }

    fn version(_meta: &Self::Meta) -> &str {
        "v1"
    }

    fn kind(_meta: &Self::Meta) -> &str {
        "ComponentStatus"
    }

    fn resource(_meta: &Self::Meta) -> &str {
        "componentstatuses"
    }

    fn group_static() -> &'static str
    where
        Self::Meta: Default,
    {
        ""
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
        "ComponentStatus"
    }

    fn resource_static() -> &'static str
    where
        Self::Meta: Default,
    {
        "componentstatuses"
    }
}

impl crate::common::traits::HasTypeMeta for ComponentStatus {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }

    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {}
