use super::{
    DeviceClass, DeviceClassList, DeviceClassSpec, ResourceClaim, ResourceClaimList,
    ResourceClaimSpec, ResourceClaimTemplate, ResourceClaimTemplateList, ResourceClaimTemplateSpec,
    ResourceSlice, ResourceSliceList, ResourceSliceSpec,
};
use crate::common::test_utils::assert_serde_roundtrip;
use crate::common::{ListMeta, ObjectMeta, Quantity, TypeMeta};
use crate::resource::v1::device_class::{
    CELDeviceSelector as ClassCel, DeviceClassConfiguration, DeviceSelector as ClassSel,
    OpaqueDeviceConfiguration,
};
use crate::resource::v1::resource_claim::{
    CapacityRequirements, DeviceClaim, DeviceConstraint, DeviceRequest, DeviceSelector as ClaimSel,
    DeviceToleration, ExactDeviceRequest, ResourceClaimConsumerReference,
};
use crate::resource::v1::resource_slice::{
    Counter, CounterSet, Device, DeviceAttribute, DeviceCapacity, DeviceCounterConsumption,
    DeviceTaint, ResourcePool,
};
use std::collections::BTreeMap;

fn device_class_basic() -> DeviceClass {
    DeviceClass {
        type_meta: TypeMeta {
            api_version: "resource.k8s.io/v1".to_string(),
            kind: "DeviceClass".to_string(),
        },
        metadata: Some(ObjectMeta {
            name: Some("gpu".to_string()),
            ..Default::default()
        }),
        spec: DeviceClassSpec {
            selectors: vec![ClassSel {
                cel: Some(ClassCel {
                    expression: "true".to_string(),
                }),
            }],
            config: vec![DeviceClassConfiguration {
                opaque: OpaqueDeviceConfiguration {
                    driver: "example.com/driver".to_string(),
                    parameters: Some(serde_json::json!({"mode": "shared"})),
                },
            }],
            extended_resource_name: Some("example.com/gpu".to_string()),
        },
    }
}

fn device_class_list_basic() -> DeviceClassList {
    DeviceClassList {
        type_meta: TypeMeta {
            api_version: "resource.k8s.io/v1".to_string(),
            kind: "DeviceClassList".to_string(),
        },
        metadata: Some(ListMeta {
            resource_version: Some("1".to_string()),
            ..Default::default()
        }),
        items: vec![device_class_basic()],
    }
}

fn resource_claim_basic() -> ResourceClaim {
    ResourceClaim {
        type_meta: TypeMeta {
            api_version: "resource.k8s.io/v1".to_string(),
            kind: "ResourceClaim".to_string(),
        },
        metadata: Some(ObjectMeta {
            name: Some("claim-a".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        spec: ResourceClaimSpec {
            devices: DeviceClaim {
                requests: vec![DeviceRequest {
                    name: "req-a".to_string(),
                    exactly: Some(ExactDeviceRequest {
                        device_class_name: "gpu".to_string(),
                        selectors: vec![ClaimSel {
                            cel: Some(crate::resource::v1::resource_claim::CELDeviceSelector {
                                expression: "attributes.vendor == 'nvidia'".to_string(),
                            }),
                        }],
                        allocation_mode: Some("ExactCount".to_string()),
                        count: Some(2),
                        admin_access: Some(false),
                        tolerations: vec![DeviceToleration {
                            key: "example.com/taint".to_string(),
                            operator: Some("Equal".to_string()),
                            value: "true".to_string(),
                            effect: Some("NoSchedule".to_string()),
                            toleration_seconds: Some(30),
                        }],
                        capacity: Some(CapacityRequirements {
                            requests: BTreeMap::from([("memory".to_string(), "1Gi".to_string())]),
                        }),
                    }),
                    first_available: vec![],
                }],
                constraints: vec![DeviceConstraint {
                    requests: vec!["req-a".to_string()],
                    match_attribute: "attributes.vendor".to_string(),
                }],
                config: vec![],
            },
        },
        status: Some(crate::resource::v1::ResourceClaimStatus {
            allocation: None,
            reserved_for: vec![ResourceClaimConsumerReference {
                api_group: "apps".to_string(),
                resource: "deployments".to_string(),
                name: "deploy-a".to_string(),
                uid: "uid-a".to_string(),
            }],
            devices: vec![],
        }),
    }
}

fn resource_claim_list_basic() -> ResourceClaimList {
    ResourceClaimList {
        type_meta: TypeMeta {
            api_version: "resource.k8s.io/v1".to_string(),
            kind: "ResourceClaimList".to_string(),
        },
        metadata: Some(ListMeta {
            resource_version: Some("2".to_string()),
            ..Default::default()
        }),
        items: vec![resource_claim_basic()],
    }
}

fn resource_claim_template_basic() -> ResourceClaimTemplate {
    ResourceClaimTemplate {
        type_meta: TypeMeta {
            api_version: "resource.k8s.io/v1".to_string(),
            kind: "ResourceClaimTemplate".to_string(),
        },
        metadata: Some(ObjectMeta {
            name: Some("template-a".to_string()),
            ..Default::default()
        }),
        spec: ResourceClaimTemplateSpec {
            metadata: Some(ObjectMeta {
                name: Some("claim-template".to_string()),
                ..Default::default()
            }),
            spec: ResourceClaimSpec::default(),
        },
    }
}

fn resource_claim_template_list_basic() -> ResourceClaimTemplateList {
    ResourceClaimTemplateList {
        type_meta: TypeMeta {
            api_version: "resource.k8s.io/v1".to_string(),
            kind: "ResourceClaimTemplateList".to_string(),
        },
        metadata: Some(ListMeta {
            resource_version: Some("3".to_string()),
            ..Default::default()
        }),
        items: vec![resource_claim_template_basic()],
    }
}

fn resource_slice_basic() -> ResourceSlice {
    let mut attributes = BTreeMap::new();
    attributes.insert(
        "model".to_string(),
        DeviceAttribute {
            string_value: Some("A100".to_string()),
            ..Default::default()
        },
    );

    let mut capacity = BTreeMap::new();
    capacity.insert(
        "memory".to_string(),
        DeviceCapacity {
            value: Quantity::from_str("1Gi"),
            ..Default::default()
        },
    );

    let mut counters = BTreeMap::new();
    counters.insert(
        "alloc".to_string(),
        Counter {
            value: Quantity::from_str("1"),
        },
    );

    ResourceSlice {
        type_meta: TypeMeta {
            api_version: "resource.k8s.io/v1".to_string(),
            kind: "ResourceSlice".to_string(),
        },
        metadata: Some(ObjectMeta {
            name: Some("slice-a".to_string()),
            ..Default::default()
        }),
        spec: ResourceSliceSpec {
            driver: "example.com/driver".to_string(),
            pool: ResourcePool {
                name: "pool-a".to_string(),
                generation: 1,
                resource_slice_count: 1,
            },
            node_name: Some("node-a".to_string()),
            node_selector: None,
            all_nodes: None,
            devices: vec![Device {
                name: "device-a".to_string(),
                attributes,
                capacity,
                consumes_counters: vec![DeviceCounterConsumption {
                    counter_set: "shared".to_string(),
                    counters: BTreeMap::new(),
                }],
                node_name: None,
                node_selector: None,
                all_nodes: None,
                taints: vec![DeviceTaint {
                    key: "example.com/taint".to_string(),
                    value: "true".to_string(),
                    effect: "NoSchedule".to_string(),
                    time_added: None,
                }],
                binds_to_node: Some(true),
                binding_conditions: vec!["Ready".to_string()],
                binding_failure_conditions: vec!["Unavailable".to_string()],
                allow_multiple_allocations: Some(false),
            }],
            per_device_node_selection: None,
            shared_counters: vec![CounterSet {
                name: "shared".to_string(),
                counters,
            }],
        },
    }
}

fn resource_slice_list_basic() -> ResourceSliceList {
    ResourceSliceList {
        type_meta: TypeMeta {
            api_version: "resource.k8s.io/v1".to_string(),
            kind: "ResourceSliceList".to_string(),
        },
        metadata: Some(ListMeta {
            resource_version: Some("4".to_string()),
            ..Default::default()
        }),
        items: vec![resource_slice_basic()],
    }
}

#[test]
fn serde_roundtrip_device_class() {
    assert_serde_roundtrip(&device_class_basic());
}

#[test]
fn serde_roundtrip_device_class_list() {
    assert_serde_roundtrip(&device_class_list_basic());
}

#[test]
fn serde_roundtrip_resource_claim() {
    assert_serde_roundtrip(&resource_claim_basic());
}

#[test]
fn serde_roundtrip_resource_claim_list() {
    assert_serde_roundtrip(&resource_claim_list_basic());
}

#[test]
fn serde_roundtrip_resource_claim_template() {
    assert_serde_roundtrip(&resource_claim_template_basic());
}

#[test]
fn serde_roundtrip_resource_claim_template_list() {
    assert_serde_roundtrip(&resource_claim_template_list_basic());
}

#[test]
fn serde_roundtrip_resource_slice() {
    assert_serde_roundtrip(&resource_slice_basic());
}

#[test]
fn serde_roundtrip_resource_slice_list() {
    assert_serde_roundtrip(&resource_slice_list_basic());
}
