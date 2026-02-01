//! Kubernetes Resource API Internal Types

pub mod device_class;
pub mod resource_claim;
pub mod resource_claim_template;
pub mod resource_slice;
pub mod validation;

pub use device_class::{
    CELDeviceSelector, DeviceClass, DeviceClassConfiguration, DeviceClassSpec, DeviceSelector,
    OpaqueDeviceConfiguration,
};
pub use resource_claim::{
    AllocatedDeviceStatus, AllocationResult, DeviceAllocationConfiguration, DeviceAllocationMode,
    DeviceAllocationResult, DeviceRequestAllocationResult, NetworkDeviceData, ResourceClaim,
    ResourceClaimConsumerReference, ResourceClaimSpec, ResourceClaimStatus,
};
pub use resource_claim_template::{ResourceClaimTemplate, ResourceClaimTemplateSpec};
pub use resource_slice::ResourceSlice;
