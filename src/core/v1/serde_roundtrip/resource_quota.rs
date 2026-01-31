use crate::common::test_utils::assert_serde_roundtrip;
use crate::common::{ListMeta, ObjectMeta, Quantity, TypeMeta};
use crate::core::v1::{
    ResourceList, ResourceQuota, ResourceQuotaList, ResourceQuotaSpec, ResourceQuotaStatus,
    ScopeSelector, ScopedResourceSelectorRequirement, resource_quota_scope,
    scope_selector_operator,
};

fn resource_quota_basic() -> ResourceQuota {
    let hard = ResourceList::from([
        ("pods".to_string(), Quantity("10".to_string())),
        ("requests.cpu".to_string(), Quantity("4".to_string())),
    ]);
    let used = ResourceList::from([("pods".to_string(), Quantity("2".to_string()))]);

    ResourceQuota {
        type_meta: TypeMeta {
            api_version: "v1".to_string(),
            kind: "ResourceQuota".to_string(),
        },
        metadata: Some(ObjectMeta {
            name: Some("quota-a".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        spec: Some(ResourceQuotaSpec {
            hard,
            scopes: vec![resource_quota_scope::NOT_TERMINATING.to_string()],
            scope_selector: Some(ScopeSelector {
                match_expressions: vec![ScopedResourceSelectorRequirement {
                    scope_name: resource_quota_scope::PRIORITY_CLASS.to_string(),
                    operator: scope_selector_operator::IN.to_string(),
                    values: vec!["high".to_string()],
                }],
            }),
        }),
        status: Some(ResourceQuotaStatus {
            hard: ResourceList::default(),
            used,
        }),
    }
}

fn resource_quota_list_basic() -> ResourceQuotaList {
    ResourceQuotaList {
        type_meta: TypeMeta {
            api_version: "v1".to_string(),
            kind: "ResourceQuotaList".to_string(),
        },
        metadata: Some(ListMeta {
            resource_version: Some("12".to_string()),
            ..Default::default()
        }),
        items: vec![resource_quota_basic()],
    }
}

#[test]
fn serde_roundtrip_resource_quota() {
    assert_serde_roundtrip(&resource_quota_basic());
}

#[test]
fn serde_roundtrip_resource_quota_list() {
    assert_serde_roundtrip(&resource_quota_list_basic());
}
