//! Kubernetes API Discovery v2 API types
//!
//! This module contains types from the Kubernetes apidiscovery/v2 API group.
//!
//! Source: https://github.com/kubernetes/api/blob/master/apidiscovery/v2/types.go

pub mod conversion;

use crate::apidiscovery::internal::{DiscoveryFreshness, ResourceScope};
use crate::common::{
    ApplyDefault, GroupVersionKind, HasTypeMeta, ListMeta, ObjectMeta, ResourceSchema, TypeMeta,
};
use crate::{impl_unimplemented_prost_message, impl_versioned_object};
use serde::{Deserialize, Serialize};

// ============================================================================
// API Group Discovery Types
// ============================================================================

/// APIGroupDiscoveryList contains a list of APIGroupDiscovery resources.
///
/// Source: https://github.com/kubernetes/api/blob/master/apidiscovery/v2/types.go#L18
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct APIGroupDiscoveryList {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,
    /// List of API groups available on the server.
    #[serde(default)]
    pub items: Vec<APIGroupDiscovery>,
}

/// APIGroupDiscovery holds discovery information for a Kubernetes API group.
///
/// Source: https://github.com/kubernetes/api/blob/master/apidiscovery/v2/types.go#L32
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct APIGroupDiscovery {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// Versions exposed by this group.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub versions: Vec<APIVersionDiscovery>,
}

/// APIVersionDiscovery holds the discovery document for a single group+version.
///
/// Source: https://github.com/kubernetes/api/blob/master/apidiscovery/v2/types.go#L58
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct APIVersionDiscovery {
    /// The version name.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub version: String,
    /// Resources served in this version.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<APIResourceDiscovery>,
    /// Freshness of the discovery document.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub freshness: Option<DiscoveryFreshness>,
}

/// APIResourceDiscovery provides discovery data for a resource.
///
/// Source: https://github.com/kubernetes/api/blob/master/apidiscovery/v2/types.go#L79
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct APIResourceDiscovery {
    /// Plural name of the resource.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub resource: String,
    /// Kind metadata describing the typical response object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_kind: Option<GroupVersionKind>,
    /// Scope of the resource (cluster or namespaced).
    #[serde(default)]
    pub scope: ResourceScope,
    /// Singular resource name.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub singular_resource: String,
    /// Supported API verbs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub verbs: Vec<String>,
    /// Suggested short names.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub short_names: Vec<String>,
    /// Group categories this resource belongs to.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<String>,
    /// Subresources exposed by this resource.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subresources: Vec<APISubresourceDiscovery>,
}

/// APISubresourceDiscovery describes an API subresource.
///
/// Source: https://github.com/kubernetes/api/blob/master/apidiscovery/v2/types.go#L133
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct APISubresourceDiscovery {
    /// Name of the subresource.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub subresource: String,
    /// Kind metadata describing the subresource response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_kind: Option<GroupVersionKind>,
    /// Accepted GroupVersionKinds for this subresource.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub accepted_types: Vec<GroupVersionKind>,
    /// Supported verbs for the subresource.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub verbs: Vec<String>,
}

// ============================================================================
// Trait Implementations
// ============================================================================

impl_versioned_object!(APIGroupDiscovery);

impl ResourceSchema for APIGroupDiscovery {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "apidiscovery.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v2"
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
        "v2"
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
        "v2"
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
        "v2"
    }
    fn kind_static() -> &'static str {
        "APIGroupDiscoveryList"
    }
    fn resource_static() -> &'static str {
        ""
    }
}

impl HasTypeMeta for APIGroupDiscovery {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for APIGroupDiscoveryList {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl ApplyDefault for APIGroupDiscovery {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "apidiscovery.k8s.io/v2".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "APIGroupDiscovery".to_string();
        }
    }
}

impl ApplyDefault for APIGroupDiscoveryList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "apidiscovery.k8s.io/v2".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "APIGroupDiscoveryList".to_string();
        }
    }
}

impl_unimplemented_prost_message!(APIGroupDiscovery);
impl_unimplemented_prost_message!(APIGroupDiscoveryList);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::apidiscovery::internal;
    use crate::common::{FromInternal, ToInternal, TypeMeta, VersionedObject};

    // ========================================================================
    // Compile-time Trait Checks
    // ========================================================================

    /// 编译时检查：确保顶级资源实现了必需的 traits
    #[test]
    fn top_level_resources_implement_required_traits() {
        fn check<T: VersionedObject + ApplyDefault>() {}

        check::<APIGroupDiscovery>();
    }

    /// 编译时检查：确保资源实现了版本转换 traits
    #[test]
    fn conversion_traits() {
        fn check<T, I>()
        where
            T: ToInternal<I> + FromInternal<I>,
        {
        }

        check::<APIGroupDiscovery, internal::APIGroupDiscovery>();
    }

    /// 编译时检查：确保资源实现了 prost::Message
    #[test]
    fn prost_message() {
        fn check<T: prost::Message>() {}

        check::<APIGroupDiscovery>();
        check::<APIGroupDiscoveryList>();
    }

    // ========================================================================
    // Runtime Behavior Tests
    // ========================================================================

    #[test]
    fn test_apply_default_api_group_discovery() {
        let mut obj = APIGroupDiscovery {
            type_meta: TypeMeta::default(),
            metadata: None,
            versions: vec![],
        };

        obj.apply_default();

        assert_eq!(obj.type_meta.api_version, "apidiscovery.k8s.io/v2");
        assert_eq!(obj.type_meta.kind, "APIGroupDiscovery");
    }

    #[test]
    fn test_apply_default_preserves_existing_values() {
        let mut obj = APIGroupDiscovery {
            type_meta: TypeMeta {
                api_version: "custom.version/v1".to_string(),
                kind: "CustomKind".to_string(),
            },
            metadata: None,
            versions: vec![],
        };

        obj.apply_default();

        // Existing values should be preserved
        assert_eq!(obj.type_meta.api_version, "custom.version/v1");
        assert_eq!(obj.type_meta.kind, "CustomKind");
    }

    #[test]
    fn test_apply_default_partial_values() {
        let mut obj = APIGroupDiscovery {
            type_meta: TypeMeta {
                api_version: "existing.version".to_string(),
                kind: "".to_string(),
            },
            metadata: None,
            versions: vec![],
        };

        obj.apply_default();

        // apiVersion should be preserved, kind should be defaulted
        assert_eq!(obj.type_meta.api_version, "existing.version");
        assert_eq!(obj.type_meta.kind, "APIGroupDiscovery");
    }

    #[test]
    fn test_apply_default_api_group_discovery_list() {
        let mut obj = APIGroupDiscoveryList {
            type_meta: TypeMeta::default(),
            metadata: None,
            items: vec![],
        };

        obj.apply_default();

        assert_eq!(obj.type_meta.api_version, "apidiscovery.k8s.io/v2");
        assert_eq!(obj.type_meta.kind, "APIGroupDiscoveryList");
    }

    #[test]
    fn test_apply_default_api_group_discovery_list_preserves_existing() {
        let mut obj = APIGroupDiscoveryList {
            type_meta: TypeMeta {
                api_version: "custom.version/v1".to_string(),
                kind: "CustomKind".to_string(),
            },
            metadata: None,
            items: vec![],
        };

        obj.apply_default();

        assert_eq!(obj.type_meta.api_version, "custom.version/v1");
        assert_eq!(obj.type_meta.kind, "CustomKind");
    }
}

#[cfg(test)]
mod trait_tests;
