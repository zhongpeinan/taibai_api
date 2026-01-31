use super::{
    APIGroupDiscovery, APIGroupDiscoveryList, APIResourceDiscovery, APISubresourceDiscovery,
    APIVersionDiscovery,
};
use crate::apidiscovery::internal::{DiscoveryFreshness, ResourceScope};
use crate::common::test_utils::assert_serde_roundtrip;
use crate::common::{GroupVersionKind, ListMeta, ObjectMeta, TypeMeta};

fn api_group_discovery_basic() -> APIGroupDiscovery {
    APIGroupDiscovery {
        type_meta: TypeMeta {
            api_version: "apidiscovery.k8s.io/v2".to_string(),
            kind: "APIGroupDiscovery".to_string(),
        },
        metadata: Some(ObjectMeta {
            name: Some("apps".to_string()),
            ..Default::default()
        }),
        versions: vec![APIVersionDiscovery {
            version: "v1".to_string(),
            resources: vec![APIResourceDiscovery {
                resource: "deployments".to_string(),
                response_kind: Some(GroupVersionKind {
                    group: "apps".to_string(),
                    version: "v1".to_string(),
                    kind: "Deployment".to_string(),
                }),
                scope: ResourceScope::Namespaced,
                singular_resource: "deployment".to_string(),
                verbs: vec!["get".to_string(), "list".to_string(), "watch".to_string()],
                short_names: vec!["deploy".to_string()],
                categories: vec!["all".to_string()],
                subresources: vec![APISubresourceDiscovery {
                    subresource: "status".to_string(),
                    response_kind: Some(GroupVersionKind {
                        group: "apps".to_string(),
                        version: "v1".to_string(),
                        kind: "Deployment".to_string(),
                    }),
                    accepted_types: Vec::new(),
                    verbs: vec!["get".to_string(), "patch".to_string()],
                }],
            }],
            freshness: Some(DiscoveryFreshness::Current),
        }],
    }
}

fn api_group_discovery_list_basic() -> APIGroupDiscoveryList {
    APIGroupDiscoveryList {
        type_meta: TypeMeta {
            api_version: "apidiscovery.k8s.io/v2".to_string(),
            kind: "APIGroupDiscoveryList".to_string(),
        },
        metadata: Some(ListMeta {
            resource_version: Some("1".to_string()),
            ..Default::default()
        }),
        items: vec![api_group_discovery_basic()],
    }
}

#[test]
fn serde_roundtrip_api_group_discovery() {
    assert_serde_roundtrip(&api_group_discovery_basic());
}

#[test]
fn serde_roundtrip_api_group_discovery_list() {
    assert_serde_roundtrip(&api_group_discovery_list_basic());
}
