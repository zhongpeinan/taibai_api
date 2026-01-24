//! Conversions between v2 and internal apidiscovery types

use crate::apidiscovery::internal;
use crate::common::{ApplyDefault, FromInternal, ListMeta, ObjectMeta, ToInternal, TypeMeta};

use super::{
    APIGroupDiscovery, APIGroupDiscoveryList, APIResourceDiscovery, APISubresourceDiscovery,
    APIVersionDiscovery,
};

// ============================================================================
// Conversion Helper Functions
// ============================================================================

fn is_empty_object_meta(meta: &ObjectMeta) -> bool {
    meta.name.is_none()
        && meta.generate_name.is_none()
        && meta.namespace.is_none()
        && meta.uid.is_none()
        && meta.resource_version.is_none()
        && meta.generation.is_none()
        && meta.self_link.is_none()
        && meta.labels.is_empty()
        && meta.annotations.is_empty()
        && meta.owner_references.is_empty()
        && meta.finalizers.is_empty()
        && meta.managed_fields.is_empty()
        && meta.creation_timestamp.is_none()
        && meta.deletion_timestamp.is_none()
        && meta.deletion_grace_period_seconds.is_none()
}

fn is_empty_list_meta(meta: &ListMeta) -> bool {
    meta.continue_.is_none()
        && meta.remaining_item_count.is_none()
        && meta.resource_version.is_none()
        && meta.self_link.is_none()
}

fn option_object_meta_to_meta(meta: Option<ObjectMeta>) -> ObjectMeta {
    meta.unwrap_or_default()
}

fn meta_to_option_object_meta(meta: ObjectMeta) -> Option<ObjectMeta> {
    if is_empty_object_meta(&meta) {
        None
    } else {
        Some(meta)
    }
}

fn option_list_meta_to_meta(meta: Option<ListMeta>) -> ListMeta {
    meta.unwrap_or_default()
}

fn meta_to_option_list_meta(meta: ListMeta) -> Option<ListMeta> {
    if is_empty_list_meta(&meta) {
        None
    } else {
        Some(meta)
    }
}

// ============================================================================
// APIGroupDiscovery Conversions
// ============================================================================

impl ToInternal<internal::APIGroupDiscovery> for APIGroupDiscovery {
    fn to_internal(self) -> internal::APIGroupDiscovery {
        internal::APIGroupDiscovery {
            type_meta: TypeMeta::default(),
            metadata: option_object_meta_to_meta(self.metadata),
            versions: self.versions.into_iter().map(Into::into).collect(),
        }
    }
}

impl FromInternal<internal::APIGroupDiscovery> for APIGroupDiscovery {
    fn from_internal(value: internal::APIGroupDiscovery) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: meta_to_option_object_meta(value.metadata),
            versions: value.versions.into_iter().map(Into::into).collect(),
        };
        result.apply_default();
        result
    }
}

// ============================================================================
// APIGroupDiscoveryList Conversions
// ============================================================================

impl From<APIGroupDiscoveryList> for internal::APIGroupDiscoveryList {
    fn from(value: APIGroupDiscoveryList) -> Self {
        Self {
            type_meta: TypeMeta::default(),
            metadata: option_list_meta_to_meta(value.metadata),
            items: value
                .items
                .into_iter()
                .map(|item| item.to_internal())
                .collect(),
        }
    }
}

impl From<internal::APIGroupDiscoveryList> for APIGroupDiscoveryList {
    fn from(value: internal::APIGroupDiscoveryList) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: meta_to_option_list_meta(value.metadata),
            items: value
                .items
                .into_iter()
                .map(APIGroupDiscovery::from_internal)
                .collect(),
        };
        result.apply_default();
        result
    }
}

// ============================================================================
// APIVersionDiscovery Conversions
// ============================================================================

impl From<APIVersionDiscovery> for internal::APIVersionDiscovery {
    fn from(value: APIVersionDiscovery) -> Self {
        Self {
            version: value.version,
            resources: value.resources.into_iter().map(Into::into).collect(),
            freshness: value.freshness.unwrap_or_default(),
        }
    }
}

impl From<internal::APIVersionDiscovery> for APIVersionDiscovery {
    fn from(value: internal::APIVersionDiscovery) -> Self {
        Self {
            version: value.version,
            resources: value.resources.into_iter().map(Into::into).collect(),
            freshness: match value.freshness {
                internal::DiscoveryFreshness::Unknown => None,
                freshness => Some(freshness),
            },
        }
    }
}

// ============================================================================
// APIResourceDiscovery Conversions
// ============================================================================

impl From<APIResourceDiscovery> for internal::APIResourceDiscovery {
    fn from(value: APIResourceDiscovery) -> Self {
        Self {
            resource: value.resource,
            response_kind: value.response_kind,
            scope: value.scope,
            singular_resource: value.singular_resource,
            verbs: value.verbs,
            short_names: value.short_names,
            categories: value.categories,
            subresources: value.subresources.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<internal::APIResourceDiscovery> for APIResourceDiscovery {
    fn from(value: internal::APIResourceDiscovery) -> Self {
        Self {
            resource: value.resource,
            response_kind: value.response_kind,
            scope: value.scope,
            singular_resource: value.singular_resource,
            verbs: value.verbs,
            short_names: value.short_names,
            categories: value.categories,
            subresources: value.subresources.into_iter().map(Into::into).collect(),
        }
    }
}

// ============================================================================
// APISubresourceDiscovery Conversions
// ============================================================================

impl From<APISubresourceDiscovery> for internal::APISubresourceDiscovery {
    fn from(value: APISubresourceDiscovery) -> Self {
        Self {
            subresource: value.subresource,
            response_kind: value.response_kind,
            accepted_types: value.accepted_types,
            verbs: value.verbs,
        }
    }
}

impl From<internal::APISubresourceDiscovery> for APISubresourceDiscovery {
    fn from(value: internal::APISubresourceDiscovery) -> Self {
        Self {
            subresource: value.subresource,
            response_kind: value.response_kind,
            accepted_types: value.accepted_types,
            verbs: value.verbs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::apidiscovery::internal::{DiscoveryFreshness, ResourceScope};
    use crate::common::GroupVersionKind;
    use crate::common::{FromInternal, ToInternal};

    #[test]
    fn test_api_group_discovery_round_trip() {
        let v2 = APIGroupDiscovery {
            type_meta: TypeMeta {
                api_version: "apidiscovery.k8s.io/v2".to_string(),
                kind: "APIGroupDiscovery".to_string(),
            },
            metadata: None,
            versions: vec![APIVersionDiscovery {
                version: "v1".to_string(),
                resources: vec![],
                freshness: Some(DiscoveryFreshness::Current),
            }],
        };

        let internal = v2.clone().to_internal();
        let back = APIGroupDiscovery::from_internal(internal);

        assert_eq!(back.versions[0].version, "v1");
        assert_eq!(
            back.versions[0].freshness,
            Some(DiscoveryFreshness::Current)
        );
        // Verify that apply_default was called - TypeMeta should be populated
        assert_eq!(back.type_meta.api_version, "apidiscovery.k8s.io/v2");
        assert_eq!(back.type_meta.kind, "APIGroupDiscovery");
    }

    #[test]
    fn test_api_group_discovery_list_apply_default() {
        let internal = internal::APIGroupDiscoveryList {
            type_meta: TypeMeta::default(),
            metadata: ListMeta::default(),
            items: vec![],
        };

        let v2: APIGroupDiscoveryList = internal.into();
        // Verify that apply_default was called
        assert_eq!(v2.type_meta.api_version, "apidiscovery.k8s.io/v2");
        assert_eq!(v2.type_meta.kind, "APIGroupDiscoveryList");
    }

    #[test]
    fn test_api_resource_discovery_conversion() {
        let v2 = APIResourceDiscovery {
            resource: "pods".to_string(),
            response_kind: Some(GroupVersionKind {
                group: "".to_string(),
                version: "v1".to_string(),
                kind: "Pod".to_string(),
            }),
            scope: ResourceScope::Namespaced,
            singular_resource: "pod".to_string(),
            verbs: vec!["get".to_string(), "list".to_string()],
            short_names: vec!["po".to_string()],
            categories: vec![],
            subresources: vec![],
        };

        let internal: internal::APIResourceDiscovery = v2.clone().into();
        let back: APIResourceDiscovery = internal.into();

        assert_eq!(back.resource, "pods");
        assert_eq!(back.scope, ResourceScope::Namespaced);
        assert_eq!(back.verbs, vec!["get".to_string(), "list".to_string()]);
    }
}
