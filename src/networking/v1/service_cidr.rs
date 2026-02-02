//! ServiceCIDR types from the Kubernetes Networking API
//!
//! This module contains types for service CIDR resources.
//!
//! Source: k8s.io/api/networking/v1/types.go

use crate::common::{Condition, ListMeta, ObjectMeta, TypeMeta};
use crate::impl_unimplemented_prost_message;
use crate::impl_versioned_object;
use serde::{Deserialize, Serialize};

// ============================================================================
// ServiceCIDR
// ============================================================================

/// ServiceCIDR defines a range of IP addresses using CIDR format (e.g. 192.168.0.0/24 or 2001:db2::/64).
/// This range is used to allocate ClusterIPs to Service objects.
///
/// Corresponds to [Kubernetes ServiceCIDR](https://github.com/kubernetes/api/blob/master/networking/v1/types.go)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct ServiceCIDR {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// spec is the desired state of the ServiceCIDR.
    #[serde(default)]
    pub spec: ServiceCIDRSpec,
    /// status represents the current state of the ServiceCIDR.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ServiceCIDRStatus>,
}
impl_versioned_object!(ServiceCIDR);

/// ServiceCIDRList is a list of ServiceCIDR objects.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceCIDRList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(default)]
    pub metadata: Option<ListMeta>,
    /// Items is the list of ServiceCIDR objects.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ServiceCIDR>,
}

// ============================================================================
// ServiceCIDRSpec
// ============================================================================

/// ServiceCIDRSpec defines the desired state of ServiceCIDR.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceCIDRSpec {
    /// CIDRs defines the IP blocks in CIDR notation (e.g. "192.168.0.0/24" or "2001:db8::/64")
    /// from which to assign service cluster IPs. Max of two CIDRs is allowed, one of each IP family.
    /// This field is immutable.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cidrs: Vec<String>,
}

// ============================================================================
// ServiceCIDRStatus
// ============================================================================

/// ServiceCIDRStatus describes the current state of the ServiceCIDR.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceCIDRStatus {
    /// conditions holds an array of metav1.Condition that describe the state of the ServiceCIDR.
    /// Current service state
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<Condition>,
}

// ============================================================================
// ServiceCIDR Condition Constants
// ============================================================================

/// ServiceCIDR condition constants
pub mod service_cidr_condition {
    /// Ready is a condition type that represents the service CIDR is ready to be used.
    pub const TYPE_READY: &str = "Ready";

    /// Terminating is a reason for the Ready condition when the service CIDR is being deleted.
    pub const REASON_TERMINATING: &str = "Terminating";
}

// ============================================================================
// Trait Implementations
// ============================================================================

impl crate::common::traits::ResourceSchema for ServiceCIDR {
    type Meta = ();

    fn group(_meta: &Self::Meta) -> &str {
        "networking.k8s.io"
    }

    fn version(_meta: &Self::Meta) -> &str {
        "v1"
    }

    fn kind(_meta: &Self::Meta) -> &str {
        "ServiceCIDR"
    }

    fn resource(_meta: &Self::Meta) -> &str {
        "servicecidrs"
    }

    fn group_static() -> &'static str
    where
        Self::Meta: Default,
    {
        "networking.k8s.io"
    }

    fn version_static() -> &'static str
    where
        Self::Meta: Default,
    {
        "v1"
    }

    fn kind_static() -> &'static str
    where
        Self::Meta: Default,
    {
        "ServiceCIDR"
    }

    fn resource_static() -> &'static str
    where
        Self::Meta: Default,
    {
        "servicecidrs"
    }
}

impl crate::common::traits::ResourceSchema for ServiceCIDRList {
    type Meta = ();

    fn group(_meta: &Self::Meta) -> &str {
        "networking.k8s.io"
    }

    fn version(_meta: &Self::Meta) -> &str {
        "v1"
    }

    fn kind(_meta: &Self::Meta) -> &str {
        "ServiceCIDRList"
    }

    fn resource(_meta: &Self::Meta) -> &str {
        "servicecidrs"
    }

    fn group_static() -> &'static str
    where
        Self::Meta: Default,
    {
        "networking.k8s.io"
    }

    fn version_static() -> &'static str
    where
        Self::Meta: Default,
    {
        "v1"
    }

    fn kind_static() -> &'static str
    where
        Self::Meta: Default,
    {
        "ServiceCIDRList"
    }

    fn resource_static() -> &'static str
    where
        Self::Meta: Default,
    {
        "servicecidrs"
    }
}

impl crate::common::traits::HasTypeMeta for ServiceCIDR {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }

    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl crate::common::traits::ApplyDefault for ServiceCIDR {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "networking.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "ServiceCIDR".to_string();
        }
    }
}

impl crate::common::traits::ApplyDefault for ServiceCIDRList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "networking.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "ServiceCIDRList".to_string();
        }
        for item in &mut self.items {
            item.apply_default();
        }
    }
}

impl crate::common::traits::UnimplementedConversion for ServiceCIDR {}
impl crate::common::traits::UnimplementedConversion for ServiceCIDRList {}

// Protobuf Placeholder (using macro)
impl_unimplemented_prost_message!(ServiceCIDR);
impl_unimplemented_prost_message!(ServiceCIDRList);

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {}
