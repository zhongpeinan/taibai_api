//! Kubernetes API Discovery v1 API types
//!
//! This module contains types from the Kubernetes apidiscovery/v1 API group.
//!
//! Source: https://github.com/kubernetes/api/blob/master/apidiscovery/v1/types.go

use crate::apidiscovery::internal::{DiscoveryFreshness, ResourceScope};
use crate::common::{
    ApplyDefault, GroupVersionKind, HasTypeMeta, ListMeta, ObjectMeta, ResourceSchema, TypeMeta,
    UnimplementedConversion,
};
use crate::impl_unimplemented_prost_message;
use serde::{Deserialize, Serialize};

// ============================================================================
// API Group Discovery Types
// ============================================================================

/// APIGroupDiscoveryList is a resource containing a list of APIGroupDiscovery.
///
/// This is one of the types able to be returned from the /api and /apis endpoint and contains an aggregated
/// list of API resources (built-ins, Custom Resource Definitions, resources from aggregated servers)
/// that a cluster supports.
///
/// Source: https://github.com/kubernetes/api/blob/master/apidiscovery/v1/types.go#L28
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct APIGroupDiscoveryList {
    /// Standard list metadata.
    ///
    /// ResourceVersion will not be set, because this does not have a replayable ordering among multiple apiservers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,
    /// items is the list of groups for discovery. The groups are listed in priority order.
    #[serde(default)]
    pub items: Vec<APIGroupDiscovery>,
}

/// APIGroupDiscovery holds information about which resources are being served for all version of the API Group.
///
/// It contains a list of APIVersionDiscovery that holds a list of APIResourceDiscovery types served for a version.
/// Versions are in descending order of preference, with the first version being the preferred entry.
///
/// Source: https://github.com/kubernetes/api/blob/master/apidiscovery/v1/types.go#L44
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct APIGroupDiscovery {
    /// Standard type metadata.
    #[serde(default)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    ///
    /// The only field completed will be name. For instance, resourceVersion will be empty.
    /// name is the name of the API group whose discovery information is presented here.
    /// name is allowed to be "" to represent the legacy, ungroupified resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// versions are the versions supported in this group. They are sorted in descending order of preference,
    /// with the preferred version being the first entry.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub versions: Vec<APIVersionDiscovery>,
}

/// APIVersionDiscovery holds a list of APIResourceDiscovery types that are served for a particular version within an API Group.
///
/// Source: https://github.com/kubernetes/api/blob/master/apidiscovery/v1/types.go#L61
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct APIVersionDiscovery {
    /// version is the name of the version within a group version.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub version: String,
    /// resources is a list of APIResourceDiscovery objects for the corresponding group version.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<APIResourceDiscovery>,
    /// freshness marks whether a group version's discovery document is up to date.
    ///
    /// "Current" indicates the discovery document was recently
    /// refreshed. "Stale" indicates the discovery document could not
    /// be retrieved and the returned discovery document may be
    /// significantly out of date. Clients that require the latest
    /// version of the discovery information be retrieved before
    /// performing an operation should not use the aggregated document
    /// and instead retrieve the necessary version docs directly.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub freshness: Option<DiscoveryFreshness>,
}

/// APIResourceDiscovery provides information about an API resource for discovery.
///
/// Source: https://github.com/kubernetes/api/blob/master/apidiscovery/v1/types.go#L80
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct APIResourceDiscovery {
    /// resource is the plural name of the resource.  This is used in the URL path and is the unique identifier
    /// for this resource across all versions in the API group.
    ///
    /// Resources with non-empty groups are located at /apis/<APIGroupDiscovery.objectMeta.name>/<APIVersionDiscovery.version>/<APIResourceDiscovery.Resource>
    /// Resources with empty groups are located at /api/v1/<APIResourceDiscovery.Resource>
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub resource: String,
    /// responseKind describes the group, version, and kind of the serialization schema for the object type this endpoint typically returns.
    ///
    /// APIs may return other objects types at their discretion, such as error conditions, requests for alternate representations, or other operation specific behavior.
    /// This value will be null or empty if an APIService reports subresources but supports no operations on the parent resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_kind: Option<GroupVersionKind>,
    /// scope indicates the scope of a resource, either Cluster or Namespaced
    #[serde(default)]
    pub scope: ResourceScope,
    /// singularResource is the singular name of the resource.  This allows clients to handle plural and singular opaquely.
    ///
    /// For many clients the singular form of the resource will be more understandable to users reading messages and should be used when integrating the name of the resource into a sentence.
    /// The command line tool kubectl, for example, allows use of the singular resource name in place of plurals.
    /// The singular form of a resource should always be an optional element - when in doubt use the canonical resource name.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub singular_resource: String,
    /// verbs is a list of supported API operation types (this includes
    /// but is not limited to get, list, watch, create, update, patch,
    /// delete, deletecollection, and proxy).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub verbs: Vec<String>,
    /// shortNames is a list of suggested short names of the resource.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub short_names: Vec<String>,
    /// categories is a list of the grouped resources this resource belongs to (e.g. 'all').
    ///
    /// Clients may use this to simplify acting on multiple resource types at once.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<String>,
    /// subresources is a list of subresources provided by this resource. Subresources are located at /apis/<APIGroupDiscovery.objectMeta.name>/<APIVersionDiscovery.version>/<APIResourceDiscovery.Resource>/name-of-instance/<APIResourceDiscovery.subresources[i].subresource>
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subresources: Vec<APISubresourceDiscovery>,
}

/// APISubresourceDiscovery provides information about an API subresource for discovery.
///
/// Source: https://github.com/kubernetes/api/blob/master/apidiscovery/v1/types.go#L132
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct APISubresourceDiscovery {
    /// subresource is the name of the subresource.  This is used in the URL path and is the unique identifier
    /// for this resource across all versions.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub subresource: String,
    /// responseKind describes the group, version, and kind of the serialization schema for the object type this endpoint typically returns.
    ///
    /// Some subresources do not return normal resources, these will have null or empty return types.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_kind: Option<GroupVersionKind>,
    /// acceptedTypes describes the kinds that this endpoint accepts.
    ///
    /// Subresources may accept the standard content types or define
    /// custom negotiation schemes. The list may not be exhaustive for
    /// all operations.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub accepted_types: Vec<GroupVersionKind>,
    /// verbs is a list of supported API operation types (this includes
    /// but is not limited to get, list, watch, create, update, patch,
    /// delete, deletecollection, and proxy). Subresources may define
    /// custom verbs outside the standard Kubernetes verb set. Clients
    /// should expect the behavior of standard verbs to align with
    /// Kubernetes interaction conventions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub verbs: Vec<String>,
}

// ============================================================================
// Trait Implementations for APIGroupDiscovery and APIGroupDiscoveryList
// ============================================================================

// ----------------------------------------------------------------------------
// ResourceSchema Implementation
// ----------------------------------------------------------------------------

impl ResourceSchema for APIGroupDiscovery {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "apidiscovery.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "APIGroupDiscovery"
    }
    fn resource(_: &Self::Meta) -> &str {
        ""
    }

    fn group_static() -> &'static str {
        "apidiscovery.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "APIGroupDiscovery"
    }
    fn resource_static() -> &'static str {
        ""
    }
}

impl ResourceSchema for APIGroupDiscoveryList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "apidiscovery.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "APIGroupDiscoveryList"
    }
    fn resource(_: &Self::Meta) -> &str {
        ""
    }

    fn group_static() -> &'static str {
        "apidiscovery.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "APIGroupDiscoveryList"
    }
    fn resource_static() -> &'static str {
        ""
    }
}

// ----------------------------------------------------------------------------
// HasTypeMeta Implementation for APIGroupDiscovery (uses type_meta field)
// ----------------------------------------------------------------------------

impl HasTypeMeta for APIGroupDiscovery {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

// Note: APIGroupDiscoveryList does not have type_meta field, so we skip HasTypeMeta
// Note: APIGroupDiscovery does not implement VersionedObject because it needs special handling

// ----------------------------------------------------------------------------
// ApplyDefaults Implementation
// ----------------------------------------------------------------------------

impl ApplyDefault for APIGroupDiscovery {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "apidiscovery.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "APIGroupDiscovery".to_string();
        }
    }
}

impl ApplyDefault for APIGroupDiscoveryList {
    fn apply_default(&mut self) {
        // APIGroupDiscoveryList does not have type_meta field
    }
}

// ----------------------------------------------------------------------------
// Version Conversion Placeholder (using UnimplementedConversion)
// ----------------------------------------------------------------------------

impl UnimplementedConversion for APIGroupDiscovery {}

// ----------------------------------------------------------------------------
// Protobuf Placeholder (using macro)
// ----------------------------------------------------------------------------

impl_unimplemented_prost_message!(APIGroupDiscovery);
impl_unimplemented_prost_message!(APIGroupDiscoveryList);

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {}
