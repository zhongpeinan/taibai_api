//! ResourceQuota and LimitRange types from the Kubernetes Core v1 API
//!
//! This module contains types for managing resource quotas and limits.

use crate::common::{
    ApplyDefault, HasTypeMeta, ListMeta, ObjectMeta, Quantity, ResourceSchema, TypeMeta,
    UnimplementedConversion, VersionedObject,
};
use crate::impl_unimplemented_prost_message;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

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
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub max: BTreeMap<String, Quantity>,

    /// Min usage constraints on this kind by resource name.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub min: BTreeMap<String, Quantity>,

    /// Default resource requirement limit value by resource name if resource limit is omitted.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub default: BTreeMap<String, Quantity>,

    /// DefaultRequest is the default resource requirement request value by resource name.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub default_request: BTreeMap<String, Quantity>,

    /// MaxLimitRequestRatio represents the max burst for the named resource.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub max_limit_request_ratio: BTreeMap<String, Quantity>,
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
    /// TypeMeta describes the type of this object
    #[serde(flatten)]
    pub type_meta: TypeMeta,

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
    /// TypeMeta describes the type of this object
    #[serde(flatten)]
    pub type_meta: TypeMeta,

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
pub type ResourceList = BTreeMap<ResourceName, Quantity>;

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
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
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
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub hard: ResourceList,

    /// Used is the current observed total usage of the resource in the namespace.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub used: ResourceList,
}

/// ResourceQuota sets aggregate quota restrictions enforced per namespace.
///
/// Corresponds to [Kubernetes ResourceQuota](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7856)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ResourceQuota {
    /// TypeMeta describes the type of this object
    #[serde(flatten)]
    pub type_meta: TypeMeta,

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
    /// TypeMeta describes the type of this object
    #[serde(flatten)]
    pub type_meta: TypeMeta,

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
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub limits: ResourceList,
    /// Requests describes the minimum amount of compute resources required.
    /// If Requests is omitted for a container, it defaults to Limits if that is explicitly specified.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub requests: ResourceList,
    /// Claims lists the names of resources, defined in spec.resourceClaims,
    /// that are used by this container.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub claims: Vec<ResourceClaim>,
}

#[cfg(test)]
mod tests {}

// ============================================================================
// Trait Implementations
// ============================================================================

// ----------------------------------------------------------------------------
// ResourceSchema Implementation
// ----------------------------------------------------------------------------

impl ResourceSchema for LimitRange {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        ""
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "LimitRange"
    }
    fn resource(_: &Self::Meta) -> &str {
        "limitranges"
    }

    fn group_static() -> &'static str {
        ""
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "LimitRange"
    }
    fn resource_static() -> &'static str {
        "limitranges"
    }
}

impl ResourceSchema for LimitRangeList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        ""
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "LimitRangeList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "limitranges"
    }

    fn group_static() -> &'static str {
        ""
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "LimitRangeList"
    }
    fn resource_static() -> &'static str {
        "limitranges"
    }
}

impl ResourceSchema for ResourceQuota {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        ""
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "ResourceQuota"
    }
    fn resource(_: &Self::Meta) -> &str {
        "resourcequotas"
    }

    fn group_static() -> &'static str {
        ""
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "ResourceQuota"
    }
    fn resource_static() -> &'static str {
        "resourcequotas"
    }
}

impl ResourceSchema for ResourceQuotaList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        ""
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "ResourceQuotaList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "resourcequotas"
    }

    fn group_static() -> &'static str {
        ""
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "ResourceQuotaList"
    }
    fn resource_static() -> &'static str {
        "resourcequotas"
    }
}

// ----------------------------------------------------------------------------
// HasTypeMeta Implementation
// ----------------------------------------------------------------------------

impl HasTypeMeta for LimitRange {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for LimitRangeList {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for ResourceQuota {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for ResourceQuotaList {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

// ----------------------------------------------------------------------------
// VersionedObject Implementation
// ----------------------------------------------------------------------------

impl VersionedObject for LimitRange {
    fn metadata(&self) -> &ObjectMeta {
        use std::sync::OnceLock;
        self.metadata.as_ref().unwrap_or_else(|| {
            static DEFAULT: OnceLock<ObjectMeta> = OnceLock::new();
            DEFAULT.get_or_init(ObjectMeta::default)
        })
    }

    fn metadata_mut(&mut self) -> &mut ObjectMeta {
        self.metadata.get_or_insert_with(ObjectMeta::default)
    }
}

impl VersionedObject for ResourceQuota {
    fn metadata(&self) -> &ObjectMeta {
        use std::sync::OnceLock;
        self.metadata.as_ref().unwrap_or_else(|| {
            static DEFAULT: OnceLock<ObjectMeta> = OnceLock::new();
            DEFAULT.get_or_init(ObjectMeta::default)
        })
    }

    fn metadata_mut(&mut self) -> &mut ObjectMeta {
        self.metadata.get_or_insert_with(ObjectMeta::default)
    }
}

// Note: List types do not implement VersionedObject because they have ListMeta, not ObjectMeta

// ----------------------------------------------------------------------------
// ApplyDefaults Implementation
// ----------------------------------------------------------------------------

impl ApplyDefault for LimitRange {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "LimitRange".to_string();
        }
    }
}

impl ApplyDefault for LimitRangeList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "LimitRangeList".to_string();
        }
    }
}

impl ApplyDefault for ResourceQuota {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "ResourceQuota".to_string();
        }
    }
}

impl ApplyDefault for ResourceQuotaList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "ResourceQuotaList".to_string();
        }
    }
}

// ----------------------------------------------------------------------------
// Version Conversion Placeholder
// ----------------------------------------------------------------------------

impl UnimplementedConversion for LimitRange {}
impl UnimplementedConversion for LimitRangeList {}
impl UnimplementedConversion for ResourceQuota {}
impl UnimplementedConversion for ResourceQuotaList {}

// ----------------------------------------------------------------------------
// Protobuf Placeholder
// ----------------------------------------------------------------------------

impl_unimplemented_prost_message!(LimitRange);
impl_unimplemented_prost_message!(LimitRangeList);
impl_unimplemented_prost_message!(ResourceQuota);
impl_unimplemented_prost_message!(ResourceQuotaList);
