//! ResourceSlice internal types
use crate::common::{ObjectMeta, Quantity, TypeMeta};
use crate::core::internal::NodeSelector;
use crate::impl_has_object_meta;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct ResourceSlice {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    pub metadata: ObjectMeta,
    #[serde(default)]
    pub spec: ResourceSliceSpec,
}
impl_has_object_meta!(ResourceSlice);

/// ResourceSliceSpec contains the information published by the driver in one ResourceSlice.
///
/// Source: k8s/pkg/apis/resource/types.go
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceSliceSpec {
    #[serde(default)]
    pub driver: String,
    #[serde(default)]
    pub pool: ResourcePool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_selector: Option<NodeSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all_nodes: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub devices: Vec<Device>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub per_device_node_selection: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub shared_counters: Vec<CounterSet>,
}

/// CounterSet defines a named set of counters.
///
/// Source: k8s/pkg/apis/resource/types.go
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CounterSet {
    #[serde(default)]
    pub name: String,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub counters: BTreeMap<String, Counter>,
}

/// Counter describes a quantity associated with a device.
///
/// Source: k8s/pkg/apis/resource/types.go
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Counter {
    #[serde(default)]
    pub value: Quantity,
}

/// ResourcePool describes the pool that ResourceSlices belong to.
///
/// Source: k8s/pkg/apis/resource/types.go
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourcePool {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub generation: i64,
    #[serde(default)]
    pub resource_slice_count: i64,
}

/// Device represents one individual hardware instance.
///
/// Source: k8s/pkg/apis/resource/types.go
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    #[serde(default)]
    pub name: String,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub attributes: BTreeMap<String, DeviceAttribute>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub capacity: BTreeMap<String, DeviceCapacity>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub consumes_counters: Vec<DeviceCounterConsumption>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_selector: Option<NodeSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all_nodes: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub taints: Vec<DeviceTaint>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub binds_to_node: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub binding_conditions: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub binding_failure_conditions: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_multiple_allocations: Option<bool>,
}

/// DeviceCounterConsumption defines a set of counters that a device will consume.
///
/// Source: k8s/pkg/apis/resource/types.go
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceCounterConsumption {
    #[serde(default)]
    pub counter_set: String,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub counters: BTreeMap<String, Counter>,
}

/// DeviceAttribute must have exactly one field set.
///
/// Source: k8s/pkg/apis/resource/types.go
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceAttribute {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "int")]
    pub int_value: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bool")]
    pub bool_value: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "string")]
    pub string_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "version")]
    pub version_value: Option<String>,
}

/// DeviceCapacity describes a quantity associated with a device.
///
/// Source: k8s/pkg/apis/resource/types.go
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceCapacity {
    #[serde(default)]
    pub value: Quantity,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_policy: Option<CapacityRequestPolicy>,
}

/// CapacityRequestPolicy defines how requests consume device capacity.
///
/// Source: k8s/pkg/apis/resource/types.go
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CapacityRequestPolicy {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<Quantity>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub valid_values: Vec<Quantity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_range: Option<CapacityRequestPolicyRange>,
}

/// CapacityRequestPolicyRange defines a valid range for consumable capacity values.
///
/// Source: k8s/pkg/apis/resource/types.go
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CapacityRequestPolicyRange {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<Quantity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<Quantity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub step: Option<Quantity>,
}

/// DeviceTaint represents a taint attached to a device.
///
/// Source: k8s/pkg/apis/resource/types.go
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceTaint {
    #[serde(default)]
    pub key: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub value: String,
    #[serde(default)]
    pub effect: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_added: Option<crate::common::time::Timestamp>,
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

impl crate::common::traits::HasTypeMeta for ResourceSlice {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}
