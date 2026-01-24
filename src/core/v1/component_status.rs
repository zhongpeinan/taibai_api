//! ComponentStatus types from the Kubernetes Core API
//!
//! This module contains types for component status resources.
//!
//! Source: k8s.io/api/core/v1/types.go

use crate::common::{ListMeta, ObjectMeta, TypeMeta};
use crate::impl_versioned_object;
use serde::{Deserialize, Serialize};

// Re-export for public use
pub use crate::core::internal::{ComponentCondition, ComponentConditionType};

// ============================================================================
// ComponentStatus
// ============================================================================

/// ComponentStatus (and ComponentStatusList) holds the cluster validation info.
///
/// Deprecated: This API is deprecated in v1.19+
///
/// Corresponds to [Kubernetes ComponentStatus](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L8018)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct ComponentStatus {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// List of component conditions observed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<ComponentCondition>,
}
impl_versioned_object!(ComponentStatus);

/// ComponentStatusList is a list of ComponentStatus objects.
///
/// Corresponds to [Kubernetes ComponentStatusList](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L8039)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComponentStatusList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(default)]
    pub metadata: Option<ListMeta>,
    /// List of ComponentStatus objects.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ComponentStatus>,
}

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

impl crate::common::traits::ResourceSchema for ComponentStatusList {
    type Meta = ();

    fn group(_meta: &Self::Meta) -> &str {
        ""
    }

    fn version(_meta: &Self::Meta) -> &str {
        "v1"
    }

    fn kind(_meta: &Self::Meta) -> &str {
        "ComponentStatusList"
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
        "ComponentStatusList"
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

impl crate::common::traits::ApplyDefault for ComponentStatus {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "ComponentStatus".to_string();
        }
    }
}

impl crate::common::traits::ApplyDefault for ComponentStatusList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "ComponentStatusList".to_string();
        }
    }
}

impl crate::common::traits::UnimplementedConversion for ComponentStatus {}
impl crate::common::traits::UnimplementedConversion for ComponentStatusList {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {}
