//! ResourceQuota and LimitRange types from the Kubernetes Core API
//!
//! This module contains types for resource constraints and limits,
//! including ResourceQuota and LimitRange.
//!
//! Source: k8s-pkg/apis/core/types.go

use crate::common::{ListMeta, ObjectMeta, TypeMeta};
use crate::core::internal::{LimitType, ResourceQuotaScope};
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// Spec defines the desired quota.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<ResourceQuotaSpec>,
    /// Status defines the actual enforced quota and its current usage.
    #[serde(default)]
    pub status: ResourceQuotaStatus,
}

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
}

pub mod scope_selector_operator {
    pub const IN: &str = "In";
    pub const NOT_IN: &str = "NotIn";
    pub const EXISTS: &str = "Exists";
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// Spec defines the limits enforced.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<LimitRangeSpec>,
}

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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<LimitRangeValue>,
    /// Max usage constraints on this kind.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<LimitRangeValue>,
    /// Default resource requirement.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<LimitRangeValue>,
    /// DefaultRequest resource requirement.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_request: Option<LimitRangeValue>,
    /// MaxLimitRequestRatio represents the max ratio.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_limit_request_ratio: Option<LimitRangeValue>,
}

/// LimitRangeValue defines the value of a limit range.
///
/// Corresponds to [Kubernetes Quantity](https://github.com/kubernetes/apimachinery/pkg/api/resource/Quantity.go)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct LimitRangeValue(pub String);

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
mod tests {
    use super::*;

    // ResourceQuota tests
    #[test]
    fn test_resource_quota_default() {
        let rq = ResourceQuota {
            type_meta: TypeMeta::default(),
            metadata: None,
            spec: None,
            status: ResourceQuotaStatus::default(),
        };
        assert!(rq.spec.is_none());
        assert!(rq.status.hard.is_empty());
    }

    #[test]
    fn test_resource_quota_spec_default() {
        let spec = ResourceQuotaSpec::default();
        assert!(spec.hard.is_empty());
        assert!(spec.scopes.is_empty());
    }

    #[test]
    fn test_resource_quota_spec_with_scopes() {
        let spec = ResourceQuotaSpec {
            scopes: vec![
                ResourceQuotaScope::Terminating,
                ResourceQuotaScope::NotTerminating,
            ],
            ..Default::default()
        };
        assert_eq!(spec.scopes.len(), 2);
    }

    #[test]
    fn test_resource_quota_status_serialize() {
        use crate::common::util::Quantity;
        let mut hard = crate::core::internal::ResourceList::default();
        hard.insert("cpu".to_string(), Quantity::new("10".to_string()));
        let mut used = crate::core::internal::ResourceList::default();
        used.insert("cpu".to_string(), Quantity::new("5".to_string()));

        let status = ResourceQuotaStatus { hard, used };
        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains(r#""hard""#));
        assert!(json.contains(r#""used""#));
    }

    #[test]
    fn test_resource_quota_list_default() {
        let list = ResourceQuotaList::default();
        assert!(list.items.is_empty());
    }

    // ScopeSelector tests
    #[test]
    fn test_scope_selector_default() {
        let selector = ScopeSelector::default();
        assert!(selector.match_expressions.is_empty());
    }

    #[test]
    fn test_scoped_resource_selector_requirement_default() {
        let req = ScopedResourceSelectorRequirement::default();
        assert!(req.scope_name == ResourceQuotaScope::BestEffort);
        assert!(req.values.is_empty());
    }

    #[test]
    fn test_scoped_resource_selector_requirement_with_operator() {
        let req = ScopedResourceSelectorRequirement {
            scope_name: ResourceQuotaScope::PriorityClass,
            operator: Some(ScopeSelectorOperator::In),
            values: vec!["high".to_string(), "medium".to_string()],
        };
        assert_eq!(req.scope_name, ResourceQuotaScope::PriorityClass);
        assert_eq!(req.operator, Some(ScopeSelectorOperator::In));
        assert_eq!(req.values.len(), 2);
    }

    #[test]
    fn test_scope_selector_operator_serialize() {
        let json = serde_json::to_string(&ScopeSelectorOperator::In).unwrap();
        assert_eq!(json, r#""In""#);

        let json = serde_json::to_string(&ScopeSelectorOperator::NotIn).unwrap();
        assert_eq!(json, r#""NotIn""#);
    }

    #[test]
    fn test_scope_selector_operator_deserialize() {
        let op: ScopeSelectorOperator = serde_json::from_str(r#""In""#).unwrap();
        assert_eq!(op, ScopeSelectorOperator::In);

        let op: ScopeSelectorOperator = serde_json::from_str(r#""Exists""#).unwrap();
        assert_eq!(op, ScopeSelectorOperator::Exists);
    }

    // LimitRange tests
    #[test]
    fn test_limit_range_default() {
        let lr = LimitRange {
            type_meta: TypeMeta::default(),
            metadata: None,
            spec: None,
        };
        assert!(lr.spec.is_none());
    }

    #[test]
    fn test_limit_range_spec_default() {
        let spec = LimitRangeSpec::default();
        assert!(spec.limits.is_empty());
    }

    #[test]
    fn test_limit_range_spec_with_limits() {
        let spec = LimitRangeSpec {
            limits: vec![LimitRangeItem {
                r#type: LimitType::Container,
                ..Default::default()
            }],
            ..Default::default()
        };
        assert_eq!(spec.limits.len(), 1);
    }

    #[test]
    fn test_limit_range_item_default() {
        let item = LimitRangeItem::default();
        assert_eq!(item.r#type, LimitType::Container);
    }

    #[test]
    fn test_limit_range_item_with_values() {
        let item = LimitRangeItem {
            r#type: LimitType::Pod,
            min: Some(LimitRangeValue("100Mi".to_string())),
            max: Some(LimitRangeValue("200Mi".to_string())),
            ..Default::default()
        };
        assert!(item.min.is_some());
        assert!(item.max.is_some());
    }

    #[test]
    fn test_limit_range_value_serialize() {
        let value = LimitRangeValue("100Mi".to_string());
        let json = serde_json::to_string(&value).unwrap();
        assert!(json.contains("100Mi"));
    }

    #[test]
    fn test_limit_range_list_default() {
        let list = LimitRangeList::default();
        assert!(list.items.is_empty());
    }

    // Integration tests
    #[test]
    fn test_resource_quota_with_hard_limits() {
        use crate::common::util::Quantity;
        let mut hard = crate::core::internal::ResourceList::default();
        hard.insert("cpu".to_string(), Quantity::new("10".to_string()));
        hard.insert("memory".to_string(), Quantity::new("20Gi".to_string()));

        let spec = ResourceQuotaSpec {
            hard,
            ..Default::default()
        };
        assert_eq!(spec.hard.len(), 2);
    }

    #[test]
    fn test_resource_quota_with_used() {
        use crate::common::util::Quantity;
        let mut hard = crate::core::internal::ResourceList::default();
        hard.insert("pods".to_string(), Quantity::new("10".to_string()));

        let mut used = crate::core::internal::ResourceList::default();
        used.insert("pods".to_string(), Quantity::new("5".to_string()));

        let status = ResourceQuotaStatus { hard, used };
        assert_eq!(status.hard.len(), 1);
        assert_eq!(status.used.len(), 1);
    }

    #[test]
    fn test_limit_range_item_with_all_fields() {
        let item = LimitRangeItem {
            r#type: LimitType::Container,
            min: Some(LimitRangeValue("10Mi".to_string())),
            max: Some(LimitRangeValue("1Gi".to_string())),
            default: Some(LimitRangeValue("128Mi".to_string())),
            default_request: Some(LimitRangeValue("64Mi".to_string())),
            max_limit_request_ratio: Some(LimitRangeValue("2".to_string())),
        };
        assert!(item.min.is_some());
        assert!(item.max.is_some());
        assert!(item.default.is_some());
        assert!(item.default_request.is_some());
        assert!(item.max_limit_request_ratio.is_some());
    }

    #[test]
    fn test_scope_selector_with_requirements() {
        let selector = ScopeSelector {
            match_expressions: vec![ScopedResourceSelectorRequirement {
                scope_name: ResourceQuotaScope::PriorityClass,
                operator: Some(ScopeSelectorOperator::In),
                values: vec!["high".to_string()],
            }],
        };
        assert_eq!(selector.match_expressions.len(), 1);
    }

    #[test]
    fn test_limit_range_list_with_items() {
        let list = LimitRangeList {
            items: vec![LimitRange {
                type_meta: TypeMeta {
                    kind: "LimitRange".to_string(),
                    api_version: "v1".to_string(),
                },
                ..Default::default()
            }],
            ..Default::default()
        };
        assert_eq!(list.items.len(), 1);
    }

    #[test]
    fn test_resource_quota_serialize() {
        let rq = ResourceQuota {
            type_meta: TypeMeta {
                kind: "ResourceQuota".to_string(),
                api_version: "v1".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("my-quota".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let json = serde_json::to_string(&rq).unwrap();
        assert!(json.contains(r#""kind":"ResourceQuota""#));
        assert!(json.contains(r#""name":"my-quota""#));
    }

    #[test]
    fn test_limit_range_serialize() {
        let lr = LimitRange {
            type_meta: TypeMeta {
                kind: "LimitRange".to_string(),
                api_version: "v1".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("my-limits".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let json = serde_json::to_string(&lr).unwrap();
        assert!(json.contains(r#""kind":"LimitRange""#));
        assert!(json.contains(r#""name":"my-limits""#));
    }

    #[test]
    fn test_limit_range_item_round_trip() {
        let original = LimitRangeItem {
            r#type: LimitType::Pod,
            min: Some(LimitRangeValue("100Mi".to_string())),
            max: Some(LimitRangeValue("200Mi".to_string())),
            ..Default::default()
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: LimitRangeItem = serde_json::from_str(&json).unwrap();
        assert_eq!(original.r#type, deserialized.r#type);
    }

    #[test]
    fn test_scoped_resource_selector_requirement_serialize() {
        let req = ScopedResourceSelectorRequirement {
            scope_name: ResourceQuotaScope::BestEffort,
            operator: Some(ScopeSelectorOperator::Exists),
            values: vec![],
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains(r#""scopeName":"BestEffort""#));
        assert!(json.contains(r#""operator":"Exists""#));
    }

    #[test]
    fn test_scope_selector_serialize() {
        let selector = ScopeSelector {
            match_expressions: vec![ScopedResourceSelectorRequirement {
                scope_name: ResourceQuotaScope::PriorityClass,
                operator: Some(ScopeSelectorOperator::In),
                values: vec!["high".to_string()],
            }],
        };
        let json = serde_json::to_string(&selector).unwrap();
        assert!(json.contains(r#""matchExpressions""#));
    }

    #[test]
    fn test_limit_range_value_default() {
        let value = LimitRangeValue::default();
        assert!(value.0.is_empty());
    }

    #[test]
    fn test_limit_range_value_with_content() {
        let value = LimitRangeValue("1Gi".to_string());
        assert_eq!(value.0, "1Gi");
    }

    #[test]
    fn test_resource_quota_list_with_items() {
        let list = ResourceQuotaList {
            items: vec![ResourceQuota {
                type_meta: TypeMeta {
                    kind: "ResourceQuota".to_string(),
                    api_version: "v1".to_string(),
                },
                ..Default::default()
            }],
            ..Default::default()
        };
        assert_eq!(list.items.len(), 1);
    }
}
