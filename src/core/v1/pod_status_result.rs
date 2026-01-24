//! PodStatusResult types from the Kubernetes Core API
//!
//! This module contains types for pod status result resources.
//!
//! Source: k8s.io/api/core/v1/types.go

use crate::common::{ObjectMeta, TypeMeta};
use crate::core::v1::PodStatus;
use crate::impl_versioned_object;
use serde::{Deserialize, Serialize};

// ============================================================================
// PodStatusResult
// ============================================================================

/// PodStatusResult represents the status of a pod.
///
/// Corresponds to [Kubernetes PodStatusResult](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L5357)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct PodStatusResult {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// Most recently observed status of the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PodStatus>,
}
impl_versioned_object!(PodStatusResult);


// ============================================================================
// Trait Implementations
// ============================================================================

impl crate::common::traits::ResourceSchema for PodStatusResult {
    type Meta = ();

    fn group(_meta: &Self::Meta) -> &str {
        ""
    }

    fn version(_meta: &Self::Meta) -> &str {
        "v1"
    }

    fn kind(_meta: &Self::Meta) -> &str {
        "PodStatusResult"
    }

    fn resource(_meta: &Self::Meta) -> &str {
        "pods/status"
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
        "PodStatusResult"
    }

    fn resource_static() -> &'static str
    where
        Self::Meta: Default,
    {
        "pods/status"
    }
}

impl crate::common::traits::HasTypeMeta for PodStatusResult {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }

    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl crate::common::traits::ApplyDefault for PodStatusResult {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "PodStatusResult".to_string();
        }
    }
}

impl crate::common::traits::UnimplementedConversion for PodStatusResult {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {}
