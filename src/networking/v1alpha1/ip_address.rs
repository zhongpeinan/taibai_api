//! IPAddress types from the Kubernetes Networking API
//!
//! This module contains types for IP address resources.
//!
//! Source: k8s.io/api/networking/v1alpha1/types.go

use crate::common::{ListMeta, ObjectMeta, TypeMeta};
use crate::impl_versioned_object;
use serde::{Deserialize, Serialize};

// ============================================================================
// IPAddress
// ============================================================================

/// IPAddress represents a single IP address.
///
/// Corresponds to [Kubernetes IPAddress](https://github.com/kubernetes/api/blob/master/networking/v1alpha1/types.go)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct IPAddress {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// spec is the desired state of the IPAddress.
    #[serde(default)]
    pub spec: IPAddressSpec,
}
impl_versioned_object!(IPAddress);


/// IPAddressList is a list of IPAddress objects.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct IPAddressList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(default)]
    pub metadata: Option<ListMeta>,
    /// Items is the list of IPAddress objects.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<IPAddress>,
}

// ============================================================================
// IPAddressSpec
// ============================================================================

/// IPAddressSpec describes the attributes of an IP address.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct IPAddressSpec {
    /// parentRef references the resource that an IPAddress is attached to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_ref: Option<String>,
}

// ============================================================================
// Trait Implementations
// ============================================================================

impl crate::common::traits::ResourceSchema for IPAddress {
    type Meta = ();

    fn group(_meta: &Self::Meta) -> &str {
        "networking.k8s.io"
    }

    fn version(_meta: &Self::Meta) -> &str {
        "v1alpha1"
    }

    fn kind(_meta: &Self::Meta) -> &str {
        "IPAddress"
    }

    fn resource(_meta: &Self::Meta) -> &str {
        "ipaddresses"
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
        "v1alpha1"
    }

    fn kind_static() -> &'static str
    where
        Self::Meta: Default,
    {
        "IPAddress"
    }

    fn resource_static() -> &'static str
    where
        Self::Meta: Default,
    {
        "ipaddresses"
    }
}

impl crate::common::traits::ResourceSchema for IPAddressList {
    type Meta = ();

    fn group(_meta: &Self::Meta) -> &str {
        "networking.k8s.io"
    }

    fn version(_meta: &Self::Meta) -> &str {
        "v1alpha1"
    }

    fn kind(_meta: &Self::Meta) -> &str {
        "IPAddressList"
    }

    fn resource(_meta: &Self::Meta) -> &str {
        "ipaddresses"
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
        "v1alpha1"
    }

    fn kind_static() -> &'static str
    where
        Self::Meta: Default,
    {
        "IPAddressList"
    }

    fn resource_static() -> &'static str
    where
        Self::Meta: Default,
    {
        "ipaddresses"
    }
}

impl crate::common::traits::HasTypeMeta for IPAddress {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }

    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl crate::common::traits::ApplyDefault for IPAddress {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "networking.k8s.io/v1alpha1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "IPAddress".to_string();
        }
    }
}

impl crate::common::traits::ApplyDefault for IPAddressList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "networking.k8s.io/v1alpha1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "IPAddressList".to_string();
        }
    }
}

impl crate::common::traits::UnimplementedConversion for IPAddress {}
impl crate::common::traits::UnimplementedConversion for IPAddressList {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {}
