use super::{
    APIGroupDiscovery, APIGroupDiscoveryList, APIResourceDiscovery, APISubresourceDiscovery,
    APIVersionDiscovery,
};
use crate::apidiscovery::internal;
use crate::apidiscovery::internal::{DiscoveryFreshness, ResourceScope};
use crate::common::test_utils::assert_conversion_roundtrip;
use crate::common::{ApplyDefault, GroupVersionKind, ListMeta, ObjectMeta, TypeMeta};

fn api_group_discovery_basic() -> APIGroupDiscovery {
    APIGroupDiscovery {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("batch".to_string()),
            ..Default::default()
        }),
        versions: vec![APIVersionDiscovery {
            version: "v1".to_string(),
            resources: vec![APIResourceDiscovery {
                resource: "jobs".to_string(),
                response_kind: Some(GroupVersionKind {
                    group: "batch".to_string(),
                    version: "v1".to_string(),
                    kind: "Job".to_string(),
                }),
                scope: ResourceScope::Namespaced,
                singular_resource: "job".to_string(),
                verbs: vec!["get".to_string(), "list".to_string(), "watch".to_string()],
                short_names: vec!["job".to_string()],
                categories: Vec::new(),
                subresources: vec![APISubresourceDiscovery {
                    subresource: "status".to_string(),
                    response_kind: Some(GroupVersionKind {
                        group: "batch".to_string(),
                        version: "v1".to_string(),
                        kind: "Job".to_string(),
                    }),
                    accepted_types: Vec::new(),
                    verbs: vec!["get".to_string(), "patch".to_string()],
                }],
            }],
            freshness: Some(DiscoveryFreshness::Stale),
        }],
    }
}

fn api_group_discovery_list_basic() -> APIGroupDiscoveryList {
    let mut item = api_group_discovery_basic();
    item.apply_default();

    APIGroupDiscoveryList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("2".to_string()),
            ..Default::default()
        }),
        items: vec![item],
    }
}

#[test]
fn conversion_roundtrip_api_group_discovery() {
    assert_conversion_roundtrip::<APIGroupDiscovery, internal::APIGroupDiscovery>(
        api_group_discovery_basic(),
    );
}

#[test]
fn conversion_roundtrip_api_group_discovery_list() {
    assert_conversion_roundtrip::<APIGroupDiscoveryList, internal::APIGroupDiscoveryList>(
        api_group_discovery_list_basic(),
    );
}
