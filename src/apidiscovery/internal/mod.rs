//! Kubernetes API Discovery Internal Types
//!
//! This module contains type definitions from k8s.io/kubernetes/pkg/apis/apidiscovery/types.go
//!
//! Source: https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/apidiscovery/types.go

use crate::common::{GroupVersionKind, InternalObject, ListMeta, ObjectMeta, TypeMeta};
use crate::{impl_has_object_meta, impl_unimplemented_prost_message};
use serde::{Deserialize, Serialize};

/// APIGroupDiscoveryList mirrors the internal discovery list.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct APIGroupDiscoveryList {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "ListMeta::is_empty")]
    pub metadata: ListMeta,
    /// API groups aggregated by the server.
    #[serde(default)]
    pub items: Vec<APIGroupDiscovery>,
}

/// APIGroupDiscovery mirrors the internal discovery resource.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct APIGroupDiscovery {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Object metadata.
    #[serde(default, skip_serializing_if = "ObjectMeta::is_empty")]
    pub metadata: ObjectMeta,
    /// Versions served by this group.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub versions: Vec<APIVersionDiscovery>,
}
impl_has_object_meta!(APIGroupDiscovery);
impl InternalObject for APIGroupDiscovery {}

/// APIVersionDiscovery mirrors the internal version discovery object.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct APIVersionDiscovery {
    /// Version identifier.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub version: String,
    /// Resources for the version.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<APIResourceDiscovery>,
    /// Freshness of the discovery document.
    #[serde(default, skip_serializing_if = "DiscoveryFreshness::is_empty")]
    pub freshness: DiscoveryFreshness,
}

/// APIResourceDiscovery mirrors the internal resource discovery object.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct APIResourceDiscovery {
    /// Plural resource name.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub resource: String,
    /// Kind metadata for the resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_kind: Option<GroupVersionKind>,
    /// Scope of the resource.
    #[serde(default)]
    pub scope: ResourceScope,
    /// Singular resource name.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub singular_resource: String,
    /// Supported verbs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub verbs: Vec<String>,
    /// Short names.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub short_names: Vec<String>,
    /// Resource categories.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<String>,
    /// Subresources.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subresources: Vec<APISubresourceDiscovery>,
}

/// APISubresourceDiscovery mirrors the internal subresource discovery object.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct APISubresourceDiscovery {
    /// Subresource name.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub subresource: String,
    /// Kind metadata for the subresource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_kind: Option<GroupVersionKind>,
    /// Accepted GroupVersionKinds.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub accepted_types: Vec<GroupVersionKind>,
    /// Supported verbs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub verbs: Vec<String>,
}

/// ResourceScope is an enum defining the different scopes available to a resource.
///
/// Source: https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/apidiscovery/types.go#L115
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum ResourceScope {
    /// Cluster-scoped resources
    #[serde(rename = "Cluster")]
    #[default]
    Cluster,
    /// Namespaced-scoped resources
    Namespaced,
}

pub mod resource_scope {
    pub const CLUSTER: &str = "Cluster";
    pub const NAMESPACED: &str = "Namespaced";
}

/// DiscoveryFreshness is an enum defining whether the Discovery document published by an apiservice is up to date (fresh).
///
/// Source: https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/apidiscovery/types.go#L123
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum DiscoveryFreshness {
    /// Empty value used when freshness is not specified.
    #[serde(rename = "")]
    #[default]
    Unknown,
    /// The discovery document was recently refreshed
    #[serde(rename = "Current")]
    Current,
    /// The discovery document could not be retrieved and may be significantly out of date
    #[serde(rename = "Stale")]
    Stale,
}

impl DiscoveryFreshness {
    pub fn is_empty(value: &Self) -> bool {
        matches!(*value, DiscoveryFreshness::Unknown)
    }
}

pub mod discovery_freshness {
    pub const CURRENT: &str = "Current";
    pub const STALE: &str = "Stale";
}

// ===========================================================================
// ===========================================================================
// Protobuf Placeholder Implementations
// ===========================================================================

impl_unimplemented_prost_message!(APIGroupDiscovery);
impl_unimplemented_prost_message!(APIGroupDiscoveryList);

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::HasObjectMeta;

    /// 编译时检查：确保内部版本资源实现了必需的 traits
    #[test]
    fn internal_resources_implement_required_traits() {
        fn check<T: HasObjectMeta>() {}

        check::<APIGroupDiscovery>();
    }

    /// 编译时检查：确保内部版本资源实现了 prost::Message
    #[test]
    fn prost_message() {
        fn check<T: prost::Message>() {}

        check::<APIGroupDiscovery>();
        check::<APIGroupDiscoveryList>();
    }
}

// AsRefStr / AsRef<str> implementations for enums
crate::impl_as_str_ref!(ResourceScope, {
    Cluster => "Cluster",
    Namespaced => "Namespaced",
});

crate::impl_as_str_ref!(DiscoveryFreshness, {
    Unknown => "Unknown",
    Current => "Current",
    Stale => "Stale",
});
