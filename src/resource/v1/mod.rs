//! Kubernetes Resource v1 API types
//!
//! This module contains types from the Kubernetes resource.k8s.io/v1 API group.

pub mod conversion;
pub mod device_class;
pub mod resource_claim;
pub mod resource_claim_template;
pub mod resource_slice;

pub use device_class::{DeviceClass, DeviceClassList, DeviceClassSpec};
pub use resource_claim::{
    AllocatedDeviceStatus, NetworkDeviceData, ResourceClaim, ResourceClaimList, ResourceClaimSpec,
    ResourceClaimStatus,
};
pub use resource_claim_template::{
    ResourceClaimTemplate, ResourceClaimTemplateList, ResourceClaimTemplateSpec,
};
pub use resource_slice::{ResourceSlice, ResourceSliceList, ResourceSliceSpec};

#[cfg(test)]
mod trait_tests;

#[cfg(test)]
mod serde_roundtrip_tests;

#[cfg(test)]
mod conversion_roundtrip_tests;
