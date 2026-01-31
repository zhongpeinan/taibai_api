use super::{DeviceClass, ResourceClaim};
use crate::common::test_utils::assert_conversion_roundtrip;
use crate::common::{ObjectMeta, TypeMeta};
use crate::resource::internal;
use crate::resource::v1::device_class::{
    CELDeviceSelector as ClassCel, DeviceClassConfiguration, DeviceClassSpec,
    DeviceSelector as ClassSel, OpaqueDeviceConfiguration,
};
use crate::resource::v1::resource_claim::{
    CapacityRequirements, DeviceClaim, DeviceConstraint, DeviceRequest, DeviceSelector as ClaimSel,
    DeviceToleration, ExactDeviceRequest, ResourceClaimSpec,
};
use std::collections::BTreeMap;

fn device_class_basic() -> DeviceClass {
    DeviceClass {
        type_meta: TypeMeta::default(),
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

fn resource_claim_basic() -> ResourceClaim {
    ResourceClaim {
        type_meta: TypeMeta::default(),
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
        status: None,
    }
}

#[test]
fn conversion_roundtrip_device_class() {
    assert_conversion_roundtrip::<DeviceClass, internal::DeviceClass>(device_class_basic());
}

#[test]
fn conversion_roundtrip_resource_claim() {
    assert_conversion_roundtrip::<ResourceClaim, internal::ResourceClaim>(resource_claim_basic());
}
