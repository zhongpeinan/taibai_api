//! ResourceClaim internal types
use crate::common::{Condition, ObjectMeta, TypeMeta};
use crate::core::internal::NodeSelector;
use crate::impl_has_object_meta;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct ResourceClaim {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ObjectMeta,
    #[serde(default)]
    pub spec: ResourceClaimSpec,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ResourceClaimStatus>,
}
impl_has_object_meta!(ResourceClaim);

/// ResourceClaimSpec defines what is being requested in a ResourceClaim and how to configure it.
///
/// Source: k8s/pkg/apis/resource/types.go
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceClaimSpec {
    #[serde(default)]
    pub devices: DeviceClaim,
}

/// DeviceClaim defines how to request devices with a ResourceClaim.
///
/// Source: k8s/pkg/apis/resource/types.go
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceClaim {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requests: Vec<DeviceRequest>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub constraints: Vec<DeviceConstraint>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub config: Vec<DeviceClaimConfiguration>,
}

/// DeviceRequest is a request for devices required for a claim.
///
/// Source: k8s/pkg/apis/resource/types.go
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceRequest {
    #[serde(default)]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exactly: Option<ExactDeviceRequest>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub first_available: Vec<DeviceSubRequest>,
}

/// ExactDeviceRequest is a request for one or more identical devices.
///
/// Source: k8s/pkg/apis/resource/types.go
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExactDeviceRequest {
    #[serde(default)]
    pub device_class_name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub selectors: Vec<DeviceSelector>,
    #[serde(default)]
    pub allocation_mode: DeviceAllocationMode,
    #[serde(default)]
    pub count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub admin_access: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tolerations: Vec<DeviceToleration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<CapacityRequirements>,
}

/// DeviceSubRequest describes a request for device provided in firstAvailable.
///
/// Source: k8s/pkg/apis/resource/types.go
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceSubRequest {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub device_class_name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub selectors: Vec<DeviceSelector>,
    #[serde(default)]
    pub allocation_mode: DeviceAllocationMode,
    #[serde(default)]
    pub count: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tolerations: Vec<DeviceToleration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<CapacityRequirements>,
}

/// DeviceAllocationMode defines how devices are allocated.
///
/// Source: k8s/pkg/apis/resource/types.go
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum DeviceAllocationMode {
    #[serde(rename = "ExactCount")]
    #[default]
    ExactCount,
    #[serde(rename = "All")]
    All,
}

/// DeviceSelector must have exactly one field set.
///
/// Source: k8s/pkg/apis/resource/types.go
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceSelector {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cel: Option<CELDeviceSelector>,
}

/// CELDeviceSelector contains a CEL expression for selecting a device.
///
/// Source: k8s/pkg/apis/resource/types.go
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CELDeviceSelector {
    #[serde(default)]
    pub expression: String,
}

/// DeviceToleration represents a toleration for device taints.
///
/// Source: k8s/pkg/apis/resource/types.go
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceToleration {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub key: String,
    #[serde(default)]
    pub operator: DeviceTolerationOperator,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub value: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub toleration_seconds: Option<i64>,
}

/// DeviceTolerationOperator is the set of operators that can be used in a toleration.
///
/// Source: k8s/pkg/apis/resource/types.go
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum DeviceTolerationOperator {
    #[serde(rename = "Exists")]
    Exists,
    #[serde(rename = "Equal")]
    #[default]
    Equal,
}

/// CapacityRequirements defines the capacity requirements for a specific device request.
///
/// Source: k8s/pkg/apis/resource/types.go
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CapacityRequirements {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub requests: BTreeMap<String, String>,
}

/// DeviceConstraint must have exactly one field set besides Requests.
///
/// Source: k8s/pkg/apis/resource/types.go
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceConstraint {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requests: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_attribute: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distinct_attribute: Option<String>,
}

/// DeviceClaimConfiguration is used for configuration parameters in DeviceClaim.
///
/// Source: k8s/pkg/apis/resource/types.go
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceClaimConfiguration {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requests: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opaque: Option<OpaqueDeviceConfiguration>,
}

/// OpaqueDeviceConfiguration contains configuration parameters for a driver.
///
/// Source: k8s/pkg/apis/resource/types.go
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct OpaqueDeviceConfiguration {
    #[serde(default)]
    pub driver: String,
    /// Note: Using serde_json::Value as a substitute for runtime.RawExtension
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}

/// NetworkDeviceData provides network-related details for the allocated device.
///
/// Source: k8s/pkg/apis/resource/types.go
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NetworkDeviceData {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub interface_name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ips: Vec<String>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub hardware_address: String,
}

/// AllocatedDeviceStatus contains the status of an allocated device.
///
/// Source: k8s/pkg/apis/resource/types.go
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AllocatedDeviceStatus {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub driver: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub pool: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub device: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub share_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<Condition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network_data: Option<NetworkDeviceData>,
}

/// AllocationResult is set once the claim has been allocated successfully.
///
/// Source: k8s/pkg/apis/resource/types.go
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AllocationResult {
    #[serde(default)]
    pub devices: DeviceAllocationResult,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_selector: Option<NodeSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allocation_timestamp: Option<crate::common::time::Timestamp>,
}

/// DeviceAllocationResult is the result of allocating devices.
///
/// Source: k8s/pkg/apis/resource/types.go
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceAllocationResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub results: Vec<DeviceRequestAllocationResult>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub config: Vec<DeviceAllocationConfiguration>,
}

/// DeviceRequestAllocationResult contains the allocation result for one request.
///
/// Source: k8s/pkg/apis/resource/types.go
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceRequestAllocationResult {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub request: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub driver: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub pool: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub device: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub admin_access: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tolerations: Vec<DeviceToleration>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub binding_conditions: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub binding_failure_conditions: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub share_id: Option<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub consumed_capacity: BTreeMap<String, String>,
}

/// DeviceAllocationConfiguration represents a configuration parameter.
///
/// Source: k8s/pkg/apis/resource/types.go
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceAllocationConfiguration {
    #[serde(default)]
    pub source: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requests: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opaque: Option<OpaqueDeviceConfiguration>,
}

/// ResourceClaimConsumerReference contains enough information to locate the consumer.
///
/// Source: k8s/pkg/apis/resource/types.go
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceClaimConsumerReference {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub api_group: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub resource: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub uid: String,
}

/// ResourceClaimStatus tracks whether the resource has been allocated.
///
/// Source: k8s/pkg/apis/resource/types.go
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceClaimStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allocation: Option<AllocationResult>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reserved_for: Vec<ResourceClaimConsumerReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub devices: Vec<AllocatedDeviceStatus>,
}

impl crate::common::traits::ResourceSchema for ResourceClaim {
    type Meta = ();
    fn group(_meta: &Self::Meta) -> &str {
        "resource.k8s.io"
    }
    fn version(_meta: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_meta: &Self::Meta) -> &str {
        "ResourceClaim"
    }
    fn resource(_meta: &Self::Meta) -> &str {
        "resourceclaims"
    }
    fn group_static() -> &'static str {
        "resource.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "ResourceClaim"
    }
    fn resource_static() -> &'static str {
        "resourceclaims"
    }
}

impl crate::common::traits::HasTypeMeta for ResourceClaim {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

// AsRefStr / AsRef<str> implementations for enums
crate::impl_as_str_ref!(DeviceAllocationMode, {
    ExactCount => "ExactCount",
    All => "All",
});

crate::impl_as_str_ref!(DeviceTolerationOperator, {
    Exists => "Exists",
    Equal => "Equal",
});
