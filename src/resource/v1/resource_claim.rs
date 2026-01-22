//! ResourceClaim types from the Kubernetes Resource API
//!
//! Source: k8s.io/api/resource/v1/types.go

use crate::common::{ListMeta, ObjectMeta, TypeMeta};
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceRequest {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub device_class_name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub selectors: Vec<DeviceSelector>,
    #[serde(default)]
    pub count: i64,
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceClaimStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allocation: Option<AllocationResult>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reserved_for: Vec<ResourceClaimConsumerReference>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AllocationResult {
    #[serde(default)]
    pub devices: DeviceAllocationResult,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_selector: Option<String>,
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
