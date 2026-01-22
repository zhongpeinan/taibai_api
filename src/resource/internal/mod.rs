//! Kubernetes Resource API Internal Types

pub mod device_class;
pub mod resource_claim;
pub mod resource_claim_template;
pub mod resource_slice;

pub use device_class::DeviceClass;
pub use resource_claim::ResourceClaim;
pub use resource_claim_template::{ResourceClaimTemplate, ResourceClaimTemplateSpec};
pub use resource_slice::ResourceSlice;
