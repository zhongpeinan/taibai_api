//! ResourceQuota and LimitRange types from the Kubernetes Core v1 API
//!
//! This module contains types for managing resource quotas and limits.

use crate::common::{ListMeta, ObjectMeta, Quantity};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// LimitRange Types
// ============================================================================

/// LimitType defines the type of resource that a limit applies to.
pub type LimitType = String;

/// LimitType constants
pub mod limit_type {

    /// Limit that applies to all pods in a namespace
    pub const POD: &str = "Pod";

    /// Limit that applies to all containers in a namespace
    pub const CONTAINER: &str = "Container";

    /// Limit that applies to all persistent volume claims in a namespace
    pub const PERSISTENT_VOLUME_CLAIM: &str = "PersistentVolumeClaim";
}

/// LimitRangeItem defines a run-time limit.
///
/// Corresponds to [Kubernetes LimitRangeItem](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7651)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct LimitRangeItem {
    /// Type of resource that this limit applies to.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub type_: LimitType,

    /// Max usage constraints on this kind by resource name.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub max: HashMap<String, Quantity>,

    /// Min usage constraints on this kind by resource name.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub min: HashMap<String, Quantity>,

    /// Default resource requirement limit value by resource name if resource limit is omitted.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub default: HashMap<String, Quantity>,

    /// DefaultRequest is the default resource requirement request value by resource name.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub default_request: HashMap<String, Quantity>,

    /// MaxLimitRequestRatio represents the max burst for the named resource.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub max_limit_request_ratio: HashMap<String, Quantity>,
}

/// LimitRangeSpec defines a min/max usage constraint for resources.
///
/// Corresponds to [Kubernetes LimitRangeSpec](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7672)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct LimitRangeSpec {
    /// Limits is the list of LimitRangeItem objects that are enforced.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub limits: Vec<LimitRangeItem>,
}

/// LimitRange defines constraints that limit resource consumption per Namespace.
///
/// Corresponds to [Kubernetes LimitRange](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7683)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LimitRange {
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// Spec defines the limits enforced.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<LimitRangeSpec>,
}

/// LimitRangeList is a list of LimitRange items.
///
/// Corresponds to [Kubernetes LimitRangeList](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7700)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LimitRangeList {
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// Items is a list of LimitRange objects.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<LimitRange>,
}

// ============================================================================
// ResourceQuota Types
// ============================================================================

/// ResourceQuotaScope defines the type of quota scope.
pub type ResourceQuotaScope = String;

/// ResourceQuotaScope constants
pub mod resource_quota_scope {

    /// Match all pod objects where spec.activeDeadlineSeconds >= 0
    pub const TERMINATING: &str = "Terminating";

    /// Match all pod objects where spec.activeDeadlineSeconds is nil
    pub const NOT_TERMINATING: &str = "NotTerminating";

    /// Match all pod objects that have best effort quality of service
    pub const BEST_EFFORT: &str = "BestEffort";

    /// Match all pod objects that do not have best effort quality of service
    pub const NOT_BEST_EFFORT: &str = "NotBestEffort";

    /// Match all pod objects that have priority class mentioned
    pub const PRIORITY_CLASS: &str = "PriorityClass";

    /// Match all pod objects that have cross-namespace pod (anti)affinity
    pub const CROSS_NAMESPACE_POD_AFFINITY: &str = "CrossNamespacePodAffinity";

    /// Match all pvc objects that have volume attributes class mentioned
    pub const VOLUME_ATTRIBUTES_CLASS: &str = "VolumeAttributesClass";
}

/// ScopeSelectorOperator defines the operator for scope selector.
pub type ScopeSelectorOperator = String;

/// ScopeSelectorOperator constants
pub mod scope_selector_operator {

    pub const IN: &str = "In";
    pub const NOT_IN: &str = "NotIn";
    pub const EXISTS: &str = "Exists";
    pub const DOES_NOT_EXIST: &str = "DoesNotExist";
}

/// ResourceName defines the name of a resource.
pub type ResourceName = String;

/// ResourceName constants
pub mod resource_name {

    /// CPU, in cores. (500m = .5 cores)
    pub const CPU: &str = "cpu";

    /// Memory, in bytes. (500Gi = 500GiB = 500 * 1024 * 1024 * 1024)
    pub const MEMORY: &str = "memory";

    /// Volume size, in bytes (e,g. 5Gi = 5GiB = 5 * 1024 * 1024 * 1024)
    pub const STORAGE: &str = "storage";

    /// Local ephemeral storage, in bytes
    pub const EPHEMERAL_STORAGE: &str = "ephemeral-storage";

    /// Default namespace prefix
    pub const DEFAULT_NAMESPACE_PREFIX: &str = "kubernetes.io/";

    /// Name prefix for huge page resources (alpha)
    pub const HUGE_PAGES_PREFIX: &str = "hugepages-";

    /// Name prefix for storage resource limits
    pub const ATTACHABLE_VOLUMES_PREFIX: &str = "attachable-volumes-";
}

/// ResourceList is a set of (resource name, quantity) pairs.
pub type ResourceList = HashMap<ResourceName, Quantity>;

/// ScopedResourceSelectorRequirement represents a scope selector requirement.
///
/// Corresponds to [Kubernetes ScopedResourceSelectorRequirement](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7813)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ScopedResourceSelectorRequirement {
    /// The name of the scope that the selector applies to.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub scope_name: String,

    /// Represents a scope's relationship to a set of values.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub operator: String,

    /// An array of string values.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
}

/// ScopeSelector is a collection of scope selectors.
///
/// Corresponds to [Kubernetes ScopeSelector](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7804)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScopeSelector {
    /// A list of scope selector requirements by scope of the resources.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub match_expressions: Vec<ScopedResourceSelectorRequirement>,
}

/// ResourceQuotaSpec defines the desired hard limits to enforce.
///
/// Corresponds to [Kubernetes ResourceQuotaSpec](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7784)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceQuotaSpec {
    /// Hard is the set of desired hard limits for each named resource.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub hard: ResourceList,

    /// A collection of filters that must match each object tracked by a quota.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<String>,

    /// ScopeSelector is also a collection of filters like scopes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope_selector: Option<ScopeSelector>,
}

/// ResourceQuotaStatus defines the actual enforced quota and current usage.
///
/// Corresponds to [Kubernetes ResourceQuotaStatus](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7841)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceQuotaStatus {
    /// Hard is the set of enforced hard limits for each named resource.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub hard: ResourceList,

    /// Used is the current observed total usage of the resource in the namespace.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub used: ResourceList,
}

/// ResourceQuota sets aggregate quota restrictions enforced per namespace.
///
/// Corresponds to [Kubernetes ResourceQuota](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7856)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ResourceQuota {
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// Spec defines the desired quota.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<ResourceQuotaSpec>,

    /// Status defines the actual enforced quota and its current usage.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ResourceQuotaStatus>,
}

/// ResourceQuotaList is a list of ResourceQuota items.
///
/// Corresponds to [Kubernetes ResourceQuotaList](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7878)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ResourceQuotaList {
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// Items is a list of ResourceQuota objects.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ResourceQuota>,
}

// ============================================================================
// Resource Requirements Types
// ============================================================================

/// ResourceClaim describes a request for resources in a Pod.
///
/// Corresponds to [Kubernetes ResourceClaim](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2881)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceClaim {
    /// Name must match the name of one entry in pod.spec.resourceClaims of
    /// the Pod where this field is used.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// Request is the name chosen for a request in the referenced claim.
    /// If empty, everything from the claim is made available.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub request: String,
}

/// ResourceRequirements describes the compute resource requirements.
///
/// Corresponds to [Kubernetes ResourceRequirements](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2833)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceRequirements {
    /// Limits describes the maximum amount of compute resources allowed.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub limits: ResourceList,
    /// Requests describes the minimum amount of compute resources required.
    /// If Requests is omitted for a container, it defaults to Limits if that is explicitly specified.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub requests: ResourceList,
    /// Claims lists the names of resources, defined in spec.resourceClaims,
    /// that are used by this container.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub claims: Vec<ResourceClaim>,
}

#[cfg(test)]
mod tests {
    use super::*;

    // LimitRange tests
    #[test]
    fn test_limit_range_item_default() {
        let item = LimitRangeItem::default();
        assert!(item.type_.is_empty());
        assert!(item.max.is_empty());
        assert!(item.min.is_empty());
    }

    #[test]
    fn test_limit_range_item_with_limits() {
        let mut max = HashMap::new();
        max.insert("cpu".to_string(), Quantity::from("2"));
        max.insert("memory".to_string(), Quantity::from("4Gi"));

        let item = LimitRangeItem {
            type_: limit_type::CONTAINER.to_string(),
            max,
            min: HashMap::new(),
            default: HashMap::new(),
            default_request: HashMap::new(),
            max_limit_request_ratio: HashMap::new(),
        };

        assert_eq!(item.type_, limit_type::CONTAINER);
        assert_eq!(item.max.len(), 2);
    }

    #[test]
    fn test_limit_range_item_serialize() {
        let mut default = HashMap::new();
        default.insert("cpu".to_string(), Quantity::from("500m"));
        default.insert("memory".to_string(), Quantity::from("512Mi"));

        let item = LimitRangeItem {
            type_: limit_type::CONTAINER.to_string(),
            max: HashMap::new(),
            min: HashMap::new(),
            default,
            default_request: HashMap::new(),
            max_limit_request_ratio: HashMap::new(),
        };

        let json = serde_json::to_string(&item).unwrap();
        assert!(json.contains(r#""type":"Container""#));
        assert!(json.contains(r#""default":{"#));
        assert!(json.contains(r#""cpu":"500m""#));
        assert!(json.contains(r#""memory":"512Mi""#));
    }

    #[test]
    fn test_limit_range_item_deserialize() {
        let json = r#"{
            "type": "Pod",
            "max": {"cpu": "4", "memory": "8Gi"},
            "min": {"cpu": "100m", "memory": "128Mi"}
        }"#;

        let item: LimitRangeItem = serde_json::from_str(json).unwrap();
        assert_eq!(item.type_, limit_type::POD);
        assert_eq!(item.max.len(), 2);
        assert_eq!(item.min.len(), 2);
    }

    #[test]
    fn test_limit_range_spec_default() {
        let spec = LimitRangeSpec::default();
        assert!(spec.limits.is_empty());
    }

    #[test]
    fn test_limit_range_spec_with_limits() {
        let item1 = LimitRangeItem {
            type_: limit_type::CONTAINER.to_string(),
            max: {
                let mut m = HashMap::new();
                m.insert("cpu".to_string(), Quantity::from("2"));
                m
            },
            ..Default::default()
        };

        let spec = LimitRangeSpec {
            limits: vec![item1],
        };

        assert_eq!(spec.limits.len(), 1);
    }

    #[test]
    fn test_limit_range_default() {
        let lr = LimitRange {
            metadata: None,
            spec: None,
        };
        assert!(lr.metadata.is_none());
        assert!(lr.spec.is_none());
    }

    #[test]
    fn test_limit_range_with_spec() {
        let spec = LimitRangeSpec {
            limits: vec![LimitRangeItem {
                type_: limit_type::CONTAINER.to_string(),
                ..Default::default()
            }],
        };

        let lr = LimitRange {
            metadata: Some(ObjectMeta {
                name: Some("my-limits".to_string()),
                ..Default::default()
            }),
            spec: Some(spec),
        };

        assert_eq!(
            lr.metadata.as_ref().unwrap().name,
            Some("my-limits".to_string())
        );
        assert!(lr.spec.is_some());
    }

    #[test]
    fn test_limit_range_serialize() {
        let lr = LimitRange {
            metadata: Some(ObjectMeta {
                name: Some("my-limits".to_string()),
                ..Default::default()
            }),
            spec: Some(LimitRangeSpec {
                limits: vec![LimitRangeItem {
                    type_: limit_type::CONTAINER.to_string(),
                    ..Default::default()
                }],
            }),
        };

        let json = serde_json::to_string(&lr).unwrap();
        assert!(json.contains(r#""name":"my-limits""#));
        assert!(json.contains(r#""type":"Container""#));
    }

    #[test]
    fn test_limit_range_deserialize() {
        let json = r#"{
            "metadata": {"name": "my-limits"},
            "spec": {
                "limits": [{
                    "type": "Container",
                    "max": {"cpu": "2"}
                }]
            }
        }"#;

        let lr: LimitRange = serde_json::from_str(json).unwrap();
        assert_eq!(
            lr.metadata.as_ref().unwrap().name,
            Some("my-limits".to_string())
        );
        assert!(lr.spec.is_some());
    }

    #[test]
    fn test_limit_range_list_empty() {
        let list = LimitRangeList {
            metadata: None,
            items: vec![],
        };
        assert!(list.items.is_empty());
    }

    #[test]
    fn test_limit_range_list_with_items() {
        let list = LimitRangeList {
            metadata: Some(ListMeta {
                resource_version: Some("12345".to_string()),
                ..Default::default()
            }),
            items: vec![LimitRange {
                metadata: Some(ObjectMeta {
                    name: Some("limits-1".to_string()),
                    ..Default::default()
                }),
                spec: None,
            }],
        };

        assert_eq!(list.items.len(), 1);
    }

    #[test]
    fn test_limit_range_round_trip() {
        let original = LimitRange {
            metadata: Some(ObjectMeta {
                name: Some("my-limits".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(LimitRangeSpec {
                limits: vec![LimitRangeItem {
                    type_: limit_type::CONTAINER.to_string(),
                    max: {
                        let mut m = HashMap::new();
                        m.insert("cpu".to_string(), Quantity::from("2"));
                        m
                    },
                    ..Default::default()
                }],
            }),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: LimitRange = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    // ResourceQuota tests
    #[test]
    fn test_scoped_resource_selector_requirement_default() {
        let req = ScopedResourceSelectorRequirement {
            scope_name: String::new(),
            operator: String::new(),
            values: vec![],
        };
        assert!(req.scope_name.is_empty());
        assert!(req.operator.is_empty());
        assert!(req.values.is_empty());
    }

    #[test]
    fn test_scoped_resource_selector_requirement_with_values() {
        let req = ScopedResourceSelectorRequirement {
            scope_name: resource_quota_scope::PRIORITY_CLASS.to_string(),
            operator: scope_selector_operator::IN.to_string(),
            values: vec!["high".to_string(), "medium".to_string()],
        };

        assert_eq!(req.scope_name, resource_quota_scope::PRIORITY_CLASS);
        assert_eq!(req.operator, scope_selector_operator::IN);
        assert_eq!(req.values.len(), 2);
    }

    #[test]
    fn test_scoped_resource_selector_requirement_serialize() {
        let req = ScopedResourceSelectorRequirement {
            scope_name: resource_quota_scope::BEST_EFFORT.to_string(),
            operator: scope_selector_operator::EXISTS.to_string(),
            values: vec![],
        };

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains(r#""scopeName":"BestEffort""#));
        assert!(json.contains(r#""operator":"Exists""#));
    }

    #[test]
    fn test_scoped_resource_selector_requirement_deserialize() {
        let json = r#"{
            "scopeName": "PriorityClass",
            "operator": "In",
            "values": ["high", "medium"]
        }"#;

        let req: ScopedResourceSelectorRequirement = serde_json::from_str(json).unwrap();
        assert_eq!(req.scope_name, resource_quota_scope::PRIORITY_CLASS);
        assert_eq!(req.operator, scope_selector_operator::IN);
        assert_eq!(req.values.len(), 2);
    }

    #[test]
    fn test_scope_selector_default() {
        let selector = ScopeSelector::default();
        assert!(selector.match_expressions.is_empty());
    }

    #[test]
    fn test_scope_selector_with_expressions() {
        let selector = ScopeSelector {
            match_expressions: vec![ScopedResourceSelectorRequirement {
                scope_name: resource_quota_scope::BEST_EFFORT.to_string(),
                operator: scope_selector_operator::EXISTS.to_string(),
                values: vec![],
            }],
        };

        assert_eq!(selector.match_expressions.len(), 1);
    }

    #[test]
    fn test_resource_quota_spec_default() {
        let spec = ResourceQuotaSpec::default();
        assert!(spec.hard.is_empty());
        assert!(spec.scopes.is_empty());
        assert!(spec.scope_selector.is_none());
    }

    #[test]
    fn test_resource_quota_spec_with_hard_limits() {
        let mut hard = HashMap::new();
        hard.insert("cpu".to_string(), Quantity::from("10"));
        hard.insert("memory".to_string(), Quantity::from("20Gi"));
        hard.insert("pods".to_string(), Quantity::from("5"));

        let spec = ResourceQuotaSpec {
            hard,
            scopes: vec![],
            scope_selector: None,
        };

        assert_eq!(spec.hard.len(), 3);
    }

    #[test]
    fn test_resource_quota_spec_with_scopes() {
        let spec = ResourceQuotaSpec {
            hard: HashMap::new(),
            scopes: vec![
                resource_quota_scope::BEST_EFFORT.to_string(),
                resource_quota_scope::NOT_BEST_EFFORT.to_string(),
            ],
            scope_selector: None,
        };

        assert_eq!(spec.scopes.len(), 2);
    }

    #[test]
    fn test_resource_quota_spec_serialize() {
        let mut hard = HashMap::new();
        hard.insert("cpu".to_string(), Quantity::from("10"));

        let spec = ResourceQuotaSpec {
            hard,
            scopes: vec![resource_quota_scope::TERMINATING.to_string()],
            scope_selector: None,
        };

        let json = serde_json::to_string(&spec).unwrap();
        assert!(json.contains(r#""hard":{"#));
        assert!(json.contains(r#""cpu":"10""#));
        assert!(json.contains(r#""scopes":["Terminating"]"#));
    }

    #[test]
    fn test_resource_quota_spec_deserialize() {
        let json = r#"{
            "hard": {"cpu": "10", "memory": "20Gi"},
            "scopes": ["BestEffort"]
        }"#;

        let spec: ResourceQuotaSpec = serde_json::from_str(json).unwrap();
        assert_eq!(spec.hard.len(), 2);
        assert_eq!(spec.scopes.len(), 1);
    }

    #[test]
    fn test_resource_quota_status_default() {
        let status = ResourceQuotaStatus::default();
        assert!(status.hard.is_empty());
        assert!(status.used.is_empty());
    }

    #[test]
    fn test_resource_quota_status_with_usage() {
        let mut hard = HashMap::new();
        hard.insert("cpu".to_string(), Quantity::from("10"));

        let mut used = HashMap::new();
        used.insert("cpu".to_string(), Quantity::from("5"));

        let status = ResourceQuotaStatus { hard, used };

        assert_eq!(status.hard.len(), 1);
        assert_eq!(status.used.len(), 1);
    }

    #[test]
    fn test_resource_quota_status_serialize() {
        let mut hard = HashMap::new();
        hard.insert("pods".to_string(), Quantity::from("10"));

        let mut used = HashMap::new();
        used.insert("pods".to_string(), Quantity::from("3"));

        let status = ResourceQuotaStatus { hard, used };

        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains(r#""hard":{"#));
        assert!(json.contains(r#""pods":"10""#));
        assert!(json.contains(r#""used":{"#));
        assert!(json.contains(r#""pods":"3""#));
    }

    #[test]
    fn test_resource_quota_default() {
        let quota = ResourceQuota {
            metadata: None,
            spec: None,
            status: None,
        };
        assert!(quota.metadata.is_none());
        assert!(quota.spec.is_none());
        assert!(quota.status.is_none());
    }

    #[test]
    fn test_resource_quota_with_spec_and_status() {
        let mut hard = HashMap::new();
        hard.insert("cpu".to_string(), Quantity::from("10"));

        let mut used = HashMap::new();
        used.insert("cpu".to_string(), Quantity::from("5"));

        let quota = ResourceQuota {
            metadata: Some(ObjectMeta {
                name: Some("my-quota".to_string()),
                ..Default::default()
            }),
            spec: Some(ResourceQuotaSpec {
                hard: hard.clone(),
                scopes: vec![],
                scope_selector: None,
            }),
            status: Some(ResourceQuotaStatus { hard, used }),
        };

        assert_eq!(
            quota.metadata.as_ref().unwrap().name,
            Some("my-quota".to_string())
        );
        assert!(quota.spec.is_some());
        assert!(quota.status.is_some());
    }

    #[test]
    fn test_resource_quota_serialize() {
        let mut hard = HashMap::new();
        hard.insert("cpu".to_string(), Quantity::from("10"));

        let quota = ResourceQuota {
            metadata: Some(ObjectMeta {
                name: Some("my-quota".to_string()),
                ..Default::default()
            }),
            spec: Some(ResourceQuotaSpec {
                hard,
                scopes: vec![],
                scope_selector: None,
            }),
            status: None,
        };

        let json = serde_json::to_string(&quota).unwrap();
        assert!(json.contains(r#""name":"my-quota""#));
        assert!(json.contains(r#""hard":{"#));
        assert!(json.contains(r#""cpu":"10""#));
    }

    #[test]
    fn test_resource_quota_deserialize() {
        let json = r#"{
            "metadata": {"name": "my-quota"},
            "spec": {
                "hard": {"cpu": "10", "pods": "5"},
                "scopes": ["BestEffort"]
            },
            "status": {
                "hard": {"cpu": "10"},
                "used": {"cpu": "3"}
            }
        }"#;

        let quota: ResourceQuota = serde_json::from_str(json).unwrap();
        assert_eq!(
            quota.metadata.as_ref().unwrap().name,
            Some("my-quota".to_string())
        );
        assert!(quota.spec.is_some());
        assert!(quota.status.is_some());
    }

    #[test]
    fn test_resource_quota_list_empty() {
        let list = ResourceQuotaList {
            metadata: None,
            items: vec![],
        };
        assert!(list.items.is_empty());
    }

    #[test]
    fn test_resource_quota_list_with_items() {
        let list = ResourceQuotaList {
            metadata: Some(ListMeta {
                resource_version: Some("67890".to_string()),
                ..Default::default()
            }),
            items: vec![ResourceQuota {
                metadata: Some(ObjectMeta {
                    name: Some("quota-1".to_string()),
                    ..Default::default()
                }),
                spec: None,
                status: None,
            }],
        };

        assert_eq!(list.items.len(), 1);
    }

    #[test]
    fn test_resource_quota_round_trip() {
        let mut hard = HashMap::new();
        hard.insert("cpu".to_string(), Quantity::from("10"));
        hard.insert("pods".to_string(), Quantity::from("5"));

        let original = ResourceQuota {
            metadata: Some(ObjectMeta {
                name: Some("my-quota".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(ResourceQuotaSpec {
                hard: hard.clone(),
                scopes: vec![resource_quota_scope::BEST_EFFORT.to_string()],
                scope_selector: None,
            }),
            status: Some(ResourceQuotaStatus {
                hard,
                used: {
                    let mut m = HashMap::new();
                    m.insert("cpu".to_string(), Quantity::from("3"));
                    m
                },
            }),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: ResourceQuota = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    // Constant tests
    #[test]
    fn test_limit_type_constants() {
        assert_eq!(limit_type::POD, "Pod");
        assert_eq!(limit_type::CONTAINER, "Container");
        assert_eq!(limit_type::PERSISTENT_VOLUME_CLAIM, "PersistentVolumeClaim");
    }

    #[test]
    fn test_resource_quota_scope_constants() {
        assert_eq!(resource_quota_scope::TERMINATING, "Terminating");
        assert_eq!(resource_quota_scope::NOT_TERMINATING, "NotTerminating");
        assert_eq!(resource_quota_scope::BEST_EFFORT, "BestEffort");
        assert_eq!(resource_quota_scope::NOT_BEST_EFFORT, "NotBestEffort");
        assert_eq!(resource_quota_scope::PRIORITY_CLASS, "PriorityClass");
        assert_eq!(
            resource_quota_scope::CROSS_NAMESPACE_POD_AFFINITY,
            "CrossNamespacePodAffinity"
        );
        assert_eq!(
            resource_quota_scope::VOLUME_ATTRIBUTES_CLASS,
            "VolumeAttributesClass"
        );
    }

    #[test]
    fn test_scope_selector_operator_constants() {
        assert_eq!(scope_selector_operator::IN, "In");
        assert_eq!(scope_selector_operator::NOT_IN, "NotIn");
        assert_eq!(scope_selector_operator::EXISTS, "Exists");
        assert_eq!(scope_selector_operator::DOES_NOT_EXIST, "DoesNotExist");
    }

    #[test]
    fn test_resource_name_constants() {
        assert_eq!(resource_name::CPU, "cpu");
        assert_eq!(resource_name::MEMORY, "memory");
        assert_eq!(resource_name::STORAGE, "storage");
        assert_eq!(resource_name::EPHEMERAL_STORAGE, "ephemeral-storage");
        assert_eq!(resource_name::DEFAULT_NAMESPACE_PREFIX, "kubernetes.io/");
        assert_eq!(resource_name::HUGE_PAGES_PREFIX, "hugepages-");
        assert_eq!(
            resource_name::ATTACHABLE_VOLUMES_PREFIX,
            "attachable-volumes-"
        );
    }

    #[test]
    fn test_resource_list_with_multiple_resources() {
        let mut resources = ResourceList::new();
        resources.insert("cpu".to_string(), Quantity::from("10"));
        resources.insert("memory".to_string(), Quantity::from("20Gi"));
        resources.insert("pods".to_string(), Quantity::from("5"));

        assert_eq!(resources.len(), 3);
        assert_eq!(resources.get("cpu").unwrap().as_str(), "10");
    }

    #[test]
    fn test_scope_selector_with_multiple_expressions() {
        let selector = ScopeSelector {
            match_expressions: vec![
                ScopedResourceSelectorRequirement {
                    scope_name: resource_quota_scope::BEST_EFFORT.to_string(),
                    operator: scope_selector_operator::EXISTS.to_string(),
                    values: vec![],
                },
                ScopedResourceSelectorRequirement {
                    scope_name: resource_quota_scope::PRIORITY_CLASS.to_string(),
                    operator: scope_selector_operator::IN.to_string(),
                    values: vec!["high".to_string()],
                },
            ],
        };

        assert_eq!(selector.match_expressions.len(), 2);
    }

    #[test]
    fn test_limit_range_item_with_all_fields() {
        let mut max = HashMap::new();
        max.insert("cpu".to_string(), Quantity::from("4"));

        let mut min = HashMap::new();
        min.insert("cpu".to_string(), Quantity::from("100m"));

        let mut default = HashMap::new();
        default.insert("cpu".to_string(), Quantity::from("500m"));

        let mut default_request = HashMap::new();
        default_request.insert("cpu".to_string(), Quantity::from("200m"));

        let mut max_limit_request_ratio = HashMap::new();
        max_limit_request_ratio.insert("cpu".to_string(), Quantity::from("5"));

        let item = LimitRangeItem {
            type_: limit_type::CONTAINER.to_string(),
            max,
            min,
            default,
            default_request,
            max_limit_request_ratio,
        };

        assert_eq!(item.max.len(), 1);
        assert_eq!(item.min.len(), 1);
        assert_eq!(item.default.len(), 1);
        assert_eq!(item.default_request.len(), 1);
        assert_eq!(item.max_limit_request_ratio.len(), 1);
    }

    // ResourceRequirements tests
    #[test]
    fn test_resource_claim_default() {
        let claim = ResourceClaim::default();
        assert!(claim.name.is_empty());
        assert!(claim.request.is_empty());
    }

    #[test]
    fn test_resource_claim_with_fields() {
        let claim = ResourceClaim {
            name: "my-gpu".to_string(),
            request: "nvidia.com/gpu".to_string(),
        };

        assert_eq!(claim.name, "my-gpu");
        assert_eq!(claim.request, "nvidia.com/gpu");
    }

    #[test]
    fn test_resource_claim_serialize() {
        let claim = ResourceClaim {
            name: "ssd".to_string(),
            request: "fast-storage".to_string(),
        };

        let json = serde_json::to_string(&claim).unwrap();
        assert!(json.contains(r#""name":"ssd""#));
        assert!(json.contains(r#""request":"fast-storage""#));
    }

    #[test]
    fn test_resource_claim_deserialize() {
        let json = r#"{"name":"my-claim","request":"specific-request"}"#;
        let claim: ResourceClaim = serde_json::from_str(json).unwrap();

        assert_eq!(claim.name, "my-claim");
        assert_eq!(claim.request, "specific-request");
    }

    #[test]
    fn test_resource_claim_round_trip() {
        let claim = ResourceClaim {
            name: "test-resource".to_string(),
            request: String::new(),
        };

        let json = serde_json::to_string(&claim).unwrap();
        let deserialized: ResourceClaim = serde_json::from_str(&json).unwrap();

        assert_eq!(claim, deserialized);
    }

    #[test]
    fn test_resource_requirements_default() {
        let req = ResourceRequirements::default();
        assert!(req.limits.is_empty());
        assert!(req.requests.is_empty());
        assert!(req.claims.is_empty());
    }

    #[test]
    fn test_resource_requirements_with_limits() {
        let mut limits = ResourceList::new();
        limits.insert("cpu".to_string(), Quantity::from("2"));
        limits.insert("memory".to_string(), Quantity::from("4Gi"));

        let req = ResourceRequirements {
            limits,
            ..Default::default()
        };

        assert_eq!(req.limits.len(), 2);
        assert!(req.requests.is_empty());
    }

    #[test]
    fn test_resource_requirements_with_requests() {
        let mut requests = ResourceList::new();
        requests.insert("cpu".to_string(), Quantity::from("500m"));
        requests.insert("memory".to_string(), Quantity::from("512Mi"));

        let req = ResourceRequirements {
            requests,
            ..Default::default()
        };

        assert_eq!(req.requests.len(), 2);
        assert!(req.limits.is_empty());
    }

    #[test]
    fn test_resource_requirements_with_limits_and_requests() {
        let mut limits = ResourceList::new();
        limits.insert("cpu".to_string(), Quantity::from("1"));

        let mut requests = ResourceList::new();
        requests.insert("cpu".to_string(), Quantity::from("500m"));

        let req = ResourceRequirements {
            limits,
            requests,
            ..Default::default()
        };

        assert_eq!(req.limits.get("cpu").unwrap().as_str(), "1");
        assert_eq!(req.requests.get("cpu").unwrap().as_str(), "500m");
    }

    #[test]
    fn test_resource_requirements_with_claims() {
        let claims = vec![ResourceClaim {
            name: "my-claim".to_string(),
            request: String::new(),
        }];

        let req = ResourceRequirements {
            claims,
            ..Default::default()
        };

        assert_eq!(req.claims.len(), 1);
        assert_eq!(req.claims[0].name, "my-claim");
    }

    #[test]
    fn test_resource_requirements_serialize() {
        let mut limits = ResourceList::new();
        limits.insert("cpu".to_string(), Quantity::from("2"));

        let mut requests = ResourceList::new();
        requests.insert("cpu".to_string(), Quantity::from("1"));

        let req = ResourceRequirements {
            limits,
            requests,
            ..Default::default()
        };

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains(r#""limits":{"#));
        assert!(json.contains(r#""requests":{"#));
        assert!(json.contains(r#""cpu":"2""#));
        assert!(json.contains(r#""cpu":"1""#));
    }

    #[test]
    fn test_resource_requirements_deserialize() {
        let json = r#"{
            "limits": {"cpu": "4", "memory": "8Gi"},
            "requests": {"cpu": "2", "memory": "4Gi"}
        }"#;

        let req: ResourceRequirements = serde_json::from_str(json).unwrap();
        assert_eq!(req.limits.get("cpu").unwrap().as_str(), "4");
        assert_eq!(req.requests.get("cpu").unwrap().as_str(), "2");
    }

    #[test]
    fn test_resource_requirements_round_trip() {
        let mut limits = ResourceList::new();
        limits.insert("cpu".to_string(), Quantity::from("1"));
        limits.insert("memory".to_string(), Quantity::from("2Gi"));

        let mut requests = ResourceList::new();
        requests.insert("cpu".to_string(), Quantity::from("500m"));
        requests.insert("memory".to_string(), Quantity::from("1Gi"));

        let claims = vec![ResourceClaim {
            name: "gpu".to_string(),
            request: "nvidia.com/gpu".to_string(),
        }];

        let req = ResourceRequirements {
            limits,
            requests,
            claims,
        };

        let json = serde_json::to_string(&req).unwrap();
        let deserialized: ResourceRequirements = serde_json::from_str(&json).unwrap();

        assert_eq!(req, deserialized);
    }

    #[test]
    fn test_resource_requirements_with_ephemeral_storage() {
        let mut limits = ResourceList::new();
        limits.insert("ephemeral-storage".to_string(), Quantity::from("10Gi"));

        let mut requests = ResourceList::new();
        requests.insert("ephemeral-storage".to_string(), Quantity::from("5Gi"));

        let req = ResourceRequirements {
            limits,
            requests,
            ..Default::default()
        };

        assert_eq!(
            req.limits.get("ephemeral-storage").unwrap().as_str(),
            "10Gi"
        );
        assert_eq!(
            req.requests.get("ephemeral-storage").unwrap().as_str(),
            "5Gi"
        );
    }
}
