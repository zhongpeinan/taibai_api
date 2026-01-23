//! ResourceSlice types from the Kubernetes Resource API
//!
//! Source: k8s.io/api/resource/v1/types.go

use crate::common::{ListMeta, ObjectMeta, Quantity, TypeMeta};
use crate::core::v1::affinity::NodeSelector;
use crate::impl_versioned_object;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ResourceSlice {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    #[serde(default)]
    pub spec: ResourceSliceSpec,
}
impl_versioned_object!(ResourceSlice);

impl Default for ResourceSlice {
    fn default() -> Self {
        Self {
            type_meta: TypeMeta::default(),
            metadata: None,
            spec: ResourceSliceSpec::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceSliceList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    #[serde(default)]
    pub metadata: Option<ListMeta>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ResourceSlice>,
}

/// ResourceSliceSpec contains the information published by the driver in one ResourceSlice.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceSliceSpec {
    /// Driver identifies the DRA driver providing the capacity information.
    #[serde(default)]
    pub driver: String,

    /// Pool describes the pool that this ResourceSlice belongs to.
    #[serde(default)]
    pub pool: ResourcePool,

    /// NodeName identifies the node which provides the resources in this pool.
    /// Exactly one of NodeName, NodeSelector, AllNodes, and PerDeviceNodeSelection must be set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,

    /// NodeSelector defines which nodes have access to the resources in the pool,
    /// when that pool is not limited to a single node.
    /// Exactly one of NodeName, NodeSelector, AllNodes, and PerDeviceNodeSelection must be set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_selector: Option<NodeSelector>,

    /// AllNodes indicates that all nodes have access to the resources in the pool.
    /// Exactly one of NodeName, NodeSelector, AllNodes, and PerDeviceNodeSelection must be set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all_nodes: Option<bool>,

    /// Devices lists some or all of the devices in this pool.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub devices: Vec<Device>,

    /// PerDeviceNodeSelection defines whether the access from nodes to
    /// resources in the pool is set on the ResourceSlice level or on each device.
    /// Exactly one of NodeName, NodeSelector, AllNodes, and PerDeviceNodeSelection must be set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub per_device_node_selection: Option<bool>,

    /// SharedCounters defines a list of counter sets, each of which
    /// has a name and a list of counters available.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub shared_counters: Vec<CounterSet>,
}

/// CounterSet defines a named set of counters that are available to be used by devices.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CounterSet {
    /// Name defines the name of the counter set.
    #[serde(default)]
    pub name: String,

    /// Counters defines the set of counters for this CounterSet.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub counters: BTreeMap<String, Counter>,
}

/// Counter describes a quantity associated with a device.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Counter {
    /// Value defines how much of a certain device counter is available.
    #[serde(default)]
    pub value: Quantity,
}

/// ResourcePool describes the pool that ResourceSlices belong to.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourcePool {
    /// Name is used to identify the pool.
    #[serde(default)]
    pub name: String,

    /// Generation tracks the change in a pool over time.
    #[serde(default)]
    pub generation: i64,

    /// ResourceSliceCount is the total number of ResourceSlices in the pool at this generation number.
    #[serde(default)]
    pub resource_slice_count: i64,
}

/// Device represents one individual hardware instance that can be selected based on its attributes.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    /// Name is unique identifier among all devices managed by the driver in the pool.
    #[serde(default)]
    pub name: String,

    /// Attributes defines the set of attributes for this device.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub attributes: BTreeMap<String, DeviceAttribute>,

    /// Capacity defines the set of capacities for this device.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub capacity: BTreeMap<String, DeviceCapacity>,

    /// ConsumesCounters defines a list of references to sharedCounters
    /// and the set of counters that the device will consume from those counter sets.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub consumes_counters: Vec<DeviceCounterConsumption>,

    /// NodeName identifies the node where the device is available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,

    /// NodeSelector defines the nodes where the device is available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_selector: Option<NodeSelector>,

    /// AllNodes indicates that all nodes have access to the device.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all_nodes: Option<bool>,

    /// Taints are the driver-defined taints.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub taints: Vec<DeviceTaint>,

    /// BindsToNode indicates if the usage of an allocation involving this device
    /// has to be limited to exactly the node that was chosen when allocating the claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub binds_to_node: Option<bool>,

    /// BindingConditions defines the conditions for proceeding with binding.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub binding_conditions: Vec<String>,

    /// BindingFailureConditions defines the conditions for binding failure.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub binding_failure_conditions: Vec<String>,

    /// AllowMultipleAllocations marks whether the device is allowed to be allocated to multiple DeviceRequests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_multiple_allocations: Option<bool>,
}

/// DeviceCounterConsumption defines a set of counters that a device will consume from a CounterSet.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceCounterConsumption {
    /// CounterSet is the name of the set from which the counters defined will be consumed.
    #[serde(default)]
    pub counter_set: String,

    /// Counters defines the counters that will be consumed by the device.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub counters: BTreeMap<String, Counter>,
}

/// DeviceAttribute must have exactly one field set.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceAttribute {
    /// IntValue is a number.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "int")]
    pub int_value: Option<i64>,

    /// BoolValue is a true/false value.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bool")]
    pub bool_value: Option<bool>,

    /// StringValue is a string. Must not be longer than 64 characters.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "string")]
    pub string_value: Option<String>,

    /// VersionValue is a semantic version according to semver.org spec 2.0.0.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "version")]
    pub version_value: Option<String>,
}

/// DeviceCapacity describes a quantity associated with a device.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceCapacity {
    /// Value defines how much of a certain capacity that device has.
    #[serde(default)]
    pub value: Quantity,

    /// RequestPolicy defines how this DeviceCapacity must be consumed
    /// when the device is allowed to be shared by multiple allocations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_policy: Option<CapacityRequestPolicy>,
}

/// CapacityRequestPolicy defines how requests consume device capacity.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CapacityRequestPolicy {
    /// Default specifies how much of this capacity is consumed by a request
    /// that does not contain an entry for it in DeviceRequest's Capacity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<Quantity>,

    /// ValidValues defines a set of acceptable quantity values in consuming requests.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub valid_values: Vec<Quantity>,

    /// ValidRange defines an acceptable quantity value range in consuming requests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_range: Option<CapacityRequestPolicyRange>,
}

/// CapacityRequestPolicyRange defines a valid range for consumable capacity values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CapacityRequestPolicyRange {
    /// Min specifies the minimum capacity allowed for a consumption request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<Quantity>,

    /// Max defines the upper limit for capacity that can be requested.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<Quantity>,

    /// Step defines the step size between valid capacity amounts within the range.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub step: Option<Quantity>,
}

/// DeviceTaint represents a device taint.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceTaint {
    /// The taint key to be applied to a device.
    #[serde(default)]
    pub key: String,

    /// The taint value corresponding to the taint key.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub value: String,

    /// The effect of the taint on claims that do not tolerate the taint.
    #[serde(default)]
    pub effect: DeviceTaintEffect,

    /// TimeAdded represents the time at which the taint was added.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_added: Option<String>,
}

/// DeviceTaintEffect is the effect of a device taint.
pub type DeviceTaintEffect = String;

/// Device taint effect constants
pub mod device_taint_effect {
    /// Do not allow new pods to schedule which use a tainted device unless they tolerate the taint.
    pub const NO_SCHEDULE: &str = "NoSchedule";
    /// Evict any already-running pods that do not tolerate the device taint.
    pub const NO_EXECUTE: &str = "NoExecute";
}

impl crate::common::traits::ResourceSchema for ResourceSlice {
    type Meta = ();
    fn group(_meta: &Self::Meta) -> &str {
        "resource.k8s.io"
    }
    fn version(_meta: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_meta: &Self::Meta) -> &str {
        "ResourceSlice"
    }
    fn resource(_meta: &Self::Meta) -> &str {
        "resourceslices"
    }
    fn group_static() -> &'static str {
        "resource.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "ResourceSlice"
    }
    fn resource_static() -> &'static str {
        "resourceslices"
    }
}

impl crate::common::traits::ResourceSchema for ResourceSliceList {
    type Meta = ();
    fn group(_meta: &Self::Meta) -> &str {
        "resource.k8s.io"
    }
    fn version(_meta: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_meta: &Self::Meta) -> &str {
        "ResourceSliceList"
    }
    fn resource(_meta: &Self::Meta) -> &str {
        "resourceslices"
    }
    fn group_static() -> &'static str {
        "resource.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "ResourceSliceList"
    }
    fn resource_static() -> &'static str {
        "resourceslices"
    }
}

impl crate::common::traits::HasTypeMeta for ResourceSlice {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl crate::common::traits::ApplyDefault for ResourceSlice {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "resource.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "ResourceSlice".to_string();
        }
    }
}

impl crate::common::traits::ApplyDefault for ResourceSliceList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "resource.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "ResourceSliceList".to_string();
        }
    }
}

impl crate::common::traits::UnimplementedConversion for ResourceSlice {}
impl crate::common::traits::UnimplementedConversion for ResourceSliceList {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_resource_slice_default() {
        let rs = ResourceSlice::default();
        assert!(rs.metadata.is_none());
    }

    #[test]
    fn test_resource_slice_spec_with_all_fields() {
        let spec = ResourceSliceSpec {
            driver: "test-driver".to_string(),
            pool: ResourcePool {
                name: "test-pool".to_string(),
                generation: 1,
                resource_slice_count: 1,
            },
            node_name: Some("node-1".to_string()),
            node_selector: None,
            all_nodes: None,
            devices: vec![],
            per_device_node_selection: None,
            shared_counters: vec![],
        };
        assert_eq!(spec.driver, "test-driver");
        assert_eq!(spec.pool.name, "test-pool");
    }

    #[test]
    fn test_device_with_attributes() {
        let mut attributes = BTreeMap::new();
        attributes.insert(
            "test-attr".to_string(),
            DeviceAttribute {
                int_value: Some(42),
                bool_value: None,
                string_value: None,
                version_value: None,
            },
        );

        let device = Device {
            name: "device-1".to_string(),
            attributes,
            capacity: BTreeMap::new(),
            consumes_counters: vec![],
            node_name: None,
            node_selector: None,
            all_nodes: None,
            taints: vec![],
            binds_to_node: None,
            binding_conditions: vec![],
            binding_failure_conditions: vec![],
            allow_multiple_allocations: None,
        };
        assert_eq!(device.name, "device-1");
        assert_eq!(device.attributes.len(), 1);
    }
}
