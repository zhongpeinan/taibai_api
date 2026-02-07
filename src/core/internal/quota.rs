//! ResourceQuota and LimitRange types from the Kubernetes Core API
//!
//! This module contains types for resource constraints and limits,
//! including ResourceQuota and LimitRange.
//!
//! Source: k8s-pkg/apis/core/types.go

use crate::common::{ListMeta, ObjectMeta, TypeMeta};
use crate::core::internal::{LimitType, ResourceList, ResourceQuotaScope};
use crate::impl_has_object_meta;
use serde::{Deserialize, Serialize};

// ============================================================================
// ResourceQuota
// ============================================================================

/// ResourceQuota sets aggregate quota restrictions enforced per namespace.
///
/// Corresponds to [Kubernetes ResourceQuota](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L6095)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceQuota {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ObjectMeta,
    /// Spec defines the desired quota.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<ResourceQuotaSpec>,
    /// Status defines the actual enforced quota and its current usage.
    #[serde(default)]
    pub status: ResourceQuotaStatus,
}
impl_has_object_meta!(ResourceQuota);

/// ResourceQuotaSpec defines the desired quota.
///
/// Corresponds to [Kubernetes ResourceQuotaSpec](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L6106)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceQuotaSpec {
    /// Hard is the set of desired hard limits for each named resource.
    #[serde(
        default,
        skip_serializing_if = "crate::core::internal::ResourceList::is_empty"
    )]
    pub hard: crate::core::internal::ResourceList,
    /// Scoped is a collection of ResourceQuotaScope which must match for the quota to be applied.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<ResourceQuotaScope>,
    /// ScopeSelector is the selector for the scope of the quota.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope_selector: Option<ScopeSelector>,
}

/// ResourceQuotaStatus defines the actual enforced quota and its current usage.
///
/// Corresponds to [Kubernetes ResourceQuotaStatus](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L6119)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceQuotaStatus {
    /// Hard is the set of enforced hard limits for each named resource.
    #[serde(
        default,
        skip_serializing_if = "crate::core::internal::ResourceList::is_empty"
    )]
    pub hard: crate::core::internal::ResourceList,
    /// Used is the current observed total usage of the resource.
    #[serde(
        default,
        skip_serializing_if = "crate::core::internal::ResourceList::is_empty"
    )]
    pub used: crate::core::internal::ResourceList,
}

/// ResourceQuotaList is a list of ResourceQuota items.
///
/// Corresponds to [Kubernetes ResourceQuotaList](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L6133)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceQuotaList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    /// Items is a list of ResourceQuota objects.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ResourceQuota>,
}

// ============================================================================
// ScopeSelector
// ============================================================================

/// ScopeSelector is a selector for scopes.
///
/// Corresponds to [Kubernetes ScopeSelector](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L6145)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScopeSelector {
    /// MatchExpressions is a list of scope selector requirements.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub match_expressions: Vec<ScopedResourceSelectorRequirement>,
}

/// ScopedResourceSelectorRequirement is a selector that contains requirements.
///
/// Corresponds to [Kubernetes ScopedResourceSelectorRequirement](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L6152)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScopedResourceSelectorRequirement {
    /// The name of the scope.
    #[serde(default)]
    pub scope_name: ResourceQuotaScope,
    /// Operator represents a scope's relationship to a set of values.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<ScopeSelectorOperator>,
    /// The list of values that the selector evaluates against.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
}

/// ScopeSelectorOperator is the operator for a scope selector.
///
/// Corresponds to [Kubernetes ScopeSelectorOperator](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L6162)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum ScopeSelectorOperator {
    /// Equal (exact match).
    #[serde(rename = "In")]
    In,
    /// Not equal.
    #[serde(rename = "NotIn")]
    NotIn,
    /// Exists.
    #[serde(rename = "Exists")]
    #[default]
    Exists,
    /// Does not exist.
    #[serde(rename = "DoesNotExist")]
    DoesNotExist,
}

pub mod scope_selector_operator {
    pub const IN: &str = "In";
    pub const NOT_IN: &str = "NotIn";
    pub const EXISTS: &str = "Exists";
    pub const DOES_NOT_EXIST: &str = "DoesNotExist";
}

// ============================================================================
// LimitRange
// ============================================================================

/// LimitRange sets resource usage limits for each kind of resource.
///
/// Corresponds to [Kubernetes LimitRange](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L6182)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct LimitRange {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ObjectMeta,
    /// Spec defines the limits enforced.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<LimitRangeSpec>,
}
impl_has_object_meta!(LimitRange);

/// LimitRangeSpec defines a min/max usage for resources that match.
///
/// Corresponds to [Kubernetes LimitRangeSpec](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L6193)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct LimitRangeSpec {
    /// Limits is the list of LimitRangeItem objects that are enforced.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub limits: Vec<LimitRangeItem>,
}

/// LimitRangeItem defines a min/max usage limit for a resource.
///
/// Corresponds to [Kubernetes LimitRangeItem](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L6199)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct LimitRangeItem {
    /// Type of resource that this limit applies to.
    #[serde(default)]
    pub r#type: LimitType,
    /// Min usage constraints on this kind.
    #[serde(
        default,
        skip_serializing_if = "crate::core::internal::ResourceList::is_empty"
    )]
    pub min: ResourceList,
    /// Max usage constraints on this kind.
    #[serde(
        default,
        skip_serializing_if = "crate::core::internal::ResourceList::is_empty"
    )]
    pub max: ResourceList,
    /// Default resource requirement.
    #[serde(
        default,
        skip_serializing_if = "crate::core::internal::ResourceList::is_empty"
    )]
    pub default: ResourceList,
    /// DefaultRequest resource requirement.
    #[serde(
        default,
        skip_serializing_if = "crate::core::internal::ResourceList::is_empty"
    )]
    pub default_request: ResourceList,
    /// MaxLimitRequestRatio represents the max ratio.
    #[serde(
        default,
        skip_serializing_if = "crate::core::internal::ResourceList::is_empty"
    )]
    pub max_limit_request_ratio: ResourceList,
}

/// LimitRangeList is a list of LimitRange items.
///
/// Corresponds to [Kubernetes LimitRangeList](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L6225)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct LimitRangeList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: ListMeta,
    /// Items is a list of LimitRange objects.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<LimitRange>,
}

#[cfg(test)]
mod tests {}

// AsRefStr / AsRef<str> implementations for enums
crate::impl_as_str_ref!(ScopeSelectorOperator, {
    In => scope_selector_operator::IN,
    NotIn => scope_selector_operator::NOT_IN,
    Exists => scope_selector_operator::EXISTS,
    DoesNotExist => scope_selector_operator::DOES_NOT_EXIST,
});
