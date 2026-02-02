//! IPAddress types from the Kubernetes Networking API
//!
//! This module contains types for IP address resources.
//!
//! Source: k8s.io/api/networking/v1/types.go

use crate::common::{ListMeta, ObjectMeta, TypeMeta};
use crate::impl_unimplemented_prost_message;
use crate::impl_versioned_object;
use serde::{Deserialize, Serialize};

// ============================================================================
// IPAddress
// ============================================================================

/// IPAddress represents a single IP address.
///
/// An IP address can be represented in different formats, to guarantee the uniqueness of the IP,
/// the name of the object is the IP address in canonical format, four decimal digits separated
/// by dots suppressing leading zeros for IPv4 and the representation defined by RFC 5952 for IPv6.
///
/// Corresponds to [Kubernetes IPAddress](https://github.com/kubernetes/api/blob/master/networking/v1/types.go)
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
    /// An IPAddress must reference a parent object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_ref: Option<ParentReference>,
}

// ============================================================================
// ParentReference
// ============================================================================

/// ParentReference describes a reference to a parent object.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ParentReference {
    /// Group is the group of the object being referenced.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub group: String,

    /// Resource is the resource of the object being referenced.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub resource: String,

    /// Namespace is the namespace of the object being referenced.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub namespace: String,

    /// Name is the name of the object being referenced.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
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
        "v1"
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
        "v1"
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
        "v1"
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
        "v1"
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
            self.type_meta.api_version = "networking.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "IPAddress".to_string();
        }
    }
}

impl crate::common::traits::ApplyDefault for IPAddressList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "networking.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "IPAddressList".to_string();
        }
        for item in &mut self.items {
            item.apply_default();
        }
    }
}

impl crate::common::traits::UnimplementedConversion for IPAddress {}
impl crate::common::traits::UnimplementedConversion for IPAddressList {}

// Protobuf Placeholder (using macro)
impl_unimplemented_prost_message!(IPAddress);
impl_unimplemented_prost_message!(IPAddressList);

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {}
