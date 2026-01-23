//! ResourceClaim types from the Kubernetes Resource API
//!
//! Source: k8s.io/api/resource/v1/types.go

use crate::common::time::Timestamp;
use crate::common::{Condition, ListMeta, ObjectMeta, TypeMeta};
use crate::impl_versioned_object;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ResourceClaim {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    #[serde(default)]
    pub spec: ResourceClaimSpec,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ResourceClaimStatus>,
}
impl_versioned_object!(ResourceClaim);

impl Default for ResourceClaim {
    fn default() -> Self {
        Self {
            type_meta: TypeMeta::default(),
            metadata: None,
            spec: ResourceClaimSpec::default(),
            status: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceClaimList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: Option<ListMeta>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ResourceClaim>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceClaimSpec {
    #[serde(default)]
    pub devices: DeviceClaim,
}

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

/// ExactDeviceRequest is a request for one or more identical devices.
///
/// Source: k8s.io/api/resource/v1/types.go
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExactDeviceRequest {
    /// DeviceClassName references a specific DeviceClass, which can define
    /// additional configuration and selectors to be inherited by this request.
    #[serde(default)]
    pub device_class_name: String,
    /// Selectors define criteria which must be satisfied by a specific
    /// device in order for that device to be considered for this request.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub selectors: Vec<DeviceSelector>,
    /// AllocationMode defines how devices are allocated to satisfy this request.
    /// Supported values are "ExactCount" and "All".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allocation_mode: Option<String>,
    /// Count is used only when the allocation mode is "ExactCount".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// AdminAccess indicates that this is a claim for administrative access to the device(s).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub admin_access: Option<bool>,
    /// Tolerations for device taints.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tolerations: Vec<DeviceToleration>,
    /// Capacity defines resource requirements against each capacity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<CapacityRequirements>,
}

/// DeviceSubRequest describes a request for device provided in the
/// claim.spec.devices.requests[].firstAvailable array.
///
/// Source: k8s.io/api/resource/v1/types.go
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceSubRequest {
    /// Name can be used to reference this subrequest.
    #[serde(default)]
    pub name: String,
    /// DeviceClassName references a specific DeviceClass.
    #[serde(default)]
    pub device_class_name: String,
    /// Selectors define criteria which must be satisfied by a specific device.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub selectors: Vec<DeviceSelector>,
    /// AllocationMode defines how devices are allocated to satisfy this subrequest.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allocation_mode: Option<String>,
    /// Count is used only when the allocation mode is "ExactCount".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// Tolerations for device taints.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tolerations: Vec<DeviceToleration>,
    /// Capacity defines resource requirements against each capacity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<CapacityRequirements>,
}

/// DeviceToleration represents a toleration for device taints.
///
/// Source: k8s.io/api/resource/v1/types.go
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceToleration {
    /// Key is the taint key that the toleration applies to.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub key: String,
    /// Operator represents a key's relationship to the value.
    /// Valid operators are "Exists" and "Equal". Defaults to "Equal".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    /// Value is the taint value the toleration matches to.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub value: String,
    /// Effect indicates the taint effect to match.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    /// TolerationSeconds represents the period of time the toleration tolerates the taint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub toleration_seconds: Option<i64>,
}

/// CapacityRequirements defines the capacity requirements for a specific device request.
///
/// Source: k8s.io/api/resource/v1/types.go
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CapacityRequirements {
    /// Requests represent individual device resource requests.
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub requests: std::collections::BTreeMap<String, String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceRequest {
    /// Name can be used to reference this request.
    #[serde(default)]
    pub name: String,
    /// Exactly specifies the details for a single request that must be met exactly.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exactly: Option<ExactDeviceRequest>,
    /// FirstAvailable contains subrequests, of which exactly one will be selected.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub first_available: Vec<DeviceSubRequest>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceSelector {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cel: Option<CELDeviceSelector>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CELDeviceSelector {
    #[serde(default)]
    pub expression: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceConstraint {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requests: Vec<String>,
    #[serde(default)]
    pub match_attribute: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceClaimConfiguration {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requests: Vec<String>,
}

/// NetworkDeviceData provides network-related details for the allocated device.
/// This information may be filled by drivers or other components to configure
/// or identify the device within a network context.
///
/// Source: k8s.io/api/resource/v1/types.go
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NetworkDeviceData {
    /// InterfaceName specifies the name of the network interface associated with
    /// the allocated device. This might be the name of a physical or virtual
    /// network interface being configured in the pod.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub interface_name: String,
    /// IPs lists the network addresses assigned to the device's network interface.
    /// This can include both IPv4 and IPv6 addresses.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ips: Vec<String>,
    /// HardwareAddress represents the hardware address (e.g. MAC Address) of the device's network interface.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub hardware_address: String,
}

/// AllocatedDeviceStatus contains the status of each device allocated for this
/// claim, as reported by the driver. This can include driver-specific
/// information. Entries are owned by their respective drivers.
///
/// Source: k8s.io/api/resource/v1/types.go
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AllocatedDeviceStatus {
    /// Driver specifies the name of the DRA driver whose kubelet
    /// plugin should be invoked to process the allocation once the claim is
    /// needed on a node.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub driver: String,
    /// This name together with the driver name and the device name field
    /// identify which device was allocated (`<driver name>/<pool name>/<device name>`).
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub pool: String,
    /// Device references one device instance via its name in the driver's
    /// resource pool. It must be a DNS label.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub device: String,
    /// ShareID uniquely identifies an individual allocation share of the device.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub share_id: Option<String>,
    /// Conditions contains the latest observation of the device's state.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<Condition>,
    /// Data contains arbitrary driver-specific data.
    /// Note: Using serde_json::Value as a substitute for runtime.RawExtension
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    /// NetworkData contains network-related information specific to the device.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network_data: Option<NetworkDeviceData>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceClaimStatus {
    /// Allocation is set once the claim has been allocated successfully.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allocation: Option<AllocationResult>,
    /// ReservedFor indicates which entities are currently allowed to use the claim.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reserved_for: Vec<ResourceClaimConsumerReference>,
    /// Devices contains the status of each device allocated for this
    /// claim, as reported by the driver. This can include driver-specific
    /// information. Entries are owned by their respective drivers.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub devices: Vec<AllocatedDeviceStatus>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AllocationResult {
    #[serde(default)]
    pub devices: DeviceAllocationResult,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_selector: Option<String>,
    /// AllocationTimestamp stores time when the resources were allocated.
    /// This field is not guaranteed to be set, in which case that time is unknown.
    /// This is an alpha field and requires enabling the DRADeviceBindingConditions
    /// and DRAResourceClaimDeviceStatus feature gate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allocation_timestamp: Option<Timestamp>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceAllocationResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub results: Vec<DeviceRequestAllocationResult>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub config: Vec<DeviceAllocationConfiguration>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceRequestAllocationResult {
    #[serde(default)]
    pub request: String,
    #[serde(default)]
    pub driver: String,
    #[serde(default)]
    pub pool: String,
    #[serde(default)]
    pub device: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceAllocationConfiguration {
    #[serde(default)]
    pub source: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceClaimConsumerReference {
    #[serde(default)]
    pub api_group: String,
    #[serde(default)]
    pub resource: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub uid: String,
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

impl crate::common::traits::ResourceSchema for ResourceClaimList {
    type Meta = ();
    fn group(_meta: &Self::Meta) -> &str {
        "resource.k8s.io"
    }
    fn version(_meta: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_meta: &Self::Meta) -> &str {
        "ResourceClaimList"
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
        "ResourceClaimList"
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

impl crate::common::traits::ApplyDefault for ResourceClaim {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "resource.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "ResourceClaim".to_string();
        }
    }
}

impl crate::common::traits::ApplyDefault for ResourceClaimList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "resource.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "ResourceClaimList".to_string();
        }
    }
}

impl crate::common::traits::UnimplementedConversion for ResourceClaim {}
impl crate::common::traits::UnimplementedConversion for ResourceClaimList {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_resource_claim_default() {
        let rc = ResourceClaim::default();
        assert!(rc.metadata.is_none());
        assert!(rc.status.is_none());
    }
}
